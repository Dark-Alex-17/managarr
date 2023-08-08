use bimap::BiMap;
use chrono::{DateTime, Utc};
use strum::IntoEnumIterator;

use crate::app::{App, Route};
use crate::models::radarr_models::{
  AddMovieSearchResult, Collection, CollectionMovie, Credit, DiskSpace, DownloadRecord,
  MinimumAvailability, Monitor, Movie, MovieHistoryItem, QueueEvent, Release, ReleaseField,
  RootFolder, Task,
};
use crate::models::{
  BlockSelectionState, HorizontallyScrollableText, ScrollableText, StatefulList, StatefulTable,
  TabRoute, TabState,
};
use crate::network::radarr_network::RadarrEvent;

#[cfg(test)]
#[path = "radarr_tests.rs"]
mod radarr_tests;

#[cfg(test)]
#[path = "radarr_test_utils.rs"]
pub mod radarr_test_utils;

pub struct RadarrData<'a> {
  pub root_folders: StatefulTable<RootFolder>,
  pub disk_space_vec: Vec<DiskSpace>,
  pub version: String,
  pub start_time: DateTime<Utc>,
  pub movies: StatefulTable<Movie>,
  pub filtered_movies: StatefulTable<Movie>,
  pub add_searched_movies: StatefulTable<AddMovieSearchResult>,
  pub monitor_list: StatefulList<Monitor>,
  pub minimum_availability_list: StatefulList<MinimumAvailability>,
  pub quality_profile_list: StatefulList<String>,
  pub root_folder_list: StatefulList<RootFolder>,
  pub selected_block: BlockSelectionState<'a, ActiveRadarrBlock>,
  pub downloads: StatefulTable<DownloadRecord>,
  pub quality_profile_map: BiMap<u64, String>,
  pub tags_map: BiMap<u64, String>,
  pub movie_details: ScrollableText,
  pub file_details: String,
  pub audio_details: String,
  pub video_details: String,
  pub movie_history: StatefulTable<MovieHistoryItem>,
  pub movie_cast: StatefulTable<Credit>,
  pub movie_crew: StatefulTable<Credit>,
  pub movie_releases: StatefulTable<Release>,
  pub movie_releases_sort: StatefulList<ReleaseField>,
  pub collections: StatefulTable<Collection>,
  pub filtered_collections: StatefulTable<Collection>,
  pub collection_movies: StatefulTable<CollectionMovie>,
  pub logs: StatefulList<HorizontallyScrollableText>,
  pub log_details: StatefulList<HorizontallyScrollableText>,
  pub tasks: StatefulTable<Task>,
  pub queued_events: StatefulTable<QueueEvent>,
  pub updates: ScrollableText,
  pub prompt_confirm_action: Option<RadarrEvent>,
  pub main_tabs: TabState,
  pub movie_info_tabs: TabState,
  pub search: HorizontallyScrollableText,
  pub filter: HorizontallyScrollableText,
  pub edit_path: HorizontallyScrollableText,
  pub edit_tags: HorizontallyScrollableText,
  pub edit_monitored: Option<bool>,
  pub edit_search_on_add: Option<bool>,
  pub sort_ascending: Option<bool>,
  pub prompt_confirm: bool,
  pub delete_movie_files: bool,
  pub add_list_exclusion: bool,
  pub is_searching: bool,
  pub is_filtering: bool,
}

impl<'a> RadarrData<'a> {
  pub fn reset_movie_collection_table(&mut self) {
    self.collection_movies = StatefulTable::default();
  }

  pub fn reset_log_details_list(&mut self) {
    self.log_details = StatefulList::default();
  }

  pub fn reset_delete_movie_preferences(&mut self) {
    self.delete_movie_files = false;
    self.add_list_exclusion = false;
  }

  pub fn reset_search(&mut self) {
    self.is_searching = false;
    self.search = HorizontallyScrollableText::default();
    self.filter = HorizontallyScrollableText::default();
    self.filtered_movies = StatefulTable::default();
    self.filtered_collections = StatefulTable::default();
    self.add_searched_movies = StatefulTable::default();
  }

  pub fn reset_filter(&mut self) {
    self.is_filtering = false;
    self.filter = HorizontallyScrollableText::default();
    self.filtered_movies = StatefulTable::default();
    self.filtered_collections = StatefulTable::default();
  }

