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

#[cfg(test)]
#[macro_use]
pub mod radarr_test_utils {
  use crate::app::radarr::RadarrData;
  use crate::models::radarr_models::{
    AddMovieSearchResult, Collection, CollectionMovie, Credit, MinimumAvailability, Monitor, Movie,
    MovieHistoryItem, Release,
  };
  use crate::models::ScrollableText;

  pub fn create_test_radarr_data() -> RadarrData {
    let mut radarr_data = RadarrData {
      is_searching: true,
      search: "test search".to_owned(),
      filter: "test filter".to_owned(),
      file_details: "test file details".to_owned(),
      audio_details: "test audio details".to_owned(),
      video_details: "test video details".to_owned(),
      movie_details: ScrollableText::with_string("test movie details".to_owned()),
      ..RadarrData::default()
    };
    radarr_data
      .movie_history
      .set_items(vec![MovieHistoryItem::default()]);
    radarr_data.movie_cast.set_items(vec![Credit::default()]);
    radarr_data.movie_crew.set_items(vec![Credit::default()]);
    radarr_data
      .movie_releases
      .set_items(vec![Release::default()]);
    radarr_data.movie_info_tabs.index = 1;
    radarr_data
      .add_movie_monitor_list
      .set_items(vec![Monitor::default()]);
    radarr_data
      .add_movie_minimum_availability_list
      .set_items(vec![MinimumAvailability::default()]);
    radarr_data
      .add_movie_quality_profile_list
      .set_items(vec![String::default()]);
    radarr_data
      .filtered_movies
      .set_items(vec![Movie::default()]);
    radarr_data
      .filtered_collections
      .set_items(vec![Collection::default()]);
    radarr_data
      .add_searched_movies
      .set_items(vec![AddMovieSearchResult::default()]);
    radarr_data
      .collection_movies
      .set_items(vec![CollectionMovie::default()]);

    radarr_data
  }

  #[macro_export]
  macro_rules! assert_movie_collection_table_reset {
    ($radarr_data:expr) => {
      assert!($radarr_data.collection_movies.items.is_empty());
    };
  }

  #[macro_export]
  macro_rules! assert_search_reset {
    ($radarr_data:expr) => {
      assert!(!$radarr_data.is_searching);
      assert!($radarr_data.search.is_empty());
      assert!($radarr_data.filter.is_empty());
      assert!($radarr_data.filtered_movies.items.is_empty());
      assert!($radarr_data.filtered_collections.items.is_empty());
      assert!($radarr_data.add_searched_movies.items.is_empty());
    };
  }

  #[macro_export]
  macro_rules! assert_movie_info_tabs_reset {
    ($radarr_data:expr) => {
      assert!($radarr_data.file_details.is_empty());
      assert!($radarr_data.audio_details.is_empty());
      assert!($radarr_data.video_details.is_empty());
      assert!($radarr_data.movie_details.get_text().is_empty());
      assert!($radarr_data.movie_history.items.is_empty());
      assert!($radarr_data.movie_cast.items.is_empty());
      assert!($radarr_data.movie_crew.items.is_empty());
      assert!($radarr_data.movie_releases.items.is_empty());
      assert_eq!($radarr_data.movie_info_tabs.index, 0);
    };
  }

  #[macro_export]
  macro_rules! assert_add_movie_selections_reset {
    ($radarr_data:expr) => {
      assert!($radarr_data.add_movie_monitor_list.items.is_empty());
      assert!($radarr_data
        .add_movie_minimum_availability_list
        .items
        .is_empty());
      assert!($radarr_data.add_movie_quality_profile_list.items.is_empty());
    };
  }
}

#[cfg(test)]
mod tests {
  mod radarr_data_tests {
    use pretty_assertions::assert_eq;

    use crate::app::radarr::radarr_test_utils::create_test_radarr_data;

    #[test]
    fn test_reset_movie_collection_table() {
      let mut radarr_data = create_test_radarr_data();

      radarr_data.reset_movie_collection_table();

      assert_movie_collection_table_reset!(radarr_data);
    }

