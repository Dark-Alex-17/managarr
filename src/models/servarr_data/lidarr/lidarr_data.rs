use bimap::BiMap;
use chrono::{DateTime, Utc};
use strum::EnumIter;

use crate::{
  app::{
    context_clues::{
      build_context_clue_string, BLOCKLIST_CONTEXT_CLUES, DOWNLOADS_CONTEXT_CLUES,
      INDEXERS_CONTEXT_CLUES, ROOT_FOLDERS_CONTEXT_CLUES, SYSTEM_CONTEXT_CLUES,
    },
    lidarr::lidarr_context_clues::{
      HISTORY_CONTEXT_CLUES, ARTISTS_CONTEXT_CLUES, ARTIST_DETAILS_CONTEXT_CLUES,
      ARTIST_HISTORY_CONTEXT_CLUES,
    },
  },
  models::{
    servarr_data::modals::{EditIndexerModal, IndexerTestResultModalItem},
    servarr_models::{DiskSpace, Indexer, QueueEvent, RootFolder},
    lidarr_models::{
      AddArtistSearchResult, BlocklistItem, DownloadRecord, IndexerSettings, Album, Artist,
      LidarrHistoryItem, LidarrTask,
    },
    stateful_list::StatefulList,
    stateful_table::StatefulTable,
    BlockSelectionState, HorizontallyScrollableText, Route, ScrollableText, TabRoute, TabState,
  },
  network::lidarr_network::LidarrEvent,
};
use crate::models::servarr_data::lidarr::modals::add_artist_modal::AddArtistModal;
use crate::models::servarr_data::lidarr::modals::album_details_modal::AlbumDetailsModal;
use crate::models::servarr_data::lidarr::modals::edit_artist_modal::EditArtistModal;

pub struct LidarrData<'a> {
  pub add_list_exclusion: bool,
  pub add_searched_artist: Option<StatefulTable<AddArtistSearchResult>>,
  pub add_artist_modal: Option<AddArtistModal>,
  pub add_artist_search: Option<HorizontallyScrollableText>,
  pub blocklist: StatefulTable<BlocklistItem>,
  pub delete_artist_files: bool,
  pub downloads: StatefulTable<DownloadRecord>,
  pub disk_space_vec: Vec<DiskSpace>,
  pub edit_indexer_modal: Option<EditIndexerModal>,
  pub edit_root_folder: Option<HorizontallyScrollableText>,
  pub edit_artist_modal: Option<EditArtistModal>,
  pub history: StatefulTable<LidarrHistoryItem>,
  pub indexers: StatefulTable<Indexer>,
  pub indexer_settings: Option<IndexerSettings>,
  pub indexer_test_all_results: Option<StatefulTable<IndexerTestResultModalItem>>,
  pub indexer_test_errors: Option<String>,
  pub metadata_profiles_map: BiMap<i64, String>,
  pub logs: StatefulList<HorizontallyScrollableText>,
  pub log_details: StatefulList<HorizontallyScrollableText>,
  pub main_tabs: TabState,
  pub prompt_confirm: bool,
  pub prompt_confirm_action: Option<LidarrEvent>,
  pub quality_profile_map: BiMap<i64, String>,
  pub queued_events: StatefulTable<QueueEvent>,
  pub root_folders: StatefulTable<RootFolder>,
  pub albums: StatefulTable<Album>,
  pub album_details_modal: Option<AlbumDetailsModal>,
  pub selected_block: BlockSelectionState<'a, ActiveLidarrBlock>,
  pub artists: StatefulTable<Artist>,
  pub artist_history: Option<StatefulTable<LidarrHistoryItem>>,
  pub artist_info_tabs: TabState,
  pub start_time: DateTime<Utc>,
  pub tags_map: BiMap<i64, String>,
  pub tasks: StatefulTable<LidarrTask>,
  pub updates: ScrollableText,
  pub version: String,
}

impl LidarrData<'_> {
  pub fn reset_delete_artist_preferences(&mut self) {
    self.delete_artist_files = false;
    self.add_list_exclusion = false;
  }

  pub fn reset_artist_info_tabs(&mut self) {
    self.artist_history = None;
    self.albums = StatefulTable::default();
    self.artist_info_tabs.index = 0;
  }
}

