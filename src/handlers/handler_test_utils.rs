#[cfg(test)]
#[macro_use]
mod test_utils {
  #[macro_export]
  macro_rules! simple_stateful_iterable_vec {
    ($name:ident) => {
      vec![
        $name {
          title: "Test 1".to_owned(),
          ..$name::default()
        },
        $name {
          title: "Test 2".to_owned(),
          ..$name::default()
        },
      ]
    };

    ($name:ident, $title_ident:ident) => {
      vec![
        $name {
          title: $title_ident::from("Test 1".to_owned()),
          ..$name::default()
        },
        $name {
          title: $title_ident::from("Test 2".to_owned()),
          ..$name::default()
        },
      ]
    };

    ($name:ident, $title_ident:ident, $field:ident) => {
      vec![
        $name {
          $field: $title_ident::from("Test 1".to_owned()),
          ..$name::default()
        },
        $name {
          $field: $title_ident::from("Test 2".to_owned()),
          ..$name::default()
        },
      ]
    };
  }

  #[macro_export]
  macro_rules! extended_stateful_iterable_vec {
    ($name:ident) => {
      vec![
        $name {
          title: "Test 1".to_owned(),
          ..$name::default()
        },
        $name {
          title: "Test 2".to_owned(),
          ..$name::default()
        },
        $name {
          title: "Test 3".to_owned(),
          ..$name::default()
        },
      ]
    };

    ($name:ident, $title_ident:ident) => {
      vec![
        $name {
          title: $title_ident::from("Test 1".to_owned()),
          ..$name::default()
        },
        $name {
          title: $title_ident::from("Test 2".to_owned()),
          ..$name::default()
        },
        $name {
          title: $title_ident::from("Test 3".to_owned()),
          ..$name::default()
        },
      ]
    };

    ($name:ident, $title_ident:ident, $field:ident) => {
      vec![
        $name {
          $field: $title_ident::from("Test 1".to_owned()),
          ..$name::default()
        },
        $name {
          $field: $title_ident::from("Test 2".to_owned()),
          ..$name::default()
        },
        $name {
          $field: $title_ident::from("Test 3".to_owned()),
          ..$name::default()
        },
      ]
    };
  }

  #[macro_export]
  macro_rules! test_iterable_scroll {
    ($func:ident, $handler:ident, $servarr_data:ident, $data_ref:ident, $block:expr, $context:expr) => {
      #[rstest]
      fn $func(#[values(DEFAULT_KEYBINDINGS.up.key, DEFAULT_KEYBINDINGS.down.key)] key: Key) {
        let mut app = App::default();
        app.push_navigation_stack($block.into());
        app
          .data
          .$servarr_data
          .$data_ref
          .set_items(vec!["Test 1".to_owned(), "Test 2".to_owned()]);

        $handler::with(&key, &mut app, &$block, &$context).handle();

        pretty_assertions::assert_str_eq!(
          app.data.$servarr_data.$data_ref.current_selection(),
          "Test 2"
        );

        $handler::with(&key, &mut app, &$block, &$context).handle();

        pretty_assertions::assert_str_eq!(
          app.data.$servarr_data.$data_ref.current_selection(),
          "Test 1"
        );
      }
    };

    ($func:ident, $handler:ident, $servarr_data:ident, $data_ref:ident, $items:ident, $block:expr, $context:expr, $field:ident) => {
      #[rstest]
      fn $func(#[values(DEFAULT_KEYBINDINGS.up.key, DEFAULT_KEYBINDINGS.down.key)] key: Key) {
        let mut app = App::default();
        app.push_navigation_stack($block.into());
        app
          .data
          .$servarr_data
          .$data_ref
          .set_items(simple_stateful_iterable_vec!($items));

        $handler::with(key, &mut app, $block, $context).handle();

        pretty_assertions::assert_str_eq!(
          app.data.$servarr_data.$data_ref.current_selection().$field,
          "Test 2"
        );

        $handler::with(key, &mut app, $block, $context).handle();

        pretty_assertions::assert_str_eq!(
          app.data.$servarr_data.$data_ref.current_selection().$field,
          "Test 1"
        );
      }
    };

    ($func:ident, $handler:ident, $servarr_data:ident, $data_ref:ident, $items:expr, $block:expr, $context:expr, $field:ident) => {
      #[rstest]
      fn $func(#[values(DEFAULT_KEYBINDINGS.up.key, DEFAULT_KEYBINDINGS.down.key)] key: Key) {
        let mut app = App::default();
        app.push_navigation_stack($block.into());
        app.data.$servarr_data.$data_ref.set_items($items);

        $handler::with(key, &mut app, $block, $context).handle();

        pretty_assertions::assert_str_eq!(
          app.data.$servarr_data.$data_ref.current_selection().$field,
          "Test 2"
        );

        $handler::with(key, &mut app, $block, $context).handle();

        pretty_assertions::assert_str_eq!(
          app.data.$servarr_data.$data_ref.current_selection().$field,
          "Test 1"
        );
      }
    };

    ($func:ident, $handler:ident, $servarr_data:ident, $data_ref:ident, $items:expr, $block:expr, $context:expr, $field:ident, $conversion_fn:ident) => {
      #[rstest]
      fn $func(#[values(DEFAULT_KEYBINDINGS.up.key, DEFAULT_KEYBINDINGS.down.key)] key: Key) {
        let mut app = App::default();
        app.push_navigation_stack($block.into());
        app.data.$servarr_data.$data_ref.set_items($items);

        $handler::with(key, &mut app, $block, $context).handle();

        pretty_assertions::assert_str_eq!(
          app
            .data
            .$servarr_data
            .$data_ref
            .current_selection()
            .$field
            .$conversion_fn(),
          "Test 2"
        );

        $handler::with(key, &mut app, $block, $context).handle();

        pretty_assertions::assert_str_eq!(
          app
            .data
            .$servarr_data
            .$data_ref
            .current_selection()
            .$field
            .$conversion_fn(),
          "Test 1"
        );
      }
    };
  }

