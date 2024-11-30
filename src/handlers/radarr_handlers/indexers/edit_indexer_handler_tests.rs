#[cfg(test)]
mod tests {
  use crate::app::key_binding::DEFAULT_KEYBINDINGS;
  use crate::app::App;
  use crate::event::Key;
  use crate::handlers::radarr_handlers::indexers::edit_indexer_handler::EditIndexerHandler;
  use crate::handlers::KeyEventHandler;
  use crate::models::servarr_data::modals::EditIndexerModal;
  use crate::models::servarr_data::radarr::radarr_data::{ActiveRadarrBlock, EDIT_INDEXER_BLOCKS};
  use strum::IntoEnumIterator;

  mod test_handle_scroll_up_and_down {
    use crate::app::App;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    use crate::models::servarr_data::modals::EditIndexerModal;
    use crate::models::servarr_data::radarr::radarr_data::EDIT_INDEXER_TORRENT_SELECTION_BLOCKS;
    use crate::models::BlockSelectionState;

    use super::*;

    #[rstest]
    fn test_edit_indexer_prompt_scroll(#[values(Key::Up, Key::Down)] key: Key) {
      let mut app = App::default();
      app.data.radarr_data.edit_indexer_modal = Some(EditIndexerModal::default());
      app.data.radarr_data.selected_block =
        BlockSelectionState::new(EDIT_INDEXER_TORRENT_SELECTION_BLOCKS);
      app.data.radarr_data.selected_block.down();

      EditIndexerHandler::with(key, &mut app, ActiveRadarrBlock::EditIndexerPrompt, None).handle();

      if key == Key::Up {
        assert_eq!(
          app.data.radarr_data.selected_block.get_active_block(),
          ActiveRadarrBlock::EditIndexerNameInput
        );
      } else {
        assert_eq!(
          app.data.radarr_data.selected_block.get_active_block(),
          ActiveRadarrBlock::EditIndexerToggleEnableAutomaticSearch
        );
      }
    }

