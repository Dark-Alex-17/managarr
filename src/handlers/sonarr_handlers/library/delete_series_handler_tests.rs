#[cfg(test)]
mod tests {
  use pretty_assertions::assert_eq;
  use rstest::rstest;
  use strum::IntoEnumIterator;

  use crate::app::App;
  use crate::app::key_binding::DEFAULT_KEYBINDINGS;
  use crate::event::Key;
  use crate::handlers::KeyEventHandler;
  use crate::handlers::sonarr_handlers::library::delete_series_handler::DeleteSeriesHandler;
  use crate::handlers::sonarr_handlers::sonarr_handler_test_utils::utils::series;
  use crate::models::servarr_data::sonarr::sonarr_data::{ActiveSonarrBlock, DELETE_SERIES_BLOCKS};
  use crate::models::sonarr_models::DeleteSeriesParams;

  mod test_handle_scroll_up_and_down {
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    use crate::models::BlockSelectionState;
    use crate::models::servarr_data::sonarr::sonarr_data::DELETE_SERIES_SELECTION_BLOCKS;

    use super::*;

    #[rstest]
    fn test_delete_series_prompt_scroll(#[values(Key::Up, Key::Down)] key: Key) {
      let mut app = App::test_default();
      app.data.sonarr_data.selected_block =
        BlockSelectionState::new(DELETE_SERIES_SELECTION_BLOCKS);
      app.data.sonarr_data.selected_block.down();

      DeleteSeriesHandler::new(key, &mut app, ActiveSonarrBlock::DeleteSeriesPrompt, None).handle();

      if key == Key::Up {
        assert_eq!(
          app.data.sonarr_data.selected_block.get_active_block(),
          ActiveSonarrBlock::DeleteSeriesToggleDeleteFile
        );
      } else {
        assert_eq!(
          app.data.sonarr_data.selected_block.get_active_block(),
          ActiveSonarrBlock::DeleteSeriesConfirmPrompt
        );
      }
    }

    #[rstest]
    fn test_delete_series_prompt_scroll_no_op_when_not_ready(
      #[values(Key::Up, Key::Down)] key: Key,
    ) {
      let mut app = App::test_default();
      app.is_loading = true;
      app.data.sonarr_data.selected_block =
        BlockSelectionState::new(DELETE_SERIES_SELECTION_BLOCKS);
      app.data.sonarr_data.selected_block.down();

      DeleteSeriesHandler::new(key, &mut app, ActiveSonarrBlock::DeleteSeriesPrompt, None).handle();

      assert_eq!(
        app.data.sonarr_data.selected_block.get_active_block(),
        ActiveSonarrBlock::DeleteSeriesToggleAddListExclusion
      );
    }
  }

  mod test_handle_left_right_action {
    use rstest::rstest;

    use super::*;

    #[rstest]
    fn test_left_right_prompt_toggle(#[values(Key::Left, Key::Right)] key: Key) {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveSonarrBlock::DeleteSeriesPrompt.into());

      DeleteSeriesHandler::new(key, &mut app, ActiveSonarrBlock::DeleteSeriesPrompt, None).handle();

      assert!(app.data.sonarr_data.prompt_confirm);

      DeleteSeriesHandler::new(key, &mut app, ActiveSonarrBlock::DeleteSeriesPrompt, None).handle();

      assert!(!app.data.sonarr_data.prompt_confirm);
    }
  }

  mod test_handle_submit {
    use pretty_assertions::assert_eq;

    use crate::models::BlockSelectionState;
    use crate::models::servarr_data::sonarr::sonarr_data::DELETE_SERIES_SELECTION_BLOCKS;
    use crate::network::sonarr_network::SonarrEvent;

    use super::*;

    const SUBMIT_KEY: Key = DEFAULT_KEYBINDINGS.submit.key;

    #[test]
    fn test_delete_series_prompt_prompt_decline_submit() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveSonarrBlock::Series.into());
      app.push_navigation_stack(ActiveSonarrBlock::DeleteSeriesPrompt.into());
      app.data.sonarr_data.selected_block =
        BlockSelectionState::new(DELETE_SERIES_SELECTION_BLOCKS);
      app
        .data
        .sonarr_data
        .selected_block
        .set_index(0, DELETE_SERIES_SELECTION_BLOCKS.len() - 1);
      app.data.sonarr_data.delete_series_files = true;
      app.data.sonarr_data.add_list_exclusion = true;

      DeleteSeriesHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveSonarrBlock::DeleteSeriesPrompt,
        None,
      )
      .handle();

      assert_eq!(app.get_current_route(), ActiveSonarrBlock::Series.into());
      assert_eq!(app.data.sonarr_data.prompt_confirm_action, None);
      assert!(!app.data.sonarr_data.prompt_confirm);
      assert!(!app.data.sonarr_data.delete_series_files);
      assert!(!app.data.sonarr_data.add_list_exclusion);
    }

    #[test]
    fn test_delete_series_confirm_prompt_prompt_confirmation_submit() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveSonarrBlock::Series.into());
      app.push_navigation_stack(ActiveSonarrBlock::DeleteSeriesPrompt.into());
      app.data.sonarr_data.prompt_confirm = true;
      app.data.sonarr_data.delete_series_files = true;
      app.data.sonarr_data.add_list_exclusion = true;
      app.data.sonarr_data.series.set_items(vec![series()]);
      let expected_delete_series_params = DeleteSeriesParams {
        id: 1,
        delete_series_files: true,
        add_list_exclusion: true,
      };
      app.data.sonarr_data.selected_block =
        BlockSelectionState::new(DELETE_SERIES_SELECTION_BLOCKS);
      app
        .data
        .sonarr_data
        .selected_block
        .set_index(0, DELETE_SERIES_SELECTION_BLOCKS.len() - 1);

      DeleteSeriesHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveSonarrBlock::DeleteSeriesPrompt,
        None,
      )
      .handle();

      assert_eq!(app.get_current_route(), ActiveSonarrBlock::Series.into());
      assert_eq!(
        app.data.sonarr_data.prompt_confirm_action,
        Some(SonarrEvent::DeleteSeries(expected_delete_series_params))
      );
      assert!(app.should_refresh);
      assert!(app.data.sonarr_data.prompt_confirm);
      assert!(!app.data.sonarr_data.delete_series_files);
      assert!(!app.data.sonarr_data.add_list_exclusion);
    }

    #[test]
    fn test_delete_series_confirm_prompt_prompt_confirmation_submit_no_op_when_not_ready() {
      let mut app = App::test_default();
      app.is_loading = true;
      app.push_navigation_stack(ActiveSonarrBlock::Series.into());
      app.push_navigation_stack(ActiveSonarrBlock::DeleteSeriesPrompt.into());
      app.data.sonarr_data.prompt_confirm = true;
      app.data.sonarr_data.delete_series_files = true;
      app.data.sonarr_data.add_list_exclusion = true;

      DeleteSeriesHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveSonarrBlock::DeleteSeriesPrompt,
        None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveSonarrBlock::DeleteSeriesPrompt.into()
      );
      assert_eq!(app.data.sonarr_data.prompt_confirm_action, None);
      assert!(!app.should_refresh);
      assert!(app.data.sonarr_data.prompt_confirm);
      assert!(app.data.sonarr_data.delete_series_files);
      assert!(app.data.sonarr_data.add_list_exclusion);
    }

    #[test]
    fn test_delete_series_toggle_delete_files_submit() {
      let current_route = ActiveSonarrBlock::DeleteSeriesPrompt.into();
      let mut app = App::test_default();
      app.data.sonarr_data.selected_block =
        BlockSelectionState::new(DELETE_SERIES_SELECTION_BLOCKS);
      app.push_navigation_stack(ActiveSonarrBlock::DeleteSeriesPrompt.into());

      DeleteSeriesHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveSonarrBlock::DeleteSeriesPrompt,
        None,
      )
      .handle();

      assert_eq!(app.get_current_route(), current_route);
      assert_eq!(app.data.sonarr_data.delete_series_files, true);

      DeleteSeriesHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveSonarrBlock::DeleteSeriesPrompt,
        None,
      )
      .handle();

      assert_eq!(app.get_current_route(), current_route);
      assert_eq!(app.data.sonarr_data.delete_series_files, false);
    }
  }

  mod test_handle_esc {
    use super::*;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    const ESC_KEY: Key = DEFAULT_KEYBINDINGS.esc.key;

    #[rstest]
    fn test_delete_series_prompt_esc(#[values(true, false)] is_ready: bool) {
      let mut app = App::test_default();
      app.is_loading = is_ready;
      app.push_navigation_stack(ActiveSonarrBlock::Series.into());
      app.push_navigation_stack(ActiveSonarrBlock::DeleteSeriesPrompt.into());
      app.data.sonarr_data.prompt_confirm = true;
      app.data.sonarr_data.delete_series_files = true;
      app.data.sonarr_data.add_list_exclusion = true;

      DeleteSeriesHandler::new(
        ESC_KEY,
        &mut app,
        ActiveSonarrBlock::DeleteSeriesPrompt,
        None,
      )
      .handle();

      assert_eq!(app.get_current_route(), ActiveSonarrBlock::Series.into());
      assert!(!app.data.sonarr_data.prompt_confirm);
      assert!(!app.data.sonarr_data.delete_series_files);
      assert!(!app.data.sonarr_data.add_list_exclusion);
    }
  }

  mod test_handle_key_char {
    use crate::{
      models::{
        BlockSelectionState, servarr_data::sonarr::sonarr_data::DELETE_SERIES_SELECTION_BLOCKS,
      },
      network::sonarr_network::SonarrEvent,
    };
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_delete_series_confirm_prompt_prompt_confirm() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveSonarrBlock::Series.into());
      app.push_navigation_stack(ActiveSonarrBlock::DeleteSeriesPrompt.into());
      app.data.sonarr_data.delete_series_files = true;
      app.data.sonarr_data.add_list_exclusion = true;
      app.data.sonarr_data.series.set_items(vec![series()]);
      let expected_delete_series_params = DeleteSeriesParams {
        id: 1,
        delete_series_files: true,
        add_list_exclusion: true,
      };
      app.data.sonarr_data.selected_block =
        BlockSelectionState::new(DELETE_SERIES_SELECTION_BLOCKS);
      app
        .data
        .sonarr_data
        .selected_block
        .set_index(0, DELETE_SERIES_SELECTION_BLOCKS.len() - 1);

      DeleteSeriesHandler::new(
        DEFAULT_KEYBINDINGS.confirm.key,
        &mut app,
        ActiveSonarrBlock::DeleteSeriesPrompt,
        None,
      )
      .handle();

      assert_eq!(app.get_current_route(), ActiveSonarrBlock::Series.into());
      assert_eq!(
        app.data.sonarr_data.prompt_confirm_action,
        Some(SonarrEvent::DeleteSeries(expected_delete_series_params))
      );
      assert!(app.should_refresh);
      assert!(app.data.sonarr_data.prompt_confirm);
      assert!(!app.data.sonarr_data.delete_series_files);
      assert!(!app.data.sonarr_data.add_list_exclusion);
    }
  }

  #[test]
  fn test_delete_series_handler_accepts() {
    ActiveSonarrBlock::iter().for_each(|active_sonarr_block| {
      if DELETE_SERIES_BLOCKS.contains(&active_sonarr_block) {
        assert!(DeleteSeriesHandler::accepts(active_sonarr_block));
      } else {
        assert!(!DeleteSeriesHandler::accepts(active_sonarr_block));
      }
    });
  }

  #[rstest]
  fn test_delete_series_handler_ignore_special_keys(
    #[values(true, false)] ignore_special_keys_for_textbox_input: bool,
  ) {
    let mut app = App::test_default();
    app.ignore_special_keys_for_textbox_input = ignore_special_keys_for_textbox_input;
    let handler = DeleteSeriesHandler::new(
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
  fn test_build_delete_series_params() {
    let mut app = App::test_default();
    app.data.sonarr_data.series.set_items(vec![series()]);
    app.data.sonarr_data.delete_series_files = true;
    app.data.sonarr_data.add_list_exclusion = true;
    let expected_delete_series_params = DeleteSeriesParams {
      id: 1,
      delete_series_files: true,
      add_list_exclusion: true,
    };

    let delete_series_params = DeleteSeriesHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveSonarrBlock::DeleteSeriesPrompt,
      None,
    )
    .build_delete_series_params();

    assert_eq!(delete_series_params, expected_delete_series_params);
    assert!(!app.data.sonarr_data.delete_series_files);
    assert!(!app.data.sonarr_data.add_list_exclusion);
  }

  #[test]
  fn test_delete_series_handler_not_ready_when_loading() {
    let mut app = App::test_default();
    app.is_loading = true;

    let handler = DeleteSeriesHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveSonarrBlock::DeleteSeriesPrompt,
      None,
    );

    assert!(!handler.is_ready());
  }

  #[test]
  fn test_delete_series_handler_ready_when_not_loading() {
    let mut app = App::test_default();
    app.is_loading = false;

    let handler = DeleteSeriesHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveSonarrBlock::DeleteSeriesPrompt,
      None,
    );

    assert!(handler.is_ready());
  }
}
