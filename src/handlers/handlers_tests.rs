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
