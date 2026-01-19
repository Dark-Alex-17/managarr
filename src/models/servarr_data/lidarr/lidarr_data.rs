use serde_json::Number;

use super::modals::{AddArtistModal, AddRootFolderModal, AlbumDetailsModal, EditArtistModal};
use crate::app::context_clues::{
  DOWNLOADS_CONTEXT_CLUES, HISTORY_CONTEXT_CLUES, INDEXERS_CONTEXT_CLUES,
  ROOT_FOLDERS_CONTEXT_CLUES, SYSTEM_CONTEXT_CLUES,
};
use crate::app::lidarr::lidarr_context_clues::{
  ARTIST_DETAILS_CONTEXT_CLUES, ARTIST_HISTORY_CONTEXT_CLUES, ARTISTS_CONTEXT_CLUES,
  MANUAL_ARTIST_SEARCH_CONTEXT_CLUES,
};
use crate::models::lidarr_models::{LidarrRelease, LidarrTask};
use crate::models::servarr_data::modals::EditIndexerModal;
use crate::models::servarr_models::{IndexerSettings, QueueEvent};
use crate::models::stateful_list::StatefulList;
use crate::models::{
  BlockSelectionState, HorizontallyScrollableText, Route, ScrollableText, TabRoute, TabState,
  lidarr_models::{AddArtistSearchResult, Album, Artist, DownloadRecord, LidarrHistoryItem},
  servarr_data::modals::IndexerTestResultModalItem,
  servarr_models::{DiskSpace, Indexer, RootFolder},
  stateful_table::StatefulTable,
};
use crate::network::lidarr_network::LidarrEvent;
use bimap::BiMap;
use chrono::{DateTime, Utc};
use itertools::Itertools;
use strum::EnumIter;
#[cfg(test)]
use {
  super::modals::TrackDetailsModal,
  crate::models::lidarr_models::{MonitorType, NewItemMonitorType},
  crate::models::stateful_table::SortOption,
  crate::network::lidarr_network::lidarr_network_test_utils::test_utils::indexer_settings,
  crate::network::lidarr_network::lidarr_network_test_utils::test_utils::quality_profile_map,
  crate::network::lidarr_network::lidarr_network_test_utils::test_utils::{
    add_artist_search_result, album, artist, download_record, indexer, lidarr_history_item,
    metadata_profile, metadata_profile_map, quality_profile, root_folder, tags_map,
  },
  crate::network::lidarr_network::lidarr_network_test_utils::test_utils::{log_line, task},
  crate::network::lidarr_network::lidarr_network_test_utils::test_utils::{
    torrent_release, usenet_release,
  },
  crate::network::lidarr_network::lidarr_network_test_utils::test_utils::{track, track_file},
  crate::network::servarr_test_utils::diskspace,
  crate::network::servarr_test_utils::indexer_test_result,
  crate::network::servarr_test_utils::queued_event,
  crate::network::sonarr_network::sonarr_network_test_utils::test_utils::updates,
  crate::sort_option,
  strum::{Display, EnumString, IntoEnumIterator},
};

#[cfg(test)]
#[path = "lidarr_data_tests.rs"]
mod lidarr_data_tests;

pub struct LidarrData<'a> {
  pub add_artist_modal: Option<AddArtistModal>,
  pub add_artist_search: Option<HorizontallyScrollableText>,
  pub add_import_list_exclusion: bool,
  pub add_root_folder_modal: Option<AddRootFolderModal>,
  pub add_searched_artists: Option<StatefulTable<AddArtistSearchResult>>,
  pub albums: StatefulTable<Album>,
  pub album_details_modal: Option<AlbumDetailsModal>,
  pub artist_history: StatefulTable<LidarrHistoryItem>,
  pub artist_info_tabs: TabState,
  pub artists: StatefulTable<Artist>,
  pub delete_files: bool,
  pub discography_releases: StatefulTable<LidarrRelease>,
  pub disk_space_vec: Vec<DiskSpace>,
  pub downloads: StatefulTable<DownloadRecord>,
  pub edit_artist_modal: Option<EditArtistModal>,
  pub edit_indexer_modal: Option<EditIndexerModal>,
  pub history: StatefulTable<LidarrHistoryItem>,
  pub indexers: StatefulTable<Indexer>,
  pub indexer_settings: Option<IndexerSettings>,
  pub indexer_test_all_results: Option<StatefulTable<IndexerTestResultModalItem>>,
  pub indexer_test_errors: Option<String>,
  pub logs: StatefulList<HorizontallyScrollableText>,
  pub log_details: StatefulList<HorizontallyScrollableText>,
  pub main_tabs: TabState,
  pub metadata_profile_map: BiMap<i64, String>,
  pub prompt_confirm: bool,
  pub prompt_confirm_action: Option<LidarrEvent>,
  pub quality_profile_map: BiMap<i64, String>,
  pub queued_events: StatefulTable<QueueEvent>,
  pub root_folders: StatefulTable<RootFolder>,
  pub selected_block: BlockSelectionState<'a, ActiveLidarrBlock>,
  pub start_time: DateTime<Utc>,
  pub tags_map: BiMap<i64, String>,
  pub tasks: StatefulTable<LidarrTask>,
  pub updates: ScrollableText,
  pub version: String,
}