  pub fn reset_add_edit_media_fields(&mut self) {
    self.edit_monitored = None;
    self.edit_search_on_add = None;
    self.edit_path = HorizontallyScrollableText::default();
    self.edit_tags = HorizontallyScrollableText::default();
    self.reset_preferences_selections();
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
    self.movie_releases_sort = StatefulList::default();
    self.sort_ascending = None;
    self.movie_info_tabs.index = 0;
  }

  pub fn reset_preferences_selections(&mut self) {
    self.monitor_list = StatefulList::default();
    self.minimum_availability_list = StatefulList::default();
    self.quality_profile_list = StatefulList::default();
    self.root_folder_list = StatefulList::default();
  }

  pub fn populate_preferences_lists(&mut self) {
    self.monitor_list.set_items(Vec::from_iter(Monitor::iter()));
    self
      .minimum_availability_list
      .set_items(Vec::from_iter(MinimumAvailability::iter()));
    let mut quality_profile_names: Vec<String> =
      self.quality_profile_map.right_values().cloned().collect();
    quality_profile_names.sort();
    self.quality_profile_list.set_items(quality_profile_names);
    self
      .root_folder_list
      .set_items(self.root_folders.items.to_vec());
  }

  pub fn populate_edit_movie_fields(&mut self) {
    self.populate_preferences_lists();
    let Movie {
      path,
      tags,
      monitored,
      minimum_availability,
      quality_profile_id,
      ..
    } = if self.filtered_movies.items.is_empty() {
      self.movies.current_selection()
    } else {
      self.filtered_movies.current_selection()
    };

    self.edit_path = path.clone().into();
    self.edit_tags = tags
      .iter()
      .map(|tag_id| {
        self
          .tags_map
          .get_by_left(&tag_id.as_u64().unwrap())
          .unwrap()
          .clone()
      })
      .collect::<Vec<String>>()
      .join(", ")
      .into();
    self.edit_monitored = Some(*monitored);

    let minimum_availability_index = self
      .minimum_availability_list
      .items
      .iter()
      .position(|ma| ma == minimum_availability);
    self
      .minimum_availability_list
      .state
      .select(minimum_availability_index);

    let quality_profile_name = self
      .quality_profile_map
      .get_by_left(&quality_profile_id.as_u64().unwrap())
      .unwrap();
    let quality_profile_index = self
      .quality_profile_list
      .items
      .iter()
      .position(|profile| profile == quality_profile_name);
    self
      .quality_profile_list
      .state
      .select(quality_profile_index);
  }

  pub fn populate_edit_collection_fields(&mut self) {
    self.populate_preferences_lists();
    let Collection {
      root_folder_path,
      monitored,
      search_on_add,
      minimum_availability,
      quality_profile_id,
      ..
    } = if self.filtered_collections.items.is_empty() {
      self.collections.current_selection()
    } else {
      self.filtered_collections.current_selection()
    };

    self.edit_path = root_folder_path.clone().unwrap_or_default().into();
    self.edit_monitored = Some(*monitored);
    self.edit_search_on_add = Some(*search_on_add);

    let minimum_availability_index = self
      .minimum_availability_list
      .items
      .iter()
      .position(|ma| ma == minimum_availability);
    self
      .minimum_availability_list
      .state
      .select(minimum_availability_index);

    let quality_profile_name = self
      .quality_profile_map
      .get_by_left(&quality_profile_id.as_u64().unwrap())
      .unwrap();
    let quality_profile_index = self
      .quality_profile_list
      .items
      .iter()
      .position(|profile| profile == quality_profile_name);
    self
      .quality_profile_list
      .state
      .select(quality_profile_index);
  }
}

