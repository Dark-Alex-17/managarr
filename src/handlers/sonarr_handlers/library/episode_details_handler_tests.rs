#[cfg(test)]
mod tests {
  use crate::app::key_binding::DEFAULT_KEYBINDINGS;
  use crate::app::App;
  use crate::handlers::sonarr_handlers::library::episode_details_handler::EpisodeDetailsHandler;
  use crate::handlers::KeyEventHandler;
  use crate::models::servarr_data::sonarr::modals::EpisodeDetailsModal;
  use crate::models::servarr_data::sonarr::sonarr_data::sonarr_test_utils::utils::create_test_sonarr_data;
  use crate::models::servarr_data::sonarr::sonarr_data::{
    ActiveSonarrBlock, EPISODE_DETAILS_BLOCKS,
  };
  use crate::models::sonarr_models::SonarrReleaseDownloadBody;
  use crate::models::stateful_table::StatefulTable;
  use rstest::rstest;
  use strum::IntoEnumIterator;

  mod test_handle_left_right_actions {
    use super::*;
    use crate::event::Key;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    #[rstest]
    fn test_left_right_prompt_toggle(
      #[values(
        ActiveSonarrBlock::AutomaticallySearchEpisodePrompt,
        ActiveSonarrBlock::ManualEpisodeSearchConfirmPrompt
      )]
      active_sonarr_block: ActiveSonarrBlock,
      #[values(Key::Left, Key::Right)] key: Key,
    ) {
      let mut app = App::default();
      app.push_navigation_stack(ActiveSonarrBlock::EpisodeDetails.into());

      EpisodeDetailsHandler::with(key, &mut app, active_sonarr_block, None).handle();

      assert!(app.data.sonarr_data.prompt_confirm);

      EpisodeDetailsHandler::with(key, &mut app, active_sonarr_block, None).handle();

      assert!(!app.data.sonarr_data.prompt_confirm);
    }

    #[rstest]
    #[case(ActiveSonarrBlock::EpisodeDetails, ActiveSonarrBlock::EpisodeHistory)]
    #[case(ActiveSonarrBlock::EpisodeHistory, ActiveSonarrBlock::EpisodeFile)]
    #[case(ActiveSonarrBlock::EpisodeFile, ActiveSonarrBlock::ManualEpisodeSearch)]
    #[case(
      ActiveSonarrBlock::ManualEpisodeSearch,
      ActiveSonarrBlock::EpisodeDetails
    )]
    fn test_episode_details_tabs_left_right_action(
      #[case] left_block: ActiveSonarrBlock,
      #[case] right_block: ActiveSonarrBlock,
      #[values(true, false)] is_ready: bool,
    ) {
      let mut app = App::default();
      app.data.sonarr_data = create_test_sonarr_data();
      app.push_navigation_stack(ActiveSonarrBlock::SeasonDetails.into());
      app.is_loading = is_ready;
      app.push_navigation_stack(right_block.into());
      app
        .data
        .sonarr_data
        .season_details_modal
        .as_mut()
        .unwrap()
        .episode_details_modal
        .as_mut()
        .unwrap()
        .episode_details_tabs
        .index = app
        .data
        .sonarr_data
        .season_details_modal
        .as_ref()
        .unwrap()
        .episode_details_modal
        .as_ref()
        .unwrap()
        .episode_details_tabs
        .tabs
        .iter()
        .position(|tab_route| tab_route.route == right_block.into())
        .unwrap_or_default();

      EpisodeDetailsHandler::with(DEFAULT_KEYBINDINGS.left.key, &mut app, right_block, None)
        .handle();

      assert_eq!(
        app.get_current_route(),
        app
          .data
          .sonarr_data
          .season_details_modal
          .as_ref()
          .unwrap()
          .episode_details_modal
          .as_ref()
          .unwrap()
          .episode_details_tabs
          .get_active_route()
      );
      assert_eq!(app.get_current_route(), left_block.into());

      EpisodeDetailsHandler::with(DEFAULT_KEYBINDINGS.right.key, &mut app, left_block, None)
        .handle();

      assert_eq!(
        app.get_current_route(),
        app
          .data
          .sonarr_data
          .season_details_modal
          .as_ref()
          .unwrap()
          .episode_details_modal
          .as_ref()
          .unwrap()
          .episode_details_tabs
          .get_active_route()
      );
      assert_eq!(app.get_current_route(), right_block.into());
    }
  }

  mod test_handle_submit {
    use super::*;
    use crate::event::Key;
    use crate::models::stateful_table::StatefulTable;
    use crate::network::sonarr_network::SonarrEvent;
    use pretty_assertions::assert_eq;

    const SUBMIT_KEY: Key = DEFAULT_KEYBINDINGS.submit.key;

    #[test]
    fn test_episode_history_submit() {
      let mut app = App::default();
      app.data.sonarr_data = create_test_sonarr_data();

      EpisodeDetailsHandler::with(
        SUBMIT_KEY,
        &mut app,
        ActiveSonarrBlock::EpisodeHistory,
        None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveSonarrBlock::EpisodeHistoryDetails.into()
      );
    }

    #[test]
    fn test_episode_history_submit_no_op_when_episode_history_is_empty() {
      let mut app = App::default();
      app.data.sonarr_data = create_test_sonarr_data();
      app
        .data
        .sonarr_data
        .season_details_modal
        .as_mut()
        .unwrap()
        .episode_details_modal
        .as_mut()
        .unwrap()
        .episode_history = StatefulTable::default();
      app.push_navigation_stack(ActiveSonarrBlock::EpisodeHistory.into());

      EpisodeDetailsHandler::with(
        SUBMIT_KEY,
        &mut app,
        ActiveSonarrBlock::EpisodeHistory,
        None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveSonarrBlock::EpisodeHistory.into()
      );
    }

    #[test]
    fn test_episode_history_submit_no_op_when_not_ready() {
      let mut app = App::default();
      app.data.sonarr_data = create_test_sonarr_data();
      app.is_loading = true;
      app.push_navigation_stack(ActiveSonarrBlock::EpisodeHistory.into());

      EpisodeDetailsHandler::with(
        SUBMIT_KEY,
        &mut app,
        ActiveSonarrBlock::EpisodeHistory,
        None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveSonarrBlock::EpisodeHistory.into()
      );
    }

    #[rstest]
    #[case(
      ActiveSonarrBlock::AutomaticallySearchEpisodePrompt,
      SonarrEvent::TriggerAutomaticEpisodeSearch(None)
    )]
    fn test_episode_details_prompt_confirm_submit(
      #[case] prompt_block: ActiveSonarrBlock,
      #[case] expected_action: SonarrEvent,
      #[values(
        ActiveSonarrBlock::EpisodeDetails,
        ActiveSonarrBlock::EpisodeHistory,
        ActiveSonarrBlock::EpisodeFile,
        ActiveSonarrBlock::ManualEpisodeSearch
      )]
      active_sonarr_block: ActiveSonarrBlock,
    ) {
      let mut app = App::default();
      app.data.sonarr_data = create_test_sonarr_data();
      app.data.sonarr_data.prompt_confirm = true;
      app.push_navigation_stack(active_sonarr_block.into());
      app.push_navigation_stack(prompt_block.into());

      EpisodeDetailsHandler::with(SUBMIT_KEY, &mut app, prompt_block, None).handle();

      assert!(app.data.sonarr_data.prompt_confirm);
      assert_eq!(app.get_current_route(), active_sonarr_block.into());
      assert_eq!(
        app.data.sonarr_data.prompt_confirm_action,
        Some(expected_action)
      );
    }

    #[test]
    fn test_manual_episode_search_confirm_prompt_confirm_submit() {
      let mut app = App::default();
      app.data.sonarr_data = create_test_sonarr_data();
      app.data.sonarr_data.prompt_confirm = true;
      app.push_navigation_stack(ActiveSonarrBlock::ManualEpisodeSearch.into());
      app.push_navigation_stack(ActiveSonarrBlock::ManualEpisodeSearchConfirmPrompt.into());

      EpisodeDetailsHandler::with(
        SUBMIT_KEY,
        &mut app,
        ActiveSonarrBlock::ManualEpisodeSearchConfirmPrompt,
        None,
      )
      .handle();

      assert!(app.data.sonarr_data.prompt_confirm);
      assert_eq!(
        app.get_current_route(),
        ActiveSonarrBlock::ManualEpisodeSearch.into()
      );
      assert_eq!(
        app.data.sonarr_data.prompt_confirm_action,
        Some(SonarrEvent::DownloadRelease(SonarrReleaseDownloadBody {
          guid: String::new(),
          indexer_id: 0,
          episode_id: Some(0),
          ..SonarrReleaseDownloadBody::default()
        }))
      );
    }

    #[rstest]
    fn test_episode_details_prompt_decline_submit(
      #[values(
        ActiveSonarrBlock::AutomaticallySearchEpisodePrompt,
        ActiveSonarrBlock::ManualEpisodeSearchConfirmPrompt
      )]
      prompt_block: ActiveSonarrBlock,
    ) {
      let mut app = App::default();
      app.data.sonarr_data = create_test_sonarr_data();
      app.push_navigation_stack(ActiveSonarrBlock::EpisodeDetails.into());
      app.push_navigation_stack(prompt_block.into());

      EpisodeDetailsHandler::with(SUBMIT_KEY, &mut app, prompt_block, None).handle();

      assert!(!app.data.sonarr_data.prompt_confirm);
      assert_eq!(
        app.get_current_route(),
        ActiveSonarrBlock::EpisodeDetails.into()
      );
      assert_eq!(app.data.sonarr_data.prompt_confirm_action, None);
    }

    #[test]
    fn test_manual_episode_search_submit() {
      let mut app = App::default();
      app.data.sonarr_data = create_test_sonarr_data();
      app.push_navigation_stack(ActiveSonarrBlock::ManualEpisodeSearch.into());

      EpisodeDetailsHandler::with(
        SUBMIT_KEY,
        &mut app,
        ActiveSonarrBlock::ManualEpisodeSearch,
        None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveSonarrBlock::ManualEpisodeSearchConfirmPrompt.into()
      );
    }

    #[test]
    fn test_manual_episode_search_submit_no_op_when_not_ready() {
      let mut app = App::default();
      app.data.sonarr_data = create_test_sonarr_data();
      app.is_loading = true;
      app.push_navigation_stack(ActiveSonarrBlock::ManualEpisodeSearch.into());

      EpisodeDetailsHandler::with(
        SUBMIT_KEY,
        &mut app,
        ActiveSonarrBlock::ManualEpisodeSearch,
        None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveSonarrBlock::ManualEpisodeSearch.into()
      );
    }
  }

  mod test_handle_esc {
    use super::*;
    use crate::event::Key;
    use pretty_assertions::assert_eq;

    const ESC_KEY: Key = DEFAULT_KEYBINDINGS.esc.key;

    #[test]
    fn test_episode_history_details_block_esc() {
      let mut app = App::default();
      app.data.sonarr_data = create_test_sonarr_data();
      app.push_navigation_stack(ActiveSonarrBlock::EpisodeHistory.into());
      app.push_navigation_stack(ActiveSonarrBlock::EpisodeHistoryDetails.into());

      EpisodeDetailsHandler::with(
        ESC_KEY,
        &mut app,
        ActiveSonarrBlock::EpisodeHistoryDetails,
        None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveSonarrBlock::EpisodeHistory.into()
      );
    }

    #[rstest]
    fn test_episode_details_prompts_esc(
      #[values(
        ActiveSonarrBlock::AutomaticallySearchEpisodePrompt,
        ActiveSonarrBlock::ManualEpisodeSearchConfirmPrompt
      )]
      prompt_block: ActiveSonarrBlock,
      #[values(true, false)] is_ready: bool,
    ) {
      let mut app = App::default();
      app.data.sonarr_data = create_test_sonarr_data();
      app.is_loading = is_ready;
      app.data.sonarr_data.prompt_confirm = true;
      app.push_navigation_stack(ActiveSonarrBlock::EpisodeDetails.into());
      app.push_navigation_stack(prompt_block.into());

      EpisodeDetailsHandler::with(ESC_KEY, &mut app, prompt_block, None).handle();

      assert!(!app.data.sonarr_data.prompt_confirm);
      assert_eq!(
        app.get_current_route(),
        ActiveSonarrBlock::EpisodeDetails.into()
      );
    }

    #[rstest]
    fn test_episode_details_tabs_esc(
      #[values(
        ActiveSonarrBlock::EpisodeDetails,
        ActiveSonarrBlock::EpisodeHistory,
        ActiveSonarrBlock::EpisodeFile,
        ActiveSonarrBlock::ManualEpisodeSearch
      )]
      active_sonarr_block: ActiveSonarrBlock,
    ) {
      let mut app = App::default();
      app.data.sonarr_data = create_test_sonarr_data();
      app.push_navigation_stack(ActiveSonarrBlock::SeasonDetails.into());
      app.push_navigation_stack(active_sonarr_block.into());

      EpisodeDetailsHandler::with(ESC_KEY, &mut app, active_sonarr_block, None).handle();

      assert_eq!(
        app.get_current_route(),
        ActiveSonarrBlock::SeasonDetails.into()
      );
      assert!(app
        .data
        .sonarr_data
        .season_details_modal
        .as_ref()
        .unwrap()
        .episode_details_modal
        .is_none());
    }
  }

  mod test_handle_key_char {
    use super::*;
    use crate::models::servarr_data::sonarr::sonarr_data::sonarr_test_utils::utils::create_test_sonarr_data;
    use crate::network::sonarr_network::SonarrEvent;
    use pretty_assertions::assert_eq;

    #[rstest]
    fn test_auto_search_key(
      #[values(
        ActiveSonarrBlock::EpisodeDetails,
        ActiveSonarrBlock::EpisodeHistory,
        ActiveSonarrBlock::EpisodeFile,
        ActiveSonarrBlock::ManualEpisodeSearch
      )]
      active_sonarr_block: ActiveSonarrBlock,
    ) {
      let mut app = App::default();
      app.data.sonarr_data = create_test_sonarr_data();
      app.push_navigation_stack(active_sonarr_block.into());

      EpisodeDetailsHandler::with(
        DEFAULT_KEYBINDINGS.auto_search.key,
        &mut app,
        active_sonarr_block,
        None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveSonarrBlock::AutomaticallySearchEpisodePrompt.into()
      );
    }

    #[rstest]
    fn test_auto_search_key_no_op_when_not_ready(
      #[values(
        ActiveSonarrBlock::EpisodeDetails,
        ActiveSonarrBlock::EpisodeHistory,
        ActiveSonarrBlock::EpisodeFile,
        ActiveSonarrBlock::ManualEpisodeSearch
      )]
      active_sonarr_block: ActiveSonarrBlock,
    ) {
      let mut app = App::default();
      app.is_loading = true;
      app.push_navigation_stack(active_sonarr_block.into());

      EpisodeDetailsHandler::with(
        DEFAULT_KEYBINDINGS.auto_search.key,
        &mut app,
        active_sonarr_block,
        None,
      )
      .handle();

      assert_eq!(app.get_current_route(), active_sonarr_block.into());
    }

    #[rstest]
    fn test_refresh_key(
      #[values(
        ActiveSonarrBlock::EpisodeDetails,
        ActiveSonarrBlock::EpisodeHistory,
        ActiveSonarrBlock::EpisodeFile,
        ActiveSonarrBlock::ManualEpisodeSearch
      )]
      active_sonarr_block: ActiveSonarrBlock,
    ) {
      let mut app = App::default();
      app.data.sonarr_data = create_test_sonarr_data();
      app.push_navigation_stack(active_sonarr_block.into());
      app.is_routing = false;

      EpisodeDetailsHandler::with(
        DEFAULT_KEYBINDINGS.refresh.key,
        &mut app,
        active_sonarr_block,
        None,
      )
      .handle();

      assert_eq!(app.get_current_route(), active_sonarr_block.into());
      assert!(app.is_routing);
    }

    #[rstest]
    fn test_refresh_key_no_op_when_not_ready(
      #[values(
        ActiveSonarrBlock::EpisodeDetails,
        ActiveSonarrBlock::EpisodeHistory,
        ActiveSonarrBlock::EpisodeFile,
        ActiveSonarrBlock::ManualEpisodeSearch
      )]
      active_sonarr_block: ActiveSonarrBlock,
    ) {
      let mut app = App::default();
      app.data.sonarr_data = create_test_sonarr_data();
      app.is_loading = true;
      app.push_navigation_stack(active_sonarr_block.into());
      app.is_routing = false;

      EpisodeDetailsHandler::with(
        DEFAULT_KEYBINDINGS.refresh.key,
        &mut app,
        active_sonarr_block,
        None,
      )
      .handle();

      assert_eq!(app.get_current_route(), active_sonarr_block.into());
      assert!(!app.is_routing);
    }

    #[rstest]
    fn test_episode_details_prompt_confirm_confirm_key(
      #[values(
        ActiveSonarrBlock::EpisodeDetails,
        ActiveSonarrBlock::EpisodeHistory,
        ActiveSonarrBlock::EpisodeFile,
        ActiveSonarrBlock::ManualEpisodeSearch
      )]
      active_sonarr_block: ActiveSonarrBlock,
    ) {
      let mut app = App::default();
      app.data.sonarr_data = create_test_sonarr_data();
      app.data.sonarr_data.prompt_confirm = true;
      app.push_navigation_stack(active_sonarr_block.into());
      app.push_navigation_stack(ActiveSonarrBlock::AutomaticallySearchEpisodePrompt.into());

      EpisodeDetailsHandler::with(
        DEFAULT_KEYBINDINGS.confirm.key,
        &mut app,
        ActiveSonarrBlock::AutomaticallySearchEpisodePrompt,
        None,
      )
      .handle();

      assert!(app.data.sonarr_data.prompt_confirm);
      assert_eq!(app.get_current_route(), active_sonarr_block.into());
      assert_eq!(
        app.data.sonarr_data.prompt_confirm_action,
        Some(SonarrEvent::TriggerAutomaticEpisodeSearch(None))
      );
    }

    #[test]
    fn test_episode_details_manual_search_confirm_prompt_confirm_confirm_key() {
      let mut app = App::default();
      app.data.sonarr_data = create_test_sonarr_data();
      app.data.sonarr_data.prompt_confirm = true;
      app.push_navigation_stack(ActiveSonarrBlock::ManualEpisodeSearch.into());
      app.push_navigation_stack(ActiveSonarrBlock::ManualEpisodeSearchConfirmPrompt.into());

      EpisodeDetailsHandler::with(
        DEFAULT_KEYBINDINGS.confirm.key,
        &mut app,
        ActiveSonarrBlock::ManualEpisodeSearchConfirmPrompt,
        None,
      )
        .handle();

      assert!(app.data.sonarr_data.prompt_confirm);
      assert_eq!(
        app.get_current_route(),
        ActiveSonarrBlock::ManualEpisodeSearch.into()
      );
      assert_eq!(
        app.data.sonarr_data.prompt_confirm_action,
        Some(SonarrEvent::DownloadRelease(SonarrReleaseDownloadBody {
          guid: String::new(),
          indexer_id: 0,
          episode_id: Some(0),
          ..SonarrReleaseDownloadBody::default()
        }))
      );
    }
  }

  #[test]
  fn test_episode_details_handler_accepts() {
    ActiveSonarrBlock::iter().for_each(|active_sonarr_block| {
      if EPISODE_DETAILS_BLOCKS.contains(&active_sonarr_block) {
        assert!(EpisodeDetailsHandler::accepts(active_sonarr_block));
      } else {
        assert!(!EpisodeDetailsHandler::accepts(active_sonarr_block));
      }
    });
  }

  #[test]
  fn test_episode_details_handler_is_not_ready_when_loading() {
    let mut app = App::default();
    app.push_navigation_stack(ActiveSonarrBlock::EpisodeDetails.into());
    app.is_loading = true;

    let handler = EpisodeDetailsHandler::with(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveSonarrBlock::EpisodeDetails,
      None,
    );

    assert!(!handler.is_ready());
  }

  #[test]
  fn test_episode_details_handler_is_not_ready_when_season_details_modal_is_empty() {
    let mut app = App::default();
    app.push_navigation_stack(ActiveSonarrBlock::EpisodeDetails.into());
    app.is_loading = false;

    let handler = EpisodeDetailsHandler::with(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveSonarrBlock::EpisodeDetails,
      None,
    );

    assert!(!handler.is_ready());
  }

  #[test]
  fn test_episode_details_handler_is_not_ready_when_episode_details_modal_is_empty() {
    let mut app = App::default();
    app.data.sonarr_data = create_test_sonarr_data();
    app
      .data
      .sonarr_data
      .season_details_modal
      .as_mut()
      .unwrap()
      .episode_details_modal = None;
    app.push_navigation_stack(ActiveSonarrBlock::EpisodeDetails.into());
    app.is_loading = false;

    let handler = EpisodeDetailsHandler::with(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveSonarrBlock::EpisodeDetails,
      None,
    );

    assert!(!handler.is_ready());
  }

  #[test]
  fn test_episode_details_handler_is_not_ready_when_episode_history_table_is_empty() {
    let mut app = App::default();
    app.data.sonarr_data = create_test_sonarr_data();
    app
      .data
      .sonarr_data
      .season_details_modal
      .as_mut()
      .unwrap()
      .episode_details_modal
      .as_mut()
      .unwrap()
      .episode_history = StatefulTable::default();
    app.push_navigation_stack(ActiveSonarrBlock::EpisodeHistory.into());
    app.is_loading = false;

    let handler = EpisodeDetailsHandler::with(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveSonarrBlock::EpisodeHistory,
      None,
    );

    assert!(!handler.is_ready());
  }

  #[test]
  fn test_episode_details_handler_is_not_ready_when_episode_releases_table_is_empty() {
    let mut app = App::default();
    app.data.sonarr_data = create_test_sonarr_data();
    app
      .data
      .sonarr_data
      .season_details_modal
      .as_mut()
      .unwrap()
      .episode_details_modal
      .as_mut()
      .unwrap()
      .episode_releases = StatefulTable::default();
    app.push_navigation_stack(ActiveSonarrBlock::ManualEpisodeSearch.into());
    app.is_loading = false;

    let handler = EpisodeDetailsHandler::with(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveSonarrBlock::ManualEpisodeSearch,
      None,
    );

    assert!(!handler.is_ready());
  }

  #[rstest]
  fn test_episode_details_handler_is_ready_with_empty_tables_for_details_and_file_routes(
    #[values(ActiveSonarrBlock::EpisodeDetails, ActiveSonarrBlock::EpisodeFile)]
    active_sonarr_block: ActiveSonarrBlock,
  ) {
    let mut app = App::default();
    app.data.sonarr_data = create_test_sonarr_data();
    app
      .data
      .sonarr_data
      .season_details_modal
      .as_mut()
      .unwrap()
      .episode_details_modal = Some(EpisodeDetailsModal::default());
    app.push_navigation_stack(active_sonarr_block.into());
    app.is_loading = false;

    let handler = EpisodeDetailsHandler::with(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      active_sonarr_block,
      None,
    );

    assert!(handler.is_ready());
  }

  #[rstest]
  fn test_episode_details_handler_is_ready(
    #[values(
      ActiveSonarrBlock::EpisodeDetails,
      ActiveSonarrBlock::EpisodeFile,
      ActiveSonarrBlock::EpisodeHistory,
      ActiveSonarrBlock::ManualEpisodeSearch
    )]
    active_sonarr_block: ActiveSonarrBlock,
  ) {
    let mut app = App::default();
    app.data.sonarr_data = create_test_sonarr_data();
    app.push_navigation_stack(active_sonarr_block.into());
    app.is_loading = false;

    let handler = EpisodeDetailsHandler::with(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      active_sonarr_block,
      None,
    );

    assert!(handler.is_ready());
  }
}