impl<'a> Default for LidarrData<'a> {
  fn default() -> LidarrData<'a> {
    LidarrData {
      add_list_exclusion: false,
      add_searched_artist: None,
      add_artist_search: None,
      add_artist_modal: None,
      blocklist: StatefulTable::default(),
      downloads: StatefulTable::default(),
      delete_artist_files: false,
      disk_space_vec: Vec::new(),
      edit_indexer_modal: None,
      edit_root_folder: None,
      edit_artist_modal: None,
      history: StatefulTable::default(),
      indexers: StatefulTable::default(),
      indexer_settings: None,
      indexer_test_errors: None,
      indexer_test_all_results: None,
      metadata_profiles_map: BiMap::new(),
      logs: StatefulList::default(),
      log_details: StatefulList::default(),
      prompt_confirm: false,
      prompt_confirm_action: None,
      quality_profile_map: BiMap::new(),
      queued_events: StatefulTable::default(),
      root_folders: StatefulTable::default(),
      albums: StatefulTable::default(),
      album_details_modal: None,
      selected_block: BlockSelectionState::default(),
      artists: StatefulTable::default(),
      artist_history: None,
      start_time: DateTime::default(),
      tags_map: BiMap::default(),
      tasks: StatefulTable::default(),
      updates: ScrollableText::default(),
      version: String::new(),
      main_tabs: TabState::new(vec![
        TabRoute {
          title: "Artists".to_string(),
          route: ActiveLidarrBlock::Artists.into(),
          help: String::new(),
          contextual_help: Some(build_context_clue_string(&ARTISTS_CONTEXT_CLUES)),
          config: None,
        },
        TabRoute {
          title: "Downloads".to_string(),
          route: ActiveLidarrBlock::Downloads.into(),
          help: String::new(),
          contextual_help: Some(build_context_clue_string(&DOWNLOADS_CONTEXT_CLUES)),
          config: None,
        },
        TabRoute {
          title: "Blocklist".to_string(),
          route: ActiveLidarrBlock::Blocklist.into(),
          help: String::new(),
          contextual_help: Some(build_context_clue_string(&BLOCKLIST_CONTEXT_CLUES)),
          config: None,
        },
        TabRoute {
          title: "History".to_string(),
          route: ActiveLidarrBlock::History.into(),
          help: String::new(),
          contextual_help: Some(build_context_clue_string(&HISTORY_CONTEXT_CLUES)),
          config: None,
        },
        TabRoute {
          title: "Root Folders".to_string(),
          route: ActiveLidarrBlock::RootFolders.into(),
          help: String::new(),
          contextual_help: Some(build_context_clue_string(&ROOT_FOLDERS_CONTEXT_CLUES)),
          config: None,
        },
        TabRoute {
          title: "Indexers".to_string(),
          route: ActiveLidarrBlock::Indexers.into(),
          help: String::new(),
          contextual_help: Some(build_context_clue_string(&INDEXERS_CONTEXT_CLUES)),
          config: None,
        },
        TabRoute {
          title: "System".to_string(),
          route: ActiveLidarrBlock::System.into(),
          help: String::new(),
          contextual_help: Some(build_context_clue_string(&SYSTEM_CONTEXT_CLUES)),
          config: None,
        },
      ]),
      artist_info_tabs: TabState::new(vec![
        TabRoute {
          title: "Albums".to_string(),
          route: ActiveLidarrBlock::ArtistDetails.into(),
          help: String::new(),
          contextual_help: Some(build_context_clue_string(&ARTIST_DETAILS_CONTEXT_CLUES)),
          config: None,
        },
        TabRoute {
          title: "History".to_string(),
          route: ActiveLidarrBlock::ArtistHistory.into(),
          help: String::new(),
          contextual_help: Some(build_context_clue_string(&ARTIST_HISTORY_CONTEXT_CLUES)),
          config: None,
        },
      ]),
    }
  }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default, EnumIter)]
