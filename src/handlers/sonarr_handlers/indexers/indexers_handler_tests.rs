#[cfg(test)]
mod tests {
  use pretty_assertions::{assert_eq, assert_str_eq};
  use rstest::rstest;
  use strum::IntoEnumIterator;

  use crate::app::key_binding::DEFAULT_KEYBINDINGS;
  use crate::app::App;
  use crate::event::Key;
  use crate::handlers::sonarr_handlers::indexers::IndexersHandler;
  use crate::handlers::KeyEventHandler;
  use crate::models::servarr_data::sonarr::sonarr_data::{
    ActiveSonarrBlock, EDIT_INDEXER_BLOCKS, INDEXERS_BLOCKS, INDEXER_SETTINGS_BLOCKS,
  };
  use crate::models::servarr_models::Indexer;
  use crate::test_handler_delegation;

  mod test_handle_scroll_up_and_down {
    use rstest::rstest;

    use crate::{simple_stateful_iterable_vec, test_iterable_scroll};

    use super::*;

    test_iterable_scroll!(
      test_indexers_scroll,
      IndexersHandler,
      sonarr_data,
      indexers,
      simple_stateful_iterable_vec!(Indexer, String, protocol),
      ActiveSonarrBlock::Indexers,
      None,
      protocol
    );

