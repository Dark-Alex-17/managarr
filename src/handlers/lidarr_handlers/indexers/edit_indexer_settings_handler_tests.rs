#[cfg(test)]
mod tests {
  use pretty_assertions::assert_eq;
  use rstest::rstest;
  use strum::IntoEnumIterator;

  use crate::app::App;
  use crate::app::key_binding::DEFAULT_KEYBINDINGS;
  use crate::assert_modal_absent;
  use crate::assert_navigation_pushed;
  use crate::event::Key;
  use crate::handlers::KeyEventHandler;
  use crate::handlers::lidarr_handlers::indexers::edit_indexer_settings_handler::IndexerSettingsHandler;
  use crate::models::servarr_data::lidarr::lidarr_data::{
    ActiveLidarrBlock, INDEXER_SETTINGS_BLOCKS,
  };
  use crate::models::servarr_models::IndexerSettings;
  use crate::network::lidarr_network::lidarr_network_test_utils::test_utils::indexer_settings;

  mod test_handle_scroll_up_and_down {
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    use crate::models::BlockSelectionState;
    use crate::models::servarr_data::lidarr::lidarr_data::INDEXER_SETTINGS_SELECTION_BLOCKS;
    use crate::models::servarr_models::IndexerSettings;

    use super::*;

    macro_rules! test_i64_counter_scroll_value {
      ($block:expr, $key:expr, $data_ref:ident, $negatives:literal) => {
        let mut app = App::test_default();
        app.push_navigation_stack(ActiveLidarrBlock::Indexers.into());
        app.data.lidarr_data.indexer_settings = Some(IndexerSettings::default());

        IndexerSettingsHandler::new($key, &mut app, $block, None).handle();

        if $key == Key::Up {
          assert_eq!(
            app
              .data
              .lidarr_data
              .indexer_settings
              .as_ref()
              .unwrap()
              .$data_ref,
            1
          );
        } else {
          if $negatives {
            assert_eq!(
              app
                .data
                .lidarr_data
                .indexer_settings
                .as_ref()
                .unwrap()
                .$data_ref,
              -1
            );
          } else {
            assert_eq!(
              app
                .data
                .lidarr_data
                .indexer_settings
                .as_ref()
                .unwrap()
                .$data_ref,
              0
            );

            IndexerSettingsHandler::new(Key::Up, &mut app, $block, None).handle();

            assert_eq!(
              app
                .data
                .lidarr_data
                .indexer_settings
                .as_ref()
                .unwrap()
                .$data_ref,
              1
            );

            IndexerSettingsHandler::new($key, &mut app, $block, None).handle();
            assert_eq!(
              app
                .data
                .lidarr_data
                .indexer_settings
                .as_ref()
                .unwrap()
                .$data_ref,
              0
            );
          }
        }
      };
    }

    #[rstest]
    fn test_edit_indexer_settings_prompt_scroll(#[values(Key::Up, Key::Down)] key: Key) {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveLidarrBlock::Indexers.into());
      app.data.lidarr_data.indexer_settings = Some(IndexerSettings::default());
      app.data.lidarr_data.selected_block =
        BlockSelectionState::new(INDEXER_SETTINGS_SELECTION_BLOCKS);
      app.data.lidarr_data.selected_block.down();

      IndexerSettingsHandler::new(
        key,
        &mut app,
        ActiveLidarrBlock::AllIndexerSettingsPrompt,
        None,
      )
      .handle();

      if key == Key::Up {
        assert_eq!(
          app.data.lidarr_data.selected_block.get_active_block(),
          ActiveLidarrBlock::IndexerSettingsMinimumAgeInput
        );
      } else {
        assert_eq!(
          app.data.lidarr_data.selected_block.get_active_block(),
          ActiveLidarrBlock::IndexerSettingsMaximumSizeInput
        );
      }
    }

