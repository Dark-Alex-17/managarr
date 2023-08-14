#[cfg(test)]
mod tests {
  use pretty_assertions::{assert_eq, assert_str_eq};
  use rstest::rstest;
  use strum::IntoEnumIterator;

  use crate::app::key_binding::DEFAULT_KEYBINDINGS;
  use crate::app::App;
  use crate::handlers::radarr_handlers::{handle_change_tab_left_right_keys, RadarrHandler};
  use crate::handlers::KeyEventHandler;
  use crate::models::radarr_models::Movie;
  use crate::models::servarr_data::radarr::radarr_data::ActiveRadarrBlock;
  use crate::models::{HorizontallyScrollableText, StatefulTable};
  use crate::utils::strip_non_search_characters;
  use crate::{
    extended_stateful_iterable_vec, filter_table, search_table, test_handler_delegation,
  };

  #[test]
  fn test_search_table_macro() {
    let mut app = App::default();
    app.push_navigation_stack(ActiveRadarrBlock::Movies.into());
    app.push_navigation_stack(ActiveRadarrBlock::SearchMovie.into());
    app
      .data
      .radarr_data
      .movies
      .set_items(extended_stateful_iterable_vec!(
        Movie,
        HorizontallyScrollableText
      ));
    app.data.radarr_data.search = Some("Test 2".into());
    app.data.radarr_data.is_searching = true;
    app.should_ignore_quit_key = true;

    search_table!(app, movies, ActiveRadarrBlock::SearchMovieError);

    assert_str_eq!(
      app.data.radarr_data.movies.current_selection().title.text,
      "Test 2"
    );
    assert_eq!(app.get_current_route(), &ActiveRadarrBlock::Movies.into());
    assert!(!app.data.radarr_data.is_searching);
    assert!(!app.should_ignore_quit_key);
    assert!(app.data.radarr_data.search.is_none());
  }

  #[test]
  fn test_search_table_macro_empty_search() {
    let mut app = App::default();
    app.push_navigation_stack(ActiveRadarrBlock::Movies.into());
    app.push_navigation_stack(ActiveRadarrBlock::SearchMovie.into());
    app
      .data
      .radarr_data
      .movies
      .set_items(extended_stateful_iterable_vec!(
        Movie,
        HorizontallyScrollableText
      ));
    app.data.radarr_data.search = Some("".into());
    app.data.radarr_data.is_searching = true;
    app.should_ignore_quit_key = true;

    search_table!(app, movies, ActiveRadarrBlock::SearchMovieError);

    assert_str_eq!(
      app.data.radarr_data.movies.current_selection().title.text,
      "Test 1"
    );
    assert_eq!(app.get_current_route(), &ActiveRadarrBlock::Movies.into());
    assert!(!app.data.radarr_data.is_searching);
    assert!(!app.should_ignore_quit_key);
    assert!(app.data.radarr_data.search.is_none());
  }

  #[test]
  fn test_search_table_macro_error_on_no_search_hits() {
    let mut app = App::default();
    app.push_navigation_stack(ActiveRadarrBlock::Movies.into());
    app.push_navigation_stack(ActiveRadarrBlock::SearchMovie.into());
    app
      .data
      .radarr_data
      .movies
      .set_items(extended_stateful_iterable_vec!(
        Movie,
        HorizontallyScrollableText
      ));
    app.data.radarr_data.search = Some("Test 5".into());
    app.data.radarr_data.is_searching = true;
    app.should_ignore_quit_key = true;

    search_table!(app, movies, ActiveRadarrBlock::SearchMovieError);

    assert_str_eq!(
      app.data.radarr_data.movies.current_selection().title.text,
      "Test 1"
    );
    assert_eq!(
      app.get_current_route(),
      &ActiveRadarrBlock::SearchMovieError.into()
    );
    assert!(!app.data.radarr_data.is_searching);
    assert!(!app.should_ignore_quit_key);
    assert!(app.data.radarr_data.search.is_none());
  }