impl LidarrData<'_> {
  pub fn reset_delete_preferences(&mut self) {
    self.delete_files = false;
    self.add_import_list_exclusion = false;
  }

  pub fn reset_artist_info_tabs(&mut self) {
    self.albums = StatefulTable::default();
    self.discography_releases = StatefulTable::default();
    self.artist_history = StatefulTable::default();
    self.artist_info_tabs.index = 0;
  }

  pub fn tag_ids_to_display(&self, tag_ids: &[Number]) -> String {
    tag_ids
      .iter()
      .filter_map(|id| {
        let id = id.as_i64()?;
        self.tags_map.get_by_left(&id).cloned()
      })
      .collect::<Vec<String>>()
      .join(", ")
  }

  pub fn sorted_quality_profile_names(&self) -> Vec<String> {
    self
      .quality_profile_map
      .iter()
      .sorted_by_key(|(id, _)| *id)
      .map(|(_, name)| name)
      .cloned()
      .collect()
  }

  pub fn sorted_metadata_profile_names(&self) -> Vec<String> {
    self
      .metadata_profile_map
      .iter()
      .sorted_by_key(|(id, _)| *id)
      .map(|(_, name)| name)
      .cloned()
      .collect()
  }
}

impl<'a> Default for LidarrData<'a> {
  fn default() -> LidarrData<'a> {
    LidarrData {
      add_artist_modal: None,
      add_artist_search: None,
      add_import_list_exclusion: false,
      add_root_folder_modal: None,
      add_searched_artists: None,
      albums: StatefulTable::default(),
      album_details_modal: None,
      artist_history: StatefulTable::default(),
      artists: StatefulTable::default(),
      delete_files: false,
      discography_releases: StatefulTable::default(),
      disk_space_vec: Vec::new(),
      downloads: StatefulTable::default(),
      edit_artist_modal: None,
      edit_indexer_modal: None,
      history: StatefulTable::default(),
      indexers: StatefulTable::default(),
      indexer_settings: None,
      indexer_test_all_results: None,
      indexer_test_errors: None,
      logs: StatefulList::default(),
      log_details: StatefulList::default(),
      metadata_profile_map: BiMap::new(),
      prompt_confirm: false,
      prompt_confirm_action: None,
      quality_profile_map: BiMap::new(),
      queued_events: StatefulTable::default(),
      root_folders: StatefulTable::default(),
      selected_block: BlockSelectionState::default(),
      start_time: DateTime::default(),
      tags_map: BiMap::new(),
      tasks: StatefulTable::default(),
      updates: ScrollableText::default(),
      version: String::new(),
      main_tabs: TabState::new(vec![
        TabRoute {
          title: "Library".to_string(),
          route: ActiveLidarrBlock::Artists.into(),
          contextual_help: Some(&ARTISTS_CONTEXT_CLUES),
          config: None,
        },
        TabRoute {
          title: "Downloads".to_string(),
          route: ActiveLidarrBlock::Downloads.into(),
          contextual_help: Some(&DOWNLOADS_CONTEXT_CLUES),
          config: None,
        },
        TabRoute {
          title: "History".to_string(),
          route: ActiveLidarrBlock::History.into(),
          contextual_help: Some(&HISTORY_CONTEXT_CLUES),
          config: None,
        },
        TabRoute {
          title: "Root Folders".to_string(),
          route: ActiveLidarrBlock::RootFolders.into(),
          contextual_help: Some(&ROOT_FOLDERS_CONTEXT_CLUES),
          config: None,
        },
        TabRoute {
          title: "Indexers".to_string(),
          route: ActiveLidarrBlock::Indexers.into(),
          contextual_help: Some(&INDEXERS_CONTEXT_CLUES),
          config: None,
        },
        TabRoute {
          title: "System".to_string(),
          route: ActiveLidarrBlock::System.into(),
          contextual_help: Some(&SYSTEM_CONTEXT_CLUES),
          config: None,
        },
      ]),
      artist_info_tabs: TabState::new(vec![
        TabRoute {
          title: "Albums".to_string(),
          route: ActiveLidarrBlock::ArtistDetails.into(),
          contextual_help: Some(&ARTIST_DETAILS_CONTEXT_CLUES),
          config: None,
        },
        TabRoute {
          title: "History".to_string(),
          route: ActiveLidarrBlock::ArtistHistory.into(),
          contextual_help: Some(&ARTIST_HISTORY_CONTEXT_CLUES),
          config: None,
        },
        TabRoute {
          title: "Manual Search".to_string(),
          route: ActiveLidarrBlock::ManualArtistSearch.into(),
          contextual_help: Some(&MANUAL_ARTIST_SEARCH_CONTEXT_CLUES),
          config: None,
        },
      ]),
    }
  }
}