    #[rstest]
    fn test_edit_indexer_settings_prompt_scroll_no_op_when_not_ready(
      #[values(Key::Up, Key::Down)] key: Key,
    ) {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveLidarrBlock::Indexers.into());
      app.is_loading = true;
      app.data.lidarr_data.indexer_settings = Some(IndexerSettings::default());
      app.data.lidarr_data.selected_block =
        BlockSelectionState::new(INDEXER_SETTINGS_SELECTION_BLOCKS);
      app.data.lidarr_data.selected_block.down();

      IndexerSettingsHandler::new(
        key,
        &mut app,
        ActiveLidarrBlock::AllIndexerSettingsPrompt,
        None,
      )
      .handle();

      assert_eq!(
        app.data.lidarr_data.selected_block.get_active_block(),
        ActiveLidarrBlock::IndexerSettingsRetentionInput
      );
    }

    #[rstest]
    fn test_edit_indexer_settings_minimum_age_scroll(#[values(Key::Up, Key::Down)] key: Key) {
      test_i64_counter_scroll_value!(
        ActiveLidarrBlock::IndexerSettingsMinimumAgeInput,
        key,
        minimum_age,
        false
      );
    }

    #[rstest]
    fn test_edit_indexer_settings_retention_scroll(#[values(Key::Up, Key::Down)] key: Key) {
      test_i64_counter_scroll_value!(
        ActiveLidarrBlock::IndexerSettingsRetentionInput,
        key,
        retention,
        false
      );
    }

    #[rstest]
    fn test_edit_indexer_settings_maximum_size_scroll(#[values(Key::Up, Key::Down)] key: Key) {
      test_i64_counter_scroll_value!(
        ActiveLidarrBlock::IndexerSettingsMaximumSizeInput,
        key,
        maximum_size,
        false
      );
    }