impl<'a> Default for RadarrData<'a> {
  fn default() -> RadarrData<'a> {
    RadarrData {
      root_folders: StatefulTable::default(),
      disk_space_vec: Vec::new(),
      version: String::default(),
      start_time: DateTime::default(),
      movies: StatefulTable::default(),
      add_searched_movies: StatefulTable::default(),
      monitor_list: StatefulList::default(),
      minimum_availability_list: StatefulList::default(),
      quality_profile_list: StatefulList::default(),
      root_folder_list: StatefulList::default(),
      selected_block: BlockSelectionState::default(),
      filtered_movies: StatefulTable::default(),
      downloads: StatefulTable::default(),
      quality_profile_map: BiMap::default(),
      tags_map: BiMap::default(),
      file_details: String::default(),
      audio_details: String::default(),
      video_details: String::default(),
      movie_details: ScrollableText::default(),
      movie_history: StatefulTable::default(),
      movie_cast: StatefulTable::default(),
      movie_crew: StatefulTable::default(),
      movie_releases: StatefulTable::default(),
      movie_releases_sort: StatefulList::default(),
      collections: StatefulTable::default(),
      filtered_collections: StatefulTable::default(),
      collection_movies: StatefulTable::default(),
      logs: StatefulList::default(),
      log_details: StatefulList::default(),
      tasks: StatefulTable::default(),
      queued_events: StatefulTable::default(),
      updates: ScrollableText::default(),
      prompt_confirm_action: None,
      search: HorizontallyScrollableText::default(),
      filter: HorizontallyScrollableText::default(),
      edit_path: HorizontallyScrollableText::default(),
      edit_tags: HorizontallyScrollableText::default(),
      edit_monitored: None,
      edit_search_on_add: None,
      sort_ascending: None,
      is_searching: false,
      is_filtering: false,
      prompt_confirm: false,
      delete_movie_files: false,
      add_list_exclusion: false,
      main_tabs: TabState::new(vec![
        TabRoute {
          title: "Library",
          route: ActiveRadarrBlock::Movies.into(),
          help: "",
          contextual_help: Some("<a> add | <e> edit | <del> delete | <s> search | <f> filter | <r> refresh | <u> update all | <enter> details | <esc> cancel filter"),
        },
        TabRoute {
          title: "Downloads",
          route: ActiveRadarrBlock::Downloads.into(),
          help: "",
          contextual_help: Some("<r> refresh | <del> delete"),
        },
        TabRoute {
          title: "Collections",
          route: ActiveRadarrBlock::Collections.into(),
          help: "",
          contextual_help: Some("<s> search | <e> edit | <f> filter | <r> refresh | <u> update all | <enter> details | <esc> cancel filter"),
        },
        TabRoute {
          title: "Root Folders",
          route: ActiveRadarrBlock::RootFolders.into(),
          help: "",
          contextual_help: Some("<a> add | <del> delete | <r> refresh"),
        },
        TabRoute {
          title: "System",
          route: ActiveRadarrBlock::System.into(),
          help: "",
          contextual_help: Some("<t> open tasks | <z> open queue | <l> open logs | <u> open updates | <r> refresh")
        }
      ]),
      movie_info_tabs: TabState::new(vec![
        TabRoute {
          title: "Details",
          route: ActiveRadarrBlock::MovieDetails.into(),
          help: "<r> refresh | <u> update | <e> edit | <s> auto search | <esc> close",
          contextual_help: None
        },
        TabRoute {
          title: "History",
          route: ActiveRadarrBlock::MovieHistory.into(),
          help: "<r> refresh | <u> update | <e> edit | <s> auto search | <esc> close",
          contextual_help: None
        },
        TabRoute {
          title: "File",
          route: ActiveRadarrBlock::FileInfo.into(),
          help: "<r> refresh | <u> update | <e> edit | <s> auto search | <esc> close",
          contextual_help: None,
        },
        TabRoute {
          title: "Cast",
          route: ActiveRadarrBlock::Cast.into(),
          help: "<r> refresh | <u> update | <e> edit | <s> auto search | <esc> close",
          contextual_help: None,
        },
        TabRoute {
          title: "Crew",
          route: ActiveRadarrBlock::Crew.into(),
          help: "<r> refresh | <u> update | <e> edit | <s> auto search | <esc> close",
          contextual_help: None,
        },
        TabRoute {
          title: "Manual Search",
          route: ActiveRadarrBlock::ManualSearch.into(),
          help: "<r> refresh | <u> update | <e> edit | <o> sort | <s> auto search | <esc> close",
          contextual_help: Some("<enter> details")
        }
      ]),
    }
  }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum ActiveRadarrBlock {
  AddMovieAlreadyInLibrary,
  AddMovieSearchInput,
  AddMovieSearchResults,
  AddMoviePrompt,
  AddMovieSelectMinimumAvailability,
  AddMovieSelectQualityProfile,
  AddMovieSelectMonitor,
  AddMovieSelectRootFolder,
  AddMovieConfirmPrompt,
  AddMovieTagsInput,
  AddMovieEmptySearchResults,
  AddRootFolderPrompt,
  AutomaticallySearchMoviePrompt,
  Collections,
  CollectionDetails,
  Cast,
  Crew,
  DeleteMoviePrompt,
  DeleteMovieConfirmPrompt,
  DeleteMovieToggleDeleteFile,
  DeleteMovieToggleAddListExclusion,
  DeleteDownloadPrompt,
  DeleteRootFolderPrompt,
  Downloads,
  EditCollectionPrompt,
  EditCollectionConfirmPrompt,
  EditCollectionRootFolderPathInput,
  EditCollectionSelectMinimumAvailability,
  EditCollectionSelectQualityProfile,
  EditCollectionToggleSearchOnAdd,
  EditCollectionToggleMonitored,
  EditMoviePrompt,
  EditMovieConfirmPrompt,
  EditMoviePathInput,
  EditMovieSelectMinimumAvailability,
  EditMovieSelectQualityProfile,
  EditMovieTagsInput,
  EditMovieToggleMonitored,
  FileInfo,
  FilterCollections,
  FilterMovies,
  ManualSearch,
  ManualSearchSortPrompt,
  ManualSearchConfirmPrompt,
  MovieDetails,
  MovieHistory,
  #[default]
  Movies,
  RootFolders,
  System,
  SystemLogs,
  SystemQueue,
  SystemTasks,
  SystemTaskStartConfirmPrompt,
  SystemUpdates,
  UpdateAndScanPrompt,
  UpdateAllCollectionsPrompt,
  UpdateAllMoviesPrompt,
  UpdateDownloadsPrompt,
  SearchMovie,
  SearchCollection,
  ViewMovieOverview,
}

