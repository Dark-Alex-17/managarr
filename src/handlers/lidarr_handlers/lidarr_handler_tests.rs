#[cfg(test)]
mod tests {
  use rstest::rstest;
  use strum::IntoEnumIterator;
  use crate::app::App;
  use crate::app::key_binding::DEFAULT_KEYBINDINGS;
  use crate::handlers::KeyEventHandler;
  use crate::handlers::lidarr_handlers::LidarrHandler;
  use crate::models::servarr_data::lidarr::lidarr_data::ActiveLidarrBlock;

  #[rstest]
  fn test_lidarr_handler_ignore_special_keys(
    #[values(true, false)] ignore_special_keys_for_textbox_input: bool,
  ) {
    let mut app = App::test_default();
    app.ignore_special_keys_for_textbox_input = ignore_special_keys_for_textbox_input;
    let handler = LidarrHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveLidarrBlock::default(),
      None,
    );

    assert_eq!(
      handler.ignore_special_keys(),
      ignore_special_keys_for_textbox_input
    );
  }

  #[test]
  fn test_lidarr_handler_is_ready() {
    let mut app = App::test_default();
    app.is_loading = true;

    let handler = LidarrHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveLidarrBlock::default(),
      None,
    );

    assert!(handler.is_ready());
  }

  #[test]
  fn test_lidarr_handler_accepts() {
    for lidarr_block in ActiveLidarrBlock::iter() {
      assert!(LidarrHandler::accepts(lidarr_block));
    }
  }
}