  #[macro_export]
  macro_rules! test_iterable_home_and_end {
    ($func:ident, $handler:ident, $servarr_data:ident, $data_ref:ident, $block:expr, $context:expr) => {
      #[test]
      fn $func() {
        let mut app = App::default();
        app.push_navigation_stack($block.into());
        app.data.$servarr_data.$data_ref.set_items(vec![
          "Test 1".to_owned(),
          "Test 2".to_owned(),
          "Test 3".to_owned(),
        ]);

        $handler::with(DEFAULT_KEYBINDINGS.end.key, &mut app, $block, $context).handle();

        pretty_assertions::assert_str_eq!(
          app.data.$servarr_data.$data_ref.current_selection(),
          "Test 3"
        );

        $handler::with(DEFAULT_KEYBINDINGS.home.key, &mut app, $block, $context).handle();

        pretty_assertions::assert_str_eq!(
          app.data.$servarr_data.$data_ref.current_selection(),
          "Test 1"
        );
      }
    };

    ($func:ident, $handler:ident, $servarr_data:ident, $data_ref:ident, $items:ident, $block:expr, $context:expr, $field:ident) => {
      #[test]
      fn $func() {
        let mut app = App::default();
        app.push_navigation_stack($block.into());
        app
          .data
          .$servarr_data
          .$data_ref
          .set_items(extended_stateful_iterable_vec!($items));

        $handler::with(DEFAULT_KEYBINDINGS.end.key, &mut app, $block, $context).handle();

        pretty_assertions::assert_str_eq!(
          app.data.$servarr_data.$data_ref.current_selection().$field,
          "Test 3"
        );

        $handler::with(DEFAULT_KEYBINDINGS.home.key, &mut app, $block, $context).handle();

        pretty_assertions::assert_str_eq!(
          app.data.$servarr_data.$data_ref.current_selection().$field,
          "Test 1"
        );
      }
    };

    ($func:ident, $handler:ident, $servarr_data:ident, $data_ref:ident, $items:expr, $block:expr, $context:expr, $field:ident) => {
      #[test]
      fn $func() {
        let mut app = App::default();
        app.push_navigation_stack($block.into());
        app.data.$servarr_data.$data_ref.set_items($items);

        $handler::with(DEFAULT_KEYBINDINGS.end.key, &mut app, $block, $context).handle();

        pretty_assertions::assert_str_eq!(
          app.data.$servarr_data.$data_ref.current_selection().$field,
          "Test 3"
        );

        $handler::with(DEFAULT_KEYBINDINGS.home.key, &mut app, $block, $context).handle();

        pretty_assertions::assert_str_eq!(
          app.data.$servarr_data.$data_ref.current_selection().$field,
          "Test 1"
        );
      }
    };

    ($func:ident, $handler:ident, $servarr_data:ident, $data_ref:ident, $items:expr, $block:expr, $context:expr, $field:ident, $conversion_fn:ident) => {
      #[test]
      fn $func() {
        let mut app = App::default();
        app.push_navigation_stack($block.into());
        app.data.$servarr_data.$data_ref.set_items($items);

        $handler::with(DEFAULT_KEYBINDINGS.end.key, &mut app, $block, $context).handle();

        pretty_assertions::assert_str_eq!(
          app
            .data
            .$servarr_data
            .$data_ref
            .current_selection()
            .$field
            .$conversion_fn(),
          "Test 3"
        );

        $handler::with(DEFAULT_KEYBINDINGS.home.key, &mut app, $block, $context).handle();

        pretty_assertions::assert_str_eq!(
          app
            .data
            .$servarr_data
            .$data_ref
            .current_selection()
            .$field
            .$conversion_fn(),
          "Test 1"
        );
      }
    };
  }

