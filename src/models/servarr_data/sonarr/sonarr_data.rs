use bimap::BiMap;
use chrono::{DateTime, Utc};
use strum::EnumIter;

use crate::{
  app::{
    context_clues::{
      build_context_clue_string, BLOCKLIST_CONTEXT_CLUES, DOWNLOADS_CONTEXT_CLUES,
      INDEXERS_CONTEXT_CLUES, ROOT_FOLDERS_CONTEXT_CLUES, SYSTEM_CONTEXT_CLUES,
    },
    sonarr::sonarr_context_clues::{
      HISTORY_CONTEXT_CLUES, SERIES_CONTEXT_CLUES, SERIES_DETAILS_CONTEXT_CLUES,
    },
  },
  models::{
    servarr_data::modals::{EditIndexerModal, IndexerTestResultModalItem},
    servarr_models::{DiskSpace, Indexer, QueueEvent, RootFolder},
    sonarr_models::{
      AddSeriesSearchResult, BlocklistItem, DownloadRecord, IndexerSettings, Season, Series,
      SonarrHistoryItem, SonarrTask,
    },
    stateful_list::StatefulList,
    stateful_table::StatefulTable,
    BlockSelectionState, HorizontallyScrollableText, Route, ScrollableText, TabRoute, TabState,
  },
  network::sonarr_network::SonarrEvent,
};

use super::modals::{AddSeriesModal, EditSeriesModal, SeasonDetailsModal};

#[cfg(test)]
#[path = "sonarr_data_tests.rs"]
mod sonarr_data_tests;

#[cfg(test)]
#[path = "sonarr_test_utils.rs"]
pub mod sonarr_test_utils;

pub struct SonarrData<'a> {
  pub add_list_exclusion: bool,
  pub add_searched_series: Option<StatefulTable<AddSeriesSearchResult>>,
  pub add_series_modal: Option<AddSeriesModal>,
  pub add_series_search: Option<HorizontallyScrollableText>,
  pub blocklist: StatefulTable<BlocklistItem>,
  pub delete_series_files: bool,
  pub downloads: StatefulTable<DownloadRecord>,
  pub disk_space_vec: Vec<DiskSpace>,
  pub edit_indexer_modal: Option<EditIndexerModal>,
  pub edit_root_folder: Option<HorizontallyScrollableText>,
  pub edit_series_modal: Option<EditSeriesModal>,
  pub history: StatefulTable<SonarrHistoryItem>,
  pub indexers: StatefulTable<Indexer>,
  pub indexer_settings: Option<IndexerSettings>,
  pub indexer_test_all_results: Option<StatefulTable<IndexerTestResultModalItem>>,
  pub indexer_test_error: Option<String>,
  pub language_profiles_map: BiMap<i64, String>,
  pub logs: StatefulList<HorizontallyScrollableText>,
  pub log_details: StatefulList<HorizontallyScrollableText>,
  pub main_tabs: TabState,
  pub prompt_confirm: bool,
  pub prompt_confirm_action: Option<SonarrEvent>,
  pub quality_profile_map: BiMap<i64, String>,
  pub queued_events: StatefulTable<QueueEvent>,
  pub root_folders: StatefulTable<RootFolder>,
  pub seasons: StatefulTable<Season>,
  pub season_details_modal: Option<SeasonDetailsModal>,
  pub selected_block: BlockSelectionState<'a, ActiveSonarrBlock>,
  pub series: StatefulTable<Series>,
  pub series_history: Option<StatefulTable<SonarrHistoryItem>>,
  pub series_info_tabs: TabState,
  pub start_time: DateTime<Utc>,
  pub tags_map: BiMap<i64, String>,
  pub tasks: StatefulTable<SonarrTask>,
  pub updates: ScrollableText,
  pub version: String,
}

impl<'a> SonarrData<'a> {
  pub fn reset_delete_series_preferences(&mut self) {
    self.delete_series_files = false;
    self.add_list_exclusion = false;
  }
}

