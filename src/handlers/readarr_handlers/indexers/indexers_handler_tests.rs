#[cfg(test)]
mod tests {
  use pretty_assertions::assert_eq;
  use rstest::rstest;
  use strum::IntoEnumIterator;

  use crate::app::App;
  use crate::app::key_binding::DEFAULT_KEYBINDINGS;
  use crate::assert_navigation_pushed;
  use crate::event::Key;
  use crate::handlers::KeyEventHandler;
  use crate::handlers::readarr_handlers::indexers::IndexersHandler;
  use crate::models::servarr_data::readarr::readarr_data::{
    ActiveReadarrBlock, EDIT_INDEXER_BLOCKS, INDEXER_SETTINGS_BLOCKS, INDEXERS_BLOCKS,
  };
  use crate::models::servarr_models::Indexer;
  use crate::network::readarr_network::readarr_network_test_utils::test_utils::indexer;
  use crate::test_handler_delegation;

  mod test_handle_delete {
    use pretty_assertions::assert_eq;

    use super::*;

    const DELETE_KEY: Key = DEFAULT_KEYBINDINGS.delete.key;

    #[test]
    fn test_delete_indexer_prompt() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveReadarrBlock::Indexers.into());
      app
        .data
        .readarr_data
        .indexers
        .set_items(vec![Indexer::default()]);

      IndexersHandler::new(DELETE_KEY, &mut app, ActiveReadarrBlock::Indexers, None).handle();

