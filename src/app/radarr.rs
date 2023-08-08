use std::collections::HashMap;

use chrono::{DateTime, Utc};
use strum::EnumIter;

use crate::app::models::{ScrollableText, StatefulTable, TabRoute, TabState};
use crate::app::App;
use crate::network::radarr_network::{DownloadRecord, Movie, RadarrEvent};

pub struct RadarrData {
  pub free_space: u64,
  pub total_space: u64,
  pub version: String,
  pub start_time: DateTime<Utc>,
  pub movies: StatefulTable<Movie>,
  pub downloads: StatefulTable<DownloadRecord>,
  pub quality_profile_map: HashMap<u64, String>,
  pub movie_details: ScrollableText,
  pub main_tabs: TabState,
}

impl Default for RadarrData {
  fn default() -> RadarrData {
    RadarrData {
      free_space: u64::default(),
      total_space: u64::default(),
      version: String::default(),
      start_time: DateTime::default(),
      movies: StatefulTable::default(),
      downloads: StatefulTable::default(),
      quality_profile_map: HashMap::default(),
      movie_details: ScrollableText::default(),
      main_tabs: TabState::new(vec![
        TabRoute {
          title: "Library".to_owned(),
          route: ActiveRadarrBlock::Movies.into(),
        },
        TabRoute {
          title: "Downloads".to_owned(),
          route: ActiveRadarrBlock::Downloads.into(),
        },
      ]),
    }
  }
}

#[derive(Clone, PartialEq, Eq, Debug, EnumIter)]
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
      }
      ActiveRadarrBlock::MovieDetails => {
        self.is_loading = true;
        self.dispatch(RadarrEvent::GetMovieDetails.into()).await
      }
      _ => (),
    }

    self.reset_tick_count();
  }
}