impl<'a> Default for SonarrData<'a> {
  fn default() -> SonarrData<'a> {
    SonarrData {
      add_list_exclusion: false,
      add_searched_series: None,
      add_series_search: None,
      add_series_modal: None,
      blocklist: StatefulTable::default(),
      downloads: StatefulTable::default(),
      delete_series_files: false,
      disk_space_vec: Vec::new(),
      edit_indexer_modal: None,
      edit_root_folder: None,
      edit_series_modal: None,
      history: StatefulTable::default(),
      indexers: StatefulTable::default(),
      indexer_settings: None,
      indexer_test_error: None,
      indexer_test_all_results: None,
      language_profiles_map: BiMap::new(),
      logs: StatefulList::default(),
      log_details: StatefulList::default(),
      prompt_confirm: false,
      prompt_confirm_action: None,
      quality_profile_map: BiMap::new(),
      queued_events: StatefulTable::default(),
      root_folders: StatefulTable::default(),
      seasons: StatefulTable::default(),
      season_details_modal: None,
      selected_block: BlockSelectionState::default(),
      series: StatefulTable::default(),
      series_history: None,
      start_time: DateTime::default(),
      tags_map: BiMap::default(),
      tasks: StatefulTable::default(),
      updates: ScrollableText::default(),
      version: String::new(),
      main_tabs: TabState::new(vec![
        TabRoute {
          title: "Library",
          route: ActiveSonarrBlock::Series.into(),
          help: String::new(),
          contextual_help: Some(build_context_clue_string(&SERIES_CONTEXT_CLUES)),
        },
        TabRoute {
          title: "Downloads",
          route: ActiveSonarrBlock::Downloads.into(),
          help: String::new(),
          contextual_help: Some(build_context_clue_string(&DOWNLOADS_CONTEXT_CLUES)),
        },
        TabRoute {
          title: "Blocklist",
          route: ActiveSonarrBlock::Blocklist.into(),
          help: String::new(),
          contextual_help: Some(build_context_clue_string(&BLOCKLIST_CONTEXT_CLUES)),
        },
        TabRoute {
          title: "History",
          route: ActiveSonarrBlock::History.into(),
          help: String::new(),
          contextual_help: Some(build_context_clue_string(&HISTORY_CONTEXT_CLUES)),
        },
        TabRoute {
          title: "Root Folders",
          route: ActiveSonarrBlock::RootFolders.into(),
          help: String::new(),
          contextual_help: Some(build_context_clue_string(&ROOT_FOLDERS_CONTEXT_CLUES)),
        },
        TabRoute {
          title: "Indexers",
          route: ActiveSonarrBlock::Indexers.into(),
          help: String::new(),
          contextual_help: Some(build_context_clue_string(&INDEXERS_CONTEXT_CLUES)),
        },
        TabRoute {
          title: "System",
          route: ActiveSonarrBlock::System.into(),
          help: String::new(),
          contextual_help: Some(build_context_clue_string(&SYSTEM_CONTEXT_CLUES)),
        },
      ]),
      series_info_tabs: TabState::new(vec![
        TabRoute {
          title: "Seasons",
          route: ActiveSonarrBlock::SeriesDetails.into(),
          help: String::new(),
          contextual_help: Some(build_context_clue_string(&SERIES_DETAILS_CONTEXT_CLUES)),
        },
        TabRoute {
          title: "History",
          route: ActiveSonarrBlock::SeriesHistory.into(),
          help: String::new(),
          contextual_help: Some(build_context_clue_string(&HISTORY_CONTEXT_CLUES)),
        },
      ]),
    }
  }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default, EnumIter)]