    #[test]
    fn test_reset_search() {
      let mut radarr_data = create_test_radarr_data();

      radarr_data.reset_search();

      assert_search_reset!(radarr_data);
    }

    #[test]
    fn test_reset_movie_info_tabs() {
      let mut radarr_data = create_test_radarr_data();

      radarr_data.reset_movie_info_tabs();

      assert_movie_info_tabs_reset!(radarr_data);
    }

    #[test]
    fn test_reset_add_movie_selections() {
      let mut radarr_data = create_test_radarr_data();

      radarr_data.reset_add_movie_selections();

      assert_add_movie_selections_reset!(radarr_data);
    }
  }

  mod active_radarr_block_tests {
    use pretty_assertions::assert_eq;

    use crate::app::radarr::ActiveRadarrBlock;

    #[test]
    fn test_next_add_prompt_block() {
      let active_block = ActiveRadarrBlock::AddMovieSelectMonitor.next_add_prompt_block();

      assert_eq!(
        active_block,
        ActiveRadarrBlock::AddMovieSelectMinimumAvailability
      );

      let active_block = active_block.next_add_prompt_block();

      assert_eq!(
        active_block,
        ActiveRadarrBlock::AddMovieSelectQualityProfile
      );

      let active_block = active_block.next_add_prompt_block();

      assert_eq!(active_block, ActiveRadarrBlock::AddMovieConfirmPrompt);

      let active_block = active_block.next_add_prompt_block();

      assert_eq!(active_block, ActiveRadarrBlock::AddMovieSelectMonitor);
    }

    #[test]
    fn test_previous_add_prompt_block() {
      let active_block = ActiveRadarrBlock::AddMovieSelectMonitor.previous_add_prompt_block();

      assert_eq!(active_block, ActiveRadarrBlock::AddMovieConfirmPrompt);

      let active_block = active_block.previous_add_prompt_block();

      assert_eq!(
        active_block,
        ActiveRadarrBlock::AddMovieSelectQualityProfile
      );

      let active_block = active_block.previous_add_prompt_block();

      assert_eq!(
        active_block,
        ActiveRadarrBlock::AddMovieSelectMinimumAvailability
      );

      let active_block = active_block.previous_add_prompt_block();

      assert_eq!(active_block, ActiveRadarrBlock::AddMovieSelectMonitor);
    }
  }

  mod radarr_tests {
    use std::time::Duration;

    use pretty_assertions::assert_eq;
    use tokio::sync::mpsc;

    use crate::app::radarr::ActiveRadarrBlock;
    use crate::app::App;
    use crate::models::radarr_models::{Collection, CollectionMovie, Credit, Release};
    use crate::models::StatefulTable;
    use crate::network::radarr_network::RadarrEvent;
    use crate::network::NetworkEvent;

    #[tokio::test]
    async fn test_dispatch_by_collections_block() {
      let (mut app, mut sync_network_rx) = construct_app_unit();

      app
        .dispatch_by_radarr_block(&ActiveRadarrBlock::Collections)
        .await;

      assert!(app.is_loading);
      assert_eq!(
        sync_network_rx.recv().await.unwrap(),
        RadarrEvent::GetCollections.into()
      );
      assert!(!app.data.radarr_data.prompt_confirm);
      assert_eq!(app.tick_count, 0);
    }

    #[tokio::test]
    async fn test_dispatch_by_collection_details_block() {
      let mut app = App::default();

      app.data.radarr_data.collections.set_items(vec![Collection {
        movies: Some(vec![CollectionMovie::default()]),
        ..Collection::default()
      }]);

      app
        .dispatch_by_radarr_block(&ActiveRadarrBlock::CollectionDetails)
        .await;

      assert!(!app.is_loading);
      assert!(!app.data.radarr_data.collection_movies.items.is_empty());
      assert_eq!(app.tick_count, 0);
    }