    #[rstest]
    fn test_edit_indexer_prompt_scroll_no_op_when_not_ready(
      #[values(Key::Up, Key::Down)] key: Key,
    ) {
      let mut app = App::default();
      app.is_loading = true;
      app.data.radarr_data.edit_indexer_modal = Some(EditIndexerModal::default());
      app.data.radarr_data.selected_block =
        BlockSelectionState::new(EDIT_INDEXER_TORRENT_SELECTION_BLOCKS);
      app.data.radarr_data.selected_block.down();

      EditIndexerHandler::with(key, &mut app, ActiveRadarrBlock::EditIndexerPrompt, None).handle();

      assert_eq!(
        app.data.radarr_data.selected_block.get_active_block(),
        ActiveRadarrBlock::EditIndexerToggleEnableRss
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
      let mut app = App::default();
      app.data.radarr_data.edit_indexer_modal = Some(EditIndexerModal {
        name: "Test".into(),
        ..EditIndexerModal::default()
      });

      EditIndexerHandler::with(
        DEFAULT_KEYBINDINGS.home.key,
        &mut app,
        ActiveRadarrBlock::EditIndexerNameInput,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .radarr_data
          .edit_indexer_modal
          .as_ref()
          .unwrap()
          .name
          .offset
          .load(Ordering::SeqCst),
        4
      );

      EditIndexerHandler::with(
        DEFAULT_KEYBINDINGS.end.key,
        &mut app,
        ActiveRadarrBlock::EditIndexerNameInput,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .radarr_data
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
      let mut app = App::default();
      app.data.radarr_data.edit_indexer_modal = Some(EditIndexerModal {
        url: "Test".into(),
        ..EditIndexerModal::default()
      });

      EditIndexerHandler::with(
        DEFAULT_KEYBINDINGS.home.key,
        &mut app,
        ActiveRadarrBlock::EditIndexerUrlInput,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .radarr_data
          .edit_indexer_modal
          .as_ref()
          .unwrap()
          .url
          .offset
          .load(Ordering::SeqCst),
        4
      );

      EditIndexerHandler::with(
        DEFAULT_KEYBINDINGS.end.key,
        &mut app,
        ActiveRadarrBlock::EditIndexerUrlInput,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .radarr_data
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
      let mut app = App::default();
      app.data.radarr_data.edit_indexer_modal = Some(EditIndexerModal {
        api_key: "Test".into(),
        ..EditIndexerModal::default()
      });

      EditIndexerHandler::with(
        DEFAULT_KEYBINDINGS.home.key,
        &mut app,
        ActiveRadarrBlock::EditIndexerApiKeyInput,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .radarr_data
          .edit_indexer_modal
          .as_ref()
          .unwrap()
          .api_key
          .offset
          .load(Ordering::SeqCst),
        4
      );

      EditIndexerHandler::with(
        DEFAULT_KEYBINDINGS.end.key,
        &mut app,
        ActiveRadarrBlock::EditIndexerApiKeyInput,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .radarr_data
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
      let mut app = App::default();
      app.data.radarr_data.edit_indexer_modal = Some(EditIndexerModal {
        seed_ratio: "Test".into(),
        ..EditIndexerModal::default()
      });

      EditIndexerHandler::with(
        DEFAULT_KEYBINDINGS.home.key,
        &mut app,
        ActiveRadarrBlock::EditIndexerSeedRatioInput,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .radarr_data
          .edit_indexer_modal
          .as_ref()
          .unwrap()
          .seed_ratio
          .offset
          .load(Ordering::SeqCst),
        4
      );

      EditIndexerHandler::with(
        DEFAULT_KEYBINDINGS.end.key,
        &mut app,
        ActiveRadarrBlock::EditIndexerSeedRatioInput,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .radarr_data
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
      let mut app = App::default();
      app.data.radarr_data.edit_indexer_modal = Some(EditIndexerModal {
        tags: "Test".into(),
        ..EditIndexerModal::default()
      });

      EditIndexerHandler::with(
        DEFAULT_KEYBINDINGS.home.key,
        &mut app,
        ActiveRadarrBlock::EditIndexerTagsInput,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .radarr_data
          .edit_indexer_modal
          .as_ref()
          .unwrap()
          .tags
          .offset
          .load(Ordering::SeqCst),
        4
      );

      EditIndexerHandler::with(
        DEFAULT_KEYBINDINGS.end.key,
        &mut app,
        ActiveRadarrBlock::EditIndexerTagsInput,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .radarr_data
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
    use crate::models::servarr_data::modals::EditIndexerModal;
    use crate::models::servarr_data::radarr::radarr_data::{
      EDIT_INDEXER_NZB_SELECTION_BLOCKS, EDIT_INDEXER_TORRENT_SELECTION_BLOCKS,
    };
    use crate::models::BlockSelectionState;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    use super::*;

    #[rstest]
    fn test_left_right_prompt_toggle(#[values(Key::Left, Key::Right)] key: Key) {
      let mut app = App::default();
      app.data.radarr_data.selected_block =
        BlockSelectionState::new(EDIT_INDEXER_TORRENT_SELECTION_BLOCKS);
      app.data.radarr_data.selected_block.y = EDIT_INDEXER_TORRENT_SELECTION_BLOCKS.len() - 1;

      EditIndexerHandler::with(key, &mut app, ActiveRadarrBlock::EditIndexerPrompt, None).handle();

      assert!(app.data.radarr_data.prompt_confirm);

      EditIndexerHandler::with(key, &mut app, ActiveRadarrBlock::EditIndexerPrompt, None).handle();

      assert!(!app.data.radarr_data.prompt_confirm);
    }

    #[rstest]
    #[case(
      0,
      ActiveRadarrBlock::EditIndexerNameInput,
      ActiveRadarrBlock::EditIndexerUrlInput
    )]
    #[case(
      1,
      ActiveRadarrBlock::EditIndexerToggleEnableRss,
      ActiveRadarrBlock::EditIndexerApiKeyInput
    )]
    #[case(
      2,
      ActiveRadarrBlock::EditIndexerToggleEnableAutomaticSearch,
      ActiveRadarrBlock::EditIndexerSeedRatioInput
    )]
    #[case(
      3,
      ActiveRadarrBlock::EditIndexerToggleEnableInteractiveSearch,
      ActiveRadarrBlock::EditIndexerTagsInput
    )]
    fn test_left_right_block_toggle_torrents(
      #[values(Key::Left, Key::Right)] key: Key,
      #[case] starting_y_index: usize,
      #[case] left_block: ActiveRadarrBlock,
      #[case] right_block: ActiveRadarrBlock,
    ) {
      let mut app = App::default();
      app.data.radarr_data.selected_block =
        BlockSelectionState::new(EDIT_INDEXER_TORRENT_SELECTION_BLOCKS);
      app.data.radarr_data.selected_block.y = starting_y_index;

      assert_eq!(
        app.data.radarr_data.selected_block.get_active_block(),
        left_block
      );

      EditIndexerHandler::with(key, &mut app, ActiveRadarrBlock::EditIndexerPrompt, None).handle();

      assert_eq!(
        app.data.radarr_data.selected_block.get_active_block(),
        right_block
      );

      EditIndexerHandler::with(key, &mut app, ActiveRadarrBlock::EditIndexerPrompt, None).handle();

      assert_eq!(
        app.data.radarr_data.selected_block.get_active_block(),
        left_block
      );
    }

