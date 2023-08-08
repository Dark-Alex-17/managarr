use std::collections::HashMap;
use std::time::Duration;

use chrono::{DateTime, Utc};

use crate::app::{App, Route};
use crate::models::radarr_models::{
  AddMovieSearchResult, Collection, CollectionMovie, Credit, DiskSpace, DownloadRecord,
  MinimumAvailability, Monitor, Movie, MovieHistoryItem, Release, RootFolder,
};
use crate::models::{ScrollableText, StatefulList, StatefulTable, TabRoute, TabState};
use crate::network::radarr_network::RadarrEvent;

pub struct RadarrData {
  pub root_folders: Vec<RootFolder>,
  pub disk_space_vec: Vec<DiskSpace>,
  pub version: String,
  pub start_time: DateTime<Utc>,
  pub movies: StatefulTable<Movie>,
  pub filtered_movies: StatefulTable<Movie>,
  pub add_searched_movies: StatefulTable<AddMovieSearchResult>,
  pub add_movie_monitor_list: StatefulList<Monitor>,
  pub add_movie_minimum_availability_list: StatefulList<MinimumAvailability>,
  pub add_movie_quality_profile_list: StatefulList<String>,
  pub selected_block: ActiveRadarrBlock,
  pub downloads: StatefulTable<DownloadRecord>,
  pub quality_profile_map: HashMap<u64, String>,
  pub movie_details: ScrollableText,
  pub file_details: String,
  pub audio_details: String,
  pub video_details: String,
  pub movie_history: StatefulTable<MovieHistoryItem>,
  pub movie_cast: StatefulTable<Credit>,
  pub movie_crew: StatefulTable<Credit>,
  pub movie_releases: StatefulTable<Release>,
  pub collections: StatefulTable<Collection>,
  pub filtered_collections: StatefulTable<Collection>,
  pub collection_movies: StatefulTable<CollectionMovie>,
  pub prompt_confirm_action: Option<RadarrEvent>,
  pub main_tabs: TabState,
  pub movie_info_tabs: TabState,
  pub search: String,
  pub filter: String,
  pub prompt_confirm: bool,
  pub is_searching: bool,
}

impl RadarrData {
  pub fn reset_movie_collection_table(&mut self) {
    self.collection_movies = StatefulTable::default();
  }

  pub fn reset_search(&mut self) {
    self.is_searching = false;
    self.search = String::default();
    self.filter = String::default();
    self.filtered_movies = StatefulTable::default();
    self.filtered_collections = StatefulTable::default();
    self.add_searched_movies = StatefulTable::default();
  }

  pub fn reset_movie_info_tabs(&mut self) {
    self.file_details = String::default();
    self.audio_details = String::default();
    self.video_details = String::default();
    self.movie_details = ScrollableText::default();
    self.movie_history = StatefulTable::default();
    self.movie_cast = StatefulTable::default();
    self.movie_crew = StatefulTable::default();
    self.movie_releases = StatefulTable::default();
    self.movie_info_tabs.index = 0;
  }

  pub fn reset_add_movie_selections(&mut self) {
    self.add_movie_monitor_list = StatefulList::default();
    self.add_movie_minimum_availability_list = StatefulList::default();
    self.add_movie_quality_profile_list = StatefulList::default();
  }

  pub fn reset_main_tab_index(&mut self) {
    self.main_tabs.index = 0;
  }
}