pub enum ActiveSonarrBlock {
  AddRootFolderPrompt,
  AddSeriesAlreadyInLibrary,
  AddSeriesConfirmPrompt,
  AddSeriesEmptySearchResults,
  AddSeriesPrompt,
  AddSeriesSearchInput,
  AddSeriesSearchResults,
  AddSeriesSelectLanguageProfile,
  AddSeriesSelectMonitor,
  AddSeriesSelectQualityProfile,
  AddSeriesSelectRootFolder,
  AddSeriesSelectSeriesType,
  AddSeriesTagsInput,
  AddSeriesToggleUseSeasonFolder,
  AllIndexerSettingsPrompt,
  AutomaticallySearchEpisodePrompt,
  AutomaticallySearchSeasonPrompt,
  AutomaticallySearchSeriesPrompt,
  Blocklist,
  BlocklistClearAllItemsPrompt,
  BlocklistItemDetails,
  BlocklistSortPrompt,
  DeleteBlocklistItemPrompt,
  DeleteDownloadPrompt,
  DeleteEpisodeFilePrompt,
  DeleteIndexerPrompt,
  DeleteRootFolderPrompt,
  DeleteSeriesConfirmPrompt,
  DeleteSeriesPrompt,
  DeleteSeriesToggleAddListExclusion,
  DeleteSeriesToggleDeleteFile,
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
  EditSeriesPrompt,
  EditSeriesConfirmPrompt,
  EditSeriesPathInput,
  EditSeriesSelectSeriesType,
  EditSeriesSelectQualityProfile,
  EditSeriesSelectLanguageProfile,
  EditSeriesTagsInput,
  EditSeriesToggleMonitored,
  EditSeriesToggleSeasonFolder,
  EpisodeDetails,
  EpisodeFile,
  EpisodeHistory,
  EpisodesSortPrompt,
  FilterEpisodes,
  FilterEpisodesError,
  FilterHistory,
  FilterHistoryError,
  FilterSeries,
  FilterSeriesError,
  FilterSeriesHistory,
  FilterSeriesHistoryError,
  History,
  HistoryItemDetails,
  HistorySortPrompt,
  Indexers,
  IndexerSettingsConfirmPrompt,
  IndexerSettingsMaximumSizeInput,
  IndexerSettingsMinimumAgeInput,
  IndexerSettingsRetentionInput,
  IndexerSettingsRssSyncIntervalInput,
  ManualEpisodeSearch,
  ManualEpisodeSearchConfirmPrompt,
  ManualEpisodeSearchSortPrompt,
  ManualSeasonSearch,
  ManualSeasonSearchConfirmPrompt,
  ManualSeasonSearchSortPrompt,
  RootFolders,
  SearchEpisodes,
  SearchEpisodesError,
  SearchHistory,
  SearchHistoryError,
  SearchSeason,
  SearchSeasonError,
  SearchSeries,
  SearchSeriesError,
  SearchSeriesHistory,
  SearchSeriesHistoryError,
  SeasonDetails,
  #[default]
  Series,
  SeriesDetails,
  SeriesHistory,
  SeriesHistorySortPrompt,
  SeriesSortPrompt,
  System,
  SystemLogs,
  SystemQueuedEvents,
  SystemTasks,
  SystemTaskStartConfirmPrompt,
  SystemUpdates,
  TestAllIndexers,
  TestIndexer,
  UpdateAllSeriesPrompt,
  UpdateAndScanSeriesPrompt,
  UpdateDownloadsPrompt,
}

pub static LIBRARY_BLOCKS: [ActiveSonarrBlock; 7] = [
  ActiveSonarrBlock::Series,
  ActiveSonarrBlock::SeriesSortPrompt,
  ActiveSonarrBlock::SearchSeries,
  ActiveSonarrBlock::SearchSeriesError,
  ActiveSonarrBlock::FilterSeries,
  ActiveSonarrBlock::FilterSeriesError,
  ActiveSonarrBlock::UpdateAllSeriesPrompt,
];

pub static ADD_SERIES_BLOCKS: [ActiveSonarrBlock; 13] = [
  ActiveSonarrBlock::AddSeriesAlreadyInLibrary,
  ActiveSonarrBlock::AddSeriesConfirmPrompt,
  ActiveSonarrBlock::AddSeriesEmptySearchResults,
  ActiveSonarrBlock::AddSeriesPrompt,
  ActiveSonarrBlock::AddSeriesSearchInput,
  ActiveSonarrBlock::AddSeriesSearchResults,
  ActiveSonarrBlock::AddSeriesSelectLanguageProfile,
  ActiveSonarrBlock::AddSeriesSelectMonitor,
  ActiveSonarrBlock::AddSeriesSelectQualityProfile,
  ActiveSonarrBlock::AddSeriesSelectRootFolder,
  ActiveSonarrBlock::AddSeriesSelectSeriesType,
  ActiveSonarrBlock::AddSeriesTagsInput,
  ActiveSonarrBlock::AddSeriesToggleUseSeasonFolder,
];

pub const ADD_SERIES_SELECTION_BLOCKS: &[&[ActiveSonarrBlock]] = &[
  &[ActiveSonarrBlock::AddSeriesSelectRootFolder],
  &[ActiveSonarrBlock::AddSeriesSelectMonitor],
  &[ActiveSonarrBlock::AddSeriesSelectQualityProfile],
  &[ActiveSonarrBlock::AddSeriesSelectLanguageProfile],
  &[ActiveSonarrBlock::AddSeriesSelectSeriesType],
  &[ActiveSonarrBlock::AddSeriesToggleUseSeasonFolder],
  &[ActiveSonarrBlock::AddSeriesTagsInput],
  &[ActiveSonarrBlock::AddSeriesConfirmPrompt],
];

