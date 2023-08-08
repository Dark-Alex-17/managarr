use std::collections::HashMap;

use chrono::{DateTime, Utc};
use strum::EnumIter;

use crate::app::models::{ScrollableText, StatefulTable, TabRoute, TabState};
use crate::app::App;
use crate::network::radarr_network::{
  DiskSpace, DownloadRecord, Movie, MovieHistoryItem, RadarrEvent,
};

pub struct RadarrData {
  pub disk_space_vec: Vec<DiskSpace>,
  pub version: String,
  pub start_time: DateTime<Utc>,
  pub movies: StatefulTable<Movie>,
  pub downloads: StatefulTable<DownloadRecord>,
  pub quality_profile_map: HashMap<u64, String>,
  pub movie_details: ScrollableText,
  pub movie_history: StatefulTable<MovieHistoryItem>,
  pub main_tabs: TabState,
  pub movie_info_tabs: TabState,
}

impl RadarrData {
  pub fn reset_movie_info_tab(&mut self) {
    self.movie_details = ScrollableText::default();
    self.movie_history = StatefulTable::default();
    self.movie_info_tabs.index = 0;
  }

  pub fn reset_main_tab_index(&mut self) {
    self.main_tabs.index = 0;
  }
}

impl Default for RadarrData {
  fn default() -> RadarrData {
    RadarrData {
      disk_space_vec: Vec::new(),
      version: String::default(),
      start_time: DateTime::default(),
      movies: StatefulTable::default(),
      downloads: StatefulTable::default(),
      quality_profile_map: HashMap::default(),
      movie_details: ScrollableText::default(),
      movie_history: StatefulTable::default(),
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
      movie_info_tabs: TabState::new(vec![
        TabRoute {
          title: "Details".to_owned(),
          route: ActiveRadarrBlock::MovieDetails.into(),
        },
        TabRoute {
          title: "History".to_owned(),
          route: ActiveRadarrBlock::MovieHistory.into(),
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
  MovieHistory,
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
      ActiveRadarrBlock::MovieHistory => {
        self.is_loading = true;
        self.dispatch(RadarrEvent::GetMovieHistory.into()).await
      }
      _ => (),
    }

    self.reset_tick_count();
  }
}