pub enum ActiveLidarrBlock {
  AddRootFolderPrompt,
  AddArtistAlreadyInLibrary,
  AddArtistConfirmPrompt,
  AddArtistEmptySearchResults,
  AddArtistPrompt,
  AddArtistSearchInput,
  AddArtistSearchResults,
  AddArtistSelectMetadataProfile,
  AddArtistSelectMonitor,
  AddArtistSelectQualityProfile,
  AddArtistSelectRootFolder,
  AddArtistTagsInput,
  AllIndexerSettingsPrompt,
  AutomaticallySearchTrackPrompt,
  AutomaticallySearchAlbumPrompt,
  AutomaticallySearchArtistPrompt,
  Blocklist,
  BlocklistClearAllItemsPrompt,
  BlocklistItemDetails,
  BlocklistSortPrompt,
  DeleteBlocklistItemPrompt,
  DeleteDownloadPrompt,
  DeleteTrackFilePrompt,
  DeleteIndexerPrompt,
  DeleteRootFolderPrompt,
  DeleteArtistConfirmPrompt,
  DeleteArtistPrompt,
  DeleteArtistToggleAddListExclusion,
  DeleteArtistToggleDeleteFile,
  Downloads,
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
  EditArtistPrompt,
  EditArtistConfirmPrompt,
  EditArtistPathInput,
  EditArtistSelectMetadataProfile,
  EditArtistSelectQualityProfile,
  EditArtistTagsInput,
  EditArtistToggleMonitored,
  TrackDetails,
  TrackFile,
  TrackHistory,
  TrackHistoryDetails,
  TracksSortPrompt,
  FilterTracks,
  FilterTracksError,
  FilterHistory,
  FilterHistoryError,
  FilterArtists,
  FilterArtistsError,
  FilterArtistHistory,
  FilterArtistHistoryError,
  FilterAlbumHistory,
  FilterAlbumHistoryError,
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
  ManualTrackSearch,
  ManualTrackSearchSortPrompt,
  ManualTrackSearchConfirmPrompt,
  RootFolders,
  SearchTracks,
  SearchTracksError,
  SearchHistory,
  SearchHistoryError,
  SearchAlbum,
  SearchAlbumError,
  SearchArtists,
  SearchArtistsError,
  SearchArtistHistory,
  SearchArtistHistoryError,
  SearchAlbumHistory,
  SearchAlbumHistoryError,
  AlbumDetails,
  AlbumHistory,
  AlbumHistoryDetails,
  AlbumHistorySortPrompt,
  #[default]
  Artists,
  ArtistDetails,
  ArtistHistory,
  ArtistHistoryDetails,
  ArtistHistorySortPrompt,
  ArtistSortPrompt,
  System,
  SystemLogs,
  SystemQueuedEvents,
  SystemTasks,
  SystemTaskStartConfirmPrompt,
  SystemUpdates,
  TestAllIndexers,
  TestIndexer,
  UpdateAllArtistsPrompt,
  UpdateAndScanArtistPrompt,
  UpdateDownloadsPrompt,
}

pub static LIBRARY_BLOCKS: [ActiveLidarrBlock; 7] = [
  ActiveLidarrBlock::Artists,
  ActiveLidarrBlock::ArtistSortPrompt,
  ActiveLidarrBlock::SearchArtists,
  ActiveLidarrBlock::SearchArtistsError,
  ActiveLidarrBlock::FilterArtists,
  ActiveLidarrBlock::FilterArtistsError,
  ActiveLidarrBlock::UpdateAllArtistsPrompt,
];

