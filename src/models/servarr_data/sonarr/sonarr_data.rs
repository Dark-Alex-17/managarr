use bimap::BiMap;
use chrono::{DateTime, Utc};
use serde_json::Number;
use strum::EnumIter;

use crate::{
  app::{
    context_clues::{
      BLOCKLIST_CONTEXT_CLUES, DOWNLOADS_CONTEXT_CLUES, INDEXERS_CONTEXT_CLUES,
      ROOT_FOLDERS_CONTEXT_CLUES, SYSTEM_CONTEXT_CLUES,
    },
    sonarr::sonarr_context_clues::{
      HISTORY_CONTEXT_CLUES, SERIES_CONTEXT_CLUES, SERIES_DETAILS_CONTEXT_CLUES,
      SERIES_HISTORY_CONTEXT_CLUES,
    },
  },
  models::{
    BlockSelectionState, HorizontallyScrollableText, Route, ScrollableText, TabRoute, TabState,
    servarr_data::modals::{EditIndexerModal, IndexerTestResultModalItem},
    servarr_models::{DiskSpace, Indexer, QueueEvent, RootFolder},
    sonarr_models::{
      AddSeriesSearchResult, BlocklistItem, DownloadRecord, IndexerSettings, Season, Series,
      SonarrHistoryItem, SonarrTask,
    },
    stateful_list::StatefulList,
    stateful_table::StatefulTable,
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
  pub indexer_test_errors: Option<String>,
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

impl SonarrData<'_> {
  pub fn reset_delete_series_preferences(&mut self) {
    self.delete_series_files = false;
    self.add_list_exclusion = false;
  }

  pub fn reset_series_info_tabs(&mut self) {
    self.series_history = None;
    self.seasons = StatefulTable::default();
    self.series_info_tabs.index = 0;
  }

  pub fn tag_ids_to_display(&self, tag_ids: &[Number]) -> String {
    tag_ids
      .iter()
      .filter_map(|tag_id| {
        let id = tag_id.as_i64()?;
        self.tags_map.get_by_left(&id).cloned()
      })
      .collect::<Vec<_>>()
      .join(", ")
  }

  pub fn sorted_quality_profile_names(&self) -> Vec<String> {
    let mut names: Vec<String> = self.quality_profile_map.right_values().cloned().collect();
    names.sort();
    names
  }

  pub fn sorted_language_profile_names(&self) -> Vec<String> {
    let mut names: Vec<String> = self.language_profiles_map.right_values().cloned().collect();
    names.sort();
    names
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
      indexer_test_errors: None,
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
          title: "Library".to_string(),
          route: ActiveSonarrBlock::Series.into(),
          contextual_help: Some(&SERIES_CONTEXT_CLUES),
          config: None,
        },
        TabRoute {
          title: "Downloads".to_string(),
          route: ActiveSonarrBlock::Downloads.into(),
          contextual_help: Some(&DOWNLOADS_CONTEXT_CLUES),
          config: None,
        },
        TabRoute {
          title: "Blocklist".to_string(),
          route: ActiveSonarrBlock::Blocklist.into(),
          contextual_help: Some(&BLOCKLIST_CONTEXT_CLUES),
          config: None,
        },
        TabRoute {
          title: "History".to_string(),
          route: ActiveSonarrBlock::History.into(),
          contextual_help: Some(&HISTORY_CONTEXT_CLUES),
          config: None,
        },
        TabRoute {
          title: "Root Folders".to_string(),
          route: ActiveSonarrBlock::RootFolders.into(),
          contextual_help: Some(&ROOT_FOLDERS_CONTEXT_CLUES),
          config: None,
        },
        TabRoute {
          title: "Indexers".to_string(),
          route: ActiveSonarrBlock::Indexers.into(),
          contextual_help: Some(&INDEXERS_CONTEXT_CLUES),
          config: None,
        },
        TabRoute {
          title: "System".to_string(),
          route: ActiveSonarrBlock::System.into(),
          contextual_help: Some(&SYSTEM_CONTEXT_CLUES),
          config: None,
        },
      ]),
      series_info_tabs: TabState::new(vec![
        TabRoute {
          title: "Seasons".to_string(),
          route: ActiveSonarrBlock::SeriesDetails.into(),
          contextual_help: Some(&SERIES_DETAILS_CONTEXT_CLUES),
          config: None,
        },
        TabRoute {
          title: "History".to_string(),
          route: ActiveSonarrBlock::SeriesHistory.into(),
          contextual_help: Some(&SERIES_HISTORY_CONTEXT_CLUES),
          config: None,
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
  EpisodeHistoryDetails,
  EpisodesSortPrompt,
  FilterEpisodes,
  FilterEpisodesError,
  FilterHistory,
  FilterHistoryError,
  FilterSeries,
  FilterSeriesError,
  FilterSeriesHistory,
  FilterSeriesHistoryError,
  FilterSeasonHistory,
  FilterSeasonHistoryError,
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
  SearchSeasonHistory,
  SearchSeasonHistoryError,
  SeasonDetails,
  SeasonHistory,
  SeasonHistoryDetails,
  SeasonHistorySortPrompt,
  #[default]
  Series,
  SeriesDetails,
  SeriesHistory,
  SeriesHistoryDetails,
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

pub static SERIES_DETAILS_BLOCKS: [ActiveSonarrBlock; 12] = [
  ActiveSonarrBlock::SeriesDetails,
  ActiveSonarrBlock::SeriesHistory,
  ActiveSonarrBlock::SearchSeason,
  ActiveSonarrBlock::SearchSeasonError,
  ActiveSonarrBlock::UpdateAndScanSeriesPrompt,
  ActiveSonarrBlock::AutomaticallySearchSeriesPrompt,
  ActiveSonarrBlock::SearchSeriesHistory,
  ActiveSonarrBlock::SearchSeriesHistoryError,
  ActiveSonarrBlock::FilterSeriesHistory,
  ActiveSonarrBlock::FilterSeriesHistoryError,
  ActiveSonarrBlock::SeriesHistorySortPrompt,
  ActiveSonarrBlock::SeriesHistoryDetails,
];

pub static SEASON_DETAILS_BLOCKS: [ActiveSonarrBlock; 15] = [
  ActiveSonarrBlock::SeasonDetails,
  ActiveSonarrBlock::SeasonHistory,
  ActiveSonarrBlock::SearchEpisodes,
  ActiveSonarrBlock::SearchEpisodesError,
  ActiveSonarrBlock::AutomaticallySearchSeasonPrompt,
  ActiveSonarrBlock::SearchSeasonHistory,
  ActiveSonarrBlock::SearchSeasonHistoryError,
  ActiveSonarrBlock::FilterSeasonHistory,
  ActiveSonarrBlock::FilterSeasonHistoryError,
  ActiveSonarrBlock::SeasonHistorySortPrompt,
  ActiveSonarrBlock::SeasonHistoryDetails,
  ActiveSonarrBlock::ManualSeasonSearch,
  ActiveSonarrBlock::ManualSeasonSearchConfirmPrompt,
  ActiveSonarrBlock::ManualSeasonSearchSortPrompt,
  ActiveSonarrBlock::DeleteEpisodeFilePrompt,
];

pub static EPISODE_DETAILS_BLOCKS: [ActiveSonarrBlock; 8] = [
  ActiveSonarrBlock::EpisodeDetails,
  ActiveSonarrBlock::EpisodeHistory,
  ActiveSonarrBlock::EpisodeHistoryDetails,
  ActiveSonarrBlock::EpisodeFile,
  ActiveSonarrBlock::ManualEpisodeSearch,
  ActiveSonarrBlock::ManualEpisodeSearchConfirmPrompt,
  ActiveSonarrBlock::ManualEpisodeSearchSortPrompt,
  ActiveSonarrBlock::AutomaticallySearchEpisodePrompt,
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
