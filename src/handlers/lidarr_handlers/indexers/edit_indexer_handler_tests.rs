#[cfg(test)]
mod tests {
  use crate::app::App;
  use crate::app::key_binding::DEFAULT_KEYBINDINGS;
  use crate::assert_modal_absent;
  use crate::assert_modal_present;
  use crate::assert_navigation_pushed;
  use crate::event::Key;
  use crate::handlers::KeyEventHandler;
  use crate::handlers::lidarr_handlers::indexers::edit_indexer_handler::EditIndexerHandler;
  use crate::models::servarr_data::lidarr::lidarr_data::{ActiveLidarrBlock, EDIT_INDEXER_BLOCKS};
  use crate::models::servarr_data::modals::EditIndexerModal;
  use crate::models::servarr_models::EditIndexerParams;
  use crate::network::lidarr_network::lidarr_network_test_utils::test_utils::indexer;
  use pretty_assertions::assert_eq;
  use rstest::rstest;
  use strum::IntoEnumIterator;

  mod test_handle_scroll_up_and_down {
    use crate::app::App;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    use crate::models::BlockSelectionState;
    use crate::models::servarr_data::lidarr::lidarr_data::EDIT_INDEXER_TORRENT_SELECTION_BLOCKS;
    use crate::models::servarr_data::modals::EditIndexerModal;

    use super::*;

    #[rstest]
    fn test_edit_indexer_priority_scroll(#[values(Key::Up, Key::Down)] key: Key) {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveLidarrBlock::Indexers.into());
      app.data.lidarr_data.edit_indexer_modal = Some(EditIndexerModal::default());

      EditIndexerHandler::new(
        key,
        &mut app,
        ActiveLidarrBlock::EditIndexerPriorityInput,
        None,
      )
      .handle();

      if key == Key::Up {
        assert_eq!(
          app
            .data
            .lidarr_data
            .edit_indexer_modal
            .as_ref()
            .unwrap()
            .priority,
          2
        );
      } else {
        assert_eq!(
          app
            .data
            .lidarr_data
            .edit_indexer_modal
            .as_ref()
            .unwrap()
            .priority,
          1
        );

        EditIndexerHandler::new(
          Key::Up,
          &mut app,
          ActiveLidarrBlock::EditIndexerPriorityInput,
          None,
        )
        .handle();

        assert_eq!(
          app
            .data
            .lidarr_data
            .edit_indexer_modal
            .as_ref()
            .unwrap()
            .priority,
          2
        );

        EditIndexerHandler::new(
          key,
          &mut app,
          ActiveLidarrBlock::EditIndexerPriorityInput,
          None,
        )
        .handle();
        assert_eq!(
          app
            .data
            .lidarr_data
            .edit_indexer_modal
            .as_ref()
            .unwrap()
            .priority,
          1
        );
      }
    }

    #[rstest]
    fn test_edit_indexer_prompt_scroll(#[values(Key::Up, Key::Down)] key: Key) {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveLidarrBlock::Indexers.into());
      app.data.lidarr_data.edit_indexer_modal = Some(EditIndexerModal::default());
      app.data.lidarr_data.selected_block =
        BlockSelectionState::new(EDIT_INDEXER_TORRENT_SELECTION_BLOCKS);
      app.data.lidarr_data.selected_block.down();

      EditIndexerHandler::new(key, &mut app, ActiveLidarrBlock::EditIndexerPrompt, None).handle();

      if key == Key::Up {
        assert_eq!(
          app.data.lidarr_data.selected_block.get_active_block(),
          ActiveLidarrBlock::EditIndexerNameInput
        );
      } else {
        assert_eq!(
          app.data.lidarr_data.selected_block.get_active_block(),
          ActiveLidarrBlock::EditIndexerToggleEnableAutomaticSearch
        );
      }
    }

    #[rstest]
    fn test_edit_indexer_prompt_scroll_no_op_when_not_ready(
      #[values(Key::Up, Key::Down)] key: Key,
    ) {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveLidarrBlock::Indexers.into());
      app.is_loading = true;
      app.data.lidarr_data.edit_indexer_modal = Some(EditIndexerModal::default());
      app.data.lidarr_data.selected_block =
        BlockSelectionState::new(EDIT_INDEXER_TORRENT_SELECTION_BLOCKS);
      app.data.lidarr_data.selected_block.down();

      EditIndexerHandler::new(key, &mut app, ActiveLidarrBlock::EditIndexerPrompt, None).handle();