#[cfg(test)]
impl LidarrData<'_> {
  pub fn test_default_fully_populated() -> Self {
    let mut add_artist_modal = AddArtistModal {
      tags: "usenet, testing".into(),
      ..AddArtistModal::default()
    };
    add_artist_modal
      .monitor_list
      .set_items(Vec::from_iter(MonitorType::iter()));
    add_artist_modal
      .monitor_new_items_list
      .set_items(Vec::from_iter(NewItemMonitorType::iter()));
    add_artist_modal
      .metadata_profile_list
      .set_items(vec![metadata_profile().name]);
    add_artist_modal
      .quality_profile_list
      .set_items(vec![quality_profile().name]);
    add_artist_modal
      .root_folder_list
      .set_items(vec![root_folder()]);
    let mut edit_artist_modal = EditArtistModal {
      monitored: Some(true),
      path: "/nfs/music".into(),
      tags: "alex".into(),
      ..EditArtistModal::default()
    };
    edit_artist_modal
      .monitor_list
      .set_items(NewItemMonitorType::iter().collect());
    edit_artist_modal
      .quality_profile_list
      .set_items(vec![quality_profile().name]);
    edit_artist_modal
      .metadata_profile_list
      .set_items(vec![metadata_profile().name]);

    let mut add_root_folder_modal = AddRootFolderModal {
      name: "Test Root Folder".into(),
      path: "/nfs/music".into(),
      tags: "test".into(),
      ..AddRootFolderModal::default()
    };
    add_root_folder_modal
      .monitor_list
      .set_items(Vec::from_iter(MonitorType::iter()));
    add_root_folder_modal
      .monitor_new_items_list
      .set_items(Vec::from_iter(NewItemMonitorType::iter()));
    add_root_folder_modal
      .quality_profile_list
      .set_items(vec![quality_profile().name]);
    add_root_folder_modal
      .metadata_profile_list
      .set_items(vec![metadata_profile().name]);

    let mut track_details_modal = TrackDetailsModal::default();
    track_details_modal.track_details = ScrollableText::with_string("Some details".to_owned());
    track_details_modal
      .track_history
      .set_items(vec![lidarr_history_item()]);
    track_details_modal.track_history.search = Some("track history search".into());
    track_details_modal.track_history.filter = Some("track history filter".into());
    track_details_modal
      .track_history
      .sorting(vec![sort_option!(id)]);

    let mut album_details_modal = AlbumDetailsModal {
      track_details_modal: Some(track_details_modal),
      ..AlbumDetailsModal::default()
    };
    album_details_modal.tracks.set_items(vec![track()]);
    album_details_modal.tracks.search = Some("album search".into());
    album_details_modal
      .track_files
      .set_items(vec![track_file()]);
    album_details_modal
      .album_history
      .set_items(vec![lidarr_history_item()]);
    album_details_modal.album_history.search = Some("album history search".into());
    album_details_modal.album_history.filter = Some("album history filter".into());
    album_details_modal
      .album_history
      .sorting(vec![sort_option!(id)]);
    album_details_modal
      .album_releases
      .set_items(vec![torrent_release(), usenet_release()]);
    album_details_modal
      .album_releases
      .sorting(vec![sort_option!(indexer_id)]);

    let edit_indexer_modal = EditIndexerModal {
      name: "DrunkenSlug".into(),
      enable_rss: Some(true),
      enable_automatic_search: Some(true),
      enable_interactive_search: Some(true),
      url: "http://127.0.0.1:9696/1/".into(),
      api_key: "someApiKey".into(),
      seed_ratio: "ratio".into(),
      tags: "25".into(),
      priority: 1,
    };

    let mut indexer_test_all_results = StatefulTable::default();
    indexer_test_all_results.set_items(vec![indexer_test_result()]);

    let mut lidarr_data = LidarrData {
      album_details_modal: Some(album_details_modal),
      delete_files: true,
      disk_space_vec: vec![diskspace()],
      quality_profile_map: quality_profile_map(),
      metadata_profile_map: metadata_profile_map(),
      edit_artist_modal: Some(edit_artist_modal),
      edit_indexer_modal: Some(edit_indexer_modal),
      add_root_folder_modal: Some(add_root_folder_modal),
      add_artist_modal: Some(add_artist_modal),
      indexer_settings: Some(indexer_settings()),
      indexer_test_all_results: Some(indexer_test_all_results),
      indexer_test_errors: Some("error".to_string()),
      start_time: DateTime::from(DateTime::parse_from_rfc3339("2023-05-20T21:29:16Z").unwrap()),
      tags_map: tags_map(),
      updates: updates(),
      version: "1.2.3.4".to_owned(),
      ..LidarrData::default()
    };
    lidarr_data
      .artist_history
      .set_items(vec![lidarr_history_item()]);
    lidarr_data.artist_history.sorting(vec![sort_option!(id)]);
    lidarr_data.artist_history.search = Some("artist history search".into());
    lidarr_data.artist_history.filter = Some("artist history filter".into());
    lidarr_data.albums.set_items(vec![album()]);
    lidarr_data.albums.search = Some("album search".into());
    lidarr_data.artists.set_items(vec![artist()]);
    lidarr_data.artists.sorting(vec![SortOption {
      name: "Name",
      cmp_fn: Some(|a: &Artist, b: &Artist| a.artist_name.text.cmp(&b.artist_name.text)),
    }]);
    lidarr_data.artists.search = Some("artist search".into());
    lidarr_data.artists.filter = Some("artist filter".into());
    lidarr_data.downloads.set_items(vec![download_record()]);
    lidarr_data.history.set_items(vec![lidarr_history_item()]);
    lidarr_data.history.sorting(vec![SortOption {
      name: "Date",
      cmp_fn: Some(|a: &LidarrHistoryItem, b: &LidarrHistoryItem| a.date.cmp(&b.date)),
    }]);
    lidarr_data.history.search = Some("test search".into());
    lidarr_data.history.filter = Some("test filter".into());
    lidarr_data
      .discography_releases
      .set_items(vec![torrent_release(), usenet_release()]);
    lidarr_data
      .discography_releases
      .sorting(vec![sort_option!(indexer_id)]);
    lidarr_data.root_folders.set_items(vec![root_folder()]);
    lidarr_data.indexers.set_items(vec![indexer()]);
    lidarr_data.queued_events.set_items(vec![queued_event()]);
    lidarr_data.add_artist_search = Some("Test Artist".into());
    let mut add_searched_artists = StatefulTable::default();
    add_searched_artists.set_items(vec![add_artist_search_result()]);
    lidarr_data.add_searched_artists = Some(add_searched_artists);
    lidarr_data.logs.set_items(vec![log_line().into()]);
    lidarr_data.log_details.set_items(vec![log_line().into()]);
    lidarr_data.tasks.set_items(vec![task()]);

    lidarr_data
  }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default, EnumIter)]
