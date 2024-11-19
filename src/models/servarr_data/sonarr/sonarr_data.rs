use bimap::BiMap;
use chrono::{DateTime, Utc};
use strum::EnumIter;

use crate::models::{
  servarr_models::{Indexer, QueueEvent},
  sonarr_models::{BlocklistItem, DownloadRecord, Episode, IndexerSettings, Series},
  stateful_list::StatefulList,
  stateful_table::StatefulTable,
  stateful_tree::StatefulTree,
  HorizontallyScrollableText, Route,
};

use super::modals::EpisodeDetailsModal;

#[cfg(test)]
#[path = "sonarr_data_tests.rs"]
mod sonarr_data_tests;

pub struct SonarrData {
  pub blocklist: StatefulTable<BlocklistItem>,
  pub downloads: StatefulTable<DownloadRecord>,
  pub episode_details_modal: Option<EpisodeDetailsModal>,
  pub episodes_table: StatefulTable<Episode>,
  pub episodes_tree: StatefulTree<Episode>,
  pub indexers: StatefulTable<Indexer>,
  pub indexer_settings: Option<IndexerSettings>,
  pub logs: StatefulList<HorizontallyScrollableText>,
  pub quality_profile_map: BiMap<i64, String>,
  pub queued_events: StatefulTable<QueueEvent>,
  pub series: StatefulTable<Series>,
  pub start_time: DateTime<Utc>,
  pub version: String,
}

impl Default for SonarrData {
  fn default() -> SonarrData {
    SonarrData {
      blocklist: StatefulTable::default(),
      downloads: StatefulTable::default(),
      episode_details_modal: None,
      episodes_table: StatefulTable::default(),
      episodes_tree: StatefulTree::default(),
      indexers: StatefulTable::default(),
      indexer_settings: None,
      logs: StatefulList::default(),
      quality_profile_map: BiMap::new(),
      queued_events: StatefulTable::default(),
      series: StatefulTable::default(),
      start_time: DateTime::default(),
      version: String::new(),
    }
  }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default, EnumIter)]
pub enum ActiveSonarrBlock {
  Blocklist,
  BlocklistSortPrompt,
  EpisodesExplorer,
  EpisodesTable,
  EpisodesTableSortPrompt,
  #[default]
  Series,
  SeriesSortPrompt,
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