pub static BLOCKLIST_BLOCKS: [ActiveSonarrBlock; 5] = [
  ActiveSonarrBlock::Blocklist,
  ActiveSonarrBlock::BlocklistItemDetails,
  ActiveSonarrBlock::DeleteBlocklistItemPrompt,
  ActiveSonarrBlock::BlocklistClearAllItemsPrompt,
  ActiveSonarrBlock::BlocklistSortPrompt,
];

pub static EDIT_SERIES_BLOCKS: [ActiveSonarrBlock; 9] = [
  ActiveSonarrBlock::EditSeriesPrompt,
  ActiveSonarrBlock::EditSeriesConfirmPrompt,
  ActiveSonarrBlock::EditSeriesPathInput,
  ActiveSonarrBlock::EditSeriesSelectSeriesType,
  ActiveSonarrBlock::EditSeriesSelectQualityProfile,
  ActiveSonarrBlock::EditSeriesSelectLanguageProfile,
  ActiveSonarrBlock::EditSeriesTagsInput,
  ActiveSonarrBlock::EditSeriesToggleMonitored,
  ActiveSonarrBlock::EditSeriesToggleSeasonFolder,
];

pub static EDIT_SERIES_SELECTION_BLOCKS: &[&[ActiveSonarrBlock]] = &[
  &[ActiveSonarrBlock::EditSeriesToggleMonitored],
  &[ActiveSonarrBlock::EditSeriesToggleSeasonFolder],
  &[ActiveSonarrBlock::EditSeriesSelectQualityProfile],
  &[ActiveSonarrBlock::EditSeriesSelectLanguageProfile],
  &[ActiveSonarrBlock::EditSeriesSelectSeriesType],
  &[ActiveSonarrBlock::EditSeriesPathInput],
  &[ActiveSonarrBlock::EditSeriesTagsInput],
  &[ActiveSonarrBlock::EditSeriesConfirmPrompt],
];

pub static DOWNLOADS_BLOCKS: [ActiveSonarrBlock; 3] = [
  ActiveSonarrBlock::Downloads,
  ActiveSonarrBlock::DeleteDownloadPrompt,
  ActiveSonarrBlock::UpdateDownloadsPrompt,
];

pub static DELETE_SERIES_BLOCKS: [ActiveSonarrBlock; 4] = [
  ActiveSonarrBlock::DeleteSeriesPrompt,
  ActiveSonarrBlock::DeleteSeriesConfirmPrompt,
  ActiveSonarrBlock::DeleteSeriesToggleDeleteFile,
  ActiveSonarrBlock::DeleteSeriesToggleAddListExclusion,
];

pub const DELETE_SERIES_SELECTION_BLOCKS: &[&[ActiveSonarrBlock]] = &[
  &[ActiveSonarrBlock::DeleteSeriesToggleDeleteFile],
  &[ActiveSonarrBlock::DeleteSeriesToggleAddListExclusion],
  &[ActiveSonarrBlock::DeleteSeriesConfirmPrompt],
];

pub static EDIT_INDEXER_BLOCKS: [ActiveSonarrBlock; 11] = [
  ActiveSonarrBlock::EditIndexerPrompt,
  ActiveSonarrBlock::EditIndexerConfirmPrompt,
  ActiveSonarrBlock::EditIndexerApiKeyInput,
  ActiveSonarrBlock::EditIndexerNameInput,
  ActiveSonarrBlock::EditIndexerSeedRatioInput,
  ActiveSonarrBlock::EditIndexerToggleEnableRss,
  ActiveSonarrBlock::EditIndexerToggleEnableAutomaticSearch,
  ActiveSonarrBlock::EditIndexerToggleEnableInteractiveSearch,
  ActiveSonarrBlock::EditIndexerPriorityInput,
  ActiveSonarrBlock::EditIndexerUrlInput,
  ActiveSonarrBlock::EditIndexerTagsInput,
];

pub const EDIT_INDEXER_TORRENT_SELECTION_BLOCKS: &[&[ActiveSonarrBlock]] = &[
  &[
    ActiveSonarrBlock::EditIndexerNameInput,
    ActiveSonarrBlock::EditIndexerUrlInput,
  ],
  &[
    ActiveSonarrBlock::EditIndexerToggleEnableRss,
    ActiveSonarrBlock::EditIndexerApiKeyInput,
  ],
  &[
    ActiveSonarrBlock::EditIndexerToggleEnableAutomaticSearch,
    ActiveSonarrBlock::EditIndexerSeedRatioInput,
  ],
  &[
    ActiveSonarrBlock::EditIndexerToggleEnableInteractiveSearch,
    ActiveSonarrBlock::EditIndexerTagsInput,
  ],
  &[
    ActiveSonarrBlock::EditIndexerPriorityInput,
    ActiveSonarrBlock::EditIndexerConfirmPrompt,
  ],
  &[
    ActiveSonarrBlock::EditIndexerConfirmPrompt,
    ActiveSonarrBlock::EditIndexerConfirmPrompt,
  ],
];