#[cfg_attr(test, derive(Display, EnumString))]
pub enum ActiveLidarrBlock {
  #[default]
  Artists,
  ArtistDetails,
  ArtistHistory,
  ArtistHistoryDetails,
  ArtistHistorySortPrompt,
  ArtistsSortPrompt,
  AddArtistAlreadyInLibrary,
  AddArtistConfirmPrompt,
  AddArtistEmptySearchResults,
  AddArtistPrompt,
  AddArtistSearchInput,
  AddArtistSearchResults,
  AddArtistSelectMetadataProfile,
  AddArtistSelectMonitor,
  AddArtistSelectMonitorNewItems,
  AddArtistSelectQualityProfile,
  AddArtistSelectRootFolder,
  AddArtistTagsInput,
  AddRootFolderPrompt,
  AddRootFolderConfirmPrompt,
  AddRootFolderNameInput,
  AddRootFolderPathInput,
  AddRootFolderSelectMonitor,
  AddRootFolderSelectMonitorNewItems,
  AddRootFolderSelectQualityProfile,
  AddRootFolderSelectMetadataProfile,
  AddRootFolderTagsInput,
  AlbumDetails,
  AlbumHistory,
  AlbumHistoryDetails,
  AlbumHistorySortPrompt,
  AllIndexerSettingsPrompt,
  AutomaticallySearchAlbumPrompt,
  AutomaticallySearchArtistPrompt,
  DeleteAlbumPrompt,
  DeleteAlbumConfirmPrompt,
  DeleteAlbumToggleDeleteFile,
  DeleteAlbumToggleAddListExclusion,
  DeleteArtistPrompt,
  DeleteArtistConfirmPrompt,
  DeleteArtistToggleDeleteFile,
  DeleteArtistToggleAddListExclusion,
  DeleteTrackFilePrompt,
  DeleteDownloadPrompt,
  DeleteRootFolderPrompt,
  Downloads,
  EditArtistPrompt,
  EditArtistConfirmPrompt,
  EditArtistPathInput,
  EditArtistSelectMetadataProfile,
  EditArtistSelectMonitorNewItems,
  EditArtistSelectQualityProfile,
  EditArtistTagsInput,
  EditArtistToggleMonitored,
  EditIndexerPrompt,
  EditIndexerConfirmPrompt,
  EditIndexerApiKeyInput,
  EditIndexerNameInput,
  EditIndexerSeedRatioInput,
  EditIndexerToggleEnableRss,
  EditIndexerToggleEnableAutomaticSearch,
  EditIndexerToggleEnableInteractiveSearch,
  EditIndexerUrlInput,
  EditIndexerPriorityInput,
  EditIndexerTagsInput,
  DeleteIndexerPrompt,
  FilterAlbumHistory,
  FilterAlbumHistoryError,
  FilterArtists,
  FilterArtistsError,
  FilterHistory,
  FilterHistoryError,
  FilterArtistHistory,
  FilterArtistHistoryError,
  FilterTrackHistory,
  FilterTrackHistoryError,
  History,
  HistoryItemDetails,
  HistorySortPrompt,
  Indexers,
  IndexerSettingsConfirmPrompt,
  IndexerSettingsMaximumSizeInput,
  IndexerSettingsMinimumAgeInput,
  IndexerSettingsRetentionInput,
  IndexerSettingsRssSyncIntervalInput,
  ManualAlbumSearch,
  ManualAlbumSearchConfirmPrompt,
  ManualAlbumSearchSortPrompt,
  ManualArtistSearch,
  ManualArtistSearchConfirmPrompt,
  ManualArtistSearchSortPrompt,
  TestAllIndexers,
  TestIndexer,
  RootFolders,
  SearchAlbumHistory,
  SearchAlbumHistoryError,
  SearchAlbums,
  SearchAlbumsError,
  SearchArtists,
  SearchArtistsError,
  SearchHistory,
  SearchHistoryError,
  SearchArtistHistory,
  SearchArtistHistoryError,
  SearchTracks,
  SearchTracksError,
  SearchTrackHistory,
  SearchTrackHistoryError,
  System,
  SystemLogs,
  SystemQueuedEvents,
  SystemTasks,
  SystemTaskStartConfirmPrompt,
  SystemUpdates,
  TrackDetails,
  TrackHistory,
  TrackHistoryDetails,
  TrackHistorySortPrompt,
  UpdateAllArtistsPrompt,
  UpdateAndScanArtistPrompt,
  UpdateDownloadsPrompt,
}