impl Default for RadarrData {
  fn default() -> RadarrData {
    RadarrData {
      root_folders: Vec::new(),
      disk_space_vec: Vec::new(),
      version: String::default(),
      start_time: DateTime::default(),
      movies: StatefulTable::default(),
      add_searched_movies: StatefulTable::default(),
      add_movie_monitor_list: StatefulList::default(),
      add_movie_minimum_availability_list: StatefulList::default(),
      add_movie_quality_profile_list: StatefulList::default(),
      selected_block: ActiveRadarrBlock::AddMovieSelectMonitor,
      filtered_movies: StatefulTable::default(),
      downloads: StatefulTable::default(),
      quality_profile_map: HashMap::default(),
      file_details: String::default(),
      audio_details: String::default(),
      video_details: String::default(),
      movie_details: ScrollableText::default(),
      movie_history: StatefulTable::default(),
      movie_cast: StatefulTable::default(),
      movie_crew: StatefulTable::default(),
      movie_releases: StatefulTable::default(),
      collections: StatefulTable::default(),
      filtered_collections: StatefulTable::default(),
      collection_movies: StatefulTable::default(),
      prompt_confirm_action: None,
      search: String::default(),
      filter: String::default(),
      is_searching: false,
      prompt_confirm: false,
      main_tabs: TabState::new(vec![
        TabRoute {
          title: "Library".to_owned(),
          route: ActiveRadarrBlock::Movies.into(),
          help: String::default(),
          contextual_help: Some("<a> add | <s> search | <f> filter | <r> refresh | <enter> details | <esc> cancel filter | <del> delete"
            .to_owned()),
        },
        TabRoute {
          title: "Downloads".to_owned(),
          route: ActiveRadarrBlock::Downloads.into(),
          help: String::default(),
          contextual_help: Some("<r> refresh | <del> delete".to_owned()),
        },
        TabRoute {
          title: "Collections".to_owned(),
          route: ActiveRadarrBlock::Collections.into(),
          help: String::default(),
          contextual_help: Some("<s> search | <f> filter | <r> refresh | <enter> details | <esc> cancel filter"
            .to_owned()),
        },
      ]),
      movie_info_tabs: TabState::new(vec![
        TabRoute {
          title: "Details".to_owned(),
          route: ActiveRadarrBlock::MovieDetails.into(),
          help: "<r> refresh | <s> auto search | <esc> close".to_owned(),
          contextual_help: None
        },
        TabRoute {
          title: "History".to_owned(),
          route: ActiveRadarrBlock::MovieHistory.into(),
          help: "<r> refresh | <s> auto search | <esc> close".to_owned(),
          contextual_help: None
        },
        TabRoute {
          title: "File".to_owned(),
          route: ActiveRadarrBlock::FileInfo.into(),
          help: "<r> refresh | <s> auto search | <esc> close".to_owned(),
          contextual_help: None,
        },
        TabRoute {
          title: "Cast".to_owned(),
          route: ActiveRadarrBlock::Cast.into(),
          help: "<r> refresh | <s> auto search | <esc> close".to_owned(),
          contextual_help: None,
        },
        TabRoute {
          title: "Crew".to_owned(),
          route: ActiveRadarrBlock::Crew.into(),
          help: "<r> refresh | <s> auto search | <esc> close".to_owned(),
          contextual_help: None,
        },
        TabRoute {
          title: "Manual Search".to_owned(),
          route: ActiveRadarrBlock::ManualSearch.into(),
          help: "<r> refresh | <s> auto search | <esc> close".to_owned(),
          contextual_help: Some("<enter> details".to_owned())
        }
      ]),
    }
  }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum ActiveRadarrBlock {
  AddMovieSearchInput,
  AddMovieSearchResults,
  AddMoviePrompt,
  AddMovieSelectMinimumAvailability,
  AddMovieSelectQualityProfile,
  AddMovieSelectMonitor,
  AddMovieConfirmPrompt,
  AutomaticallySearchMoviePrompt,
  Collections,
  CollectionDetails,
  Cast,
  Crew,
  DeleteMoviePrompt,
  DeleteDownloadPrompt,
  Downloads,
  FileInfo,
  FilterCollections,
  FilterMovies,
  ManualSearch,
  ManualSearchConfirmPrompt,
  MovieDetails,
  MovieHistory,
  Movies,
  RefreshAndScanPrompt,
  RefreshAllCollectionsPrompt,
  RefreshAllMoviesPrompt,
  RefreshDownloadsPrompt,
  SearchMovie,
  SearchCollection,
  ViewMovieOverview,
}

impl ActiveRadarrBlock {
  pub fn next_add_prompt_block(&self) -> Self {
    match self {
      ActiveRadarrBlock::AddMovieSelectMonitor => {
        ActiveRadarrBlock::AddMovieSelectMinimumAvailability
      }
      ActiveRadarrBlock::AddMovieSelectMinimumAvailability => {
        ActiveRadarrBlock::AddMovieSelectQualityProfile
      }
      ActiveRadarrBlock::AddMovieSelectQualityProfile => ActiveRadarrBlock::AddMovieConfirmPrompt,
      _ => ActiveRadarrBlock::AddMovieSelectMonitor,
    }
  }

