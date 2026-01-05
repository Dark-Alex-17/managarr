#[cfg(test)]
mod tests {
  use pretty_assertions::assert_eq;
  use rstest::rstest;
  use strum::IntoEnumIterator;

  use crate::app::App;
  use crate::app::key_binding::DEFAULT_KEYBINDINGS;
  use crate::event::Key;
  use crate::handlers::KeyEventHandler;
  use crate::handlers::lidarr_handlers::library::delete_artist_handler::DeleteArtistHandler;
  use crate::models::lidarr_models::{Artist, DeleteArtistParams};
  use crate::models::servarr_data::lidarr::lidarr_data::{ActiveLidarrBlock, DELETE_ARTIST_BLOCKS};

  mod test_handle_scroll_up_and_down {
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    use crate::models::BlockSelectionState;
    use crate::models::servarr_data::lidarr::lidarr_data::DELETE_ARTIST_SELECTION_BLOCKS;

    use super::*;

    #[rstest]
    fn test_delete_artist_prompt_scroll(#[values(Key::Up, Key::Down)] key: Key) {
      let mut app = App::test_default();
      app.data.lidarr_data.selected_block =
        BlockSelectionState::new(DELETE_ARTIST_SELECTION_BLOCKS);
      app.data.lidarr_data.selected_block.down();

      DeleteArtistHandler::new(key, &mut app, ActiveLidarrBlock::DeleteArtistPrompt, None).handle();

      if key == Key::Up {
        assert_eq!(
          app.data.lidarr_data.selected_block.get_active_block(),
          ActiveLidarrBlock::DeleteArtistToggleDeleteFile
        );
      } else {
        assert_eq!(
          app.data.lidarr_data.selected_block.get_active_block(),
          ActiveLidarrBlock::DeleteArtistConfirmPrompt
        );
      }
    }

    #[rstest]
    fn test_delete_artist_prompt_scroll_no_op_when_not_ready(
      #[values(Key::Up, Key::Down)] key: Key,
    ) {
      let mut app = App::test_default();
      app.is_loading = true;
      app.data.lidarr_data.selected_block =
        BlockSelectionState::new(DELETE_ARTIST_SELECTION_BLOCKS);
      app.data.lidarr_data.selected_block.down();

      DeleteArtistHandler::new(key, &mut app, ActiveLidarrBlock::DeleteArtistPrompt, None).handle();

      assert_eq!(
        app.data.lidarr_data.selected_block.get_active_block(),
        ActiveLidarrBlock::DeleteArtistToggleAddListExclusion
      );
    }
  }

  mod test_handle_left_right_action {
    use rstest::rstest;

    use super::*;

    #[rstest]
    fn test_left_right_prompt_toggle(#[values(Key::Left, Key::Right)] key: Key) {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveLidarrBlock::DeleteArtistPrompt.into());

      DeleteArtistHandler::new(key, &mut app, ActiveLidarrBlock::DeleteArtistPrompt, None).handle();

      assert!(app.data.lidarr_data.prompt_confirm);

      DeleteArtistHandler::new(key, &mut app, ActiveLidarrBlock::DeleteArtistPrompt, None).handle();

      assert!(!app.data.lidarr_data.prompt_confirm);
    }
  }

  mod test_handle_submit {
    use pretty_assertions::assert_eq;

    use crate::models::BlockSelectionState;
    use crate::models::servarr_data::lidarr::lidarr_data::DELETE_ARTIST_SELECTION_BLOCKS;
    use crate::network::lidarr_network::LidarrEvent;

    use super::*;
    use crate::assert_navigation_popped;

    const SUBMIT_KEY: Key = DEFAULT_KEYBINDINGS.submit.key;

    #[test]
    fn test_delete_artist_prompt_prompt_decline_submit() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveLidarrBlock::Artists.into());
      app.push_navigation_stack(ActiveLidarrBlock::DeleteArtistPrompt.into());
      app.data.lidarr_data.selected_block =
        BlockSelectionState::new(DELETE_ARTIST_SELECTION_BLOCKS);
      app
        .data
        .lidarr_data
        .selected_block
        .set_index(0, DELETE_ARTIST_SELECTION_BLOCKS.len() - 1);
      app.data.lidarr_data.delete_artist_files = true;
      app.data.lidarr_data.add_import_list_exclusion = true;

      DeleteArtistHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveLidarrBlock::DeleteArtistPrompt,
        None,
      )
      .handle();

      assert_navigation_popped!(app, ActiveLidarrBlock::Artists.into());
      assert_none!(app.data.lidarr_data.prompt_confirm_action);
      assert!(!app.data.lidarr_data.prompt_confirm);
      assert!(!app.data.lidarr_data.delete_artist_files);
      assert!(!app.data.lidarr_data.add_import_list_exclusion);
    }

    #[test]
    fn test_delete_artist_confirm_prompt_prompt_confirmation_submit() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveLidarrBlock::Artists.into());
      app.push_navigation_stack(ActiveLidarrBlock::DeleteArtistPrompt.into());
      app.data.lidarr_data.prompt_confirm = true;
      app.data.lidarr_data.delete_artist_files = true;
      app.data.lidarr_data.add_import_list_exclusion = true;
      app
        .data
        .lidarr_data
        .artists
        .set_items(vec![Artist::default()]);
      let expected_delete_artist_params = DeleteArtistParams {
        id: 0,
        delete_files: true,
        add_import_list_exclusion: true,
      };
      app.data.lidarr_data.selected_block =
        BlockSelectionState::new(DELETE_ARTIST_SELECTION_BLOCKS);
      app
        .data
        .lidarr_data
        .selected_block
        .set_index(0, DELETE_ARTIST_SELECTION_BLOCKS.len() - 1);

      DeleteArtistHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveLidarrBlock::DeleteArtistPrompt,
        None,
      )
      .handle();

      assert_navigation_popped!(app, ActiveLidarrBlock::Artists.into());
      assert_eq!(
        app.data.lidarr_data.prompt_confirm_action,
        Some(LidarrEvent::DeleteArtist(expected_delete_artist_params))
      );
      assert!(app.should_refresh);
      assert!(app.data.lidarr_data.prompt_confirm);
      assert!(!app.data.lidarr_data.delete_artist_files);
      assert!(!app.data.lidarr_data.add_import_list_exclusion);
    }

    #[test]
    fn test_delete_artist_confirm_prompt_prompt_confirmation_submit_no_op_when_not_ready() {
      let mut app = App::test_default();
      app.is_loading = true;
      app.push_navigation_stack(ActiveLidarrBlock::Artists.into());
      app.push_navigation_stack(ActiveLidarrBlock::DeleteArtistPrompt.into());
      app.data.lidarr_data.prompt_confirm = true;
      app.data.lidarr_data.delete_artist_files = true;
      app.data.lidarr_data.add_import_list_exclusion = true;

      DeleteArtistHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveLidarrBlock::DeleteArtistPrompt,
        None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveLidarrBlock::DeleteArtistPrompt.into()
      );
      assert_none!(app.data.lidarr_data.prompt_confirm_action);
      assert!(!app.should_refresh);
      assert!(app.data.lidarr_data.prompt_confirm);
      assert!(app.data.lidarr_data.delete_artist_files);
      assert!(app.data.lidarr_data.add_import_list_exclusion);
    }

    #[test]
    fn test_delete_artist_toggle_delete_files_submit() {
      let current_route = ActiveLidarrBlock::DeleteArtistPrompt.into();
      let mut app = App::test_default();
      app.data.lidarr_data.selected_block =
        BlockSelectionState::new(DELETE_ARTIST_SELECTION_BLOCKS);
      app.push_navigation_stack(ActiveLidarrBlock::DeleteArtistPrompt.into());

      DeleteArtistHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveLidarrBlock::DeleteArtistPrompt,
        None,
      )
      .handle();

      assert_eq!(app.get_current_route(), current_route);
      assert_eq!(app.data.lidarr_data.delete_artist_files, true);

      DeleteArtistHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveLidarrBlock::DeleteArtistPrompt,
        None,
      )
      .handle();

      assert_eq!(app.get_current_route(), current_route);
      assert_eq!(app.data.lidarr_data.delete_artist_files, false);
    }
  }

  mod test_handle_esc {
    use super::*;
    use crate::assert_navigation_popped;
    use rstest::rstest;

    const ESC_KEY: Key = DEFAULT_KEYBINDINGS.esc.key;

    #[rstest]
    fn test_delete_artist_prompt_esc(#[values(true, false)] is_ready: bool) {
      let mut app = App::test_default();
      app.is_loading = is_ready;
      app.push_navigation_stack(ActiveLidarrBlock::Artists.into());
      app.push_navigation_stack(ActiveLidarrBlock::DeleteArtistPrompt.into());
      app.data.lidarr_data.prompt_confirm = true;
      app.data.lidarr_data.delete_artist_files = true;
      app.data.lidarr_data.add_import_list_exclusion = true;

      DeleteArtistHandler::new(
        ESC_KEY,
        &mut app,
        ActiveLidarrBlock::DeleteArtistPrompt,
        None,
      )
      .handle();

      assert_navigation_popped!(app, ActiveLidarrBlock::Artists.into());
      assert!(!app.data.lidarr_data.prompt_confirm);
      assert!(!app.data.lidarr_data.delete_artist_files);
      assert!(!app.data.lidarr_data.add_import_list_exclusion);
    }
  }

  mod test_handle_key_char {
    use crate::{
      assert_navigation_popped,
      models::{
        BlockSelectionState, servarr_data::lidarr::lidarr_data::DELETE_ARTIST_SELECTION_BLOCKS,
      },
      network::lidarr_network::LidarrEvent,
    };
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_delete_artist_confirm_prompt_prompt_confirm() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveLidarrBlock::Artists.into());
      app.push_navigation_stack(ActiveLidarrBlock::DeleteArtistPrompt.into());
      app.data.lidarr_data.delete_artist_files = true;
      app.data.lidarr_data.add_import_list_exclusion = true;
      app
        .data
        .lidarr_data
        .artists
        .set_items(vec![Artist::default()]);
      let expected_delete_artist_params = DeleteArtistParams {
        id: 0,
        delete_files: true,
        add_import_list_exclusion: true,
      };
      app.data.lidarr_data.selected_block =
        BlockSelectionState::new(DELETE_ARTIST_SELECTION_BLOCKS);
      app
        .data
        .lidarr_data
        .selected_block
        .set_index(0, DELETE_ARTIST_SELECTION_BLOCKS.len() - 1);

      DeleteArtistHandler::new(
        DEFAULT_KEYBINDINGS.confirm.key,
        &mut app,
        ActiveLidarrBlock::DeleteArtistPrompt,
        None,
      )
      .handle();

      assert_navigation_popped!(app, ActiveLidarrBlock::Artists.into());
      assert_eq!(
        app.data.lidarr_data.prompt_confirm_action,
        Some(LidarrEvent::DeleteArtist(expected_delete_artist_params))
      );
      assert!(app.should_refresh);
      assert!(app.data.lidarr_data.prompt_confirm);
      assert!(!app.data.lidarr_data.delete_artist_files);
      assert!(!app.data.lidarr_data.add_import_list_exclusion);
    }
  }

  #[test]
  fn test_delete_artist_handler_accepts() {
    ActiveLidarrBlock::iter().for_each(|active_lidarr_block| {
      if DELETE_ARTIST_BLOCKS.contains(&active_lidarr_block) {
        assert!(DeleteArtistHandler::accepts(active_lidarr_block));
      } else {
        assert!(!DeleteArtistHandler::accepts(active_lidarr_block));
      }
    });
  }

  #[rstest]
  fn test_delete_artist_handler_ignore_special_keys(
    #[values(true, false)] ignore_special_keys_for_textbox_input: bool,
  ) {
    let mut app = App::test_default();
    app.ignore_special_keys_for_textbox_input = ignore_special_keys_for_textbox_input;
    let handler = DeleteArtistHandler::new(
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
  fn test_build_delete_artist_params() {
    let mut app = App::test_default();
    app
      .data
      .lidarr_data
      .artists
      .set_items(vec![Artist::default()]);
    app.data.lidarr_data.delete_artist_files = true;
    app.data.lidarr_data.add_import_list_exclusion = true;
    let expected_delete_artist_params = DeleteArtistParams {
      id: 0,
      delete_files: true,
      add_import_list_exclusion: true,
    };

    let delete_artist_params = DeleteArtistHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveLidarrBlock::DeleteArtistPrompt,
      None,
    )
    .build_delete_artist_params();

    assert_eq!(delete_artist_params, expected_delete_artist_params);
    assert!(!app.data.lidarr_data.delete_artist_files);
    assert!(!app.data.lidarr_data.add_import_list_exclusion);
  }

  #[test]
  fn test_delete_artist_handler_not_ready_when_loading() {
    let mut app = App::test_default();
    app.is_loading = true;

    let handler = DeleteArtistHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveLidarrBlock::DeleteArtistPrompt,
      None,
    );

    assert!(!handler.is_ready());
  }

  #[test]
  fn test_delete_artist_handler_ready_when_not_loading() {
    let mut app = App::test_default();
    app.is_loading = false;

    let handler = DeleteArtistHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveLidarrBlock::DeleteArtistPrompt,
      None,
    );

    assert!(handler.is_ready());
  }
}