    #[rstest]
    #[case(
      0,
      ActiveRadarrBlock::EditIndexerNameInput,
      ActiveRadarrBlock::EditIndexerUrlInput
    )]
    #[case(
      1,
      ActiveRadarrBlock::EditIndexerToggleEnableRss,
      ActiveRadarrBlock::EditIndexerApiKeyInput
    )]
    #[case(
      2,
      ActiveRadarrBlock::EditIndexerToggleEnableAutomaticSearch,
      ActiveRadarrBlock::EditIndexerTagsInput
    )]
    fn test_left_right_block_toggle_nzb(
      #[values(Key::Left, Key::Right)] key: Key,
      #[case] starting_y_index: usize,
      #[case] left_block: ActiveRadarrBlock,
      #[case] right_block: ActiveRadarrBlock,
    ) {
      let mut app = App::default();
      app.data.radarr_data.selected_block =
        BlockSelectionState::new(EDIT_INDEXER_NZB_SELECTION_BLOCKS);
      app.data.radarr_data.selected_block.y = starting_y_index;

      assert_eq!(
        app.data.radarr_data.selected_block.get_active_block(),
        left_block
      );

      EditIndexerHandler::with(key, &mut app, ActiveRadarrBlock::EditIndexerPrompt, None).handle();

      assert_eq!(
        app.data.radarr_data.selected_block.get_active_block(),
        right_block
      );

      EditIndexerHandler::with(key, &mut app, ActiveRadarrBlock::EditIndexerPrompt, None).handle();

      assert_eq!(
        app.data.radarr_data.selected_block.get_active_block(),
        left_block
      );
    }

    #[rstest]
    fn test_left_right_block_toggle_nzb_empty_row_to_prompt_confirm(
      #[values(Key::Left, Key::Right)] key: Key,
    ) {
      let mut app = App::default();
      app.data.radarr_data.selected_block =
        BlockSelectionState::new(EDIT_INDEXER_NZB_SELECTION_BLOCKS);
      app.data.radarr_data.selected_block.y = 3;
      app.data.radarr_data.prompt_confirm = false;

      assert_eq!(
        app.data.radarr_data.selected_block.get_active_block(),
        ActiveRadarrBlock::EditIndexerToggleEnableInteractiveSearch
      );

      EditIndexerHandler::with(key, &mut app, ActiveRadarrBlock::EditIndexerPrompt, None).handle();

      assert_eq!(
        app.data.radarr_data.selected_block.get_active_block(),
        ActiveRadarrBlock::EditIndexerConfirmPrompt
      );

      EditIndexerHandler::with(key, &mut app, ActiveRadarrBlock::EditIndexerPrompt, None).handle();

      assert_eq!(
        app.data.radarr_data.selected_block.get_active_block(),
        ActiveRadarrBlock::EditIndexerConfirmPrompt
      );
      assert!(app.data.radarr_data.prompt_confirm);
    }

    #[test]
    fn test_edit_indexer_name_input_left_right_keys() {
      let mut app = App::default();
      app.data.radarr_data.edit_indexer_modal = Some(EditIndexerModal {
        name: "Test".into(),
        ..EditIndexerModal::default()
      });

      EditIndexerHandler::with(
        DEFAULT_KEYBINDINGS.left.key,
        &mut app,
        ActiveRadarrBlock::EditIndexerNameInput,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .radarr_data
          .edit_indexer_modal
          .as_ref()
          .unwrap()
          .name
          .offset
          .load(Ordering::SeqCst),
        1
      );

      EditIndexerHandler::with(
        DEFAULT_KEYBINDINGS.right.key,
        &mut app,
        ActiveRadarrBlock::EditIndexerNameInput,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .radarr_data
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
      let mut app = App::default();
      app.data.radarr_data.edit_indexer_modal = Some(EditIndexerModal {
        url: "Test".into(),
        ..EditIndexerModal::default()
      });

      EditIndexerHandler::with(
        DEFAULT_KEYBINDINGS.left.key,
        &mut app,
        ActiveRadarrBlock::EditIndexerUrlInput,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .radarr_data
          .edit_indexer_modal
          .as_ref()
          .unwrap()
          .url
          .offset
          .load(Ordering::SeqCst),
        1
      );

      EditIndexerHandler::with(
        DEFAULT_KEYBINDINGS.right.key,
        &mut app,
        ActiveRadarrBlock::EditIndexerUrlInput,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .radarr_data
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
      let mut app = App::default();
      app.data.radarr_data.edit_indexer_modal = Some(EditIndexerModal {
        api_key: "Test".into(),
        ..EditIndexerModal::default()
      });

      EditIndexerHandler::with(
        DEFAULT_KEYBINDINGS.left.key,
        &mut app,
        ActiveRadarrBlock::EditIndexerApiKeyInput,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .radarr_data
          .edit_indexer_modal
          .as_ref()
          .unwrap()
          .api_key
          .offset
          .load(Ordering::SeqCst),
        1
      );

      EditIndexerHandler::with(
        DEFAULT_KEYBINDINGS.right.key,
        &mut app,
        ActiveRadarrBlock::EditIndexerApiKeyInput,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .radarr_data
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
      let mut app = App::default();
      app.data.radarr_data.edit_indexer_modal = Some(EditIndexerModal {
        seed_ratio: "Test".into(),
        ..EditIndexerModal::default()
      });

      EditIndexerHandler::with(
        DEFAULT_KEYBINDINGS.left.key,
        &mut app,
        ActiveRadarrBlock::EditIndexerSeedRatioInput,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .radarr_data
          .edit_indexer_modal
          .as_ref()
          .unwrap()
          .seed_ratio
          .offset
          .load(Ordering::SeqCst),
        1
      );

      EditIndexerHandler::with(
        DEFAULT_KEYBINDINGS.right.key,
        &mut app,
        ActiveRadarrBlock::EditIndexerSeedRatioInput,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .radarr_data
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
      let mut app = App::default();
      app.data.radarr_data.edit_indexer_modal = Some(EditIndexerModal {
        tags: "Test".into(),
        ..EditIndexerModal::default()
      });

      EditIndexerHandler::with(
        DEFAULT_KEYBINDINGS.left.key,
        &mut app,
        ActiveRadarrBlock::EditIndexerTagsInput,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .radarr_data
          .edit_indexer_modal
          .as_ref()
          .unwrap()
          .tags
          .offset
          .load(Ordering::SeqCst),
        1
      );

      EditIndexerHandler::with(
        DEFAULT_KEYBINDINGS.right.key,
        &mut app,
        ActiveRadarrBlock::EditIndexerTagsInput,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .radarr_data
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
    use crate::models::servarr_data::modals::EditIndexerModal;
    use crate::models::{
      servarr_data::radarr::radarr_data::EDIT_INDEXER_TORRENT_SELECTION_BLOCKS, BlockSelectionState,
    };
    use crate::network::radarr_network::RadarrEvent;

    use super::*;

    const SUBMIT_KEY: Key = DEFAULT_KEYBINDINGS.submit.key;

    #[test]
    fn test_edit_indexer_prompt_prompt_decline_submit() {
      let mut app = App::default();
      app.push_navigation_stack(ActiveRadarrBlock::Indexers.into());
      app.push_navigation_stack(ActiveRadarrBlock::EditIndexerPrompt.into());
      app.data.radarr_data.selected_block =
        BlockSelectionState::new(EDIT_INDEXER_TORRENT_SELECTION_BLOCKS);
      app
        .data
        .radarr_data
        .selected_block
        .set_index(0, EDIT_INDEXER_TORRENT_SELECTION_BLOCKS.len() - 1);
      app.data.radarr_data.edit_indexer_modal = Some(EditIndexerModal::default());

      EditIndexerHandler::with(
        SUBMIT_KEY,
        &mut app,
        ActiveRadarrBlock::EditIndexerPrompt,
        None,
      )
      .handle();

      assert_eq!(app.get_current_route(), ActiveRadarrBlock::Indexers.into());
      assert_eq!(app.data.radarr_data.prompt_confirm_action, None);
      assert!(!app.should_refresh);
      assert_eq!(app.data.radarr_data.edit_indexer_modal, None);
    }

    #[test]
    fn test_edit_indexer_prompt_prompt_confirmation_submit() {
      let mut app = App::default();
      app.push_navigation_stack(ActiveRadarrBlock::Indexers.into());
      app.push_navigation_stack(ActiveRadarrBlock::EditIndexerPrompt.into());
      app.data.radarr_data.selected_block =
        BlockSelectionState::new(EDIT_INDEXER_TORRENT_SELECTION_BLOCKS);
      app
        .data
        .radarr_data
        .selected_block
        .set_index(0, EDIT_INDEXER_TORRENT_SELECTION_BLOCKS.len() - 1);
      app.data.radarr_data.edit_indexer_modal = Some(EditIndexerModal::default());
      app.data.radarr_data.prompt_confirm = true;

      EditIndexerHandler::with(
        SUBMIT_KEY,
        &mut app,
        ActiveRadarrBlock::EditIndexerPrompt,
        None,
      )
      .handle();

      assert_eq!(app.get_current_route(), ActiveRadarrBlock::Indexers.into());
      assert!(app.data.radarr_data.edit_indexer_modal.is_some());
      assert!(app.should_refresh);
      assert_eq!(
        app.data.radarr_data.prompt_confirm_action,
        Some(RadarrEvent::EditIndexer(None))
      );
    }

    #[test]
    fn test_edit_indexer_prompt_prompt_confirmation_submit_no_op_when_not_ready() {
      let mut app = App::default();
      app.is_loading = true;
      app.push_navigation_stack(ActiveRadarrBlock::Indexers.into());
      app.push_navigation_stack(ActiveRadarrBlock::EditIndexerPrompt.into());
      app.data.radarr_data.edit_indexer_modal = Some(EditIndexerModal::default());
      app.data.radarr_data.prompt_confirm = true;

      EditIndexerHandler::with(
        SUBMIT_KEY,
        &mut app,
        ActiveRadarrBlock::EditIndexerPrompt,
        None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveRadarrBlock::EditIndexerPrompt.into()
      );
      assert!(app.data.radarr_data.edit_indexer_modal.is_some());
      assert!(!app.should_refresh);
      assert_eq!(app.data.radarr_data.prompt_confirm_action, None);
    }

    #[rstest]
    #[case(0, 0, ActiveRadarrBlock::EditIndexerNameInput)]
    #[case(0, 1, ActiveRadarrBlock::EditIndexerUrlInput)]
    #[case(1, 1, ActiveRadarrBlock::EditIndexerApiKeyInput)]
    #[case(2, 1, ActiveRadarrBlock::EditIndexerSeedRatioInput)]
    #[case(3, 1, ActiveRadarrBlock::EditIndexerTagsInput)]
    fn test_edit_indexer_prompt_submit_input_fields(
      #[case] starting_y: usize,
      #[case] starting_x: usize,
      #[case] block: ActiveRadarrBlock,
    ) {
      let mut app = App::default();
      app.data.radarr_data.edit_indexer_modal = Some(EditIndexerModal::default());
      app.push_navigation_stack(ActiveRadarrBlock::EditIndexerPrompt.into());
      app.data.radarr_data.selected_block =
        BlockSelectionState::new(EDIT_INDEXER_TORRENT_SELECTION_BLOCKS);
      app
        .data
        .radarr_data
        .selected_block
        .set_index(starting_x, starting_y);

      EditIndexerHandler::with(
        SUBMIT_KEY,
        &mut app,
        ActiveRadarrBlock::EditIndexerPrompt,
        None,
      )
      .handle();

      assert_eq!(app.get_current_route(), block.into());
      assert!(app.should_ignore_quit_key);
    }

    #[test]
    fn test_edit_indexer_toggle_enable_rss_submit() {
      let mut app = App::default();
      app.data.radarr_data.edit_indexer_modal = Some(EditIndexerModal::default());
      app.data.radarr_data.selected_block =
        BlockSelectionState::new(EDIT_INDEXER_TORRENT_SELECTION_BLOCKS);
      app.data.radarr_data.selected_block.set_index(0, 1);
      app.push_navigation_stack(ActiveRadarrBlock::EditIndexerPrompt.into());

      EditIndexerHandler::with(
        SUBMIT_KEY,
        &mut app,
        ActiveRadarrBlock::EditIndexerPrompt,
        None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveRadarrBlock::EditIndexerPrompt.into()
      );
      assert!(app
        .data
        .radarr_data
        .edit_indexer_modal
        .as_ref()
        .unwrap()
        .enable_rss
        .unwrap());

      EditIndexerHandler::with(
        SUBMIT_KEY,
        &mut app,
        ActiveRadarrBlock::EditIndexerPrompt,
        None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveRadarrBlock::EditIndexerPrompt.into()
      );
      assert!(!app
        .data
        .radarr_data
        .edit_indexer_modal
        .as_ref()
        .unwrap()
        .enable_rss
        .unwrap());
    }

    #[test]
    fn test_edit_indexer_toggle_enable_automatic_search_submit() {
      let mut app = App::default();
      app.data.radarr_data.edit_indexer_modal = Some(EditIndexerModal::default());
      app.data.radarr_data.selected_block =
        BlockSelectionState::new(EDIT_INDEXER_TORRENT_SELECTION_BLOCKS);
      app.data.radarr_data.selected_block.set_index(0, 2);
      app.push_navigation_stack(ActiveRadarrBlock::EditIndexerPrompt.into());

      EditIndexerHandler::with(
        SUBMIT_KEY,
        &mut app,
        ActiveRadarrBlock::EditIndexerPrompt,
        None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveRadarrBlock::EditIndexerPrompt.into()
      );
      assert!(app
        .data
        .radarr_data
        .edit_indexer_modal
        .as_ref()
        .unwrap()
        .enable_automatic_search
        .unwrap());

      EditIndexerHandler::with(
        SUBMIT_KEY,
        &mut app,
        ActiveRadarrBlock::EditIndexerPrompt,
        None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveRadarrBlock::EditIndexerPrompt.into()
      );
      assert!(!app
        .data
        .radarr_data
        .edit_indexer_modal
        .as_ref()
        .unwrap()
        .enable_automatic_search
        .unwrap());
    }

    #[test]
    fn test_edit_indexer_toggle_enable_interactive_search_submit() {
      let mut app = App::default();
      app.data.radarr_data.edit_indexer_modal = Some(EditIndexerModal::default());
      app.data.radarr_data.selected_block =
        BlockSelectionState::new(EDIT_INDEXER_TORRENT_SELECTION_BLOCKS);
      app.data.radarr_data.selected_block.set_index(0, 3);
      app.push_navigation_stack(ActiveRadarrBlock::EditIndexerPrompt.into());

      EditIndexerHandler::with(
        SUBMIT_KEY,
        &mut app,
        ActiveRadarrBlock::EditIndexerPrompt,
        None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveRadarrBlock::EditIndexerPrompt.into()
      );
      assert!(app
        .data
        .radarr_data
        .edit_indexer_modal
        .as_ref()
        .unwrap()
        .enable_interactive_search
        .unwrap());

      EditIndexerHandler::with(
        SUBMIT_KEY,
        &mut app,
        ActiveRadarrBlock::EditIndexerPrompt,
        None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveRadarrBlock::EditIndexerPrompt.into()
      );
      assert!(!app
        .data
        .radarr_data
        .edit_indexer_modal
        .as_ref()
        .unwrap()
        .enable_interactive_search
        .unwrap());
    }

    #[test]
    fn test_edit_indexer_name_input_submit() {
      let mut app = App::default();
      app.should_ignore_quit_key = true;
      app.data.radarr_data.edit_indexer_modal = Some(EditIndexerModal {
        name: "Test".into(),
        ..EditIndexerModal::default()
      });
      app.push_navigation_stack(ActiveRadarrBlock::EditIndexerPrompt.into());
      app.push_navigation_stack(ActiveRadarrBlock::EditIndexerNameInput.into());

      EditIndexerHandler::with(
        SUBMIT_KEY,
        &mut app,
        ActiveRadarrBlock::EditIndexerNameInput,
        None,
      )
      .handle();

      assert!(!app.should_ignore_quit_key);
      assert!(!app
        .data
        .radarr_data
        .edit_indexer_modal
        .as_ref()
        .unwrap()
        .name
        .text
        .is_empty());
      assert_eq!(
        app.get_current_route(),
        ActiveRadarrBlock::EditIndexerPrompt.into()
      );
    }

    #[test]
    fn test_edit_indexer_url_input_submit() {
      let mut app = App::default();
      app.should_ignore_quit_key = true;
      app.data.radarr_data.edit_indexer_modal = Some(EditIndexerModal {
        url: "Test".into(),
        ..EditIndexerModal::default()
      });
      app.push_navigation_stack(ActiveRadarrBlock::EditIndexerPrompt.into());
      app.push_navigation_stack(ActiveRadarrBlock::EditIndexerUrlInput.into());

      EditIndexerHandler::with(
        SUBMIT_KEY,
        &mut app,
        ActiveRadarrBlock::EditIndexerUrlInput,
        None,
      )
      .handle();

      assert!(!app.should_ignore_quit_key);
      assert!(!app
        .data
        .radarr_data
        .edit_indexer_modal
        .as_ref()
        .unwrap()
        .url
        .text
        .is_empty());
      assert_eq!(
        app.get_current_route(),
        ActiveRadarrBlock::EditIndexerPrompt.into()
      );
    }

    #[test]
    fn test_edit_indexer_api_key_input_submit() {
      let mut app = App::default();
      app.should_ignore_quit_key = true;
      app.data.radarr_data.edit_indexer_modal = Some(EditIndexerModal {
        api_key: "Test".into(),
        ..EditIndexerModal::default()
      });
      app.push_navigation_stack(ActiveRadarrBlock::EditIndexerPrompt.into());
      app.push_navigation_stack(ActiveRadarrBlock::EditIndexerApiKeyInput.into());

      EditIndexerHandler::with(
        SUBMIT_KEY,
        &mut app,
        ActiveRadarrBlock::EditIndexerApiKeyInput,
        None,
      )
      .handle();

      assert!(!app.should_ignore_quit_key);
      assert!(!app
        .data
        .radarr_data
        .edit_indexer_modal
        .as_ref()
        .unwrap()
        .api_key
        .text
        .is_empty());
      assert_eq!(
        app.get_current_route(),
        ActiveRadarrBlock::EditIndexerPrompt.into()
      );
    }

    #[test]
    fn test_edit_indexer_seed_ratio_input_submit() {
      let mut app = App::default();
      app.should_ignore_quit_key = true;
      app.data.radarr_data.edit_indexer_modal = Some(EditIndexerModal {
        seed_ratio: "Test".into(),
        ..EditIndexerModal::default()
      });
      app.push_navigation_stack(ActiveRadarrBlock::EditIndexerPrompt.into());
      app.push_navigation_stack(ActiveRadarrBlock::EditIndexerSeedRatioInput.into());

      EditIndexerHandler::with(
        SUBMIT_KEY,
        &mut app,
        ActiveRadarrBlock::EditIndexerSeedRatioInput,
        None,
      )
      .handle();

      assert!(!app.should_ignore_quit_key);
      assert!(!app
        .data
        .radarr_data
        .edit_indexer_modal
        .as_ref()
        .unwrap()
        .seed_ratio
        .text
        .is_empty());
      assert_eq!(
        app.get_current_route(),
        ActiveRadarrBlock::EditIndexerPrompt.into()
      );
    }

    #[test]
    fn test_edit_indexer_tags_input_submit() {
      let mut app = App::default();
      app.should_ignore_quit_key = true;
      app.data.radarr_data.edit_indexer_modal = Some(EditIndexerModal {
        tags: "Test".into(),
        ..EditIndexerModal::default()
      });
      app.push_navigation_stack(ActiveRadarrBlock::EditIndexerPrompt.into());
      app.push_navigation_stack(ActiveRadarrBlock::EditIndexerTagsInput.into());

      EditIndexerHandler::with(
        SUBMIT_KEY,
        &mut app,
        ActiveRadarrBlock::EditIndexerTagsInput,
        None,
      )
      .handle();

      assert!(!app.should_ignore_quit_key);
      assert!(!app
        .data
        .radarr_data
        .edit_indexer_modal
        .as_ref()
        .unwrap()
        .tags
        .text
        .is_empty());
      assert_eq!(
        app.get_current_route(),
        ActiveRadarrBlock::EditIndexerPrompt.into()
      );
    }
  }

  mod test_handle_esc {
    use super::*;
    use crate::app::App;
    use crate::event::Key;
    use crate::models::servarr_data::modals::EditIndexerModal;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    const ESC_KEY: Key = DEFAULT_KEYBINDINGS.esc.key;

    #[rstest]
    fn test_edit_indexer_prompt_esc(#[values(true, false)] is_ready: bool) {
      let mut app = App::default();
      app.is_loading = is_ready;
      app.push_navigation_stack(ActiveRadarrBlock::Indexers.into());
      app.push_navigation_stack(ActiveRadarrBlock::EditIndexerPrompt.into());
      app.data.radarr_data.edit_indexer_modal = Some(EditIndexerModal::default());

      EditIndexerHandler::with(
        ESC_KEY,
        &mut app,
        ActiveRadarrBlock::EditIndexerPrompt,
        None,
      )
      .handle();

      assert_eq!(app.get_current_route(), ActiveRadarrBlock::Indexers.into());
      assert!(!app.data.radarr_data.prompt_confirm);
      assert_eq!(app.data.radarr_data.edit_indexer_modal, None);
    }

    #[rstest]
    fn test_edit_indexer_input_fields_esc(
      #[values(
        ActiveRadarrBlock::EditIndexerNameInput,
        ActiveRadarrBlock::EditIndexerUrlInput,
        ActiveRadarrBlock::EditIndexerApiKeyInput,
        ActiveRadarrBlock::EditIndexerSeedRatioInput,
        ActiveRadarrBlock::EditIndexerTagsInput
      )]
      active_radarr_block: ActiveRadarrBlock,
    ) {
      let mut app = App::default();
      app.push_navigation_stack(ActiveRadarrBlock::Indexers.into());
      app.push_navigation_stack(active_radarr_block.into());
      app.data.radarr_data.edit_indexer_modal = Some(EditIndexerModal::default());
      app.should_ignore_quit_key = true;

      EditIndexerHandler::with(ESC_KEY, &mut app, active_radarr_block, None).handle();

      assert_eq!(app.get_current_route(), ActiveRadarrBlock::Indexers.into());
      assert!(!app.should_ignore_quit_key);
      assert_eq!(
        app.data.radarr_data.edit_indexer_modal,
        Some(EditIndexerModal::default())
      );
    }
  }

  mod test_handle_key_char {
    use crate::app::App;
    use crate::models::servarr_data::modals::EditIndexerModal;
    use crate::models::servarr_data::radarr::radarr_data::EDIT_INDEXER_TORRENT_SELECTION_BLOCKS;
    use crate::models::BlockSelectionState;
    use crate::network::radarr_network::RadarrEvent;
    use pretty_assertions::assert_str_eq;

    use super::*;

    #[test]
    fn test_edit_indexer_name_input_backspace() {
      let mut app = App::default();
      app.data.radarr_data.edit_indexer_modal = Some(EditIndexerModal {
        name: "Test".into(),
        ..EditIndexerModal::default()
      });

      EditIndexerHandler::with(
        DEFAULT_KEYBINDINGS.backspace.key,
        &mut app,
        ActiveRadarrBlock::EditIndexerNameInput,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .radarr_data
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
      let mut app = App::default();
      app.data.radarr_data.edit_indexer_modal = Some(EditIndexerModal {
        url: "Test".into(),
        ..EditIndexerModal::default()
      });

      EditIndexerHandler::with(
        DEFAULT_KEYBINDINGS.backspace.key,
        &mut app,
        ActiveRadarrBlock::EditIndexerUrlInput,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .radarr_data
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
      let mut app = App::default();
      app.data.radarr_data.edit_indexer_modal = Some(EditIndexerModal {
        api_key: "Test".into(),
        ..EditIndexerModal::default()
      });

      EditIndexerHandler::with(
        DEFAULT_KEYBINDINGS.backspace.key,
        &mut app,
        ActiveRadarrBlock::EditIndexerApiKeyInput,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .radarr_data
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
      let mut app = App::default();
      app.data.radarr_data.edit_indexer_modal = Some(EditIndexerModal {
        seed_ratio: "Test".into(),
        ..EditIndexerModal::default()
      });

      EditIndexerHandler::with(
        DEFAULT_KEYBINDINGS.backspace.key,
        &mut app,
        ActiveRadarrBlock::EditIndexerSeedRatioInput,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .radarr_data
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
      let mut app = App::default();
      app.data.radarr_data.edit_indexer_modal = Some(EditIndexerModal {
        tags: "Test".into(),
        ..EditIndexerModal::default()
      });

      EditIndexerHandler::with(
        DEFAULT_KEYBINDINGS.backspace.key,
        &mut app,
        ActiveRadarrBlock::EditIndexerTagsInput,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .radarr_data
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
      let mut app = App::default();
      app.data.radarr_data.edit_indexer_modal = Some(EditIndexerModal::default());

      EditIndexerHandler::with(
        Key::Char('h'),
        &mut app,
        ActiveRadarrBlock::EditIndexerNameInput,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .radarr_data
          .edit_indexer_modal
          .as_ref()
          .unwrap()
          .name
          .text,
        "h"
      );
    }

    #[test]
    fn test_edit_indexer_url_input_char_key() {
      let mut app = App::default();
      app.data.radarr_data.edit_indexer_modal = Some(EditIndexerModal::default());

      EditIndexerHandler::with(
        Key::Char('h'),
        &mut app,
        ActiveRadarrBlock::EditIndexerUrlInput,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .radarr_data
          .edit_indexer_modal
          .as_ref()
          .unwrap()
          .url
          .text,
        "h"
      );
    }

    #[test]
    fn test_edit_indexer_api_key_input_char_key() {
      let mut app = App::default();
      app.data.radarr_data.edit_indexer_modal = Some(EditIndexerModal::default());

      EditIndexerHandler::with(
        Key::Char('h'),
        &mut app,
        ActiveRadarrBlock::EditIndexerApiKeyInput,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .radarr_data
          .edit_indexer_modal
          .as_ref()
          .unwrap()
          .api_key
          .text,
        "h"
      );
    }

    #[test]
    fn test_edit_indexer_seed_ratio_input_char_key() {
      let mut app = App::default();
      app.data.radarr_data.edit_indexer_modal = Some(EditIndexerModal::default());

      EditIndexerHandler::with(
        Key::Char('h'),
        &mut app,
        ActiveRadarrBlock::EditIndexerSeedRatioInput,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .radarr_data
          .edit_indexer_modal
          .as_ref()
          .unwrap()
          .seed_ratio
          .text,
        "h"
      );
    }

    #[test]
    fn test_edit_indexer_tags_input_char_key() {
      let mut app = App::default();
      app.data.radarr_data.edit_indexer_modal = Some(EditIndexerModal::default());

      EditIndexerHandler::with(
        Key::Char('h'),
        &mut app,
        ActiveRadarrBlock::EditIndexerTagsInput,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .radarr_data
          .edit_indexer_modal
          .as_ref()
          .unwrap()
          .tags
          .text,
        "h"
      );
    }

    #[test]
    fn test_edit_indexer_prompt_prompt_confirmation_confirm() {
      let mut app = App::default();
      app.push_navigation_stack(ActiveRadarrBlock::Indexers.into());
      app.push_navigation_stack(ActiveRadarrBlock::EditIndexerPrompt.into());
      app.data.radarr_data.selected_block =
        BlockSelectionState::new(EDIT_INDEXER_TORRENT_SELECTION_BLOCKS);
      app
        .data
        .radarr_data
        .selected_block
        .set_index(0, EDIT_INDEXER_TORRENT_SELECTION_BLOCKS.len() - 1);
      app.data.radarr_data.edit_indexer_modal = Some(EditIndexerModal::default());

      EditIndexerHandler::with(
        DEFAULT_KEYBINDINGS.confirm.key,
        &mut app,
        ActiveRadarrBlock::EditIndexerPrompt,
        None,
      )
      .handle();

      assert_eq!(app.get_current_route(), ActiveRadarrBlock::Indexers.into());
      assert!(app.data.radarr_data.edit_indexer_modal.is_some());
      assert!(app.should_refresh);
      assert_eq!(
        app.data.radarr_data.prompt_confirm_action,
        Some(RadarrEvent::EditIndexer(None))
      );
    }
  }

  #[test]
  fn test_indexer_settings_handler_accepts() {
    ActiveRadarrBlock::iter().for_each(|active_radarr_block| {
      if EDIT_INDEXER_BLOCKS.contains(&active_radarr_block) {
        assert!(EditIndexerHandler::accepts(active_radarr_block));
      } else {
        assert!(!EditIndexerHandler::accepts(active_radarr_block));
      }
    })
  }

  #[test]
  fn test_edit_indexer_handler_is_not_ready_when_loading() {
    let mut app = App::default();
    app.is_loading = true;

    let handler = EditIndexerHandler::with(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveRadarrBlock::EditIndexerPrompt,
      None,
    );

    assert!(!handler.is_ready());
  }

  #[test]
  fn test_edit_indexer_handler_is_not_ready_when_edit_indexer_modal_is_none() {
    let mut app = App::default();
    app.is_loading = false;

    let handler = EditIndexerHandler::with(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveRadarrBlock::EditIndexerPrompt,
      None,
    );

    assert!(!handler.is_ready());
  }

  #[test]
  fn test_edit_indexer_handler_is_ready_when_edit_indexer_modal_is_some() {
    let mut app = App::default();
    app.is_loading = false;
    app.data.radarr_data.edit_indexer_modal = Some(EditIndexerModal::default());

    let handler = EditIndexerHandler::with(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveRadarrBlock::EditIndexerPrompt,
      None,
    );

    assert!(handler.is_ready());
  }
}