      assert_navigation_pushed!(app, ActiveReadarrBlock::DeleteIndexerPrompt.into());
    }

    #[test]
    fn test_delete_indexer_prompt_no_op_when_not_ready() {
      let mut app = App::test_default();
      app.is_loading = true;
      app.push_navigation_stack(ActiveReadarrBlock::Indexers.into());
      app
        .data
        .readarr_data
        .indexers
        .set_items(vec![Indexer::default()]);

      IndexersHandler::new(DELETE_KEY, &mut app, ActiveReadarrBlock::Indexers, None).handle();

      assert_eq!(app.get_current_route(), ActiveReadarrBlock::Indexers.into());
    }
  }

  mod test_handle_left_right_action {
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    use super::*;

    #[rstest]
    fn test_indexers_tab_left(#[values(true, false)] is_ready: bool) {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveReadarrBlock::Indexers.into());
      app.is_loading = is_ready;
      app.data.readarr_data.main_tabs.set_index(5);

      IndexersHandler::new(
        DEFAULT_KEYBINDINGS.left.key,
        &mut app,
        ActiveReadarrBlock::Indexers,
        None,
      )
      .handle();

      assert_eq!(
        app.data.readarr_data.main_tabs.get_active_route(),
        ActiveReadarrBlock::RootFolders.into()
      );
      assert_navigation_pushed!(app, ActiveReadarrBlock::RootFolders.into());
    }

    #[rstest]
    fn test_indexers_tab_right(#[values(true, false)] is_ready: bool) {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveReadarrBlock::Indexers.into());
      app.is_loading = is_ready;
      app.data.readarr_data.main_tabs.set_index(5);

      IndexersHandler::new(
        DEFAULT_KEYBINDINGS.right.key,
        &mut app,
        ActiveReadarrBlock::Indexers,
        None,
      )
      .handle();

      assert_eq!(
        app.data.readarr_data.main_tabs.get_active_route(),
        ActiveReadarrBlock::System.into()
      );
      assert_navigation_pushed!(app, ActiveReadarrBlock::System.into());
    }

    #[rstest]
    fn test_left_right_delete_indexer_prompt_toggle(
      #[values(DEFAULT_KEYBINDINGS.left.key, DEFAULT_KEYBINDINGS.right.key)] key: Key,
    ) {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveReadarrBlock::Indexers.into());

      IndexersHandler::new(key, &mut app, ActiveReadarrBlock::DeleteIndexerPrompt, None).handle();

      assert!(app.data.readarr_data.prompt_confirm);

      IndexersHandler::new(key, &mut app, ActiveReadarrBlock::DeleteIndexerPrompt, None).handle();

      assert!(!app.data.readarr_data.prompt_confirm);
    }
  }

  mod test_handle_submit {
    use super::*;
    use crate::assert_navigation_popped;
    use crate::models::servarr_data::modals::EditIndexerModal;
    use crate::models::servarr_data::readarr::readarr_data::{
      EDIT_INDEXER_NZB_SELECTION_BLOCKS, EDIT_INDEXER_TORRENT_SELECTION_BLOCKS, ReadarrData,
    };
    use crate::models::servarr_models::{Indexer, IndexerField};
    use crate::network::readarr_network::ReadarrEvent;
    use crate::network::readarr_network::readarr_network_test_utils::test_utils::indexer;
    use bimap::BiMap;
    use pretty_assertions::assert_eq;
    use serde_json::{Number, Value};

    const SUBMIT_KEY: Key = DEFAULT_KEYBINDINGS.submit.key;

    #[rstest]
    fn test_edit_indexer_submit(#[values(true, false)] torrent_protocol: bool) {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveReadarrBlock::Indexers.into());
      let protocol = if torrent_protocol {
        "torrent".to_owned()
      } else {
        "usenet".to_owned()
      };
      let decoy_protocol = if torrent_protocol {
        "usenet".to_owned()
      } else {
        "torrent".to_owned()
      };
      let mut expected_edit_indexer_modal = EditIndexerModal {
        name: "Test".into(),
        enable_rss: Some(true),
        enable_automatic_search: Some(false),
        enable_interactive_search: Some(true),
        url: "https://test.com".into(),
        api_key: "1234".into(),
        tags: "usenet, test".into(),
        ..EditIndexerModal::default()
      };
      let mut readarr_data = ReadarrData {
        tags_map: BiMap::from_iter([(1, "usenet".to_owned()), (2, "test".to_owned())]),
        ..ReadarrData::default()
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
        enable_automatic_search: false,
        enable_interactive_search: true,
        protocol,
        tags: vec![Number::from(1), Number::from(2)],
        fields: Some(fields),
        ..Indexer::default()
      };
      let decoy_indexer = Indexer {
        name: Some("Decoy".to_owned()),
        enable_rss: false,
        enable_automatic_search: true,
        enable_interactive_search: false,
        protocol: decoy_protocol,
        tags: vec![Number::from(2)],
        fields: Some(vec![
          IndexerField {
            name: Some("baseUrl".to_owned()),
            value: Some(Value::String("https://decoy.com".to_owned())),
          },
          IndexerField {
            name: Some("apiKey".to_owned()),
            value: Some(Value::String("9999".to_owned())),
          },
        ]),
        ..Indexer::default()
      };
      readarr_data
        .indexers
        .set_items(vec![decoy_indexer, indexer]);
      readarr_data.indexers.select_index(Some(1));
      app.data.readarr_data = readarr_data;

      IndexersHandler::new(SUBMIT_KEY, &mut app, ActiveReadarrBlock::Indexers, None).handle();

      assert_navigation_pushed!(app, ActiveReadarrBlock::EditIndexerPrompt.into());
      assert_some_eq_x!(
        &app.data.readarr_data.edit_indexer_modal,
        &EditIndexerModal::from(&app.data.readarr_data)
      );
      assert_some_eq_x!(
        &app.data.readarr_data.edit_indexer_modal,
        &expected_edit_indexer_modal
      );
      if torrent_protocol {
        assert_eq!(
          app.data.readarr_data.selected_block.blocks,
          EDIT_INDEXER_TORRENT_SELECTION_BLOCKS
        );
      } else {
        assert_eq!(
          app.data.readarr_data.selected_block.blocks,
          EDIT_INDEXER_NZB_SELECTION_BLOCKS
        );
      }
    }

    #[test]
    fn test_edit_indexer_submit_no_op_when_not_ready() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveReadarrBlock::Indexers.into());
      app.is_loading = true;
      app.push_navigation_stack(ActiveReadarrBlock::Indexers.into());
      app
        .data
        .readarr_data
        .indexers
        .set_items(vec![Indexer::default()]);

      IndexersHandler::new(SUBMIT_KEY, &mut app, ActiveReadarrBlock::Indexers, None).handle();

      assert_eq!(app.get_current_route(), ActiveReadarrBlock::Indexers.into());
      assert_none!(app.data.readarr_data.edit_indexer_modal);
    }

    #[test]
    fn test_delete_indexer_prompt_confirm_submit() {
      let mut app = App::test_default();
      app.data.readarr_data.indexers.set_items(vec![indexer()]);
      app.data.readarr_data.prompt_confirm = true;
      app.push_navigation_stack(ActiveReadarrBlock::Indexers.into());
      app.push_navigation_stack(ActiveReadarrBlock::DeleteIndexerPrompt.into());

      IndexersHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveReadarrBlock::DeleteIndexerPrompt,
        None,
      )
      .handle();

      assert!(app.data.readarr_data.prompt_confirm);
      assert_some_eq_x!(
        &app.data.readarr_data.prompt_confirm_action,
        &ReadarrEvent::DeleteIndexer(8)
      );
      assert_navigation_popped!(app, ActiveReadarrBlock::Indexers.into());
    }

    #[test]
    fn test_prompt_decline_submit() {
      let mut app = App::test_default();
      app
        .data
        .readarr_data
        .indexers
        .set_items(vec![Indexer::default()]);
      app.push_navigation_stack(ActiveReadarrBlock::Indexers.into());
      app.push_navigation_stack(ActiveReadarrBlock::DeleteIndexerPrompt.into());

      IndexersHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveReadarrBlock::DeleteIndexerPrompt,
        None,
      )
      .handle();

      assert!(!app.data.readarr_data.prompt_confirm);
      assert_none!(app.data.readarr_data.prompt_confirm_action);
      assert_navigation_popped!(app, ActiveReadarrBlock::Indexers.into());
    }
  }

  mod test_handle_esc {

    use super::*;
    use crate::assert_navigation_popped;

    const ESC_KEY: Key = DEFAULT_KEYBINDINGS.esc.key;

    #[rstest]
    fn test_delete_indexer_prompt_block_esc(#[values(true, false)] is_ready: bool) {
      let mut app = App::test_default();
      app.is_loading = is_ready;
      app.push_navigation_stack(ActiveReadarrBlock::Indexers.into());
      app.push_navigation_stack(ActiveReadarrBlock::DeleteIndexerPrompt.into());
      app.data.readarr_data.prompt_confirm = true;

      IndexersHandler::new(
        ESC_KEY,
        &mut app,
        ActiveReadarrBlock::DeleteIndexerPrompt,
        None,
      )
      .handle();

      assert_navigation_popped!(app, ActiveReadarrBlock::Indexers.into());
      assert!(!app.data.readarr_data.prompt_confirm);
    }

    #[rstest]
    fn test_test_indexer_esc(#[values(true, false)] is_ready: bool) {
      let mut app = App::test_default();
      app.is_loading = is_ready;
      app.data.readarr_data.indexer_test_errors = Some("test result".to_owned());
      app.push_navigation_stack(ActiveReadarrBlock::Indexers.into());
      app.push_navigation_stack(ActiveReadarrBlock::TestIndexer.into());

      IndexersHandler::new(ESC_KEY, &mut app, ActiveReadarrBlock::TestIndexer, None).handle();

      assert_navigation_popped!(app, ActiveReadarrBlock::Indexers.into());
      assert_none!(app.data.readarr_data.indexer_test_errors);
    }

    #[rstest]
    fn test_default_esc(#[values(true, false)] is_ready: bool) {
      let mut app = App::test_default();
      app.is_loading = is_ready;
      app.error = "test error".to_owned().into();
      app.push_navigation_stack(ActiveReadarrBlock::Indexers.into());
      app.push_navigation_stack(ActiveReadarrBlock::Indexers.into());

      IndexersHandler::new(ESC_KEY, &mut app, ActiveReadarrBlock::Indexers, None).handle();

      assert_navigation_popped!(app, ActiveReadarrBlock::Indexers.into());
      assert_is_empty!(app.error.text);
    }
  }

  mod test_handle_key_char {
    use pretty_assertions::assert_eq;

    use super::*;
    use crate::network::readarr_network::readarr_network_test_utils::test_utils::indexer;
    use crate::{
      assert_navigation_popped,
      models::servarr_data::readarr::readarr_data::INDEXER_SETTINGS_SELECTION_BLOCKS,
      network::readarr_network::ReadarrEvent,
    };

    #[test]
    fn test_refresh_indexers_key() {
      let mut app = App::test_default();
      app
        .data
        .readarr_data
        .indexers
        .set_items(vec![Indexer::default()]);
      app.push_navigation_stack(ActiveReadarrBlock::Indexers.into());

      IndexersHandler::new(
        DEFAULT_KEYBINDINGS.refresh.key,
        &mut app,
        ActiveReadarrBlock::Indexers,
        None,
      )
      .handle();

      assert_navigation_pushed!(app, ActiveReadarrBlock::Indexers.into());
      assert!(app.should_refresh);
    }

    #[test]
    fn test_refresh_indexers_key_no_op_when_not_ready() {
      let mut app = App::test_default();
      app.is_loading = true;
      app
        .data
        .readarr_data
        .indexers
        .set_items(vec![Indexer::default()]);
      app.push_navigation_stack(ActiveReadarrBlock::Indexers.into());

      IndexersHandler::new(
        DEFAULT_KEYBINDINGS.refresh.key,
        &mut app,
        ActiveReadarrBlock::Indexers,
        None,
      )
      .handle();

      assert_eq!(app.get_current_route(), ActiveReadarrBlock::Indexers.into());
      assert!(!app.should_refresh);
    }

    #[test]
    fn test_indexer_settings_key() {
      let mut app = App::test_default();
      app
        .data
        .readarr_data
        .indexers
        .set_items(vec![Indexer::default()]);
      app.push_navigation_stack(ActiveReadarrBlock::Indexers.into());
      app
        .data
        .readarr_data
        .indexers
        .set_items(vec![Indexer::default()]);

      IndexersHandler::new(
        DEFAULT_KEYBINDINGS.settings.key,
        &mut app,
        ActiveReadarrBlock::Indexers,
        None,
      )
      .handle();

      assert_navigation_pushed!(app, ActiveReadarrBlock::AllIndexerSettingsPrompt.into());
      assert_eq!(
        app.data.readarr_data.selected_block.blocks,
        INDEXER_SETTINGS_SELECTION_BLOCKS
      );
    }

    #[test]
    fn test_indexer_settings_key_no_op_when_not_ready() {
      let mut app = App::test_default();
      app.is_loading = true;
      app.push_navigation_stack(ActiveReadarrBlock::Indexers.into());
      app
        .data
        .readarr_data
        .indexers
        .set_items(vec![Indexer::default()]);

      IndexersHandler::new(
        DEFAULT_KEYBINDINGS.settings.key,
        &mut app,
        ActiveReadarrBlock::Indexers,
        None,
      )
      .handle();

      assert_eq!(app.get_current_route(), ActiveReadarrBlock::Indexers.into());
    }

    #[test]
    fn test_test_key() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveReadarrBlock::Indexers.into());
      app
        .data
        .readarr_data
        .indexers
        .set_items(vec![Indexer::default()]);

      IndexersHandler::new(
        DEFAULT_KEYBINDINGS.test.key,
        &mut app,
        ActiveReadarrBlock::Indexers,
        None,
      )
      .handle();

      assert_navigation_pushed!(app, ActiveReadarrBlock::TestIndexer.into());
    }

    #[test]
    fn test_test_key_no_op_when_not_ready() {
      let mut app = App::test_default();
      app.is_loading = true;
      app.push_navigation_stack(ActiveReadarrBlock::Indexers.into());
      app
        .data
        .readarr_data
        .indexers
        .set_items(vec![Indexer::default()]);

      IndexersHandler::new(
        DEFAULT_KEYBINDINGS.test.key,
        &mut app,
        ActiveReadarrBlock::Indexers,
        None,
      )
      .handle();

      assert_eq!(app.get_current_route(), ActiveReadarrBlock::Indexers.into());
    }

    #[test]
    fn test_test_all_key() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveReadarrBlock::Indexers.into());
      app
        .data
        .readarr_data
        .indexers
        .set_items(vec![Indexer::default()]);

      IndexersHandler::new(
        DEFAULT_KEYBINDINGS.test_all.key,
        &mut app,
        ActiveReadarrBlock::Indexers,
        None,
      )
      .handle();

      assert_navigation_pushed!(app, ActiveReadarrBlock::TestAllIndexers.into());
    }

    #[test]
    fn test_test_all_key_no_op_when_not_ready() {
      let mut app = App::test_default();
      app.is_loading = true;
      app.push_navigation_stack(ActiveReadarrBlock::Indexers.into());
      app
        .data
        .readarr_data
        .indexers
        .set_items(vec![Indexer::default()]);

      IndexersHandler::new(
        DEFAULT_KEYBINDINGS.test_all.key,
        &mut app,
        ActiveReadarrBlock::Indexers,
        None,
      )
      .handle();

      assert_eq!(app.get_current_route(), ActiveReadarrBlock::Indexers.into());
    }

    #[test]
    fn test_delete_indexer_prompt_confirm() {
      let mut app = App::test_default();
      app.data.readarr_data.indexers.set_items(vec![indexer()]);
      app.push_navigation_stack(ActiveReadarrBlock::Indexers.into());
      app.push_navigation_stack(ActiveReadarrBlock::DeleteIndexerPrompt.into());

      IndexersHandler::new(
        DEFAULT_KEYBINDINGS.confirm.key,
        &mut app,
        ActiveReadarrBlock::DeleteIndexerPrompt,
        None,
      )
      .handle();

      assert!(app.data.readarr_data.prompt_confirm);
      assert_some_eq_x!(
        &app.data.readarr_data.prompt_confirm_action,
        &ReadarrEvent::DeleteIndexer(8)
      );
      assert_navigation_popped!(app, ActiveReadarrBlock::Indexers.into());
    }
  }

  #[rstest]
  fn test_delegates_edit_indexer_blocks_to_edit_indexer_handler(
    #[values(
      ActiveReadarrBlock::EditIndexerPrompt,
      ActiveReadarrBlock::EditIndexerConfirmPrompt,
      ActiveReadarrBlock::EditIndexerApiKeyInput,
      ActiveReadarrBlock::EditIndexerNameInput,
      ActiveReadarrBlock::EditIndexerSeedRatioInput,
      ActiveReadarrBlock::EditIndexerToggleEnableRss,
      ActiveReadarrBlock::EditIndexerToggleEnableAutomaticSearch,
      ActiveReadarrBlock::EditIndexerToggleEnableInteractiveSearch,
      ActiveReadarrBlock::EditIndexerUrlInput,
      ActiveReadarrBlock::EditIndexerTagsInput
    )]
    active_readarr_block: ActiveReadarrBlock,
  ) {
    test_handler_delegation!(
      IndexersHandler,
      ActiveReadarrBlock::Indexers,
      active_readarr_block
    );
  }

  #[rstest]
  fn test_delegates_indexer_settings_blocks_to_indexer_settings_handler(
    #[values(
      ActiveReadarrBlock::AllIndexerSettingsPrompt,
      ActiveReadarrBlock::IndexerSettingsConfirmPrompt,
      ActiveReadarrBlock::IndexerSettingsMaximumSizeInput,
      ActiveReadarrBlock::IndexerSettingsMinimumAgeInput,
      ActiveReadarrBlock::IndexerSettingsRetentionInput,
      ActiveReadarrBlock::IndexerSettingsRssSyncIntervalInput
    )]
    active_readarr_block: ActiveReadarrBlock,
  ) {
    test_handler_delegation!(
      IndexersHandler,
      ActiveReadarrBlock::Indexers,
      active_readarr_block
    );
  }

  #[test]
  fn test_delegates_test_all_indexers_block_to_test_all_indexers_handler() {
    test_handler_delegation!(
      IndexersHandler,
      ActiveReadarrBlock::Indexers,
      ActiveReadarrBlock::TestAllIndexers
    );
  }

  #[test]
  fn test_indexers_handler_accepts() {
    let mut indexers_blocks = Vec::new();
    indexers_blocks.extend(INDEXERS_BLOCKS);
    indexers_blocks.extend(INDEXER_SETTINGS_BLOCKS);
    indexers_blocks.extend(EDIT_INDEXER_BLOCKS);
    indexers_blocks.push(ActiveReadarrBlock::TestAllIndexers);

    ActiveReadarrBlock::iter().for_each(|active_readarr_block| {
      if indexers_blocks.contains(&active_readarr_block) {
        assert!(IndexersHandler::accepts(active_readarr_block));
      } else {
        assert!(!IndexersHandler::accepts(active_readarr_block));
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
      ActiveReadarrBlock::default(),
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
    let decoy_indexer = Indexer { id: 3, ..indexer() };
    app
      .data
      .readarr_data
      .indexers
      .set_items(vec![decoy_indexer, indexer()]);
    app.data.readarr_data.indexers.select_index(Some(1));

    let indexer_id = IndexersHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveReadarrBlock::Indexers,
      None,
    )
    .extract_indexer_id();

    assert_eq!(indexer_id, 8);
  }

  #[test]
  fn test_indexers_handler_not_ready_when_loading() {
    let mut app = App::test_default();
    app.push_navigation_stack(ActiveReadarrBlock::Indexers.into());
    app.is_loading = true;

    let handler = IndexersHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveReadarrBlock::Indexers,
      None,
    );

    assert!(!handler.is_ready());
  }

  #[test]
  fn test_indexers_handler_not_ready_when_indexers_is_empty() {
    let mut app = App::test_default();
    app.push_navigation_stack(ActiveReadarrBlock::Indexers.into());
    app.is_loading = false;

    let handler = IndexersHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveReadarrBlock::Indexers,
      None,
    );

    assert!(!handler.is_ready());
  }

  #[test]
  fn test_indexers_handler_ready_when_not_loading_and_indexers_is_not_empty() {
    let mut app = App::test_default();
    app.push_navigation_stack(ActiveReadarrBlock::Indexers.into());
    app.is_loading = false;
    app
      .data
      .readarr_data
      .indexers
      .set_items(vec![Indexer::default()]);

    let handler = IndexersHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveReadarrBlock::Indexers,
      None,
    );

    assert!(handler.is_ready());
  }
}
