use crate::models::radarr_models::IndexerSettings;
use crate::models::servarr_data::modals::IndexerTestResultModalItem;
use crate::models::servarr_models::{EditIndexerParams, Indexer, IndexerTestResult};
use crate::models::stateful_table::StatefulTable;
use crate::network::radarr_network::RadarrEvent;
use crate::network::{Network, RequestMethod};
use anyhow::{Context, Result};
use log::{debug, info};
use serde_json::{Value, json};

#[cfg(test)]
#[path = "radarr_indexers_network_tests.rs"]
mod radarr_indexers_network_tests;

impl Network<'_, '_> {
  pub(in crate::network::radarr_network) async fn delete_radarr_indexer(
    &mut self,
    indexer_id: i64,
  ) -> Result<()> {
    let event = RadarrEvent::DeleteIndexer(indexer_id);
    info!("Deleting Radarr indexer for indexer with id: {indexer_id}");

    let request_props = self
      .request_props_from(
        event,
        RequestMethod::Delete,
        None::<()>,
        Some(format!("/{indexer_id}")),
        None,
      )
      .await;

    self
      .handle_request::<(), ()>(request_props, |_, _| ())
      .await
  }

  pub(in crate::network::radarr_network) async fn edit_all_radarr_indexer_settings(
    &mut self,
    params: IndexerSettings,
  ) -> Result<Value> {
    info!("Updating Radarr indexer settings");
    let event = RadarrEvent::EditAllIndexerSettings(IndexerSettings::default());

    debug!("Indexer settings body: {params:?}");

    let request_props = self
      .request_props_from(event, RequestMethod::Put, Some(params), None, None)
      .await;

    self
      .handle_request::<IndexerSettings, Value>(request_props, |_, _| {})
      .await
  }