    #[tokio::test]
    async fn test_dispatch_by_downloads_block() {
      let (mut app, mut sync_network_rx) = construct_app_unit();

      app
        .dispatch_by_radarr_block(&ActiveRadarrBlock::Downloads)
        .await;

      assert!(app.is_loading);
      assert_eq!(
        sync_network_rx.recv().await.unwrap(),
        RadarrEvent::GetDownloads.into()
      );
      assert!(!app.data.radarr_data.prompt_confirm);
      assert_eq!(app.tick_count, 0);
    }

    #[tokio::test]
    async fn test_dispatch_by_movies_block() {
      let (mut app, mut sync_network_rx) = construct_app_unit();

      app
        .dispatch_by_radarr_block(&ActiveRadarrBlock::Movies)
        .await;

      assert!(!app.is_loading);
      assert_eq!(
        sync_network_rx.recv().await.unwrap(),
        RadarrEvent::GetMovies.into()
      );
      assert_eq!(
        sync_network_rx.recv().await.unwrap(),
        RadarrEvent::GetDownloads.into()
      );
      assert!(!app.data.radarr_data.prompt_confirm);
      assert_eq!(app.tick_count, 0);
    }

    #[tokio::test]
    async fn test_dispatch_by_add_movie_search_results_block() {
      let (mut app, mut sync_network_rx) = construct_app_unit();

      app
        .dispatch_by_radarr_block(&ActiveRadarrBlock::AddMovieSearchResults)
        .await;

      assert!(app.is_loading);
      assert_eq!(
        sync_network_rx.recv().await.unwrap(),
        RadarrEvent::SearchNewMovie.into()
      );
      assert!(!app.data.radarr_data.prompt_confirm);
      assert_eq!(app.tick_count, 0);
    }

    #[tokio::test]
    async fn test_dispatch_by_movie_details_block() {
      let (mut app, mut sync_network_rx) = construct_app_unit();

      app
        .dispatch_by_radarr_block(&ActiveRadarrBlock::MovieDetails)
        .await;

      assert!(app.is_loading);
      assert_eq!(
        sync_network_rx.recv().await.unwrap(),
        RadarrEvent::GetMovieDetails.into()
      );
      assert!(!app.data.radarr_data.prompt_confirm);
      assert_eq!(app.tick_count, 0);
    }

    #[tokio::test]
    async fn test_dispatch_by_file_info_block() {
      let (mut app, mut sync_network_rx) = construct_app_unit();

      app
        .dispatch_by_radarr_block(&ActiveRadarrBlock::FileInfo)
        .await;

      assert!(app.is_loading);
      assert_eq!(
        sync_network_rx.recv().await.unwrap(),
        RadarrEvent::GetMovieDetails.into()
      );
      assert!(!app.data.radarr_data.prompt_confirm);
      assert_eq!(app.tick_count, 0);
    }

    #[tokio::test]
    async fn test_dispatch_by_movie_history_block() {
      let (mut app, mut sync_network_rx) = construct_app_unit();

      app
        .dispatch_by_radarr_block(&ActiveRadarrBlock::MovieHistory)
        .await;

      assert!(app.is_loading);
      assert_eq!(
        sync_network_rx.recv().await.unwrap(),
        RadarrEvent::GetMovieHistory.into()
      );
      assert!(!app.data.radarr_data.prompt_confirm);
      assert_eq!(app.tick_count, 0);
    }

    #[tokio::test]
    async fn test_dispatch_by_cast_crew_blocks() {
      let (mut app, mut sync_network_rx) = construct_app_unit();

      for active_radarr_block in &[ActiveRadarrBlock::Cast, ActiveRadarrBlock::Crew] {
        app.data.radarr_data.movie_cast = StatefulTable::default();
        app.data.radarr_data.movie_crew = StatefulTable::default();

        app.dispatch_by_radarr_block(active_radarr_block).await;

        assert!(app.is_loading);
        assert_eq!(
          sync_network_rx.recv().await.unwrap(),
          RadarrEvent::GetMovieCredits.into()
        );
        assert!(!app.data.radarr_data.prompt_confirm);
        assert_eq!(app.tick_count, 0);
      }
    }

