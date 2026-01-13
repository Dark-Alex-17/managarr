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
        let mut app = App::test_default();
        app.push_navigation_stack($block.into());
        app
          .data
          .$servarr_data
          .$data_ref
          .set_items(vec!["Test 1".to_owned(), "Test 2".to_owned()]);

        $handler::new(&key, &mut app, &$block, &$context).handle();

        pretty_assertions::assert_str_eq!(
          app.data.$servarr_data.$data_ref.current_selection(),
          "Test 2"
        );

        $handler::new(&key, &mut app, &$block, &$context).handle();

        pretty_assertions::assert_str_eq!(
          app.data.$servarr_data.$data_ref.current_selection(),
          "Test 1"
        );
      }
    };

    ($func:ident, $handler:ident, $servarr_data:ident, $data_ref:ident, $items:ident, $block:expr, $context:expr, $field:ident) => {
      #[rstest]
      fn $func(#[values(DEFAULT_KEYBINDINGS.up.key, DEFAULT_KEYBINDINGS.down.key)] key: Key) {
        let mut app = App::test_default();
        app.push_navigation_stack($block.into());
        app
          .data
          .$servarr_data
          .$data_ref
          .set_items(simple_stateful_iterable_vec!($items));

        $handler::new(key, &mut app, $block, $context).handle();

        pretty_assertions::assert_str_eq!(
          app.data.$servarr_data.$data_ref.current_selection().$field,
          "Test 2"
        );

        $handler::new(key, &mut app, $block, $context).handle();

        pretty_assertions::assert_str_eq!(
          app.data.$servarr_data.$data_ref.current_selection().$field,
          "Test 1"
        );
      }
    };

    ($func:ident, $handler:ident, $servarr_data:ident, $data_ref:ident, $items:expr, $block:expr, $context:expr, $field:ident) => {
      #[rstest]
      fn $func(#[values(DEFAULT_KEYBINDINGS.up.key, DEFAULT_KEYBINDINGS.down.key)] key: Key) {
        let mut app = App::test_default();
        app.push_navigation_stack($block.into());
        app.data.$servarr_data.$data_ref.set_items($items);

        $handler::new(key, &mut app, $block, $context).handle();

        pretty_assertions::assert_str_eq!(
          app.data.$servarr_data.$data_ref.current_selection().$field,
          "Test 2"
        );

        $handler::new(key, &mut app, $block, $context).handle();

        pretty_assertions::assert_str_eq!(
          app.data.$servarr_data.$data_ref.current_selection().$field,
          "Test 1"
        );
      }
    };

    ($func:ident, $handler:ident, $servarr_data:ident, $data_ref:ident, $items:expr, $block:expr, $context:expr, $field:ident, $conversion_fn:ident) => {
      #[rstest]
      fn $func(#[values(DEFAULT_KEYBINDINGS.up.key, DEFAULT_KEYBINDINGS.down.key)] key: Key) {
        let mut app = App::test_default();
        app.push_navigation_stack($block.into());
        app.data.$servarr_data.$data_ref.set_items($items);

        $handler::new(key, &mut app, $block, $context).handle();

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

        $handler::new(key, &mut app, $block, $context).handle();

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
        let mut app = App::test_default();
        app.push_navigation_stack($block.into());
        app.data.$servarr_data.$data_ref.set_items(vec![
          "Test 1".to_owned(),
          "Test 2".to_owned(),
          "Test 3".to_owned(),
        ]);

        $handler::new(DEFAULT_KEYBINDINGS.end.key, &mut app, $block, $context).handle();

        pretty_assertions::assert_str_eq!(
          app.data.$servarr_data.$data_ref.current_selection(),
          "Test 3"
        );

        $handler::new(DEFAULT_KEYBINDINGS.home.key, &mut app, $block, $context).handle();

        pretty_assertions::assert_str_eq!(
          app.data.$servarr_data.$data_ref.current_selection(),
          "Test 1"
        );
      }
    };

    ($func:ident, $handler:ident, $servarr_data:ident, $data_ref:ident, $items:ident, $block:expr, $context:expr, $field:ident) => {
      #[test]
      fn $func() {
        let mut app = App::test_default();
        app.push_navigation_stack($block.into());
        app
          .data
          .$servarr_data
          .$data_ref
          .set_items(extended_stateful_iterable_vec!($items));

        $handler::new(DEFAULT_KEYBINDINGS.end.key, &mut app, $block, $context).handle();

        pretty_assertions::assert_str_eq!(
          app.data.$servarr_data.$data_ref.current_selection().$field,
          "Test 3"
        );

        $handler::new(DEFAULT_KEYBINDINGS.home.key, &mut app, $block, $context).handle();

        pretty_assertions::assert_str_eq!(
          app.data.$servarr_data.$data_ref.current_selection().$field,
          "Test 1"
        );
      }
    };

    ($func:ident, $handler:ident, $servarr_data:ident, $data_ref:ident, $items:expr, $block:expr, $context:expr, $field:ident) => {
      #[test]
      fn $func() {
        let mut app = App::test_default();
        app.push_navigation_stack($block.into());
        app.data.$servarr_data.$data_ref.set_items($items);

        $handler::new(DEFAULT_KEYBINDINGS.end.key, &mut app, $block, $context).handle();

        pretty_assertions::assert_str_eq!(
          app.data.$servarr_data.$data_ref.current_selection().$field,
          "Test 3"
        );

        $handler::new(DEFAULT_KEYBINDINGS.home.key, &mut app, $block, $context).handle();

        pretty_assertions::assert_str_eq!(
          app.data.$servarr_data.$data_ref.current_selection().$field,
          "Test 1"
        );
      }
    };

    ($func:ident, $handler:ident, $servarr_data:ident, $data_ref:ident, $items:expr, $block:expr, $context:expr, $field:ident, $conversion_fn:ident) => {
      #[test]
      fn $func() {
        let mut app = App::test_default();
        app.push_navigation_stack($block.into());
        app.data.$servarr_data.$data_ref.set_items($items);

        $handler::new(DEFAULT_KEYBINDINGS.end.key, &mut app, $block, $context).handle();

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

        $handler::new(DEFAULT_KEYBINDINGS.home.key, &mut app, $block, $context).handle();

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
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack($base.into());
      app.push_navigation_stack($active_block.into());

      $handler::new(DEFAULT_KEYBINDINGS.esc.key, &mut app, $active_block, None).handle();

      pretty_assertions::assert_eq!(app.get_current_route(), $base.into());
    };
  }

  #[macro_export]
  macro_rules! assert_delete_prompt {
    ($handler:ident, $block:expr, $expected_block:expr) => {
      let mut app = App::test_default();

      $handler::new(DELETE_KEY, &mut app, $block, None).handle();

      pretty_assertions::assert_eq!(app.get_current_route(), $expected_block.into());
    };

    ($handler:ident, $app:expr, $block:expr, $expected_block:expr) => {
      $handler::new(DELETE_KEY, &mut $app, $block, None).handle();

      pretty_assertions::assert_eq!($app.get_current_route(), $expected_block.into());
    };
  }

  #[macro_export]
  macro_rules! assert_refresh_key {
    ($handler:ident, $block:expr) => {
      let mut app = App::test_default();
      app.push_navigation_stack($block.into());

      $handler::new(DEFAULT_KEYBINDINGS.refresh.key, &mut app, $block, None).handle();

      pretty_assertions::assert_eq!(app.get_current_route(), $block.into());
      assert!(app.should_refresh);
    };
  }

  #[macro_export]
  macro_rules! assert_modal_present {
    ($modal:expr) => {
      assert!($modal.is_some(), "Expected modal to be present");
    };
  }

  #[macro_export]
  macro_rules! assert_modal_absent {
    ($modal:expr) => {
      assert!($modal.is_none(), "Expected modal to be absent");
    };
  }

  #[macro_export]
  macro_rules! assert_navigation_pushed {
    ($app:expr, $expected_route:expr) => {
      pretty_assertions::assert_eq!(
        $app.get_current_route(),
        $expected_route,
        "Expected route to be pushed onto navigation stack"
      );
    };
  }

  #[macro_export]
  macro_rules! assert_navigation_popped {
    ($app:expr, $expected_route:expr) => {
      pretty_assertions::assert_eq!(
        $app.get_current_route(),
        $expected_route,
        "Expected route after popping navigation stack"
      );
    };
  }
}