  pub(in crate::network::radarr_network) async fn edit_radarr_indexer(
    &mut self,
    mut edit_indexer_params: EditIndexerParams,
  ) -> Result<()> {
    let detail_event = RadarrEvent::GetIndexers;
    let event = RadarrEvent::EditIndexer(EditIndexerParams::default());
    let id = edit_indexer_params.indexer_id;
    if let Some(tag_input_str) = edit_indexer_params.tag_input_string.as_ref() {
      let tag_ids_vec = self.extract_and_add_radarr_tag_ids_vec(tag_input_str).await;
      edit_indexer_params.tags = Some(tag_ids_vec);
    }
    info!("Updating Radarr indexer with ID: {id}");

    info!("Fetching indexer details for indexer with ID: {id}");

    let request_props = self
      .request_props_from(
        detail_event,
        RequestMethod::Get,
        None::<()>,
        Some(format!("/{id}")),
        None,
      )
      .await;

    let mut response = String::new();

    self
      .handle_request::<(), Value>(request_props, |detailed_indexer_body, _| {
        response = detailed_indexer_body.to_string()
      })
      .await?;

    info!("Constructing edit indexer body");

    let mut detailed_indexer_body: Value = serde_json::from_str(&response)?;

    let (
      name,
      enable_rss,
      enable_automatic_search,
      enable_interactive_search,
      url,
      api_key,
      seed_ratio,
      tags,
      priority,
    ) = {
      let priority = detailed_indexer_body["priority"]
        .as_i64()
        .context("Failed to deserialize indexer 'priority' field")?;
      let seed_ratio_field_option = detailed_indexer_body["fields"]
        .as_array()
        .context("Failed to get indexer 'fields' array")?
        .iter()
        .find(|field| field["name"] == "seedCriteria.seedRatio");
      let name = edit_indexer_params.name.unwrap_or(
        detailed_indexer_body["name"]
          .as_str()
          .context("Failed to deserialize indexer 'name' field")?
          .to_owned(),
      );
      let enable_rss = edit_indexer_params.enable_rss.unwrap_or(
        detailed_indexer_body["enableRss"]
          .as_bool()
          .context("Failed to deserialize indexer 'enableRss' field")?,
      );
      let enable_automatic_search = edit_indexer_params.enable_automatic_search.unwrap_or(
        detailed_indexer_body["enableAutomaticSearch"]
          .as_bool()
          .context("Failed to deserialize indexer 'enableAutomaticSearch' field")?,
      );
      let enable_interactive_search = edit_indexer_params.enable_interactive_search.unwrap_or(
        detailed_indexer_body["enableInteractiveSearch"]
          .as_bool()
          .context("Failed to deserialize indexer 'enableInteractiveSearch' field")?,
      );
      let url = edit_indexer_params.url.unwrap_or(
        detailed_indexer_body["fields"]
          .as_array()
          .context("Failed to get indexer 'fields' array for baseUrl")?
          .iter()
          .find(|field| field["name"] == "baseUrl")
          .context("Field 'baseUrl' was not found in the indexer fields array")?
          .get("value")
          .unwrap_or(&json!(""))
          .as_str()
          .context("Failed to deserialize indexer 'baseUrl' value")?
          .to_owned(),
      );
      let api_key = edit_indexer_params.api_key.unwrap_or(
        detailed_indexer_body["fields"]
          .as_array()
          .context("Failed to get indexer 'fields' array for apiKey")?
          .iter()
          .find(|field| field["name"] == "apiKey")
          .context("Field 'apiKey' was not found in the indexer fields array")?
          .get("value")
          .unwrap_or(&json!(""))
          .as_str()
          .context("Failed to deserialize indexer 'apiKey' value")?
          .to_owned(),
      );
      let seed_ratio = edit_indexer_params.seed_ratio.unwrap_or_else(|| {
        if let Some(seed_ratio_field) = seed_ratio_field_option {
          return seed_ratio_field
            .get("value")
            .unwrap_or(&json!(""))
            .as_str()
            .unwrap_or("")
            .to_owned();
        }

        String::new()
      });
      let tags = if edit_indexer_params.clear_tags {
        vec![]
      } else {
        edit_indexer_params.tags.unwrap_or(
          detailed_indexer_body["tags"]
            .as_array()
            .context("Failed to get indexer 'tags' array")?
            .iter()
            .map(|item| {
              item
                .as_i64()
                .context("Failed to deserialize indexer tag ID")
            })
            .collect::<Result<Vec<_>>>()?,
        )
      };
      let priority = edit_indexer_params.priority.unwrap_or(priority);

      (
        name,
        enable_rss,
        enable_automatic_search,
        enable_interactive_search,
        url,
        api_key,
        seed_ratio,
        tags,
        priority,
      )
    };

    *detailed_indexer_body
      .get_mut("name")
      .context("Failed to get mutable reference to indexer 'name' field")? = json!(name);
    *detailed_indexer_body
      .get_mut("priority")
      .context("Failed to get mutable reference to indexer 'priority' field")? = json!(priority);
    *detailed_indexer_body
      .get_mut("enableRss")
      .context("Failed to get mutable reference to indexer 'enableRss' field")? = json!(enable_rss);
    *detailed_indexer_body
      .get_mut("enableAutomaticSearch")
      .context("Failed to get mutable reference to indexer 'enableAutomaticSearch' field")? =
      json!(enable_automatic_search);
    *detailed_indexer_body
      .get_mut("enableInteractiveSearch")
      .context("Failed to get mutable reference to indexer 'enableInteractiveSearch' field")? =
      json!(enable_interactive_search);
    *detailed_indexer_body
      .get_mut("fields")
      .and_then(|f| f.as_array_mut())
      .context("Failed to get mutable reference to indexer 'fields' array")?
      .iter_mut()
      .find(|field| field["name"] == "baseUrl")
      .context("Failed to find 'baseUrl' field in indexer fields array")?
      .get_mut("value")
      .context("Failed to get mutable reference to 'baseUrl' value")? = json!(url);
    *detailed_indexer_body
      .get_mut("fields")
      .and_then(|f| f.as_array_mut())
      .context("Failed to get mutable reference to indexer 'fields' array for apiKey")?
      .iter_mut()
      .find(|field| field["name"] == "apiKey")
      .context("Failed to find 'apiKey' field in indexer fields array")?
      .get_mut("value")
      .context("Failed to get mutable reference to 'apiKey' value")? = json!(api_key);
    *detailed_indexer_body
      .get_mut("tags")
      .context("Failed to get mutable reference to indexer 'tags' field")? = json!(tags);
    let seed_ratio_field_option = detailed_indexer_body
      .get_mut("fields")
      .and_then(|f| f.as_array_mut())
      .context("Failed to get mutable reference to indexer 'fields' array for seed ratio")?
      .iter_mut()
      .find(|field| field["name"] == "seedCriteria.seedRatio");
    if let Some(seed_ratio_field) = seed_ratio_field_option {
      seed_ratio_field
        .as_object_mut()
        .context("Failed to get mutable reference to 'seedCriteria.seedRatio' object")?
        .insert("value".to_string(), json!(seed_ratio));
    }

    debug!("Edit indexer body: {detailed_indexer_body:?}");

    let request_props = self
      .request_props_from(
        event,
        RequestMethod::Put,
        Some(detailed_indexer_body),
        Some(format!("/{id}")),
        Some("forceSave=true".to_owned()),
      )
      .await;

    self
      .handle_request::<Value, ()>(request_props, |_, _| ())
      .await
  }

