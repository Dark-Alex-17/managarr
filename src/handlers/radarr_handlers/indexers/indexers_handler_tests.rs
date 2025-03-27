#[cfg(test)]
mod tests {
  use pretty_assertions::assert_eq;
  use rstest::rstest;
  use strum::IntoEnumIterator;

  use crate::app::key_binding::DEFAULT_KEYBINDINGS;
  use crate::app::App;
  use crate::event::Key;
  use crate::handlers::radarr_handlers::indexers::IndexersHandler;
  use crate::handlers::radarr_handlers::radarr_handler_test_utils::utils::indexer;
  use crate::handlers::KeyEventHandler;
  use crate::models::servarr_data::radarr::radarr_data::{
    ActiveRadarrBlock, EDIT_INDEXER_BLOCKS, INDEXERS_BLOCKS, INDEXER_SETTINGS_BLOCKS,
  };
  use crate::models::servarr_models::Indexer;
  use crate::test_handler_delegation;

  mod test_handle_delete {
    use pretty_assertions::assert_eq;

    use super::*;

    const DELETE_KEY: Key = DEFAULT_KEYBINDINGS.delete.key;

    #[test]
    fn test_delete_indexer_prompt() {
      let mut app = App::test_default();
      app
        .data
        .radarr_data
        .indexers
        .set_items(vec![Indexer::default()]);

      IndexersHandler::new(DELETE_KEY, &mut app, ActiveRadarrBlock::Indexers, None).handle();

      assert_eq!(
        app.get_current_route(),
        ActiveRadarrBlock::DeleteIndexerPrompt.into()
      );
    }

    #[test]
    fn test_delete_indexer_prompt_no_op_when_not_ready() {
      let mut app = App::test_default();
      app.is_loading = true;
      app.push_navigation_stack(ActiveRadarrBlock::Indexers.into());
      app
        .data
        .radarr_data
        .indexers
        .set_items(vec![Indexer::default()]);

      IndexersHandler::new(DELETE_KEY, &mut app, ActiveRadarrBlock::Indexers, None).handle();

      assert_eq!(app.get_current_route(), ActiveRadarrBlock::Indexers.into());
    }
  }

  mod test_handle_left_right_action {
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    use super::*;

    #[rstest]
    fn test_indexers_tab_left(#[values(true, false)] is_ready: bool) {
      let mut app = App::test_default();
      app.is_loading = is_ready;
      app.data.radarr_data.main_tabs.set_index(5);

      IndexersHandler::new(
        DEFAULT_KEYBINDINGS.left.key,
        &mut app,
        ActiveRadarrBlock::Indexers,
        None,
      )
      .handle();

      assert_eq!(
        app.data.radarr_data.main_tabs.get_active_route(),
        ActiveRadarrBlock::RootFolders.into()
      );
      assert_eq!(
        app.get_current_route(),
        ActiveRadarrBlock::RootFolders.into()
      );
    }

    #[rstest]
    fn test_indexers_tab_right(#[values(true, false)] is_ready: bool) {
      let mut app = App::test_default();
      app.is_loading = is_ready;
      app.data.radarr_data.main_tabs.set_index(5);

      IndexersHandler::new(
        DEFAULT_KEYBINDINGS.right.key,
        &mut app,
        ActiveRadarrBlock::Indexers,
        None,
      )
      .handle();

      assert_eq!(
        app.data.radarr_data.main_tabs.get_active_route(),
        ActiveRadarrBlock::System.into()
      );
      assert_eq!(app.get_current_route(), ActiveRadarrBlock::System.into());
    }

    #[rstest]
    fn test_left_right_delete_indexer_prompt_toggle(
      #[values(DEFAULT_KEYBINDINGS.left.key, DEFAULT_KEYBINDINGS.right.key)] key: Key,
    ) {
      let mut app = App::test_default();

      IndexersHandler::new(key, &mut app, ActiveRadarrBlock::DeleteIndexerPrompt, None).handle();

      assert!(app.data.radarr_data.prompt_confirm);

      IndexersHandler::new(key, &mut app, ActiveRadarrBlock::DeleteIndexerPrompt, None).handle();

      assert!(!app.data.radarr_data.prompt_confirm);
    }
  }

  mod test_handle_submit {
    use crate::handlers::radarr_handlers::radarr_handler_test_utils::utils::indexer;
    use crate::models::servarr_data::modals::EditIndexerModal;
    use crate::models::servarr_data::radarr::radarr_data::{
      RadarrData, EDIT_INDEXER_NZB_SELECTION_BLOCKS, EDIT_INDEXER_TORRENT_SELECTION_BLOCKS,
    };
    use crate::models::servarr_models::{Indexer, IndexerField};
    use crate::network::radarr_network::RadarrEvent;
    use bimap::BiMap;
    use pretty_assertions::assert_eq;
    use serde_json::{Number, Value};

