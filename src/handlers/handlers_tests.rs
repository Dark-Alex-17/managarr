#[cfg(test)]
mod tests {
  use pretty_assertions::assert_eq;
  use rstest::rstest;

  use crate::app::key_binding::DEFAULT_KEYBINDINGS;
  use crate::app::App;
  use crate::event::Key;
  use crate::handlers::handle_events;
  use crate::handlers::{handle_clear_errors, handle_prompt_toggle};
  use crate::models::servarr_data::radarr::radarr_data::ActiveRadarrBlock;
  use crate::models::servarr_data::sonarr::sonarr_data::ActiveSonarrBlock;
  use crate::models::HorizontallyScrollableText;
  use crate::models::Route;

  #[test]
  fn test_handle_clear_errors() {
    let mut app = App::default();
    app.error = "test error".to_owned().into();

    handle_clear_errors(&mut app);

    assert!(app.error.text.is_empty());
  }

  #[rstest]
  #[case(0, ActiveSonarrBlock::Series, ActiveSonarrBlock::Series)]
  #[case(1, ActiveRadarrBlock::Movies, ActiveRadarrBlock::Movies)]
  fn test_handle_change_tabs<T>(#[case] index: usize, #[case] left_block: T, #[case] right_block: T)
  where
    T: Into<Route> + Copy,
  {
    let mut app = App::default();
    app.error = "Test".into();
    app.server_tabs.set_index(index);

    handle_events(DEFAULT_KEYBINDINGS.previous_servarr.key, &mut app);

    assert_eq!(app.server_tabs.get_active_route(), left_block.into());
    assert_eq!(app.get_current_route(), left_block.into());
    assert!(app.is_first_render);
    assert_eq!(app.error, HorizontallyScrollableText::default());

    app.server_tabs.set_index(index);
    app.is_first_render = false;
    app.error = "Test".into();

    handle_events(DEFAULT_KEYBINDINGS.next_servarr.key, &mut app);

    assert_eq!(app.server_tabs.get_active_route(), right_block.into());
    assert_eq!(app.get_current_route(), right_block.into());
    assert!(app.is_first_render);
    assert_eq!(app.error, HorizontallyScrollableText::default());
  }

  #[rstest]
  fn test_handle_prompt_toggle_left_right(#[values(Key::Left, Key::Right)] key: Key) {
    let mut app = App::default();

    assert!(!app.data.radarr_data.prompt_confirm);

    handle_prompt_toggle(&mut app, key);

    assert!(app.data.radarr_data.prompt_confirm);

    handle_prompt_toggle(&mut app, key);

    assert!(!app.data.radarr_data.prompt_confirm);
  }
}