      assert_eq!(
        app.data.lidarr_data.selected_block.get_active_block(),
        ActiveLidarrBlock::EditIndexerToggleEnableRss
      );
    }
  }

  mod test_handle_home_end {
    use std::sync::atomic::Ordering;

    use crate::app::App;
    use crate::models::servarr_data::modals::EditIndexerModal;
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_edit_indexer_name_input_home_end() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveLidarrBlock::Indexers.into());
      app.data.lidarr_data.edit_indexer_modal = Some(EditIndexerModal {
        name: "Test".into(),
        ..EditIndexerModal::default()
      });

      EditIndexerHandler::new(
        DEFAULT_KEYBINDINGS.home.key,
        &mut app,
        ActiveLidarrBlock::EditIndexerNameInput,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .lidarr_data
          .edit_indexer_modal
          .as_ref()
          .unwrap()
          .name
          .offset
          .load(Ordering::SeqCst),
        4
      );

      EditIndexerHandler::new(
        DEFAULT_KEYBINDINGS.end.key,
        &mut app,
        ActiveLidarrBlock::EditIndexerNameInput,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .lidarr_data
          .edit_indexer_modal
          .as_ref()
          .unwrap()
          .name
          .offset
          .load(Ordering::SeqCst),
        0
      );
    }

    #[test]
    fn test_edit_indexer_url_input_home_end() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveLidarrBlock::Indexers.into());
      app.data.lidarr_data.edit_indexer_modal = Some(EditIndexerModal {
        url: "Test".into(),
        ..EditIndexerModal::default()
      });

      EditIndexerHandler::new(
        DEFAULT_KEYBINDINGS.home.key,
        &mut app,
        ActiveLidarrBlock::EditIndexerUrlInput,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .lidarr_data
          .edit_indexer_modal
          .as_ref()
          .unwrap()
          .url
          .offset
          .load(Ordering::SeqCst),
        4
      );

      EditIndexerHandler::new(
        DEFAULT_KEYBINDINGS.end.key,
        &mut app,
        ActiveLidarrBlock::EditIndexerUrlInput,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .lidarr_data
          .edit_indexer_modal
          .as_ref()
          .unwrap()
          .url
          .offset
          .load(Ordering::SeqCst),
        0
      );
    }

    #[test]
    fn test_edit_indexer_api_key_input_home_end() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveLidarrBlock::Indexers.into());
      app.data.lidarr_data.edit_indexer_modal = Some(EditIndexerModal {
        api_key: "Test".into(),
        ..EditIndexerModal::default()
      });

      EditIndexerHandler::new(
        DEFAULT_KEYBINDINGS.home.key,
        &mut app,
        ActiveLidarrBlock::EditIndexerApiKeyInput,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .lidarr_data
          .edit_indexer_modal
          .as_ref()
          .unwrap()
          .api_key
          .offset
          .load(Ordering::SeqCst),
        4
      );

      EditIndexerHandler::new(
        DEFAULT_KEYBINDINGS.end.key,
        &mut app,
        ActiveLidarrBlock::EditIndexerApiKeyInput,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .lidarr_data
          .edit_indexer_modal
          .as_ref()
          .unwrap()
          .api_key
          .offset
          .load(Ordering::SeqCst),
        0
      );
    }

    #[test]
    fn test_edit_indexer_seed_ratio_input_home_end() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveLidarrBlock::Indexers.into());
      app.data.lidarr_data.edit_indexer_modal = Some(EditIndexerModal {
        seed_ratio: "Test".into(),
        ..EditIndexerModal::default()
      });

      EditIndexerHandler::new(
        DEFAULT_KEYBINDINGS.home.key,
        &mut app,
        ActiveLidarrBlock::EditIndexerSeedRatioInput,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .lidarr_data
          .edit_indexer_modal
          .as_ref()
          .unwrap()
          .seed_ratio
          .offset
          .load(Ordering::SeqCst),
        4
      );

      EditIndexerHandler::new(
        DEFAULT_KEYBINDINGS.end.key,
        &mut app,
        ActiveLidarrBlock::EditIndexerSeedRatioInput,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .lidarr_data
          .edit_indexer_modal
          .as_ref()
          .unwrap()
          .seed_ratio
          .offset
          .load(Ordering::SeqCst),
        0
      );
    }

    #[test]
    fn test_edit_indexer_tags_input_home_end() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveLidarrBlock::Indexers.into());
      app.data.lidarr_data.edit_indexer_modal = Some(EditIndexerModal {
        tags: "Test".into(),
        ..EditIndexerModal::default()
      });

      EditIndexerHandler::new(
        DEFAULT_KEYBINDINGS.home.key,
        &mut app,
        ActiveLidarrBlock::EditIndexerTagsInput,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .lidarr_data
          .edit_indexer_modal
          .as_ref()
          .unwrap()
          .tags
          .offset
          .load(Ordering::SeqCst),
        4
      );

      EditIndexerHandler::new(
        DEFAULT_KEYBINDINGS.end.key,
        &mut app,
        ActiveLidarrBlock::EditIndexerTagsInput,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .lidarr_data
          .edit_indexer_modal
          .as_ref()
          .unwrap()
          .tags
          .offset
          .load(Ordering::SeqCst),
        0
      );
    }
  }

  mod test_handle_left_right_action {
    use std::sync::atomic::Ordering;

    use crate::app::App;
    use crate::models::BlockSelectionState;
    use crate::models::servarr_data::lidarr::lidarr_data::{
      EDIT_INDEXER_NZB_SELECTION_BLOCKS, EDIT_INDEXER_TORRENT_SELECTION_BLOCKS,
    };
    use crate::models::servarr_data::modals::EditIndexerModal;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    use super::*;

    #[rstest]
    fn test_left_right_prompt_toggle(#[values(Key::Left, Key::Right)] key: Key) {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveLidarrBlock::Indexers.into());
      app.data.lidarr_data.selected_block =
        BlockSelectionState::new(EDIT_INDEXER_TORRENT_SELECTION_BLOCKS);
      app.data.lidarr_data.selected_block.y = EDIT_INDEXER_TORRENT_SELECTION_BLOCKS.len() - 1;

      EditIndexerHandler::new(key, &mut app, ActiveLidarrBlock::EditIndexerPrompt, None).handle();

      assert!(app.data.lidarr_data.prompt_confirm);

      EditIndexerHandler::new(key, &mut app, ActiveLidarrBlock::EditIndexerPrompt, None).handle();

      assert!(!app.data.lidarr_data.prompt_confirm);
    }

    #[rstest]
    #[case(
      0,
      ActiveLidarrBlock::EditIndexerNameInput,
      ActiveLidarrBlock::EditIndexerUrlInput
    )]
    #[case(
      1,
      ActiveLidarrBlock::EditIndexerToggleEnableRss,
      ActiveLidarrBlock::EditIndexerApiKeyInput
    )]
    #[case(
      2,
      ActiveLidarrBlock::EditIndexerToggleEnableAutomaticSearch,
      ActiveLidarrBlock::EditIndexerSeedRatioInput
    )]
    #[case(
      3,
      ActiveLidarrBlock::EditIndexerToggleEnableInteractiveSearch,
      ActiveLidarrBlock::EditIndexerTagsInput
    )]
    fn test_left_right_block_toggle_torrents(
      #[values(Key::Left, Key::Right)] key: Key,
      #[case] starting_y_index: usize,
      #[case] left_block: ActiveLidarrBlock,
      #[case] right_block: ActiveLidarrBlock,
    ) {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveLidarrBlock::Indexers.into());
      app.data.lidarr_data.selected_block =
        BlockSelectionState::new(EDIT_INDEXER_TORRENT_SELECTION_BLOCKS);
      app.data.lidarr_data.selected_block.y = starting_y_index;

      assert_eq!(
        app.data.lidarr_data.selected_block.get_active_block(),
        left_block
      );

      EditIndexerHandler::new(key, &mut app, ActiveLidarrBlock::EditIndexerPrompt, None).handle();

      assert_eq!(
        app.data.lidarr_data.selected_block.get_active_block(),
        right_block
      );

      EditIndexerHandler::new(key, &mut app, ActiveLidarrBlock::EditIndexerPrompt, None).handle();

      assert_eq!(
        app.data.lidarr_data.selected_block.get_active_block(),
        left_block
      );
    }

    #[rstest]
    #[case(
      0,
      ActiveLidarrBlock::EditIndexerNameInput,
      ActiveLidarrBlock::EditIndexerUrlInput
    )]
    #[case(
      1,
      ActiveLidarrBlock::EditIndexerToggleEnableRss,
      ActiveLidarrBlock::EditIndexerApiKeyInput
    )]
    #[case(
      2,
      ActiveLidarrBlock::EditIndexerToggleEnableAutomaticSearch,
      ActiveLidarrBlock::EditIndexerTagsInput
    )]
    #[case(
      3,
      ActiveLidarrBlock::EditIndexerToggleEnableInteractiveSearch,
      ActiveLidarrBlock::EditIndexerPriorityInput
    )]
    fn test_left_right_block_toggle_nzb(
      #[values(Key::Left, Key::Right)] key: Key,
      #[case] starting_y_index: usize,
      #[case] left_block: ActiveLidarrBlock,
      #[case] right_block: ActiveLidarrBlock,
    ) {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveLidarrBlock::Indexers.into());
      app.data.lidarr_data.selected_block =
        BlockSelectionState::new(EDIT_INDEXER_NZB_SELECTION_BLOCKS);
      app.data.lidarr_data.selected_block.y = starting_y_index;

      assert_eq!(
        app.data.lidarr_data.selected_block.get_active_block(),
        left_block
      );

      EditIndexerHandler::new(key, &mut app, ActiveLidarrBlock::EditIndexerPrompt, None).handle();

      assert_eq!(
        app.data.lidarr_data.selected_block.get_active_block(),
        right_block
      );

      EditIndexerHandler::new(key, &mut app, ActiveLidarrBlock::EditIndexerPrompt, None).handle();

      assert_eq!(
        app.data.lidarr_data.selected_block.get_active_block(),
        left_block
      );
    }

    #[rstest]
    fn test_left_right_block_toggle_torren_empty_row_to_prompt_confirm(
      #[values(Key::Left, Key::Right)] key: Key,
    ) {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveLidarrBlock::Indexers.into());
      app.data.lidarr_data.selected_block =
        BlockSelectionState::new(EDIT_INDEXER_TORRENT_SELECTION_BLOCKS);
      app.data.lidarr_data.selected_block.y = 4;
      app.data.lidarr_data.prompt_confirm = false;

      assert_eq!(
        app.data.lidarr_data.selected_block.get_active_block(),
        ActiveLidarrBlock::EditIndexerPriorityInput
      );

      EditIndexerHandler::new(key, &mut app, ActiveLidarrBlock::EditIndexerPrompt, None).handle();

      assert_eq!(
        app.data.lidarr_data.selected_block.get_active_block(),
        ActiveLidarrBlock::EditIndexerConfirmPrompt
      );

      EditIndexerHandler::new(key, &mut app, ActiveLidarrBlock::EditIndexerPrompt, None).handle();

      assert_eq!(
        app.data.lidarr_data.selected_block.get_active_block(),
        ActiveLidarrBlock::EditIndexerConfirmPrompt
      );
      assert!(app.data.lidarr_data.prompt_confirm);
    }

    #[test]
    fn test_edit_indexer_name_input_left_right_keys() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveLidarrBlock::Indexers.into());
      app.data.lidarr_data.edit_indexer_modal = Some(EditIndexerModal {
        name: "Test".into(),
        ..EditIndexerModal::default()
      });

      EditIndexerHandler::new(
        DEFAULT_KEYBINDINGS.left.key,
        &mut app,
        ActiveLidarrBlock::EditIndexerNameInput,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .lidarr_data
          .edit_indexer_modal
          .as_ref()
          .unwrap()
          .name
          .offset
          .load(Ordering::SeqCst),
        1
      );

      EditIndexerHandler::new(
        DEFAULT_KEYBINDINGS.right.key,
        &mut app,
        ActiveLidarrBlock::EditIndexerNameInput,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .lidarr_data
          .edit_indexer_modal
          .as_ref()
          .unwrap()
          .name
          .offset
          .load(Ordering::SeqCst),
        0
      );
    }

    #[test]
    fn test_edit_indexer_url_input_left_right_keys() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveLidarrBlock::Indexers.into());
      app.data.lidarr_data.edit_indexer_modal = Some(EditIndexerModal {
        url: "Test".into(),
        ..EditIndexerModal::default()
      });

      EditIndexerHandler::new(
        DEFAULT_KEYBINDINGS.left.key,
        &mut app,
        ActiveLidarrBlock::EditIndexerUrlInput,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .lidarr_data
          .edit_indexer_modal
          .as_ref()
          .unwrap()
          .url
          .offset
          .load(Ordering::SeqCst),
        1
      );

      EditIndexerHandler::new(
        DEFAULT_KEYBINDINGS.right.key,
        &mut app,
        ActiveLidarrBlock::EditIndexerUrlInput,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .lidarr_data
          .edit_indexer_modal
          .as_ref()
          .unwrap()
          .url
          .offset
          .load(Ordering::SeqCst),
        0
      );
    }

    #[test]
    fn test_edit_indexer_api_key_input_left_right_keys() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveLidarrBlock::Indexers.into());
      app.data.lidarr_data.edit_indexer_modal = Some(EditIndexerModal {
        api_key: "Test".into(),
        ..EditIndexerModal::default()
      });

      EditIndexerHandler::new(
        DEFAULT_KEYBINDINGS.left.key,
        &mut app,
        ActiveLidarrBlock::EditIndexerApiKeyInput,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .lidarr_data
          .edit_indexer_modal
          .as_ref()
          .unwrap()
          .api_key
          .offset
          .load(Ordering::SeqCst),
        1
      );

      EditIndexerHandler::new(
        DEFAULT_KEYBINDINGS.right.key,
        &mut app,
        ActiveLidarrBlock::EditIndexerApiKeyInput,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .lidarr_data
          .edit_indexer_modal
          .as_ref()
          .unwrap()
          .api_key
          .offset
          .load(Ordering::SeqCst),
        0
      );
    }

    #[test]
    fn test_edit_indexer_seed_ratio_input_left_right_keys() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveLidarrBlock::Indexers.into());
      app.data.lidarr_data.edit_indexer_modal = Some(EditIndexerModal {
        seed_ratio: "Test".into(),
        ..EditIndexerModal::default()
      });

      EditIndexerHandler::new(
        DEFAULT_KEYBINDINGS.left.key,
        &mut app,
        ActiveLidarrBlock::EditIndexerSeedRatioInput,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .lidarr_data
          .edit_indexer_modal
          .as_ref()
          .unwrap()
          .seed_ratio
          .offset
          .load(Ordering::SeqCst),
        1
      );

      EditIndexerHandler::new(
        DEFAULT_KEYBINDINGS.right.key,
        &mut app,
        ActiveLidarrBlock::EditIndexerSeedRatioInput,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .lidarr_data
          .edit_indexer_modal
          .as_ref()
          .unwrap()
          .seed_ratio
          .offset
          .load(Ordering::SeqCst),
        0
      );
    }

    #[test]
    fn test_edit_indexer_tags_input_left_right_keys() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveLidarrBlock::Indexers.into());
      app.data.lidarr_data.edit_indexer_modal = Some(EditIndexerModal {
        tags: "Test".into(),
        ..EditIndexerModal::default()
      });

      EditIndexerHandler::new(
        DEFAULT_KEYBINDINGS.left.key,
        &mut app,
        ActiveLidarrBlock::EditIndexerTagsInput,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .lidarr_data
          .edit_indexer_modal
          .as_ref()
          .unwrap()
          .tags
          .offset
          .load(Ordering::SeqCst),
        1
      );

      EditIndexerHandler::new(
        DEFAULT_KEYBINDINGS.right.key,
        &mut app,
        ActiveLidarrBlock::EditIndexerTagsInput,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .lidarr_data
          .edit_indexer_modal
          .as_ref()
          .unwrap()
          .tags
          .offset
          .load(Ordering::SeqCst),
        0
      );
    }
  }

  mod test_handle_submit {
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    use crate::app::App;
    use crate::assert_navigation_popped;
    use crate::models::servarr_data::modals::EditIndexerModal;
    use crate::models::{
      BlockSelectionState, servarr_data::lidarr::lidarr_data::EDIT_INDEXER_TORRENT_SELECTION_BLOCKS,
    };
    use crate::network::lidarr_network::LidarrEvent;
    use crate::network::lidarr_network::lidarr_network_test_utils::test_utils::indexer;

    use super::*;

    const SUBMIT_KEY: Key = DEFAULT_KEYBINDINGS.submit.key;

    #[test]
    fn test_edit_indexer_prompt_prompt_decline_submit() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveLidarrBlock::Indexers.into());
      app.push_navigation_stack(ActiveLidarrBlock::EditIndexerPrompt.into());
      app.data.lidarr_data.selected_block =
        BlockSelectionState::new(EDIT_INDEXER_TORRENT_SELECTION_BLOCKS);
      app
        .data
        .lidarr_data
        .selected_block
        .set_index(0, EDIT_INDEXER_TORRENT_SELECTION_BLOCKS.len() - 1);
      app.data.lidarr_data.edit_indexer_modal = Some(EditIndexerModal::default());

      EditIndexerHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveLidarrBlock::EditIndexerPrompt,
        None,
      )
      .handle();

      assert_navigation_popped!(app, ActiveLidarrBlock::Indexers.into());
      assert_none!(app.data.lidarr_data.prompt_confirm_action);
      assert!(!app.should_refresh);
      assert_none!(app.data.lidarr_data.edit_indexer_modal);
    }

    #[test]
    fn test_edit_indexer_prompt_prompt_confirmation_submit() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveLidarrBlock::Indexers.into());
      app.push_navigation_stack(ActiveLidarrBlock::EditIndexerPrompt.into());
      app.data.lidarr_data.selected_block =
        BlockSelectionState::new(EDIT_INDEXER_TORRENT_SELECTION_BLOCKS);
      app
        .data
        .lidarr_data
        .selected_block
        .set_index(0, EDIT_INDEXER_TORRENT_SELECTION_BLOCKS.len() - 1);
      let edit_indexer_modal = EditIndexerModal {
        name: "Test Update".into(),
        enable_rss: Some(false),
        enable_automatic_search: Some(false),
        enable_interactive_search: Some(false),
        url: "https://localhost:9696/1/".into(),
        api_key: "test1234".into(),
        seed_ratio: "1.3".into(),
        tags: "usenet, testing".into(),
        priority: 0,
      };
      app.data.lidarr_data.edit_indexer_modal = Some(edit_indexer_modal);
      app.data.lidarr_data.indexers.set_items(vec![indexer()]);
      let expected_edit_indexer_params = EditIndexerParams {
        indexer_id: 1,
        name: Some("Test Update".to_owned()),
        enable_rss: Some(false),
        enable_automatic_search: Some(false),
        enable_interactive_search: Some(false),
        url: Some("https://localhost:9696/1/".to_owned()),
        api_key: Some("test1234".to_owned()),
        seed_ratio: Some("1.3".to_owned()),
        tag_input_string: Some("usenet, testing".to_owned()),
        priority: Some(0),
        ..EditIndexerParams::default()
      };
      app.data.lidarr_data.prompt_confirm = true;

      EditIndexerHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveLidarrBlock::EditIndexerPrompt,
        None,
      )
      .handle();

      assert_navigation_popped!(app, ActiveLidarrBlock::Indexers.into());
      assert_modal_absent!(app.data.lidarr_data.edit_indexer_modal);
      assert!(app.should_refresh);
      assert_eq!(
        app.data.lidarr_data.prompt_confirm_action,
        Some(LidarrEvent::EditIndexer(expected_edit_indexer_params))
      );
    }

    #[test]
    fn test_edit_indexer_prompt_prompt_confirmation_submit_no_op_when_not_ready() {
      let mut app = App::test_default();
      app.is_loading = true;
      app.push_navigation_stack(ActiveLidarrBlock::Indexers.into());
      app.push_navigation_stack(ActiveLidarrBlock::EditIndexerPrompt.into());
      app.data.lidarr_data.edit_indexer_modal = Some(EditIndexerModal::default());
      app.data.lidarr_data.prompt_confirm = true;

      EditIndexerHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveLidarrBlock::EditIndexerPrompt,
        None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveLidarrBlock::EditIndexerPrompt.into()
      );
      assert_modal_present!(app.data.lidarr_data.edit_indexer_modal);
      assert!(!app.should_refresh);
      assert_none!(app.data.lidarr_data.prompt_confirm_action);
    }

    #[rstest]
    #[case(0, 0, ActiveLidarrBlock::EditIndexerNameInput)]
    #[case(0, 1, ActiveLidarrBlock::EditIndexerUrlInput)]
    #[case(1, 1, ActiveLidarrBlock::EditIndexerApiKeyInput)]
    #[case(2, 1, ActiveLidarrBlock::EditIndexerSeedRatioInput)]
    #[case(3, 1, ActiveLidarrBlock::EditIndexerTagsInput)]
    fn test_edit_indexer_prompt_submit_input_fields(
      #[case] starting_y: usize,
      #[case] starting_x: usize,
      #[case] block: ActiveLidarrBlock,
    ) {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveLidarrBlock::Indexers.into());
      app.data.lidarr_data.edit_indexer_modal = Some(EditIndexerModal::default());
      app.push_navigation_stack(ActiveLidarrBlock::EditIndexerPrompt.into());
      app.data.lidarr_data.selected_block =
        BlockSelectionState::new(EDIT_INDEXER_TORRENT_SELECTION_BLOCKS);
      app
        .data
        .lidarr_data
        .selected_block
        .set_index(starting_x, starting_y);

      EditIndexerHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveLidarrBlock::EditIndexerPrompt,
        None,
      )
      .handle();

      assert_navigation_pushed!(app, block.into());
      assert!(app.ignore_special_keys_for_textbox_input);
    }

    #[test]
    fn test_edit_indexer_priority_input_submit() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveLidarrBlock::Indexers.into());
      app.data.lidarr_data.edit_indexer_modal = Some(EditIndexerModal::default());
      app.push_navigation_stack(ActiveLidarrBlock::EditIndexerPrompt.into());
      app.data.lidarr_data.selected_block =
        BlockSelectionState::new(EDIT_INDEXER_TORRENT_SELECTION_BLOCKS);
      app.data.lidarr_data.selected_block.set_index(0, 4);

      EditIndexerHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveLidarrBlock::EditIndexerPrompt,
        None,
      )
      .handle();

      assert_navigation_pushed!(app, ActiveLidarrBlock::EditIndexerPriorityInput.into());
      assert!(!app.ignore_special_keys_for_textbox_input);
    }

    #[test]
    fn test_edit_indexer_toggle_enable_rss_submit() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveLidarrBlock::Indexers.into());
      app.data.lidarr_data.edit_indexer_modal = Some(EditIndexerModal::default());
      app.data.lidarr_data.selected_block =
        BlockSelectionState::new(EDIT_INDEXER_TORRENT_SELECTION_BLOCKS);
      app.data.lidarr_data.selected_block.set_index(0, 1);
      app.push_navigation_stack(ActiveLidarrBlock::EditIndexerPrompt.into());

      EditIndexerHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveLidarrBlock::EditIndexerPrompt,
        None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveLidarrBlock::EditIndexerPrompt.into()
      );
      assert!(
        app
          .data
          .lidarr_data
          .edit_indexer_modal
          .as_ref()
          .unwrap()
          .enable_rss
          .unwrap()
      );

      EditIndexerHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveLidarrBlock::EditIndexerPrompt,
        None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveLidarrBlock::EditIndexerPrompt.into()
      );
      assert!(
        !app
          .data
          .lidarr_data
          .edit_indexer_modal
          .as_ref()
          .unwrap()
          .enable_rss
          .unwrap()
      );
    }

    #[test]
    fn test_edit_indexer_toggle_enable_automatic_search_submit() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveLidarrBlock::Indexers.into());
      app.data.lidarr_data.edit_indexer_modal = Some(EditIndexerModal::default());
      app.data.lidarr_data.selected_block =
        BlockSelectionState::new(EDIT_INDEXER_TORRENT_SELECTION_BLOCKS);
      app.data.lidarr_data.selected_block.set_index(0, 2);
      app.push_navigation_stack(ActiveLidarrBlock::EditIndexerPrompt.into());

      EditIndexerHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveLidarrBlock::EditIndexerPrompt,
        None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveLidarrBlock::EditIndexerPrompt.into()
      );
      assert!(
        app
          .data
          .lidarr_data
          .edit_indexer_modal
          .as_ref()
          .unwrap()
          .enable_automatic_search
          .unwrap()
      );

      EditIndexerHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveLidarrBlock::EditIndexerPrompt,
        None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveLidarrBlock::EditIndexerPrompt.into()
      );
      assert!(
        !app
          .data
          .lidarr_data
          .edit_indexer_modal
          .as_ref()
          .unwrap()
          .enable_automatic_search
          .unwrap()
      );
    }

    #[test]
    fn test_edit_indexer_toggle_enable_interactive_search_submit() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveLidarrBlock::Indexers.into());
      app.data.lidarr_data.edit_indexer_modal = Some(EditIndexerModal::default());
      app.data.lidarr_data.selected_block =
        BlockSelectionState::new(EDIT_INDEXER_TORRENT_SELECTION_BLOCKS);
      app.data.lidarr_data.selected_block.set_index(0, 3);
      app.push_navigation_stack(ActiveLidarrBlock::EditIndexerPrompt.into());

      EditIndexerHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveLidarrBlock::EditIndexerPrompt,
        None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveLidarrBlock::EditIndexerPrompt.into()
      );
      assert!(
        app
          .data
          .lidarr_data
          .edit_indexer_modal
          .as_ref()
          .unwrap()
          .enable_interactive_search
          .unwrap()
      );

      EditIndexerHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveLidarrBlock::EditIndexerPrompt,
        None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveLidarrBlock::EditIndexerPrompt.into()
      );
      assert!(
        !app
          .data
          .lidarr_data
          .edit_indexer_modal
          .as_ref()
          .unwrap()
          .enable_interactive_search
          .unwrap()
      );
    }

    #[test]
    fn test_edit_indexer_name_input_submit() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveLidarrBlock::Indexers.into());
      app.ignore_special_keys_for_textbox_input = true;
      app.data.lidarr_data.edit_indexer_modal = Some(EditIndexerModal {
        name: "Test".into(),
        ..EditIndexerModal::default()
      });
      app.push_navigation_stack(ActiveLidarrBlock::EditIndexerPrompt.into());
      app.push_navigation_stack(ActiveLidarrBlock::EditIndexerNameInput.into());

      EditIndexerHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveLidarrBlock::EditIndexerNameInput,
        None,
      )
      .handle();

      assert!(!app.ignore_special_keys_for_textbox_input);
      assert!(
        !app
          .data
          .lidarr_data
          .edit_indexer_modal
          .as_ref()
          .unwrap()
          .name
          .text
          .is_empty()
      );
      assert_navigation_popped!(app, ActiveLidarrBlock::EditIndexerPrompt.into());
    }

    #[test]
    fn test_edit_indexer_url_input_submit() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveLidarrBlock::Indexers.into());
      app.ignore_special_keys_for_textbox_input = true;
      app.data.lidarr_data.edit_indexer_modal = Some(EditIndexerModal {
        url: "Test".into(),
        ..EditIndexerModal::default()
      });
      app.push_navigation_stack(ActiveLidarrBlock::EditIndexerPrompt.into());
      app.push_navigation_stack(ActiveLidarrBlock::EditIndexerUrlInput.into());

      EditIndexerHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveLidarrBlock::EditIndexerUrlInput,
        None,
      )
      .handle();

      assert!(!app.ignore_special_keys_for_textbox_input);
      assert!(
        !app
          .data
          .lidarr_data
          .edit_indexer_modal
          .as_ref()
          .unwrap()
          .url
          .text
          .is_empty()
      );
      assert_navigation_popped!(app, ActiveLidarrBlock::EditIndexerPrompt.into());
    }

    #[test]
    fn test_edit_indexer_api_key_input_submit() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveLidarrBlock::Indexers.into());
      app.ignore_special_keys_for_textbox_input = true;
      app.data.lidarr_data.edit_indexer_modal = Some(EditIndexerModal {
        api_key: "Test".into(),
        ..EditIndexerModal::default()
      });
      app.push_navigation_stack(ActiveLidarrBlock::EditIndexerPrompt.into());
      app.push_navigation_stack(ActiveLidarrBlock::EditIndexerApiKeyInput.into());

      EditIndexerHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveLidarrBlock::EditIndexerApiKeyInput,
        None,
      )
      .handle();

      assert!(!app.ignore_special_keys_for_textbox_input);
      assert!(
        !app
          .data
          .lidarr_data
          .edit_indexer_modal
          .as_ref()
          .unwrap()
          .api_key
          .text
          .is_empty()
      );
      assert_navigation_popped!(app, ActiveLidarrBlock::EditIndexerPrompt.into());
    }

    #[test]
    fn test_edit_indexer_seed_ratio_input_submit() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveLidarrBlock::Indexers.into());
      app.ignore_special_keys_for_textbox_input = true;
      app.data.lidarr_data.edit_indexer_modal = Some(EditIndexerModal {
        seed_ratio: "Test".into(),
        ..EditIndexerModal::default()
      });
      app.push_navigation_stack(ActiveLidarrBlock::EditIndexerPrompt.into());
      app.push_navigation_stack(ActiveLidarrBlock::EditIndexerSeedRatioInput.into());

      EditIndexerHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveLidarrBlock::EditIndexerSeedRatioInput,
        None,
      )
      .handle();

      assert!(!app.ignore_special_keys_for_textbox_input);
      assert!(
        !app
          .data
          .lidarr_data
          .edit_indexer_modal
          .as_ref()
          .unwrap()
          .seed_ratio
          .text
          .is_empty()
      );
      assert_navigation_popped!(app, ActiveLidarrBlock::EditIndexerPrompt.into());
    }

    #[test]
    fn test_edit_indexer_tags_input_submit() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveLidarrBlock::Indexers.into());
      app.ignore_special_keys_for_textbox_input = true;
      app.data.lidarr_data.edit_indexer_modal = Some(EditIndexerModal {
        tags: "Test".into(),
        ..EditIndexerModal::default()
      });
      app.push_navigation_stack(ActiveLidarrBlock::EditIndexerPrompt.into());
      app.push_navigation_stack(ActiveLidarrBlock::EditIndexerTagsInput.into());

      EditIndexerHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveLidarrBlock::EditIndexerTagsInput,
        None,
      )
      .handle();

      assert!(!app.ignore_special_keys_for_textbox_input);
      assert!(
        !app
          .data
          .lidarr_data
          .edit_indexer_modal
          .as_ref()
          .unwrap()
          .tags
          .text
          .is_empty()
      );
      assert_navigation_popped!(app, ActiveLidarrBlock::EditIndexerPrompt.into());
    }
  }

  mod test_handle_esc {
    use super::*;
    use crate::app::App;
    use crate::assert_navigation_popped;
    use crate::event::Key;
    use crate::models::servarr_data::modals::EditIndexerModal;
    use rstest::rstest;

    const ESC_KEY: Key = DEFAULT_KEYBINDINGS.esc.key;

    #[rstest]
    fn test_edit_indexer_prompt_esc(#[values(true, false)] is_ready: bool) {
      let mut app = App::test_default();
      app.is_loading = is_ready;
      app.push_navigation_stack(ActiveLidarrBlock::Indexers.into());
      app.push_navigation_stack(ActiveLidarrBlock::EditIndexerPrompt.into());
      app.data.lidarr_data.edit_indexer_modal = Some(EditIndexerModal::default());

      EditIndexerHandler::new(
        ESC_KEY,
        &mut app,
        ActiveLidarrBlock::EditIndexerPrompt,
        None,
      )
      .handle();

      assert_navigation_popped!(app, ActiveLidarrBlock::Indexers.into());
      assert!(!app.data.lidarr_data.prompt_confirm);
      assert_none!(app.data.lidarr_data.edit_indexer_modal);
    }

    #[rstest]
    fn test_edit_indexer_input_fields_esc(
      #[values(
        ActiveLidarrBlock::EditIndexerNameInput,
        ActiveLidarrBlock::EditIndexerUrlInput,
        ActiveLidarrBlock::EditIndexerApiKeyInput,
        ActiveLidarrBlock::EditIndexerSeedRatioInput,
        ActiveLidarrBlock::EditIndexerTagsInput,
        ActiveLidarrBlock::EditIndexerPriorityInput
      )]
      active_lidarr_block: ActiveLidarrBlock,
    ) {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveLidarrBlock::Indexers.into());
      app.push_navigation_stack(active_lidarr_block.into());
      app.data.lidarr_data.edit_indexer_modal = Some(EditIndexerModal::default());
      app.ignore_special_keys_for_textbox_input = true;

      EditIndexerHandler::new(ESC_KEY, &mut app, active_lidarr_block, None).handle();

      assert_navigation_popped!(app, ActiveLidarrBlock::Indexers.into());
      assert!(!app.ignore_special_keys_for_textbox_input);
      assert_some_eq_x!(
        &app.data.lidarr_data.edit_indexer_modal,
        &EditIndexerModal::default()
      );
    }
  }

  mod test_handle_key_char {
    use super::*;
    use crate::app::App;
    use crate::assert_navigation_popped;
    use crate::models::BlockSelectionState;
    use crate::models::servarr_data::lidarr::lidarr_data::EDIT_INDEXER_TORRENT_SELECTION_BLOCKS;
    use crate::models::servarr_data::modals::EditIndexerModal;
    use crate::network::lidarr_network::LidarrEvent;
    use crate::network::lidarr_network::lidarr_network_test_utils::test_utils::indexer;
    use pretty_assertions::{assert_eq, assert_str_eq};

    #[test]
    fn test_edit_indexer_name_input_backspace() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveLidarrBlock::Indexers.into());
      app.data.lidarr_data.edit_indexer_modal = Some(EditIndexerModal {
        name: "Test".into(),
        ..EditIndexerModal::default()
      });

      EditIndexerHandler::new(
        DEFAULT_KEYBINDINGS.backspace.key,
        &mut app,
        ActiveLidarrBlock::EditIndexerNameInput,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .lidarr_data
          .edit_indexer_modal
          .as_ref()
          .unwrap()
          .name
          .text,
        "Tes"
      );
    }

    #[test]
    fn test_edit_indexer_url_input_backspace() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveLidarrBlock::Indexers.into());
      app.data.lidarr_data.edit_indexer_modal = Some(EditIndexerModal {
        url: "Test".into(),
        ..EditIndexerModal::default()
      });

      EditIndexerHandler::new(
        DEFAULT_KEYBINDINGS.backspace.key,
        &mut app,
        ActiveLidarrBlock::EditIndexerUrlInput,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .lidarr_data
          .edit_indexer_modal
          .as_ref()
          .unwrap()
          .url
          .text,
        "Tes"
      );
    }

    #[test]
    fn test_edit_indexer_api_key_input_backspace() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveLidarrBlock::Indexers.into());
      app.data.lidarr_data.edit_indexer_modal = Some(EditIndexerModal {
        api_key: "Test".into(),
        ..EditIndexerModal::default()
      });

      EditIndexerHandler::new(
        DEFAULT_KEYBINDINGS.backspace.key,
        &mut app,
        ActiveLidarrBlock::EditIndexerApiKeyInput,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .lidarr_data
          .edit_indexer_modal
          .as_ref()
          .unwrap()
          .api_key
          .text,
        "Tes"
      );
    }

    #[test]
    fn test_edit_indexer_seed_ratio_input_backspace() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveLidarrBlock::Indexers.into());
      app.data.lidarr_data.edit_indexer_modal = Some(EditIndexerModal {
        seed_ratio: "Test".into(),
        ..EditIndexerModal::default()
      });

      EditIndexerHandler::new(
        DEFAULT_KEYBINDINGS.backspace.key,
        &mut app,
        ActiveLidarrBlock::EditIndexerSeedRatioInput,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .lidarr_data
          .edit_indexer_modal
          .as_ref()
          .unwrap()
          .seed_ratio
          .text,
        "Tes"
      );
    }

    #[test]
    fn test_edit_indexer_tags_input_backspace() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveLidarrBlock::Indexers.into());
      app.data.lidarr_data.edit_indexer_modal = Some(EditIndexerModal {
        tags: "Test".into(),
        ..EditIndexerModal::default()
      });

      EditIndexerHandler::new(
        DEFAULT_KEYBINDINGS.backspace.key,
        &mut app,
        ActiveLidarrBlock::EditIndexerTagsInput,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .lidarr_data
          .edit_indexer_modal
          .as_ref()
          .unwrap()
          .tags
          .text,
        "Tes"
      );
    }

    #[test]
    fn test_edit_indexer_name_input_char_key() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveLidarrBlock::Indexers.into());
      app.data.lidarr_data.edit_indexer_modal = Some(EditIndexerModal::default());

      EditIndexerHandler::new(
        Key::Char('a'),
        &mut app,
        ActiveLidarrBlock::EditIndexerNameInput,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .lidarr_data
          .edit_indexer_modal
          .as_ref()
          .unwrap()
          .name
          .text,
        "a"
      );
    }

    #[test]
    fn test_edit_indexer_url_input_char_key() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveLidarrBlock::Indexers.into());
      app.data.lidarr_data.edit_indexer_modal = Some(EditIndexerModal::default());

      EditIndexerHandler::new(
        Key::Char('a'),
        &mut app,
        ActiveLidarrBlock::EditIndexerUrlInput,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .lidarr_data
          .edit_indexer_modal
          .as_ref()
          .unwrap()
          .url
          .text,
        "a"
      );
    }

    #[test]
    fn test_edit_indexer_api_key_input_char_key() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveLidarrBlock::Indexers.into());
      app.data.lidarr_data.edit_indexer_modal = Some(EditIndexerModal::default());

      EditIndexerHandler::new(
        Key::Char('a'),
        &mut app,
        ActiveLidarrBlock::EditIndexerApiKeyInput,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .lidarr_data
          .edit_indexer_modal
          .as_ref()
          .unwrap()
          .api_key
          .text,
        "a"
      );
    }

    #[test]
    fn test_edit_indexer_seed_ratio_input_char_key() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveLidarrBlock::Indexers.into());
      app.data.lidarr_data.edit_indexer_modal = Some(EditIndexerModal::default());

      EditIndexerHandler::new(
        Key::Char('a'),
        &mut app,
        ActiveLidarrBlock::EditIndexerSeedRatioInput,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .lidarr_data
          .edit_indexer_modal
          .as_ref()
          .unwrap()
          .seed_ratio
          .text,
        "a"
      );
    }

    #[test]
    fn test_edit_indexer_tags_input_char_key() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveLidarrBlock::Indexers.into());
      app.data.lidarr_data.edit_indexer_modal = Some(EditIndexerModal::default());

      EditIndexerHandler::new(
        Key::Char('a'),
        &mut app,
        ActiveLidarrBlock::EditIndexerTagsInput,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .lidarr_data
          .edit_indexer_modal
          .as_ref()
          .unwrap()
          .tags
          .text,
        "a"
      );
    }

    #[test]
    fn test_edit_indexer_prompt_prompt_confirmation_confirm() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveLidarrBlock::Indexers.into());
      app.push_navigation_stack(ActiveLidarrBlock::EditIndexerPrompt.into());
      app.data.lidarr_data.selected_block =
        BlockSelectionState::new(EDIT_INDEXER_TORRENT_SELECTION_BLOCKS);
      app
        .data
        .lidarr_data
        .selected_block
        .set_index(0, EDIT_INDEXER_TORRENT_SELECTION_BLOCKS.len() - 1);
      let edit_indexer_modal = EditIndexerModal {
        name: "Test Update".into(),
        enable_rss: Some(false),
        enable_automatic_search: Some(false),
        enable_interactive_search: Some(false),
        url: "https://localhost:9696/1/".into(),
        api_key: "test1234".into(),
        seed_ratio: "1.3".into(),
        tags: "usenet, testing".into(),
        priority: 0,
      };
      app.data.lidarr_data.edit_indexer_modal = Some(edit_indexer_modal);
      app.data.lidarr_data.indexers.set_items(vec![indexer()]);
      let expected_edit_indexer_params = EditIndexerParams {
        indexer_id: 1,
        name: Some("Test Update".to_owned()),
        enable_rss: Some(false),
        enable_automatic_search: Some(false),
        enable_interactive_search: Some(false),
        url: Some("https://localhost:9696/1/".to_owned()),
        api_key: Some("test1234".to_owned()),
        seed_ratio: Some("1.3".to_owned()),
        tag_input_string: Some("usenet, testing".to_owned()),
        priority: Some(0),
        ..EditIndexerParams::default()
      };

      EditIndexerHandler::new(
        DEFAULT_KEYBINDINGS.confirm.key,
        &mut app,
        ActiveLidarrBlock::EditIndexerPrompt,
        None,
      )
      .handle();

      assert_navigation_popped!(app, ActiveLidarrBlock::Indexers.into());
      assert_modal_absent!(app.data.lidarr_data.edit_indexer_modal);
      assert!(app.should_refresh);
      assert_eq!(
        app.data.lidarr_data.prompt_confirm_action,
        Some(LidarrEvent::EditIndexer(expected_edit_indexer_params))
      );
    }
  }

  #[test]
  fn test_edit_indexer_handler_accepts() {
    ActiveLidarrBlock::iter().for_each(|active_lidarr_block| {
      if EDIT_INDEXER_BLOCKS.contains(&active_lidarr_block) {
        assert!(EditIndexerHandler::accepts(active_lidarr_block));
      } else {
        assert!(!EditIndexerHandler::accepts(active_lidarr_block));
      }
    })
  }

  #[rstest]
  fn test_edit_indexer_handler_ignore_special_keys(
    #[values(true, false)] ignore_special_keys_for_textbox_input: bool,
  ) {
    let mut app = App::test_default();
    app.ignore_special_keys_for_textbox_input = ignore_special_keys_for_textbox_input;
    let handler = EditIndexerHandler::new(
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
  fn test_build_edit_indexer_params() {
    let mut app = App::test_default();
    let edit_indexer_modal = EditIndexerModal {
      name: "Test Update".into(),
      enable_rss: Some(false),
      enable_automatic_search: Some(false),
      enable_interactive_search: Some(false),
      url: "https://localhost:9696/1/".into(),
      api_key: "test1234".into(),
      seed_ratio: "1.3".into(),
      tags: "usenet, testing".into(),
      priority: 0,
    };
    app.data.lidarr_data.edit_indexer_modal = Some(edit_indexer_modal);
    app.data.lidarr_data.indexers.set_items(vec![indexer()]);
    let expected_edit_indexer_params = EditIndexerParams {
      indexer_id: 1,
      name: Some("Test Update".to_owned()),
      enable_rss: Some(false),
      enable_automatic_search: Some(false),
      enable_interactive_search: Some(false),
      url: Some("https://localhost:9696/1/".to_owned()),
      api_key: Some("test1234".to_owned()),
      seed_ratio: Some("1.3".to_owned()),
      tag_input_string: Some("usenet, testing".to_owned()),
      priority: Some(0),
      ..EditIndexerParams::default()
    };

    let params = EditIndexerHandler::new(
      DEFAULT_KEYBINDINGS.confirm.key,
      &mut app,
      ActiveLidarrBlock::EditIndexerPrompt,
      None,
    )
    .build_edit_indexer_params();

    assert_eq!(params, expected_edit_indexer_params);
    assert_modal_absent!(app.data.lidarr_data.edit_indexer_modal);
  }

  #[test]
  fn test_edit_indexer_handler_is_not_ready_when_loading() {
    let mut app = App::test_default();
    app.push_navigation_stack(ActiveLidarrBlock::Indexers.into());
    app.is_loading = true;

    let handler = EditIndexerHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveLidarrBlock::EditIndexerPrompt,
      None,
    );

    assert!(!handler.is_ready());
  }

  #[test]
  fn test_edit_indexer_handler_is_not_ready_when_edit_indexer_modal_is_none() {
    let mut app = App::test_default();
    app.push_navigation_stack(ActiveLidarrBlock::Indexers.into());
    app.is_loading = false;

    let handler = EditIndexerHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveLidarrBlock::EditIndexerPrompt,
      None,
    );

    assert!(!handler.is_ready());
  }

  #[test]
  fn test_edit_indexer_handler_is_ready_when_edit_indexer_modal_is_some() {
    let mut app = App::test_default();
    app.push_navigation_stack(ActiveLidarrBlock::Indexers.into());
    app.is_loading = false;
    app.data.lidarr_data.edit_indexer_modal = Some(EditIndexerModal::default());

    let handler = EditIndexerHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveLidarrBlock::EditIndexerPrompt,
      None,
    );

    assert!(handler.is_ready());
  }
}