pub static ADD_MOVIE_BLOCKS: [ActiveRadarrBlock; 10] = [
  ActiveRadarrBlock::AddMovieSearchInput,
  ActiveRadarrBlock::AddMovieSearchResults,
  ActiveRadarrBlock::AddMovieEmptySearchResults,
  ActiveRadarrBlock::AddMoviePrompt,
  ActiveRadarrBlock::AddMovieSelectMinimumAvailability,
  ActiveRadarrBlock::AddMovieSelectMonitor,
  ActiveRadarrBlock::AddMovieSelectQualityProfile,
  ActiveRadarrBlock::AddMovieSelectRootFolder,
  ActiveRadarrBlock::AddMovieAlreadyInLibrary,
  ActiveRadarrBlock::AddMovieTagsInput,
];
pub static ADD_MOVIE_SELECTION_BLOCKS: [ActiveRadarrBlock; 6] = [
  ActiveRadarrBlock::AddMovieSelectRootFolder,
  ActiveRadarrBlock::AddMovieSelectMonitor,
  ActiveRadarrBlock::AddMovieSelectMinimumAvailability,
  ActiveRadarrBlock::AddMovieSelectQualityProfile,
  ActiveRadarrBlock::AddMovieTagsInput,
  ActiveRadarrBlock::AddMovieConfirmPrompt,
];
pub static EDIT_COLLECTION_BLOCKS: [ActiveRadarrBlock; 7] = [
  ActiveRadarrBlock::EditCollectionPrompt,
  ActiveRadarrBlock::EditCollectionConfirmPrompt,
  ActiveRadarrBlock::EditCollectionRootFolderPathInput,
  ActiveRadarrBlock::EditCollectionSelectMinimumAvailability,
  ActiveRadarrBlock::EditCollectionSelectQualityProfile,
  ActiveRadarrBlock::EditCollectionToggleSearchOnAdd,
  ActiveRadarrBlock::EditCollectionToggleMonitored,
];
pub static EDIT_COLLECTION_SELECTION_BLOCKS: [ActiveRadarrBlock; 6] = [
  ActiveRadarrBlock::EditCollectionToggleMonitored,
  ActiveRadarrBlock::EditCollectionSelectMinimumAvailability,
  ActiveRadarrBlock::EditCollectionSelectQualityProfile,
  ActiveRadarrBlock::EditCollectionRootFolderPathInput,
  ActiveRadarrBlock::EditCollectionToggleSearchOnAdd,
  ActiveRadarrBlock::EditCollectionConfirmPrompt,
];
pub static EDIT_MOVIE_BLOCKS: [ActiveRadarrBlock; 7] = [
  ActiveRadarrBlock::EditMoviePrompt,
  ActiveRadarrBlock::EditMovieConfirmPrompt,
  ActiveRadarrBlock::EditMoviePathInput,
  ActiveRadarrBlock::EditMovieSelectMinimumAvailability,
  ActiveRadarrBlock::EditMovieSelectQualityProfile,
  ActiveRadarrBlock::EditMovieTagsInput,
  ActiveRadarrBlock::EditMovieToggleMonitored,
];
pub static EDIT_MOVIE_SELECTION_BLOCKS: [ActiveRadarrBlock; 6] = [
  ActiveRadarrBlock::EditMovieToggleMonitored,
  ActiveRadarrBlock::EditMovieSelectMinimumAvailability,
  ActiveRadarrBlock::EditMovieSelectQualityProfile,
  ActiveRadarrBlock::EditMoviePathInput,
  ActiveRadarrBlock::EditMovieTagsInput,
  ActiveRadarrBlock::EditMovieConfirmPrompt,
];
pub static MOVIE_DETAILS_BLOCKS: [ActiveRadarrBlock; 10] = [
  ActiveRadarrBlock::MovieDetails,
  ActiveRadarrBlock::MovieHistory,
  ActiveRadarrBlock::FileInfo,
  ActiveRadarrBlock::Cast,
  ActiveRadarrBlock::Crew,
  ActiveRadarrBlock::AutomaticallySearchMoviePrompt,
  ActiveRadarrBlock::UpdateAndScanPrompt,
  ActiveRadarrBlock::ManualSearch,
  ActiveRadarrBlock::ManualSearchSortPrompt,
  ActiveRadarrBlock::ManualSearchConfirmPrompt,
];
pub static COLLECTION_DETAILS_BLOCKS: [ActiveRadarrBlock; 2] = [
  ActiveRadarrBlock::CollectionDetails,
  ActiveRadarrBlock::ViewMovieOverview,
];
pub static SEARCH_BLOCKS: [ActiveRadarrBlock; 2] = [
  ActiveRadarrBlock::SearchMovie,
  ActiveRadarrBlock::SearchCollection,
];
pub static FILTER_BLOCKS: [ActiveRadarrBlock; 2] = [
  ActiveRadarrBlock::FilterMovies,
  ActiveRadarrBlock::FilterCollections,
];
pub static DELETE_MOVIE_BLOCKS: [ActiveRadarrBlock; 4] = [
  ActiveRadarrBlock::DeleteMoviePrompt,
  ActiveRadarrBlock::DeleteMovieConfirmPrompt,
  ActiveRadarrBlock::DeleteMovieToggleDeleteFile,
  ActiveRadarrBlock::DeleteMovieToggleAddListExclusion,
];
pub static DELETE_MOVIE_SELECTION_BLOCKS: [ActiveRadarrBlock; 3] = [
  ActiveRadarrBlock::DeleteMovieToggleDeleteFile,
  ActiveRadarrBlock::DeleteMovieToggleAddListExclusion,
  ActiveRadarrBlock::DeleteMovieConfirmPrompt,
];
pub static SYSTEM_DETAILS_BLOCKS: [ActiveRadarrBlock; 5] = [
  ActiveRadarrBlock::SystemLogs,
  ActiveRadarrBlock::SystemQueue,
  ActiveRadarrBlock::SystemTasks,
  ActiveRadarrBlock::SystemTaskStartConfirmPrompt,
  ActiveRadarrBlock::SystemUpdates,
];