  #[test]
  fn test_filter_table_macro() {
    let mut app = App::default();
    app.push_navigation_stack(ActiveRadarrBlock::Movies.into());
    app.push_navigation_stack(ActiveRadarrBlock::FilterMovies.into());
    app
      .data
      .radarr_data
      .movies
      .set_items(extended_stateful_iterable_vec!(
        Movie,
        HorizontallyScrollableText
      ));
    app.data.radarr_data.filter = Some("Test 2".into());
    app.data.radarr_data.is_filtering = true;
    app.should_ignore_quit_key = true;

    filter_table!(
      app,
      movies,
      filtered_movies,
      ActiveRadarrBlock::FilterMoviesError
    );

    assert!(app.data.radarr_data.filtered_movies.is_some());
    assert_eq!(
      app
        .data
        .radarr_data
        .filtered_movies
        .as_ref()
        .unwrap()
        .items
        .len(),
      1
    );
    assert_str_eq!(
      app.data.radarr_data.filtered_movies.as_ref().unwrap().items[0]
        .title
        .text,
      "Test 2"
    );
    assert_eq!(app.get_current_route(), &ActiveRadarrBlock::Movies.into());
    assert!(!app.data.radarr_data.is_filtering);
    assert!(!app.should_ignore_quit_key);
    assert!(app.data.radarr_data.filter.is_none());
  }

  #[test]
  fn test_filter_table_macro_reset_and_pop_on_empty_filter() {
    let mut app = App::default();
    app.push_navigation_stack(ActiveRadarrBlock::Movies.into());
    app.push_navigation_stack(ActiveRadarrBlock::FilterMovies.into());
    app
      .data
      .radarr_data
      .movies
      .set_items(extended_stateful_iterable_vec!(
        Movie,
        HorizontallyScrollableText
      ));
    app.data.radarr_data.filter = Some("".into());
    app.data.radarr_data.is_filtering = true;
    app.should_ignore_quit_key = true;

    filter_table!(
      app,
      movies,
      filtered_movies,
      ActiveRadarrBlock::FilterMoviesError
    );

    assert!(app.data.radarr_data.filtered_movies.is_none());
    assert_eq!(app.get_current_route(), &ActiveRadarrBlock::Movies.into());
    assert!(!app.data.radarr_data.is_filtering);
    assert!(!app.should_ignore_quit_key);
    assert!(app.data.radarr_data.filter.is_none());
  }

  #[test]
  fn test_filter_table_error_on_no_filter_matches() {
    let mut app = App::default();
    app.push_navigation_stack(ActiveRadarrBlock::Movies.into());
    app.push_navigation_stack(ActiveRadarrBlock::FilterMovies.into());
    app
      .data
      .radarr_data
      .movies
      .set_items(extended_stateful_iterable_vec!(
        Movie,
        HorizontallyScrollableText
      ));
    app.data.radarr_data.filter = Some("Test 5".into());
    app.data.radarr_data.is_filtering = true;
    app.should_ignore_quit_key = true;

    filter_table!(
      app,
      movies,
      filtered_movies,
      ActiveRadarrBlock::FilterMoviesError
    );

    assert!(app.data.radarr_data.filtered_movies.is_none());
    assert_eq!(
      app.get_current_route(),
      &ActiveRadarrBlock::FilterMoviesError.into()
    );
    assert!(!app.data.radarr_data.is_filtering);
    assert!(!app.should_ignore_quit_key);
    assert!(app.data.radarr_data.filter.is_none());
  }

  #[test]
  fn test_filter_table_macro_error_on_none_filter() {
    let mut app = App::default();
    app.push_navigation_stack(ActiveRadarrBlock::Movies.into());
    app.push_navigation_stack(ActiveRadarrBlock::FilterMovies.into());
    app
      .data
      .radarr_data
      .movies
      .set_items(extended_stateful_iterable_vec!(
        Movie,
        HorizontallyScrollableText
      ));
    app.data.radarr_data.is_filtering = true;
    app.should_ignore_quit_key = true;

    filter_table!(
      app,
      movies,
      filtered_movies,
      ActiveRadarrBlock::FilterMoviesError
    );

    assert!(app.data.radarr_data.filtered_movies.is_none());
    assert_eq!(
      app.get_current_route(),
      &ActiveRadarrBlock::FilterMoviesError.into()
    );
    assert!(!app.data.radarr_data.is_filtering);
    assert!(!app.should_ignore_quit_key);
    assert!(app.data.radarr_data.filter.is_none());
  }

