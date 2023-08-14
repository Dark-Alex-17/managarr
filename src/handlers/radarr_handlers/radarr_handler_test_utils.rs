#[cfg(test)]
#[macro_use]
mod utils {
  #[macro_export]
  macro_rules! test_edit_movie_key {
    ($handler:ident, $block:expr, $context:expr) => {
      let mut app = App::default();
      let mut radarr_data = RadarrData {
        quality_profile_map: BiMap::from_iter([
          (2222, "HD - 1080p".to_owned()),
          (1111, "Any".to_owned()),
        ]),
        tags_map: BiMap::from_iter([(1, "test".to_owned())]),
        filtered_movies: None,
        ..create_test_radarr_data()
      };
      radarr_data.movies.set_items(vec![Movie {
        path: "/nfs/movies/Test".to_owned().into(),
        monitored: true,
        quality_profile_id: Number::from(2222),
        minimum_availability: MinimumAvailability::Released,
        tags: vec![Number::from(1)],
        ..Movie::default()
      }]);
      app.data.radarr_data = radarr_data;

      $handler::with(&DEFAULT_KEYBINDINGS.edit.key, &mut app, &$block, &None).handle();

      assert_eq!(
        app.get_current_route(),
        &(ActiveRadarrBlock::EditMoviePrompt, Some($context)).into()
      );
      assert_eq!(
        app.data.radarr_data.selected_block.get_active_block(),
        &ActiveRadarrBlock::EditMovieToggleMonitored
      );
      assert_eq!(
        app
          .data
          .radarr_data
          .edit_movie_modal
          .as_ref()
          .unwrap()
          .minimum_availability_list
          .items,
        Vec::from_iter(MinimumAvailability::iter())
      );
      assert_eq!(
        app
          .data
          .radarr_data
          .edit_movie_modal
          .as_ref()
          .unwrap()
          .minimum_availability_list
          .current_selection(),
        &MinimumAvailability::Released
      );
      assert_eq!(
        app
          .data
          .radarr_data
          .edit_movie_modal
          .as_ref()
          .unwrap()
          .quality_profile_list
          .items,
        vec!["Any".to_owned(), "HD - 1080p".to_owned()]
      );
      assert_str_eq!(
        app
          .data
          .radarr_data
          .edit_movie_modal
          .as_ref()
          .unwrap()
          .quality_profile_list
          .current_selection(),
        "HD - 1080p"
      );
      assert_str_eq!(
        app
          .data
          .radarr_data
          .edit_movie_modal
          .as_ref()
          .unwrap()
          .path
          .text,
        "/nfs/movies/Test"
      );
      assert_str_eq!(
        app
          .data
          .radarr_data
          .edit_movie_modal
          .as_ref()
          .unwrap()
          .tags
          .text,
        "test"
      );
      assert_eq!(
        app
          .data
          .radarr_data
          .edit_movie_modal
          .as_ref()
          .unwrap()
          .monitored,
        Some(true)
      );
      assert_eq!(
        app.data.radarr_data.selected_block.blocks,
        &EDIT_MOVIE_SELECTION_BLOCKS
      );
    };
  }

  #[macro_export]
  macro_rules! test_edit_collection_key {
    ($handler:ident, $block:expr, $context:expr) => {
      let mut app = App::default();
      let mut radarr_data = RadarrData {
        quality_profile_map: BiMap::from_iter([
          (2222, "HD - 1080p".to_owned()),
          (1111, "Any".to_owned()),
        ]),
        filtered_collections: None,
        ..create_test_radarr_data()
      };
      radarr_data.collections.set_items(vec![Collection {
        root_folder_path: "/nfs/movies/Test".to_owned().into(),
        monitored: true,
        search_on_add: true,
        quality_profile_id: Number::from(2222),
        minimum_availability: MinimumAvailability::Released,
        ..Collection::default()
      }]);
      app.data.radarr_data = radarr_data;

      $handler::with(&DEFAULT_KEYBINDINGS.edit.key, &mut app, &$block, &None).handle();

      assert_eq!(
        app.get_current_route(),
        &(ActiveRadarrBlock::EditCollectionPrompt, Some($context)).into()
      );
      assert_eq!(
        app.data.radarr_data.selected_block.get_active_block(),
        &ActiveRadarrBlock::EditCollectionToggleMonitored
      );
      assert_eq!(
        app
          .data
          .radarr_data
          .edit_collection_modal
          .as_ref()
          .unwrap()
          .minimum_availability_list
          .items,
        Vec::from_iter(MinimumAvailability::iter())
      );
      assert_eq!(
        app
          .data
          .radarr_data
          .edit_collection_modal
          .as_ref()
          .unwrap()
          .minimum_availability_list
          .current_selection(),
        &MinimumAvailability::Released
      );
      assert_eq!(
        app
          .data
          .radarr_data
          .edit_collection_modal
          .as_ref()
          .unwrap()
          .quality_profile_list
          .items,
        vec!["Any".to_owned(), "HD - 1080p".to_owned()]
      );
      assert_str_eq!(
        app
          .data
          .radarr_data
          .edit_collection_modal
          .as_ref()
          .unwrap()
          .quality_profile_list
          .current_selection(),
        "HD - 1080p"
      );
      assert_str_eq!(
        app
          .data
          .radarr_data
          .edit_collection_modal
          .as_ref()
          .unwrap()
          .path
          .text,
        "/nfs/movies/Test"
      );
      assert_eq!(
        app
          .data
          .radarr_data
          .edit_collection_modal
          .as_ref()
          .unwrap()
          .monitored,
        Some(true)
      );
      assert_eq!(
        app
          .data
          .radarr_data
          .edit_collection_modal
          .as_ref()
          .unwrap()
          .search_on_add,
        Some(true)
      );
      assert_eq!(
        app.data.radarr_data.selected_block.blocks,
        &EDIT_COLLECTION_SELECTION_BLOCKS
      );
    };
  }

  #[macro_export]
  macro_rules! assert_delete_prompt {
    ($block:expr, $expected_block:expr) => {
      let mut app = App::default();

      RadarrHandler::with(&DELETE_KEY, &mut app, &$block, &None).handle();

      assert_eq!(app.get_current_route(), &$expected_block.into());
    };

    ($handler:ident, $block:expr, $expected_block:expr) => {
      let mut app = App::default();

      $handler::with(&DELETE_KEY, &mut app, &$block, &None).handle();

      assert_eq!(app.get_current_route(), &$expected_block.into());
    };

    ($app:expr, $block:expr, $expected_block:expr) => {
      RadarrHandler::with(&DELETE_KEY, &mut $app, &$block, &None).handle();

      assert_eq!($app.get_current_route(), &$expected_block.into());
    };

    ($handler:ident, $app:expr, $block:expr, $expected_block:expr) => {
      $handler::with(&DELETE_KEY, &mut $app, &$block, &None).handle();

      assert_eq!($app.get_current_route(), &$expected_block.into());
    };
  }

  #[macro_export]
  macro_rules! assert_refresh_key {
    ($handler:ident, $block:expr) => {
      let mut app = App::default();
      app.push_navigation_stack($block.into());

      $handler::with(&DEFAULT_KEYBINDINGS.refresh.key, &mut app, &$block, &None).handle();

      assert_eq!(app.get_current_route(), &$block.into());
      assert!(app.should_refresh);
    };
  }
}
