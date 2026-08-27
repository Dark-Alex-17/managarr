use anyhow::Result;
use log::{debug, info, warn};
use serde_json::{Value, json};

use crate::models::Route;
use crate::models::readarr_models::{
  AddAuthorBody, AddAuthorSearchResult, Author, DeleteParams, EditAuthorParams, ReadarrCommandBody,
};
use crate::models::servarr_data::readarr::readarr_data::ActiveReadarrBlock;
use crate::models::stateful_table::StatefulTable;
use crate::network::readarr_network::ReadarrEvent;
use crate::network::{Network, RequestMethod};
use urlencoding::encode;

#[cfg(test)]
#[path = "readarr_authors_network_tests.rs"]
mod readarr_authors_network_tests;

impl Network<'_, '_> {
  pub(in crate::network::readarr_network) async fn add_author(
    &mut self,
    mut add_author_body: AddAuthorBody,
  ) -> Result<Value> {
    info!("Adding Readarr author: {}", add_author_body.author_name);
    if let Some(tag_input_str) = add_author_body.tag_input_string.as_ref() {
      let tag_ids_vec = self
        .extract_and_add_readarr_tag_ids_vec(tag_input_str)
        .await;
      add_author_body.tags = tag_ids_vec;
    }
    let event = ReadarrEvent::AddAuthor(AddAuthorBody::default());

    let request_props = self
      .request_props_from(
        event,
        RequestMethod::Post,
        Some(add_author_body),
        None,
        None,
      )
      .await;

    self
      .handle_request::<AddAuthorBody, Value>(request_props, |_, _| ())
      .await
  }

  pub(in crate::network::readarr_network) async fn delete_author(
    &mut self,
    delete_author_params: DeleteParams,
  ) -> Result<()> {
    let event = ReadarrEvent::DeleteAuthor(DeleteParams::default());
    let DeleteParams {
      id,
      delete_files,
      add_import_list_exclusion,
    } = delete_author_params;

    info!(
      "Deleting Readarr author with ID: {id} with deleteFiles={delete_files} and addImportListExclusion={add_import_list_exclusion}"
    );

    let request_props = self
      .request_props_from(
        event,
        RequestMethod::Delete,
        None::<()>,
        Some(format!("/{id}")),
        Some(format!(
          "deleteFiles={delete_files}&addImportListExclusion={add_import_list_exclusion}"
        )),
      )
      .await;

    self
      .handle_request::<(), ()>(request_props, |_, _| ())
      .await
  }

  pub(in crate::network::readarr_network) async fn edit_author(
    &mut self,
    mut edit_author_params: EditAuthorParams,
  ) -> Result<()> {
    info!("Editing Readarr author");
    if let Some(tag_input_str) = edit_author_params.tag_input_string.as_ref() {
      let tag_ids_vec = self
        .extract_and_add_readarr_tag_ids_vec(tag_input_str)
        .await;
      edit_author_params.tags = Some(tag_ids_vec);
    }
    let author_id = edit_author_params.author_id;
    let detail_event = ReadarrEvent::GetAuthorDetails(author_id);
    let event = ReadarrEvent::EditAuthor(EditAuthorParams::default());
    info!("Fetching author details for author with ID: {author_id}");

    let request_props = self
      .request_props_from(
        detail_event,
        RequestMethod::Get,
        None::<()>,
        Some(format!("/{author_id}")),
        None,
      )
      .await;

    let mut response = String::new();

    self
      .handle_request::<(), Value>(request_props, |detailed_author_body, _| {
        response = detailed_author_body.to_string()
      })
      .await?;

    info!("Constructing edit author body");

    let mut detailed_author_body: Value = serde_json::from_str(&response)?;
    let (
      monitored,
      monitor_new_items,
      quality_profile_id,
      metadata_profile_id,
      root_folder_path,
      tags,
    ) = {
      let monitored = edit_author_params.monitored.unwrap_or(
        detailed_author_body["monitored"]
          .as_bool()
          .expect("Unable to deserialize 'monitored'"),
      );
      let monitor_new_items = edit_author_params.monitor_new_items.unwrap_or_else(|| {
        serde_json::from_value(detailed_author_body["monitorNewItems"].clone())
          .expect("Unable to deserialize 'monitorNewItems'")
      });
      let quality_profile_id = edit_author_params.quality_profile_id.unwrap_or_else(|| {
        detailed_author_body["qualityProfileId"]
          .as_i64()
          .expect("Unable to deserialize 'qualityProfileId'")
      });
      let metadata_profile_id = edit_author_params.metadata_profile_id.unwrap_or_else(|| {
        detailed_author_body["metadataProfileId"]
          .as_i64()
          .expect("Unable to deserialize 'metadataProfileId'")
      });
      let root_folder_path = edit_author_params.root_folder_path.unwrap_or_else(|| {
        detailed_author_body["path"]
          .as_str()
          .expect("Unable to deserialize 'path'")
          .to_owned()
      });
      let tags = if edit_author_params.clear_tags {
        vec![]
      } else {
        edit_author_params.tags.unwrap_or(
          detailed_author_body["tags"]
            .as_array()
            .expect("Unable to deserialize 'tags'")
            .iter()
            .map(|item| item.as_i64().expect("Unable to deserialize tag ID"))
            .collect(),
        )
      };

      (
        monitored,
        monitor_new_items,
        quality_profile_id,
        metadata_profile_id,
        root_folder_path,
        tags,
      )
    };

    *detailed_author_body.get_mut("monitored").unwrap() = json!(monitored);
    *detailed_author_body.get_mut("monitorNewItems").unwrap() = json!(monitor_new_items);
    *detailed_author_body.get_mut("qualityProfileId").unwrap() = json!(quality_profile_id);
    *detailed_author_body.get_mut("metadataProfileId").unwrap() = json!(metadata_profile_id);
    *detailed_author_body.get_mut("path").unwrap() = json!(root_folder_path);
    *detailed_author_body.get_mut("tags").unwrap() = json!(tags);

    debug!("Edit author body: {detailed_author_body:?}");

    let request_props = self
      .request_props_from(
        event,
        RequestMethod::Put,
        Some(detailed_author_body),
        Some(format!("/{author_id}")),
        None,
      )
      .await;

    self
      .handle_request::<Value, ()>(request_props, |_, _| ())
      .await
  }