    use super::*;

    const SUBMIT_KEY: Key = DEFAULT_KEYBINDINGS.submit.key;

    #[rstest]
    fn test_edit_indexer_submit(#[values(true, false)] torrent_protocol: bool) {
      let mut app = App::test_default();
      let protocol = if torrent_protocol {
        "torrent".to_owned()
      } else {
        "usenet".to_owned()
      };
      let mut expected_edit_indexer_modal = EditIndexerModal {
        name: "Test".into(),
        enable_rss: Some(true),
        enable_automatic_search: Some(true),
        enable_interactive_search: Some(true),
        url: "https://test.com".into(),
        api_key: "1234".into(),
        tags: "usenet, test".into(),
        ..EditIndexerModal::default()
      };
      let mut radarr_data = RadarrData {
        tags_map: BiMap::from_iter([(1, "usenet".to_owned()), (2, "test".to_owned())]),
        ..RadarrData::default()
      };
      let mut fields = vec![
        IndexerField {
          name: Some("baseUrl".to_owned()),
          value: Some(Value::String("https://test.com".to_owned())),
        },
        IndexerField {
          name: Some("apiKey".to_owned()),
          value: Some(Value::String("1234".to_owned())),
        },
      ];

      if torrent_protocol {
        fields.push(IndexerField {
          name: Some("seedCriteria.seedRatio".to_owned()),
          value: Some(Value::from(1.2f64)),
        });
        expected_edit_indexer_modal.seed_ratio = "1.2".into();
      }

      let indexer = Indexer {
        name: Some("Test".to_owned()),
        enable_rss: true,
        enable_automatic_search: true,
        enable_interactive_search: true,
        protocol,
        tags: vec![Number::from(1), Number::from(2)],
        fields: Some(fields),
        ..Indexer::default()
      };
      radarr_data.indexers.set_items(vec![indexer]);
      app.data.radarr_data = radarr_data;

      IndexersHandler::new(SUBMIT_KEY, &mut app, ActiveRadarrBlock::Indexers, None).handle();

      assert_eq!(
        app.get_current_route(),
        ActiveRadarrBlock::EditIndexerPrompt.into()
      );
      assert_eq!(
        app.data.radarr_data.edit_indexer_modal,
        Some((&app.data.radarr_data).into())
      );
      assert_eq!(
        app.data.radarr_data.edit_indexer_modal,
        Some(expected_edit_indexer_modal)
      );
      if torrent_protocol {
        assert_eq!(
          app.data.radarr_data.selected_block.blocks,
          EDIT_INDEXER_TORRENT_SELECTION_BLOCKS
        );
      } else {
        assert_eq!(
          app.data.radarr_data.selected_block.blocks,
          EDIT_INDEXER_NZB_SELECTION_BLOCKS
        );
      }
    }

    #[test]
    fn test_edit_indexer_submit_no_op_when_not_ready() {
      let mut app = App::test_default();
      app.is_loading = true;
      app.push_navigation_stack(ActiveRadarrBlock::Indexers.into());
      app
        .data
        .radarr_data
        .indexers
        .set_items(vec![Indexer::default()]);

      IndexersHandler::new(SUBMIT_KEY, &mut app, ActiveRadarrBlock::Indexers, None).handle();

      assert_eq!(app.get_current_route(), ActiveRadarrBlock::Indexers.into());
      assert_eq!(app.data.radarr_data.edit_indexer_modal, None);
    }

    #[test]
    fn test_delete_indexer_prompt_confirm_submit() {
      let mut app = App::test_default();
      app.data.radarr_data.indexers.set_items(vec![indexer()]);
      app.data.radarr_data.prompt_confirm = true;
      app.push_navigation_stack(ActiveRadarrBlock::Indexers.into());
      app.push_navigation_stack(ActiveRadarrBlock::DeleteIndexerPrompt.into());

      IndexersHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveRadarrBlock::DeleteIndexerPrompt,
        None,
      )
      .handle();

      assert!(app.data.radarr_data.prompt_confirm);
      assert_eq!(
        app.data.radarr_data.prompt_confirm_action,
        Some(RadarrEvent::DeleteIndexer(1))
      );
      assert_eq!(app.get_current_route(), ActiveRadarrBlock::Indexers.into());
    }

