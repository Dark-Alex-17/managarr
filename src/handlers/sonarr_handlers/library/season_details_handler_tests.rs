#[cfg(test)]
mod tests {
  use crate::app::App;
  use crate::app::key_binding::DEFAULT_KEYBINDINGS;
  use crate::assert_modal_absent;
  use crate::assert_navigation_pushed;
  use crate::handlers::KeyEventHandler;
  use crate::handlers::sonarr_handlers::library::season_details_handler::{
    SeasonDetailsHandler, releases_sorting_options,
  };
  use crate::handlers::sonarr_handlers::sonarr_handler_test_utils::utils::episode;
  use crate::models::HorizontallyScrollableText;
  use crate::models::servarr_data::sonarr::modals::SeasonDetailsModal;
  use crate::models::servarr_data::sonarr::sonarr_data::sonarr_test_utils::utils::create_test_sonarr_data;
  use crate::models::servarr_data::sonarr::sonarr_data::{
    ActiveSonarrBlock, SEASON_DETAILS_BLOCKS,
  };
  use crate::models::servarr_models::{Language, Quality, QualityWrapper};
  use crate::models::sonarr_models::{SonarrRelease, SonarrReleaseDownloadBody};
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
    fn test_delete_episode_prompt() {
      let mut app = App::test_default();
      app.data.sonarr_data = create_test_sonarr_data();
      app.push_navigation_stack(ActiveSonarrBlock::SeasonDetails.into());

      SeasonDetailsHandler::new(DELETE_KEY, &mut app, ActiveSonarrBlock::SeasonDetails, None)
        .handle();

      assert_navigation_pushed!(app, ActiveSonarrBlock::DeleteEpisodeFilePrompt.into());
    }