  pub(in crate::network::radarr_network) async fn get_radarr_indexers(
    &mut self,
  ) -> Result<Vec<Indexer>> {
    info!("Fetching Radarr indexers");
    let event = RadarrEvent::GetIndexers;

    let request_props = self
      .request_props_from(event, RequestMethod::Get, None::<()>, None, None)
      .await;

    self
      .handle_request::<(), Vec<Indexer>>(request_props, |indexers, mut app| {
        app.data.radarr_data.indexers.set_items(indexers);
      })
      .await
  }

  pub(in crate::network::radarr_network) async fn get_all_radarr_indexer_settings(
    &mut self,
  ) -> Result<IndexerSettings> {
    info!("Fetching Radarr indexer settings");
    let event = RadarrEvent::GetAllIndexerSettings;

    let request_props = self
      .request_props_from(event, RequestMethod::Get, None::<()>, None, None)
      .await;

    self
      .handle_request::<(), IndexerSettings>(request_props, |indexer_settings, mut app| {
        if app.data.radarr_data.indexer_settings.is_none() {
          app.data.radarr_data.indexer_settings = Some(indexer_settings);
        } else {
          debug!("Indexer Settings are being modified. Ignoring update...");
        }
      })
      .await
  }

  pub(in crate::network::radarr_network) async fn test_radarr_indexer(
    &mut self,
    indexer_id: i64,
  ) -> Result<Value> {
    let detail_event = RadarrEvent::GetIndexers;
    let event = RadarrEvent::TestIndexer(indexer_id);
    info!("Testing Radarr indexer with ID: {indexer_id}");

    info!("Fetching indexer details for indexer with ID: {indexer_id}");

    let request_props = self
      .request_props_from(
        detail_event,
        RequestMethod::Get,
        None::<()>,
        Some(format!("/{indexer_id}")),
        None,
      )
      .await;

    let mut test_body: Value = Value::default();

    self
      .handle_request::<(), Value>(request_props, |detailed_indexer_body, _| {
        test_body = detailed_indexer_body;
      })
      .await?;

    info!("Testing indexer");

    let mut request_props = self
      .request_props_from(event, RequestMethod::Post, Some(test_body), None, None)
      .await;
    request_props.ignore_status_code = true;

    self
      .handle_request::<Value, Value>(request_props, |test_results, mut app| {
        if test_results.as_object().is_none() {
          let error_message = test_results
            .as_array()
            .and_then(|arr| arr.first())
            .and_then(|item| item.get("errorMessage"))
            .map(|msg| msg.to_string())
            .unwrap_or_else(|| "Unknown indexer test error".to_string());
          app.data.radarr_data.indexer_test_errors = Some(error_message);
        } else {
          app.data.radarr_data.indexer_test_errors = Some(String::new());
        };
      })
      .await
  }

  pub(in crate::network::radarr_network) async fn test_all_radarr_indexers(
    &mut self,
  ) -> Result<Vec<IndexerTestResult>> {
    info!("Testing all Radarr indexers");
    let event = RadarrEvent::TestAllIndexers;

    let mut request_props = self
      .request_props_from(event, RequestMethod::Post, None, None, None)
      .await;
    request_props.ignore_status_code = true;

    let result = self
      .handle_request::<(), Vec<IndexerTestResult>>(request_props, |test_results, mut app| {
        let mut test_all_indexer_results = StatefulTable::default();
        let indexers = app.data.radarr_data.indexers.items.clone();
        let modal_test_results = test_results
          .iter()
          .map(|result| {
            let name = indexers
              .iter()
              .filter(|&indexer| indexer.id == result.id)
              .map(|indexer| indexer.name.clone())
              .nth(0)
              .unwrap_or_default();
            let validation_failures = result
              .validation_failures
              .iter()
              .map(|failure| {
                format!(
                  "Failure for field '{}': {}",
                  failure.property_name, failure.error_message
                )
              })
              .collect::<Vec<String>>()
              .join(", ");

            IndexerTestResultModalItem {
              name: name.unwrap_or_default(),
              is_valid: result.is_valid,
              validation_failures: validation_failures.into(),
            }
          })
          .collect();
        test_all_indexer_results.set_items(modal_test_results);
        app.data.radarr_data.indexer_test_all_results = Some(test_all_indexer_results);
      })
      .await;

    if result.is_err() {
      self
        .app
        .lock()
        .await
        .data
        .radarr_data
        .indexer_test_all_results = Some(StatefulTable::default());
    }

    result
  }
}
