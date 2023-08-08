#[cfg(test)]
mod tests {
  use pretty_assertions::{assert_eq, assert_str_eq};
  use rstest::rstest;

  use crate::app::key_binding::DEFAULT_KEYBINDINGS;
  use crate::app::radarr::ActiveRadarrBlock;
  use crate::app::App;
  use crate::event::Key;
  use crate::handlers::radarr_handlers::RadarrHandler;
  use crate::handlers::KeyEventHandler;
  use crate::models::radarr_models::{Collection, Movie};
  use crate::models::HorizontallyScrollableText;
  use crate::{extended_stateful_iterable_vec, test_handler_delegation};

  mod test_handle_scroll_up_and_down {
    use rstest::rstest;

    use crate::models::radarr_models::{DownloadRecord, Indexer, RootFolder};
    use crate::{simple_stateful_iterable_vec, test_iterable_scroll};

    use super::*;

    test_iterable_scroll!(
      test_collections_scroll,
      RadarrHandler,
      collections,
      simple_stateful_iterable_vec!(Collection, HorizontallyScrollableText),
      ActiveRadarrBlock::Collections,
      None,
      title,
      to_string
    );

    test_iterable_scroll!(
      test_filtered_collections_scroll,
      RadarrHandler,
      filtered_collections,
      simple_stateful_iterable_vec!(Collection, HorizontallyScrollableText),
      ActiveRadarrBlock::Collections,
      None,
      title,
      to_string
    );

    test_iterable_scroll!(
      test_movies_scroll,
      RadarrHandler,
      movies,
      simple_stateful_iterable_vec!(Movie, HorizontallyScrollableText),
      ActiveRadarrBlock::Movies,
      None,
      title,
      to_string
    );

    test_iterable_scroll!(
      test_filtered_movies_scroll,
      RadarrHandler,
      filtered_movies,
      simple_stateful_iterable_vec!(Movie, HorizontallyScrollableText),
      ActiveRadarrBlock::Movies,
      None,
      title,
      to_string
    );

    test_iterable_scroll!(
      test_downloads_scroll,
      RadarrHandler,
      downloads,
      DownloadRecord,
      ActiveRadarrBlock::Downloads,
      None,
      title
    );

    test_iterable_scroll!(
      test_indexers_scroll,
      RadarrHandler,
      indexers,
      simple_stateful_iterable_vec!(Indexer, String, protocol),
      ActiveRadarrBlock::Indexers,
      None,
      protocol
    );

    test_iterable_scroll!(
      test_root_folders_scroll,
      RadarrHandler,
      root_folders,
      simple_stateful_iterable_vec!(RootFolder, String, path),
      ActiveRadarrBlock::RootFolders,
      None,
      path
    );
  }

  mod test_handle_home_end {
    use pretty_assertions::assert_eq;

    use crate::models::radarr_models::{DownloadRecord, Indexer, RootFolder};
    use crate::{
      extended_stateful_iterable_vec, test_iterable_home_and_end, test_text_box_home_end_keys,
    };

    use super::*;

    test_iterable_home_and_end!(
      test_collections_home_end,
      RadarrHandler,
      collections,
      extended_stateful_iterable_vec!(Collection, HorizontallyScrollableText),
      ActiveRadarrBlock::Collections,
      None,
      title,
      to_string
    );

    test_iterable_home_and_end!(
      test_filtered_collections_home_end,
      RadarrHandler,
      filtered_collections,
      extended_stateful_iterable_vec!(Collection, HorizontallyScrollableText),
      ActiveRadarrBlock::Collections,
      None,
      title,
      to_string
    );

    test_iterable_home_and_end!(
      test_movies_home_end,
      RadarrHandler,
      movies,
      extended_stateful_iterable_vec!(Movie, HorizontallyScrollableText),
      ActiveRadarrBlock::Movies,
      None,
      title,
      to_string
    );

    test_iterable_home_and_end!(
      test_filtered_movies_home_end,
      RadarrHandler,
      filtered_movies,
      extended_stateful_iterable_vec!(Movie, HorizontallyScrollableText),
      ActiveRadarrBlock::Movies,
      None,
      title,
      to_string
    );

    test_iterable_home_and_end!(
      test_downloads_home_end,
      RadarrHandler,
      downloads,
      DownloadRecord,
      ActiveRadarrBlock::Downloads,
      None,
      title
    );

    test_iterable_home_and_end!(
      test_indexers_home_end,
      RadarrHandler,
      indexers,
      extended_stateful_iterable_vec!(Indexer, String, protocol),
      ActiveRadarrBlock::Indexers,
      None,
      protocol
    );

    test_iterable_home_and_end!(
      test_root_folders_home_end,
      RadarrHandler,
      root_folders,
      extended_stateful_iterable_vec!(RootFolder, String, path),
      ActiveRadarrBlock::RootFolders,
      None,
      path
    );

    #[test]
    fn test_add_root_folder_prompt_home_end_keys() {
      test_text_box_home_end_keys!(
        RadarrHandler,
        ActiveRadarrBlock::AddRootFolderPrompt,
        edit_path
      );
    }

    #[rstest]
    fn test_search_boxes_home_end_keys(
      #[values(ActiveRadarrBlock::SearchMovie, ActiveRadarrBlock::SearchCollection)]
      active_radarr_block: ActiveRadarrBlock,
    ) {
      test_text_box_home_end_keys!(RadarrHandler, active_radarr_block, search);
    }