    #[test]
    fn test_delete_episode_prompt_no_op_when_not_ready() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveSonarrBlock::SeasonDetails.into());
      app.is_loading = true;

      SeasonDetailsHandler::new(DELETE_KEY, &mut app, ActiveSonarrBlock::SeasonDetails, None)
        .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveSonarrBlock::SeasonDetails.into()
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
        ActiveSonarrBlock::AutomaticallySearchSeasonPrompt,
        ActiveSonarrBlock::ManualSeasonSearchConfirmPrompt,
        ActiveSonarrBlock::DeleteEpisodeFilePrompt
      )]
      active_sonarr_block: ActiveSonarrBlock,
      #[values(Key::Left, Key::Right)] key: Key,
    ) {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveSonarrBlock::SeasonDetails.into());

      SeasonDetailsHandler::new(key, &mut app, active_sonarr_block, None).handle();

      assert!(app.data.sonarr_data.prompt_confirm);

      SeasonDetailsHandler::new(key, &mut app, active_sonarr_block, None).handle();

      assert!(!app.data.sonarr_data.prompt_confirm);
    }

    #[rstest]
    #[case(ActiveSonarrBlock::SeasonDetails, ActiveSonarrBlock::SeasonHistory)]
    #[case(
      ActiveSonarrBlock::SeasonHistory,
      ActiveSonarrBlock::ManualSeasonSearch
    )]
    #[case(
      ActiveSonarrBlock::ManualSeasonSearch,
      ActiveSonarrBlock::SeasonDetails
    )]
    fn test_season_details_tabs_left_right_action(
      #[case] left_block: ActiveSonarrBlock,
      #[case] right_block: ActiveSonarrBlock,
      #[values(true, false)] is_ready: bool,
    ) {
      let mut app = App::test_default();
      app.data.sonarr_data = create_test_sonarr_data();
      app.push_navigation_stack(ActiveSonarrBlock::SeriesDetails.into());
      app.is_loading = is_ready;
      app.push_navigation_stack(right_block.into());
      app
        .data
        .sonarr_data
        .season_details_modal
        .as_mut()
        .unwrap()
        .season_details_tabs
        .index = app
        .data
        .sonarr_data
        .season_details_modal
        .as_ref()
        .unwrap()
        .season_details_tabs
        .tabs
        .iter()
        .position(|tab_route| tab_route.route == right_block.into())
        .unwrap_or_default();

      SeasonDetailsHandler::new(DEFAULT_KEYBINDINGS.left.key, &mut app, right_block, None).handle();

      assert_eq!(
        app.get_current_route(),
        app
          .data
          .sonarr_data
          .season_details_modal
          .as_ref()
          .unwrap()
          .season_details_tabs
          .get_active_route()
      );
      assert_navigation_pushed!(app, left_block.into());

      SeasonDetailsHandler::new(DEFAULT_KEYBINDINGS.right.key, &mut app, left_block, None).handle();

      assert_eq!(
        app.get_current_route(),
        app
          .data
          .sonarr_data
          .season_details_modal
          .as_ref()
          .unwrap()
          .season_details_tabs
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
    use crate::network::sonarr_network::SonarrEvent;
    use pretty_assertions::assert_eq;

    const SUBMIT_KEY: Key = DEFAULT_KEYBINDINGS.submit.key;

    #[test]
    fn test_season_details_submit() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveSonarrBlock::SeasonDetails.into());
      app.data.sonarr_data = create_test_sonarr_data();

      SeasonDetailsHandler::new(SUBMIT_KEY, &mut app, ActiveSonarrBlock::SeasonDetails, None)
        .handle();

      assert_navigation_pushed!(app, ActiveSonarrBlock::EpisodeDetails.into());
    }

    #[test]
    fn test_season_details_submit_no_op_on_empty_episodes_table() {
      let mut app = App::test_default();
      app.data.sonarr_data = create_test_sonarr_data();
      app
        .data
        .sonarr_data
        .season_details_modal
        .as_mut()
        .unwrap()
        .episodes = StatefulTable::default();
      app.push_navigation_stack(ActiveSonarrBlock::SeasonDetails.into());

      SeasonDetailsHandler::new(SUBMIT_KEY, &mut app, ActiveSonarrBlock::SeasonDetails, None)
        .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveSonarrBlock::SeasonDetails.into()
      );
    }

    #[test]
    fn test_season_details_submit_no_op_when_not_ready() {
      let mut app = App::test_default();
      app.is_loading = true;
      app.push_navigation_stack(ActiveSonarrBlock::SeasonDetails.into());

      SeasonDetailsHandler::new(SUBMIT_KEY, &mut app, ActiveSonarrBlock::SeasonDetails, None)
        .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveSonarrBlock::SeasonDetails.into()
      );
    }

    #[test]
    fn test_season_history_submit() {
      let mut app = App::test_default();
      app.data.sonarr_data = create_test_sonarr_data();

      SeasonDetailsHandler::new(SUBMIT_KEY, &mut app, ActiveSonarrBlock::SeasonHistory, None)
        .handle();

      assert_navigation_pushed!(app, ActiveSonarrBlock::SeasonHistoryDetails.into());
    }

    #[test]
    fn test_season_history_submit_no_op_when_season_history_is_empty() {
      let mut app = App::test_default();
      app.data.sonarr_data = create_test_sonarr_data();
      app
        .data
        .sonarr_data
        .season_details_modal
        .as_mut()
        .unwrap()
        .season_history = StatefulTable::default();
      app.push_navigation_stack(ActiveSonarrBlock::SeasonHistory.into());

      SeasonDetailsHandler::new(SUBMIT_KEY, &mut app, ActiveSonarrBlock::SeasonHistory, None)
        .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveSonarrBlock::SeasonHistory.into()
      );
    }

    #[test]
    fn test_season_history_submit_no_op_when_not_ready() {
      let mut app = App::test_default();
      app.data.sonarr_data = create_test_sonarr_data();
      app.is_loading = true;
      app.push_navigation_stack(ActiveSonarrBlock::SeasonHistory.into());

      SeasonDetailsHandler::new(SUBMIT_KEY, &mut app, ActiveSonarrBlock::SeasonHistory, None)
        .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveSonarrBlock::SeasonHistory.into()
      );
    }

    #[rstest]
    #[case(
      ActiveSonarrBlock::AutomaticallySearchSeasonPrompt,
      SonarrEvent::TriggerAutomaticSeasonSearch((0, 0))
    )]
    #[case(
      ActiveSonarrBlock::DeleteEpisodeFilePrompt,
      SonarrEvent::DeleteEpisodeFile(0)
    )]
    fn test_season_details_prompt_confirm_submit(
      #[case] prompt_block: ActiveSonarrBlock,
      #[case] expected_action: SonarrEvent,
      #[values(ActiveSonarrBlock::SeasonDetails, ActiveSonarrBlock::SeasonHistory)]
      active_sonarr_block: ActiveSonarrBlock,
    ) {
      let mut app = App::test_default();
      app.data.sonarr_data = create_test_sonarr_data();
      app.data.sonarr_data.prompt_confirm = true;
      app.push_navigation_stack(active_sonarr_block.into());
      app.push_navigation_stack(prompt_block.into());

      SeasonDetailsHandler::new(SUBMIT_KEY, &mut app, prompt_block, None).handle();

      assert!(app.data.sonarr_data.prompt_confirm);
      assert_navigation_popped!(app, active_sonarr_block.into());
      assert_some_eq_x!(
        &app.data.sonarr_data.prompt_confirm_action,
        &expected_action
      );
    }

    #[test]
    fn test_season_details_manual_search_confirm_prompt_confirm_submit() {
      let mut app = App::test_default();
      app.data.sonarr_data = create_test_sonarr_data();
      app.data.sonarr_data.prompt_confirm = true;
      app.push_navigation_stack(ActiveSonarrBlock::ManualSeasonSearch.into());
      app.push_navigation_stack(ActiveSonarrBlock::ManualSeasonSearchConfirmPrompt.into());

      SeasonDetailsHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveSonarrBlock::ManualSeasonSearchConfirmPrompt,
        None,
      )
      .handle();

      assert!(app.data.sonarr_data.prompt_confirm);
      assert_navigation_popped!(app, ActiveSonarrBlock::ManualSeasonSearch.into());
      assert_some_eq_x!(
        &app.data.sonarr_data.prompt_confirm_action,
        &SonarrEvent::DownloadRelease(SonarrReleaseDownloadBody {
          guid: String::new(),
          indexer_id: 0,
          series_id: Some(0),
          season_number: Some(0),
          ..SonarrReleaseDownloadBody::default()
        })
      );
    }

    #[rstest]
    fn test_season_details_prompt_decline_submit(
      #[values(
        ActiveSonarrBlock::AutomaticallySearchSeasonPrompt,
        ActiveSonarrBlock::DeleteEpisodeFilePrompt,
        ActiveSonarrBlock::ManualSeasonSearchConfirmPrompt
      )]
      prompt_block: ActiveSonarrBlock,
    ) {
      let mut app = App::test_default();
      app.data.sonarr_data = create_test_sonarr_data();
      app.push_navigation_stack(ActiveSonarrBlock::SeasonDetails.into());
      app.push_navigation_stack(prompt_block.into());

      SeasonDetailsHandler::new(SUBMIT_KEY, &mut app, prompt_block, None).handle();

      assert!(!app.data.sonarr_data.prompt_confirm);
      assert_navigation_popped!(app, ActiveSonarrBlock::SeasonDetails.into());
      assert_none!(app.data.sonarr_data.prompt_confirm_action);
    }

    #[test]
    fn test_manual_season_search_submit() {
      let mut app = App::test_default();
      app.data.sonarr_data = create_test_sonarr_data();
      app.push_navigation_stack(ActiveSonarrBlock::ManualSeasonSearch.into());

      SeasonDetailsHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveSonarrBlock::ManualSeasonSearch,
        None,
      )
      .handle();

      assert_navigation_pushed!(
        app,
        ActiveSonarrBlock::ManualSeasonSearchConfirmPrompt.into()
      );
    }

    #[test]
    fn test_manual_season_search_submit_no_op_when_not_ready() {
      let mut app = App::test_default();
      app.data.sonarr_data = create_test_sonarr_data();
      app.is_loading = true;
      app.push_navigation_stack(ActiveSonarrBlock::ManualSeasonSearch.into());

      SeasonDetailsHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveSonarrBlock::ManualSeasonSearch,
        None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveSonarrBlock::ManualSeasonSearch.into()
      );
    }
  }

  mod test_handle_esc {
    use super::*;
    use crate::assert_navigation_popped;
    use crate::event::Key;
    use crate::models::sonarr_models::SonarrHistoryItem;
    use crate::models::stateful_table::StatefulTable;
    use pretty_assertions::assert_eq;
    use ratatui::widgets::TableState;

    const ESC_KEY: Key = DEFAULT_KEYBINDINGS.esc.key;

    #[test]
    fn test_season_history_details_block_esc() {
      let mut app = App::test_default();
      app.data.sonarr_data = create_test_sonarr_data();
      app.push_navigation_stack(ActiveSonarrBlock::SeasonHistory.into());
      app.push_navigation_stack(ActiveSonarrBlock::SeasonHistoryDetails.into());

      SeasonDetailsHandler::new(
        ESC_KEY,
        &mut app,
        ActiveSonarrBlock::SeasonHistoryDetails,
        None,
      )
      .handle();

      assert_navigation_popped!(app, ActiveSonarrBlock::SeasonHistory.into());
    }

    #[rstest]
    fn test_season_details_prompts_esc(
      #[values(
        ActiveSonarrBlock::AutomaticallySearchSeasonPrompt,
        ActiveSonarrBlock::DeleteEpisodeFilePrompt,
        ActiveSonarrBlock::ManualSeasonSearchConfirmPrompt
      )]
      prompt_block: ActiveSonarrBlock,
      #[values(true, false)] is_ready: bool,
    ) {
      let mut app = App::test_default();
      app.data.sonarr_data = create_test_sonarr_data();
      app.is_loading = is_ready;
      app.data.sonarr_data.prompt_confirm = true;
      app.push_navigation_stack(ActiveSonarrBlock::SeasonDetails.into());
      app.push_navigation_stack(prompt_block.into());

      SeasonDetailsHandler::new(ESC_KEY, &mut app, prompt_block, None).handle();

      assert!(!app.data.sonarr_data.prompt_confirm);
      assert_navigation_popped!(app, ActiveSonarrBlock::SeasonDetails.into());
    }

    #[test]
    fn test_season_history_esc_resets_filter_if_one_is_set_instead_of_closing_the_window() {
      let mut app = App::test_default();
      app.data.sonarr_data = create_test_sonarr_data();
      let mut season_history = StatefulTable {
        filter: Some("Test".into()),
        filtered_items: Some(vec![SonarrHistoryItem::default()]),
        filtered_state: Some(TableState::default()),
        ..StatefulTable::default()
      };
      season_history.set_items(vec![SonarrHistoryItem::default()]);
      app
        .data
        .sonarr_data
        .season_details_modal
        .as_mut()
        .unwrap()
        .season_history = season_history;
      app.push_navigation_stack(ActiveSonarrBlock::SeriesDetails.into());
      app.push_navigation_stack(ActiveSonarrBlock::SeasonHistory.into());

      SeasonDetailsHandler::new(ESC_KEY, &mut app, ActiveSonarrBlock::SeasonHistory, None).handle();

      assert_eq!(
        app.get_current_route(),
        ActiveSonarrBlock::SeasonHistory.into()
      );
      assert_none!(
        app
          .data
          .sonarr_data
          .season_details_modal
          .as_ref()
          .unwrap()
          .season_history
          .filter
      );
      assert_none!(
        app
          .data
          .sonarr_data
          .season_details_modal
          .as_ref()
          .unwrap()
          .season_history
          .filtered_items
      );
      assert_none!(
        app
          .data
          .sonarr_data
          .season_details_modal
          .as_ref()
          .unwrap()
          .season_history
          .filtered_state
      );
    }

    #[rstest]
    fn test_season_details_tabs_esc(
      #[values(
        ActiveSonarrBlock::SeasonDetails,
        ActiveSonarrBlock::SeasonHistory,
        ActiveSonarrBlock::ManualSeasonSearch
      )]
      active_sonarr_block: ActiveSonarrBlock,
    ) {
      let mut app = App::test_default();
      app.data.sonarr_data = create_test_sonarr_data();
      app.push_navigation_stack(ActiveSonarrBlock::SeriesDetails.into());
      app.push_navigation_stack(active_sonarr_block.into());

      SeasonDetailsHandler::new(ESC_KEY, &mut app, active_sonarr_block, None).handle();

      assert_navigation_popped!(app, ActiveSonarrBlock::SeriesDetails.into());
      assert_modal_absent!(app.data.sonarr_data.season_details_modal);
    }
  }

  mod test_handle_key_char {
    use super::*;
    use crate::assert_navigation_popped;
    use crate::models::servarr_data::sonarr::sonarr_data::sonarr_test_utils::utils::create_test_sonarr_data;
    use crate::network::sonarr_network::SonarrEvent;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_toggle_monitoring_key() {
      let mut app = App::test_default();
      app.data.sonarr_data = create_test_sonarr_data();
      app
        .data
        .sonarr_data
        .season_details_modal
        .as_mut()
        .unwrap()
        .episodes
        .set_items(vec![episode()]);
      app.push_navigation_stack(ActiveSonarrBlock::SeasonDetails.into());
      app.is_routing = false;

      SeasonDetailsHandler::new(
        DEFAULT_KEYBINDINGS.toggle_monitoring.key,
        &mut app,
        ActiveSonarrBlock::SeasonDetails,
        None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveSonarrBlock::SeasonDetails.into()
      );
      assert!(app.data.sonarr_data.prompt_confirm);
      assert!(app.is_routing);
      assert_some_eq_x!(
        &app.data.sonarr_data.prompt_confirm_action,
        &SonarrEvent::ToggleEpisodeMonitoring(1)
      );
    }

    #[test]
    fn test_toggle_monitoring_key_no_op_when_not_ready() {
      let mut app = App::test_default();
      app.is_loading = true;
      app.push_navigation_stack(ActiveSonarrBlock::SeasonDetails.into());
      app.is_routing = false;

      SeasonDetailsHandler::new(
        DEFAULT_KEYBINDINGS.toggle_monitoring.key,
        &mut app,
        ActiveSonarrBlock::SeasonDetails,
        None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveSonarrBlock::SeasonDetails.into()
      );
      assert!(!app.data.sonarr_data.prompt_confirm);
      assert_modal_absent!(app.data.sonarr_data.prompt_confirm_action);
      assert!(!app.is_routing);
    }

    #[rstest]
    fn test_auto_search_key(
      #[values(
        ActiveSonarrBlock::SeasonDetails,
        ActiveSonarrBlock::SeasonHistory,
        ActiveSonarrBlock::ManualSeasonSearch
      )]
      active_sonarr_block: ActiveSonarrBlock,
    ) {
      let mut app = App::test_default();
      app.data.sonarr_data = create_test_sonarr_data();
      app.push_navigation_stack(active_sonarr_block.into());

      SeasonDetailsHandler::new(
        DEFAULT_KEYBINDINGS.auto_search.key,
        &mut app,
        active_sonarr_block,
        None,
      )
      .handle();

      assert_navigation_pushed!(
        app,
        ActiveSonarrBlock::AutomaticallySearchSeasonPrompt.into()
      );
    }

    #[rstest]
    fn test_auto_search_key_no_op_when_not_ready(
      #[values(
        ActiveSonarrBlock::SeasonDetails,
        ActiveSonarrBlock::SeasonHistory,
        ActiveSonarrBlock::ManualSeasonSearch
      )]
      active_sonarr_block: ActiveSonarrBlock,
    ) {
      let mut app = App::test_default();
      app.is_loading = true;
      app.push_navigation_stack(active_sonarr_block.into());

      SeasonDetailsHandler::new(
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
        ActiveSonarrBlock::SeasonDetails,
        ActiveSonarrBlock::SeasonHistory,
        ActiveSonarrBlock::ManualSeasonSearch
      )]
      active_sonarr_block: ActiveSonarrBlock,
    ) {
      let mut app = App::test_default();
      app.data.sonarr_data = create_test_sonarr_data();
      app.push_navigation_stack(active_sonarr_block.into());
      app.is_routing = false;

      SeasonDetailsHandler::new(
        DEFAULT_KEYBINDINGS.refresh.key,
        &mut app,
        active_sonarr_block,
        None,
      )
      .handle();

      assert_navigation_pushed!(app, active_sonarr_block.into());
      assert!(app.is_routing);
    }

    #[rstest]
    fn test_refresh_key_no_op_when_not_ready(
      #[values(
        ActiveSonarrBlock::SeasonDetails,
        ActiveSonarrBlock::SeasonHistory,
        ActiveSonarrBlock::ManualSeasonSearch
      )]
      active_sonarr_block: ActiveSonarrBlock,
    ) {
      let mut app = App::test_default();
      app.data.sonarr_data = create_test_sonarr_data();
      app.is_loading = true;
      app.push_navigation_stack(active_sonarr_block.into());
      app.is_routing = false;

      SeasonDetailsHandler::new(
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
    #[case(
      ActiveSonarrBlock::AutomaticallySearchSeasonPrompt,
      SonarrEvent::TriggerAutomaticSeasonSearch((0, 0))
    )]
    #[case(
      ActiveSonarrBlock::DeleteEpisodeFilePrompt,
      SonarrEvent::DeleteEpisodeFile(0)
    )]
    fn test_season_details_prompt_confirm_confirm_key(
      #[case] prompt_block: ActiveSonarrBlock,
      #[case] expected_action: SonarrEvent,
      #[values(ActiveSonarrBlock::SeasonDetails, ActiveSonarrBlock::SeasonHistory)]
      active_sonarr_block: ActiveSonarrBlock,
    ) {
      let mut app = App::test_default();
      app.data.sonarr_data = create_test_sonarr_data();
      app.data.sonarr_data.prompt_confirm = true;
      app.push_navigation_stack(active_sonarr_block.into());
      app.push_navigation_stack(prompt_block.into());

      SeasonDetailsHandler::new(
        DEFAULT_KEYBINDINGS.confirm.key,
        &mut app,
        prompt_block,
        None,
      )
      .handle();

      assert!(app.data.sonarr_data.prompt_confirm);
      assert_navigation_popped!(app, active_sonarr_block.into());
      assert_some_eq_x!(
        &app.data.sonarr_data.prompt_confirm_action,
        &expected_action
      );
    }

    #[test]
    fn test_season_details_manual_search_confirm_prompt_confirm_confirm_key() {
      let mut app = App::test_default();
      app.data.sonarr_data = create_test_sonarr_data();
      app.data.sonarr_data.prompt_confirm = true;
      app.push_navigation_stack(ActiveSonarrBlock::ManualSeasonSearch.into());
      app.push_navigation_stack(ActiveSonarrBlock::ManualSeasonSearchConfirmPrompt.into());

      SeasonDetailsHandler::new(
        DEFAULT_KEYBINDINGS.confirm.key,
        &mut app,
        ActiveSonarrBlock::ManualSeasonSearchConfirmPrompt,
        None,
      )
      .handle();

      assert!(app.data.sonarr_data.prompt_confirm);
      assert_navigation_popped!(app, ActiveSonarrBlock::ManualSeasonSearch.into());
      assert_some_eq_x!(
        &app.data.sonarr_data.prompt_confirm_action,
        &SonarrEvent::DownloadRelease(SonarrReleaseDownloadBody {
          guid: String::new(),
          indexer_id: 0,
          series_id: Some(0),
          season_number: Some(0),
          ..SonarrReleaseDownloadBody::default()
        })
      );
    }
  }

  #[test]
  fn test_season_details_handler_accepts() {
    ActiveSonarrBlock::iter().for_each(|active_sonarr_block| {
      if SEASON_DETAILS_BLOCKS.contains(&active_sonarr_block) {
        assert!(SeasonDetailsHandler::accepts(active_sonarr_block));
      } else {
        assert!(!SeasonDetailsHandler::accepts(active_sonarr_block));
      }
    });
  }

  #[rstest]
  fn test_season_details_handler_ignore_special_keys(
    #[values(true, false)] ignore_special_keys_for_textbox_input: bool,
  ) {
    let mut app = App::test_default();
    app.ignore_special_keys_for_textbox_input = ignore_special_keys_for_textbox_input;
    let handler = SeasonDetailsHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveSonarrBlock::default(),
      None,
    );

    assert_eq!(
      handler.ignore_special_keys(),
      ignore_special_keys_for_textbox_input
    );
  }

  #[test]
  fn test_extract_episode_file_id() {
    let mut app = App::test_default();
    app.data.sonarr_data = create_test_sonarr_data();

    let episode_file_id = SeasonDetailsHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveSonarrBlock::SeasonDetails,
      None,
    )
    .extract_episode_file_id();

    assert_eq!(episode_file_id, 0);
  }

  #[test]
  #[should_panic(expected = "Season details have not been loaded")]
  fn test_extract_episode_file_id_empty_season_details_modal_panics() {
    let mut app = App::test_default();

    SeasonDetailsHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveSonarrBlock::SeasonDetails,
      None,
    )
    .extract_episode_file_id();
  }

  #[test]
  fn test_extract_episode_id() {
    let mut app = App::test_default();
    let mut season_details_modal = SeasonDetailsModal::default();
    season_details_modal.episodes.set_items(vec![episode()]);
    app.data.sonarr_data.season_details_modal = Some(season_details_modal);

    let episode_id = SeasonDetailsHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveSonarrBlock::SeasonDetails,
      None,
    )
    .extract_episode_id();

    assert_eq!(episode_id, 1);
  }

  #[test]
  #[should_panic(expected = "Season details have not been loaded")]
  fn test_extract_episode_id_panic_when_season_details_modal_is_none() {
    let mut app = App::test_default();

    SeasonDetailsHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveSonarrBlock::SeasonDetails,
      None,
    )
    .extract_episode_id();
  }

  #[test]
  fn test_extract_series_id_season_number_tuple() {
    let mut app = App::test_default();
    app.data.sonarr_data = create_test_sonarr_data();

    let (series_id, season_number) = SeasonDetailsHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveSonarrBlock::SeasonDetails,
      None,
    )
    .extract_series_id_season_number_tuple();

    assert_eq!(series_id, 0);
    assert_eq!(season_number, 0);
  }

  #[test]
  fn test_season_details_handler_is_not_ready_when_loading() {
    let mut app = App::test_default();
    app.push_navigation_stack(ActiveSonarrBlock::SeasonDetails.into());
    app.is_loading = true;

    let handler = SeasonDetailsHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveSonarrBlock::SeasonDetails,
      None,
    );

    assert!(!handler.is_ready());
  }

  #[test]
  fn test_season_details_handler_is_not_ready_when_not_loading_and_season_details_is_none() {
    let mut app = App::test_default();
    app.push_navigation_stack(ActiveSonarrBlock::SeasonDetails.into());

    let handler = SeasonDetailsHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveSonarrBlock::SeasonDetails,
      None,
    );

    assert!(!handler.is_ready());
  }

  #[test]
  fn test_season_details_handler_is_not_ready_when_not_loading_and_episodes_table_is_empty() {
    let mut app = App::test_default();
    app.data.sonarr_data.season_details_modal = Some(SeasonDetailsModal::default());
    app.push_navigation_stack(ActiveSonarrBlock::SeasonDetails.into());

    let handler = SeasonDetailsHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveSonarrBlock::SeasonDetails,
      None,
    );

    assert!(!handler.is_ready());
  }

  #[test]
  fn test_season_details_handler_is_not_ready_when_not_loading_and_history_table_is_empty() {
    let mut app = App::test_default();
    app.data.sonarr_data.season_details_modal = Some(SeasonDetailsModal::default());
    app.push_navigation_stack(ActiveSonarrBlock::SeasonHistory.into());

    let handler = SeasonDetailsHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveSonarrBlock::SeasonHistory,
      None,
    );

    assert!(!handler.is_ready());
  }

  #[test]
  fn test_season_details_handler_is_not_ready_when_not_loading_and_releases_table_is_empty() {
    let mut app = App::test_default();
    app.data.sonarr_data.season_details_modal = Some(SeasonDetailsModal::default());
    app.push_navigation_stack(ActiveSonarrBlock::ManualSeasonSearch.into());

    let handler = SeasonDetailsHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveSonarrBlock::ManualSeasonSearch,
      None,
    );

    assert!(!handler.is_ready());
  }

  #[rstest]
  fn test_season_details_handler_is_ready_when_not_loading_and_season_details_modal_is_populated(
    #[values(
      ActiveSonarrBlock::SeasonDetails,
      ActiveSonarrBlock::SeasonHistory,
      ActiveSonarrBlock::ManualSeasonSearch
    )]
    active_sonarr_block: ActiveSonarrBlock,
  ) {
    let mut app = App::test_default();
    app.data.sonarr_data = create_test_sonarr_data();
    app.push_navigation_stack(ActiveSonarrBlock::SeriesDetails.into());
    app.push_navigation_stack(active_sonarr_block.into());

    let handler = SeasonDetailsHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      active_sonarr_block,
      None,
    );

    assert!(handler.is_ready());
  }

  #[test]
  fn test_releases_sorting_options_source() {
    let expected_cmp_fn: fn(&SonarrRelease, &SonarrRelease) -> Ordering =
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
    let expected_cmp_fn: fn(&SonarrRelease, &SonarrRelease) -> Ordering = |a, b| a.age.cmp(&b.age);
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
    let expected_cmp_fn: fn(&SonarrRelease, &SonarrRelease) -> Ordering =
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
    let expected_cmp_fn: fn(&SonarrRelease, &SonarrRelease) -> Ordering = |a, b| {
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
    let expected_cmp_fn: fn(&SonarrRelease, &SonarrRelease) -> Ordering =
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
    let expected_cmp_fn: fn(&SonarrRelease, &SonarrRelease) -> Ordering =
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
    let expected_cmp_fn: fn(&SonarrRelease, &SonarrRelease) -> Ordering = |a, b| {
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
  fn test_releases_sorting_options_language() {
    let expected_cmp_fn: fn(&SonarrRelease, &SonarrRelease) -> Ordering = |a, b| {
      let default_language = Language {
        id: 1,
        name: "_".to_owned(),
      };
      let default_language_vec = vec![Some(default_language.clone())];
      let language_a = a.languages.as_ref().unwrap_or(&default_language_vec)[0]
        .as_ref()
        .unwrap_or(&default_language);
      let language_b = b.languages.as_ref().unwrap_or(&default_language_vec)[0]
        .as_ref()
        .unwrap_or(&default_language);

      language_a.cmp(language_b)
    };
    let mut expected_releases_vec = release_vec();
    expected_releases_vec.sort_by(expected_cmp_fn);

    let sort_option = releases_sorting_options()[7].clone();
    let mut sorted_releases_vec = release_vec();
    sorted_releases_vec.sort_by(sort_option.cmp_fn.unwrap());

    assert_eq!(sorted_releases_vec, expected_releases_vec);
    assert_str_eq!(sort_option.name, "Language");
  }

  #[test]
  fn test_releases_sorting_options_quality() {
    let expected_cmp_fn: fn(&SonarrRelease, &SonarrRelease) -> Ordering =
      |a, b| a.quality.cmp(&b.quality);
    let mut expected_releases_vec = release_vec();
    expected_releases_vec.sort_by(expected_cmp_fn);

    let sort_option = releases_sorting_options()[8].clone();
    let mut sorted_releases_vec = release_vec();
    sorted_releases_vec.sort_by(sort_option.cmp_fn.unwrap());

    assert_eq!(sorted_releases_vec, expected_releases_vec);
    assert_str_eq!(sort_option.name, "Quality");
  }

  fn release_vec() -> Vec<SonarrRelease> {
    let release_a = SonarrRelease {
      protocol: "Protocol A".to_owned(),
      age: 1,
      title: HorizontallyScrollableText::from("Title A"),
      indexer: "Indexer A".to_owned(),
      size: 1,
      rejected: true,
      seeders: Some(Number::from(1)),
      languages: Some(vec![Some(Language {
        id: 1,
        name: "Language A".to_owned(),
      })]),
      quality: QualityWrapper {
        quality: Quality {
          name: "Quality A".to_owned(),
        },
      },
      ..SonarrRelease::default()
    };
    let release_b = SonarrRelease {
      protocol: "Protocol B".to_owned(),
      age: 2,
      title: HorizontallyScrollableText::from("title B"),
      indexer: "indexer B".to_owned(),
      size: 2,
      rejected: false,
      seeders: Some(Number::from(2)),
      languages: Some(vec![Some(Language {
        id: 2,
        name: "Language B".to_owned(),
      })]),
      quality: QualityWrapper {
        quality: Quality {
          name: "Quality B".to_owned(),
        },
      },
      ..SonarrRelease::default()
    };
    let release_c = SonarrRelease {
      protocol: "Protocol C".to_owned(),
      age: 3,
      title: HorizontallyScrollableText::from("Title C"),
      indexer: "Indexer C".to_owned(),
      size: 3,
      rejected: false,
      seeders: None,
      languages: None,
      quality: QualityWrapper {
        quality: Quality {
          name: "Quality C".to_owned(),
        },
      },
      ..SonarrRelease::default()
    };

    vec![release_a, release_b, release_c]
  }
}
