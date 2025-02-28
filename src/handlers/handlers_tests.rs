#[cfg(test)]
mod tests {
  use crate::models::radarr_models::Movie;
  use crate::models::sonarr_models::Series;
  use pretty_assertions::assert_eq;
  use rstest::rstest;
  use tokio_util::sync::CancellationToken;

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
    let mut app = App::test_default();
    app.error = "test error".to_owned().into();

    handle_clear_errors(&mut app);

    assert!(app.error.text.is_empty());
  }

  #[rstest]
  #[case(ActiveRadarrBlock::Movies.into(), ActiveRadarrBlock::SearchMovie.into())]
  #[case(ActiveSonarrBlock::Series.into(), ActiveSonarrBlock::SearchSeries.into())]
  fn test_handle_events(#[case] base_block: Route, #[case] top_block: Route) {
    let mut app = App::test_default();
    app.push_navigation_stack(base_block);
    app.push_navigation_stack(top_block);
    app
      .data
      .sonarr_data
      .series
      .set_items(vec![Series::default()]);
    app
      .data
      .radarr_data
      .movies
      .set_items(vec![Movie::default()]);

    handle_events(DEFAULT_KEYBINDINGS.esc.key, &mut app);

    assert_eq!(app.get_current_route(), base_block);
  }

  #[rstest]
  #[case(0, ActiveSonarrBlock::Series, ActiveSonarrBlock::Series)]
  #[case(1, ActiveRadarrBlock::Movies, ActiveRadarrBlock::Movies)]
  fn test_handle_change_tabs<T>(#[case] index: usize, #[case] left_block: T, #[case] right_block: T)
  where
    T: Into<Route> + Copy,
  {
    let mut app = App::test_default();
    app.error = "Test".into();
    app.server_tabs.set_index(index);

    handle_events(DEFAULT_KEYBINDINGS.previous_servarr.key, &mut app);

    assert_eq!(app.server_tabs.get_active_route(), left_block.into());
    assert_eq!(app.get_current_route(), left_block.into());
    assert!(app.is_first_render);
    assert_eq!(app.error, HorizontallyScrollableText::default());
    assert!(app.cancellation_token.is_cancelled());

    app.server_tabs.set_index(index);
    app.is_first_render = false;
    app.error = "Test".into();
    app.cancellation_token = CancellationToken::new();

    handle_events(DEFAULT_KEYBINDINGS.next_servarr.key, &mut app);

    assert_eq!(app.server_tabs.get_active_route(), right_block.into());
    assert_eq!(app.get_current_route(), right_block.into());
    assert!(app.is_first_render);
    assert_eq!(app.error, HorizontallyScrollableText::default());
    assert!(app.cancellation_token.is_cancelled());
  }

  #[rstest]
  fn test_handle_prompt_toggle_left_right_radarr(#[values(Key::Left, Key::Right)] key: Key) {
    let mut app = App::test_default();
    app.push_navigation_stack(ActiveRadarrBlock::Movies.into());

    assert!(!app.data.radarr_data.prompt_confirm);

    handle_prompt_toggle(&mut app, key);

    assert!(app.data.radarr_data.prompt_confirm);

    handle_prompt_toggle(&mut app, key);

    assert!(!app.data.radarr_data.prompt_confirm);
  }

  #[rstest]
  fn test_handle_prompt_toggle_left_right_sonarr(#[values(Key::Left, Key::Right)] key: Key) {
    let mut app = App::test_default();
    app.push_navigation_stack(ActiveSonarrBlock::Series.into());

    assert!(!app.data.sonarr_data.prompt_confirm);

    handle_prompt_toggle(&mut app, key);

    assert!(app.data.sonarr_data.prompt_confirm);

    handle_prompt_toggle(&mut app, key);

    assert!(!app.data.sonarr_data.prompt_confirm);
  }
}