  #[rstest]
  #[case(0, ActiveRadarrBlock::System, ActiveRadarrBlock::Downloads)]
  #[case(1, ActiveRadarrBlock::Movies, ActiveRadarrBlock::Collections)]
  #[case(2, ActiveRadarrBlock::Downloads, ActiveRadarrBlock::RootFolders)]
  #[case(3, ActiveRadarrBlock::Collections, ActiveRadarrBlock::Indexers)]
  #[case(4, ActiveRadarrBlock::RootFolders, ActiveRadarrBlock::System)]
  #[case(5, ActiveRadarrBlock::Indexers, ActiveRadarrBlock::Movies)]
  fn test_radarr_handler_change_tab_left_right_keys(
    #[case] index: usize,
    #[case] left_block: ActiveRadarrBlock,
    #[case] right_block: ActiveRadarrBlock,
  ) {
    let mut app = App::default();
    app.data.radarr_data.main_tabs.set_index(index);

    handle_change_tab_left_right_keys(&mut app, &DEFAULT_KEYBINDINGS.left.key);

    assert_eq!(
      app.data.radarr_data.main_tabs.get_active_route(),
      &left_block.into()
    );
    assert_eq!(app.get_current_route(), &left_block.into());

    app.data.radarr_data.main_tabs.set_index(index);

    handle_change_tab_left_right_keys(&mut app, &DEFAULT_KEYBINDINGS.right.key);

    assert_eq!(
      app.data.radarr_data.main_tabs.get_active_route(),
      &right_block.into()
    );
    assert_eq!(app.get_current_route(), &right_block.into());
  }