  pub fn previous_add_prompt_block(&self) -> Self {
    match self {
      ActiveRadarrBlock::AddMovieSelectMonitor => ActiveRadarrBlock::AddMovieConfirmPrompt,
      ActiveRadarrBlock::AddMovieSelectMinimumAvailability => {
        ActiveRadarrBlock::AddMovieSelectMonitor
      }
      ActiveRadarrBlock::AddMovieSelectQualityProfile => {
        ActiveRadarrBlock::AddMovieSelectMinimumAvailability
      }
      ActiveRadarrBlock::AddMovieConfirmPrompt => ActiveRadarrBlock::AddMovieSelectQualityProfile,
      _ => ActiveRadarrBlock::AddMovieSelectMonitor,
    }
  }
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
          .await;
        self.check_for_prompt_action().await;
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
          .await;
        self.check_for_prompt_action().await;
      }
      ActiveRadarrBlock::Movies => {
        self
          .dispatch_network_event(RadarrEvent::GetMovies.into())
          .await;
        self
          .dispatch_network_event(RadarrEvent::GetDownloads.into())
          .await;
        self.check_for_prompt_action().await;
      }
      ActiveRadarrBlock::AddMovieSearchResults => {
        self.is_loading = true;
        self
          .dispatch_network_event(RadarrEvent::SearchNewMovie.into())
          .await;

        self.check_for_prompt_action().await;
      }
      ActiveRadarrBlock::MovieDetails | ActiveRadarrBlock::FileInfo => {
        self.is_loading = true;
        self
          .dispatch_network_event(RadarrEvent::GetMovieDetails.into())
          .await;
        self.check_for_prompt_action().await;
      }
      ActiveRadarrBlock::MovieHistory => {
        self.is_loading = true;
        self
          .dispatch_network_event(RadarrEvent::GetMovieHistory.into())
          .await;
        self.check_for_prompt_action().await;
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
        self.check_for_prompt_action().await;
      }
      ActiveRadarrBlock::ManualSearch => {
        if self.data.radarr_data.movie_releases.items.is_empty() && !self.is_loading {
          self.is_loading = true;
          self
            .dispatch_network_event(RadarrEvent::GetReleases.into())
            .await;
        }

        self.check_for_prompt_action().await;
      }
      _ => (),
    }

    self.reset_tick_count();
  }

  async fn check_for_prompt_action(&mut self) {
    if self.data.radarr_data.prompt_confirm {
      self.data.radarr_data.prompt_confirm = false;
      if let Some(radarr_event) = &self.data.radarr_data.prompt_confirm_action {
        self
          .dispatch_network_event(radarr_event.clone().into())
          .await;
        self.should_refresh = true;
        self.data.radarr_data.prompt_confirm_action = None;
      }
    }
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
        .dispatch_network_event(RadarrEvent::GetRootFolders.into())
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
    let collection_movies = if !self.data.radarr_data.filtered_collections.items.is_empty() {
      self
        .data
        .radarr_data
        .filtered_collections
        .current_selection_clone()
        .movies
        .unwrap_or_default()
    } else {
      self
        .data
        .radarr_data
        .collections
        .current_selection_clone()
        .movies
        .unwrap_or_default()
    };
    self
      .data
      .radarr_data
      .collection_movies
      .set_items(collection_movies);
  }
}