  pub(in crate::network::readarr_network) async fn get_author_details(
    &mut self,
    author_id: i64,
  ) -> Result<Author> {
    info!("Fetching details for Readarr author with ID: {author_id}");
    let event = ReadarrEvent::GetAuthorDetails(author_id);

    let request_props = self
      .request_props_from(
        event,
        RequestMethod::Get,
        None::<()>,
        Some(format!("/{author_id}")),
        None,
      )
      .await;

    self
      .handle_request::<(), Author>(request_props, |_, _| ())
      .await
  }

  pub(in crate::network::readarr_network) async fn list_authors(&mut self) -> Result<Vec<Author>> {
    info!("Fetching Readarr authors");
    let event = ReadarrEvent::ListAuthors;

    let request_props = self
      .request_props_from(event, RequestMethod::Get, None::<()>, None, None)
      .await;

    self
      .handle_request::<(), Vec<Author>>(request_props, |mut authors_vec, mut app| {
        if !matches!(
          app.get_current_route(),
          Route::Readarr(ActiveReadarrBlock::AuthorsSortPrompt, _)
        ) {
          authors_vec.sort_by_key(|a| a.id);
          app.data.readarr_data.authors.set_items(authors_vec);
          app.data.readarr_data.authors.apply_sorting_toggle(false);
        }
      })
      .await
  }

  pub(in crate::network::readarr_network) async fn search_author(
    &mut self,
    query: String,
  ) -> Result<Vec<AddAuthorSearchResult>> {
    info!("Searching for author: {query}");
    let event = ReadarrEvent::SearchNewAuthor(String::new());

    let request_props = self
      .request_props_from(
        event,
        RequestMethod::Get,
        None::<()>,
        None,
        Some(format!("term={}", encode(&query))),
      )
      .await;

    let result = self
      .handle_request::<(), Vec<AddAuthorSearchResult>>(request_props, |author_vec, mut app| {
        if author_vec.is_empty() {
          app.pop_and_push_navigation_stack(ActiveReadarrBlock::AddAuthorEmptySearchResults.into());
        } else if let Some(add_searched_authors) =
          app.data.readarr_data.add_searched_authors.as_mut()
        {
          add_searched_authors.set_items(author_vec);
        } else {
          let mut add_searched_authors = StatefulTable::default();
          add_searched_authors.set_items(author_vec);
          app.data.readarr_data.add_searched_authors = Some(add_searched_authors);
        }
      })
      .await;

    if result.is_err() {
      self.app.lock().await.data.readarr_data.add_searched_authors = Some(StatefulTable::default());
    }

    result
  }

  pub(in crate::network::readarr_network) async fn toggle_author_monitoring(
    &mut self,
    author_id: i64,
  ) -> Result<()> {
    let event = ReadarrEvent::ToggleAuthorMonitoring(author_id);

    let detail_event = ReadarrEvent::GetAuthorDetails(author_id);
    info!("Toggling author monitoring for author with ID: {author_id}");
    info!("Fetching author details for author with ID: {author_id}");

    let request_props = self
      .request_props_from(
        detail_event,
        RequestMethod::Get,
        None::<()>,
        Some(format!("/{author_id}")),
        None,
      )
      .await;

    let mut response = String::new();

    self
      .handle_request::<(), Value>(request_props, |detailed_author_body, _| {
        response = detailed_author_body.to_string()
      })
      .await?;

    info!("Constructing toggle author monitoring body");

    match serde_json::from_str::<Value>(&response) {
      Ok(mut detailed_author_body) => {
        let monitored = detailed_author_body
          .get("monitored")
          .unwrap()
          .as_bool()
          .unwrap();

        *detailed_author_body.get_mut("monitored").unwrap() = json!(!monitored);

        debug!("Toggle author monitoring body: {detailed_author_body:?}");

        let request_props = self
          .request_props_from(
            event,
            RequestMethod::Put,
            Some(detailed_author_body),
            Some(format!("/{author_id}")),
            None,
          )
          .await;

        self
          .handle_request::<Value, ()>(request_props, |_, _| ())
          .await
      }
      Err(_) => {
        warn!("Request for detailed author body was interrupted");
        Ok(())
      }
    }
  }

  pub(in crate::network::readarr_network) async fn trigger_automatic_author_search(
    &mut self,
    author_id: i64,
  ) -> Result<Value> {
    let event = ReadarrEvent::TriggerAutomaticAuthorSearch(author_id);
    info!("Searching indexers for author with ID: {author_id}");
    let body = ReadarrCommandBody {
      name: "AuthorSearch".to_owned(),
      author_id: Some(author_id),
      ..ReadarrCommandBody::default()
    };

    let request_props = self
      .request_props_from(event, RequestMethod::Post, Some(body), None, None)
      .await;

    self
      .handle_request::<ReadarrCommandBody, Value>(request_props, |_, _| ())
      .await
  }
}
