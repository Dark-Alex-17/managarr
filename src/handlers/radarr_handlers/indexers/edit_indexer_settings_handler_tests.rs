#[cfg(test)]
mod tests {
  use pretty_assertions::assert_eq;
  use strum::IntoEnumIterator;

  use crate::app::key_binding::DEFAULT_KEYBINDINGS;
  use crate::app::App;
  use crate::event::Key;
  use crate::handlers::radarr_handlers::indexers::edit_indexer_settings_handler::IndexerSettingsHandler;
  use crate::handlers::radarr_handlers::radarr_handler_test_utils::utils::indexer_settings;
  use crate::handlers::KeyEventHandler;
  use crate::models::radarr_models::IndexerSettings;
  use crate::models::servarr_data::radarr::radarr_data::{
    ActiveRadarrBlock, INDEXER_SETTINGS_BLOCKS,
  };

  mod test_handle_scroll_up_and_down {
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    use crate::models::radarr_models::IndexerSettings;
    use crate::models::servarr_data::radarr::radarr_data::INDEXER_SETTINGS_SELECTION_BLOCKS;
    use crate::models::BlockSelectionState;

    use super::*;

    macro_rules! test_i64_counter_scroll_value {
      ($block:expr, $key:expr, $data_ref:ident, $negatives:literal) => {
        let mut app = App::test_default();
        app.data.radarr_data.indexer_settings = Some(IndexerSettings::default());

        IndexerSettingsHandler::new($key, &mut app, $block, None).handle();

        if $key == Key::Up {
          assert_eq!(
            app
              .data
              .radarr_data
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
                .radarr_data
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
                .radarr_data
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
                .radarr_data
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
                .radarr_data
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
      app.data.radarr_data.indexer_settings = Some(IndexerSettings::default());
      app.data.radarr_data.selected_block =
        BlockSelectionState::new(INDEXER_SETTINGS_SELECTION_BLOCKS);
      app.data.radarr_data.selected_block.down();

      IndexerSettingsHandler::new(
        key,
        &mut app,
        ActiveRadarrBlock::AllIndexerSettingsPrompt,
        None,
      )
      .handle();

      if key == Key::Up {
        assert_eq!(
          app.data.radarr_data.selected_block.get_active_block(),
          ActiveRadarrBlock::IndexerSettingsMinimumAgeInput
        );
      } else {
        assert_eq!(
          app.data.radarr_data.selected_block.get_active_block(),
          ActiveRadarrBlock::IndexerSettingsMaximumSizeInput
        );
      }
    }

    #[rstest]
    fn test_edit_indexer_settings_prompt_scroll_no_op_when_not_ready(
      #[values(Key::Up, Key::Down)] key: Key,
    ) {
      let mut app = App::test_default();
      app.is_loading = true;
      app.data.radarr_data.indexer_settings = Some(IndexerSettings::default());
      app.data.radarr_data.selected_block =
        BlockSelectionState::new(INDEXER_SETTINGS_SELECTION_BLOCKS);
      app.data.radarr_data.selected_block.down();

      IndexerSettingsHandler::new(
        key,
        &mut app,
        ActiveRadarrBlock::AllIndexerSettingsPrompt,
        None,
      )
      .handle();

      assert_eq!(
        app.data.radarr_data.selected_block.get_active_block(),
        ActiveRadarrBlock::IndexerSettingsRetentionInput
      );
    }

    #[rstest]
    fn test_edit_indexer_settings_minimum_age_scroll(#[values(Key::Up, Key::Down)] key: Key) {
      test_i64_counter_scroll_value!(
        ActiveRadarrBlock::IndexerSettingsMinimumAgeInput,
        key,
        minimum_age,
        false
      );
    }

    #[rstest]
    fn test_edit_indexer_settings_retention_scroll(#[values(Key::Up, Key::Down)] key: Key) {
      test_i64_counter_scroll_value!(
        ActiveRadarrBlock::IndexerSettingsRetentionInput,
        key,
        retention,
        false
      );
    }

    #[rstest]
    fn test_edit_indexer_settings_maximum_size_scroll(#[values(Key::Up, Key::Down)] key: Key) {
      test_i64_counter_scroll_value!(
        ActiveRadarrBlock::IndexerSettingsMaximumSizeInput,
        key,
        maximum_size,
        false
      );
    }

    #[rstest]
    fn test_edit_indexer_settings_availability_delay_scroll(
      #[values(Key::Up, Key::Down)] key: Key,
    ) {
      test_i64_counter_scroll_value!(
        ActiveRadarrBlock::IndexerSettingsAvailabilityDelayInput,
        key,
        availability_delay,
        true
      );
    }

    #[rstest]
    fn test_edit_indexer_settings_rss_sync_interval_scroll(#[values(Key::Up, Key::Down)] key: Key) {
      test_i64_counter_scroll_value!(
        ActiveRadarrBlock::IndexerSettingsRssSyncIntervalInput,
        key,
        rss_sync_interval,
        false
      );
    }
  }

  mod test_handle_home_end {
    use std::sync::atomic::Ordering;

    use pretty_assertions::assert_eq;

    use crate::models::radarr_models::IndexerSettings;

    use super::*;

    #[test]
    fn test_edit_indexer_settings_whiteliested_subtitle_tags_input_home_end() {
      let mut app = App::test_default();
      app.data.radarr_data.indexer_settings = Some(IndexerSettings {
        whitelisted_hardcoded_subs: "Test".into(),
        ..IndexerSettings::default()
      });

      IndexerSettingsHandler::new(
        DEFAULT_KEYBINDINGS.home.key,
        &mut app,
        ActiveRadarrBlock::IndexerSettingsWhitelistedSubtitleTagsInput,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .radarr_data
          .indexer_settings
          .as_ref()
          .unwrap()
          .whitelisted_hardcoded_subs
          .offset
          .load(Ordering::SeqCst),
        4
      );

      IndexerSettingsHandler::new(
        DEFAULT_KEYBINDINGS.end.key,
        &mut app,
        ActiveRadarrBlock::IndexerSettingsWhitelistedSubtitleTagsInput,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .radarr_data
          .indexer_settings
          .as_ref()
          .unwrap()
          .whitelisted_hardcoded_subs
          .offset
          .load(Ordering::SeqCst),
        0
      );
    }
  }

  mod test_handle_left_right_action {
    use std::sync::atomic::Ordering;

    use crate::models::radarr_models::IndexerSettings;
    use crate::models::servarr_data::radarr::radarr_data::INDEXER_SETTINGS_SELECTION_BLOCKS;
    use crate::models::BlockSelectionState;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    use super::*;

    #[rstest]
    fn test_left_right_prompt_toggle(#[values(Key::Left, Key::Right)] key: Key) {
      let mut app = App::test_default();
      app.data.radarr_data.selected_block =
        BlockSelectionState::new(INDEXER_SETTINGS_SELECTION_BLOCKS);
      app.data.radarr_data.selected_block.y = INDEXER_SETTINGS_SELECTION_BLOCKS.len() - 1;

      IndexerSettingsHandler::new(
        key,
        &mut app,
        ActiveRadarrBlock::AllIndexerSettingsPrompt,
        None,
      )
      .handle();

      assert!(app.data.radarr_data.prompt_confirm);

      IndexerSettingsHandler::new(
        key,
        &mut app,
        ActiveRadarrBlock::AllIndexerSettingsPrompt,
        None,
      )
      .handle();

      assert!(!app.data.radarr_data.prompt_confirm);
    }

    #[rstest]
    #[case(
      0,
      ActiveRadarrBlock::IndexerSettingsMinimumAgeInput,
      ActiveRadarrBlock::IndexerSettingsAvailabilityDelayInput
    )]
    #[case(
      1,
      ActiveRadarrBlock::IndexerSettingsRetentionInput,
      ActiveRadarrBlock::IndexerSettingsRssSyncIntervalInput
    )]
    #[case(
      2,
      ActiveRadarrBlock::IndexerSettingsMaximumSizeInput,
      ActiveRadarrBlock::IndexerSettingsWhitelistedSubtitleTagsInput
    )]
    #[case(
      3,
      ActiveRadarrBlock::IndexerSettingsTogglePreferIndexerFlags,
      ActiveRadarrBlock::IndexerSettingsToggleAllowHardcodedSubs
    )]
    fn test_left_right_block_toggle(
      #[values(Key::Left, Key::Right)] key: Key,
      #[case] starting_y_index: usize,
      #[case] left_block: ActiveRadarrBlock,
      #[case] right_block: ActiveRadarrBlock,
    ) {
      let mut app = App::test_default();
      app.data.radarr_data.selected_block =
        BlockSelectionState::new(INDEXER_SETTINGS_SELECTION_BLOCKS);
      app.data.radarr_data.selected_block.y = starting_y_index;

      assert_eq!(
        app.data.radarr_data.selected_block.get_active_block(),
        left_block
      );

      IndexerSettingsHandler::new(
        key,
        &mut app,
        ActiveRadarrBlock::AllIndexerSettingsPrompt,
        None,
      )
      .handle();

      assert_eq!(
        app.data.radarr_data.selected_block.get_active_block(),
        right_block
      );

      IndexerSettingsHandler::new(
        key,
        &mut app,
        ActiveRadarrBlock::AllIndexerSettingsPrompt,
        None,
      )
      .handle();

      assert_eq!(
        app.data.radarr_data.selected_block.get_active_block(),
        left_block
      );
    }

    #[test]
    fn test_edit_indexer_settings_whitelisted_subtitle_tags_input_left_right_keys() {
      let mut app = App::test_default();
      app.data.radarr_data.indexer_settings = Some(IndexerSettings {
        whitelisted_hardcoded_subs: "Test".into(),
        ..IndexerSettings::default()
      });

      IndexerSettingsHandler::new(
        DEFAULT_KEYBINDINGS.left.key,
        &mut app,
        ActiveRadarrBlock::IndexerSettingsWhitelistedSubtitleTagsInput,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .radarr_data
          .indexer_settings
          .as_ref()
          .unwrap()
          .whitelisted_hardcoded_subs
          .offset
          .load(Ordering::SeqCst),
        1
      );

      IndexerSettingsHandler::new(
        DEFAULT_KEYBINDINGS.right.key,
        &mut app,
        ActiveRadarrBlock::IndexerSettingsWhitelistedSubtitleTagsInput,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .radarr_data
          .indexer_settings
          .as_ref()
          .unwrap()
          .whitelisted_hardcoded_subs
          .offset
          .load(Ordering::SeqCst),
        0
      );
    }
  }

  mod test_handle_submit {
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    use crate::{
      models::{
        radarr_models::IndexerSettings,
        servarr_data::radarr::radarr_data::INDEXER_SETTINGS_SELECTION_BLOCKS, BlockSelectionState,
      },
      network::radarr_network::RadarrEvent,
    };

    use super::*;

    const SUBMIT_KEY: Key = DEFAULT_KEYBINDINGS.submit.key;

    #[test]
    fn test_edit_indexer_settings_prompt_prompt_decline_submit() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveRadarrBlock::Indexers.into());
      app.push_navigation_stack(ActiveRadarrBlock::AllIndexerSettingsPrompt.into());
      app.data.radarr_data.selected_block =
        BlockSelectionState::new(INDEXER_SETTINGS_SELECTION_BLOCKS);
      app
        .data
        .radarr_data
        .selected_block
        .set_index(0, INDEXER_SETTINGS_SELECTION_BLOCKS.len() - 1);
      app.data.radarr_data.indexer_settings = Some(IndexerSettings::default());

      IndexerSettingsHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveRadarrBlock::AllIndexerSettingsPrompt,
        None,
      )
      .handle();

      assert_eq!(app.get_current_route(), ActiveRadarrBlock::Indexers.into());
      assert_eq!(app.data.radarr_data.prompt_confirm_action, None);
      assert!(!app.should_refresh);
      assert_eq!(app.data.radarr_data.indexer_settings, None);
    }

    #[test]
    fn test_edit_indexer_settings_prompt_prompt_confirmation_submit() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveRadarrBlock::Indexers.into());
      app.push_navigation_stack(ActiveRadarrBlock::AllIndexerSettingsPrompt.into());
      app.data.radarr_data.selected_block =
        BlockSelectionState::new(INDEXER_SETTINGS_SELECTION_BLOCKS);
      app
        .data
        .radarr_data
        .selected_block
        .set_index(0, INDEXER_SETTINGS_SELECTION_BLOCKS.len() - 1);
      app.data.radarr_data.indexer_settings = Some(indexer_settings());
      app.data.radarr_data.prompt_confirm = true;

      IndexerSettingsHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveRadarrBlock::AllIndexerSettingsPrompt,
        None,
      )
      .handle();

      assert_eq!(app.get_current_route(), ActiveRadarrBlock::Indexers.into());
      assert_eq!(
        app.data.radarr_data.prompt_confirm_action,
        Some(RadarrEvent::EditAllIndexerSettings(indexer_settings()))
      );
      assert!(app.data.radarr_data.indexer_settings.is_none());
      assert!(app.should_refresh);
    }

    #[test]
    fn test_edit_indexer_settings_prompt_prompt_confirmation_submit_no_op_when_not_ready() {
      let mut app = App::test_default();
      app.is_loading = true;
      app.push_navigation_stack(ActiveRadarrBlock::Indexers.into());
      app.push_navigation_stack(ActiveRadarrBlock::AllIndexerSettingsPrompt.into());
      app.data.radarr_data.indexer_settings = Some(IndexerSettings::default());
      app.data.radarr_data.prompt_confirm = true;

      IndexerSettingsHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveRadarrBlock::AllIndexerSettingsPrompt,
        None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveRadarrBlock::AllIndexerSettingsPrompt.into()
      );
      assert!(!app.should_refresh);
    }

    #[rstest]
    #[case(ActiveRadarrBlock::IndexerSettingsMinimumAgeInput, 0, 0)]
    #[case(ActiveRadarrBlock::IndexerSettingsRetentionInput, 1, 0)]
    #[case(ActiveRadarrBlock::IndexerSettingsMaximumSizeInput, 2, 0)]
    #[case(ActiveRadarrBlock::IndexerSettingsAvailabilityDelayInput, 0, 1)]
    #[case(ActiveRadarrBlock::IndexerSettingsRssSyncIntervalInput, 1, 1)]
    fn test_edit_indexer_settings_prompt_submit_selected_block(
      #[case] selected_block: ActiveRadarrBlock,
      #[case] y_index: usize,
      #[case] x_index: usize,
    ) {
      let mut app = App::test_default();
      app.data.radarr_data.indexer_settings = Some(IndexerSettings::default());
      app.push_navigation_stack(ActiveRadarrBlock::AllIndexerSettingsPrompt.into());
      app.data.radarr_data.selected_block =
        BlockSelectionState::new(INDEXER_SETTINGS_SELECTION_BLOCKS);
      app
        .data
        .radarr_data
        .selected_block
        .set_index(x_index, y_index);

      IndexerSettingsHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveRadarrBlock::AllIndexerSettingsPrompt,
        None,
      )
      .handle();

      assert_eq!(app.get_current_route(), selected_block.into());
    }

    #[rstest]
    fn test_edit_indexer_settings_prompt_submit_selected_block_no_op_when_not_ready(
      #[values((0, 0), (1, 0), (2, 0), (0, 1), (1, 1))] index: (usize, usize),
    ) {
      let mut app = App::test_default();
      app.is_loading = true;
      app.data.radarr_data.indexer_settings = Some(IndexerSettings::default());
      app.push_navigation_stack(ActiveRadarrBlock::AllIndexerSettingsPrompt.into());
      app.data.radarr_data.selected_block =
        BlockSelectionState::new(INDEXER_SETTINGS_SELECTION_BLOCKS);
      app
        .data
        .radarr_data
        .selected_block
        .set_index(index.1, index.0);

      IndexerSettingsHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveRadarrBlock::AllIndexerSettingsPrompt,
        None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveRadarrBlock::AllIndexerSettingsPrompt.into()
      );
    }

    #[test]
    fn test_edit_indexer_settings_prompt_submit_whitelisted_subtitle_tags_input() {
      let mut app = App::test_default();
      app.data.radarr_data.indexer_settings = Some(IndexerSettings::default());
      app.push_navigation_stack(ActiveRadarrBlock::AllIndexerSettingsPrompt.into());
      app.data.radarr_data.selected_block =
        BlockSelectionState::new(INDEXER_SETTINGS_SELECTION_BLOCKS);
      app.data.radarr_data.selected_block.set_index(1, 2);

      IndexerSettingsHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveRadarrBlock::AllIndexerSettingsPrompt,
        None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveRadarrBlock::IndexerSettingsWhitelistedSubtitleTagsInput.into()
      );
      assert!(app.should_ignore_quit_key);
    }

    #[test]
    fn test_edit_indexer_settings_toggle_prefer_indexer_flags_submit() {
      let mut app = App::test_default();
      app.data.radarr_data.indexer_settings = Some(IndexerSettings::default());
      app.data.radarr_data.selected_block =
        BlockSelectionState::new(INDEXER_SETTINGS_SELECTION_BLOCKS);
      app.data.radarr_data.selected_block.set_index(0, 3);
      app.push_navigation_stack(ActiveRadarrBlock::AllIndexerSettingsPrompt.into());

      IndexerSettingsHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveRadarrBlock::AllIndexerSettingsPrompt,
        None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveRadarrBlock::AllIndexerSettingsPrompt.into()
      );
      assert!(
        app
          .data
          .radarr_data
          .indexer_settings
          .as_ref()
          .unwrap()
          .prefer_indexer_flags
      );

      IndexerSettingsHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveRadarrBlock::AllIndexerSettingsPrompt,
        None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveRadarrBlock::AllIndexerSettingsPrompt.into()
      );
      assert!(
        !app
          .data
          .radarr_data
          .indexer_settings
          .as_ref()
          .unwrap()
          .prefer_indexer_flags
      );
    }

    #[test]
    fn test_edit_indexer_settings_toggle_allow_hardcoded_subs_submit() {
      let mut app = App::test_default();
      app.data.radarr_data.indexer_settings = Some(IndexerSettings::default());
      app.data.radarr_data.selected_block =
        BlockSelectionState::new(INDEXER_SETTINGS_SELECTION_BLOCKS);
      app.data.radarr_data.selected_block.set_index(1, 3);
      app.push_navigation_stack(ActiveRadarrBlock::AllIndexerSettingsPrompt.into());

      IndexerSettingsHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveRadarrBlock::AllIndexerSettingsPrompt,
        None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveRadarrBlock::AllIndexerSettingsPrompt.into()
      );
      assert!(
        app
          .data
          .radarr_data
          .indexer_settings
          .as_ref()
          .unwrap()
          .allow_hardcoded_subs
      );

      IndexerSettingsHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveRadarrBlock::AllIndexerSettingsPrompt,
        None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveRadarrBlock::AllIndexerSettingsPrompt.into()
      );
      assert!(
        !app
          .data
          .radarr_data
          .indexer_settings
          .as_ref()
          .unwrap()
          .allow_hardcoded_subs
      );
    }

    #[test]
    fn test_edit_indexer_settings_whitelisted_subtitle_tags_input_submit() {
      let mut app = App::test_default();
      app.should_ignore_quit_key = true;
      app.data.radarr_data.indexer_settings = Some(IndexerSettings {
        whitelisted_hardcoded_subs: "Test tags".into(),
        ..IndexerSettings::default()
      });
      app.push_navigation_stack(ActiveRadarrBlock::AllIndexerSettingsPrompt.into());
      app.push_navigation_stack(
        ActiveRadarrBlock::IndexerSettingsWhitelistedSubtitleTagsInput.into(),
      );

      IndexerSettingsHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveRadarrBlock::IndexerSettingsWhitelistedSubtitleTagsInput,
        None,
      )
      .handle();

      assert!(!app.should_ignore_quit_key);
      assert!(!app
        .data
        .radarr_data
        .indexer_settings
        .as_ref()
        .unwrap()
        .whitelisted_hardcoded_subs
        .text
        .is_empty());
      assert_eq!(
        app.get_current_route(),
        ActiveRadarrBlock::AllIndexerSettingsPrompt.into()
      );
    }

    #[rstest]
    fn test_edit_indexer_settings_selected_block_submit(
      #[values(
        ActiveRadarrBlock::IndexerSettingsMinimumAgeInput,
        ActiveRadarrBlock::IndexerSettingsRetentionInput,
        ActiveRadarrBlock::IndexerSettingsMaximumSizeInput,
        ActiveRadarrBlock::IndexerSettingsAvailabilityDelayInput,
        ActiveRadarrBlock::IndexerSettingsRssSyncIntervalInput
      )]
      active_radarr_block: ActiveRadarrBlock,
    ) {
      let mut app = App::test_default();
      app.data.radarr_data.indexer_settings = Some(IndexerSettings::default());
      app.push_navigation_stack(ActiveRadarrBlock::AllIndexerSettingsPrompt.into());
      app.push_navigation_stack(active_radarr_block.into());

      IndexerSettingsHandler::new(SUBMIT_KEY, &mut app, active_radarr_block, None).handle();

      assert_eq!(
        app.get_current_route(),
        ActiveRadarrBlock::AllIndexerSettingsPrompt.into()
      );
    }
  }

  mod test_handle_esc {
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    use crate::models::radarr_models::IndexerSettings;

    use super::*;

    const ESC_KEY: Key = DEFAULT_KEYBINDINGS.esc.key;

    #[rstest]
    fn test_edit_indexer_settings_prompt_esc(#[values(true, false)] is_ready: bool) {
      let mut app = App::test_default();
      app.is_loading = is_ready;
      app.push_navigation_stack(ActiveRadarrBlock::Indexers.into());
      app.push_navigation_stack(ActiveRadarrBlock::AllIndexerSettingsPrompt.into());
      app.data.radarr_data.indexer_settings = Some(IndexerSettings::default());

      IndexerSettingsHandler::new(
        ESC_KEY,
        &mut app,
        ActiveRadarrBlock::AllIndexerSettingsPrompt,
        None,
      )
      .handle();

      assert_eq!(app.get_current_route(), ActiveRadarrBlock::Indexers.into());
      assert!(!app.data.radarr_data.prompt_confirm);
      assert_eq!(app.data.radarr_data.indexer_settings, None);
    }

    #[test]
    fn test_edit_indexer_settings_whitelisted_subtitle_tags_input_esc() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveRadarrBlock::Indexers.into());
      app.push_navigation_stack(
        ActiveRadarrBlock::IndexerSettingsWhitelistedSubtitleTagsInput.into(),
      );
      app.data.radarr_data.indexer_settings = Some(IndexerSettings::default());
      app.should_ignore_quit_key = true;

      IndexerSettingsHandler::new(
        ESC_KEY,
        &mut app,
        ActiveRadarrBlock::IndexerSettingsWhitelistedSubtitleTagsInput,
        None,
      )
      .handle();

      assert_eq!(app.get_current_route(), ActiveRadarrBlock::Indexers.into());
      assert!(!app.should_ignore_quit_key);
      assert_eq!(
        app.data.radarr_data.indexer_settings,
        Some(IndexerSettings::default())
      );
    }

    #[rstest]
    fn test_edit_indexer_settings_selected_blocks_esc(
      #[values(
        ActiveRadarrBlock::IndexerSettingsMinimumAgeInput,
        ActiveRadarrBlock::IndexerSettingsRetentionInput,
        ActiveRadarrBlock::IndexerSettingsMaximumSizeInput,
        ActiveRadarrBlock::IndexerSettingsAvailabilityDelayInput,
        ActiveRadarrBlock::IndexerSettingsRssSyncIntervalInput,
        ActiveRadarrBlock::IndexerSettingsWhitelistedSubtitleTagsInput
      )]
      active_radarr_block: ActiveRadarrBlock,
    ) {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveRadarrBlock::Indexers.into());
      app.push_navigation_stack(active_radarr_block.into());
      app.data.radarr_data.indexer_settings = Some(IndexerSettings::default());

      IndexerSettingsHandler::new(ESC_KEY, &mut app, active_radarr_block, None).handle();

      assert_eq!(app.get_current_route(), ActiveRadarrBlock::Indexers.into());
      assert_eq!(
        app.data.radarr_data.indexer_settings,
        Some(IndexerSettings::default())
      );
    }
  }

  mod test_handle_key_char {
    use pretty_assertions::{assert_eq, assert_str_eq};

    use crate::{
      models::{
        radarr_models::IndexerSettings,
        servarr_data::radarr::radarr_data::INDEXER_SETTINGS_SELECTION_BLOCKS, BlockSelectionState,
      },
      network::radarr_network::RadarrEvent,
    };

    use super::*;

    #[test]
    fn test_edit_indexer_settings_whitelisted_subtitle_tags_input_backspace() {
      let mut app = App::test_default();
      app.data.radarr_data.indexer_settings = Some(IndexerSettings {
        whitelisted_hardcoded_subs: "Test".into(),
        ..IndexerSettings::default()
      });

      IndexerSettingsHandler::new(
        DEFAULT_KEYBINDINGS.backspace.key,
        &mut app,
        ActiveRadarrBlock::IndexerSettingsWhitelistedSubtitleTagsInput,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .radarr_data
          .indexer_settings
          .as_ref()
          .unwrap()
          .whitelisted_hardcoded_subs
          .text,
        "Tes"
      );
    }

    #[test]
    fn test_edit_indexer_settings_whitelisted_subtitle_tags_input_char_key() {
      let mut app = App::test_default();
      app.data.radarr_data.indexer_settings = Some(IndexerSettings::default());

      IndexerSettingsHandler::new(
        Key::Char('h'),
        &mut app,
        ActiveRadarrBlock::IndexerSettingsWhitelistedSubtitleTagsInput,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .radarr_data
          .indexer_settings
          .as_ref()
          .unwrap()
          .whitelisted_hardcoded_subs
          .text,
        "h"
      );
    }

    #[test]
    fn test_edit_indexer_settings_prompt_prompt_confirmation_confirm() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveRadarrBlock::Indexers.into());
      app.push_navigation_stack(ActiveRadarrBlock::AllIndexerSettingsPrompt.into());
      app.data.radarr_data.selected_block =
        BlockSelectionState::new(INDEXER_SETTINGS_SELECTION_BLOCKS);
      app
        .data
        .radarr_data
        .selected_block
        .set_index(0, INDEXER_SETTINGS_SELECTION_BLOCKS.len() - 1);
      app.data.radarr_data.indexer_settings = Some(indexer_settings());

      IndexerSettingsHandler::new(
        DEFAULT_KEYBINDINGS.confirm.key,
        &mut app,
        ActiveRadarrBlock::AllIndexerSettingsPrompt,
        None,
      )
      .handle();

      assert_eq!(app.get_current_route(), ActiveRadarrBlock::Indexers.into());
      assert_eq!(
        app.data.radarr_data.prompt_confirm_action,
        Some(RadarrEvent::EditAllIndexerSettings(indexer_settings()))
      );
      assert!(app.data.radarr_data.indexer_settings.is_none());
      assert!(app.should_refresh);
    }
  }

  #[test]
  fn test_indexer_settings_handler_accepts() {
    ActiveRadarrBlock::iter().for_each(|active_radarr_block| {
      if INDEXER_SETTINGS_BLOCKS.contains(&active_radarr_block) {
        assert!(IndexerSettingsHandler::accepts(active_radarr_block));
      } else {
        assert!(!IndexerSettingsHandler::accepts(active_radarr_block));
      }
    })
  }

  #[test]
  fn test_build_edit_indexer_settings_body() {
    let mut app = App::test_default();
    app.data.radarr_data.indexer_settings = Some(indexer_settings());

    let body = IndexerSettingsHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveRadarrBlock::AllIndexerSettingsPrompt,
      None,
    )
    .build_edit_indexer_settings_body();

    assert_eq!(body, indexer_settings());
    assert!(app.data.radarr_data.indexer_settings.is_none());
  }

  #[test]
  fn test_edit_indexer_settings_handler_not_ready_when_loading() {
    let mut app = App::test_default();
    app.is_loading = true;

    let handler = IndexerSettingsHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveRadarrBlock::AllIndexerSettingsPrompt,
      None,
    );

    assert!(!handler.is_ready());
  }

  #[test]
  fn test_edit_indexer_settings_handler_not_ready_when_indexer_settings_is_none() {
    let mut app = App::test_default();
    app.is_loading = false;

    let handler = IndexerSettingsHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveRadarrBlock::AllIndexerSettingsPrompt,
      None,
    );

    assert!(!handler.is_ready());
  }

  #[test]
  fn test_edit_indexer_settings_handler_ready_when_not_loading_and_indexer_settings_is_some() {
    let mut app = App::test_default();
    app.is_loading = false;
    app.data.radarr_data.indexer_settings = Some(IndexerSettings::default());

    let handler = IndexerSettingsHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveRadarrBlock::AllIndexerSettingsPrompt,
      None,
    );

    assert!(handler.is_ready());
  }
}
