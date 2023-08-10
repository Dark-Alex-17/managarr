use crate::app::context_clues::build_context_clue_string;
use crate::app::radarr::radarr_context_clues::{
  COLLECTIONS_CONTEXT_CLUES, DOWNLOADS_CONTEXT_CLUES, INDEXERS_CONTEXT_CLUES,
  LIBRARY_CONTEXT_CLUES, MANUAL_MOVIE_SEARCH_CONTEXTUAL_CONTEXT_CLUES,
  MANUAL_MOVIE_SEARCH_CONTEXT_CLUES, MOVIE_DETAILS_CONTEXT_CLUES, ROOT_FOLDERS_CONTEXT_CLUES,
  SYSTEM_CONTEXT_CLUES,
};
use crate::models::radarr_models::{
  AddMovieSearchResult, Collection, CollectionMovie, Credit, DiskSpace, DownloadRecord, Indexer,
  IndexerSettings, Movie, MovieHistoryItem, QueueEvent, Release, ReleaseField, RootFolder, Task,
};
use crate::models::servarr_data::radarr::modals::{
  AddMovieModal, EditCollectionModal, EditMovieModal,
};
use crate::models::{
  BlockSelectionState, HorizontallyScrollableText, Route, ScrollableText, StatefulList,
  StatefulTable, TabRoute, TabState,
};
use crate::network::radarr_network::RadarrEvent;
use bimap::BiMap;
use chrono::{DateTime, Utc};
use strum::EnumIter;

#[cfg(test)]
#[path = "radarr_data_tests.rs"]
mod radarr_data_tests;

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
  pub selected_block: BlockSelectionState<'a, ActiveRadarrBlock>,
  pub downloads: StatefulTable<DownloadRecord>,
  pub indexers: StatefulTable<Indexer>,
  pub indexer_settings: Option<IndexerSettings>,
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
  pub search: Option<HorizontallyScrollableText>,
  pub filter: Option<HorizontallyScrollableText>,
  pub add_movie_modal: Option<AddMovieModal>,
  pub edit_movie_modal: Option<EditMovieModal>,
  pub edit_collection_modal: Option<EditCollectionModal>,
  pub edit_root_folder: Option<HorizontallyScrollableText>,
  pub sort_ascending: Option<bool>,
  pub prompt_confirm: bool,
  pub delete_movie_files: bool,
  pub add_list_exclusion: bool,
  pub is_searching: bool,
  pub is_filtering: bool,
}

impl<'a> RadarrData<'a> {
  pub fn reset_delete_movie_preferences(&mut self) {
    self.delete_movie_files = false;
    self.add_list_exclusion = false;
  }

  pub fn reset_search(&mut self) {
    self.is_searching = false;
    self.search = None;
    self.filter = None;
    self.filtered_movies = StatefulTable::default();
    self.filtered_collections = StatefulTable::default();
    self.add_searched_movies = StatefulTable::default();
  }

