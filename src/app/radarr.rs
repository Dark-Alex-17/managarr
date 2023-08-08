use std::collections::HashMap;

use chrono::{DateTime, Utc};

use crate::app::{App, StatefulTable};
use crate::network::radarr_network::{DownloadRecord, Movie, RadarrEvent};

#[derive(Default)]
pub struct RadarrData {
  pub free_space: u64,
  pub total_space: u64,
  pub version: String,
  pub start_time: DateTime<Utc>,
  pub movies: StatefulTable<Movie>,
  pub downloads: StatefulTable<DownloadRecord>,
  pub quality_profile_map: HashMap<u64, String>,
}

#[derive(Clone, PartialEq, Eq)]
pub enum ActiveRadarrBlock {
  AddMovie,
  Calendar,
  Collections,
  Events,
  Logs,
  Movies,
  MovieDetails,
  Downloads,
  SearchMovie,
  SortOptions,
  Tasks,
}

impl App {
  pub(super) async fn dispatch_by_radarr_block(&mut self, active_radarr_block: ActiveRadarrBlock) {
    match active_radarr_block {
      ActiveRadarrBlock::Downloads => self.dispatch(RadarrEvent::GetDownloads.into()).await,
      ActiveRadarrBlock::Movies => {
        self.dispatch(RadarrEvent::GetMovies.into()).await;
        self.dispatch(RadarrEvent::GetDownloads.into()).await;
      },
      _ => ()
    }
  }
}