    #[rstest]
    fn test_edit_indexer_settings_rss_sync_interval_scroll(#[values(Key::Up, Key::Down)] key: Key) {
      test_i64_counter_scroll_value!(
        ActiveLidarrBlock::IndexerSettingsRssSyncIntervalInput,
        key,
        rss_sync_interval,
        false
      );
    }
  }

  mod test_handle_left_right_action {
    use crate::models::servarr_data::lidarr::lidarr_data::INDEXER_SETTINGS_SELECTION_BLOCKS;

    use crate::models::BlockSelectionState;
    use rstest::rstest;

    use super::*;

    #[rstest]
    fn test_left_right_prompt_toggle(#[values(Key::Left, Key::Right)] key: Key) {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveLidarrBlock::Indexers.into());
      app.data.lidarr_data.selected_block =
        BlockSelectionState::new(INDEXER_SETTINGS_SELECTION_BLOCKS);
      app.data.lidarr_data.selected_block.y = INDEXER_SETTINGS_SELECTION_BLOCKS.len() - 1;

      IndexerSettingsHandler::new(
        key,
        &mut app,
        ActiveLidarrBlock::AllIndexerSettingsPrompt,
        None,
      )
      .handle();

      assert!(app.data.lidarr_data.prompt_confirm);

      IndexerSettingsHandler::new(
        key,
        &mut app,
        ActiveLidarrBlock::AllIndexerSettingsPrompt,
        None,
      )
      .handle();

      assert!(!app.data.lidarr_data.prompt_confirm);
    }
  }

  mod test_handle_submit {
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    use crate::{
      assert_navigation_popped,
      models::{
        BlockSelectionState, servarr_data::lidarr::lidarr_data::INDEXER_SETTINGS_SELECTION_BLOCKS,
        servarr_models::IndexerSettings,
      },
      network::lidarr_network::LidarrEvent,
    };

    use super::*;

    const SUBMIT_KEY: Key = DEFAULT_KEYBINDINGS.submit.key;

    #[test]
    fn test_edit_indexer_settings_prompt_prompt_decline_submit() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveLidarrBlock::Indexers.into());
      app.push_navigation_stack(ActiveLidarrBlock::AllIndexerSettingsPrompt.into());
      app.data.lidarr_data.selected_block =
        BlockSelectionState::new(INDEXER_SETTINGS_SELECTION_BLOCKS);
      app
        .data
        .lidarr_data
        .selected_block
        .set_index(0, INDEXER_SETTINGS_SELECTION_BLOCKS.len() - 1);
      app.data.lidarr_data.indexer_settings = Some(IndexerSettings::default());

      IndexerSettingsHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveLidarrBlock::AllIndexerSettingsPrompt,
        None,
      )
      .handle();

      assert_navigation_popped!(app, ActiveLidarrBlock::Indexers.into());
      assert_none!(app.data.lidarr_data.prompt_confirm_action);
      assert!(!app.should_refresh);
      assert_none!(app.data.lidarr_data.indexer_settings);
    }

    #[test]
    fn test_edit_indexer_settings_prompt_prompt_confirmation_submit() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveLidarrBlock::Indexers.into());
      app.push_navigation_stack(ActiveLidarrBlock::AllIndexerSettingsPrompt.into());
      app.data.lidarr_data.selected_block =
        BlockSelectionState::new(INDEXER_SETTINGS_SELECTION_BLOCKS);
      app
        .data
        .lidarr_data
        .selected_block
        .set_index(0, INDEXER_SETTINGS_SELECTION_BLOCKS.len() - 1);
      app.data.lidarr_data.indexer_settings = Some(indexer_settings());
      app.data.lidarr_data.prompt_confirm = true;

      IndexerSettingsHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveLidarrBlock::AllIndexerSettingsPrompt,
        None,
      )
      .handle();

      assert_navigation_popped!(app, ActiveLidarrBlock::Indexers.into());
      assert_some_eq_x!(
        &app.data.lidarr_data.prompt_confirm_action,
        &LidarrEvent::EditAllIndexerSettings(indexer_settings())
      );
      assert_modal_absent!(app.data.lidarr_data.indexer_settings);
      assert!(app.should_refresh);
    }

    #[test]
    fn test_edit_indexer_settings_prompt_prompt_confirmation_submit_no_op_when_not_ready() {
      let mut app = App::test_default();
      app.is_loading = true;
      app.push_navigation_stack(ActiveLidarrBlock::Indexers.into());
      app.push_navigation_stack(ActiveLidarrBlock::AllIndexerSettingsPrompt.into());
      app.data.lidarr_data.indexer_settings = Some(IndexerSettings::default());
      app.data.lidarr_data.prompt_confirm = true;

      IndexerSettingsHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveLidarrBlock::AllIndexerSettingsPrompt,
        None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveLidarrBlock::AllIndexerSettingsPrompt.into()
      );
      assert!(!app.should_refresh);
    }

    #[rstest]
    #[case(ActiveLidarrBlock::IndexerSettingsMinimumAgeInput, 0)]
    #[case(ActiveLidarrBlock::IndexerSettingsRetentionInput, 1)]
    #[case(ActiveLidarrBlock::IndexerSettingsMaximumSizeInput, 2)]
    #[case(ActiveLidarrBlock::IndexerSettingsRssSyncIntervalInput, 3)]
    fn test_edit_indexer_settings_prompt_submit_selected_block(
      #[case] selected_block: ActiveLidarrBlock,
      #[case] y_index: usize,
    ) {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveLidarrBlock::Indexers.into());
      app.data.lidarr_data.indexer_settings = Some(IndexerSettings::default());
      app.push_navigation_stack(ActiveLidarrBlock::AllIndexerSettingsPrompt.into());
      app.data.lidarr_data.selected_block =
        BlockSelectionState::new(INDEXER_SETTINGS_SELECTION_BLOCKS);
      app.data.lidarr_data.selected_block.set_index(0, y_index);

      IndexerSettingsHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveLidarrBlock::AllIndexerSettingsPrompt,
        None,
      )
      .handle();

      assert_navigation_pushed!(app, selected_block.into());
    }

    #[rstest]
    fn test_edit_indexer_settings_prompt_submit_selected_block_no_op_when_not_ready(
      #[values(0, 1, 2, 3, 4)] y_index: usize,
    ) {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveLidarrBlock::Indexers.into());
      app.is_loading = true;
      app.data.lidarr_data.indexer_settings = Some(IndexerSettings::default());
      app.push_navigation_stack(ActiveLidarrBlock::AllIndexerSettingsPrompt.into());
      app.data.lidarr_data.selected_block =
        BlockSelectionState::new(INDEXER_SETTINGS_SELECTION_BLOCKS);
      app.data.lidarr_data.selected_block.set_index(0, y_index);

      IndexerSettingsHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveLidarrBlock::AllIndexerSettingsPrompt,
        None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveLidarrBlock::AllIndexerSettingsPrompt.into()
      );
    }

    #[rstest]
    fn test_edit_indexer_settings_selected_block_submit(
      #[values(
        ActiveLidarrBlock::IndexerSettingsMinimumAgeInput,
        ActiveLidarrBlock::IndexerSettingsRetentionInput,
        ActiveLidarrBlock::IndexerSettingsMaximumSizeInput,
        ActiveLidarrBlock::IndexerSettingsRssSyncIntervalInput
      )]
      active_lidarr_block: ActiveLidarrBlock,
    ) {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveLidarrBlock::Indexers.into());
      app.data.lidarr_data.indexer_settings = Some(IndexerSettings::default());
      app.push_navigation_stack(ActiveLidarrBlock::AllIndexerSettingsPrompt.into());
      app.push_navigation_stack(active_lidarr_block.into());

      IndexerSettingsHandler::new(SUBMIT_KEY, &mut app, active_lidarr_block, None).handle();

      assert_navigation_popped!(app, ActiveLidarrBlock::AllIndexerSettingsPrompt.into());
    }
  }

  mod test_handle_esc {
    use rstest::rstest;

    use crate::models::servarr_models::IndexerSettings;

    use super::*;
    use crate::assert_navigation_popped;

    const ESC_KEY: Key = DEFAULT_KEYBINDINGS.esc.key;

    #[rstest]
    fn test_edit_indexer_settings_prompt_esc(#[values(true, false)] is_ready: bool) {
      let mut app = App::test_default();
      app.is_loading = is_ready;
      app.push_navigation_stack(ActiveLidarrBlock::Indexers.into());
      app.push_navigation_stack(ActiveLidarrBlock::AllIndexerSettingsPrompt.into());
      app.data.lidarr_data.indexer_settings = Some(IndexerSettings::default());

      IndexerSettingsHandler::new(
        ESC_KEY,
        &mut app,
        ActiveLidarrBlock::AllIndexerSettingsPrompt,
        None,
      )
      .handle();

      assert_navigation_popped!(app, ActiveLidarrBlock::Indexers.into());
      assert!(!app.data.lidarr_data.prompt_confirm);
      assert_none!(app.data.lidarr_data.indexer_settings);
    }

    #[rstest]
    fn test_edit_indexer_settings_selected_blocks_esc(
      #[values(
        ActiveLidarrBlock::IndexerSettingsMinimumAgeInput,
        ActiveLidarrBlock::IndexerSettingsRetentionInput,
        ActiveLidarrBlock::IndexerSettingsMaximumSizeInput,
        ActiveLidarrBlock::IndexerSettingsRssSyncIntervalInput
      )]
      active_lidarr_block: ActiveLidarrBlock,
    ) {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveLidarrBlock::Indexers.into());
      app.push_navigation_stack(active_lidarr_block.into());
      app.data.lidarr_data.indexer_settings = Some(IndexerSettings::default());

      IndexerSettingsHandler::new(ESC_KEY, &mut app, active_lidarr_block, None).handle();

      assert_navigation_popped!(app, ActiveLidarrBlock::Indexers.into());
      assert_some_eq_x!(
        &app.data.lidarr_data.indexer_settings,
        &IndexerSettings::default()
      );
    }
  }

  mod test_handle_key_char {
    use crate::{
      assert_navigation_popped,
      models::{
        BlockSelectionState, servarr_data::lidarr::lidarr_data::INDEXER_SETTINGS_SELECTION_BLOCKS,
      },
      network::lidarr_network::LidarrEvent,
    };

    use super::*;

    #[test]
    fn test_edit_indexer_settings_prompt_prompt_confirmation_confirm() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveLidarrBlock::Indexers.into());
      app.push_navigation_stack(ActiveLidarrBlock::AllIndexerSettingsPrompt.into());
      app.data.lidarr_data.selected_block =
        BlockSelectionState::new(INDEXER_SETTINGS_SELECTION_BLOCKS);
      app
        .data
        .lidarr_data
        .selected_block
        .set_index(0, INDEXER_SETTINGS_SELECTION_BLOCKS.len() - 1);
      app.data.lidarr_data.indexer_settings = Some(indexer_settings());

      IndexerSettingsHandler::new(
        DEFAULT_KEYBINDINGS.confirm.key,
        &mut app,
        ActiveLidarrBlock::AllIndexerSettingsPrompt,
        None,
      )
      .handle();

      assert_navigation_popped!(app, ActiveLidarrBlock::Indexers.into());
      assert_some_eq_x!(
        &app.data.lidarr_data.prompt_confirm_action,
        &LidarrEvent::EditAllIndexerSettings(indexer_settings())
      );
      assert_modal_absent!(app.data.lidarr_data.indexer_settings);
      assert!(app.should_refresh);
    }
  }

  #[test]
  fn test_indexer_settings_handler_accepts() {
    ActiveLidarrBlock::iter().for_each(|active_lidarr_block| {
      if INDEXER_SETTINGS_BLOCKS.contains(&active_lidarr_block) {
        assert!(IndexerSettingsHandler::accepts(active_lidarr_block));
      } else {
        assert!(!IndexerSettingsHandler::accepts(active_lidarr_block));
      }
    })
  }

  #[rstest]
  fn test_indexer_settings_handler_ignore_special_keys(
    #[values(true, false)] ignore_special_keys_for_textbox_input: bool,
  ) {
    let mut app = App::test_default();
    app.ignore_special_keys_for_textbox_input = ignore_special_keys_for_textbox_input;
    let handler = IndexerSettingsHandler::new(
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
  fn test_build_edit_indexer_settings_params() {
    let mut app = App::test_default();
    app.data.lidarr_data.indexer_settings = Some(indexer_settings());

    let actual_indexer_settings = IndexerSettingsHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveLidarrBlock::AllIndexerSettingsPrompt,
      None,
    )
    .build_edit_indexer_settings_params();

    assert_eq!(actual_indexer_settings, indexer_settings());
    assert_modal_absent!(app.data.lidarr_data.indexer_settings);
  }

  #[test]
  fn test_edit_indexer_settings_handler_not_ready_when_loading() {
    let mut app = App::test_default();
    app.push_navigation_stack(ActiveLidarrBlock::Indexers.into());
    app.is_loading = true;

    let handler = IndexerSettingsHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveLidarrBlock::AllIndexerSettingsPrompt,
      None,
    );

    assert!(!handler.is_ready());
  }

  #[test]
  fn test_edit_indexer_settings_handler_not_ready_when_indexer_settings_is_none() {
    let mut app = App::test_default();
    app.push_navigation_stack(ActiveLidarrBlock::Indexers.into());
    app.is_loading = false;

    let handler = IndexerSettingsHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveLidarrBlock::AllIndexerSettingsPrompt,
      None,
    );

    assert!(!handler.is_ready());
  }

  #[test]
  fn test_edit_indexer_settings_handler_ready_when_not_loading_and_indexer_settings_is_some() {
    let mut app = App::test_default();
    app.push_navigation_stack(ActiveLidarrBlock::Indexers.into());
    app.is_loading = false;
    app.data.lidarr_data.indexer_settings = Some(IndexerSettings::default());

    let handler = IndexerSettingsHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveLidarrBlock::AllIndexerSettingsPrompt,
      None,
    );

    assert!(handler.is_ready());
  }
}
