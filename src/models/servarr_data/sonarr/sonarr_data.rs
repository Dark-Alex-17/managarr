use bimap::BiMap;
use chrono::{DateTime, Utc};
use strum::EnumIter;

use crate::models::{
  servarr_data::modals::{EditIndexerModal, IndexerTestResultModalItem},
  servarr_models::{DiskSpace, Indexer, QueueEvent, RootFolder},
  sonarr_models::{
    AddSeriesSearchResult, BlocklistItem, DownloadRecord, IndexerSettings, Season, Series,
    SonarrHistoryItem, SonarrTask,
  },
  stateful_list::StatefulList,
  stateful_table::StatefulTable,
  HorizontallyScrollableText, Route, ScrollableText,
};

use super::modals::{AddSeriesModal, SeasonDetailsModal};

#[cfg(test)]
#[path = "sonarr_data_tests.rs"]
mod sonarr_data_tests;

pub struct SonarrData {
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
  pub history: StatefulTable<SonarrHistoryItem>,
  pub indexers: StatefulTable<Indexer>,
  pub indexer_settings: Option<IndexerSettings>,
  pub indexer_test_all_results: Option<StatefulTable<IndexerTestResultModalItem>>,
  pub indexer_test_error: Option<String>,
  pub language_profiles_map: BiMap<i64, String>,
  pub logs: StatefulList<HorizontallyScrollableText>,
  pub quality_profile_map: BiMap<i64, String>,
  pub queued_events: StatefulTable<QueueEvent>,
  pub root_folders: StatefulTable<RootFolder>,
  pub seasons: StatefulTable<Season>,
  pub season_details_modal: Option<SeasonDetailsModal>,
  pub series: StatefulTable<Series>,
  pub series_history: Option<StatefulTable<SonarrHistoryItem>>,
  pub start_time: DateTime<Utc>,
  pub tags_map: BiMap<i64, String>,
  pub tasks: StatefulTable<SonarrTask>,
  pub updates: ScrollableText,
  pub version: String,
}

impl SonarrData {
  pub fn reset_delete_series_preferences(&mut self) {
    self.delete_series_files = false;
    self.add_list_exclusion = false;
  }
}

impl Default for SonarrData {
  fn default() -> SonarrData {
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
      history: StatefulTable::default(),
      indexers: StatefulTable::default(),
      indexer_settings: None,
      indexer_test_error: None,
      indexer_test_all_results: None,
      language_profiles_map: BiMap::new(),
      logs: StatefulList::default(),
      quality_profile_map: BiMap::new(),
      queued_events: StatefulTable::default(),
      root_folders: StatefulTable::default(),
      seasons: StatefulTable::default(),
      season_details_modal: None,
      series: StatefulTable::default(),
      series_history: None,
      start_time: DateTime::default(),
      tags_map: BiMap::default(),
      tasks: StatefulTable::default(),
      updates: ScrollableText::default(),
      version: String::new(),
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
  EditSeriesPrompt,
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
  HistoryDetails,
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
  MarkHistoryItemAsFailedConfirmPrompt,
  MarkHistoryItemAsFailedPrompt,
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
  SeasonHistory,
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
}

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