pub const EDIT_INDEXER_NZB_SELECTION_BLOCKS: &[&[ActiveSonarrBlock]] = &[
  &[
    ActiveSonarrBlock::EditIndexerNameInput,
    ActiveSonarrBlock::EditIndexerUrlInput,
  ],
  &[
    ActiveSonarrBlock::EditIndexerToggleEnableRss,
    ActiveSonarrBlock::EditIndexerApiKeyInput,
  ],
  &[
    ActiveSonarrBlock::EditIndexerToggleEnableAutomaticSearch,
    ActiveSonarrBlock::EditIndexerTagsInput,
  ],
  &[
    ActiveSonarrBlock::EditIndexerToggleEnableInteractiveSearch,
    ActiveSonarrBlock::EditIndexerPriorityInput,
  ],
  &[
    ActiveSonarrBlock::EditIndexerConfirmPrompt,
    ActiveSonarrBlock::EditIndexerConfirmPrompt,
  ],
];

pub static INDEXER_SETTINGS_BLOCKS: [ActiveSonarrBlock; 6] = [
  ActiveSonarrBlock::AllIndexerSettingsPrompt,
  ActiveSonarrBlock::IndexerSettingsConfirmPrompt,
  ActiveSonarrBlock::IndexerSettingsMaximumSizeInput,
  ActiveSonarrBlock::IndexerSettingsMinimumAgeInput,
  ActiveSonarrBlock::IndexerSettingsRetentionInput,
  ActiveSonarrBlock::IndexerSettingsRssSyncIntervalInput,
];

pub const INDEXER_SETTINGS_SELECTION_BLOCKS: &[&[ActiveSonarrBlock]] = &[
  &[ActiveSonarrBlock::IndexerSettingsMinimumAgeInput],
  &[ActiveSonarrBlock::IndexerSettingsRetentionInput],
  &[ActiveSonarrBlock::IndexerSettingsMaximumSizeInput],
  &[ActiveSonarrBlock::IndexerSettingsRssSyncIntervalInput],
  &[ActiveSonarrBlock::IndexerSettingsConfirmPrompt],
];

pub static HISTORY_BLOCKS: [ActiveSonarrBlock; 7] = [
  ActiveSonarrBlock::History,
  ActiveSonarrBlock::HistoryItemDetails,
  ActiveSonarrBlock::HistorySortPrompt,
  ActiveSonarrBlock::FilterHistory,
  ActiveSonarrBlock::FilterHistoryError,
  ActiveSonarrBlock::SearchHistory,
  ActiveSonarrBlock::SearchHistoryError,
];

pub static ROOT_FOLDERS_BLOCKS: [ActiveSonarrBlock; 3] = [
  ActiveSonarrBlock::RootFolders,
  ActiveSonarrBlock::AddRootFolderPrompt,
  ActiveSonarrBlock::DeleteRootFolderPrompt,
];

pub static INDEXERS_BLOCKS: [ActiveSonarrBlock; 3] = [
  ActiveSonarrBlock::DeleteIndexerPrompt,
  ActiveSonarrBlock::Indexers,
  ActiveSonarrBlock::TestIndexer,
];

pub static SYSTEM_DETAILS_BLOCKS: [ActiveSonarrBlock; 5] = [
  ActiveSonarrBlock::SystemLogs,
  ActiveSonarrBlock::SystemQueuedEvents,
  ActiveSonarrBlock::SystemTasks,
  ActiveSonarrBlock::SystemTaskStartConfirmPrompt,
  ActiveSonarrBlock::SystemUpdates,
];

impl From<ActiveSonarrBlock> for Route {
  fn from(active_sonarr_block: ActiveSonarrBlock) -> Route {
    Route::Sonarr(active_sonarr_block, None)
  }
}

impl From<(ActiveSonarrBlock, Option<ActiveSonarrBlock>)> for Route {
  fn from(value: (ActiveSonarrBlock, Option<ActiveSonarrBlock>)) -> Route {
    Route::Sonarr(value.0, value.1)
  }
}