  pub fn reset_filter(&mut self) {
    self.is_filtering = false;
    self.filter = None;
    self.filtered_movies = StatefulTable::default();
    self.filtered_collections = StatefulTable::default();
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
      selected_block: BlockSelectionState::default(),
      filtered_movies: StatefulTable::default(),
      downloads: StatefulTable::default(),
      indexers: StatefulTable::default(),
      indexer_settings: None,
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
      search: None,
      filter: None,
      add_movie_modal: None,
      edit_movie_modal: None,
      edit_collection_modal: None,
      edit_root_folder: None,
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
          help: String::new(),
          contextual_help: Some(build_context_clue_string(&LIBRARY_CONTEXT_CLUES)),
        },
        TabRoute {
          title: "Downloads",
          route: ActiveRadarrBlock::Downloads.into(),
          help: String::new(),
          contextual_help: Some(build_context_clue_string(&DOWNLOADS_CONTEXT_CLUES)),
        },
        TabRoute {
          title: "Collections",
          route: ActiveRadarrBlock::Collections.into(),
          help: String::new(),
          contextual_help: Some(build_context_clue_string(&COLLECTIONS_CONTEXT_CLUES)),
        },
        TabRoute {
          title: "Root Folders",
          route: ActiveRadarrBlock::RootFolders.into(),
          help: String::new(),
          contextual_help: Some(build_context_clue_string(&ROOT_FOLDERS_CONTEXT_CLUES)),
        },
        TabRoute {
          title: "Indexers",
          route: ActiveRadarrBlock::Indexers.into(),
          help: String::new(),
          contextual_help: Some(build_context_clue_string(&INDEXERS_CONTEXT_CLUES)),
        },
        TabRoute {
          title: "System",
          route: ActiveRadarrBlock::System.into(),
          help: String::new(),
          contextual_help: Some(build_context_clue_string(&SYSTEM_CONTEXT_CLUES)),
        },
      ]),
      movie_info_tabs: TabState::new(vec![
        TabRoute {
          title: "Details",
          route: ActiveRadarrBlock::MovieDetails.into(),
          help: build_context_clue_string(&MOVIE_DETAILS_CONTEXT_CLUES),
          contextual_help: None,
        },
        TabRoute {
          title: "History",
          route: ActiveRadarrBlock::MovieHistory.into(),
          help: build_context_clue_string(&MOVIE_DETAILS_CONTEXT_CLUES),
          contextual_help: None,
        },
        TabRoute {
          title: "File",
          route: ActiveRadarrBlock::FileInfo.into(),
          help: build_context_clue_string(&MOVIE_DETAILS_CONTEXT_CLUES),
          contextual_help: None,
        },
        TabRoute {
          title: "Cast",
          route: ActiveRadarrBlock::Cast.into(),
          help: build_context_clue_string(&MOVIE_DETAILS_CONTEXT_CLUES),
          contextual_help: None,
        },
        TabRoute {
          title: "Crew",
          route: ActiveRadarrBlock::Crew.into(),
          help: build_context_clue_string(&MOVIE_DETAILS_CONTEXT_CLUES),
          contextual_help: None,
        },
        TabRoute {
          title: "Manual Search",
          route: ActiveRadarrBlock::ManualSearch.into(),
          help: build_context_clue_string(&MANUAL_MOVIE_SEARCH_CONTEXT_CLUES),
          contextual_help: Some(build_context_clue_string(
            &MANUAL_MOVIE_SEARCH_CONTEXTUAL_CONTEXT_CLUES,
          )),
        },
      ]),
    }
  }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default, EnumIter)]
pub enum ActiveRadarrBlock {
  AddIndexer,
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
  DeleteDownloadPrompt,
  DeleteIndexerPrompt,
  DeleteMoviePrompt,
  DeleteMovieConfirmPrompt,
  DeleteMovieToggleDeleteFile,
  DeleteMovieToggleAddListExclusion,
  DeleteRootFolderPrompt,
  Downloads,
  EditCollectionPrompt,
  EditCollectionConfirmPrompt,
  EditCollectionRootFolderPathInput,
  EditCollectionSelectMinimumAvailability,
  EditCollectionSelectQualityProfile,
  EditCollectionToggleSearchOnAdd,
  EditCollectionToggleMonitored,
  EditIndexer,
  EditMoviePrompt,
  EditMovieConfirmPrompt,
  EditMoviePathInput,
  EditMovieSelectMinimumAvailability,
  EditMovieSelectQualityProfile,
  EditMovieTagsInput,
  EditMovieToggleMonitored,
  FileInfo,
  FilterCollections,
  FilterCollectionsError,
  FilterMovies,
  FilterMoviesError,
  Indexers,
  IndexerSettingsPrompt,
  IndexerSettingsAvailabilityDelayInput,
  IndexerSettingsConfirmPrompt,
  IndexerSettingsMaximumSizeInput,
  IndexerSettingsMinimumAgeInput,
  IndexerSettingsRetentionInput,
  IndexerSettingsRssSyncIntervalInput,
  IndexerSettingsToggleAllowHardcodedSubs,
  IndexerSettingsTogglePreferIndexerFlags,
  IndexerSettingsWhitelistedSubtitleTagsInput,
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
  SystemQueuedEvents,
  SystemTasks,
  SystemTaskStartConfirmPrompt,
  SystemUpdates,
  UpdateAndScanPrompt,
  UpdateAllCollectionsPrompt,
  UpdateAllMoviesPrompt,
  UpdateDownloadsPrompt,
  SearchCollection,
  SearchCollectionError,
  SearchMovie,
  SearchMovieError,
  ViewMovieOverview,
}