    #[rstest]
    fn test_filter_boxes_home_end_keys(
      #[values(ActiveRadarrBlock::FilterMovies, ActiveRadarrBlock::FilterCollections)]
      active_radarr_block: ActiveRadarrBlock,
    ) {
      test_text_box_home_end_keys!(RadarrHandler, active_radarr_block, filter);
    }
  }

  mod test_handle_delete {
    use pretty_assertions::assert_eq;

    use crate::assert_delete_prompt;

    use super::*;

    const DELETE_KEY: Key = DEFAULT_KEYBINDINGS.delete.key;

    #[test]
    fn test_movies_delete() {
      let mut app = App::default();

      assert_delete_prompt!(
        app,
        ActiveRadarrBlock::Movies,
        ActiveRadarrBlock::DeleteMoviePrompt
      );
      assert_eq!(
        app.data.radarr_data.selected_block.get_active_block(),
        &ActiveRadarrBlock::DeleteMovieToggleDeleteFile
      );
    }

    #[rstest]
    #[case(ActiveRadarrBlock::Downloads, ActiveRadarrBlock::DeleteDownloadPrompt)]
    #[case(
      ActiveRadarrBlock::RootFolders,
      ActiveRadarrBlock::DeleteRootFolderPrompt
    )]
    #[case(ActiveRadarrBlock::Indexers, ActiveRadarrBlock::DeleteIndexerPrompt)]
    fn test_delete_prompt(
      #[case] active_radarr_block: ActiveRadarrBlock,
      #[case] expected_radarr_block: ActiveRadarrBlock,
    ) {
      assert_delete_prompt!(active_radarr_block, expected_radarr_block);
    }
  }

  mod test_handle_left_right_action {
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    use crate::test_text_box_left_right_keys;

    use super::*;

    #[rstest]
    #[case(ActiveRadarrBlock::Movies, 0, ActiveRadarrBlock::System)]
    #[case(ActiveRadarrBlock::System, 5, ActiveRadarrBlock::Indexers)]
    #[case(ActiveRadarrBlock::Indexers, 4, ActiveRadarrBlock::RootFolders)]
    #[case(ActiveRadarrBlock::RootFolders, 3, ActiveRadarrBlock::Collections)]
    #[case(ActiveRadarrBlock::Collections, 2, ActiveRadarrBlock::Downloads)]
    #[case(ActiveRadarrBlock::Downloads, 1, ActiveRadarrBlock::Movies)]
    fn test_radarr_tab_left(
      #[case] active_radarr_block: ActiveRadarrBlock,
      #[case] index: usize,
      #[case] expected_radarr_block: ActiveRadarrBlock,
    ) {
      let mut app = App::default();
      app.data.radarr_data.main_tabs.set_index(index);

      RadarrHandler::with(
        &DEFAULT_KEYBINDINGS.left.key,
        &mut app,
        &active_radarr_block,
        &None,
      )
      .handle();

      assert_eq!(
        app.data.radarr_data.main_tabs.get_active_route(),
        &expected_radarr_block.into()
      );
      assert_eq!(app.get_current_route(), &expected_radarr_block.into());
    }

    #[rstest]
    #[case(ActiveRadarrBlock::Movies, 0, ActiveRadarrBlock::Downloads)]
    #[case(ActiveRadarrBlock::Downloads, 1, ActiveRadarrBlock::Collections)]
    #[case(ActiveRadarrBlock::Collections, 2, ActiveRadarrBlock::RootFolders)]
    #[case(ActiveRadarrBlock::RootFolders, 3, ActiveRadarrBlock::Indexers)]
    #[case(ActiveRadarrBlock::Indexers, 4, ActiveRadarrBlock::System)]
    #[case(ActiveRadarrBlock::System, 5, ActiveRadarrBlock::Movies)]
    fn test_radarr_tab_right(
      #[case] active_radarr_block: ActiveRadarrBlock,
      #[case] index: usize,
      #[case] expected_radarr_block: ActiveRadarrBlock,
    ) {
      let mut app = App::default();
      app.data.radarr_data.main_tabs.set_index(index);

      RadarrHandler::with(
        &DEFAULT_KEYBINDINGS.right.key,
        &mut app,
        &active_radarr_block,
        &None,
      )
      .handle();

      assert_eq!(
        app.data.radarr_data.main_tabs.get_active_route(),
        &expected_radarr_block.into()
      );
      assert_eq!(app.get_current_route(), &expected_radarr_block.into());
    }

    #[rstest]
    fn test_left_right_prompt_toggle(
      #[values(
        ActiveRadarrBlock::DeleteDownloadPrompt,
        ActiveRadarrBlock::DeleteIndexerPrompt,
        ActiveRadarrBlock::DeleteRootFolderPrompt,
        ActiveRadarrBlock::UpdateAllMoviesPrompt,
        ActiveRadarrBlock::UpdateAllCollectionsPrompt,
        ActiveRadarrBlock::UpdateDownloadsPrompt
      )]
      active_radarr_block: ActiveRadarrBlock,
      #[values(DEFAULT_KEYBINDINGS.left.key, DEFAULT_KEYBINDINGS.right.key)] key: Key,
    ) {
      let mut app = App::default();

      RadarrHandler::with(&key, &mut app, &active_radarr_block, &None).handle();

      assert!(app.data.radarr_data.prompt_confirm);

      RadarrHandler::with(&key, &mut app, &active_radarr_block, &None).handle();

      assert!(!app.data.radarr_data.prompt_confirm);
    }

    #[test]
    fn test_add_root_folder_prompt_left_right_keys() {
      test_text_box_left_right_keys!(
        RadarrHandler,
        ActiveRadarrBlock::AddRootFolderPrompt,
        edit_path
      );
    }

    #[rstest]
    fn test_search_boxes_left_right_keys(
      #[values(ActiveRadarrBlock::SearchMovie, ActiveRadarrBlock::SearchCollection)]
      active_radarr_block: ActiveRadarrBlock,
    ) {
      test_text_box_left_right_keys!(RadarrHandler, active_radarr_block, search);
    }

    #[rstest]
    fn test_filter_boxes_left_right_keys(
      #[values(ActiveRadarrBlock::FilterMovies, ActiveRadarrBlock::FilterCollections)]
      active_radarr_block: ActiveRadarrBlock,
    ) {
      test_text_box_left_right_keys!(RadarrHandler, active_radarr_block, filter);
    }
  }

  mod test_handle_submit {
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    use crate::network::radarr_network::RadarrEvent;

    use super::*;

    const SUBMIT_KEY: Key = DEFAULT_KEYBINDINGS.submit.key;

    #[test]
    fn test_indexer_submit_aka_edit() {
      let mut app = App::default();

      RadarrHandler::with(&SUBMIT_KEY, &mut app, &ActiveRadarrBlock::Indexers, &None).handle();

      assert_eq!(
        app.get_current_route(),
        &ActiveRadarrBlock::EditIndexer.into()
      );
    }

    #[rstest]
    #[case(ActiveRadarrBlock::Movies, ActiveRadarrBlock::MovieDetails)]
    #[case(ActiveRadarrBlock::Collections, ActiveRadarrBlock::CollectionDetails)]
    fn test_movies_collections_details_submit(
      #[case] active_radarr_block: ActiveRadarrBlock,
      #[case] expected_radarr_block: ActiveRadarrBlock,
    ) {
      let mut app = App::default();

      RadarrHandler::with(&SUBMIT_KEY, &mut app, &active_radarr_block, &None).handle();

      assert_eq!(app.get_current_route(), &expected_radarr_block.into());
    }

    #[test]
    fn test_search_movie_submit() {
      let mut app = App::default();
      app
        .data
        .radarr_data
        .movies
        .set_items(extended_stateful_iterable_vec!(
          Movie,
          HorizontallyScrollableText
        ));
      app.data.radarr_data.search = "Test 2".to_owned().into();

      RadarrHandler::with(
        &SUBMIT_KEY,
        &mut app,
        &ActiveRadarrBlock::SearchMovie,
        &None,
      )
      .handle();

      assert_str_eq!(
        app.data.radarr_data.movies.current_selection().title.text,
        "Test 2"
      );
    }

    #[test]
    fn test_search_filtered_movies_submit() {
      let mut app = App::default();
      app
        .data
        .radarr_data
        .filtered_movies
        .set_items(extended_stateful_iterable_vec!(
          Movie,
          HorizontallyScrollableText
        ));
      app.data.radarr_data.search = "Test 2".to_owned().into();

      RadarrHandler::with(
        &SUBMIT_KEY,
        &mut app,
        &ActiveRadarrBlock::SearchMovie,
        &None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .radarr_data
          .filtered_movies
          .current_selection()
          .title
          .text,
        "Test 2"
      );
    }

    #[test]
    fn test_search_collections_submit() {
      let mut app = App::default();
      app
        .data
        .radarr_data
        .collections
        .set_items(extended_stateful_iterable_vec!(
          Collection,
          HorizontallyScrollableText
        ));
      app.data.radarr_data.search = "Test 2".to_owned().into();

      RadarrHandler::with(
        &SUBMIT_KEY,
        &mut app,
        &ActiveRadarrBlock::SearchCollection,
        &None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .radarr_data
          .collections
          .current_selection()
          .title
          .text,
        "Test 2"
      );
    }

    #[test]
    fn test_search_filtered_collections_submit() {
      let mut app = App::default();
      app
        .data
        .radarr_data
        .filtered_collections
        .set_items(extended_stateful_iterable_vec!(
          Collection,
          HorizontallyScrollableText
        ));
      app.data.radarr_data.search = "Test 2".to_owned().into();

      RadarrHandler::with(
        &SUBMIT_KEY,
        &mut app,
        &ActiveRadarrBlock::SearchCollection,
        &None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .radarr_data
          .filtered_collections
          .current_selection()
          .title
          .text,
        "Test 2"
      );
    }

    #[test]
    fn test_filter_movies_submit() {
      let mut app = App::default();
      app
        .data
        .radarr_data
        .movies
        .set_items(extended_stateful_iterable_vec!(
          Movie,
          HorizontallyScrollableText
        ));
      app.data.radarr_data.filter = "Test".to_owned().into();

      RadarrHandler::with(
        &SUBMIT_KEY,
        &mut app,
        &ActiveRadarrBlock::FilterMovies,
        &None,
      )
      .handle();

      assert_eq!(app.data.radarr_data.filtered_movies.items.len(), 3);
      assert_str_eq!(
        app
          .data
          .radarr_data
          .filtered_movies
          .current_selection()
          .title
          .text,
        "Test 1"
      );
    }

    #[test]
    fn test_filter_collections_submit() {
      let mut app = App::default();
      app
        .data
        .radarr_data
        .collections
        .set_items(extended_stateful_iterable_vec!(
          Collection,
          HorizontallyScrollableText
        ));
      app.data.radarr_data.filter = "Test".to_owned().into();

      RadarrHandler::with(
        &SUBMIT_KEY,
        &mut app,
        &ActiveRadarrBlock::FilterCollections,
        &None,
      )
      .handle();

      assert_eq!(app.data.radarr_data.filtered_collections.items.len(), 3);
      assert_str_eq!(
        app
          .data
          .radarr_data
          .filtered_collections
          .current_selection()
          .title
          .text,
        "Test 1"
      );
    }

    #[test]
    fn test_add_root_folder_prompt_confirm_submit() {
      let mut app = App::default();
      app.data.radarr_data.prompt_confirm = true;
      app.should_ignore_quit_key = true;
      app.push_navigation_stack(ActiveRadarrBlock::RootFolders.into());
      app.push_navigation_stack(ActiveRadarrBlock::AddRootFolderPrompt.into());

      RadarrHandler::with(
        &SUBMIT_KEY,
        &mut app,
        &ActiveRadarrBlock::AddRootFolderPrompt,
        &None,
      )
      .handle();

      assert!(app.data.radarr_data.prompt_confirm);
      assert!(!app.should_ignore_quit_key);
      assert_eq!(
        app.data.radarr_data.prompt_confirm_action,
        Some(RadarrEvent::AddRootFolder)
      );
      assert_eq!(
        app.get_current_route(),
        &ActiveRadarrBlock::RootFolders.into()
      );
    }

    #[rstest]
    #[case(
      ActiveRadarrBlock::Downloads,
      ActiveRadarrBlock::DeleteDownloadPrompt,
      RadarrEvent::DeleteDownload
    )]
    #[case(
      ActiveRadarrBlock::Indexers,
      ActiveRadarrBlock::DeleteIndexerPrompt,
      RadarrEvent::DeleteIndexer
    )]
    #[case(
      ActiveRadarrBlock::RootFolders,
      ActiveRadarrBlock::DeleteRootFolderPrompt,
      RadarrEvent::DeleteRootFolder
    )]
    #[case(
      ActiveRadarrBlock::Movies,
      ActiveRadarrBlock::UpdateAllMoviesPrompt,
      RadarrEvent::UpdateAllMovies
    )]
    #[case(
      ActiveRadarrBlock::Downloads,
      ActiveRadarrBlock::UpdateDownloadsPrompt,
      RadarrEvent::UpdateDownloads
    )]
    #[case(
      ActiveRadarrBlock::Collections,
      ActiveRadarrBlock::UpdateAllCollectionsPrompt,
      RadarrEvent::UpdateCollections
    )]
    fn test_prompt_confirm_submit(
      #[case] base_route: ActiveRadarrBlock,
      #[case] prompt_block: ActiveRadarrBlock,
      #[case] expected_action: RadarrEvent,
    ) {
      let mut app = App::default();
      app.data.radarr_data.prompt_confirm = true;
      app.push_navigation_stack(base_route.into());
      app.push_navigation_stack(prompt_block.into());

      RadarrHandler::with(&SUBMIT_KEY, &mut app, &prompt_block, &None).handle();

      assert!(app.data.radarr_data.prompt_confirm);
      assert_eq!(
        app.data.radarr_data.prompt_confirm_action,
        Some(expected_action)
      );
      assert_eq!(app.get_current_route(), &base_route.into());
    }

    #[rstest]
    #[case(ActiveRadarrBlock::Downloads, ActiveRadarrBlock::DeleteDownloadPrompt)]
    #[case(ActiveRadarrBlock::Movies, ActiveRadarrBlock::UpdateAllMoviesPrompt)]
    #[case(ActiveRadarrBlock::Downloads, ActiveRadarrBlock::UpdateDownloadsPrompt)]
    #[case(
      ActiveRadarrBlock::Collections,
      ActiveRadarrBlock::UpdateAllCollectionsPrompt
    )]
    fn test_prompt_decline_submit(
      #[case] base_route: ActiveRadarrBlock,
      #[case] prompt_block: ActiveRadarrBlock,
    ) {
      let mut app = App::default();
      app.push_navigation_stack(base_route.into());
      app.push_navigation_stack(prompt_block.into());

      RadarrHandler::with(&SUBMIT_KEY, &mut app, &prompt_block, &None).handle();

      assert!(!app.data.radarr_data.prompt_confirm);
      assert_eq!(app.data.radarr_data.prompt_confirm_action, None);
      assert_eq!(app.get_current_route(), &base_route.into());
    }
  }

  mod test_handle_esc {
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    use crate::app::radarr::radarr_test_utils::utils::create_test_radarr_data;
    use crate::{assert_filter_reset, assert_search_reset};

    use super::*;

    const ESC_KEY: Key = DEFAULT_KEYBINDINGS.esc.key;

    #[rstest]
    #[case(ActiveRadarrBlock::Movies, ActiveRadarrBlock::SearchMovie)]
    #[case(ActiveRadarrBlock::Collections, ActiveRadarrBlock::SearchCollection)]
    fn test_search_blocks_esc(
      #[case] base_block: ActiveRadarrBlock,
      #[case] search_block: ActiveRadarrBlock,
    ) {
      let mut app = App::default();
      app.should_ignore_quit_key = true;
      app.push_navigation_stack(base_block.into());
      app.push_navigation_stack(search_block.into());
      app.data.radarr_data = create_test_radarr_data();

      RadarrHandler::with(&ESC_KEY, &mut app, &search_block, &None).handle();

      assert_eq!(app.get_current_route(), &base_block.into());
      assert!(!app.should_ignore_quit_key);
      assert_search_reset!(app.data.radarr_data);
    }

    #[rstest]
    #[case(ActiveRadarrBlock::Movies, ActiveRadarrBlock::FilterMovies)]
    #[case(ActiveRadarrBlock::Collections, ActiveRadarrBlock::FilterCollections)]
    fn test_filter_blocks_esc(
      #[case] base_block: ActiveRadarrBlock,
      #[case] filter_block: ActiveRadarrBlock,
    ) {
      let mut app = App::default();
      app.should_ignore_quit_key = true;
      app.push_navigation_stack(base_block.into());
      app.push_navigation_stack(filter_block.into());
      app.data.radarr_data = create_test_radarr_data();

      RadarrHandler::with(&ESC_KEY, &mut app, &filter_block, &None).handle();

      assert_eq!(app.get_current_route(), &base_block.into());
      assert!(!app.should_ignore_quit_key);
      assert_filter_reset!(app.data.radarr_data);
    }

    #[rstest]
    #[case(ActiveRadarrBlock::Movies, ActiveRadarrBlock::UpdateAllMoviesPrompt)]
    #[case(
      ActiveRadarrBlock::RootFolders,
      ActiveRadarrBlock::DeleteRootFolderPrompt
    )]
    #[case(ActiveRadarrBlock::Downloads, ActiveRadarrBlock::DeleteDownloadPrompt)]
    #[case(ActiveRadarrBlock::Indexers, ActiveRadarrBlock::DeleteIndexerPrompt)]
    #[case(ActiveRadarrBlock::Downloads, ActiveRadarrBlock::UpdateDownloadsPrompt)]
    #[case(
      ActiveRadarrBlock::Collections,
      ActiveRadarrBlock::UpdateAllCollectionsPrompt
    )]
    fn test_prompt_blocks_esc(
      #[case] base_block: ActiveRadarrBlock,
      #[case] prompt_block: ActiveRadarrBlock,
    ) {
      let mut app = App::default();
      app.push_navigation_stack(base_block.into());
      app.push_navigation_stack(prompt_block.into());
      app.data.radarr_data.prompt_confirm = true;

      RadarrHandler::with(&ESC_KEY, &mut app, &prompt_block, &None).handle();

      assert_eq!(app.get_current_route(), &base_block.into());
      assert!(!app.data.radarr_data.prompt_confirm);
    }

    #[test]
    fn test_add_root_folder_prompt_esc() {
      let mut app = App::default();
      app.push_navigation_stack(ActiveRadarrBlock::RootFolders.into());
      app.push_navigation_stack(ActiveRadarrBlock::AddRootFolderPrompt.into());
      app.data.radarr_data.edit_path = HorizontallyScrollableText::from("/nfs/test");
      app.should_ignore_quit_key = true;

      RadarrHandler::with(
        &ESC_KEY,
        &mut app,
        &ActiveRadarrBlock::AddRootFolderPrompt,
        &None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        &ActiveRadarrBlock::RootFolders.into()
      );

      assert!(app.data.radarr_data.edit_path.text.is_empty());
      assert!(!app.data.radarr_data.prompt_confirm);
      assert!(!app.should_ignore_quit_key);
    }

    #[test]
    fn test_default_esc() {
      let mut app = App::default();
      app.error = "test error".to_owned().into();
      app.push_navigation_stack(ActiveRadarrBlock::Downloads.into());
      app.push_navigation_stack(ActiveRadarrBlock::Downloads.into());
      app.data.radarr_data = create_test_radarr_data();

      RadarrHandler::with(&ESC_KEY, &mut app, &ActiveRadarrBlock::Downloads, &None).handle();

      assert_eq!(
        app.get_current_route(),
        &ActiveRadarrBlock::Downloads.into()
      );
      assert!(app.error.text.is_empty());
      assert_search_reset!(app.data.radarr_data);
      assert_filter_reset!(app.data.radarr_data);
    }
  }

  mod test_handle_key_char {
    use bimap::BiMap;
    use pretty_assertions::{assert_eq, assert_str_eq};
    use rstest::rstest;
    use serde_json::Number;
    use strum::IntoEnumIterator;

    use crate::app::radarr::radarr_test_utils::utils::create_test_radarr_data;
    use crate::app::radarr::RadarrData;
    use crate::app::radarr::EDIT_COLLECTION_SELECTION_BLOCKS;
    use crate::app::radarr::EDIT_MOVIE_SELECTION_BLOCKS;
    use crate::models::radarr_models::MinimumAvailability;
    use crate::models::BlockSelectionState;
    use crate::models::HorizontallyScrollableText;
    use crate::models::StatefulTable;
    use crate::{test_edit_collection_key, test_edit_movie_key};

    use super::*;

    #[rstest]
    #[case(ActiveRadarrBlock::Movies, ActiveRadarrBlock::SearchMovie)]
    #[case(ActiveRadarrBlock::Collections, ActiveRadarrBlock::SearchCollection)]
    fn test_search_key(
      #[case] active_radarr_block: ActiveRadarrBlock,
      #[case] expected_radarr_block: ActiveRadarrBlock,
    ) {
      let mut app = App::default();

      RadarrHandler::with(
        &DEFAULT_KEYBINDINGS.search.key,
        &mut app,
        &active_radarr_block,
        &None,
      )
      .handle();

      assert_eq!(app.get_current_route(), &expected_radarr_block.into());
      assert!(app.data.radarr_data.is_searching);
      assert!(app.should_ignore_quit_key);
    }

    #[rstest]
    #[case(ActiveRadarrBlock::Movies, ActiveRadarrBlock::FilterMovies)]
    #[case(ActiveRadarrBlock::Collections, ActiveRadarrBlock::FilterCollections)]
    fn test_filter_key(
      #[case] active_radarr_block: ActiveRadarrBlock,
      #[case] expected_radarr_block: ActiveRadarrBlock,
    ) {
      let mut app = App::default();

      RadarrHandler::with(
        &DEFAULT_KEYBINDINGS.filter.key,
        &mut app,
        &active_radarr_block,
        &None,
      )
      .handle();

      assert_eq!(app.get_current_route(), &expected_radarr_block.into());
      assert!(app.data.radarr_data.is_filtering);
      assert!(app.should_ignore_quit_key);
    }

    #[test]
    fn test_movie_add() {
      let mut app = App::default();

      RadarrHandler::with(
        &DEFAULT_KEYBINDINGS.add.key,
        &mut app,
        &ActiveRadarrBlock::Movies,
        &None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        &ActiveRadarrBlock::AddMovieSearchInput.into()
      );
      assert!(app.should_ignore_quit_key);
    }

    #[test]
    fn test_indexer_add() {
      let mut app = App::default();

      RadarrHandler::with(
        &DEFAULT_KEYBINDINGS.add.key,
        &mut app,
        &ActiveRadarrBlock::Indexers,
        &None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        &ActiveRadarrBlock::AddIndexer.into()
      );
    }

    #[test]
    fn test_root_folder_add() {
      let mut app = App::default();

      RadarrHandler::with(
        &DEFAULT_KEYBINDINGS.add.key,
        &mut app,
        &ActiveRadarrBlock::RootFolders,
        &None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        &ActiveRadarrBlock::AddRootFolderPrompt.into()
      );
      assert!(app.should_ignore_quit_key);
    }

    #[test]
    fn test_movie_edit_key() {
      test_edit_movie_key!(
        RadarrHandler,
        ActiveRadarrBlock::Movies,
        ActiveRadarrBlock::Movies
      );
    }

    #[test]
    fn test_collection_edit_key() {
      test_edit_collection_key!(
        RadarrHandler,
        ActiveRadarrBlock::Collections,
        ActiveRadarrBlock::Collections
      );
    }

    #[rstest]
    #[case(ActiveRadarrBlock::Movies, ActiveRadarrBlock::UpdateAllMoviesPrompt)]
    #[case(ActiveRadarrBlock::Downloads, ActiveRadarrBlock::UpdateDownloadsPrompt)]
    #[case(
      ActiveRadarrBlock::Collections,
      ActiveRadarrBlock::UpdateAllCollectionsPrompt
    )]
    #[case(ActiveRadarrBlock::System, ActiveRadarrBlock::SystemUpdates)]
    fn test_update_key(
      #[case] active_radarr_block: ActiveRadarrBlock,
      #[case] expected_radarr_block: ActiveRadarrBlock,
    ) {
      let mut app = App::default();

      RadarrHandler::with(
        &DEFAULT_KEYBINDINGS.update.key,
        &mut app,
        &active_radarr_block,
        &None,
      )
      .handle();

      assert_eq!(app.get_current_route(), &expected_radarr_block.into());
    }

    #[test]
    fn test_queued_events_key() {
      let mut app = App::default();

      RadarrHandler::with(
        &DEFAULT_KEYBINDINGS.events.key,
        &mut app,
        &ActiveRadarrBlock::System,
        &None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        &ActiveRadarrBlock::SystemQueuedEvents.into()
      );
    }

    #[rstest]
    fn test_refresh_key(
      #[values(
        ActiveRadarrBlock::Movies,
        ActiveRadarrBlock::Collections,
        ActiveRadarrBlock::Downloads,
        ActiveRadarrBlock::Indexers,
        ActiveRadarrBlock::RootFolders,
        ActiveRadarrBlock::System
      )]
      active_radarr_block: ActiveRadarrBlock,
    ) {
      let mut app = App::default();
      app.push_navigation_stack(active_radarr_block.into());

      RadarrHandler::with(
        &DEFAULT_KEYBINDINGS.refresh.key,
        &mut app,
        &active_radarr_block,
        &None,
      )
      .handle();

      assert_eq!(app.get_current_route(), &active_radarr_block.into());
      assert!(app.should_refresh);
    }

    #[test]
    fn test_logs_key() {
      let mut app = App::default();
      app.data.radarr_data.logs.set_items(vec![
        HorizontallyScrollableText::from("test 1"),
        HorizontallyScrollableText::from("test 2"),
      ]);

      RadarrHandler::with(
        &DEFAULT_KEYBINDINGS.logs.key,
        &mut app,
        &ActiveRadarrBlock::System,
        &None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        &ActiveRadarrBlock::SystemLogs.into()
      );
      assert_eq!(
        app.data.radarr_data.log_details.items,
        app.data.radarr_data.logs.items
      );
      assert_str_eq!(
        app.data.radarr_data.log_details.current_selection().text,
        "test 2"
      );
    }

    #[test]
    fn test_tasks_key() {
      let mut app = App::default();

      RadarrHandler::with(
        &DEFAULT_KEYBINDINGS.tasks.key,
        &mut app,
        &ActiveRadarrBlock::System,
        &None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        &ActiveRadarrBlock::SystemTasks.into()
      );
    }

    #[test]
    fn test_indexer_settings_key() {
      let mut app = App::default();

      RadarrHandler::with(
        &DEFAULT_KEYBINDINGS.settings.key,
        &mut app,
        &ActiveRadarrBlock::Indexers,
        &None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        &ActiveRadarrBlock::IndexerSettings.into()
      );
    }

    #[test]
    fn test_add_root_folder_prompt_backspace_key() {
      let mut app = App::default();
      app.data.radarr_data.edit_path = "/nfs/test".to_owned().into();

      RadarrHandler::with(
        &DEFAULT_KEYBINDINGS.backspace.key,
        &mut app,
        &ActiveRadarrBlock::AddRootFolderPrompt,
        &None,
      )
      .handle();

      assert_str_eq!(app.data.radarr_data.edit_path.text, "/nfs/tes");
    }

    #[rstest]
    fn test_search_boxes_backspace_key(
      #[values(ActiveRadarrBlock::SearchMovie, ActiveRadarrBlock::SearchCollection)]
      active_radarr_block: ActiveRadarrBlock,
    ) {
      let mut app = App::default();
      app.data.radarr_data.search = "Test".to_owned().into();

      RadarrHandler::with(
        &DEFAULT_KEYBINDINGS.backspace.key,
        &mut app,
        &active_radarr_block,
        &None,
      )
      .handle();

      assert_str_eq!(app.data.radarr_data.search.text, "Tes");
    }

    #[rstest]
    fn test_filter_boxes_backspace_key(
      #[values(ActiveRadarrBlock::FilterMovies, ActiveRadarrBlock::FilterCollections)]
      active_radarr_block: ActiveRadarrBlock,
    ) {
      let mut app = App::default();
      app.data.radarr_data.filter = "Test".to_owned().into();

      RadarrHandler::with(
        &DEFAULT_KEYBINDINGS.backspace.key,
        &mut app,
        &active_radarr_block,
        &None,
      )
      .handle();

      assert_str_eq!(app.data.radarr_data.filter.text, "Tes");
    }

    #[test]
    fn test_add_root_folder_prompt_char_key() {
      let mut app = App::default();

      RadarrHandler::with(
        &Key::Char('h'),
        &mut app,
        &ActiveRadarrBlock::AddRootFolderPrompt,
        &None,
      )
      .handle();

      assert_str_eq!(app.data.radarr_data.edit_path.text, "h");
    }

    #[rstest]
    fn test_search_boxes_char_key(
      #[values(ActiveRadarrBlock::SearchMovie, ActiveRadarrBlock::SearchCollection)]
      active_radarr_block: ActiveRadarrBlock,
    ) {
      let mut app = App::default();

      RadarrHandler::with(&Key::Char('h'), &mut app, &active_radarr_block, &None).handle();

      assert_str_eq!(app.data.radarr_data.search.text, "h");
    }

    #[rstest]
    fn test_filter_boxes_char_key(
      #[values(ActiveRadarrBlock::FilterMovies, ActiveRadarrBlock::FilterCollections)]
      active_radarr_block: ActiveRadarrBlock,
    ) {
      let mut app = App::default();

      RadarrHandler::with(&Key::Char('h'), &mut app, &active_radarr_block, &None).handle();

      assert_str_eq!(app.data.radarr_data.filter.text, "h");
    }
  }

  #[test]
  fn test_search_table() {
    let mut app = App::default();
    app
      .data
      .radarr_data
      .movies
      .set_items(extended_stateful_iterable_vec!(
        Movie,
        HorizontallyScrollableText
      ));
    app.data.radarr_data.search = "Test 2".to_owned().into();
    app.data.radarr_data.is_searching = true;
    app.should_ignore_quit_key = true;
    app.push_navigation_stack(ActiveRadarrBlock::SearchMovie.into());

    let movies = &app.data.radarr_data.movies.items.clone();

    let index = RadarrHandler::with(
      &DEFAULT_KEYBINDINGS.submit.key,
      &mut app,
      &ActiveRadarrBlock::SearchMovie,
      &None,
    )
    .search_table(movies, |movie| &movie.title.text);

    assert_eq!(index, Some(1));
    assert_eq!(app.get_current_route(), &ActiveRadarrBlock::Movies.into());
    assert!(!app.data.radarr_data.is_searching);
    assert!(!app.should_ignore_quit_key);
    assert!(app.data.radarr_data.search.text.is_empty());
  }

  #[test]
  fn test_search_table_no_search_hits() {
    let mut app = App::default();
    app
      .data
      .radarr_data
      .movies
      .set_items(extended_stateful_iterable_vec!(
        Movie,
        HorizontallyScrollableText
      ));
    app.data.radarr_data.search = "Test 5".to_owned().into();
    app.data.radarr_data.is_searching = true;
    app.should_ignore_quit_key = true;
    app.push_navigation_stack(ActiveRadarrBlock::SearchMovie.into());

    let movies = &app.data.radarr_data.movies.items.clone();

    let index = RadarrHandler::with(
      &DEFAULT_KEYBINDINGS.submit.key,
      &mut app,
      &ActiveRadarrBlock::SearchMovie,
      &None,
    )
    .search_table(movies, |movie| &movie.title.text);

    assert_eq!(index, None);
    assert_eq!(
      app.get_current_route(),
      &ActiveRadarrBlock::SearchMovie.into()
    );
    assert!(!app.data.radarr_data.is_searching);
    assert!(!app.should_ignore_quit_key);
    assert!(app.data.radarr_data.search.text.is_empty());
  }

  #[test]
  fn test_filter_table() {
    let mut app = App::default();
    app
      .data
      .radarr_data
      .movies
      .set_items(extended_stateful_iterable_vec!(
        Movie,
        HorizontallyScrollableText
      ));
    app.data.radarr_data.filter = "Test 2".to_owned().into();
    app.data.radarr_data.is_searching = true;
    app.should_ignore_quit_key = true;
    app.push_navigation_stack(ActiveRadarrBlock::FilterMovies.into());

    let movies = &app.data.radarr_data.movies.items.clone();

    let filter_matches = RadarrHandler::with(
      &DEFAULT_KEYBINDINGS.submit.key,
      &mut app,
      &ActiveRadarrBlock::FilterMovies,
      &None,
    )
    .filter_table(movies, |movie| &movie.title.text);

    assert_eq!(filter_matches.len(), 1);
    assert_str_eq!(filter_matches[0].title.text, "Test 2");
    assert_eq!(app.get_current_route(), &ActiveRadarrBlock::Movies.into());
    assert!(!app.data.radarr_data.is_filtering);
    assert!(!app.should_ignore_quit_key);
    assert!(app.data.radarr_data.filter.text.is_empty());
  }

  #[test]
  fn test_filter_table_no_filter_matches() {
    let mut app = App::default();
    app
      .data
      .radarr_data
      .movies
      .set_items(extended_stateful_iterable_vec!(
        Movie,
        HorizontallyScrollableText
      ));
    app.data.radarr_data.filter = "Test 5".to_owned().into();
    app.data.radarr_data.is_filtering = true;
    app.should_ignore_quit_key = true;
    app.push_navigation_stack(ActiveRadarrBlock::FilterMovies.into());

    let movies = &app.data.radarr_data.movies.items.clone();

    let filter_matches = RadarrHandler::with(
      &DEFAULT_KEYBINDINGS.submit.key,
      &mut app,
      &ActiveRadarrBlock::FilterMovies,
      &None,
    )
    .filter_table(movies, |movie| &movie.title.text);

    assert!(filter_matches.is_empty());
    assert_eq!(
      app.get_current_route(),
      &ActiveRadarrBlock::FilterMovies.into()
    );
    assert!(!app.data.radarr_data.is_searching);
    assert!(!app.should_ignore_quit_key);
    assert!(app.data.radarr_data.filter.text.is_empty());
  }

  #[rstest]
  fn test_delegates_system_details_blocks_to_system_details_handler(
    #[values(
      ActiveRadarrBlock::System,
      ActiveRadarrBlock::SystemLogs,
      ActiveRadarrBlock::SystemTasks,
      ActiveRadarrBlock::SystemQueuedEvents,
      ActiveRadarrBlock::SystemUpdates
    )]
    active_radarr_block: ActiveRadarrBlock,
  ) {
    test_handler_delegation!(ActiveRadarrBlock::System, active_radarr_block);
  }

  #[rstest]
  fn test_delegates_add_movie_blocks_to_add_movie_handler(
    #[values(
      ActiveRadarrBlock::AddMovieSearchInput,
      ActiveRadarrBlock::AddMovieSearchResults,
      ActiveRadarrBlock::AddMoviePrompt,
      ActiveRadarrBlock::AddMovieSelectMonitor,
      ActiveRadarrBlock::AddMovieSelectMinimumAvailability,
      ActiveRadarrBlock::AddMovieSelectQualityProfile,
      ActiveRadarrBlock::AddMovieSelectRootFolder,
      ActiveRadarrBlock::AddMovieAlreadyInLibrary,
      ActiveRadarrBlock::AddMovieTagsInput
    )]
    active_radarr_block: ActiveRadarrBlock,
  ) {
    test_handler_delegation!(ActiveRadarrBlock::Movies, active_radarr_block);
  }

  #[rstest]
  fn test_delegate_collection_details_blocks_to_collection_details_handler(
    #[values(
      ActiveRadarrBlock::CollectionDetails,
      ActiveRadarrBlock::ViewMovieOverview
    )]
    active_radarr_block: ActiveRadarrBlock,
  ) {
    test_handler_delegation!(ActiveRadarrBlock::Collections, active_radarr_block);
  }

  #[rstest]
  fn test_delegate_movie_details_blocks_to_movie_details_handler(
    #[values(
      ActiveRadarrBlock::MovieDetails,
      ActiveRadarrBlock::MovieHistory,
      ActiveRadarrBlock::FileInfo,
      ActiveRadarrBlock::Cast,
      ActiveRadarrBlock::Crew,
      ActiveRadarrBlock::AutomaticallySearchMoviePrompt,
      ActiveRadarrBlock::UpdateAndScanPrompt,
      ActiveRadarrBlock::ManualSearch,
      ActiveRadarrBlock::ManualSearchConfirmPrompt
    )]
    active_radarr_block: ActiveRadarrBlock,
  ) {
    test_handler_delegation!(ActiveRadarrBlock::Movies, active_radarr_block);
  }

  #[rstest]
  fn test_delegate_edit_movie_blocks_to_edit_movie_handler(
    #[values(
      ActiveRadarrBlock::EditMoviePrompt,
      ActiveRadarrBlock::EditMoviePathInput,
      ActiveRadarrBlock::EditMovieSelectMinimumAvailability,
      ActiveRadarrBlock::EditMovieSelectQualityProfile,
      ActiveRadarrBlock::EditMovieTagsInput
    )]
    active_radarr_block: ActiveRadarrBlock,
  ) {
    test_handler_delegation!(ActiveRadarrBlock::Movies, active_radarr_block);
  }

  #[test]
  fn test_delegate_delete_movie_blocks_to_delete_movie_handler() {
    test_handler_delegation!(
      ActiveRadarrBlock::Movies,
      ActiveRadarrBlock::DeleteMoviePrompt
    );
  }

  #[rstest]
  fn test_delegate_edit_collection_blocks_to_edit_collection_handler(
    #[values(
      ActiveRadarrBlock::EditCollectionPrompt,
      ActiveRadarrBlock::EditCollectionRootFolderPathInput,
      ActiveRadarrBlock::EditCollectionSelectMinimumAvailability,
      ActiveRadarrBlock::EditCollectionSelectQualityProfile
    )]
    active_radarr_block: ActiveRadarrBlock,
  ) {
    test_handler_delegation!(ActiveRadarrBlock::Collections, active_radarr_block);
  }
}
