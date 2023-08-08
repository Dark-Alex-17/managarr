use radarr_handlers::RadarrHandler;

use crate::app::key_binding::DEFAULT_KEYBINDINGS;
use crate::app::App;
use crate::event::Key;
use crate::models::{HorizontallyScrollableText, Route};

mod radarr_handlers;

pub trait KeyEventHandler<'a, T: Into<Route>> {
  fn handle_key_event(&mut self) {
    let key = self.get_key();
    match key {
      _ if *key == DEFAULT_KEYBINDINGS.up.key => self.handle_scroll_up(),
      _ if *key == DEFAULT_KEYBINDINGS.down.key => self.handle_scroll_down(),
      _ if *key == DEFAULT_KEYBINDINGS.home.key => self.handle_home(),
      _ if *key == DEFAULT_KEYBINDINGS.end.key => self.handle_end(),
      _ if *key == DEFAULT_KEYBINDINGS.delete.key => self.handle_delete(),
      _ if *key == DEFAULT_KEYBINDINGS.left.key || *key == DEFAULT_KEYBINDINGS.right.key => {
        self.handle_left_right_action()
      }
      _ if *key == DEFAULT_KEYBINDINGS.submit.key => self.handle_submit(),
      _ if *key == DEFAULT_KEYBINDINGS.esc.key => self.handle_esc(),
      _ => self.handle_char_key_event(),
    }
  }

  fn handle(&mut self) {
    self.handle_key_event();
  }

  fn with(key: &'a Key, app: &'a mut App, active_block: &'a T) -> Self;
  fn get_key(&self) -> &Key;
  fn handle_scroll_up(&mut self);
  fn handle_scroll_down(&mut self);
  fn handle_home(&mut self);
  fn handle_end(&mut self);
  fn handle_delete(&mut self);
  fn handle_left_right_action(&mut self);
  fn handle_submit(&mut self);
  fn handle_esc(&mut self);
  fn handle_char_key_event(&mut self);
}

pub fn handle_events(key: Key, app: &mut App) {
  if let Route::Radarr(active_radarr_block) = app.get_current_route().clone() {
    RadarrHandler::with(&key, app, &active_radarr_block).handle()
  }
}

fn handle_clear_errors(app: &mut App) {
  if !app.error.text.is_empty() {
    app.error = HorizontallyScrollableText::default();
  }
}

fn handle_prompt_toggle(app: &mut App, key: &Key) {
  match key {
    _ if *key == DEFAULT_KEYBINDINGS.left.key || *key == DEFAULT_KEYBINDINGS.right.key => {
      if let Route::Radarr(_) = app.get_current_route().clone() {
        app.data.radarr_data.prompt_confirm = !app.data.radarr_data.prompt_confirm;
      }
    }
    _ => (),
  }
}