pub static LIBRARY_BLOCKS: [ActiveLidarrBlock; 7] = [
  ActiveLidarrBlock::Artists,
  ActiveLidarrBlock::ArtistsSortPrompt,
  ActiveLidarrBlock::FilterArtists,
  ActiveLidarrBlock::FilterArtistsError,
  ActiveLidarrBlock::SearchArtists,
  ActiveLidarrBlock::SearchArtistsError,
  ActiveLidarrBlock::UpdateAllArtistsPrompt,
];

pub static ARTIST_DETAILS_BLOCKS: [ActiveLidarrBlock; 15] = [
  ActiveLidarrBlock::ArtistDetails,
  ActiveLidarrBlock::ArtistHistory,
  ActiveLidarrBlock::ArtistHistoryDetails,
  ActiveLidarrBlock::ArtistHistorySortPrompt,
  ActiveLidarrBlock::AutomaticallySearchArtistPrompt,
  ActiveLidarrBlock::FilterArtistHistory,
  ActiveLidarrBlock::FilterArtistHistoryError,
  ActiveLidarrBlock::ManualArtistSearch,
  ActiveLidarrBlock::ManualArtistSearchConfirmPrompt,
  ActiveLidarrBlock::ManualArtistSearchSortPrompt,
  ActiveLidarrBlock::SearchAlbums,
  ActiveLidarrBlock::SearchAlbumsError,
  ActiveLidarrBlock::SearchArtistHistory,
  ActiveLidarrBlock::SearchArtistHistoryError,
  ActiveLidarrBlock::UpdateAndScanArtistPrompt,
];

