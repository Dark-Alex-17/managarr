#[cfg(test)]
mod tests {
  use crate::app::App;
  use crate::app::key_binding::DEFAULT_KEYBINDINGS;
  use crate::assert_modal_absent;
  use crate::assert_navigation_pushed;
  use crate::handlers::KeyEventHandler;
  use crate::handlers::lidarr_handlers::library::album_details_handler::{
    AlbumDetailsHandler, releases_sorting_options,
  };
  use crate::models::HorizontallyScrollableText;
  use crate::models::lidarr_models::{LidarrRelease, LidarrReleaseDownloadBody};
  use crate::models::servarr_data::lidarr::lidarr_data::{ALBUM_DETAILS_BLOCKS, ActiveLidarrBlock};
  use crate::models::servarr_data::lidarr::modals::AlbumDetailsModal;
  use crate::models::servarr_models::{Quality, QualityWrapper};
  use pretty_assertions::{assert_eq, assert_str_eq};
  use rstest::rstest;
  use serde_json::Number;
  use std::cmp::Ordering;
  use strum::IntoEnumIterator;

  mod test_handle_delete {
    use super::*;
    use crate::event::Key;
    use pretty_assertions::assert_eq;

    const DELETE_KEY: Key = DEFAULT_KEYBINDINGS.delete.key;

    #[test]
    fn test_delete_track_prompt() {
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack(ActiveLidarrBlock::AlbumDetails.into());

      AlbumDetailsHandler::new(DELETE_KEY, &mut app, ActiveLidarrBlock::AlbumDetails, None)
        .handle();

      assert_navigation_pushed!(app, ActiveLidarrBlock::DeleteTrackFilePrompt.into());
    }

    #[test]
    fn test_delete_track_prompt_no_op_when_not_ready() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveLidarrBlock::AlbumDetails.into());
      app.is_loading = true;