pub static LIBRARY_BLOCKS: [ActiveRadarrBlock; 6] = [
  ActiveRadarrBlock::Movies,
  ActiveRadarrBlock::SearchMovie,
  ActiveRadarrBlock::SearchMovieError,
  ActiveRadarrBlock::FilterMovies,
  ActiveRadarrBlock::FilterMoviesError,
  ActiveRadarrBlock::UpdateAllMoviesPrompt,
];
pub static COLLECTIONS_BLOCKS: [ActiveRadarrBlock; 6] = [
  ActiveRadarrBlock::Collections,
  ActiveRadarrBlock::SearchCollection,
  ActiveRadarrBlock::SearchCollectionError,
  ActiveRadarrBlock::FilterCollections,
  ActiveRadarrBlock::FilterCollectionsError,
  ActiveRadarrBlock::UpdateAllCollectionsPrompt,
];
pub static INDEXERS_BLOCKS: [ActiveRadarrBlock; 4] = [
  ActiveRadarrBlock::AddIndexer,
  ActiveRadarrBlock::EditIndexer,
  ActiveRadarrBlock::DeleteIndexerPrompt,
  ActiveRadarrBlock::Indexers,
];
pub static ROOT_FOLDERS_BLOCKS: [ActiveRadarrBlock; 3] = [
  ActiveRadarrBlock::RootFolders,
  ActiveRadarrBlock::AddRootFolderPrompt,
  ActiveRadarrBlock::DeleteRootFolderPrompt,
];
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
pub static DOWNLOADS_BLOCKS: [ActiveRadarrBlock; 3] = [
  ActiveRadarrBlock::Downloads,
  ActiveRadarrBlock::DeleteDownloadPrompt,
  ActiveRadarrBlock::UpdateDownloadsPrompt,
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
pub static INDEXER_SETTINGS_BLOCKS: [ActiveRadarrBlock; 10] = [
  ActiveRadarrBlock::IndexerSettingsPrompt,
  ActiveRadarrBlock::IndexerSettingsAvailabilityDelayInput,
  ActiveRadarrBlock::IndexerSettingsConfirmPrompt,
  ActiveRadarrBlock::IndexerSettingsMaximumSizeInput,
  ActiveRadarrBlock::IndexerSettingsMinimumAgeInput,
  ActiveRadarrBlock::IndexerSettingsRetentionInput,
  ActiveRadarrBlock::IndexerSettingsRssSyncIntervalInput,
  ActiveRadarrBlock::IndexerSettingsToggleAllowHardcodedSubs,
  ActiveRadarrBlock::IndexerSettingsTogglePreferIndexerFlags,
  ActiveRadarrBlock::IndexerSettingsWhitelistedSubtitleTagsInput,
];
pub static INDEXER_SETTINGS_SELECTION_BLOCKS: [ActiveRadarrBlock; 9] = [
  ActiveRadarrBlock::IndexerSettingsMinimumAgeInput,
  ActiveRadarrBlock::IndexerSettingsRetentionInput,
  ActiveRadarrBlock::IndexerSettingsMaximumSizeInput,
  ActiveRadarrBlock::IndexerSettingsTogglePreferIndexerFlags,
  ActiveRadarrBlock::IndexerSettingsAvailabilityDelayInput,
  ActiveRadarrBlock::IndexerSettingsRssSyncIntervalInput,
  ActiveRadarrBlock::IndexerSettingsWhitelistedSubtitleTagsInput,
  ActiveRadarrBlock::IndexerSettingsToggleAllowHardcodedSubs,
  ActiveRadarrBlock::IndexerSettingsConfirmPrompt,
];
pub static SYSTEM_DETAILS_BLOCKS: [ActiveRadarrBlock; 5] = [
  ActiveRadarrBlock::SystemLogs,
  ActiveRadarrBlock::SystemQueuedEvents,
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

#[allow(dead_code)] // Returning to this work tomorrow
pub struct EditIndexerSettings {
  pub allow_hardcoded_subs: bool,
  pub availability_delay: HorizontallyScrollableText,
  pub id: HorizontallyScrollableText,
  pub maximum_size: HorizontallyScrollableText,
  pub minimum_age: HorizontallyScrollableText,
  pub prefer_indexer_flags: bool,
  pub retention: HorizontallyScrollableText,
  pub rss_sync_interval: HorizontallyScrollableText,
  pub whitelisted_hardcoded_subs: HorizontallyScrollableText,
}