pub static ALBUM_DETAILS_BLOCKS: [ActiveLidarrBlock; 15] = [
  ActiveLidarrBlock::AlbumDetails,
  ActiveLidarrBlock::AlbumHistory,
  ActiveLidarrBlock::SearchTracks,
  ActiveLidarrBlock::SearchTracksError,
  ActiveLidarrBlock::AutomaticallySearchAlbumPrompt,
  ActiveLidarrBlock::SearchAlbumHistory,
  ActiveLidarrBlock::SearchAlbumHistoryError,
  ActiveLidarrBlock::FilterAlbumHistory,
  ActiveLidarrBlock::FilterAlbumHistoryError,
  ActiveLidarrBlock::AlbumHistorySortPrompt,
  ActiveLidarrBlock::AlbumHistoryDetails,
  ActiveLidarrBlock::ManualAlbumSearch,
  ActiveLidarrBlock::ManualAlbumSearchConfirmPrompt,
  ActiveLidarrBlock::ManualAlbumSearchSortPrompt,
  ActiveLidarrBlock::DeleteTrackFilePrompt,
];

pub static DOWNLOADS_BLOCKS: [ActiveLidarrBlock; 3] = [
  ActiveLidarrBlock::Downloads,
  ActiveLidarrBlock::DeleteDownloadPrompt,
  ActiveLidarrBlock::UpdateDownloadsPrompt,
];

pub static HISTORY_BLOCKS: [ActiveLidarrBlock; 7] = [
  ActiveLidarrBlock::History,
  ActiveLidarrBlock::HistoryItemDetails,
  ActiveLidarrBlock::HistorySortPrompt,
  ActiveLidarrBlock::SearchHistory,
  ActiveLidarrBlock::SearchHistoryError,
  ActiveLidarrBlock::FilterHistory,
  ActiveLidarrBlock::FilterHistoryError,
];

pub static ADD_ARTIST_BLOCKS: [ActiveLidarrBlock; 12] = [
  ActiveLidarrBlock::AddArtistAlreadyInLibrary,
  ActiveLidarrBlock::AddArtistConfirmPrompt,
  ActiveLidarrBlock::AddArtistEmptySearchResults,
  ActiveLidarrBlock::AddArtistPrompt,
  ActiveLidarrBlock::AddArtistSearchInput,
  ActiveLidarrBlock::AddArtistSearchResults,
  ActiveLidarrBlock::AddArtistSelectMetadataProfile,
  ActiveLidarrBlock::AddArtistSelectMonitor,
  ActiveLidarrBlock::AddArtistSelectMonitorNewItems,
  ActiveLidarrBlock::AddArtistSelectQualityProfile,
  ActiveLidarrBlock::AddArtistSelectRootFolder,
  ActiveLidarrBlock::AddArtistTagsInput,
];

pub const ADD_ARTIST_SELECTION_BLOCKS: &[&[ActiveLidarrBlock]] = &[
  &[ActiveLidarrBlock::AddArtistSelectRootFolder],
  &[ActiveLidarrBlock::AddArtistSelectMonitor],
  &[ActiveLidarrBlock::AddArtistSelectMonitorNewItems],
  &[ActiveLidarrBlock::AddArtistSelectQualityProfile],
  &[ActiveLidarrBlock::AddArtistSelectMetadataProfile],
  &[ActiveLidarrBlock::AddArtistTagsInput],
  &[ActiveLidarrBlock::AddArtistConfirmPrompt],
];

pub static DELETE_ARTIST_BLOCKS: [ActiveLidarrBlock; 4] = [
  ActiveLidarrBlock::DeleteArtistPrompt,
  ActiveLidarrBlock::DeleteArtistConfirmPrompt,
  ActiveLidarrBlock::DeleteArtistToggleDeleteFile,
  ActiveLidarrBlock::DeleteArtistToggleAddListExclusion,
];

pub const DELETE_ARTIST_SELECTION_BLOCKS: &[&[ActiveLidarrBlock]] = &[
  &[ActiveLidarrBlock::DeleteArtistToggleDeleteFile],
  &[ActiveLidarrBlock::DeleteArtistToggleAddListExclusion],
  &[ActiveLidarrBlock::DeleteArtistConfirmPrompt],
];

pub static DELETE_ALBUM_BLOCKS: [ActiveLidarrBlock; 4] = [
  ActiveLidarrBlock::DeleteAlbumPrompt,
  ActiveLidarrBlock::DeleteAlbumConfirmPrompt,
  ActiveLidarrBlock::DeleteAlbumToggleDeleteFile,
  ActiveLidarrBlock::DeleteAlbumToggleAddListExclusion,
];