pub static ARTIST_DETAILS_BLOCKS: [ActiveLidarrBlock; 12] = [
  ActiveLidarrBlock::ArtistDetails,
  ActiveLidarrBlock::ArtistHistory,
  ActiveLidarrBlock::SearchAlbum,
  ActiveLidarrBlock::SearchAlbumError,
  ActiveLidarrBlock::UpdateAndScanArtistPrompt,
  ActiveLidarrBlock::AutomaticallySearchArtistPrompt,
  ActiveLidarrBlock::SearchArtistHistory,
  ActiveLidarrBlock::SearchArtistHistoryError,
  ActiveLidarrBlock::FilterArtistHistory,
  ActiveLidarrBlock::FilterArtistHistoryError,
  ActiveLidarrBlock::ArtistHistorySortPrompt,
  ActiveLidarrBlock::ArtistHistoryDetails,
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

pub static TRACK_DETAILS_BLOCKS: [ActiveLidarrBlock; 8] = [
  ActiveLidarrBlock::TrackDetails,
  ActiveLidarrBlock::TrackHistory,
  ActiveLidarrBlock::TrackHistoryDetails,
  ActiveLidarrBlock::TrackFile,
  ActiveLidarrBlock::ManualTrackSearch,
  ActiveLidarrBlock::ManualTrackSearchConfirmPrompt,
  ActiveLidarrBlock::ManualTrackSearchSortPrompt,
  ActiveLidarrBlock::AutomaticallySearchTrackPrompt,
];

pub static ADD_ARTIST_BLOCKS: [ActiveLidarrBlock; 11] = [
  ActiveLidarrBlock::AddArtistAlreadyInLibrary,
  ActiveLidarrBlock::AddArtistConfirmPrompt,
  ActiveLidarrBlock::AddArtistEmptySearchResults,
  ActiveLidarrBlock::AddArtistPrompt,
  ActiveLidarrBlock::AddArtistSearchInput,
  ActiveLidarrBlock::AddArtistSearchResults,
  ActiveLidarrBlock::AddArtistSelectMetadataProfile,
  ActiveLidarrBlock::AddArtistSelectMonitor,
  ActiveLidarrBlock::AddArtistSelectQualityProfile,
  ActiveLidarrBlock::AddArtistSelectRootFolder,
  ActiveLidarrBlock::AddArtistTagsInput,
];

pub const ADD_ARTIST_SELECTION_BLOCKS: &[&[ActiveLidarrBlock]] = &[
  &[ActiveLidarrBlock::AddArtistSelectRootFolder],
  &[ActiveLidarrBlock::AddArtistSelectMonitor],
  &[ActiveLidarrBlock::AddArtistSelectQualityProfile],
  &[ActiveLidarrBlock::AddArtistSelectMetadataProfile],
  &[ActiveLidarrBlock::AddArtistTagsInput],
  &[ActiveLidarrBlock::AddArtistConfirmPrompt],
];

pub static BLOCKLIST_BLOCKS: [ActiveLidarrBlock; 5] = [
  ActiveLidarrBlock::Blocklist,
  ActiveLidarrBlock::BlocklistItemDetails,
  ActiveLidarrBlock::DeleteBlocklistItemPrompt,
  ActiveLidarrBlock::BlocklistClearAllItemsPrompt,
  ActiveLidarrBlock::BlocklistSortPrompt,
];

pub static EDIT_ARTIST_BLOCKS: [ActiveLidarrBlock; 7] = [
  ActiveLidarrBlock::EditArtistPrompt,
  ActiveLidarrBlock::EditArtistConfirmPrompt,
  ActiveLidarrBlock::EditArtistPathInput,
  ActiveLidarrBlock::EditArtistSelectMetadataProfile,
  ActiveLidarrBlock::EditArtistSelectQualityProfile,
  ActiveLidarrBlock::EditArtistTagsInput,
  ActiveLidarrBlock::EditArtistToggleMonitored,
];

pub static EDIT_ARTIST_SELECTION_BLOCKS: &[&[ActiveLidarrBlock]] = &[
  &[ActiveLidarrBlock::EditArtistToggleMonitored],
  &[ActiveLidarrBlock::EditArtistSelectQualityProfile],
  &[ActiveLidarrBlock::EditArtistSelectMetadataProfile],
  &[ActiveLidarrBlock::EditArtistPathInput],
  &[ActiveLidarrBlock::EditArtistTagsInput],
  &[ActiveLidarrBlock::EditArtistConfirmPrompt],
];

pub static DOWNLOADS_BLOCKS: [ActiveLidarrBlock; 3] = [
  ActiveLidarrBlock::Downloads,
  ActiveLidarrBlock::DeleteDownloadPrompt,
  ActiveLidarrBlock::UpdateDownloadsPrompt,
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

pub static EDIT_INDEXER_BLOCKS: [ActiveLidarrBlock; 11] = [
  ActiveLidarrBlock::EditIndexerPrompt,
  ActiveLidarrBlock::EditIndexerConfirmPrompt,
  ActiveLidarrBlock::EditIndexerApiKeyInput,
  ActiveLidarrBlock::EditIndexerNameInput,
  ActiveLidarrBlock::EditIndexerSeedRatioInput,
  ActiveLidarrBlock::EditIndexerToggleEnableRss,
  ActiveLidarrBlock::EditIndexerToggleEnableAutomaticSearch,
  ActiveLidarrBlock::EditIndexerToggleEnableInteractiveSearch,
  ActiveLidarrBlock::EditIndexerUrlInput,
  ActiveLidarrBlock::EditIndexerPriorityInput,
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

pub static HISTORY_BLOCKS: [ActiveLidarrBlock; 7] = [
  ActiveLidarrBlock::History,
  ActiveLidarrBlock::HistoryItemDetails,
  ActiveLidarrBlock::HistorySortPrompt,
  ActiveLidarrBlock::FilterHistory,
  ActiveLidarrBlock::FilterHistoryError,
  ActiveLidarrBlock::SearchHistory,
  ActiveLidarrBlock::SearchHistoryError,
];

pub static ROOT_FOLDERS_BLOCKS: [ActiveLidarrBlock; 3] = [
  ActiveLidarrBlock::RootFolders,
  ActiveLidarrBlock::AddRootFolderPrompt,
  ActiveLidarrBlock::DeleteRootFolderPrompt,
];

pub static INDEXERS_BLOCKS: [ActiveLidarrBlock; 3] = [
  ActiveLidarrBlock::DeleteIndexerPrompt,
  ActiveLidarrBlock::Indexers,
  ActiveLidarrBlock::TestIndexer,
];

pub static SYSTEM_DETAILS_BLOCKS: [ActiveLidarrBlock; 5] = [
  ActiveLidarrBlock::SystemLogs,
  ActiveLidarrBlock::SystemQueuedEvents,
  ActiveLidarrBlock::SystemTasks,
  ActiveLidarrBlock::SystemTaskStartConfirmPrompt,
  ActiveLidarrBlock::SystemUpdates,
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
