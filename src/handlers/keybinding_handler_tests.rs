#[cfg(test)]
mod tests {
  use crate::app::App;
  use crate::app::key_binding::DEFAULT_KEYBINDINGS;
  use crate::event::Key;
  use crate::handlers::KeyEventHandler;
  use crate::handlers::KeybindingHandler;
  use crate::models::servarr_data::ActiveKeybindingBlock;
  use crate::models::stateful_table::StatefulTable;
  use rstest::rstest;

  mod test_handle_esc {
    use super::*;
    use crate::models::servarr_data::radarr::radarr_data::ActiveRadarrBlock;
    use pretty_assertions::assert_eq;

    const ESC_KEY: Key = DEFAULT_KEYBINDINGS.esc.key;

    #[test]
    fn test_esc_empties_keymapping_table() {
      let mut app = App::test_default();
      app.is_loading = true;
      app.push_navigation_stack(ActiveRadarrBlock::Movies.into());

      KeybindingHandler::new(ESC_KEY, &mut app, ActiveKeybindingBlock::Help, None).handle();

      assert_eq!(app.get_current_route(), ActiveRadarrBlock::Movies.into());
      assert!(app.keymapping_table.is_none());
    }
  }

  #[test]
  fn test_keybinding_handler_accepts() {
    assert!(KeybindingHandler::accepts(ActiveKeybindingBlock::Help));
  }

  #[test]
  fn test_keybinding_handler_not_ready_when_keybinding_is_empty() {
    let mut app = App::test_default();
    app.is_loading = false;

    let handler = KeybindingHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveKeybindingBlock::Help,
      None,
    );

    assert!(!handler.is_ready());
  }

  #[rstest]
  fn test_keybinding_handler_ready_when_keymapping_table_is_not_empty(
    #[values(true, false)] is_loading: bool,
  ) {
    let mut app = App::test_default();
    app.keymapping_table = Some(StatefulTable::default());
    app.is_loading = is_loading;

    let handler = KeybindingHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveKeybindingBlock::Help,
      None,
    );

    assert!(handler.is_ready());
  }
}
