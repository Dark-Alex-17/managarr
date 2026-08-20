#[cfg(test)]
mod tests {
  use crate::app::App;
  use crate::app::key_binding::DEFAULT_KEYBINDINGS;
  use crate::handlers::KeyEventHandler;
  use crate::handlers::readarr_handlers::ReadarrHandler;
  use crate::models::servarr_data::readarr::readarr_data::ActiveReadarrBlock;
  use pretty_assertions::assert_eq;
  use rstest::rstest;
  use strum::IntoEnumIterator;

  #[rstest]
  fn test_readarr_handler_ignore_special_keys(
    #[values(true, false)] ignore_special_keys_for_textbox_input: bool,
  ) {
    let mut app = App::test_default();
    app.ignore_special_keys_for_textbox_input = ignore_special_keys_for_textbox_input;
    let handler = ReadarrHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveReadarrBlock::default(),
      None,
    );

    assert_eq!(
      handler.ignore_special_keys(),
      ignore_special_keys_for_textbox_input
    );
  }

  #[test]
  fn test_readarr_handler_is_ready() {
    let mut app = App::test_default();
    app.is_loading = true;

    let handler = ReadarrHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveReadarrBlock::default(),
      None,
    );

    assert!(handler.is_ready());
  }

  #[test]
  fn test_readarr_handler_accepts() {
    for readarr_block in ActiveReadarrBlock::iter() {
      assert!(ReadarrHandler::accepts(readarr_block));
    }
  }
}
