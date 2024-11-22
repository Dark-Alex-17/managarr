use bimap::BiMap;
use chrono::{DateTime, Utc};
use strum::EnumIter;

use crate::models::{
  servarr_models::{Indexer, QueueEvent, RootFolder},
  sonarr_models::{
    BlocklistItem, DownloadRecord, IndexerSettings, Season, Series, SonarrHistoryItem,
  },
  stateful_list::StatefulList,
  stateful_table::StatefulTable,
  HorizontallyScrollableText, Route,
};

use super::modals::SeasonDetailsModal;

#[cfg(test)]
#[path = "sonarr_data_tests.rs"]
mod sonarr_data_tests;

pub struct SonarrData {
  pub blocklist: StatefulTable<BlocklistItem>,
  pub downloads: StatefulTable<DownloadRecord>,
  pub edit_root_folder: Option<HorizontallyScrollableText>,
  pub history: StatefulTable<SonarrHistoryItem>,
  pub indexers: StatefulTable<Indexer>,
  pub indexer_settings: Option<IndexerSettings>,
  pub logs: StatefulList<HorizontallyScrollableText>,
  pub quality_profile_map: BiMap<i64, String>,
  pub queued_events: StatefulTable<QueueEvent>,
  pub root_folders: StatefulTable<RootFolder>,
  pub seasons: StatefulTable<Season>,
  pub season_details_modal: Option<SeasonDetailsModal>,
  pub series: StatefulTable<Series>,
  pub series_history: Option<StatefulTable<SonarrHistoryItem>>,
  pub start_time: DateTime<Utc>,
  pub version: String,
}

impl Default for SonarrData {
  fn default() -> SonarrData {
    SonarrData {
      blocklist: StatefulTable::default(),
      downloads: StatefulTable::default(),
      edit_root_folder: None,
      history: StatefulTable::default(),
      indexers: StatefulTable::default(),
      indexer_settings: None,
      logs: StatefulList::default(),
      quality_profile_map: BiMap::new(),
      queued_events: StatefulTable::default(),
      root_folders: StatefulTable::default(),
      seasons: StatefulTable::default(),
      season_details_modal: None,
      series: StatefulTable::default(),
      series_history: None,
      start_time: DateTime::default(),
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
  Downloads,
  EditEpisodePrompt,
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
  ManualEpisodeSearch,
  ManualEpisodeSearchConfirmPrompt,
  ManualEpisodeSearchSortPrompt,
  ManualSeasonSearch,
  ManualSeasonSearchConfirmPrompt,
  ManualSeasonSearchSortPrompt,
  MarkHistoryItemAsFailureConfirmPrompt,
  MarkHistoryItemAsFailurePrompt,
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
