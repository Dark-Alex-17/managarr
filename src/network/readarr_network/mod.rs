use anyhow::Result;
use log::info;
use serde_json::{Value, json};

use super::{NetworkEvent, NetworkResource};
use crate::models::readarr_models::{
  AddAuthorBody, AddReadarrRootFolderBody, DeleteParams, EditAuthorParams, ReadarrSerdeable,
  ReadarrTaskName,
};
use crate::models::servarr_models::{MetadataProfile, QualityProfile, Tag};
use crate::network::{Network, RequestMethod};

mod library;
mod root_folders;
mod system;

#[cfg(test)]
#[path = "readarr_network_tests.rs"]
mod readarr_network_tests;

#[cfg(test)]
#[path = "readarr_network_test_utils.rs"]
pub mod readarr_network_test_utils;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum ReadarrEvent {
  AddAuthor(AddAuthorBody),
  AddRootFolder(AddReadarrRootFolderBody),
  AddTag(String),
  DeleteAuthor(DeleteParams),
  DeleteRootFolder(i64),
  DeleteTag(i64),
  EditAuthor(EditAuthorParams),
  GetAuthorDetails(i64),
  GetDiskSpace,
  GetHostConfig,
  GetLogs(u64),
  GetMetadataProfiles,
  GetQualityProfiles,
  GetQueuedEvents,
  GetRootFolders,
  GetSecurityConfig,
  GetStatus,
  GetTags,
  GetTasks,
  GetUpdates,
  HealthCheck,
  ListAuthors,
  SearchNewAuthor(String),
  StartTask(ReadarrTaskName),
  ToggleAuthorMonitoring(i64),
  TriggerAutomaticAuthorSearch(i64),
  UpdateAndScanAuthor(i64),
}

impl NetworkResource for ReadarrEvent {
  fn resource(&self) -> &'static str {
    match &self {
      ReadarrEvent::AddAuthor(_)
      | ReadarrEvent::DeleteAuthor(_)
      | ReadarrEvent::EditAuthor(_)
      | ReadarrEvent::GetAuthorDetails(_)
      | ReadarrEvent::ListAuthors
      | ReadarrEvent::ToggleAuthorMonitoring(_) => "/author",
      ReadarrEvent::SearchNewAuthor(_) => "/author/lookup",
      ReadarrEvent::GetQueuedEvents
      | ReadarrEvent::StartTask(_)
      | ReadarrEvent::TriggerAutomaticAuthorSearch(_)
      | ReadarrEvent::UpdateAndScanAuthor(_) => "/command",
      ReadarrEvent::GetHostConfig | ReadarrEvent::GetSecurityConfig => "/config/host",
      ReadarrEvent::GetDiskSpace => "/diskspace",
      ReadarrEvent::HealthCheck => "/health",
      ReadarrEvent::GetLogs(_) => "/log",
      ReadarrEvent::GetMetadataProfiles => "/metadataprofile",
      ReadarrEvent::GetQualityProfiles => "/qualityprofile",
      ReadarrEvent::AddRootFolder(_)
      | ReadarrEvent::DeleteRootFolder(_)
      | ReadarrEvent::GetRootFolders => "/rootfolder",
      ReadarrEvent::GetStatus => "/system/status",
      ReadarrEvent::GetTasks => "/system/task",
      ReadarrEvent::AddTag(_) | ReadarrEvent::DeleteTag(_) | ReadarrEvent::GetTags => "/tag",
      ReadarrEvent::GetUpdates => "/update",
    }
  }
}

impl From<ReadarrEvent> for NetworkEvent {
  fn from(readarr_event: ReadarrEvent) -> Self {
    NetworkEvent::Readarr(readarr_event)
  }
}