    #[tokio::test]
    async fn test_dispatch_by_cast_crew_blocks_movie_cast_non_empty() {
      let (mut app, mut sync_network_rx) = construct_app_unit();

      for active_radarr_block in &[ActiveRadarrBlock::Cast, ActiveRadarrBlock::Crew] {
        app
          .data
          .radarr_data
          .movie_cast
          .set_items(vec![Credit::default()]);

        app.dispatch_by_radarr_block(active_radarr_block).await;

        assert!(app.is_loading);
        assert_eq!(
          sync_network_rx.recv().await.unwrap(),
          RadarrEvent::GetMovieCredits.into()
        );
        assert!(!app.data.radarr_data.prompt_confirm);
        assert_eq!(app.tick_count, 0);
      }
    }

    #[tokio::test]
    async fn test_dispatch_by_cast_crew_blocks_movie_crew_non_empty() {
      let (mut app, mut sync_network_rx) = construct_app_unit();

      for active_radarr_block in &[ActiveRadarrBlock::Cast, ActiveRadarrBlock::Crew] {
        app
          .data
          .radarr_data
          .movie_crew
          .set_items(vec![Credit::default()]);

        app.dispatch_by_radarr_block(active_radarr_block).await;

        assert!(app.is_loading);
        assert_eq!(
          sync_network_rx.recv().await.unwrap(),
          RadarrEvent::GetMovieCredits.into()
        );
        assert!(!app.data.radarr_data.prompt_confirm);
        assert_eq!(app.tick_count, 0);
      }
    }

    #[tokio::test]
    async fn test_dispatch_by_cast_crew_blocks_cast_and_crew_non_empty() {
      let mut app = App::default();

      for active_radarr_block in &[ActiveRadarrBlock::Cast, ActiveRadarrBlock::Crew] {
        app
          .data
          .radarr_data
          .movie_cast
          .set_items(vec![Credit::default()]);
        app
          .data
          .radarr_data
          .movie_crew
          .set_items(vec![Credit::default()]);

        app.dispatch_by_radarr_block(active_radarr_block).await;

        assert!(!app.is_loading);
        assert!(!app.data.radarr_data.prompt_confirm);
        assert_eq!(app.tick_count, 0);
      }
    }

    #[tokio::test]
    async fn test_dispatch_by_manual_search_block() {
      let (mut app, mut sync_network_rx) = construct_app_unit();

      app
        .dispatch_by_radarr_block(&ActiveRadarrBlock::ManualSearch)
        .await;

      assert!(app.is_loading);
      assert_eq!(
        sync_network_rx.recv().await.unwrap(),
        RadarrEvent::GetReleases.into()
      );
      assert!(!app.data.radarr_data.prompt_confirm);
      assert_eq!(app.tick_count, 0);
    }

    #[tokio::test]
    async fn test_dispatch_by_manual_search_block_movie_releases_non_empty() {
      let mut app = App::default();
      app
        .data
        .radarr_data
        .movie_releases
        .set_items(vec![Release::default()]);

      app
        .dispatch_by_radarr_block(&ActiveRadarrBlock::ManualSearch)
        .await;

      assert!(!app.is_loading);
      assert!(!app.data.radarr_data.prompt_confirm);
      assert_eq!(app.tick_count, 0);
    }

    #[tokio::test]
    async fn test_dispatch_by_manual_search_block_is_loading() {
      let mut app = App {
        is_loading: true,
        ..App::default()
      };

      app
        .dispatch_by_radarr_block(&ActiveRadarrBlock::ManualSearch)
        .await;

      assert!(app.is_loading);
      assert!(!app.data.radarr_data.prompt_confirm);
      assert_eq!(app.tick_count, 0);
    }

    #[tokio::test]
    async fn test_check_for_prompt_action_no_prompt_confirm() {
      let mut app = App::default();
      app.data.radarr_data.prompt_confirm = false;

      app.check_for_prompt_action().await;

      assert!(!app.data.radarr_data.prompt_confirm);
      assert!(!app.should_refresh);
    }

