use std::collections::HashMap;
use std::time::Duration;

use chrono::{DateTime, Utc};
use strum::EnumIter;

use crate::app::{App, Route};
use crate::models::radarr_models::{
  Collection, CollectionMovie, Credit, DiskSpace, DownloadRecord, Movie, MovieHistoryItem,
};
use crate::models::{ScrollableText, StatefulTable, TabRoute, TabState};
use crate::network::radarr_network::RadarrEvent;

pub struct RadarrData {
  pub disk_space_vec: Vec<DiskSpace>,
  pub version: String,
  pub start_time: DateTime<Utc>,
  pub movies: StatefulTable<Movie>,
  pub downloads: StatefulTable<DownloadRecord>,
  pub quality_profile_map: HashMap<u64, String>,
  pub movie_details: ScrollableText,
  pub file_details: String,
  pub audio_details: String,
  pub video_details: String,
  pub movie_history: StatefulTable<MovieHistoryItem>,
  pub movie_cast: StatefulTable<Credit>,
  pub movie_crew: StatefulTable<Credit>,
  pub collections: StatefulTable<Collection>,
  pub collection_movies: StatefulTable<CollectionMovie>,
  pub main_tabs: TabState,
  pub movie_info_tabs: TabState,
  pub search: String,
  pub is_searching: bool,
}

impl RadarrData {
  pub fn reset_movie_collection_table(&mut self) {
    self.collection_movies = StatefulTable::default();
  }

  pub fn reset_movie_info_tabs(&mut self) {
    self.file_details = String::default();
    self.audio_details = String::default();
    self.video_details = String::default();
    self.movie_details = ScrollableText::default();
    self.movie_history = StatefulTable::default();
    self.movie_cast = StatefulTable::default();
    self.movie_crew = StatefulTable::default();
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
      file_details: String::default(),
      audio_details: String::default(),
      video_details: String::default(),
      movie_details: ScrollableText::default(),
      movie_history: StatefulTable::default(),
      movie_cast: StatefulTable::default(),
      movie_crew: StatefulTable::default(),
      collections: StatefulTable::default(),
      collection_movies: StatefulTable::default(),
      search: String::default(),
      is_searching: false,
      main_tabs: TabState::new(vec![
        TabRoute {
          title: "Library".to_owned(),
          route: ActiveRadarrBlock::Movies.into(),
          help: "<↑↓> scroll table | <s> search | <enter> movie details | ←→ change tab "
            .to_owned(),
        },
        TabRoute {
          title: "Downloads".to_owned(),
          route: ActiveRadarrBlock::Downloads.into(),
          help: "<↑↓> scroll table | ←→ change tab ".to_owned(),
        },
        TabRoute {
          title: "Collections".to_owned(),
          route: ActiveRadarrBlock::Collections.into(),
          help: "<↑↓> scroll table | <enter> collection details | ←→ change tab ".to_owned(),
        },
      ]),
      movie_info_tabs: TabState::new(vec![
        TabRoute {
          title: "Details".to_owned(),
          route: ActiveRadarrBlock::MovieDetails.into(),
          help: "←→ change tab | <esc> close ".to_owned(),
        },
        TabRoute {
          title: "History".to_owned(),
          route: ActiveRadarrBlock::MovieHistory.into(),
          help: "<↑↓> scroll table | ←→ change tab | <esc> close ".to_owned(),
        },
        TabRoute {
          title: "File".to_owned(),
          route: ActiveRadarrBlock::FileInfo.into(),
          help: "←→ change tab | <esc> close ".to_owned(),
        },
        TabRoute {
          title: "Cast".to_owned(),
          route: ActiveRadarrBlock::Cast.into(),
          help: "<↑↓> scroll table | ←→ change tab | <esc> close ".to_owned(),
        },
        TabRoute {
          title: "Crew".to_owned(),
          route: ActiveRadarrBlock::Crew.into(),
          help: "<↑↓> scroll table | ←→ change tab | <esc> close ".to_owned(),
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
  CollectionDetails,
  Cast,
  Crew,
  FileInfo,
  Movies,
  MovieDetails,
  MovieHistory,
  Downloads,
  SearchMovie,
  SortOptions,
  ViewMovieOverview,
}

impl From<ActiveRadarrBlock> for Route {
  fn from(active_radarr_block: ActiveRadarrBlock) -> Route {
    Route::Radarr(active_radarr_block)
  }
}

impl App {
  pub(super) async fn dispatch_by_radarr_block(&mut self, active_radarr_block: &ActiveRadarrBlock) {
    match active_radarr_block {
      ActiveRadarrBlock::Collections => {
        self.is_loading = true;
        self
          .dispatch_network_event(RadarrEvent::GetCollections.into())
          .await
      }
      ActiveRadarrBlock::CollectionDetails => {
        self.is_loading = true;
        self.populate_movie_collection_table().await;
        self.is_loading = false;
      }
      ActiveRadarrBlock::Downloads => {
        self.is_loading = true;
        self
          .dispatch_network_event(RadarrEvent::GetDownloads.into())
          .await
      }
      ActiveRadarrBlock::Movies => {
        self
          .dispatch_network_event(RadarrEvent::GetMovies.into())
          .await;
        self
          .dispatch_network_event(RadarrEvent::GetDownloads.into())
          .await;
      }
      ActiveRadarrBlock::MovieDetails | ActiveRadarrBlock::FileInfo => {
        self.is_loading = true;
        self
          .dispatch_network_event(RadarrEvent::GetMovieDetails.into())
          .await;
      }
      ActiveRadarrBlock::MovieHistory => {
        self.is_loading = true;
        self
          .dispatch_network_event(RadarrEvent::GetMovieHistory.into())
          .await;
      }
      ActiveRadarrBlock::Cast | ActiveRadarrBlock::Crew => {
        if self.data.radarr_data.movie_cast.items.is_empty()
          || self.data.radarr_data.movie_crew.items.is_empty()
        {
          self.is_loading = true;
          self
            .dispatch_network_event(RadarrEvent::GetMovieCredits.into())
            .await;
        }
      }
      _ => (),
    }

    self.reset_tick_count();
  }

  pub(super) async fn radarr_on_tick(
    &mut self,
    active_radarr_block: ActiveRadarrBlock,
    is_first_render: bool,
  ) {
    if is_first_render {
      self
        .dispatch_network_event(RadarrEvent::GetQualityProfiles.into())
        .await;
      self
        .dispatch_network_event(RadarrEvent::GetOverview.into())
        .await;
      self
        .dispatch_network_event(RadarrEvent::GetStatus.into())
        .await;
      self.dispatch_by_radarr_block(&active_radarr_block).await;
    }

    if self.is_routing
      || self
        .network_tick_frequency
        .checked_sub(self.last_tick.elapsed())
        .unwrap_or_else(|| Duration::from_secs(0))
        .is_zero()
    {
      self.dispatch_by_radarr_block(&active_radarr_block).await;
    }
  }

  async fn populate_movie_collection_table(&mut self) {
    self.data.radarr_data.collection_movies.set_items(
      self
        .data
        .radarr_data
        .collections
        .current_selection_clone()
        .movies
        .unwrap_or_default(),
    );
  }
}