pub const DELETE_ALBUM_SELECTION_BLOCKS: &[&[ActiveLidarrBlock]] = &[
  &[ActiveLidarrBlock::DeleteAlbumToggleDeleteFile],
  &[ActiveLidarrBlock::DeleteAlbumToggleAddListExclusion],
  &[ActiveLidarrBlock::DeleteAlbumConfirmPrompt],
];

pub static EDIT_ARTIST_BLOCKS: [ActiveLidarrBlock; 8] = [
  ActiveLidarrBlock::EditArtistPrompt,
  ActiveLidarrBlock::EditArtistConfirmPrompt,
  ActiveLidarrBlock::EditArtistPathInput,
  ActiveLidarrBlock::EditArtistSelectMetadataProfile,
  ActiveLidarrBlock::EditArtistSelectMonitorNewItems,
  ActiveLidarrBlock::EditArtistSelectQualityProfile,
  ActiveLidarrBlock::EditArtistTagsInput,
  ActiveLidarrBlock::EditArtistToggleMonitored,
];

pub const EDIT_ARTIST_SELECTION_BLOCKS: &[&[ActiveLidarrBlock]] = &[
  &[ActiveLidarrBlock::EditArtistToggleMonitored],
  &[ActiveLidarrBlock::EditArtistSelectMonitorNewItems],
  &[ActiveLidarrBlock::EditArtistSelectQualityProfile],
  &[ActiveLidarrBlock::EditArtistSelectMetadataProfile],
  &[ActiveLidarrBlock::EditArtistPathInput],
  &[ActiveLidarrBlock::EditArtistTagsInput],
  &[ActiveLidarrBlock::EditArtistConfirmPrompt],
];

pub const ROOT_FOLDERS_BLOCKS: [ActiveLidarrBlock; 2] = [
  ActiveLidarrBlock::RootFolders,
  ActiveLidarrBlock::DeleteRootFolderPrompt,
];

pub static ADD_ROOT_FOLDER_BLOCKS: [ActiveLidarrBlock; 9] = [
  ActiveLidarrBlock::AddRootFolderPrompt,
  ActiveLidarrBlock::AddRootFolderConfirmPrompt,
  ActiveLidarrBlock::AddRootFolderNameInput,
  ActiveLidarrBlock::AddRootFolderPathInput,
  ActiveLidarrBlock::AddRootFolderSelectMonitor,
  ActiveLidarrBlock::AddRootFolderSelectMonitorNewItems,
  ActiveLidarrBlock::AddRootFolderSelectQualityProfile,
  ActiveLidarrBlock::AddRootFolderSelectMetadataProfile,
  ActiveLidarrBlock::AddRootFolderTagsInput,
];

pub const ADD_ROOT_FOLDER_SELECTION_BLOCKS: &[&[ActiveLidarrBlock]] = &[
  &[ActiveLidarrBlock::AddRootFolderNameInput],
  &[ActiveLidarrBlock::AddRootFolderPathInput],
  &[ActiveLidarrBlock::AddRootFolderSelectMonitor],
  &[ActiveLidarrBlock::AddRootFolderSelectMonitorNewItems],
  &[ActiveLidarrBlock::AddRootFolderSelectQualityProfile],
  &[ActiveLidarrBlock::AddRootFolderSelectMetadataProfile],
  &[ActiveLidarrBlock::AddRootFolderTagsInput],
  &[ActiveLidarrBlock::AddRootFolderConfirmPrompt],
];

pub static EDIT_INDEXER_BLOCKS: [ActiveLidarrBlock; 11] = [
  ActiveLidarrBlock::EditIndexerPrompt,
  ActiveLidarrBlock::EditIndexerConfirmPrompt,
  ActiveLidarrBlock::EditIndexerApiKeyInput,
  ActiveLidarrBlock::EditIndexerNameInput,
  ActiveLidarrBlock::EditIndexerSeedRatioInput,
  ActiveLidarrBlock::EditIndexerToggleEnableRss,
  ActiveLidarrBlock::EditIndexerToggleEnableAutomaticSearch,
  ActiveLidarrBlock::EditIndexerToggleEnableInteractiveSearch,
  ActiveLidarrBlock::EditIndexerPriorityInput,
  ActiveLidarrBlock::EditIndexerUrlInput,
  ActiveLidarrBlock::EditIndexerTagsInput,
];