    #[rstest]
    fn test_indexers_scroll_no_op_when_not_ready(
      #[values(
			DEFAULT_KEYBINDINGS.up.key, DEFAULT_KEYBINDINGS.down.key
		)]
      key: Key,
    ) {
      let mut app = App::default();
      app.push_navigation_stack(ActiveSonarrBlock::Indexers.into());
      app.is_loading = true;
      app
        .data
        .sonarr_data
        .indexers
        .set_items(simple_stateful_iterable_vec!(Indexer, String, protocol));

      IndexersHandler::with(key, &mut app, ActiveSonarrBlock::Indexers, None).handle();

      assert_str_eq!(
        app.data.sonarr_data.indexers.current_selection().protocol,
        "Test 1"
      );

      IndexersHandler::with(key, &mut app, ActiveSonarrBlock::Indexers, None).handle();

      assert_str_eq!(
        app.data.sonarr_data.indexers.current_selection().protocol,
        "Test 1"
      );
    }
  }

  mod test_handle_home_end {
    use crate::{extended_stateful_iterable_vec, test_iterable_home_and_end};

    use super::*;

    test_iterable_home_and_end!(
      test_indexers_home_end,
      IndexersHandler,
      sonarr_data,
      indexers,
      extended_stateful_iterable_vec!(Indexer, String, protocol),
      ActiveSonarrBlock::Indexers,
      None,
      protocol
    );

    #[test]
    fn test_indexers_home_end_no_op_when_not_ready() {
      let mut app = App::default();
      app.push_navigation_stack(ActiveSonarrBlock::Indexers.into());
      app.is_loading = true;
      app
        .data
        .sonarr_data
        .indexers
        .set_items(extended_stateful_iterable_vec!(Indexer, String, protocol));

      IndexersHandler::with(
        DEFAULT_KEYBINDINGS.end.key,
        &mut app,
        ActiveSonarrBlock::Indexers,
        None,
      )
      .handle();

      assert_str_eq!(
        app.data.sonarr_data.indexers.current_selection().protocol,
        "Test 1"
      );

      IndexersHandler::with(
        DEFAULT_KEYBINDINGS.home.key,
        &mut app,
        ActiveSonarrBlock::Indexers,
        None,
      )
      .handle();

      assert_str_eq!(
        app.data.sonarr_data.indexers.current_selection().protocol,
        "Test 1"
      );
    }
  }

  mod test_handle_delete {
    use pretty_assertions::assert_eq;

    use super::*;

    const DELETE_KEY: Key = DEFAULT_KEYBINDINGS.delete.key;

    #[test]
    fn test_delete_indexer_prompt() {
      let mut app = App::default();
      app.push_navigation_stack(ActiveSonarrBlock::Indexers.into());
      app
        .data
        .sonarr_data
        .indexers
        .set_items(vec![Indexer::default()]);

      IndexersHandler::with(DELETE_KEY, &mut app, ActiveSonarrBlock::Indexers, None).handle();

      assert_eq!(
        app.get_current_route(),
        ActiveSonarrBlock::DeleteIndexerPrompt.into()
      );
    }

    #[test]
    fn test_delete_indexer_prompt_no_op_when_not_ready() {
      let mut app = App::default();
      app.is_loading = true;
      app.push_navigation_stack(ActiveSonarrBlock::Indexers.into());
      app
        .data
        .sonarr_data
        .indexers
        .set_items(vec![Indexer::default()]);

      IndexersHandler::with(DELETE_KEY, &mut app, ActiveSonarrBlock::Indexers, None).handle();

      assert_eq!(app.get_current_route(), ActiveSonarrBlock::Indexers.into());
    }
  }

  mod test_handle_left_right_action {
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    use super::*;

    #[rstest]
    fn test_indexers_tab_left(#[values(true, false)] is_ready: bool) {
      let mut app = App::default();
      app.push_navigation_stack(ActiveSonarrBlock::Indexers.into());
      app.is_loading = is_ready;
      app.data.sonarr_data.main_tabs.set_index(5);

      IndexersHandler::with(
        DEFAULT_KEYBINDINGS.left.key,
        &mut app,
        ActiveSonarrBlock::Indexers,
        None,
      )
      .handle();

      assert_eq!(
        app.data.sonarr_data.main_tabs.get_active_route(),
        ActiveSonarrBlock::RootFolders.into()
      );
      assert_eq!(
        app.get_current_route(),
        ActiveSonarrBlock::RootFolders.into()
      );
    }

    #[rstest]
    fn test_indexers_tab_right(#[values(true, false)] is_ready: bool) {
      let mut app = App::default();
      app.push_navigation_stack(ActiveSonarrBlock::Indexers.into());
      app.is_loading = is_ready;
      app.data.sonarr_data.main_tabs.set_index(5);

      IndexersHandler::with(
        DEFAULT_KEYBINDINGS.right.key,
        &mut app,
        ActiveSonarrBlock::Indexers,
        None,
      )
      .handle();

      assert_eq!(
        app.data.sonarr_data.main_tabs.get_active_route(),
        ActiveSonarrBlock::System.into()
      );
      assert_eq!(app.get_current_route(), ActiveSonarrBlock::System.into());
    }

    #[rstest]
    fn test_left_right_delete_indexer_prompt_toggle(
      #[values(DEFAULT_KEYBINDINGS.left.key, DEFAULT_KEYBINDINGS.right.key)] key: Key,
    ) {
      let mut app = App::default();
      app.push_navigation_stack(ActiveSonarrBlock::Indexers.into());

      IndexersHandler::with(key, &mut app, ActiveSonarrBlock::DeleteIndexerPrompt, None).handle();

      assert!(app.data.sonarr_data.prompt_confirm);

      IndexersHandler::with(key, &mut app, ActiveSonarrBlock::DeleteIndexerPrompt, None).handle();

      assert!(!app.data.sonarr_data.prompt_confirm);
    }
  }

  mod test_handle_submit {
    use crate::models::servarr_data::modals::EditIndexerModal;
    use crate::models::servarr_data::sonarr::sonarr_data::{
      SonarrData, EDIT_INDEXER_NZB_SELECTION_BLOCKS, EDIT_INDEXER_TORRENT_SELECTION_BLOCKS,
    };
    use crate::models::servarr_models::{Indexer, IndexerField};
    use bimap::BiMap;
    use pretty_assertions::assert_eq;
    use serde_json::{Number, Value};

    use crate::network::sonarr_network::SonarrEvent;

    use super::*;

    const SUBMIT_KEY: Key = DEFAULT_KEYBINDINGS.submit.key;

    #[rstest]
    fn test_edit_indexer_submit(#[values(true, false)] torrent_protocol: bool) {
      let mut app = App::default();
      app.push_navigation_stack(ActiveSonarrBlock::Indexers.into());
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
      let mut sonarr_data = SonarrData {
        tags_map: BiMap::from_iter([(1, "usenet".to_owned()), (2, "test".to_owned())]),
        ..SonarrData::default()
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
      sonarr_data.indexers.set_items(vec![indexer]);
      app.data.sonarr_data = sonarr_data;

      IndexersHandler::with(SUBMIT_KEY, &mut app, ActiveSonarrBlock::Indexers, None).handle();

      assert_eq!(
        app.get_current_route(),
        ActiveSonarrBlock::EditIndexerPrompt.into()
      );
      assert_eq!(
        app.data.sonarr_data.edit_indexer_modal,
        Some((&app.data.sonarr_data).into())
      );
      assert_eq!(
        app.data.sonarr_data.edit_indexer_modal,
        Some(expected_edit_indexer_modal)
      );
      if torrent_protocol {
        assert_eq!(
          app.data.sonarr_data.selected_block.blocks,
          EDIT_INDEXER_TORRENT_SELECTION_BLOCKS
        );
      } else {
        assert_eq!(
          app.data.sonarr_data.selected_block.blocks,
          EDIT_INDEXER_NZB_SELECTION_BLOCKS
        );
      }
    }

    #[test]
    fn test_edit_indexer_submit_no_op_when_not_ready() {
      let mut app = App::default();
      app.push_navigation_stack(ActiveSonarrBlock::Indexers.into());
      app.is_loading = true;
      app.push_navigation_stack(ActiveSonarrBlock::Indexers.into());
      app
        .data
        .sonarr_data
        .indexers
        .set_items(vec![Indexer::default()]);

      IndexersHandler::with(SUBMIT_KEY, &mut app, ActiveSonarrBlock::Indexers, None).handle();

      assert_eq!(app.get_current_route(), ActiveSonarrBlock::Indexers.into());
      assert_eq!(app.data.sonarr_data.edit_indexer_modal, None);
    }

    #[test]
    fn test_delete_indexer_prompt_confirm_submit() {
      let mut app = App::default();
      app
        .data
        .sonarr_data
        .indexers
        .set_items(vec![Indexer::default()]);
      app.data.sonarr_data.prompt_confirm = true;
      app.push_navigation_stack(ActiveSonarrBlock::Indexers.into());
      app.push_navigation_stack(ActiveSonarrBlock::DeleteIndexerPrompt.into());

      IndexersHandler::with(
        SUBMIT_KEY,
        &mut app,
        ActiveSonarrBlock::DeleteIndexerPrompt,
        None,
      )
      .handle();

      assert!(app.data.sonarr_data.prompt_confirm);
      assert_eq!(
        app.data.sonarr_data.prompt_confirm_action,
        Some(SonarrEvent::DeleteIndexer(None))
      );
      assert_eq!(app.get_current_route(), ActiveSonarrBlock::Indexers.into());
    }

    #[test]
    fn test_prompt_decline_submit() {
      let mut app = App::default();
      app
        .data
        .sonarr_data
        .indexers
        .set_items(vec![Indexer::default()]);
      app.push_navigation_stack(ActiveSonarrBlock::Indexers.into());
      app.push_navigation_stack(ActiveSonarrBlock::DeleteIndexerPrompt.into());

      IndexersHandler::with(
        SUBMIT_KEY,
        &mut app,
        ActiveSonarrBlock::DeleteIndexerPrompt,
        None,
      )
      .handle();

      assert!(!app.data.sonarr_data.prompt_confirm);
      assert_eq!(app.data.sonarr_data.prompt_confirm_action, None);
      assert_eq!(app.get_current_route(), ActiveSonarrBlock::Indexers.into());
    }
  }

  mod test_handle_esc {
    use pretty_assertions::assert_eq;

    use super::*;

    const ESC_KEY: Key = DEFAULT_KEYBINDINGS.esc.key;

    #[rstest]
    fn test_delete_indexer_prompt_block_esc(#[values(true, false)] is_ready: bool) {
      let mut app = App::default();
      app.is_loading = is_ready;
      app.push_navigation_stack(ActiveSonarrBlock::Indexers.into());
      app.push_navigation_stack(ActiveSonarrBlock::DeleteIndexerPrompt.into());
      app.data.sonarr_data.prompt_confirm = true;

      IndexersHandler::with(
        ESC_KEY,
        &mut app,
        ActiveSonarrBlock::DeleteIndexerPrompt,
        None,
      )
      .handle();

      assert_eq!(app.get_current_route(), ActiveSonarrBlock::Indexers.into());
      assert!(!app.data.sonarr_data.prompt_confirm);
    }

    #[rstest]
    fn test_test_indexer_esc(#[values(true, false)] is_ready: bool) {
      let mut app = App::default();
      app.is_loading = is_ready;
      app.data.sonarr_data.indexer_test_error = Some("test result".to_owned());
      app.push_navigation_stack(ActiveSonarrBlock::Indexers.into());
      app.push_navigation_stack(ActiveSonarrBlock::TestIndexer.into());

      IndexersHandler::with(ESC_KEY, &mut app, ActiveSonarrBlock::TestIndexer, None).handle();

      assert_eq!(app.get_current_route(), ActiveSonarrBlock::Indexers.into());
      assert_eq!(app.data.sonarr_data.indexer_test_error, None);
    }

    #[rstest]
    fn test_default_esc(#[values(true, false)] is_ready: bool) {
      let mut app = App::default();
      app.is_loading = is_ready;
      app.error = "test error".to_owned().into();
      app.push_navigation_stack(ActiveSonarrBlock::Indexers.into());
      app.push_navigation_stack(ActiveSonarrBlock::Indexers.into());

      IndexersHandler::with(ESC_KEY, &mut app, ActiveSonarrBlock::Indexers, None).handle();

      assert_eq!(app.get_current_route(), ActiveSonarrBlock::Indexers.into());
      assert!(app.error.text.is_empty());
    }
  }

  mod test_handle_key_char {
    use pretty_assertions::assert_eq;

    use crate::{
      models::servarr_data::sonarr::sonarr_data::INDEXER_SETTINGS_SELECTION_BLOCKS,
      network::sonarr_network::SonarrEvent,
    };

    use super::*;

    #[test]
    fn test_refresh_indexers_key() {
      let mut app = App::default();
      app
        .data
        .sonarr_data
        .indexers
        .set_items(vec![Indexer::default()]);
      app.push_navigation_stack(ActiveSonarrBlock::Indexers.into());

      IndexersHandler::with(
        DEFAULT_KEYBINDINGS.refresh.key,
        &mut app,
        ActiveSonarrBlock::Indexers,
        None,
      )
      .handle();

      assert_eq!(app.get_current_route(), ActiveSonarrBlock::Indexers.into());
      assert!(app.should_refresh);
    }

    #[test]
    fn test_refresh_indexers_key_no_op_when_not_ready() {
      let mut app = App::default();
      app.is_loading = true;
      app
        .data
        .sonarr_data
        .indexers
        .set_items(vec![Indexer::default()]);
      app.push_navigation_stack(ActiveSonarrBlock::Indexers.into());

      IndexersHandler::with(
        DEFAULT_KEYBINDINGS.refresh.key,
        &mut app,
        ActiveSonarrBlock::Indexers,
        None,
      )
      .handle();

      assert_eq!(app.get_current_route(), ActiveSonarrBlock::Indexers.into());
      assert!(!app.should_refresh);
    }

    #[test]
    fn test_indexer_settings_key() {
      let mut app = App::default();
      app.data.sonarr_data.indexers.set_items(vec![Indexer::default()]);
      app.push_navigation_stack(ActiveSonarrBlock::Indexers.into());
      app
        .data
        .sonarr_data
        .indexers
        .set_items(vec![Indexer::default()]);

      IndexersHandler::with(
        DEFAULT_KEYBINDINGS.settings.key,
        &mut app,
        ActiveSonarrBlock::Indexers,
        None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveSonarrBlock::AllIndexerSettingsPrompt.into()
      );
      assert_eq!(
        app.data.sonarr_data.selected_block.blocks,
        INDEXER_SETTINGS_SELECTION_BLOCKS
      );
    }

    #[test]
    fn test_indexer_settings_key_no_op_when_not_ready() {
      let mut app = App::default();
      app.is_loading = true;
      app.push_navigation_stack(ActiveSonarrBlock::Indexers.into());
      app
        .data
        .sonarr_data
        .indexers
        .set_items(vec![Indexer::default()]);

      IndexersHandler::with(
        DEFAULT_KEYBINDINGS.settings.key,
        &mut app,
        ActiveSonarrBlock::Indexers,
        None,
      )
      .handle();

      assert_eq!(app.get_current_route(), ActiveSonarrBlock::Indexers.into());
    }

    #[test]
    fn test_test_key() {
      let mut app = App::default();
      app.push_navigation_stack(ActiveSonarrBlock::Indexers.into());
      app
        .data
        .sonarr_data
        .indexers
        .set_items(vec![Indexer::default()]);

      IndexersHandler::with(
        DEFAULT_KEYBINDINGS.test.key,
        &mut app,
        ActiveSonarrBlock::Indexers,
        None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveSonarrBlock::TestIndexer.into()
      );
    }

    #[test]
    fn test_test_key_no_op_when_not_ready() {
      let mut app = App::default();
      app.is_loading = true;
      app.push_navigation_stack(ActiveSonarrBlock::Indexers.into());
      app
        .data
        .sonarr_data
        .indexers
        .set_items(vec![Indexer::default()]);

      IndexersHandler::with(
        DEFAULT_KEYBINDINGS.test.key,
        &mut app,
        ActiveSonarrBlock::Indexers,
        None,
      )
      .handle();

      assert_eq!(app.get_current_route(), ActiveSonarrBlock::Indexers.into());
    }

    #[test]
    fn test_test_all_key() {
      let mut app = App::default();
      app.push_navigation_stack(ActiveSonarrBlock::Indexers.into());
      app
        .data
        .sonarr_data
        .indexers
        .set_items(vec![Indexer::default()]);

      IndexersHandler::with(
        DEFAULT_KEYBINDINGS.test_all.key,
        &mut app,
        ActiveSonarrBlock::Indexers,
        None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveSonarrBlock::TestAllIndexers.into()
      );
    }

    #[test]
    fn test_test_all_key_no_op_when_not_ready() {
      let mut app = App::default();
      app.is_loading = true;
      app.push_navigation_stack(ActiveSonarrBlock::Indexers.into());
      app
        .data
        .sonarr_data
        .indexers
        .set_items(vec![Indexer::default()]);

      IndexersHandler::with(
        DEFAULT_KEYBINDINGS.test_all.key,
        &mut app,
        ActiveSonarrBlock::Indexers,
        None,
      )
      .handle();

      assert_eq!(app.get_current_route(), ActiveSonarrBlock::Indexers.into());
    }

    #[test]
    fn test_delete_indexer_prompt_confirm() {
      let mut app = App::default();
      app
        .data
        .sonarr_data
        .indexers
        .set_items(vec![Indexer::default()]);
      app.push_navigation_stack(ActiveSonarrBlock::Indexers.into());
      app.push_navigation_stack(ActiveSonarrBlock::DeleteIndexerPrompt.into());

      IndexersHandler::with(
        DEFAULT_KEYBINDINGS.confirm.key,
        &mut app,
        ActiveSonarrBlock::DeleteIndexerPrompt,
        None,
      )
      .handle();

      assert!(app.data.sonarr_data.prompt_confirm);
      assert_eq!(
        app.data.sonarr_data.prompt_confirm_action,
        Some(SonarrEvent::DeleteIndexer(None))
      );
      assert_eq!(app.get_current_route(), ActiveSonarrBlock::Indexers.into());
    }
  }

  #[rstest]
  fn test_delegates_edit_indexer_blocks_to_edit_indexer_handler(
    #[values(
      ActiveSonarrBlock::EditIndexerPrompt,
      ActiveSonarrBlock::EditIndexerConfirmPrompt,
      ActiveSonarrBlock::EditIndexerApiKeyInput,
      ActiveSonarrBlock::EditIndexerNameInput,
      ActiveSonarrBlock::EditIndexerSeedRatioInput,
      ActiveSonarrBlock::EditIndexerToggleEnableRss,
      ActiveSonarrBlock::EditIndexerToggleEnableAutomaticSearch,
      ActiveSonarrBlock::EditIndexerToggleEnableInteractiveSearch,
      ActiveSonarrBlock::EditIndexerUrlInput,
      ActiveSonarrBlock::EditIndexerTagsInput
    )]
    active_sonarr_block: ActiveSonarrBlock,
  ) {
    test_handler_delegation!(
      IndexersHandler,
      ActiveSonarrBlock::Indexers,
      active_sonarr_block
    );
  }

  #[rstest]
  fn test_delegates_indexer_settings_blocks_to_indexer_settings_handler(
    #[values(
      ActiveSonarrBlock::AllIndexerSettingsPrompt,
      ActiveSonarrBlock::IndexerSettingsConfirmPrompt,
      ActiveSonarrBlock::IndexerSettingsMaximumSizeInput,
      ActiveSonarrBlock::IndexerSettingsMinimumAgeInput,
      ActiveSonarrBlock::IndexerSettingsRetentionInput,
      ActiveSonarrBlock::IndexerSettingsRssSyncIntervalInput
    )]
    active_sonarr_block: ActiveSonarrBlock,
  ) {
    test_handler_delegation!(
      IndexersHandler,
      ActiveSonarrBlock::Indexers,
      active_sonarr_block
    );
  }

  #[test]
  fn test_delegates_test_all_indexers_block_to_test_all_indexers_handler() {
    test_handler_delegation!(
      IndexersHandler,
      ActiveSonarrBlock::Indexers,
      ActiveSonarrBlock::TestAllIndexers
    );
  }

  #[test]
  fn test_indexers_handler_accepts() {
    let mut indexers_blocks = Vec::new();
    indexers_blocks.extend(INDEXERS_BLOCKS);
    indexers_blocks.extend(INDEXER_SETTINGS_BLOCKS);
    indexers_blocks.extend(EDIT_INDEXER_BLOCKS);
    indexers_blocks.push(ActiveSonarrBlock::TestAllIndexers);

    ActiveSonarrBlock::iter().for_each(|active_sonarr_block| {
      if indexers_blocks.contains(&active_sonarr_block) {
        assert!(IndexersHandler::accepts(active_sonarr_block));
      } else {
        assert!(!IndexersHandler::accepts(active_sonarr_block));
      }
    })
  }

  #[test]
  fn test_indexers_handler_not_ready_when_loading() {
    let mut app = App::default();
    app.push_navigation_stack(ActiveSonarrBlock::Indexers.into());
    app.is_loading = true;

    let handler = IndexersHandler::with(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveSonarrBlock::Indexers,
      None,
    );

    assert!(!handler.is_ready());
  }

  #[test]
  fn test_indexers_handler_not_ready_when_indexers_is_empty() {
    let mut app = App::default();
    app.push_navigation_stack(ActiveSonarrBlock::Indexers.into());
    app.is_loading = false;

    let handler = IndexersHandler::with(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveSonarrBlock::Indexers,
      None,
    );

    assert!(!handler.is_ready());
  }

  #[test]
  fn test_indexers_handler_ready_when_not_loading_and_indexers_is_not_empty() {
    let mut app = App::default();
    app.push_navigation_stack(ActiveSonarrBlock::Indexers.into());
    app.is_loading = false;
    app
      .data
      .sonarr_data
      .indexers
      .set_items(vec![Indexer::default()]);

    let handler = IndexersHandler::with(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveSonarrBlock::Indexers,
      None,
    );

    assert!(handler.is_ready());
  }
}
