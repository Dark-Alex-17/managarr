use chrono::{DateTime, Utc};
use strum::EnumIter;

use crate::models::{sonarr_models::Series, stateful_table::StatefulTable, Route};

#[cfg(test)]
#[path = "sonarr_data_tests.rs"]
mod sonarr_data_tests;

pub struct SonarrData {
  pub version: String,
  pub start_time: DateTime<Utc>,
  pub series: StatefulTable<Series>,
}

impl Default for SonarrData {
  fn default() -> SonarrData {
    SonarrData {
      version: String::new(),
      start_time: DateTime::default(),
      series: StatefulTable::default(),
    }
  }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default, EnumIter)]
pub enum ActiveSonarrBlock {
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