pub const EDIT_INDEXER_TORRENT_SELECTION_BLOCKS: &[&[ActiveLidarrBlock]] = &[
  &[
    ActiveLidarrBlock::EditIndexerNameInput,
    ActiveLidarrBlock::EditIndexerUrlInput,
  ],
  &[
    ActiveLidarrBlock::EditIndexerToggleEnableRss,
    ActiveLidarrBlock::EditIndexerApiKeyInput,
  ],
  &[
    ActiveLidarrBlock::EditIndexerToggleEnableAutomaticSearch,
    ActiveLidarrBlock::EditIndexerSeedRatioInput,
  ],
  &[
    ActiveLidarrBlock::EditIndexerToggleEnableInteractiveSearch,
    ActiveLidarrBlock::EditIndexerTagsInput,
  ],
  &[
    ActiveLidarrBlock::EditIndexerPriorityInput,
    ActiveLidarrBlock::EditIndexerConfirmPrompt,
  ],
  &[
    ActiveLidarrBlock::EditIndexerConfirmPrompt,
    ActiveLidarrBlock::EditIndexerConfirmPrompt,
  ],
];

pub const EDIT_INDEXER_NZB_SELECTION_BLOCKS: &[&[ActiveLidarrBlock]] = &[
  &[
    ActiveLidarrBlock::EditIndexerNameInput,
    ActiveLidarrBlock::EditIndexerUrlInput,
  ],
  &[
    ActiveLidarrBlock::EditIndexerToggleEnableRss,
    ActiveLidarrBlock::EditIndexerApiKeyInput,
  ],
  &[
    ActiveLidarrBlock::EditIndexerToggleEnableAutomaticSearch,
    ActiveLidarrBlock::EditIndexerTagsInput,
  ],
  &[
    ActiveLidarrBlock::EditIndexerToggleEnableInteractiveSearch,
    ActiveLidarrBlock::EditIndexerPriorityInput,
  ],
  &[
    ActiveLidarrBlock::EditIndexerConfirmPrompt,
    ActiveLidarrBlock::EditIndexerConfirmPrompt,
  ],
];

pub static INDEXER_SETTINGS_BLOCKS: [ActiveLidarrBlock; 6] = [
  ActiveLidarrBlock::AllIndexerSettingsPrompt,
  ActiveLidarrBlock::IndexerSettingsConfirmPrompt,
  ActiveLidarrBlock::IndexerSettingsMaximumSizeInput,
  ActiveLidarrBlock::IndexerSettingsMinimumAgeInput,
  ActiveLidarrBlock::IndexerSettingsRetentionInput,
  ActiveLidarrBlock::IndexerSettingsRssSyncIntervalInput,
];

pub const INDEXER_SETTINGS_SELECTION_BLOCKS: &[&[ActiveLidarrBlock]] = &[
  &[ActiveLidarrBlock::IndexerSettingsMinimumAgeInput],
  &[ActiveLidarrBlock::IndexerSettingsRetentionInput],
  &[ActiveLidarrBlock::IndexerSettingsMaximumSizeInput],
  &[ActiveLidarrBlock::IndexerSettingsRssSyncIntervalInput],
  &[ActiveLidarrBlock::IndexerSettingsConfirmPrompt],
];

pub static INDEXERS_BLOCKS: [ActiveLidarrBlock; 3] = [
  ActiveLidarrBlock::Indexers,
  ActiveLidarrBlock::DeleteIndexerPrompt,
  ActiveLidarrBlock::TestIndexer,
];

pub static SYSTEM_DETAILS_BLOCKS: [ActiveLidarrBlock; 5] = [
  ActiveLidarrBlock::SystemLogs,
  ActiveLidarrBlock::SystemQueuedEvents,
  ActiveLidarrBlock::SystemTasks,
  ActiveLidarrBlock::SystemTaskStartConfirmPrompt,
  ActiveLidarrBlock::SystemUpdates,
];

pub static TRACK_DETAILS_BLOCKS: [ActiveLidarrBlock; 8] = [
  ActiveLidarrBlock::TrackDetails,
  ActiveLidarrBlock::TrackHistory,
  ActiveLidarrBlock::TrackHistoryDetails,
  ActiveLidarrBlock::SearchTrackHistory,
  ActiveLidarrBlock::SearchTrackHistoryError,
  ActiveLidarrBlock::FilterTrackHistory,
  ActiveLidarrBlock::FilterTrackHistoryError,
  ActiveLidarrBlock::TrackHistorySortPrompt,
];

impl From<ActiveLidarrBlock> for Route {
  fn from(active_lidarr_block: ActiveLidarrBlock) -> Route {
    Route::Lidarr(active_lidarr_block, None)
  }
}

impl From<(ActiveLidarrBlock, Option<ActiveLidarrBlock>)> for Route {
  fn from(value: (ActiveLidarrBlock, Option<ActiveLidarrBlock>)) -> Route {
    Route::Lidarr(value.0, value.1)
  }
}
