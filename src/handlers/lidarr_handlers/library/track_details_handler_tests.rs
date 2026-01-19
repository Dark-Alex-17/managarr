#[cfg(test)]
mod tests {
  use crate::app::App;
  use crate::app::key_binding::DEFAULT_KEYBINDINGS;
  use crate::assert_navigation_pushed;
  use crate::handlers::KeyEventHandler;
  use crate::handlers::lidarr_handlers::library::track_details_handler::TrackDetailsHandler;
  use crate::models::ScrollableText;
  use crate::models::servarr_data::lidarr::lidarr_data::{ActiveLidarrBlock, TRACK_DETAILS_BLOCKS};
  use crate::models::stateful_table::StatefulTable;
  use pretty_assertions::assert_eq;
  use rstest::rstest;
  use strum::IntoEnumIterator;

  mod test_handle_left_right_actions {
    use super::*;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    #[rstest]
    #[case(ActiveLidarrBlock::TrackDetails, ActiveLidarrBlock::TrackHistory)]
    #[case(ActiveLidarrBlock::TrackHistory, ActiveLidarrBlock::TrackDetails)]
    fn test_track_details_tabs_left_right_action(
      #[case] left_block: ActiveLidarrBlock,
      #[case] right_block: ActiveLidarrBlock,
      #[values(true, false)] is_ready: bool,
    ) {
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack(ActiveLidarrBlock::AlbumDetails.into());
      app.is_loading = is_ready;
      app.push_navigation_stack(right_block.into());
      app
        .data
        .lidarr_data
        .album_details_modal
        .as_mut()
        .unwrap()
        .track_details_modal
        .as_mut()
        .unwrap()
        .track_details_tabs
        .index = app
        .data
        .lidarr_data
        .album_details_modal
        .as_ref()
        .unwrap()
        .track_details_modal
        .as_ref()
        .unwrap()
        .track_details_tabs
        .tabs
        .iter()
        .position(|tab_route| tab_route.route == right_block.into())
        .unwrap_or_default();

      TrackDetailsHandler::new(DEFAULT_KEYBINDINGS.left.key, &mut app, right_block, None).handle();

      assert_eq!(
        app.get_current_route(),
        app
          .data
          .lidarr_data
          .album_details_modal
          .as_ref()
          .unwrap()
          .track_details_modal
          .as_ref()
          .unwrap()
          .track_details_tabs
          .get_active_route()
      );
      assert_navigation_pushed!(app, left_block.into());

      TrackDetailsHandler::new(DEFAULT_KEYBINDINGS.right.key, &mut app, left_block, None).handle();

      assert_eq!(
        app.get_current_route(),
        app
          .data
          .lidarr_data
          .album_details_modal
          .as_ref()
          .unwrap()
          .track_details_modal
          .as_ref()
          .unwrap()
          .track_details_tabs
          .get_active_route()
      );
      assert_navigation_pushed!(app, right_block.into());
    }
  }

  mod test_handle_submit {
    use super::*;
    use crate::event::Key;
    use crate::models::stateful_table::StatefulTable;
    use pretty_assertions::assert_eq;

    const SUBMIT_KEY: Key = DEFAULT_KEYBINDINGS.submit.key;

    #[test]
    fn test_track_history_submit() {
      let mut app = App::test_default_fully_populated();

      TrackDetailsHandler::new(SUBMIT_KEY, &mut app, ActiveLidarrBlock::TrackHistory, None)
        .handle();

      assert_navigation_pushed!(app, ActiveLidarrBlock::TrackHistoryDetails.into());
    }

    #[test]
    fn test_track_history_submit_no_op_when_track_history_is_empty() {
      let mut app = App::test_default_fully_populated();
      app
        .data
        .lidarr_data
        .album_details_modal
        .as_mut()
        .unwrap()
        .track_details_modal
        .as_mut()
        .unwrap()
        .track_history = StatefulTable::default();
      app.push_navigation_stack(ActiveLidarrBlock::TrackHistory.into());

      TrackDetailsHandler::new(SUBMIT_KEY, &mut app, ActiveLidarrBlock::TrackHistory, None)
        .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveLidarrBlock::TrackHistory.into()
      );
    }