#[macro_export]
macro_rules! handle_text_box_keys {
  ($self:expr, $key:expr, $input:expr) => {
    match $self.key {
      _ if *$key == DEFAULT_KEYBINDINGS.backspace.key => {
        $input.pop();
      }
      Key::Char(character) => {
        $input.push(*character);
      }
      _ => (),
    }
  };
}

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
  }

  #[macro_export]
  macro_rules! test_iterable_scroll {
    ($func:ident, $handler:ident, $data_ref:ident, $block:expr) => {
      #[rstest]
      fn $func(#[values(DEFAULT_KEYBINDINGS.up.key, DEFAULT_KEYBINDINGS.down.key)] key: Key) {
        let mut app = App::default();
        app
          .data
          .radarr_data
          .$data_ref
          .set_items(vec!["Test 1".to_owned(), "Test 2".to_owned()]);

        $handler::with(&key, &mut app, &$block).handle();

        assert_str_eq!(app.data.radarr_data.$data_ref.current_selection(), "Test 2");

        $handler::with(&key, &mut app, &$block).handle();

        assert_str_eq!(app.data.radarr_data.$data_ref.current_selection(), "Test 1");
      }
    };
    ($func:ident, $handler:ident, $data_ref:ident, $items:ident, $block:expr, $field:ident) => {
      #[rstest]
      fn $func(#[values(DEFAULT_KEYBINDINGS.up.key, DEFAULT_KEYBINDINGS.down.key)] key: Key) {
        let mut app = App::default();
        app
          .data
          .radarr_data
          .$data_ref
          .set_items(simple_stateful_iterable_vec!($items));

        $handler::with(&key, &mut app, &$block).handle();

        assert_str_eq!(
          app.data.radarr_data.$data_ref.current_selection().$field,
          "Test 2"
        );

        $handler::with(&key, &mut app, &$block).handle();

        assert_str_eq!(
          app.data.radarr_data.$data_ref.current_selection().$field,
          "Test 1"
        );
      }
    };
    ($func:ident, $handler:ident, $data_ref:ident, $items:expr, $block:expr, $field:ident, $conversion_fn:ident) => {
      #[rstest]
      fn $func(#[values(DEFAULT_KEYBINDINGS.up.key, DEFAULT_KEYBINDINGS.down.key)] key: Key) {
        let mut app = App::default();
        app.data.radarr_data.$data_ref.set_items($items);

        $handler::with(&key, &mut app, &$block).handle();

        assert_str_eq!(
          app
            .data
            .radarr_data
            .$data_ref
            .current_selection()
            .$field
            .$conversion_fn(),
          "Test 2"
        );

        $handler::with(&key, &mut app, &$block).handle();

        assert_str_eq!(
          app
            .data
            .radarr_data
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
  macro_rules! test_enum_scroll {
    ($func:ident, $handler:ident, $name:ident, $data_ref:ident, $block:expr) => {
      #[rstest]
      fn $func(#[values(DEFAULT_KEYBINDINGS.up.key, DEFAULT_KEYBINDINGS.down.key)] key: Key) {
        let reference_vec = Vec::from_iter($name::iter());
        let mut app = App::default();
        app
          .data
          .radarr_data
          .$data_ref
          .set_items(reference_vec.clone());

        if key == Key::Up {
          for i in (0..reference_vec.len()).rev() {
            $handler::with(&key, &mut app, &$block).handle();

            assert_eq!(
              app.data.radarr_data.$data_ref.current_selection(),
              &reference_vec[i]
            );
          }
        } else {
          for i in 0..reference_vec.len() {
            $handler::with(&key, &mut app, &$block).handle();

            assert_eq!(
              app.data.radarr_data.$data_ref.current_selection(),
              &reference_vec[(i + 1) % reference_vec.len()]
            );
          }
        }
      }
    };
  }

  #[macro_export]
  macro_rules! test_iterable_home_and_end {
    ($func:ident, $handler:ident, $data_ref:ident, $block:expr) => {
      #[test]
      fn $func() {
        let mut app = App::default();
        app.data.radarr_data.$data_ref.set_items(vec![
          "Test 1".to_owned(),
          "Test 2".to_owned(),
          "Test 3".to_owned(),
        ]);

        $handler::with(&DEFAULT_KEYBINDINGS.end.key, &mut app, &$block).handle();

        assert_str_eq!(app.data.radarr_data.$data_ref.current_selection(), "Test 3");

        $handler::with(&DEFAULT_KEYBINDINGS.home.key, &mut app, &$block).handle();

        assert_str_eq!(app.data.radarr_data.$data_ref.current_selection(), "Test 1");
      }
    };
    ($func:ident, $handler:ident, $data_ref:ident, $items:ident, $block:expr, $field:ident) => {
      #[test]
      fn $func() {
        let mut app = App::default();
        app
          .data
          .radarr_data
          .$data_ref
          .set_items(extended_stateful_iterable_vec!($items));

        $handler::with(&DEFAULT_KEYBINDINGS.end.key, &mut app, &$block).handle();

        assert_str_eq!(
          app.data.radarr_data.$data_ref.current_selection().$field,
          "Test 3"
        );

        $handler::with(&DEFAULT_KEYBINDINGS.home.key, &mut app, &$block).handle();

        assert_str_eq!(
          app.data.radarr_data.$data_ref.current_selection().$field,
          "Test 1"
        );
      }
    };
    ($func:ident, $handler:ident, $data_ref:ident, $items:expr, $block:expr, $field:ident, $conversion_fn:ident) => {
      #[test]
      fn $func() {
        let mut app = App::default();
        app.data.radarr_data.$data_ref.set_items($items);

        $handler::with(&DEFAULT_KEYBINDINGS.end.key, &mut app, &$block).handle();

        assert_str_eq!(
          app
            .data
            .radarr_data
            .$data_ref
            .current_selection()
            .$field
            .$conversion_fn(),
          "Test 3"
        );

        $handler::with(&DEFAULT_KEYBINDINGS.home.key, &mut app, &$block).handle();

        assert_str_eq!(
          app
            .data
            .radarr_data
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
  macro_rules! test_enum_home_and_end {
    ($func:ident, $handler:ident, $name:ident, $data_ref:ident, $block:expr) => {
      #[test]
      fn $func() {
        let reference_vec = Vec::from_iter($name::iter());
        let mut app = App::default();
        app
          .data
          .radarr_data
          .$data_ref
          .set_items(reference_vec.clone());

        $handler::with(&DEFAULT_KEYBINDINGS.end.key, &mut app, &$block).handle();

        assert_eq!(
          app.data.radarr_data.$data_ref.current_selection(),
          &reference_vec[reference_vec.len() - 1]
        );

        $handler::with(&DEFAULT_KEYBINDINGS.home.key, &mut app, &$block).handle();

        assert_eq!(
          app.data.radarr_data.$data_ref.current_selection(),
          &reference_vec[0]
        );
      }
    };
  }

  #[macro_export]
  macro_rules! test_handler_delegation {
    ($base:expr, $active_block:expr) => {
      let mut app = App::default();
      app.push_navigation_stack($base.clone().into());
      app.push_navigation_stack($active_block.clone().into());

      RadarrHandler::with(&DEFAULT_KEYBINDINGS.esc.key, &mut app, &$active_block).handle();

      assert_eq!(app.get_current_route(), &$base.into());
    };
  }
}
#[cfg(test)]
mod tests {
  use rstest::rstest;

  use crate::app::App;
  use crate::event::Key;
  use crate::handlers::{handle_clear_errors, handle_prompt_toggle};

  #[test]
  fn test_handle_clear_errors() {
    let mut app = App::default();
    app.error = "test error".to_owned().into();

    handle_clear_errors(&mut app);

    assert!(app.error.text.is_empty());
  }

  #[rstest]
  fn test_handle_prompt_toggle_left_right(#[values(Key::Left, Key::Right)] key: Key) {
    let mut app = App::default();

    assert!(!app.data.radarr_data.prompt_confirm);

    handle_prompt_toggle(&mut app, &key);

    assert!(app.data.radarr_data.prompt_confirm);

    handle_prompt_toggle(&mut app, &key);

    assert!(!app.data.radarr_data.prompt_confirm);
  }
}
