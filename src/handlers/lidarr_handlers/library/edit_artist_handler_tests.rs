#[cfg(test)]
mod tests {
  use pretty_assertions::assert_eq;
  use strum::IntoEnumIterator;

  use crate::app::App;
  use crate::app::key_binding::DEFAULT_KEYBINDINGS;
  use crate::event::Key;
  use crate::handlers::KeyEventHandler;
  use crate::handlers::lidarr_handlers::library::edit_artist_handler::EditArtistHandler;
  use crate::models::servarr_data::lidarr::lidarr_data::{ActiveLidarrBlock, EDIT_ARTIST_BLOCKS};
  use crate::models::servarr_data::lidarr::modals::EditArtistModal;

  mod test_handle_scroll_up_and_down {
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    use crate::models::BlockSelectionState;
    use crate::models::servarr_data::lidarr::lidarr_data::EDIT_ARTIST_SELECTION_BLOCKS;

    use super::*;

    #[rstest]
    fn test_edit_artist_prompt_scroll(#[values(Key::Up, Key::Down)] key: Key) {
      let mut app = App::test_default();
      app.data.lidarr_data.edit_artist_modal = Some(EditArtistModal::default());
      app.data.lidarr_data.selected_block = BlockSelectionState::new(EDIT_ARTIST_SELECTION_BLOCKS);
      app.data.lidarr_data.selected_block.down();

      EditArtistHandler::new(key, &mut app, ActiveLidarrBlock::EditArtistPrompt, None).handle();

      if key == Key::Up {
        assert_eq!(
          app.data.lidarr_data.selected_block.get_active_block(),
          ActiveLidarrBlock::EditArtistToggleMonitored
        );
      } else {
        assert_eq!(
          app.data.lidarr_data.selected_block.get_active_block(),
          ActiveLidarrBlock::EditArtistSelectQualityProfile
        );
      }
    }

    #[rstest]
    fn test_edit_artist_prompt_scroll_no_op_when_not_ready(#[values(Key::Up, Key::Down)] key: Key) {
      let mut app = App::test_default();
      app.is_loading = true;
      app.data.lidarr_data.edit_artist_modal = Some(EditArtistModal::default());
      app.data.lidarr_data.selected_block = BlockSelectionState::new(EDIT_ARTIST_SELECTION_BLOCKS);
      app.data.lidarr_data.selected_block.down();

      EditArtistHandler::new(key, &mut app, ActiveLidarrBlock::EditArtistPrompt, None).handle();

      assert_eq!(
        app.data.lidarr_data.selected_block.get_active_block(),
        ActiveLidarrBlock::EditArtistSelectMonitorNewItems
      );
    }
  }

  mod test_handle_left_right_action {
    use rstest::rstest;

    use super::*;

    #[rstest]
    fn test_left_right_prompt_toggle(#[values(Key::Left, Key::Right)] key: Key) {
      let mut app = App::test_default();
      app.data.lidarr_data.edit_artist_modal = Some(EditArtistModal::default());
      app.push_navigation_stack(ActiveLidarrBlock::EditArtistPrompt.into());

      EditArtistHandler::new(key, &mut app, ActiveLidarrBlock::EditArtistPrompt, None).handle();

      assert!(app.data.lidarr_data.prompt_confirm);

      EditArtistHandler::new(key, &mut app, ActiveLidarrBlock::EditArtistPrompt, None).handle();

      assert!(!app.data.lidarr_data.prompt_confirm);
    }
  }

  mod test_handle_submit {
    use crate::models::BlockSelectionState;
    use crate::models::servarr_data::lidarr::lidarr_data::EDIT_ARTIST_SELECTION_BLOCKS;

    use super::*;
    use crate::assert_navigation_popped;

    const SUBMIT_KEY: Key = DEFAULT_KEYBINDINGS.submit.key;

