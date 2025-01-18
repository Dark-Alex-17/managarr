#[cfg(test)]
mod tests {
  use crate::app::key_binding::DEFAULT_KEYBINDINGS;
  use crate::app::App;
  use crate::event::Key;
  use crate::handlers::sonarr_handlers::library::series_details_handler::SeriesDetailsHandler;
  use crate::handlers::sonarr_handlers::sonarr_handler_test_utils::utils::{season, series};
  use crate::handlers::KeyEventHandler;
  use crate::models::servarr_data::sonarr::sonarr_data::{
    ActiveSonarrBlock, SERIES_DETAILS_BLOCKS,
  };
  use crate::models::sonarr_models::Season;
  use crate::models::sonarr_models::SonarrHistoryItem;
  use crate::models::stateful_table::StatefulTable;
  use pretty_assertions::assert_eq;
  use rstest::rstest;
  use strum::IntoEnumIterator;

  mod test_handle_left_right_actions {
    use super::*;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    #[rstest]
    fn test_left_right_prompt_toggle(
      #[values(
        ActiveSonarrBlock::AutomaticallySearchSeriesPrompt,
        ActiveSonarrBlock::UpdateAndScanSeriesPrompt
      )]
      active_sonarr_block: ActiveSonarrBlock,
      #[values(Key::Left, Key::Right)] key: Key,
    ) {
      let mut app = App::default();
      app.push_navigation_stack(ActiveSonarrBlock::Series.into());

      SeriesDetailsHandler::new(key, &mut app, active_sonarr_block, None).handle();

      assert!(app.data.sonarr_data.prompt_confirm);

      SeriesDetailsHandler::new(key, &mut app, active_sonarr_block, None).handle();

      assert!(!app.data.sonarr_data.prompt_confirm);
    }

    #[rstest]
    #[case(ActiveSonarrBlock::SeriesDetails, ActiveSonarrBlock::SeriesHistory)]
    #[case(ActiveSonarrBlock::SeriesHistory, ActiveSonarrBlock::SeriesDetails)]
    fn test_series_details_tabs_left_right_action(
      #[case] left_block: ActiveSonarrBlock,
      #[case] right_block: ActiveSonarrBlock,
      #[values(true, false)] is_ready: bool,
    ) {
      let mut app = App::default();
      app.push_navigation_stack(ActiveSonarrBlock::Series.into());
      app.is_loading = is_ready;
      app.push_navigation_stack(right_block.into());
      app.data.sonarr_data.series_info_tabs.index = app
        .data
        .sonarr_data
        .series_info_tabs
        .tabs
        .iter()
        .position(|tab_route| tab_route.route == right_block.into())
        .unwrap_or_default();

      SeriesDetailsHandler::new(DEFAULT_KEYBINDINGS.left.key, &mut app, right_block, None)
        .handle();

      assert_eq!(
        app.get_current_route(),
        app.data.sonarr_data.series_info_tabs.get_active_route()
      );
      assert_eq!(app.get_current_route(), left_block.into());

      SeriesDetailsHandler::new(DEFAULT_KEYBINDINGS.right.key, &mut app, left_block, None)
        .handle();

      assert_eq!(
        app.get_current_route(),
        app.data.sonarr_data.series_info_tabs.get_active_route()
      );
      assert_eq!(app.get_current_route(), right_block.into());
    }
  }

  mod test_handle_submit {
    use pretty_assertions::assert_eq;

    use crate::extended_stateful_iterable_vec;
    use crate::network::sonarr_network::SonarrEvent;

    use super::*;

    const SUBMIT_KEY: Key = DEFAULT_KEYBINDINGS.submit.key;

    #[test]
    fn test_series_details_submit() {
      let mut app = App::default();
      app
        .data
        .sonarr_data
        .seasons
        .set_items(extended_stateful_iterable_vec!(Season, Option));

      SeriesDetailsHandler::new(SUBMIT_KEY, &mut app, ActiveSonarrBlock::SeriesDetails, None)
        .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveSonarrBlock::SeasonDetails.into()
      );
    }

    #[test]
    fn test_series_details_submit_no_op_on_empty_seasons_table() {
      let mut app = App::default();
      app.push_navigation_stack(ActiveSonarrBlock::SeriesDetails.into());

      SeriesDetailsHandler::new(SUBMIT_KEY, &mut app, ActiveSonarrBlock::SeriesDetails, None)
        .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveSonarrBlock::SeriesDetails.into()
      );
    }

    #[test]
    fn test_series_details_submit_no_op_when_not_ready() {
      let mut app = App::default();
      app.is_loading = true;
      app.push_navigation_stack(ActiveSonarrBlock::Series.into());

      SeriesDetailsHandler::new(SUBMIT_KEY, &mut app, ActiveSonarrBlock::SeriesDetails, None)
        .handle();

      assert_eq!(app.get_current_route(), ActiveSonarrBlock::Series.into());
    }

    #[test]
    fn test_series_history_submit() {
      let mut app = App::default();
      let mut series_history = StatefulTable::default();
      series_history.set_items(vec![SonarrHistoryItem::default()]);
      app.data.sonarr_data.series_history = Some(series_history);

      SeriesDetailsHandler::new(SUBMIT_KEY, &mut app, ActiveSonarrBlock::SeriesHistory, None)
        .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveSonarrBlock::SeriesHistoryDetails.into()
      );
    }

    #[test]
    fn test_series_history_submit_no_op_when_series_history_is_empty() {
      let mut app = App::default();
      app.push_navigation_stack(ActiveSonarrBlock::SeriesHistory.into());
      app.data.sonarr_data.series_history = Some(StatefulTable::default());

      SeriesDetailsHandler::new(SUBMIT_KEY, &mut app, ActiveSonarrBlock::SeriesHistory, None)
        .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveSonarrBlock::SeriesHistory.into()
      );
    }

    #[test]
    fn test_series_history_submit_no_op_when_not_ready() {
      let mut app = App::default();
      app.is_loading = true;
      app.push_navigation_stack(ActiveSonarrBlock::Series.into());

      SeriesDetailsHandler::new(SUBMIT_KEY, &mut app, ActiveSonarrBlock::SeriesHistory, None)
        .handle();

      assert_eq!(app.get_current_route(), ActiveSonarrBlock::Series.into());
    }

    #[rstest]
    #[case(
      ActiveSonarrBlock::AutomaticallySearchSeriesPrompt,
      SonarrEvent::TriggerAutomaticSeriesSearch(1)
    )]
    #[case(
      ActiveSonarrBlock::UpdateAndScanSeriesPrompt,
      SonarrEvent::UpdateAndScanSeries(1)
    )]
    fn test_series_details_prompt_confirm_submit(
      #[case] prompt_block: ActiveSonarrBlock,
      #[case] expected_action: SonarrEvent,
    ) {
      let mut app = App::default();
      app.data.sonarr_data.prompt_confirm = true;
      app.data.sonarr_data.series.set_items(vec![series()]);
      app.push_navigation_stack(ActiveSonarrBlock::SeriesDetails.into());
      app.push_navigation_stack(prompt_block.into());

      SeriesDetailsHandler::new(SUBMIT_KEY, &mut app, prompt_block, None).handle();

      assert!(app.data.sonarr_data.prompt_confirm);
      assert_eq!(
        app.get_current_route(),
        ActiveSonarrBlock::SeriesDetails.into()
      );
      assert_eq!(
        app.data.sonarr_data.prompt_confirm_action,
        Some(expected_action)
      );
    }

    #[rstest]
    fn test_series_details_prompt_decline_submit(
      #[values(
        ActiveSonarrBlock::AutomaticallySearchSeriesPrompt,
        ActiveSonarrBlock::UpdateAndScanSeriesPrompt
      )]
      prompt_block: ActiveSonarrBlock,
    ) {
      let mut app = App::default();
      app.push_navigation_stack(ActiveSonarrBlock::SeriesDetails.into());
      app.push_navigation_stack(prompt_block.into());

      SeriesDetailsHandler::new(SUBMIT_KEY, &mut app, prompt_block, None).handle();

      assert!(!app.data.sonarr_data.prompt_confirm);
      assert_eq!(
        app.get_current_route(),
        ActiveSonarrBlock::SeriesDetails.into()
      );
      assert_eq!(app.data.sonarr_data.prompt_confirm_action, None);
    }
  }

  mod test_handle_esc {
    use super::*;
    use crate::models::stateful_table::StatefulTable;
    use pretty_assertions::assert_eq;
    use ratatui::widgets::TableState;

    const ESC_KEY: Key = DEFAULT_KEYBINDINGS.esc.key;

    #[test]
    fn test_series_history_details_block_esc() {
      let mut app = App::default();
      app.push_navigation_stack(ActiveSonarrBlock::SeriesHistory.into());
      app.push_navigation_stack(ActiveSonarrBlock::SeriesHistoryDetails.into());

      SeriesDetailsHandler::new(
        ESC_KEY,
        &mut app,
        ActiveSonarrBlock::SeriesHistoryDetails,
        None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveSonarrBlock::SeriesHistory.into()
      );
    }

    #[rstest]
    fn test_series_details_prompts_esc(
      #[values(
        ActiveSonarrBlock::AutomaticallySearchSeriesPrompt,
        ActiveSonarrBlock::UpdateAndScanSeriesPrompt
      )]
      prompt_block: ActiveSonarrBlock,
      #[values(true, false)] is_ready: bool,
    ) {
      let mut app = App::default();
      app.is_loading = is_ready;
      app.data.sonarr_data.prompt_confirm = true;
      app.push_navigation_stack(ActiveSonarrBlock::SeriesDetails.into());
      app.push_navigation_stack(prompt_block.into());

      SeriesDetailsHandler::new(ESC_KEY, &mut app, prompt_block, None).handle();

      assert!(!app.data.sonarr_data.prompt_confirm);
      assert_eq!(
        app.get_current_route(),
        ActiveSonarrBlock::SeriesDetails.into()
      );
    }

    #[test]
    fn test_series_history_esc_resets_filter_if_one_is_set_instead_of_closing_the_window() {
      let mut app = App::default();
      let series_history = StatefulTable {
        filter: Some("Test".into()),
        filtered_items: Some(vec![SonarrHistoryItem::default()]),
        filtered_state: Some(TableState::default()),
        ..StatefulTable::default()
      };
      app.data.sonarr_data.series_history = Some(series_history);
      app.push_navigation_stack(ActiveSonarrBlock::Series.into());
      app.push_navigation_stack(ActiveSonarrBlock::SeriesHistory.into());

      SeriesDetailsHandler::new(ESC_KEY, &mut app, ActiveSonarrBlock::SeriesHistory, None)
        .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveSonarrBlock::SeriesHistory.into()
      );
      assert!(app
        .data
        .sonarr_data
        .series_history
        .as_ref()
        .unwrap()
        .filter
        .is_none());
      assert!(app
        .data
        .sonarr_data
        .series_history
        .as_ref()
        .unwrap()
        .filtered_items
        .is_none());
      assert!(app
        .data
        .sonarr_data
        .series_history
        .as_ref()
        .unwrap()
        .filtered_state
        .is_none());
    }
  }

  mod test_handle_key_char {
    use super::*;
    use crate::models::servarr_data::sonarr::sonarr_data::sonarr_test_utils::utils::create_test_sonarr_data;
    use crate::models::servarr_data::sonarr::sonarr_data::SonarrData;
    use crate::models::sonarr_models::{Series, SeriesType};
    use crate::network::sonarr_network::SonarrEvent;
    use crate::test_edit_series_key;
    use pretty_assertions::{assert_eq, assert_str_eq};
    use serde_json::Number;
    use strum::IntoEnumIterator;

    #[rstest]
    fn test_series_details_edit_key(
      #[values(ActiveSonarrBlock::SeriesDetails, ActiveSonarrBlock::SeriesHistory)]
      active_sonarr_block: ActiveSonarrBlock,
    ) {
      test_edit_series_key!(
        SeriesDetailsHandler,
        active_sonarr_block,
        active_sonarr_block
      );
    }

    #[rstest]
    fn test_series_edit_key_no_op_when_not_ready(
      #[values(ActiveSonarrBlock::SeriesDetails, ActiveSonarrBlock::SeriesHistory)]
      active_sonarr_block: ActiveSonarrBlock,
    ) {
      let mut app = App::default();
      app.is_loading = true;
      app.push_navigation_stack(active_sonarr_block.into());

      SeriesDetailsHandler::new(
        DEFAULT_KEYBINDINGS.edit.key,
        &mut app,
        active_sonarr_block,
        None,
      )
      .handle();

      assert_eq!(app.get_current_route(), active_sonarr_block.into());
      assert!(app.data.sonarr_data.edit_series_modal.is_none());
    }

    #[test]
    fn test_toggle_monitoring_key() {
      let mut app = App::default();
      app.data.sonarr_data = create_test_sonarr_data();
      app.push_navigation_stack(ActiveSonarrBlock::SeriesDetails.into());
      app.is_routing = false;

      SeriesDetailsHandler::new(
        DEFAULT_KEYBINDINGS.toggle_monitoring.key,
        &mut app,
        ActiveSonarrBlock::SeriesDetails,
        None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveSonarrBlock::SeriesDetails.into()
      );
      assert!(app.data.sonarr_data.prompt_confirm);
      assert!(app.is_routing);
      assert_eq!(
        app.data.sonarr_data.prompt_confirm_action,
        Some(SonarrEvent::ToggleSeasonMonitoring((0, 0)))
      );
    }

    #[test]
    fn test_toggle_monitoring_key_no_op_when_not_ready() {
      let mut app = App::default();
      app.is_loading = true;
      app.push_navigation_stack(ActiveSonarrBlock::SeriesDetails.into());
      app.is_routing = false;

      SeriesDetailsHandler::new(
        DEFAULT_KEYBINDINGS.toggle_monitoring.key,
        &mut app,
        ActiveSonarrBlock::SeriesDetails,
        None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveSonarrBlock::SeriesDetails.into()
      );
      assert!(!app.data.sonarr_data.prompt_confirm);
      assert!(app.data.sonarr_data.prompt_confirm_action.is_none());
      assert!(!app.is_routing);
    }

    #[rstest]
    fn test_auto_search_key(
      #[values(ActiveSonarrBlock::SeriesDetails, ActiveSonarrBlock::SeriesHistory)]
      active_sonarr_block: ActiveSonarrBlock,
    ) {
      let mut app = App::default();
      let mut series_history = StatefulTable::default();
      series_history.set_items(vec![SonarrHistoryItem::default()]);
      app.data.sonarr_data.series_history = Some(series_history);
      app.push_navigation_stack(active_sonarr_block.into());

      SeriesDetailsHandler::new(
        DEFAULT_KEYBINDINGS.auto_search.key,
        &mut app,
        active_sonarr_block,
        None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveSonarrBlock::AutomaticallySearchSeriesPrompt.into()
      );
    }

    #[rstest]
    fn test_auto_search_key_no_op_when_not_ready(
      #[values(ActiveSonarrBlock::SeriesDetails, ActiveSonarrBlock::SeriesHistory)]
      active_sonarr_block: ActiveSonarrBlock,
    ) {
      let mut app = App::default();
      app.is_loading = true;
      app.push_navigation_stack(active_sonarr_block.into());

      SeriesDetailsHandler::new(
        DEFAULT_KEYBINDINGS.auto_search.key,
        &mut app,
        active_sonarr_block,
        None,
      )
      .handle();

      assert_eq!(app.get_current_route(), active_sonarr_block.into());
    }

    #[rstest]
    fn test_update_key(
      #[values(ActiveSonarrBlock::SeriesDetails, ActiveSonarrBlock::SeriesHistory)]
      active_sonarr_block: ActiveSonarrBlock,
    ) {
      let mut app = App::default();
      let mut series_history = StatefulTable::default();
      series_history.set_items(vec![SonarrHistoryItem::default()]);
      app.data.sonarr_data.series_history = Some(series_history);
      app.push_navigation_stack(active_sonarr_block.into());

      SeriesDetailsHandler::new(
        DEFAULT_KEYBINDINGS.update.key,
        &mut app,
        active_sonarr_block,
        None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveSonarrBlock::UpdateAndScanSeriesPrompt.into()
      );
    }

    #[rstest]
    fn test_update_key_no_op_when_not_ready(
      #[values(ActiveSonarrBlock::SeriesDetails, ActiveSonarrBlock::SeriesHistory)]
      active_sonarr_block: ActiveSonarrBlock,
    ) {
      let mut app = App::default();
      app.is_loading = true;
      app.push_navigation_stack(active_sonarr_block.into());

      SeriesDetailsHandler::new(
        DEFAULT_KEYBINDINGS.update.key,
        &mut app,
        active_sonarr_block,
        None,
      )
      .handle();

      assert_eq!(app.get_current_route(), active_sonarr_block.into());
    }

    #[rstest]
    fn test_refresh_key(
      #[values(ActiveSonarrBlock::SeriesDetails, ActiveSonarrBlock::SeriesHistory)]
      active_sonarr_block: ActiveSonarrBlock,
    ) {
      let mut app = App::default();
      let mut series_history = StatefulTable::default();
      series_history.set_items(vec![SonarrHistoryItem::default()]);
      app.data.sonarr_data.series_history = Some(series_history);
      app.push_navigation_stack(active_sonarr_block.into());
      app.is_routing = false;

      SeriesDetailsHandler::new(
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
      #[values(ActiveSonarrBlock::SeriesDetails, ActiveSonarrBlock::SeriesHistory)]
      active_sonarr_block: ActiveSonarrBlock,
    ) {
      let mut app = App::default();
      app.is_loading = true;
      app.push_navigation_stack(active_sonarr_block.into());
      app.is_routing = false;

      SeriesDetailsHandler::new(
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
      ActiveSonarrBlock::AutomaticallySearchSeriesPrompt,
      SonarrEvent::TriggerAutomaticSeriesSearch(1)
    )]
    #[case(
      ActiveSonarrBlock::UpdateAndScanSeriesPrompt,
      SonarrEvent::UpdateAndScanSeries(1)
    )]
    fn test_series_details_prompt_confirm_confirm_key(
      #[case] prompt_block: ActiveSonarrBlock,
      #[case] expected_action: SonarrEvent,
      #[values(ActiveSonarrBlock::SeriesDetails, ActiveSonarrBlock::SeriesHistory)]
      active_sonarr_block: ActiveSonarrBlock,
    ) {
      let mut app = App::default();
      app.data.sonarr_data.prompt_confirm = true;
      app.data.sonarr_data.series.set_items(vec![series()]);
      app.push_navigation_stack(active_sonarr_block.into());
      app.push_navigation_stack(prompt_block.into());

      SeriesDetailsHandler::new(
        DEFAULT_KEYBINDINGS.confirm.key,
        &mut app,
        prompt_block,
        None,
      )
      .handle();

      assert!(app.data.sonarr_data.prompt_confirm);
      assert_eq!(app.get_current_route(), active_sonarr_block.into());
      assert_eq!(
        app.data.sonarr_data.prompt_confirm_action,
        Some(expected_action)
      );
    }
  }

  #[test]
  fn test_series_details_handler_accepts() {
    ActiveSonarrBlock::iter().for_each(|active_sonarr_block| {
      if SERIES_DETAILS_BLOCKS.contains(&active_sonarr_block) {
        assert!(SeriesDetailsHandler::accepts(active_sonarr_block));
      } else {
        assert!(!SeriesDetailsHandler::accepts(active_sonarr_block));
      }
    });
  }

  #[test]
  fn test_extract_series_id_season_number_tuple() {
    let mut app = App::default();
    app.data.sonarr_data.series.set_items(vec![series()]);
    app.data.sonarr_data.seasons.set_items(vec![season()]);

    let series_id_season_number_tuple = SeriesDetailsHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveSonarrBlock::SeriesDetails,
      None,
    )
    .extract_series_id_season_number_tuple();

    assert_eq!(series_id_season_number_tuple, (1, 1));
  }

  #[test]
  fn test_extract_series_id() {
    let mut app = App::default();
    app.data.sonarr_data.series.set_items(vec![series()]);

    let series_id = SeriesDetailsHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveSonarrBlock::SeriesDetails,
      None,
    )
    .extract_series_id();

    assert_eq!(series_id, 1);
  }

  #[test]
  fn test_series_details_handler_is_not_ready_when_loading() {
    let mut app = App::default();
    app.push_navigation_stack(ActiveSonarrBlock::Series.into());
    app.is_loading = true;

    let handler = SeriesDetailsHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveSonarrBlock::SeriesDetails,
      None,
    );

    assert!(!handler.is_ready());
  }

  #[test]
  fn test_series_details_handler_is_not_ready_when_not_loading_and_series_history_is_none() {
    let mut app = App::default();
    app.push_navigation_stack(ActiveSonarrBlock::Series.into());

    let handler = SeriesDetailsHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveSonarrBlock::SeriesHistory,
      None,
    );

    assert!(!handler.is_ready());
  }

  #[test]
  fn test_series_details_handler_ready_when_not_loading_and_series_history_is_some() {
    let mut app = App::default();
    app.push_navigation_stack(ActiveSonarrBlock::Series.into());
    app.data.sonarr_data.series_history = Some(StatefulTable::default());

    let handler = SeriesDetailsHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveSonarrBlock::SeriesHistory,
      None,
    );

    assert!(handler.is_ready());
  }

  #[test]
  fn test_series_details_handler_ready_when_not_loading_for_series_details() {
    let mut app = App::default();
    app.push_navigation_stack(ActiveSonarrBlock::Series.into());

    let handler = SeriesDetailsHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveSonarrBlock::SeriesDetails,
      None,
    );

    assert!(handler.is_ready());
  }
}