  #[macro_export]
  macro_rules! test_handler_delegation {
    ($handler:ident, $base:expr, $active_block:expr) => {
      let mut app = App::default();
      app.data.sonarr_data.history.set_items(vec![$crate::models::sonarr_models::SonarrHistoryItem::default()]);
      app.data.sonarr_data.root_folders.set_items(vec![$crate::models::servarr_models::RootFolder::default()]);
      app.data.sonarr_data.indexers.set_items(vec![$crate::models::servarr_models::Indexer::default()]);
      app.data.sonarr_data.blocklist.set_items(vec![$crate::models::sonarr_models::BlocklistItem::default()]);
      app.data.sonarr_data.add_searched_series = Some($crate::models::stateful_table::StatefulTable::default());
      app.data.radarr_data.movies.set_items(vec![$crate::models::radarr_models::Movie::default()]);
      app.data.radarr_data.collections.set_items(vec![$crate::models::radarr_models::Collection::default()]);
      app.data.radarr_data.collection_movies.set_items(vec![$crate::models::radarr_models::CollectionMovie::default()]);
      app.data.radarr_data.indexers.set_items(vec![$crate::models::servarr_models::Indexer::default()]);
      app.data.radarr_data.root_folders.set_items(vec![$crate::models::servarr_models::RootFolder::default()]);
      app.data.radarr_data.blocklist.set_items(vec![$crate::models::radarr_models::BlocklistItem::default()]);
      app.data.radarr_data.add_searched_movies = Some($crate::models::stateful_table::StatefulTable::default());
      let mut movie_details_modal = $crate::models::servarr_data::radarr::modals::MovieDetailsModal::default();
      movie_details_modal
        .movie_history
        .set_items(vec![$crate::models::radarr_models::MovieHistoryItem::default()]);
      movie_details_modal.movie_cast.set_items(vec![$crate::models::radarr_models::Credit::default()]);
      movie_details_modal.movie_crew.set_items(vec![$crate::models::radarr_models::Credit::default()]);
      movie_details_modal.movie_releases.set_items(vec![$crate::models::radarr_models::RadarrRelease::default()]);
      app.data.radarr_data.movie_details_modal = Some(movie_details_modal);
      let mut season_details_modal = $crate::models::servarr_data::sonarr::modals::SeasonDetailsModal::default();
      season_details_modal.season_history.set_items(vec![$crate::models::sonarr_models::SonarrHistoryItem::default()]);
      season_details_modal.episode_details_modal = Some($crate::models::servarr_data::sonarr::modals::EpisodeDetailsModal::default());
      app.data.sonarr_data.season_details_modal = Some(season_details_modal);
      let mut series_history = $crate::models::stateful_table::StatefulTable::default();
      series_history.set_items(vec![
        $crate::models::sonarr_models::SonarrHistoryItem::default(),
      ]);
      app.data.sonarr_data.series_history = Some(series_history);
      app.data.sonarr_data.series.set_items(vec![
        $crate::models::sonarr_models::Series::default(),
      ]);
      app.push_navigation_stack($base.into());
      app.push_navigation_stack($active_block.into());

      $handler::with(DEFAULT_KEYBINDINGS.esc.key, &mut app, $active_block, None).handle();

      pretty_assertions::assert_eq!(app.get_current_route(), $base.into());
    };
  }

  #[macro_export]
  macro_rules! assert_delete_prompt {
    ($handler:ident, $block:expr, $expected_block:expr) => {
      let mut app = App::default();

      $handler::with(DELETE_KEY, &mut app, $block, None).handle();

      pretty_assertions::assert_eq!(app.get_current_route(), $expected_block.into());
    };

    ($handler:ident, $app:expr, $block:expr, $expected_block:expr) => {
      $handler::with(DELETE_KEY, &mut $app, $block, None).handle();

      pretty_assertions::assert_eq!($app.get_current_route(), $expected_block.into());
    };
  }

  #[macro_export]
  macro_rules! assert_refresh_key {
    ($handler:ident, $block:expr) => {
      let mut app = App::default();
      app.push_navigation_stack($block.into());

      $handler::with(DEFAULT_KEYBINDINGS.refresh.key, &mut app, $block, None).handle();

      pretty_assertions::assert_eq!(app.get_current_route(), $block.into());
      assert!(app.should_refresh);
    };
  }
}
