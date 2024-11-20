use bimap::BiMap;
use chrono::{DateTime, Utc};
use strum::EnumIter;

use crate::models::{
  servarr_models::{Indexer, QueueEvent},
  sonarr_models::{BlocklistItem, DownloadRecord, IndexerSettings, Season, Series},
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
  pub indexers: StatefulTable<Indexer>,
  pub indexer_settings: Option<IndexerSettings>,
  pub logs: StatefulList<HorizontallyScrollableText>,
  pub quality_profile_map: BiMap<i64, String>,
  pub queued_events: StatefulTable<QueueEvent>,
  pub seasons: StatefulTable<Season>,
  pub season_details_modal: Option<SeasonDetailsModal>,
  pub series: StatefulTable<Series>,
  pub start_time: DateTime<Utc>,
  pub version: String,
}

impl Default for SonarrData {
  fn default() -> SonarrData {
    SonarrData {
      blocklist: StatefulTable::default(),
      downloads: StatefulTable::default(),
      indexers: StatefulTable::default(),
      indexer_settings: None,
      logs: StatefulList::default(),
      quality_profile_map: BiMap::new(),
      queued_events: StatefulTable::default(),
      seasons: StatefulTable::default(),
      series: StatefulTable::default(),
      season_details_modal: None,
      start_time: DateTime::default(),
      version: String::new(),
    }
  }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default, EnumIter)]
pub enum ActiveSonarrBlock {
  #[default]
  Series,
  UpdateAndScanSeriesPrompt,
  EditSeriesPrompt,
  SeriesSortPrompt,
  SearchSeries,
  SearchSeriesError,
  FilterSeries,
  FilterSeriesError,
  DeleteSeriesPrompt,
  DeleteSeriesConfirmPrompt,
  AutomaticallySearchSeriesPrompt,
  UpdateAllSeriesPrompt,
  SeriesDetails,
  SeriesHistory,
  HistoryDetails,
  MarkHistoryItemAsFailurePrompt,
  MarkHistoryItemAsFailureConfirmPrompt,
  SearchSeason,
  SearchSeasonError,
  AutomaticallySearchSeasonPrompt,
  SeasonDetails,
  SeasonHistory,
  ManualSeasonSearch,
  ManualSeasonSearchSortPrompt,
  ManualSeasonSearchConfirmPrompt,
  DeleteEpisodeFilePrompt,
  EpisodeDetails,
  EpisodesSortPrompt,
  SearchEpisodes,
  SearchEpisodesError,
  FilterEpisodes,
  FilterEpisodesError,
  AutomaticallySearchEpisodePrompt,
  EditEpisodePrompt,
  EpisodeHistory,
  EpisodeFile,
  ManualEpisodeSearch,
  ManualEpisodeSearchSortPrompt,
  ManualEpisodeSearchConfirmPrompt,
  AddSeriesPrompt,
  AddSeriesSearchInput,
  AddSeriesSearchResults,
  AddSeriesAlreadyInLibrary,
  AddSeriesEmptySearchResults,
  AddSeriesConfirmPrompt,
  Downloads,
  DeleteDownloadPrompt,
  Blocklist,
  BlocklistClearAllItemsPrompt,
  BlocklistItemDetails,
  BlocklistSortPrompt,
  DeleteBlocklistItemPrompt,
  RootFolders,
  AddRootFolderPrompt,
  DeleteRootFolderPrompt,
  Indexers,
  DeleteIndexerPrompt,
  EditIndexerPrompt,
  AllIndexerSettingsPrompt,
  TestIndexer,
  TestAllIndexers,
  System,
  SystemTasks,
  SystemTaskStartConfirmPrompt,
  SystemLogs,
  SystemQueuedEvents,
  SystemUpdates,
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