      AlbumDetailsHandler::new(DELETE_KEY, &mut app, ActiveLidarrBlock::AlbumDetails, None)
        .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveLidarrBlock::AlbumDetails.into()
      );
    }
  }

  mod test_handle_left_right_actions {
    use super::*;
    use crate::event::Key;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    #[rstest]
    fn test_left_right_prompt_toggle(
      #[values(
        ActiveLidarrBlock::AutomaticallySearchAlbumPrompt,
        ActiveLidarrBlock::ManualAlbumSearchConfirmPrompt,
        ActiveLidarrBlock::DeleteTrackFilePrompt
      )]
      active_lidarr_block: ActiveLidarrBlock,
      #[values(Key::Left, Key::Right)] key: Key,
    ) {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveLidarrBlock::AlbumDetails.into());

      AlbumDetailsHandler::new(key, &mut app, active_lidarr_block, None).handle();

      assert!(app.data.lidarr_data.prompt_confirm);

      AlbumDetailsHandler::new(key, &mut app, active_lidarr_block, None).handle();

      assert!(!app.data.lidarr_data.prompt_confirm);
    }

    #[rstest]
    #[case(ActiveLidarrBlock::AlbumDetails, ActiveLidarrBlock::AlbumHistory)]
    #[case(ActiveLidarrBlock::AlbumHistory, ActiveLidarrBlock::ManualAlbumSearch)]
    #[case(ActiveLidarrBlock::ManualAlbumSearch, ActiveLidarrBlock::AlbumDetails)]
    fn test_album_details_tabs_left_right_action(
      #[case] left_block: ActiveLidarrBlock,
      #[case] right_block: ActiveLidarrBlock,
      #[values(true, false)] is_ready: bool,
    ) {
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack(ActiveLidarrBlock::ArtistDetails.into());
      app.is_loading = is_ready;
      app.push_navigation_stack(right_block.into());
      app
        .data
        .lidarr_data
        .album_details_modal
        .as_mut()
        .unwrap()
        .album_details_tabs
        .index = app
        .data
        .lidarr_data
        .album_details_modal
        .as_ref()
        .unwrap()
        .album_details_tabs
        .tabs
        .iter()
        .position(|tab_route| tab_route.route == right_block.into())
        .unwrap_or_default();

      AlbumDetailsHandler::new(DEFAULT_KEYBINDINGS.left.key, &mut app, right_block, None).handle();

      assert_eq!(
        app.get_current_route(),
        app
          .data
          .lidarr_data
          .album_details_modal
          .as_ref()
          .unwrap()
          .album_details_tabs
          .get_active_route()
      );
      assert_navigation_pushed!(app, left_block.into());

      AlbumDetailsHandler::new(DEFAULT_KEYBINDINGS.right.key, &mut app, left_block, None).handle();

      assert_eq!(
        app.get_current_route(),
        app
          .data
          .lidarr_data
          .album_details_modal
          .as_ref()
          .unwrap()
          .album_details_tabs
          .get_active_route()
      );
      assert_navigation_pushed!(app, right_block.into());
    }
  }

  mod test_handle_submit {
    use super::*;
    use crate::assert_navigation_popped;
    use crate::event::Key;
    use crate::models::stateful_table::StatefulTable;
    use crate::network::lidarr_network::LidarrEvent;
    use pretty_assertions::assert_eq;

    const SUBMIT_KEY: Key = DEFAULT_KEYBINDINGS.submit.key;

    // #[test]
    // fn test_album_details_submit() {
    // 	let mut app = App::test_default_fully_populated();
    // 	app.push_navigation_stack(ActiveLidarrBlock::AlbumDetails.into());
    //
    // 	AlbumDetailsHandler::new(SUBMIT_KEY, &mut app, ActiveLidarrBlock::AlbumDetails, None)
    // 		.handle();
    //
    // 	assert_navigation_pushed!(app, ActiveLidarrBlock::TrackDetails.into());
    // }

    // #[test]
    // fn test_album_details_submit_no_op_on_empty_tracks_table() {
    // 	let mut app = App::test_default_fully_populated();
    // 	app
    // 		.data
    // 		.lidarr_data
    // 		.album_details_modal
    // 		.as_mut()
    // 		.unwrap()
    // 		.tracks = StatefulTable::default();
    // 	app.push_navigation_stack(ActiveLidarrBlock::AlbumDetails.into());
    //
    // 	AlbumDetailsHandler::new(SUBMIT_KEY, &mut app, ActiveLidarrBlock::AlbumDetails, None)
    // 		.handle();
    //
    // 	assert_eq!(
    // 		app.get_current_route(),
    // 		ActiveLidarrBlock::AlbumDetails.into()
    // 	);
    // }

    #[test]
    fn test_album_details_submit_no_op_when_not_ready() {
      let mut app = App::test_default();
      app.is_loading = true;
      app.push_navigation_stack(ActiveLidarrBlock::AlbumDetails.into());

      AlbumDetailsHandler::new(SUBMIT_KEY, &mut app, ActiveLidarrBlock::AlbumDetails, None)
        .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveLidarrBlock::AlbumDetails.into()
      );
    }

    #[test]
    fn test_album_history_submit() {
      let mut app = App::test_default_fully_populated();

      AlbumDetailsHandler::new(SUBMIT_KEY, &mut app, ActiveLidarrBlock::AlbumHistory, None)
        .handle();

      assert_navigation_pushed!(app, ActiveLidarrBlock::AlbumHistoryDetails.into());
    }

    #[test]
    fn test_album_history_submit_no_op_when_album_history_is_empty() {
      let mut app = App::test_default_fully_populated();
      app
        .data
        .lidarr_data
        .album_details_modal
        .as_mut()
        .unwrap()
        .album_history = StatefulTable::default();
      app.push_navigation_stack(ActiveLidarrBlock::AlbumHistory.into());

      AlbumDetailsHandler::new(SUBMIT_KEY, &mut app, ActiveLidarrBlock::AlbumHistory, None)
        .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveLidarrBlock::AlbumHistory.into()
      );
    }

    #[test]
    fn test_album_history_submit_no_op_when_not_ready() {
      let mut app = App::test_default_fully_populated();
      app.is_loading = true;
      app.push_navigation_stack(ActiveLidarrBlock::AlbumHistory.into());

      AlbumDetailsHandler::new(SUBMIT_KEY, &mut app, ActiveLidarrBlock::AlbumHistory, None)
        .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveLidarrBlock::AlbumHistory.into()
      );
    }

    #[rstest]
    #[case(
      ActiveLidarrBlock::AutomaticallySearchAlbumPrompt,
      LidarrEvent::TriggerAutomaticAlbumSearch(1)
    )]
    #[case(
      ActiveLidarrBlock::DeleteTrackFilePrompt,
      LidarrEvent::DeleteTrackFile(1)
    )]
    fn test_album_details_prompt_confirm_submit(
      #[case] prompt_block: ActiveLidarrBlock,
      #[case] expected_action: LidarrEvent,
      #[values(ActiveLidarrBlock::AlbumDetails, ActiveLidarrBlock::AlbumHistory)]
      active_lidarr_block: ActiveLidarrBlock,
    ) {
      let mut app = App::test_default_fully_populated();
      app.data.lidarr_data.prompt_confirm = true;
      app.push_navigation_stack(active_lidarr_block.into());
      app.push_navigation_stack(prompt_block.into());

      AlbumDetailsHandler::new(SUBMIT_KEY, &mut app, prompt_block, None).handle();

      assert!(app.data.lidarr_data.prompt_confirm);
      assert_navigation_popped!(app, active_lidarr_block.into());
      assert_some_eq_x!(
        &app.data.lidarr_data.prompt_confirm_action,
        &expected_action
      );
    }

    #[test]
    fn test_album_details_manual_search_confirm_prompt_confirm_submit() {
      let mut app = App::test_default_fully_populated();
      app.data.lidarr_data.prompt_confirm = true;
      app.push_navigation_stack(ActiveLidarrBlock::ManualAlbumSearch.into());
      app.push_navigation_stack(ActiveLidarrBlock::ManualAlbumSearchConfirmPrompt.into());

      AlbumDetailsHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveLidarrBlock::ManualAlbumSearchConfirmPrompt,
        None,
      )
      .handle();

      assert!(app.data.lidarr_data.prompt_confirm);
      assert_navigation_popped!(app, ActiveLidarrBlock::ManualAlbumSearch.into());
      assert_some_eq_x!(
        &app.data.lidarr_data.prompt_confirm_action,
        &LidarrEvent::DownloadRelease(LidarrReleaseDownloadBody {
          guid: "1234".to_owned(),
          indexer_id: 2,
        })
      );
    }

    #[rstest]
    fn test_album_details_prompt_decline_submit(
      #[values(
        ActiveLidarrBlock::AutomaticallySearchAlbumPrompt,
        ActiveLidarrBlock::DeleteTrackFilePrompt,
        ActiveLidarrBlock::ManualAlbumSearchConfirmPrompt
      )]
      prompt_block: ActiveLidarrBlock,
    ) {
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack(ActiveLidarrBlock::AlbumDetails.into());
      app.push_navigation_stack(prompt_block.into());

      AlbumDetailsHandler::new(SUBMIT_KEY, &mut app, prompt_block, None).handle();

      assert!(!app.data.lidarr_data.prompt_confirm);
      assert_navigation_popped!(app, ActiveLidarrBlock::AlbumDetails.into());
      assert_none!(app.data.lidarr_data.prompt_confirm_action);
    }

    #[test]
    fn test_manual_album_search_submit() {
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack(ActiveLidarrBlock::ManualAlbumSearch.into());

      AlbumDetailsHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveLidarrBlock::ManualAlbumSearch,
        None,
      )
      .handle();

      assert_navigation_pushed!(
        app,
        ActiveLidarrBlock::ManualAlbumSearchConfirmPrompt.into()
      );
    }

    #[test]
    fn test_manual_album_search_submit_no_op_when_not_ready() {
      let mut app = App::test_default_fully_populated();
      app.is_loading = true;
      app.push_navigation_stack(ActiveLidarrBlock::ManualAlbumSearch.into());

      AlbumDetailsHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveLidarrBlock::ManualAlbumSearch,
        None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveLidarrBlock::ManualAlbumSearch.into()
      );
    }
  }

  mod test_handle_esc {
    use super::*;
    use crate::assert_navigation_popped;
    use crate::event::Key;
    use crate::models::lidarr_models::LidarrHistoryItem;
    use crate::models::stateful_table::StatefulTable;
    use pretty_assertions::assert_eq;
    use ratatui::widgets::TableState;

    const ESC_KEY: Key = DEFAULT_KEYBINDINGS.esc.key;

    #[test]
    fn test_album_history_details_block_esc() {
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack(ActiveLidarrBlock::AlbumHistory.into());
      app.push_navigation_stack(ActiveLidarrBlock::AlbumHistoryDetails.into());

      AlbumDetailsHandler::new(
        ESC_KEY,
        &mut app,
        ActiveLidarrBlock::AlbumHistoryDetails,
        None,
      )
      .handle();

      assert_navigation_popped!(app, ActiveLidarrBlock::AlbumHistory.into());
    }

    #[rstest]
    fn test_album_details_prompts_esc(
      #[values(
        ActiveLidarrBlock::AutomaticallySearchAlbumPrompt,
        ActiveLidarrBlock::DeleteTrackFilePrompt,
        ActiveLidarrBlock::ManualAlbumSearchConfirmPrompt
      )]
      prompt_block: ActiveLidarrBlock,
      #[values(true, false)] is_ready: bool,
    ) {
      let mut app = App::test_default_fully_populated();
      app.is_loading = is_ready;
      app.data.lidarr_data.prompt_confirm = true;
      app.push_navigation_stack(ActiveLidarrBlock::AlbumDetails.into());
      app.push_navigation_stack(prompt_block.into());

      AlbumDetailsHandler::new(ESC_KEY, &mut app, prompt_block, None).handle();

      assert!(!app.data.lidarr_data.prompt_confirm);
      assert_navigation_popped!(app, ActiveLidarrBlock::AlbumDetails.into());
    }

    #[test]
    fn test_album_history_esc_resets_filter_if_one_is_set_instead_of_closing_the_window() {
      let mut app = App::test_default_fully_populated();
      let mut album_history = StatefulTable {
        filter: Some("Test".into()),
        filtered_items: Some(vec![LidarrHistoryItem::default()]),
        filtered_state: Some(TableState::default()),
        ..StatefulTable::default()
      };
      album_history.set_items(vec![LidarrHistoryItem::default()]);
      app
        .data
        .lidarr_data
        .album_details_modal
        .as_mut()
        .unwrap()
        .album_history = album_history;
      app.push_navigation_stack(ActiveLidarrBlock::ArtistDetails.into());
      app.push_navigation_stack(ActiveLidarrBlock::AlbumHistory.into());

      AlbumDetailsHandler::new(ESC_KEY, &mut app, ActiveLidarrBlock::AlbumHistory, None).handle();

      assert_eq!(
        app.get_current_route(),
        ActiveLidarrBlock::AlbumHistory.into()
      );
      assert_none!(
        app
          .data
          .lidarr_data
          .album_details_modal
          .as_ref()
          .unwrap()
          .album_history
          .filter
      );
      assert_none!(
        app
          .data
          .lidarr_data
          .album_details_modal
          .as_ref()
          .unwrap()
          .album_history
          .filtered_items
      );
      assert_none!(
        app
          .data
          .lidarr_data
          .album_details_modal
          .as_ref()
          .unwrap()
          .album_history
          .filtered_state
      );
    }

    #[rstest]
    fn test_album_details_tabs_esc(
      #[values(
        ActiveLidarrBlock::AlbumDetails,
        ActiveLidarrBlock::AlbumHistory,
        ActiveLidarrBlock::ManualAlbumSearch
      )]
      active_lidarr_block: ActiveLidarrBlock,
    ) {
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack(ActiveLidarrBlock::ArtistDetails.into());
      app.push_navigation_stack(active_lidarr_block.into());

      AlbumDetailsHandler::new(ESC_KEY, &mut app, active_lidarr_block, None).handle();

      assert_navigation_popped!(app, ActiveLidarrBlock::ArtistDetails.into());
      assert_modal_absent!(app.data.lidarr_data.album_details_modal);
    }
  }

  mod test_handle_key_char {
    use super::*;
    use crate::assert_navigation_popped;
    use crate::network::lidarr_network::LidarrEvent;
    use pretty_assertions::assert_eq;

    #[rstest]
    fn test_auto_search_key(
      #[values(
        ActiveLidarrBlock::AlbumDetails,
        ActiveLidarrBlock::AlbumHistory,
        ActiveLidarrBlock::ManualAlbumSearch
      )]
      active_lidarr_block: ActiveLidarrBlock,
    ) {
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack(active_lidarr_block.into());

      AlbumDetailsHandler::new(
        DEFAULT_KEYBINDINGS.auto_search.key,
        &mut app,
        active_lidarr_block,
        None,
      )
      .handle();

      assert_navigation_pushed!(
        app,
        ActiveLidarrBlock::AutomaticallySearchAlbumPrompt.into()
      );
    }

    #[rstest]
    fn test_auto_search_key_no_op_when_not_ready(
      #[values(
        ActiveLidarrBlock::AlbumDetails,
        ActiveLidarrBlock::AlbumHistory,
        ActiveLidarrBlock::ManualAlbumSearch
      )]
      active_lidarr_block: ActiveLidarrBlock,
    ) {
      let mut app = App::test_default();
      app.is_loading = true;
      app.push_navigation_stack(active_lidarr_block.into());

      AlbumDetailsHandler::new(
        DEFAULT_KEYBINDINGS.auto_search.key,
        &mut app,
        active_lidarr_block,
        None,
      )
      .handle();

      assert_eq!(app.get_current_route(), active_lidarr_block.into());
    }

    #[rstest]
    fn test_refresh_key(
      #[values(
        ActiveLidarrBlock::AlbumDetails,
        ActiveLidarrBlock::AlbumHistory,
        ActiveLidarrBlock::ManualAlbumSearch
      )]
      active_lidarr_block: ActiveLidarrBlock,
    ) {
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack(active_lidarr_block.into());
      app.is_routing = false;

      AlbumDetailsHandler::new(
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
      #[values(
        ActiveLidarrBlock::AlbumDetails,
        ActiveLidarrBlock::AlbumHistory,
        ActiveLidarrBlock::ManualAlbumSearch
      )]
      active_lidarr_block: ActiveLidarrBlock,
    ) {
      let mut app = App::test_default_fully_populated();
      app.is_loading = true;
      app.push_navigation_stack(active_lidarr_block.into());
      app.is_routing = false;

      AlbumDetailsHandler::new(
        DEFAULT_KEYBINDINGS.refresh.key,
        &mut app,
        active_lidarr_block,
        None,
      )
      .handle();

      assert_eq!(app.get_current_route(), active_lidarr_block.into());
      assert!(!app.is_routing);
    }

    #[rstest]
    #[case(
      ActiveLidarrBlock::AutomaticallySearchAlbumPrompt,
      LidarrEvent::TriggerAutomaticAlbumSearch(1)
    )]
    #[case(
      ActiveLidarrBlock::DeleteTrackFilePrompt,
      LidarrEvent::DeleteTrackFile(1)
    )]
    fn test_album_details_prompt_confirm_confirm_key(
      #[case] prompt_block: ActiveLidarrBlock,
      #[case] expected_action: LidarrEvent,
      #[values(ActiveLidarrBlock::AlbumDetails, ActiveLidarrBlock::AlbumHistory)]
      active_lidarr_block: ActiveLidarrBlock,
    ) {
      let mut app = App::test_default_fully_populated();
      app.data.lidarr_data.prompt_confirm = true;
      app.push_navigation_stack(active_lidarr_block.into());
      app.push_navigation_stack(prompt_block.into());

      AlbumDetailsHandler::new(
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

    #[test]
    fn test_album_details_manual_search_confirm_prompt_confirm_confirm_key() {
      let mut app = App::test_default_fully_populated();
      app.data.lidarr_data.prompt_confirm = true;
      app.push_navigation_stack(ActiveLidarrBlock::ManualAlbumSearch.into());
      app.push_navigation_stack(ActiveLidarrBlock::ManualAlbumSearchConfirmPrompt.into());

      AlbumDetailsHandler::new(
        DEFAULT_KEYBINDINGS.confirm.key,
        &mut app,
        ActiveLidarrBlock::ManualAlbumSearchConfirmPrompt,
        None,
      )
      .handle();

      assert!(app.data.lidarr_data.prompt_confirm);
      assert_navigation_popped!(app, ActiveLidarrBlock::ManualAlbumSearch.into());
      assert_some_eq_x!(
        &app.data.lidarr_data.prompt_confirm_action,
        &LidarrEvent::DownloadRelease(LidarrReleaseDownloadBody {
          guid: "1234".to_owned(),
          indexer_id: 2,
        })
      );
    }
  }

  #[test]
  fn test_album_details_handler_accepts() {
    ActiveLidarrBlock::iter().for_each(|active_lidarr_block| {
      if ALBUM_DETAILS_BLOCKS.contains(&active_lidarr_block) {
        assert!(AlbumDetailsHandler::accepts(active_lidarr_block));
      } else {
        assert!(!AlbumDetailsHandler::accepts(active_lidarr_block));
      }
    });
  }

  #[rstest]
  fn test_album_details_handler_ignore_special_keys(
    #[values(true, false)] ignore_special_keys_for_textbox_input: bool,
  ) {
    let mut app = App::test_default();
    app.ignore_special_keys_for_textbox_input = ignore_special_keys_for_textbox_input;
    let handler = AlbumDetailsHandler::new(
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
  fn test_extract_track_file_id() {
    let mut app = App::test_default_fully_populated();

    let track_file_id = AlbumDetailsHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveLidarrBlock::AlbumDetails,
      None,
    )
    .extract_track_file_id();

    assert_eq!(track_file_id, 1);
  }

  #[test]
  #[should_panic(expected = "Album details have not been loaded")]
  fn test_extract_track_file_id_empty_album_details_modal_panics() {
    let mut app = App::test_default();

    AlbumDetailsHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveLidarrBlock::AlbumDetails,
      None,
    )
    .extract_track_file_id();
  }

  #[test]
  fn test_extract_album_id() {
    let mut app = App::test_default_fully_populated();

    let track_file_id = AlbumDetailsHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveLidarrBlock::AlbumDetails,
      None,
    )
    .extract_album_id();

    assert_eq!(track_file_id, 1);
  }

  #[test]
  fn test_album_details_handler_is_not_ready_when_loading() {
    let mut app = App::test_default();
    app.push_navigation_stack(ActiveLidarrBlock::AlbumDetails.into());
    app.is_loading = true;

    let handler = AlbumDetailsHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveLidarrBlock::AlbumDetails,
      None,
    );

    assert!(!handler.is_ready());
  }

  #[test]
  fn test_album_details_handler_is_not_ready_when_not_loading_and_album_details_is_none() {
    let mut app = App::test_default();
    app.push_navigation_stack(ActiveLidarrBlock::AlbumDetails.into());

    let handler = AlbumDetailsHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveLidarrBlock::AlbumDetails,
      None,
    );

    assert!(!handler.is_ready());
  }

  #[test]
  fn test_album_details_handler_is_not_ready_when_not_loading_and_tracks_table_is_empty() {
    let mut app = App::test_default();
    app.data.lidarr_data.album_details_modal = Some(AlbumDetailsModal::default());
    app.push_navigation_stack(ActiveLidarrBlock::AlbumDetails.into());

    let handler = AlbumDetailsHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveLidarrBlock::AlbumDetails,
      None,
    );

    assert!(!handler.is_ready());
  }

  #[test]
  fn test_album_details_handler_is_not_ready_when_not_loading_and_history_table_is_empty() {
    let mut app = App::test_default();
    app.data.lidarr_data.album_details_modal = Some(AlbumDetailsModal::default());
    app.push_navigation_stack(ActiveLidarrBlock::AlbumHistory.into());

    let handler = AlbumDetailsHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveLidarrBlock::AlbumHistory,
      None,
    );

    assert!(!handler.is_ready());
  }

  #[test]
  fn test_album_details_handler_is_not_ready_when_not_loading_and_releases_table_is_empty() {
    let mut app = App::test_default();
    app.data.lidarr_data.album_details_modal = Some(AlbumDetailsModal::default());
    app.push_navigation_stack(ActiveLidarrBlock::ManualAlbumSearch.into());

    let handler = AlbumDetailsHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveLidarrBlock::ManualAlbumSearch,
      None,
    );

    assert!(!handler.is_ready());
  }

  #[rstest]
  fn test_album_details_handler_is_ready_when_not_loading_and_album_details_modal_is_populated(
    #[values(
      ActiveLidarrBlock::AlbumDetails,
      ActiveLidarrBlock::AlbumHistory,
      ActiveLidarrBlock::ManualAlbumSearch
    )]
    active_lidarr_block: ActiveLidarrBlock,
  ) {
    let mut app = App::test_default_fully_populated();
    app.push_navigation_stack(ActiveLidarrBlock::ArtistDetails.into());
    app.push_navigation_stack(active_lidarr_block.into());

    let handler = AlbumDetailsHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      active_lidarr_block,
      None,
    );

    assert!(handler.is_ready());
  }

  #[test]
  fn test_releases_sorting_options_source() {
    let expected_cmp_fn: fn(&LidarrRelease, &LidarrRelease) -> Ordering =
      |a, b| a.protocol.cmp(&b.protocol);
    let mut expected_releases_vec = release_vec();
    expected_releases_vec.sort_by(expected_cmp_fn);

    let sort_option = releases_sorting_options()[0].clone();
    let mut sorted_releases_vec = release_vec();
    sorted_releases_vec.sort_by(sort_option.cmp_fn.unwrap());

    assert_eq!(sorted_releases_vec, expected_releases_vec);
    assert_str_eq!(sort_option.name, "Source");
  }

  #[test]
  fn test_releases_sorting_options_age() {
    let expected_cmp_fn: fn(&LidarrRelease, &LidarrRelease) -> Ordering = |a, b| a.age.cmp(&b.age);
    let mut expected_releases_vec = release_vec();
    expected_releases_vec.sort_by(expected_cmp_fn);

    let sort_option = releases_sorting_options()[1].clone();
    let mut sorted_releases_vec = release_vec();
    sorted_releases_vec.sort_by(sort_option.cmp_fn.unwrap());

    assert_eq!(sorted_releases_vec, expected_releases_vec);
    assert_str_eq!(sort_option.name, "Age");
  }

  #[test]
  fn test_releases_sorting_options_rejected() {
    let expected_cmp_fn: fn(&LidarrRelease, &LidarrRelease) -> Ordering =
      |a, b| a.rejected.cmp(&b.rejected);
    let mut expected_releases_vec = release_vec();
    expected_releases_vec.sort_by(expected_cmp_fn);

    let sort_option = releases_sorting_options()[2].clone();
    let mut sorted_releases_vec = release_vec();
    sorted_releases_vec.sort_by(sort_option.cmp_fn.unwrap());

    assert_eq!(sorted_releases_vec, expected_releases_vec);
    assert_str_eq!(sort_option.name, "Rejected");
  }

  #[test]
  fn test_releases_sorting_options_title() {
    let expected_cmp_fn: fn(&LidarrRelease, &LidarrRelease) -> Ordering = |a, b| {
      a.title
        .text
        .to_lowercase()
        .cmp(&b.title.text.to_lowercase())
    };
    let mut expected_releases_vec = release_vec();
    expected_releases_vec.sort_by(expected_cmp_fn);

    let sort_option = releases_sorting_options()[3].clone();
    let mut sorted_releases_vec = release_vec();
    sorted_releases_vec.sort_by(sort_option.cmp_fn.unwrap());

    assert_eq!(sorted_releases_vec, expected_releases_vec);
    assert_str_eq!(sort_option.name, "Title");
  }

  #[test]
  fn test_releases_sorting_options_indexer() {
    let expected_cmp_fn: fn(&LidarrRelease, &LidarrRelease) -> Ordering =
      |a, b| a.indexer.to_lowercase().cmp(&b.indexer.to_lowercase());
    let mut expected_releases_vec = release_vec();
    expected_releases_vec.sort_by(expected_cmp_fn);

    let sort_option = releases_sorting_options()[4].clone();
    let mut sorted_releases_vec = release_vec();
    sorted_releases_vec.sort_by(sort_option.cmp_fn.unwrap());

    assert_eq!(sorted_releases_vec, expected_releases_vec);
    assert_str_eq!(sort_option.name, "Indexer");
  }

  #[test]
  fn test_releases_sorting_options_size() {
    let expected_cmp_fn: fn(&LidarrRelease, &LidarrRelease) -> Ordering =
      |a, b| a.size.cmp(&b.size);
    let mut expected_releases_vec = release_vec();
    expected_releases_vec.sort_by(expected_cmp_fn);

    let sort_option = releases_sorting_options()[5].clone();
    let mut sorted_releases_vec = release_vec();
    sorted_releases_vec.sort_by(sort_option.cmp_fn.unwrap());

    assert_eq!(sorted_releases_vec, expected_releases_vec);
    assert_str_eq!(sort_option.name, "Size");
  }

  #[test]
  fn test_releases_sorting_options_peers() {
    let expected_cmp_fn: fn(&LidarrRelease, &LidarrRelease) -> Ordering = |a, b| {
      let default_number = Number::from(i64::MAX);
      let seeder_a = a
        .seeders
        .as_ref()
        .unwrap_or(&default_number)
        .as_u64()
        .unwrap();
      let seeder_b = b
        .seeders
        .as_ref()
        .unwrap_or(&default_number)
        .as_u64()
        .unwrap();

      seeder_a.cmp(&seeder_b)
    };
    let mut expected_releases_vec = release_vec();
    expected_releases_vec.sort_by(expected_cmp_fn);

    let sort_option = releases_sorting_options()[6].clone();
    let mut sorted_releases_vec = release_vec();
    sorted_releases_vec.sort_by(sort_option.cmp_fn.unwrap());

    assert_eq!(sorted_releases_vec, expected_releases_vec);
    assert_str_eq!(sort_option.name, "Peers");
  }

  #[test]
  fn test_releases_sorting_options_quality() {
    let expected_cmp_fn: fn(&LidarrRelease, &LidarrRelease) -> Ordering =
      |a, b| a.quality.cmp(&b.quality);
    let mut expected_releases_vec = release_vec();
    expected_releases_vec.sort_by(expected_cmp_fn);

    let sort_option = releases_sorting_options()[7].clone();
    let mut sorted_releases_vec = release_vec();
    sorted_releases_vec.sort_by(sort_option.cmp_fn.unwrap());

    assert_eq!(sorted_releases_vec, expected_releases_vec);
    assert_str_eq!(sort_option.name, "Quality");
  }

  fn release_vec() -> Vec<LidarrRelease> {
    let release_a = LidarrRelease {
      protocol: "Protocol A".to_owned(),
      age: 1,
      title: HorizontallyScrollableText::from("Title A"),
      indexer: "Indexer A".to_owned(),
      size: 1,
      rejected: true,
      seeders: Some(Number::from(1)),
      quality: QualityWrapper {
        quality: Quality {
          name: "Quality A".to_owned(),
        },
      },
      ..LidarrRelease::default()
    };
    let release_b = LidarrRelease {
      protocol: "Protocol B".to_owned(),
      age: 2,
      title: HorizontallyScrollableText::from("title B"),
      indexer: "indexer B".to_owned(),
      size: 2,
      rejected: false,
      seeders: Some(Number::from(2)),
      quality: QualityWrapper {
        quality: Quality {
          name: "Quality B".to_owned(),
        },
      },
      ..LidarrRelease::default()
    };
    let release_c = LidarrRelease {
      protocol: "Protocol C".to_owned(),
      age: 3,
      title: HorizontallyScrollableText::from("Title C"),
      indexer: "Indexer C".to_owned(),
      size: 3,
      rejected: false,
      seeders: None,
      quality: QualityWrapper {
        quality: Quality {
          name: "Quality C".to_owned(),
        },
      },
      ..LidarrRelease::default()
    };

    vec![release_a, release_b, release_c]
  }
}
