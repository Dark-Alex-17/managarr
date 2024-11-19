use bimap::BiMap;
use chrono::{DateTime, Utc};
use strum::EnumIter;

use crate::models::{
  sonarr_models::{BlocklistItem, DownloadRecord, Episode, Indexer, Series},
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
  pub version: String,
  pub start_time: DateTime<Utc>,
  pub series: StatefulTable<Series>,
  pub blocklist: StatefulTable<BlocklistItem>,
  pub logs: StatefulList<HorizontallyScrollableText>,
  pub episodes_tree: StatefulTree<Episode>,
  pub episodes_table: StatefulTable<Episode>,
  pub downloads: StatefulTable<DownloadRecord>,
  pub episode_details_modal: Option<EpisodeDetailsModal>,
  pub quality_profile_map: BiMap<i64, String>,
  pub indexers: StatefulTable<Indexer>,
}

impl Default for SonarrData {
  fn default() -> SonarrData {
    SonarrData {
      version: String::new(),
      start_time: DateTime::default(),
      series: StatefulTable::default(),
      blocklist: StatefulTable::default(),
      logs: StatefulList::default(),
      episodes_tree: StatefulTree::default(),
      episodes_table: StatefulTable::default(),
      downloads: StatefulTable::default(),
      episode_details_modal: None,
      quality_profile_map: BiMap::new(),
      indexers: StatefulTable::default(),
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