  #[rstest]
  fn test_delegates_system_blocks_to_system_handler(
    #[values(
      ActiveRadarrBlock::System,
      ActiveRadarrBlock::SystemLogs,
      ActiveRadarrBlock::SystemQueuedEvents,
      ActiveRadarrBlock::SystemTasks,
      ActiveRadarrBlock::SystemTaskStartConfirmPrompt,
      ActiveRadarrBlock::SystemUpdates
    )]
    active_radarr_block: ActiveRadarrBlock,
  ) {
    test_handler_delegation!(
      RadarrHandler,
      ActiveRadarrBlock::System,
      active_radarr_block
    );
  }

  #[rstest]
  fn test_delegates_library_blocks_to_library_handler(
    #[values(
      ActiveRadarrBlock::Movies,
      ActiveRadarrBlock::SearchMovie,
      ActiveRadarrBlock::SearchMovieError,
      ActiveRadarrBlock::FilterMovies,
      ActiveRadarrBlock::FilterMoviesError,
      ActiveRadarrBlock::UpdateAllMoviesPrompt,
      ActiveRadarrBlock::AddMovieSearchInput,
      ActiveRadarrBlock::AddMovieSearchResults,
      ActiveRadarrBlock::AddMoviePrompt,
      ActiveRadarrBlock::AddMovieSelectMonitor,
      ActiveRadarrBlock::AddMovieSelectMinimumAvailability,
      ActiveRadarrBlock::AddMovieSelectQualityProfile,
      ActiveRadarrBlock::AddMovieSelectRootFolder,
      ActiveRadarrBlock::AddMovieAlreadyInLibrary,
      ActiveRadarrBlock::AddMovieTagsInput,
      ActiveRadarrBlock::MovieDetails,
      ActiveRadarrBlock::MovieHistory,
      ActiveRadarrBlock::FileInfo,
      ActiveRadarrBlock::Cast,
      ActiveRadarrBlock::Crew,
      ActiveRadarrBlock::AutomaticallySearchMoviePrompt,
      ActiveRadarrBlock::UpdateAndScanPrompt,
      ActiveRadarrBlock::ManualSearch,
      ActiveRadarrBlock::ManualSearchConfirmPrompt,
      ActiveRadarrBlock::EditMoviePrompt,
      ActiveRadarrBlock::EditMoviePathInput,
      ActiveRadarrBlock::EditMovieSelectMinimumAvailability,
      ActiveRadarrBlock::EditMovieSelectQualityProfile,
      ActiveRadarrBlock::EditMovieTagsInput,
      ActiveRadarrBlock::DeleteMoviePrompt
    )]
    active_radarr_block: ActiveRadarrBlock,
  ) {
    test_handler_delegation!(
      RadarrHandler,
      ActiveRadarrBlock::Movies,
      active_radarr_block
    );
  }

  #[rstest]
  fn test_delegates_collections_blocks_to_collections_handler(
    #[values(
      ActiveRadarrBlock::Collections,
      ActiveRadarrBlock::SearchCollection,
      ActiveRadarrBlock::SearchCollectionError,
      ActiveRadarrBlock::FilterCollections,
      ActiveRadarrBlock::FilterCollectionsError,
      ActiveRadarrBlock::UpdateAllCollectionsPrompt,
      ActiveRadarrBlock::CollectionDetails,
      ActiveRadarrBlock::ViewMovieOverview,
      ActiveRadarrBlock::EditCollectionPrompt,
      ActiveRadarrBlock::EditCollectionRootFolderPathInput,
      ActiveRadarrBlock::EditCollectionSelectMinimumAvailability,
      ActiveRadarrBlock::EditCollectionSelectQualityProfile
    )]
    active_radarr_block: ActiveRadarrBlock,
  ) {
    test_handler_delegation!(
      RadarrHandler,
      ActiveRadarrBlock::Collections,
      active_radarr_block
    );
  }

  #[rstest]
  fn test_delegates_indexers_blocks_to_indexers_handler(
    // Add these once implemented:
    // ActiveRadarrBlock::AddIndexer,
    // ActiveRadarrBlock::EditIndexer,
    #[values(
      ActiveRadarrBlock::DeleteIndexerPrompt,
      ActiveRadarrBlock::Indexers,
      ActiveRadarrBlock::IndexerSettingsPrompt,
      ActiveRadarrBlock::IndexerSettingsAvailabilityDelayInput,
      ActiveRadarrBlock::IndexerSettingsConfirmPrompt,
      ActiveRadarrBlock::IndexerSettingsMaximumSizeInput,
      ActiveRadarrBlock::IndexerSettingsMinimumAgeInput,
      ActiveRadarrBlock::IndexerSettingsRetentionInput,
      ActiveRadarrBlock::IndexerSettingsRssSyncIntervalInput,
      ActiveRadarrBlock::IndexerSettingsToggleAllowHardcodedSubs,
      ActiveRadarrBlock::IndexerSettingsTogglePreferIndexerFlags,
      ActiveRadarrBlock::IndexerSettingsWhitelistedSubtitleTagsInput
    )]
    active_radarr_block: ActiveRadarrBlock,
  ) {
    test_handler_delegation!(
      RadarrHandler,
      ActiveRadarrBlock::Indexers,
      active_radarr_block
    );
  }

  #[rstest]
  fn test_delegates_downloads_blocks_to_downloads_handler(
    #[values(
      ActiveRadarrBlock::Downloads,
      ActiveRadarrBlock::DeleteDownloadPrompt,
      ActiveRadarrBlock::UpdateDownloadsPrompt
    )]
    active_radarr_block: ActiveRadarrBlock,
  ) {
    test_handler_delegation!(
      RadarrHandler,
      ActiveRadarrBlock::Downloads,
      active_radarr_block
    );
  }

  #[rstest]
  fn test_delegates_root_folders_blocks_to_root_folders_handler(
    #[values(
      ActiveRadarrBlock::RootFolders,
      ActiveRadarrBlock::AddRootFolderPrompt,
      ActiveRadarrBlock::DeleteRootFolderPrompt
    )]
    active_radarr_block: ActiveRadarrBlock,
  ) {
    test_handler_delegation!(
      RadarrHandler,
      ActiveRadarrBlock::RootFolders,
      active_radarr_block
    );
  }

  #[test]
  fn test_radarr_handler_accepts() {
    ActiveRadarrBlock::iter().for_each(|active_radarr_block| {
      assert!(RadarrHandler::accepts(&active_radarr_block));
    })
  }
}