impl Network<'_, '_> {
  pub async fn handle_readarr_event(
    &mut self,
    readarr_event: ReadarrEvent,
  ) -> Result<ReadarrSerdeable> {
    match readarr_event {
      ReadarrEvent::AddAuthor(body) => self.add_author(body).await.map(ReadarrSerdeable::from),
      ReadarrEvent::AddRootFolder(add_root_folder_body) => self
        .add_readarr_root_folder(add_root_folder_body)
        .await
        .map(ReadarrSerdeable::from),
      ReadarrEvent::AddTag(tag) => self.add_readarr_tag(tag).await.map(ReadarrSerdeable::from),
      ReadarrEvent::DeleteAuthor(delete_author_params) => self
        .delete_author(delete_author_params)
        .await
        .map(ReadarrSerdeable::from),
      ReadarrEvent::DeleteRootFolder(root_folder_id) => self
        .delete_readarr_root_folder(root_folder_id)
        .await
        .map(ReadarrSerdeable::from),
      ReadarrEvent::DeleteTag(tag_id) => self
        .delete_readarr_tag(tag_id)
        .await
        .map(ReadarrSerdeable::from),
      ReadarrEvent::EditAuthor(edit_author_params) => self
        .edit_author(edit_author_params)
        .await
        .map(ReadarrSerdeable::from),
      ReadarrEvent::GetAuthorDetails(author_id) => self
        .get_author_details(author_id)
        .await
        .map(ReadarrSerdeable::from),
      ReadarrEvent::GetDiskSpace => self
        .get_readarr_diskspace()
        .await
        .map(ReadarrSerdeable::from),
      ReadarrEvent::GetHostConfig => self
        .get_readarr_host_config()
        .await
        .map(ReadarrSerdeable::from),
      ReadarrEvent::GetLogs(events) => self
        .get_readarr_logs(events)
        .await
        .map(ReadarrSerdeable::from),
      ReadarrEvent::GetMetadataProfiles => self
        .get_readarr_metadata_profiles()
        .await
        .map(ReadarrSerdeable::from),
      ReadarrEvent::GetQualityProfiles => self
        .get_readarr_quality_profiles()
        .await
        .map(ReadarrSerdeable::from),
      ReadarrEvent::GetQueuedEvents => self
        .get_queued_readarr_events()
        .await
        .map(ReadarrSerdeable::from),
      ReadarrEvent::GetRootFolders => self
        .get_readarr_root_folders()
        .await
        .map(ReadarrSerdeable::from),
      ReadarrEvent::GetSecurityConfig => self
        .get_readarr_security_config()
        .await
        .map(ReadarrSerdeable::from),
      ReadarrEvent::GetStatus => self.get_readarr_status().await.map(ReadarrSerdeable::from),
      ReadarrEvent::GetTags => self.get_readarr_tags().await.map(ReadarrSerdeable::from),
      ReadarrEvent::GetTasks => self.get_readarr_tasks().await.map(ReadarrSerdeable::from),
      ReadarrEvent::GetUpdates => self.get_readarr_updates().await.map(ReadarrSerdeable::from),
      ReadarrEvent::HealthCheck => self
        .get_readarr_healthcheck()
        .await
        .map(ReadarrSerdeable::from),
      ReadarrEvent::ListAuthors => self.list_authors().await.map(ReadarrSerdeable::from),
      ReadarrEvent::SearchNewAuthor(query) => {
        self.search_author(query).await.map(ReadarrSerdeable::from)
      }
      ReadarrEvent::StartTask(task_name) => self
        .start_readarr_task(task_name)
        .await
        .map(ReadarrSerdeable::from),
      ReadarrEvent::ToggleAuthorMonitoring(author_id) => self
        .toggle_author_monitoring(author_id)
        .await
        .map(ReadarrSerdeable::from),
      ReadarrEvent::TriggerAutomaticAuthorSearch(author_id) => self
        .trigger_automatic_author_search(author_id)
        .await
        .map(ReadarrSerdeable::from),
      ReadarrEvent::UpdateAndScanAuthor(author_id) => self
        .update_and_scan_author(author_id)
        .await
        .map(ReadarrSerdeable::from),
    }
  }

