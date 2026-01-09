#[cfg(test)]
mod tests {
  use pretty_assertions::assert_eq;
  use rstest::rstest;
  use strum::IntoEnumIterator;

  use crate::app::key_binding::DEFAULT_KEYBINDINGS;
  use crate::app::App;
  use crate::handlers::lidarr_handlers::library::artist_details_handler::ArtistDetailsHandler;
  use crate::handlers::KeyEventHandler;
  use crate::models::servarr_data::lidarr::lidarr_data::{
    ActiveLidarrBlock, ARTIST_DETAILS_BLOCKS,
  };

  mod test_handle_left_right_action {
    use rstest::rstest;

    use crate::app::App;
    use crate::event::Key;
    use crate::handlers::lidarr_handlers::library::artist_details_handler::ArtistDetailsHandler;
    use crate::handlers::KeyEventHandler;
    use crate::models::servarr_data::lidarr::lidarr_data::ActiveLidarrBlock;

    #[rstest]
    fn test_left_right_prompt_toggle(
      #[values(
      ActiveLidarrBlock::UpdateAndScanArtistPrompt,
      ActiveLidarrBlock::AutomaticallySearchArtistPrompt,
      )] active_lidarr_block: ActiveLidarrBlock,
      #[values(Key::Left, Key::Right)] key: Key,
    ) {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveLidarrBlock::ArtistDetails.into());
      app.push_navigation_stack(active_lidarr_block.into());

      ArtistDetailsHandler::new(
        key,
        &mut app,
        active_lidarr_block,
        None,
      )
      .handle();

      assert!(app.data.lidarr_data.prompt_confirm);

      ArtistDetailsHandler::new(
        key,
        &mut app,
        active_lidarr_block,
        None,
      )
      .handle();

      assert!(!app.data.lidarr_data.prompt_confirm);
    }
  }

  mod test_handle_submit {
    use rstest::rstest;
    use crate::app::key_binding::DEFAULT_KEYBINDINGS;
    use crate::app::App;
    use crate::assert_navigation_popped;
    use crate::event::Key;
    use crate::handlers::lidarr_handlers::library::artist_details_handler::ArtistDetailsHandler;
    use crate::handlers::KeyEventHandler;
    use crate::models::servarr_data::lidarr::lidarr_data::ActiveLidarrBlock;
    use crate::network::lidarr_network::lidarr_network_test_utils::test_utils::artist;
    use crate::network::lidarr_network::LidarrEvent;

    const SUBMIT_KEY: Key = DEFAULT_KEYBINDINGS.submit.key;

    #[rstest]
    #[case(ActiveLidarrBlock::AutomaticallySearchArtistPrompt, LidarrEvent::TriggerAutomaticArtistSearch(1))]
    #[case(ActiveLidarrBlock::UpdateAndScanArtistPrompt, LidarrEvent::UpdateAndScanArtist(1))]
    fn test_artist_details_prompt_confirm_submit(
      #[case] prompt_block: ActiveLidarrBlock,
      #[case] expected_action: LidarrEvent
    ) {
      let mut app = App::test_default();
      app.data.lidarr_data.prompt_confirm = true;
      app.data.lidarr_data.artists.set_items(vec![artist()]);
      app.push_navigation_stack(ActiveLidarrBlock::ArtistDetails.into());
      app.push_navigation_stack(prompt_block.into());

      ArtistDetailsHandler::new(
        SUBMIT_KEY,
        &mut app,
        prompt_block,
        None,
      )
      .handle();

      assert!(app.data.lidarr_data.prompt_confirm);
      assert_navigation_popped!(app, ActiveLidarrBlock::ArtistDetails.into());
      assert_some_eq_x!(
        &app.data.lidarr_data.prompt_confirm_action,
        &expected_action
      );
    }

    #[rstest]
    fn test_artist_details_prompt_decline_submit(
      #[values(
      ActiveLidarrBlock::AutomaticallySearchArtistPrompt,
      ActiveLidarrBlock::UpdateAndScanArtistPrompt,
      )] prompt_block: ActiveLidarrBlock
    ) {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveLidarrBlock::ArtistDetails.into());
      app.push_navigation_stack(prompt_block.into());

      ArtistDetailsHandler::new(
        SUBMIT_KEY,
        &mut app,
        prompt_block,
        None,
      )
      .handle();

      assert!(!app.data.lidarr_data.prompt_confirm);
      assert_navigation_popped!(app, ActiveLidarrBlock::ArtistDetails.into());
      assert_none!(app.data.lidarr_data.prompt_confirm_action);
    }
  }

  mod test_handle_esc {
    use rstest::rstest;
    use crate::app::key_binding::DEFAULT_KEYBINDINGS;
    use crate::app::App;
    use crate::assert_navigation_popped;
    use crate::event::Key;
    use crate::handlers::lidarr_handlers::library::artist_details_handler::ArtistDetailsHandler;
    use crate::handlers::KeyEventHandler;
    use crate::models::servarr_data::lidarr::lidarr_data::ActiveLidarrBlock;

    const ESC_KEY: Key = DEFAULT_KEYBINDINGS.esc.key;

    #[rstest]
    fn test_artist_details_esc(
      #[values(
        ActiveLidarrBlock::AutomaticallySearchArtistPrompt,
        ActiveLidarrBlock::UpdateAndScanArtistPrompt
      )] prompt_block: ActiveLidarrBlock,
      #[values(true, false)] is_ready: bool
    ) {
      let mut app = App::test_default();
      app.is_loading = is_ready;
      app.data.lidarr_data.prompt_confirm = true;
      app.push_navigation_stack(ActiveLidarrBlock::ArtistDetails.into());
      app.push_navigation_stack(prompt_block.into());

      ArtistDetailsHandler::new(ESC_KEY, &mut app, prompt_block, None).handle();

      assert!(!app.data.lidarr_data.prompt_confirm);
      assert_navigation_popped!(app, ActiveLidarrBlock::ArtistDetails.into());
    }
  }

  mod test_handle_char_key_event {
    use pretty_assertions::assert_eq;
    use rstest::rstest;
    use crate::app::key_binding::DEFAULT_KEYBINDINGS;
    use crate::app::App;
    use crate::{assert_modal_absent, assert_modal_present, assert_navigation_popped};
    use crate::assert_navigation_pushed;
    use crate::handlers::lidarr_handlers::library::artist_details_handler::ArtistDetailsHandler;
    use crate::handlers::KeyEventHandler;
    use crate::models::lidarr_models::{Artist};
    use crate::models::servarr_data::lidarr::lidarr_data::{ActiveLidarrBlock, EDIT_ARTIST_SELECTION_BLOCKS};
    use crate::network::lidarr_network::LidarrEvent;

    #[rstest]
    fn test_artist_details_edit_key(
      #[values(ActiveLidarrBlock::ArtistDetails)] active_lidarr_block: ActiveLidarrBlock
    ) {
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack(ActiveLidarrBlock::ArtistDetails.into());
      app.push_navigation_stack(active_lidarr_block.into());

      ArtistDetailsHandler::new(
        DEFAULT_KEYBINDINGS.edit.key,
        &mut app,
        active_lidarr_block,
        None,
      )
        .handle();

      assert_navigation_pushed!(
        app,
        (
          ActiveLidarrBlock::EditArtistPrompt,
          Some(ActiveLidarrBlock::ArtistDetails)
        )
          .into()
      );
      assert_modal_present!(app.data.lidarr_data.edit_artist_modal);
      assert!(app.data.lidarr_data.edit_artist_modal.is_some());
      assert_eq!(
        app.data.lidarr_data.selected_block.blocks,
        EDIT_ARTIST_SELECTION_BLOCKS
      );
    }

    #[rstest]
    fn test_artist_details_edit_key_no_op_when_not_ready(
      #[values(ActiveLidarrBlock::ArtistDetails)] active_lidarr_block: ActiveLidarrBlock
    ) {
      let mut app = App::test_default();
      app.is_loading = true;
      app.push_navigation_stack(ActiveLidarrBlock::ArtistDetails.into());
      app.push_navigation_stack(active_lidarr_block.into());

      ArtistDetailsHandler::new(
        DEFAULT_KEYBINDINGS.edit.key,
        &mut app,
        active_lidarr_block,
        None,
      )
        .handle();

      assert_eq!(app.get_current_route(), active_lidarr_block.into());
      assert_modal_absent!(app.data.lidarr_data.edit_artist_modal);
    }

    #[test]
    fn test_artist_details_toggle_monitoring_key() {
      let mut app = App::test_default_fully_populated();
      app.is_routing = false;
      app.push_navigation_stack(ActiveLidarrBlock::Artists.into());
      app.push_navigation_stack(ActiveLidarrBlock::ArtistDetails.into());

      ArtistDetailsHandler::new(
        DEFAULT_KEYBINDINGS.toggle_monitoring.key,
        &mut app,
        ActiveLidarrBlock::ArtistDetails,
        None,
      )
        .handle();

      assert_eq!(app.get_current_route(), ActiveLidarrBlock::ArtistDetails.into());
      assert!(app.data.lidarr_data.prompt_confirm);
      assert!(app.is_routing);
      assert_eq!(
        app.data.lidarr_data.prompt_confirm_action,
        Some(LidarrEvent::ToggleAlbumMonitoring(1))
      );
    }

    #[test]
    fn test_artist_details_toggle_monitoring_key_no_op_when_not_ready() {
      let mut app = App::test_default();
      app.is_loading = true;
      app.data.lidarr_data.prompt_confirm = false;
      app.push_navigation_stack(ActiveLidarrBlock::ArtistDetails.into());

      ArtistDetailsHandler::new(
        DEFAULT_KEYBINDINGS.toggle_monitoring.key,
        &mut app,
        ActiveLidarrBlock::ArtistDetails,
        None,
      )
        .handle();

      assert_eq!(app.get_current_route(), ActiveLidarrBlock::ArtistDetails.into());
      assert!(!app.data.lidarr_data.prompt_confirm);
      assert_none!(app.data.lidarr_data.prompt_confirm_action);
    }

    #[test]
    fn test_artist_details_toggle_monitoring_key_no_op_when_albums_empty() {
      let mut app = App::test_default();
      app.data.lidarr_data.artists.set_items(vec![Artist {
        id: 1,
        ..Artist::default()
      }]);
      app.push_navigation_stack(ActiveLidarrBlock::ArtistDetails.into());

      ArtistDetailsHandler::new(
        DEFAULT_KEYBINDINGS.toggle_monitoring.key,
        &mut app,
        ActiveLidarrBlock::ArtistDetails,
        None,
      )
        .handle();

      assert!(!app.data.lidarr_data.prompt_confirm);
      assert!(app.data.lidarr_data.prompt_confirm_action.is_none());
    }

    #[rstest]
    fn test_artist_details_auto_search_key(
      #[values(ActiveLidarrBlock::ArtistDetails)] active_lidarr_block: ActiveLidarrBlock
    ) {
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack(active_lidarr_block.into());

      ArtistDetailsHandler::new(
        DEFAULT_KEYBINDINGS.auto_search.key,
        &mut app,
        active_lidarr_block,
        None,
      )
        .handle();

      assert_navigation_pushed!(
        app,
        ActiveLidarrBlock::AutomaticallySearchArtistPrompt.into()
      );
    }

    #[rstest]
    fn test_artist_details_auto_search_key_no_op_when_not_ready(
      #[values(ActiveLidarrBlock::ArtistDetails)] active_lidarr_block: ActiveLidarrBlock
    ) {
      let mut app = App::test_default_fully_populated();
      app.is_loading = true;
      app.push_navigation_stack(active_lidarr_block.into());

      ArtistDetailsHandler::new(
        DEFAULT_KEYBINDINGS.auto_search.key,
        &mut app,
        active_lidarr_block,
        None,
      )
        .handle();

      assert_eq!(app.get_current_route(), active_lidarr_block.into());
    }

    #[rstest]
    fn test_artist_details_update_key(
      #[values(ActiveLidarrBlock::ArtistDetails)] active_lidarr_block: ActiveLidarrBlock
    ) {
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack(active_lidarr_block.into());

      ArtistDetailsHandler::new(
        DEFAULT_KEYBINDINGS.update.key,
        &mut app,
        active_lidarr_block,
        None,
      )
        .handle();

      assert_navigation_pushed!(app, ActiveLidarrBlock::UpdateAndScanArtistPrompt.into());
    }

    #[rstest]
    fn test_artist_details_update_key_no_op_when_not_ready(
      #[values(ActiveLidarrBlock::ArtistDetails)] active_lidarr_block: ActiveLidarrBlock
    ) {
      let mut app = App::test_default_fully_populated();
      app.is_loading = true;
      app.push_navigation_stack(active_lidarr_block.into());

      ArtistDetailsHandler::new(
        DEFAULT_KEYBINDINGS.update.key,
        &mut app,
        active_lidarr_block,
        None,
      )
        .handle();

      assert_eq!(app.get_current_route(), active_lidarr_block.into());
    }

    #[rstest]
    fn test_artist_details_refresh_key(
      #[values(ActiveLidarrBlock::ArtistDetails)] active_lidarr_block: ActiveLidarrBlock
    ) {
      let mut app = App::test_default_fully_populated();
      app.is_routing = false;
      app.push_navigation_stack(ActiveLidarrBlock::Artists.into());
      app.push_navigation_stack(active_lidarr_block.into());

      ArtistDetailsHandler::new(
        DEFAULT_KEYBINDINGS.refresh.key,
        &mut app,
        active_lidarr_block,
        None,
      )
      .handle();

      assert_navigation_pushed!(
        app,
        active_lidarr_block.into()
      );
      assert!(app.is_routing);
    }

    #[rstest]
    fn test_artist_details_refresh_key_no_op_when_not_ready(
      #[values(ActiveLidarrBlock::ArtistDetails)] active_lidarr_block: ActiveLidarrBlock
    ) {
      let mut app = App::test_default();
      app.is_loading = true;
      app.push_navigation_stack(active_lidarr_block.into());
      app.is_routing = false;

      ArtistDetailsHandler::new(
        DEFAULT_KEYBINDINGS.refresh.key,
        &mut app,
        active_lidarr_block,
        None,
      )
        .handle();

      assert_eq!(
        app.get_current_route(),
        active_lidarr_block.into()
      );
      assert!(!app.is_routing);
    }

    #[rstest]
    #[case(
      ActiveLidarrBlock::AutomaticallySearchArtistPrompt,
      LidarrEvent::TriggerAutomaticArtistSearch(1)
    )]
    #[case(
      ActiveLidarrBlock::UpdateAndScanArtistPrompt,
      LidarrEvent::UpdateAndScanArtist(1)
    )]
    fn test_artist_details_prompt_confirm_key(
      #[case] prompt_block: ActiveLidarrBlock,
      #[case] expected_action: LidarrEvent,
      #[values(ActiveLidarrBlock::ArtistDetails)]
      active_lidarr_block: ActiveLidarrBlock,
    ) {
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack(active_lidarr_block.into());
      app.push_navigation_stack(prompt_block.into());

      ArtistDetailsHandler::new(
        DEFAULT_KEYBINDINGS.confirm.key,
        &mut app,
        prompt_block,
        None,
      )
      .handle();

      assert!(app.data.lidarr_data.prompt_confirm);
      assert_navigation_popped!(app, active_lidarr_block.into());
      assert_some_eq_x!(
        &app.data.lidarr_data.prompt_confirm_action,
        &expected_action
      );
    }
  }

  #[test]
  fn test_artist_details_handler_accepts() {
    ActiveLidarrBlock::iter().for_each(|active_lidarr_block| {
      if ARTIST_DETAILS_BLOCKS.contains(&active_lidarr_block) {
        assert!(ArtistDetailsHandler::accepts(active_lidarr_block));
      } else {
        assert!(!ArtistDetailsHandler::accepts(active_lidarr_block));
      }
    });
  }

  #[test]
  fn test_extract_artist_id() {
    let mut app = App::test_default_fully_populated();

    let artist_id = ArtistDetailsHandler::new(DEFAULT_KEYBINDINGS.esc.key,
    &mut app, ActiveLidarrBlock::ArtistDetails, None).extract_artist_id();

    assert_eq!(artist_id, 1);
  }

  #[test]
  fn test_extract_album_id() {
    let mut app = App::test_default_fully_populated();

    let album_id = ArtistDetailsHandler::new(DEFAULT_KEYBINDINGS.esc.key,
                                              &mut app, ActiveLidarrBlock::ArtistDetails, None).extract_album_id();

    assert_eq!(album_id, 1);
  }

  #[rstest]
  fn test_artist_details_handler_ignore_special_keys(
    #[values(true, false)] ignore_special_keys_for_textbox_input: bool,
  ) {
    let mut app = App::test_default();
    app.ignore_special_keys_for_textbox_input = ignore_special_keys_for_textbox_input;

    let handler = ArtistDetailsHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveLidarrBlock::ArtistDetails,
      None,
    );

    assert_eq!(
      handler.ignore_special_keys(),
      ignore_special_keys_for_textbox_input
    );
  }

  #[test]
  fn test_artist_details_handler_is_not_ready_when_loading() {
    let mut app = App::test_default();
    app.push_navigation_stack(ActiveLidarrBlock::ArtistDetails.into());
    app.is_loading = true;

    let handler = ArtistDetailsHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveLidarrBlock::ArtistDetails,
      None,
    );

    assert!(!handler.is_ready());
  }

  #[test]
  fn test_artist_details_handler_is_ready_when_not_loading() {
    let mut app = App::test_default();
    app.push_navigation_stack(ActiveLidarrBlock::ArtistDetails.into());
    app.is_loading = false;

    let handler = ArtistDetailsHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveLidarrBlock::ArtistDetails,
      None,
    );

    assert!(handler.is_ready());
  }
}
