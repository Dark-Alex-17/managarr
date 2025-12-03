#[cfg(test)]
mod tests {
  use crate::models::radarr_models::Movie;
  use crate::models::sonarr_models::Series;
  use pretty_assertions::assert_eq;
  use rstest::rstest;
  use tokio_util::sync::CancellationToken;

  use crate::app::App;
  use crate::app::context_clues::SERVARR_CONTEXT_CLUES;
  use crate::app::key_binding::{DEFAULT_KEYBINDINGS, KeyBinding};
  use crate::app::radarr::radarr_context_clues::{
    LIBRARY_CONTEXT_CLUES, MOVIE_DETAILS_CONTEXT_CLUES,
  };
  use crate::event::Key;
  use crate::handlers::{handle_clear_errors, handle_prompt_toggle};
  use crate::handlers::{handle_events, populate_keymapping_table};
  use crate::models::HorizontallyScrollableText;
  use crate::models::Route;
  use crate::models::servarr_data::ActiveKeybindingBlock;
  use crate::models::servarr_data::radarr::radarr_data::{ActiveRadarrBlock, RadarrData};
  use crate::models::servarr_data::sonarr::sonarr_data::ActiveSonarrBlock;
  use crate::models::servarr_models::KeybindingItem;
  use crate::models::stateful_table::StatefulTable;

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

  #[test]
  fn test_handle_populate_keybindings_table_on_help_button_press() {
    let mut app = App::test_default();
    let expected_keybinding_items = Vec::from(SERVARR_CONTEXT_CLUES)
      .iter()
      .map(|(key, desc)| context_clue_to_keybinding_item(key, desc))
      .collect::<Vec<_>>();
    app.push_navigation_stack(ActiveKeybindingBlock::Help.into());

    handle_events(DEFAULT_KEYBINDINGS.help.key, &mut app);

    assert!(app.keymapping_table.is_some());
    assert_eq!(
      expected_keybinding_items,
      app.keymapping_table.unwrap().items
    );
  }

  #[test]
  fn test_handle_ignore_help_button_when_ignore_special_keys_for_textbox_input_is_true() {
    let mut app = App::test_default();
    app.ignore_special_keys_for_textbox_input = true;
    app.push_navigation_stack(ActiveRadarrBlock::default().into());

    handle_events(DEFAULT_KEYBINDINGS.help.key, &mut app);

    assert!(app.keymapping_table.is_none());
  }

  #[test]
  fn test_handle_empties_keybindings_table_on_help_button_press_when_keybindings_table_is_already_populated()
   {
    let mut app = App::test_default();
    let keybinding_items = Vec::from(SERVARR_CONTEXT_CLUES)
      .iter()
      .map(|(key, desc)| context_clue_to_keybinding_item(key, desc))
      .collect::<Vec<_>>();
    let mut stateful_table = StatefulTable::default();
    stateful_table.set_items(keybinding_items);
    app.keymapping_table = Some(stateful_table);
    app.push_navigation_stack(ActiveRadarrBlock::default().into());

    handle_events(DEFAULT_KEYBINDINGS.help.key, &mut app);

    assert!(app.keymapping_table.is_none());
  }

  #[test]
  fn test_handle_shows_keymapping_popup_when_keymapping_table_is_populated() {
    let mut app = App::test_default();
    let keybinding_items = Vec::from(SERVARR_CONTEXT_CLUES)
      .iter()
      .map(|(key, desc)| context_clue_to_keybinding_item(key, desc))
      .collect::<Vec<_>>();
    let mut stateful_table = StatefulTable::default();
    stateful_table.set_items(keybinding_items);
    app.keymapping_table = Some(stateful_table);
    app.push_navigation_stack(ActiveRadarrBlock::default().into());
    let expected_selection = KeybindingItem {
      key: SERVARR_CONTEXT_CLUES[1].0.key.to_string(),
      alt_key: SERVARR_CONTEXT_CLUES[1]
        .0
        .alt
        .map_or(String::new(), |k| k.to_string()),
      desc: SERVARR_CONTEXT_CLUES[1].1.to_string(),
    };

    handle_events(DEFAULT_KEYBINDINGS.down.key, &mut app);

    assert!(app.keymapping_table.is_some());
    assert_eq!(
      &expected_selection,
      app.keymapping_table.unwrap().current_selection()
    );
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

  #[test]
  fn test_populate_keymapping_table_global_options() {
    let expected_keybinding_items = Vec::from(SERVARR_CONTEXT_CLUES)
      .iter()
      .map(|(key, desc)| {
        let (key, alt_key) = if key.alt.is_some() {
          (key.key.to_string(), key.alt.as_ref().unwrap().to_string())
        } else {
          (key.key.to_string(), String::new())
        };
        KeybindingItem {
          key,
          alt_key,
          desc: desc.to_string(),
        }
      })
      .collect::<Vec<_>>();
    let mut app = App::test_default();
    app.push_navigation_stack(ActiveKeybindingBlock::Help.into());

    populate_keymapping_table(&mut app);

    assert!(app.keymapping_table.is_some());
    assert_eq!(
      expected_keybinding_items,
      app.keymapping_table.unwrap().items
    );
  }

  #[test]
  fn test_populate_keymapping_table_populates_servarr_specific_tab_info_before_global_options() {
    let mut expected_keybinding_items = LIBRARY_CONTEXT_CLUES
      .iter()
      .map(|(key, desc)| context_clue_to_keybinding_item(key, desc))
      .collect::<Vec<_>>();
    expected_keybinding_items.extend(
      SERVARR_CONTEXT_CLUES
        .iter()
        .map(|(key, desc)| context_clue_to_keybinding_item(key, desc)),
    );
    let mut app = App::test_default();
    app.data.radarr_data = RadarrData::default();
    app.push_navigation_stack(ActiveRadarrBlock::default().into());

    populate_keymapping_table(&mut app);

    assert!(app.keymapping_table.is_some());
    assert_eq!(
      expected_keybinding_items,
      app.keymapping_table.unwrap().items
    );
  }

  #[test]
  fn test_populate_keymapping_table_populates_delegated_servarr_context_provider_options_before_global_options()
   {
    let mut expected_keybinding_items = MOVIE_DETAILS_CONTEXT_CLUES
      .iter()
      .map(|(key, desc)| context_clue_to_keybinding_item(key, desc))
      .collect::<Vec<_>>();
    expected_keybinding_items.extend(
      SERVARR_CONTEXT_CLUES
        .iter()
        .map(|(key, desc)| context_clue_to_keybinding_item(key, desc)),
    );
    let mut app = App::test_default();
    app.data.radarr_data = RadarrData::default();
    app.push_navigation_stack(ActiveRadarrBlock::MovieDetails.into());

    populate_keymapping_table(&mut app);

    assert!(app.keymapping_table.is_some());
    assert_eq!(
      expected_keybinding_items,
      app.keymapping_table.unwrap().items
    );
  }

  fn context_clue_to_keybinding_item(key: &KeyBinding, desc: &&str) -> KeybindingItem {
    let (key, alt_key) = if key.alt.is_some() {
      (key.key.to_string(), key.alt.as_ref().unwrap().to_string())
    } else {
      (key.key.to_string(), String::new())
    };
    KeybindingItem {
      key,
      alt_key,
      desc: desc.to_string(),
    }
  }
}