impl From<ActiveRadarrBlock> for Route {
  fn from(active_radarr_block: ActiveRadarrBlock) -> Route {
    Route::Radarr(active_radarr_block, None)
  }
}

impl From<(ActiveRadarrBlock, Option<ActiveRadarrBlock>)> for Route {
  fn from(value: (ActiveRadarrBlock, Option<ActiveRadarrBlock>)) -> Route {
    Route::Radarr(value.0, value.1)
  }
}

impl<'a> App<'a> {
  pub(super) async fn dispatch_by_radarr_block(&mut self, active_radarr_block: &ActiveRadarrBlock) {
    match active_radarr_block {
      ActiveRadarrBlock::Collections => {
        self
          .dispatch_network_event(RadarrEvent::GetCollections.into())
          .await;
      }
      ActiveRadarrBlock::CollectionDetails => {
        self.is_loading = true;
        self.populate_movie_collection_table().await;
        self.is_loading = false;
      }
      ActiveRadarrBlock::Downloads => {
        self
          .dispatch_network_event(RadarrEvent::GetDownloads.into())
          .await;
      }
      ActiveRadarrBlock::RootFolders => {
        self
          .dispatch_network_event(RadarrEvent::GetRootFolders.into())
          .await;
      }
      ActiveRadarrBlock::Movies => {
        self
          .dispatch_network_event(RadarrEvent::GetMovies.into())
          .await;
        self
          .dispatch_network_event(RadarrEvent::GetDownloads.into())
          .await;
      }
      ActiveRadarrBlock::System => {
        self
          .dispatch_network_event(RadarrEvent::GetTasks.into())
          .await;
        self
          .dispatch_network_event(RadarrEvent::GetQueuedEvents.into())
          .await;
        self
          .dispatch_network_event(RadarrEvent::GetLogs.into())
          .await;
      }
      ActiveRadarrBlock::SystemUpdates => {
        self
          .dispatch_network_event(RadarrEvent::GetUpdates.into())
          .await;
      }
      ActiveRadarrBlock::AddMovieSearchResults => {
        self
          .dispatch_network_event(RadarrEvent::SearchNewMovie.into())
          .await;
      }
      ActiveRadarrBlock::MovieDetails | ActiveRadarrBlock::FileInfo => {
        self
          .dispatch_network_event(RadarrEvent::GetMovieDetails.into())
          .await;
      }
      ActiveRadarrBlock::MovieHistory => {
        self
          .dispatch_network_event(RadarrEvent::GetMovieHistory.into())
          .await;
      }
      ActiveRadarrBlock::Cast | ActiveRadarrBlock::Crew => {
        if self.data.radarr_data.movie_cast.items.is_empty()
          || self.data.radarr_data.movie_crew.items.is_empty()
        {
          self
            .dispatch_network_event(RadarrEvent::GetMovieCredits.into())
            .await;
        }
      }
      ActiveRadarrBlock::ManualSearch => {
        if self.data.radarr_data.movie_releases.items.is_empty() {
          self
            .dispatch_network_event(RadarrEvent::GetReleases.into())
            .await;
        }
      }
      _ => (),
    }

    self.check_for_prompt_action().await;
    self.reset_tick_count();
  }

