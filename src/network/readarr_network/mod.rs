use anyhow::Result;
use log::info;
use serde_json::{Value, json};

use super::{NetworkEvent, NetworkResource};
use crate::models::readarr_models::{
  AddAuthorBody, AddReadarrRootFolderBody, DeleteParams, EditAuthorParams, ReadarrSerdeable,
  ReadarrTaskName,
};
use crate::models::servarr_models::{
  EditIndexerParams, MetadataProfile, QualityProfile, ReleaseDownloadBody, Tag,
};
use crate::network::{Network, RequestMethod};

mod blocklist;
mod downloads;
mod history;
mod indexers;
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
  ClearBlocklist,
  DeleteAuthor(DeleteParams),
  DeleteBlocklistItem(i64),
  DeleteBook(DeleteParams),
  DeleteBookFile(i64),
  DeleteDownload(i64),
  DeleteIndexer(i64),
  DeleteRootFolder(i64),
  DeleteTag(i64),
  DownloadRelease(ReleaseDownloadBody),
  EditAuthor(EditAuthorParams),
  EditIndexer(EditIndexerParams),
  GetAllIndexerSettings,
  GetAuthorDetails(i64),
  GetAuthorHistory(i64),
  GetAuthorReleases(i64),
  GetBlocklist,
  GetBookDetails(i64),
  GetBookEditions(i64),
  GetBookFiles(i64),
  GetBookHistory(i64, i64),
  GetBookReleases(i64),
  GetBooks(i64),
  GetDiskSpace,
  GetDownloads(u64),
  GetHistory(u64),
  GetHostConfig,
  GetIndexers,
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
  MarkHistoryItemAsFailed(i64),
  SearchNewAuthor(String),
  StartTask(ReadarrTaskName),
  TestAllIndexers,
  TestIndexer(i64),
  ToggleAuthorMonitoring(i64),
  ToggleBookMonitoring(i64),
  TriggerAutomaticAuthorSearch(i64),
  TriggerAutomaticBookSearch(i64),
  UpdateAllAuthors,
  UpdateAndScanAuthor(i64),
  UpdateDownloads,
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
      ReadarrEvent::DeleteBlocklistItem(_) => "/blocklist",
      ReadarrEvent::ClearBlocklist => "/blocklist/bulk",
      ReadarrEvent::GetBlocklist => "/blocklist?page=1&pageSize=10000",
      ReadarrEvent::DeleteBook(_)
      | ReadarrEvent::GetBookDetails(_)
      | ReadarrEvent::GetBooks(_)
      | ReadarrEvent::ToggleBookMonitoring(_) => "/book",
      ReadarrEvent::DeleteBookFile(_) | ReadarrEvent::GetBookFiles(_) => "/bookfile",
      ReadarrEvent::GetQueuedEvents
      | ReadarrEvent::StartTask(_)
      | ReadarrEvent::TriggerAutomaticAuthorSearch(_)
      | ReadarrEvent::TriggerAutomaticBookSearch(_)
      | ReadarrEvent::UpdateAllAuthors
      | ReadarrEvent::UpdateAndScanAuthor(_)
      | ReadarrEvent::UpdateDownloads => "/command",
      ReadarrEvent::GetHostConfig | ReadarrEvent::GetSecurityConfig => "/config/host",
      ReadarrEvent::GetAllIndexerSettings => "/config/indexer",
      ReadarrEvent::GetDiskSpace => "/diskspace",
      ReadarrEvent::GetBookEditions(_) => "/edition",
      ReadarrEvent::HealthCheck => "/health",
      ReadarrEvent::GetHistory(_) => "/history",
      ReadarrEvent::GetAuthorHistory(_) | ReadarrEvent::GetBookHistory(_, _) => "/history/author",
      ReadarrEvent::MarkHistoryItemAsFailed(_) => "/history/failed",
      ReadarrEvent::DeleteIndexer(_) | ReadarrEvent::EditIndexer(_) | ReadarrEvent::GetIndexers => {
        "/indexer"
      }
      ReadarrEvent::TestIndexer(_) => "/indexer/test",
      ReadarrEvent::TestAllIndexers => "/indexer/testall",
      ReadarrEvent::GetLogs(_) => "/log",
      ReadarrEvent::GetMetadataProfiles => "/metadataprofile",
      ReadarrEvent::GetQualityProfiles => "/qualityprofile",
      ReadarrEvent::DeleteDownload(_) | ReadarrEvent::GetDownloads(_) => "/queue",
      ReadarrEvent::DownloadRelease(_)
      | ReadarrEvent::GetAuthorReleases(_)
      | ReadarrEvent::GetBookReleases(_) => "/release",
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
      ReadarrEvent::ClearBlocklist => self
        .clear_readarr_blocklist()
        .await
        .map(ReadarrSerdeable::from),
      ReadarrEvent::DeleteAuthor(delete_author_params) => self
        .delete_author(delete_author_params)
        .await
        .map(ReadarrSerdeable::from),
      ReadarrEvent::DeleteBlocklistItem(blocklist_item_id) => self
        .delete_readarr_blocklist_item(blocklist_item_id)
        .await
        .map(ReadarrSerdeable::from),
      ReadarrEvent::DeleteBook(delete_book_params) => self
        .delete_book(delete_book_params)
        .await
        .map(ReadarrSerdeable::from),
      ReadarrEvent::DeleteBookFile(book_file_id) => self
        .delete_readarr_book_file(book_file_id)
        .await
        .map(ReadarrSerdeable::from),
      ReadarrEvent::DeleteDownload(download_id) => self
        .delete_readarr_download(download_id)
        .await
        .map(ReadarrSerdeable::from),
      ReadarrEvent::DeleteIndexer(indexer_id) => self
        .delete_readarr_indexer(indexer_id)
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
      ReadarrEvent::DownloadRelease(release_download_body) => self
        .download_readarr_release(release_download_body)
        .await
        .map(ReadarrSerdeable::from),
      ReadarrEvent::EditAuthor(edit_author_params) => self
        .edit_author(edit_author_params)
        .await
        .map(ReadarrSerdeable::from),
      ReadarrEvent::EditIndexer(edit_indexer_params) => self
        .edit_readarr_indexer(edit_indexer_params)
        .await
        .map(ReadarrSerdeable::from),
      ReadarrEvent::GetAllIndexerSettings => self
        .get_all_readarr_indexer_settings()
        .await
        .map(ReadarrSerdeable::from),
      ReadarrEvent::GetAuthorDetails(author_id) => self
        .get_author_details(author_id)
        .await
        .map(ReadarrSerdeable::from),
      ReadarrEvent::GetAuthorHistory(author_id) => self
        .get_readarr_author_history(author_id)
        .await
        .map(ReadarrSerdeable::from),
      ReadarrEvent::GetAuthorReleases(author_id) => self
        .get_author_releases(author_id)
        .await
        .map(ReadarrSerdeable::from),
      ReadarrEvent::GetBlocklist => self
        .get_readarr_blocklist()
        .await
        .map(ReadarrSerdeable::from),
      ReadarrEvent::GetBookDetails(book_id) => self
        .get_book_details(book_id)
        .await
        .map(ReadarrSerdeable::from),
      ReadarrEvent::GetBookEditions(book_id) => self
        .get_book_editions(book_id)
        .await
        .map(ReadarrSerdeable::from),
      ReadarrEvent::GetBookFiles(book_id) => self
        .get_book_files(book_id)
        .await
        .map(ReadarrSerdeable::from),
      ReadarrEvent::GetBookHistory(author_id, book_id) => self
        .get_readarr_book_history(author_id, book_id)
        .await
        .map(ReadarrSerdeable::from),
      ReadarrEvent::GetBookReleases(book_id) => self
        .get_book_releases(book_id)
        .await
        .map(ReadarrSerdeable::from),
      ReadarrEvent::GetBooks(author_id) => {
        self.get_books(author_id).await.map(ReadarrSerdeable::from)
      }
      ReadarrEvent::GetDiskSpace => self
        .get_readarr_diskspace()
        .await
        .map(ReadarrSerdeable::from),
      ReadarrEvent::GetDownloads(count) => self
        .get_readarr_downloads(count)
        .await
        .map(ReadarrSerdeable::from),
      ReadarrEvent::GetHistory(events) => self
        .get_readarr_history(events)
        .await
        .map(ReadarrSerdeable::from),
      ReadarrEvent::GetHostConfig => self
        .get_readarr_host_config()
        .await
        .map(ReadarrSerdeable::from),
      ReadarrEvent::GetIndexers => self
        .get_readarr_indexers()
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
      ReadarrEvent::MarkHistoryItemAsFailed(history_item_id) => self
        .mark_readarr_history_item_as_failed(history_item_id)
        .await
        .map(ReadarrSerdeable::from),
      ReadarrEvent::SearchNewAuthor(query) => {
        self.search_author(query).await.map(ReadarrSerdeable::from)
      }
      ReadarrEvent::StartTask(task_name) => self
        .start_readarr_task(task_name)
        .await
        .map(ReadarrSerdeable::from),
      ReadarrEvent::TestAllIndexers => self
        .test_all_readarr_indexers()
        .await
        .map(ReadarrSerdeable::from),
      ReadarrEvent::TestIndexer(indexer_id) => self
        .test_readarr_indexer(indexer_id)
        .await
        .map(ReadarrSerdeable::from),
      ReadarrEvent::ToggleAuthorMonitoring(author_id) => self
        .toggle_author_monitoring(author_id)
        .await
        .map(ReadarrSerdeable::from),
      ReadarrEvent::ToggleBookMonitoring(book_id) => self
        .toggle_book_monitoring(book_id)
        .await
        .map(ReadarrSerdeable::from),
      ReadarrEvent::TriggerAutomaticAuthorSearch(author_id) => self
        .trigger_automatic_author_search(author_id)
        .await
        .map(ReadarrSerdeable::from),
      ReadarrEvent::TriggerAutomaticBookSearch(book_id) => self
        .trigger_automatic_book_search(book_id)
        .await
        .map(ReadarrSerdeable::from),
      ReadarrEvent::UpdateAllAuthors => self.update_all_authors().await.map(ReadarrSerdeable::from),
      ReadarrEvent::UpdateAndScanAuthor(author_id) => self
        .update_and_scan_author(author_id)
        .await
        .map(ReadarrSerdeable::from),
      ReadarrEvent::UpdateDownloads => self
        .update_readarr_downloads()
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