    #[test]
    fn test_edit_artist_prompt_prompt_decline_submit() {
      let mut app = App::test_default();
      app.data.lidarr_data.edit_artist_modal = Some(EditArtistModal::default());
      app.push_navigation_stack(ActiveLidarrBlock::Artists.into());
      app.push_navigation_stack(ActiveLidarrBlock::EditArtistPrompt.into());
      app.data.lidarr_data.selected_block = BlockSelectionState::new(EDIT_ARTIST_SELECTION_BLOCKS);
      // Navigate to the confirm prompt (last selection block)
      for _ in 0..EDIT_ARTIST_SELECTION_BLOCKS.len() - 1 {
        app.data.lidarr_data.selected_block.down();
      }

      EditArtistHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveLidarrBlock::EditArtistPrompt,
        None,
      )
      .handle();

      assert!(app.data.lidarr_data.prompt_confirm_action.is_none());
      assert_navigation_popped!(&app, ActiveLidarrBlock::Artists.into());
    }
  }

  mod test_handle_esc {
    use super::*;
    use crate::assert_navigation_popped;

    const ESC_KEY: Key = DEFAULT_KEYBINDINGS.esc.key;

    #[test]
    fn test_edit_artist_prompt_esc() {
      let mut app = App::test_default();
      app.data.lidarr_data.edit_artist_modal = Some(EditArtistModal::default());
      app.push_navigation_stack(ActiveLidarrBlock::Artists.into());
      app.push_navigation_stack(ActiveLidarrBlock::EditArtistPrompt.into());
      app.data.lidarr_data.prompt_confirm = true;

      EditArtistHandler::new(ESC_KEY, &mut app, ActiveLidarrBlock::EditArtistPrompt, None).handle();

      assert_navigation_popped!(&app, ActiveLidarrBlock::Artists.into());
      assert!(app.data.lidarr_data.edit_artist_modal.is_none());
      assert!(!app.data.lidarr_data.prompt_confirm);
    }

    #[test]
    fn test_edit_artist_select_blocks_esc() {
      let mut app = App::test_default();
      app.data.lidarr_data.edit_artist_modal = Some(EditArtistModal::default());
      app.push_navigation_stack(ActiveLidarrBlock::Artists.into());
      app.push_navigation_stack(ActiveLidarrBlock::EditArtistPrompt.into());
      app.push_navigation_stack(ActiveLidarrBlock::EditArtistSelectQualityProfile.into());

      EditArtistHandler::new(
        ESC_KEY,
        &mut app,
        ActiveLidarrBlock::EditArtistSelectQualityProfile,
        None,
      )
      .handle();

      assert_navigation_popped!(&app, ActiveLidarrBlock::EditArtistPrompt.into());
    }
  }

  #[test]
  fn test_edit_artist_handler_accepts() {
    let mut edit_artist_handler_blocks = Vec::new();
    for block in ActiveLidarrBlock::iter() {
      if EditArtistHandler::accepts(block) {
        edit_artist_handler_blocks.push(block);
      }
    }

    assert_eq!(edit_artist_handler_blocks, EDIT_ARTIST_BLOCKS.to_vec());
  }

  #[test]
  fn test_edit_artist_handler_is_not_ready_when_loading() {
    let mut app = App::test_default();
    app.is_loading = true;
    app.data.lidarr_data.edit_artist_modal = Some(EditArtistModal::default());

    let handler = EditArtistHandler::new(
      Key::Esc,
      &mut app,
      ActiveLidarrBlock::EditArtistPrompt,
      None,
    );

    assert!(!handler.is_ready());
  }

  #[test]
  fn test_edit_artist_handler_is_not_ready_when_edit_artist_modal_is_none() {
    let mut app = App::test_default();
    app.data.lidarr_data.edit_artist_modal = None;

    let handler = EditArtistHandler::new(
      Key::Esc,
      &mut app,
      ActiveLidarrBlock::EditArtistPrompt,
      None,
    );

    assert!(!handler.is_ready());
  }

  #[test]
  fn test_edit_artist_handler_is_ready_when_not_loading_and_modal_is_some() {
    let mut app = App::test_default();
    app.data.lidarr_data.edit_artist_modal = Some(EditArtistModal::default());

    let handler = EditArtistHandler::new(
      Key::Esc,
      &mut app,
      ActiveLidarrBlock::EditArtistPrompt,
      None,
    );

    assert!(handler.is_ready());
  }
}