  async fn check_for_prompt_action(&mut self) {
    if self.data.radarr_data.prompt_confirm {
      self.data.radarr_data.prompt_confirm = false;
      if let Some(radarr_event) = &self.data.radarr_data.prompt_confirm_action {
        self.dispatch_network_event((*radarr_event).into()).await;
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
        .dispatch_network_event(RadarrEvent::GetTags.into())
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

    if self.should_refresh {
      self.dispatch_by_radarr_block(&active_radarr_block).await;
    }

    if self.is_routing || self.tick_count % self.tick_until_poll == 0 {
      self.dispatch_by_radarr_block(&active_radarr_block).await;
      self.refresh_metadata().await;
    }
  }

  async fn refresh_metadata(&mut self) {
    self
      .dispatch_network_event(RadarrEvent::GetQualityProfiles.into())
      .await;
    self
      .dispatch_network_event(RadarrEvent::GetTags.into())
      .await;
    self
      .dispatch_network_event(RadarrEvent::GetRootFolders.into())
      .await;
    self
      .dispatch_network_event(RadarrEvent::GetDownloads.into())
      .await;
  }

  async fn populate_movie_collection_table(&mut self) {
    let collection_movies = if !self.data.radarr_data.filtered_collections.items.is_empty() {
      self
        .data
        .radarr_data
        .filtered_collections
        .current_selection()
        .clone()
        .movies
        .unwrap_or_default()
    } else {
      self
        .data
        .radarr_data
        .collections
        .current_selection()
        .clone()
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