    #[test]
    fn test_track_history_submit_no_op_when_not_ready() {
      let mut app = App::test_default_fully_populated();
      app.is_loading = true;
      app.push_navigation_stack(ActiveLidarrBlock::TrackHistory.into());

      TrackDetailsHandler::new(SUBMIT_KEY, &mut app, ActiveLidarrBlock::TrackHistory, None)
        .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveLidarrBlock::TrackHistory.into()
      );
    }
  }

  mod test_handle_esc {
    use super::*;
    use crate::assert_navigation_popped;
    use crate::event::Key;

    const ESC_KEY: Key = DEFAULT_KEYBINDINGS.esc.key;

    #[test]
    fn test_track_history_details_block_esc() {
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack(ActiveLidarrBlock::TrackHistory.into());
      app.push_navigation_stack(ActiveLidarrBlock::TrackHistoryDetails.into());

      TrackDetailsHandler::new(
        ESC_KEY,
        &mut app,
        ActiveLidarrBlock::TrackHistoryDetails,
        None,
      )
      .handle();

      assert_navigation_popped!(app, ActiveLidarrBlock::TrackHistory.into());
    }

    #[rstest]
    fn test_track_details_tabs_esc(
      #[values(ActiveLidarrBlock::TrackDetails, ActiveLidarrBlock::TrackHistory)]
      active_lidarr_block: ActiveLidarrBlock,
    ) {
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack(ActiveLidarrBlock::AlbumDetails.into());
      app.push_navigation_stack(active_lidarr_block.into());

      TrackDetailsHandler::new(ESC_KEY, &mut app, active_lidarr_block, None).handle();

      assert_navigation_popped!(app, ActiveLidarrBlock::AlbumDetails.into());
      assert_none!(
        app
          .data
          .lidarr_data
          .album_details_modal
          .as_ref()
          .unwrap()
          .track_details_modal
      );
    }
  }

  mod test_handle_key_char {
    use super::*;
    use pretty_assertions::assert_eq;

    #[rstest]
    fn test_refresh_key(
      #[values(ActiveLidarrBlock::TrackDetails, ActiveLidarrBlock::TrackHistory)]
      active_lidarr_block: ActiveLidarrBlock,
    ) {
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack(active_lidarr_block.into());
      app.is_routing = false;

      TrackDetailsHandler::new(
        DEFAULT_KEYBINDINGS.refresh.key,
        &mut app,
        active_lidarr_block,
        None,
      )
      .handle();

      assert_navigation_pushed!(app, active_lidarr_block.into());
      assert!(app.is_routing);
    }

    #[rstest]
    fn test_refresh_key_no_op_when_not_ready(
      #[values(ActiveLidarrBlock::TrackDetails, ActiveLidarrBlock::TrackHistory)]
      active_lidarr_block: ActiveLidarrBlock,
    ) {
      let mut app = App::test_default_fully_populated();
      app.is_loading = true;
      app.push_navigation_stack(active_lidarr_block.into());
      app.is_routing = false;

      TrackDetailsHandler::new(
        DEFAULT_KEYBINDINGS.refresh.key,
        &mut app,
        active_lidarr_block,
        None,
      )
      .handle();

      assert_eq!(app.get_current_route(), active_lidarr_block.into());
      assert!(!app.is_routing);
    }
  }

  #[test]
  fn test_track_details_handler_accepts() {
    ActiveLidarrBlock::iter().for_each(|active_lidarr_block| {
      if TRACK_DETAILS_BLOCKS.contains(&active_lidarr_block) {
        assert!(TrackDetailsHandler::accepts(active_lidarr_block));
      } else {
        assert!(!TrackDetailsHandler::accepts(active_lidarr_block));
      }
    });
  }

  #[rstest]
  fn test_track_details_handler_ignore_special_keys(
    #[values(true, false)] ignore_special_keys_for_textbox_input: bool,
  ) {
    let mut app = App::test_default();
    app.ignore_special_keys_for_textbox_input = ignore_special_keys_for_textbox_input;
    let handler = TrackDetailsHandler::new(
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
  fn test_track_details_handler_is_not_ready_when_loading() {
    let mut app = App::test_default();
    app.push_navigation_stack(ActiveLidarrBlock::TrackDetails.into());
    app.is_loading = true;

    let handler = TrackDetailsHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveLidarrBlock::TrackDetails,
      None,
    );

    assert!(!handler.is_ready());
  }

  #[test]
  fn test_track_details_handler_is_not_ready_when_album_details_modal_is_empty() {
    let mut app = App::test_default();
    app.push_navigation_stack(ActiveLidarrBlock::TrackDetails.into());
    app.is_loading = false;

    let handler = TrackDetailsHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveLidarrBlock::TrackDetails,
      None,
    );

    assert!(!handler.is_ready());
  }

  #[test]
  fn test_track_details_handler_is_not_ready_when_track_details_modal_is_empty() {
    let mut app = App::test_default_fully_populated();
    app
      .data
      .lidarr_data
      .album_details_modal
      .as_mut()
      .unwrap()
      .track_details_modal = None;
    app.push_navigation_stack(ActiveLidarrBlock::TrackDetails.into());
    app.is_loading = false;

    let handler = TrackDetailsHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveLidarrBlock::TrackDetails,
      None,
    );

    assert!(!handler.is_ready());
  }

  #[test]
  fn test_track_details_handler_is_not_ready_when_track_details_is_empty() {
    let mut app = App::test_default_fully_populated();
    app
      .data
      .lidarr_data
      .album_details_modal
      .as_mut()
      .unwrap()
      .track_details_modal
      .as_mut()
      .unwrap()
      .track_details = ScrollableText::default();
    app.push_navigation_stack(ActiveLidarrBlock::TrackDetails.into());
    app.is_loading = false;

    let handler = TrackDetailsHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveLidarrBlock::TrackDetails,
      None,
    );

    assert!(!handler.is_ready());
  }

  #[test]
  fn test_track_details_handler_is_not_ready_when_track_history_table_is_empty() {
    let mut app = App::test_default_fully_populated();
    app
      .data
      .lidarr_data
      .album_details_modal
      .as_mut()
      .unwrap()
      .track_details_modal
      .as_mut()
      .unwrap()
      .track_history = StatefulTable::default();
    app.push_navigation_stack(ActiveLidarrBlock::TrackHistory.into());
    app.is_loading = false;

    let handler = TrackDetailsHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveLidarrBlock::TrackHistory,
      None,
    );

    assert!(!handler.is_ready());
  }

  #[rstest]
  fn test_track_details_handler_is_ready(
    #[values(
			ActiveLidarrBlock::TrackDetails,
			ActiveLidarrBlock::TrackHistory,
		)]
		active_lidarr_block: ActiveLidarrBlock,
  ) {
    let mut app = App::test_default_fully_populated();
    app.push_navigation_stack(active_lidarr_block.into());
    app.is_loading = false;

    let handler = TrackDetailsHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      active_lidarr_block,
      None,
    );

    assert!(handler.is_ready());
  }
}