  pub(in crate::network::readarr_network) async fn get_readarr_healthcheck(
    &mut self,
  ) -> Result<()> {
    info!("Performing Readarr health check");
    let event = ReadarrEvent::HealthCheck;

    let request_props = self
      .request_props_from(event, RequestMethod::Get, None::<()>, None, None)
      .await;

    self
      .handle_request::<(), ()>(request_props, |_, _| ())
      .await
  }

  async fn get_readarr_metadata_profiles(&mut self) -> Result<Vec<MetadataProfile>> {
    info!("Fetching Readarr metadata profiles");
    let event = ReadarrEvent::GetMetadataProfiles;

    let request_props = self
      .request_props_from(event, RequestMethod::Get, None::<()>, None, None)
      .await;

    self
      .handle_request::<(), Vec<MetadataProfile>>(request_props, |metadata_profiles, mut app| {
        app.data.readarr_data.metadata_profile_map = metadata_profiles
          .into_iter()
          .map(|profile| (profile.id, profile.name))
          .collect();
      })
      .await
  }

  async fn get_readarr_quality_profiles(&mut self) -> Result<Vec<QualityProfile>> {
    info!("Fetching Readarr quality profiles");
    let event = ReadarrEvent::GetQualityProfiles;

    let request_props = self
      .request_props_from(event, RequestMethod::Get, None::<()>, None, None)
      .await;

    self
      .handle_request::<(), Vec<QualityProfile>>(request_props, |quality_profiles, mut app| {
        app.data.readarr_data.quality_profile_map = quality_profiles
          .into_iter()
          .map(|profile| (profile.id, profile.name))
          .collect();
      })
      .await
  }

  async fn get_readarr_tags(&mut self) -> Result<Vec<Tag>> {
    info!("Fetching Readarr tags");
    let event = ReadarrEvent::GetTags;

    let request_props = self
      .request_props_from(event, RequestMethod::Get, None::<()>, None, None)
      .await;

    self
      .handle_request::<(), Vec<Tag>>(request_props, |tags_vec, mut app| {
        app.data.readarr_data.tags_map = tags_vec
          .into_iter()
          .map(|tag| (tag.id, tag.label))
          .collect();
      })
      .await
  }

  async fn add_readarr_tag(&mut self, tag: String) -> Result<Tag> {
    info!("Adding a new Readarr tag");
    let event = ReadarrEvent::AddTag(String::new());

    let request_props = self
      .request_props_from(
        event,
        RequestMethod::Post,
        Some(json!({ "label": tag })),
        None,
        None,
      )
      .await;

    self
      .handle_request::<Value, Tag>(request_props, |tag, mut app| {
        app.data.readarr_data.tags_map.insert(tag.id, tag.label);
      })
      .await
  }

  async fn delete_readarr_tag(&mut self, id: i64) -> Result<()> {
    info!("Deleting Readarr tag with ID: {id}");
    let event = ReadarrEvent::DeleteTag(id);

    let request_props = self
      .request_props_from(
        event,
        RequestMethod::Delete,
        None::<()>,
        Some(format!("/{id}")),
        None,
      )
      .await;

    self
      .handle_request::<(), ()>(request_props, |_, _| ())
      .await
  }

  pub(in crate::network::readarr_network) async fn extract_and_add_readarr_tag_ids_vec(
    &mut self,
    edit_tags: &str,
  ) -> Vec<i64> {
    let missing_tags_vec = {
      let tags_map = &self.app.lock().await.data.readarr_data.tags_map;
      edit_tags
        .split(',')
        .filter(|&tag| {
          !tag.is_empty() && tags_map.get_by_right(tag.to_lowercase().trim()).is_none()
        })
        .collect::<Vec<&str>>()
    };

    for tag in missing_tags_vec {
      self
        .add_readarr_tag(tag.trim().to_owned())
        .await
        .expect("Unable to add tag");
    }

    let app = self.app.lock().await;
    edit_tags
      .split(',')
      .filter(|tag| !tag.is_empty())
      .map(|tag| {
        *app
          .data
          .readarr_data
          .tags_map
          .get_by_right(tag.to_lowercase().trim())
          .unwrap()
      })
      .collect()
  }
}