    #[tokio::test]
    async fn test_check_for_prompt_action() {
      let (mut app, mut sync_network_rx) = construct_app_unit();
      app.data.radarr_data.prompt_confirm_action = Some(RadarrEvent::GetStatus);

      app.check_for_prompt_action().await;

      assert!(!app.data.radarr_data.prompt_confirm);
      assert_eq!(
        sync_network_rx.recv().await.unwrap(),
        RadarrEvent::GetStatus.into()
      );
      assert!(app.should_refresh);
      assert_eq!(app.data.radarr_data.prompt_confirm_action, None);
    }

    #[tokio::test]
    async fn test_radarr_on_tick_first_render() {
      let (mut app, mut sync_network_rx) = construct_app_unit();

      app.radarr_on_tick(ActiveRadarrBlock::Downloads, true).await;

      assert_eq!(
        sync_network_rx.recv().await.unwrap(),
        RadarrEvent::GetQualityProfiles.into()
      );
      assert_eq!(
        sync_network_rx.recv().await.unwrap(),
        RadarrEvent::GetRootFolders.into()
      );
      assert_eq!(
        sync_network_rx.recv().await.unwrap(),
        RadarrEvent::GetOverview.into()
      );
      assert_eq!(
        sync_network_rx.recv().await.unwrap(),
        RadarrEvent::GetStatus.into()
      );
      assert_eq!(
        sync_network_rx.recv().await.unwrap(),
        RadarrEvent::GetDownloads.into()
      );
      assert!(app.is_loading);
      assert!(!app.data.radarr_data.prompt_confirm);
    }

    #[tokio::test]
    async fn test_radarr_on_tick_not_routing() {
      let mut app = App::default();

      app
        .radarr_on_tick(ActiveRadarrBlock::Downloads, false)
        .await;

      assert!(!app.is_routing);
    }

    #[tokio::test]
    async fn test_radarr_on_tick_routing() {
      let (mut app, mut sync_network_rx) = construct_app_unit();
      app.is_routing = true;

      app
        .radarr_on_tick(ActiveRadarrBlock::Downloads, false)
        .await;

      assert_eq!(
        sync_network_rx.recv().await.unwrap(),
        RadarrEvent::GetDownloads.into()
      );
      assert!(app.is_loading);
      assert!(!app.data.radarr_data.prompt_confirm);
    }

    #[tokio::test]
    async fn test_radarr_on_tick_network_tick_frequency() {
      let (mut app, mut sync_network_rx) = construct_app_unit();
      app.network_tick_frequency = Duration::from_secs(0);

      app
        .radarr_on_tick(ActiveRadarrBlock::Downloads, false)
        .await;

      assert_eq!(
        sync_network_rx.recv().await.unwrap(),
        RadarrEvent::GetDownloads.into()
      );
      assert!(app.is_loading);
      assert!(!app.data.radarr_data.prompt_confirm);
    }

    #[tokio::test]
    async fn test_populate_movie_collection_table_unfiltered() {
      let mut app = App::default();
      app.data.radarr_data.collections.set_items(vec![Collection {
        movies: Some(vec![CollectionMovie::default()]),
        ..Collection::default()
      }]);

      app.populate_movie_collection_table().await;

      assert!(!app.data.radarr_data.collection_movies.items.is_empty());
    }

    #[tokio::test]
    async fn test_populate_movie_collection_table_filtered() {
      let mut app = App::default();
      app
        .data
        .radarr_data
        .filtered_collections
        .set_items(vec![Collection {
          movies: Some(vec![CollectionMovie::default()]),
          ..Collection::default()
        }]);

      app.populate_movie_collection_table().await;

      assert!(!app.data.radarr_data.collection_movies.items.is_empty());
    }

    fn construct_app_unit() -> (App, mpsc::Receiver<NetworkEvent>) {
      let (sync_network_tx, sync_network_rx) = mpsc::channel::<NetworkEvent>(500);
      let mut app = App {
        network_tx: Some(sync_network_tx),
        tick_count: 1,
        ..App::default()
      };
      app.data.radarr_data.prompt_confirm = true;

      (app, sync_network_rx)
    }
  }
}