    #[test]
    fn test_prompt_decline_submit() {
      let mut app = App::test_default();
      app
        .data
        .radarr_data
        .indexers
        .set_items(vec![Indexer::default()]);
      app.push_navigation_stack(ActiveRadarrBlock::Indexers.into());
      app.push_navigation_stack(ActiveRadarrBlock::DeleteIndexerPrompt.into());

      IndexersHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveRadarrBlock::DeleteIndexerPrompt,
        None,
      )
      .handle();

      assert!(!app.data.radarr_data.prompt_confirm);
      assert_eq!(app.data.radarr_data.prompt_confirm_action, None);
      assert_eq!(app.get_current_route(), ActiveRadarrBlock::Indexers.into());
    }
  }

  mod test_handle_esc {
    use pretty_assertions::assert_eq;

    use super::*;

    const ESC_KEY: Key = DEFAULT_KEYBINDINGS.esc.key;

    #[rstest]
    fn test_delete_indexer_prompt_block_esc(#[values(true, false)] is_ready: bool) {
      let mut app = App::test_default();
      app.is_loading = is_ready;
      app.push_navigation_stack(ActiveRadarrBlock::Indexers.into());
      app.push_navigation_stack(ActiveRadarrBlock::DeleteIndexerPrompt.into());
      app.data.radarr_data.prompt_confirm = true;

      IndexersHandler::new(
        ESC_KEY,
        &mut app,
        ActiveRadarrBlock::DeleteIndexerPrompt,
        None,
      )
      .handle();

      assert_eq!(app.get_current_route(), ActiveRadarrBlock::Indexers.into());
      assert!(!app.data.radarr_data.prompt_confirm);
    }

    #[rstest]
    fn test_test_indexer_esc(#[values(true, false)] is_ready: bool) {
      let mut app = App::test_default();
      app.is_loading = is_ready;
      app.data.radarr_data.indexer_test_errors = Some("test result".to_owned());
      app.push_navigation_stack(ActiveRadarrBlock::Indexers.into());
      app.push_navigation_stack(ActiveRadarrBlock::TestIndexer.into());

      IndexersHandler::new(ESC_KEY, &mut app, ActiveRadarrBlock::TestIndexer, None).handle();

      assert_eq!(app.get_current_route(), ActiveRadarrBlock::Indexers.into());
      assert_eq!(app.data.radarr_data.indexer_test_errors, None);
    }

    #[rstest]
    fn test_default_esc(#[values(true, false)] is_ready: bool) {
      let mut app = App::test_default();
      app.is_loading = is_ready;
      app.error = "test error".to_owned().into();
      app.push_navigation_stack(ActiveRadarrBlock::Indexers.into());
      app.push_navigation_stack(ActiveRadarrBlock::Indexers.into());

      IndexersHandler::new(ESC_KEY, &mut app, ActiveRadarrBlock::Indexers, None).handle();

      assert_eq!(app.get_current_route(), ActiveRadarrBlock::Indexers.into());
      assert!(app.error.text.is_empty());
    }
  }

  mod test_handle_key_char {
    use pretty_assertions::assert_eq;

    use super::*;
    use crate::handlers::radarr_handlers::radarr_handler_test_utils::utils::indexer;
    use crate::{
      models::servarr_data::radarr::radarr_data::INDEXER_SETTINGS_SELECTION_BLOCKS,
      network::radarr_network::RadarrEvent,
    };

    #[test]
    fn test_refresh_indexers_key() {
      let mut app = App::test_default();
      app
        .data
        .radarr_data
        .indexers
        .set_items(vec![Indexer::default()]);
      app.push_navigation_stack(ActiveRadarrBlock::Indexers.into());

      IndexersHandler::new(
        DEFAULT_KEYBINDINGS.refresh.key,
        &mut app,
        ActiveRadarrBlock::Indexers,
        None,
      )
      .handle();

      assert_eq!(app.get_current_route(), ActiveRadarrBlock::Indexers.into());
      assert!(app.should_refresh);
    }

    #[test]
    fn test_refresh_indexers_key_no_op_when_not_ready() {
      let mut app = App::test_default();
      app.is_loading = true;
      app
        .data
        .radarr_data
        .indexers
        .set_items(vec![Indexer::default()]);
      app.push_navigation_stack(ActiveRadarrBlock::Indexers.into());

      IndexersHandler::new(
        DEFAULT_KEYBINDINGS.refresh.key,
        &mut app,
        ActiveRadarrBlock::Indexers,
        None,
      )
      .handle();

      assert_eq!(app.get_current_route(), ActiveRadarrBlock::Indexers.into());
      assert!(!app.should_refresh);
    }

    #[test]
    fn test_indexer_settings_key() {
      let mut app = App::test_default();
      app
        .data
        .radarr_data
        .indexers
        .set_items(vec![Indexer::default()]);

      IndexersHandler::new(
        DEFAULT_KEYBINDINGS.settings.key,
        &mut app,
        ActiveRadarrBlock::Indexers,
        None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveRadarrBlock::AllIndexerSettingsPrompt.into()
      );
      assert_eq!(
        app.data.radarr_data.selected_block.blocks,
        INDEXER_SETTINGS_SELECTION_BLOCKS
      );
    }

    #[test]
    fn test_indexer_settings_key_no_op_when_not_ready() {
      let mut app = App::test_default();
      app.is_loading = true;
      app.push_navigation_stack(ActiveRadarrBlock::Indexers.into());
      app
        .data
        .radarr_data
        .indexers
        .set_items(vec![Indexer::default()]);

      IndexersHandler::new(
        DEFAULT_KEYBINDINGS.settings.key,
        &mut app,
        ActiveRadarrBlock::Indexers,
        None,
      )
      .handle();

      assert_eq!(app.get_current_route(), ActiveRadarrBlock::Indexers.into());
    }

    #[test]
    fn test_test_key() {
      let mut app = App::test_default();
      app
        .data
        .radarr_data
        .indexers
        .set_items(vec![Indexer::default()]);

      IndexersHandler::new(
        DEFAULT_KEYBINDINGS.test.key,
        &mut app,
        ActiveRadarrBlock::Indexers,
        None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveRadarrBlock::TestIndexer.into()
      );
    }

    #[test]
    fn test_test_key_no_op_when_not_ready() {
      let mut app = App::test_default();
      app.is_loading = true;
      app.push_navigation_stack(ActiveRadarrBlock::Indexers.into());
      app
        .data
        .radarr_data
        .indexers
        .set_items(vec![Indexer::default()]);

      IndexersHandler::new(
        DEFAULT_KEYBINDINGS.test.key,
        &mut app,
        ActiveRadarrBlock::Indexers,
        None,
      )
      .handle();

      assert_eq!(app.get_current_route(), ActiveRadarrBlock::Indexers.into());
    }

    #[test]
    fn test_test_all_key() {
      let mut app = App::test_default();
      app
        .data
        .radarr_data
        .indexers
        .set_items(vec![Indexer::default()]);

      IndexersHandler::new(
        DEFAULT_KEYBINDINGS.test_all.key,
        &mut app,
        ActiveRadarrBlock::Indexers,
        None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveRadarrBlock::TestAllIndexers.into()
      );
    }

    #[test]
    fn test_test_all_key_no_op_when_not_ready() {
      let mut app = App::test_default();
      app.is_loading = true;
      app.push_navigation_stack(ActiveRadarrBlock::Indexers.into());
      app
        .data
        .radarr_data
        .indexers
        .set_items(vec![Indexer::default()]);

      IndexersHandler::new(
        DEFAULT_KEYBINDINGS.test_all.key,
        &mut app,
        ActiveRadarrBlock::Indexers,
        None,
      )
      .handle();

      assert_eq!(app.get_current_route(), ActiveRadarrBlock::Indexers.into());
    }

    #[test]
    fn test_delete_indexer_prompt_confirm() {
      let mut app = App::test_default();
      app.data.radarr_data.indexers.set_items(vec![indexer()]);
      app.push_navigation_stack(ActiveRadarrBlock::Indexers.into());
      app.push_navigation_stack(ActiveRadarrBlock::DeleteIndexerPrompt.into());

      IndexersHandler::new(
        DEFAULT_KEYBINDINGS.confirm.key,
        &mut app,
        ActiveRadarrBlock::DeleteIndexerPrompt,
        None,
      )
      .handle();

      assert!(app.data.radarr_data.prompt_confirm);
      assert_eq!(
        app.data.radarr_data.prompt_confirm_action,
        Some(RadarrEvent::DeleteIndexer(1))
      );
      assert_eq!(app.get_current_route(), ActiveRadarrBlock::Indexers.into());
    }
  }

  #[rstest]
  fn test_delegates_edit_indexer_blocks_to_edit_indexer_handler(
    #[values(
      ActiveRadarrBlock::EditIndexerPrompt,
      ActiveRadarrBlock::EditIndexerConfirmPrompt,
      ActiveRadarrBlock::EditIndexerApiKeyInput,
      ActiveRadarrBlock::EditIndexerNameInput,
      ActiveRadarrBlock::EditIndexerSeedRatioInput,
      ActiveRadarrBlock::EditIndexerToggleEnableRss,
      ActiveRadarrBlock::EditIndexerToggleEnableAutomaticSearch,
      ActiveRadarrBlock::EditIndexerToggleEnableInteractiveSearch,
      ActiveRadarrBlock::EditIndexerUrlInput,
      ActiveRadarrBlock::EditIndexerTagsInput
    )]
    active_radarr_block: ActiveRadarrBlock,
  ) {
    test_handler_delegation!(
      IndexersHandler,
      ActiveRadarrBlock::Indexers,
      active_radarr_block
    );
  }

  #[rstest]
  fn test_delegates_indexer_settings_blocks_to_indexer_settings_handler(
    #[values(
      ActiveRadarrBlock::AllIndexerSettingsPrompt,
      ActiveRadarrBlock::IndexerSettingsAvailabilityDelayInput,
      ActiveRadarrBlock::IndexerSettingsConfirmPrompt,
      ActiveRadarrBlock::IndexerSettingsMaximumSizeInput,
      ActiveRadarrBlock::IndexerSettingsMinimumAgeInput,
      ActiveRadarrBlock::IndexerSettingsRetentionInput,
      ActiveRadarrBlock::IndexerSettingsRssSyncIntervalInput,
      ActiveRadarrBlock::IndexerSettingsToggleAllowHardcodedSubs,
      ActiveRadarrBlock::IndexerSettingsTogglePreferIndexerFlags,
      ActiveRadarrBlock::IndexerSettingsWhitelistedSubtitleTagsInput
    )]
    active_radarr_block: ActiveRadarrBlock,
  ) {
    test_handler_delegation!(
      IndexersHandler,
      ActiveRadarrBlock::Indexers,
      active_radarr_block
    );
  }

  #[test]
  fn test_delegates_test_all_indexers_block_to_test_all_indexers_handler() {
    test_handler_delegation!(
      IndexersHandler,
      ActiveRadarrBlock::Indexers,
      ActiveRadarrBlock::TestAllIndexers
    );
  }

  #[test]
  fn test_indexers_handler_accepts() {
    let mut indexers_blocks = Vec::new();
    indexers_blocks.extend(INDEXERS_BLOCKS);
    indexers_blocks.extend(INDEXER_SETTINGS_BLOCKS);
    indexers_blocks.extend(EDIT_INDEXER_BLOCKS);
    indexers_blocks.push(ActiveRadarrBlock::TestAllIndexers);

    ActiveRadarrBlock::iter().for_each(|active_radarr_block| {
      if indexers_blocks.contains(&active_radarr_block) {
        assert!(IndexersHandler::accepts(active_radarr_block));
      } else {
        assert!(!IndexersHandler::accepts(active_radarr_block));
      }
    })
  }

  #[rstest]
  fn test_indexers_handler_ignore_special_keys(
    #[values(true, false)] ignore_special_keys_for_textbox_input: bool,
  ) {
    let mut app = App::test_default();
    app.ignore_special_keys_for_textbox_input = ignore_special_keys_for_textbox_input;
    let handler = IndexersHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveRadarrBlock::default(),
      None,
    );

    assert_eq!(
      handler.ignore_special_keys(),
      ignore_special_keys_for_textbox_input
    );
  }

  #[test]
  fn test_extract_indexer_id() {
    let mut app = App::test_default();
    app.data.radarr_data.indexers.set_items(vec![indexer()]);

    let indexer_id = IndexersHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveRadarrBlock::Indexers,
      None,
    )
    .extract_indexer_id();

    assert_eq!(indexer_id, 1);
  }

  #[test]
  fn test_indexers_handler_not_ready_when_loading() {
    let mut app = App::test_default();
    app.is_loading = true;

    let handler = IndexersHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveRadarrBlock::Indexers,
      None,
    );

    assert!(!handler.is_ready());
  }

  #[test]
  fn test_indexers_handler_not_ready_when_indexers_is_empty() {
    let mut app = App::test_default();
    app.is_loading = false;

    let handler = IndexersHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveRadarrBlock::Indexers,
      None,
    );

    assert!(!handler.is_ready());
  }

  #[test]
  fn test_indexers_handler_ready_when_not_loading_and_indexers_is_not_empty() {
    let mut app = App::test_default();
    app.is_loading = false;
    app
      .data
      .radarr_data
      .indexers
      .set_items(vec![Indexer::default()]);

    let handler = IndexersHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveRadarrBlock::Indexers,
      None,
    );

    assert!(handler.is_ready());
  }
}
