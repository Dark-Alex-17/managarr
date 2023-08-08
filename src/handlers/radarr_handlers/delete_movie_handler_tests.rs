#[cfg(test)]
mod tests {
  use crate::app::key_binding::DEFAULT_KEYBINDINGS;
  use crate::app::radarr::ActiveRadarrBlock;
  use crate::app::App;
  use crate::event::Key;
  use crate::handlers::radarr_handlers::delete_movie_handler::DeleteMovieHandler;
  use crate::handlers::KeyEventHandler;

  mod test_handle_scroll_up_and_down {
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    use crate::app::radarr::DELETE_MOVIE_SELECTION_BLOCKS;
    use crate::models::BlockSelectionState;

    use super::*;

    #[rstest]
    fn test_delete_movie_prompt_scroll(#[values(Key::Up, Key::Down)] key: Key) {
      let mut app = App::default();
      app.data.radarr_data.selected_block =
        BlockSelectionState::new(&DELETE_MOVIE_SELECTION_BLOCKS);
      app.data.radarr_data.selected_block.next();

      DeleteMovieHandler::with(&key, &mut app, &ActiveRadarrBlock::DeleteMoviePrompt, &None)
        .handle();

      if key == Key::Up {
        assert_eq!(
          app.data.radarr_data.selected_block.get_active_block(),
          &ActiveRadarrBlock::DeleteMovieToggleDeleteFile
        );
      } else {
        assert_eq!(
          app.data.radarr_data.selected_block.get_active_block(),
          &ActiveRadarrBlock::DeleteMovieConfirmPrompt
        );
      }
    }
  }

  mod test_handle_left_right_action {
    use rstest::rstest;

    use super::*;

    #[rstest]
    fn test_left_right_prompt_toggle(#[values(Key::Left, Key::Right)] key: Key) {
      let mut app = App::default();

      DeleteMovieHandler::with(&key, &mut app, &ActiveRadarrBlock::DeleteMoviePrompt, &None)
        .handle();

      assert!(app.data.radarr_data.prompt_confirm);

      DeleteMovieHandler::with(&key, &mut app, &ActiveRadarrBlock::DeleteMoviePrompt, &None)
        .handle();

      assert!(!app.data.radarr_data.prompt_confirm);
    }
  }

  mod test_handle_submit {
    use pretty_assertions::assert_eq;

    use crate::app::radarr::DELETE_MOVIE_SELECTION_BLOCKS;
    use crate::models::BlockSelectionState;
    use crate::network::radarr_network::RadarrEvent;

    use super::*;

    const SUBMIT_KEY: Key = DEFAULT_KEYBINDINGS.submit.key;

    #[test]
    fn test_delete_movie_prompt_prompt_decline_submit() {
      let mut app = App::default();
      app.push_navigation_stack(ActiveRadarrBlock::Movies.into());
      app.push_navigation_stack(ActiveRadarrBlock::DeleteMoviePrompt.into());
      app.data.radarr_data.selected_block =
        BlockSelectionState::new(&DELETE_MOVIE_SELECTION_BLOCKS);
      app
        .data
        .radarr_data
        .selected_block
        .set_index(DELETE_MOVIE_SELECTION_BLOCKS.len() - 1);
      app.data.radarr_data.delete_movie_files = true;
      app.data.radarr_data.add_list_exclusion = true;

      DeleteMovieHandler::with(
        &SUBMIT_KEY,
        &mut app,
        &ActiveRadarrBlock::DeleteMoviePrompt,
        &None,
      )
      .handle();

      assert_eq!(app.get_current_route(), &ActiveRadarrBlock::Movies.into());
      assert_eq!(app.data.radarr_data.prompt_confirm_action, None);
      assert!(!app.data.radarr_data.prompt_confirm);
      assert!(!app.data.radarr_data.delete_movie_files);
      assert!(!app.data.radarr_data.add_list_exclusion);
    }

    #[test]
    fn test_delete_movie_confirm_prompt_prompt_confirmation_submit() {
      let mut app = App::default();
      app.push_navigation_stack(ActiveRadarrBlock::Movies.into());
      app.push_navigation_stack(ActiveRadarrBlock::DeleteMoviePrompt.into());
      app.data.radarr_data.prompt_confirm = true;
      app.data.radarr_data.delete_movie_files = true;
      app.data.radarr_data.add_list_exclusion = true;
      app.data.radarr_data.selected_block =
        BlockSelectionState::new(&DELETE_MOVIE_SELECTION_BLOCKS);
      app
        .data
        .radarr_data
        .selected_block
        .set_index(DELETE_MOVIE_SELECTION_BLOCKS.len() - 1);

      DeleteMovieHandler::with(
        &SUBMIT_KEY,
        &mut app,
        &ActiveRadarrBlock::DeleteMoviePrompt,
        &None,
      )
      .handle();

      assert_eq!(app.get_current_route(), &ActiveRadarrBlock::Movies.into());
      assert_eq!(
        app.data.radarr_data.prompt_confirm_action,
        Some(RadarrEvent::DeleteMovie)
      );
      assert!(app.should_refresh);
      assert!(app.data.radarr_data.prompt_confirm);
      assert!(app.data.radarr_data.delete_movie_files);
      assert!(app.data.radarr_data.add_list_exclusion);
    }

    #[test]
    fn test_delete_movie_toggle_delete_files_submit() {
      let current_route = ActiveRadarrBlock::DeleteMoviePrompt.into();
      let mut app = App::default();
      app.data.radarr_data.selected_block =
        BlockSelectionState::new(&DELETE_MOVIE_SELECTION_BLOCKS);
      app.push_navigation_stack(ActiveRadarrBlock::DeleteMoviePrompt.into());

      DeleteMovieHandler::with(
        &SUBMIT_KEY,
        &mut app,
        &ActiveRadarrBlock::DeleteMoviePrompt,
        &None,
      )
      .handle();

      assert_eq!(app.get_current_route(), &current_route);
      assert_eq!(app.data.radarr_data.delete_movie_files, true);

      DeleteMovieHandler::with(
        &SUBMIT_KEY,
        &mut app,
        &ActiveRadarrBlock::DeleteMoviePrompt,
        &None,
      )
      .handle();

      assert_eq!(app.get_current_route(), &current_route);
      assert_eq!(app.data.radarr_data.delete_movie_files, false);
    }
  }

  mod test_handle_esc {
    use super::*;

    const ESC_KEY: Key = DEFAULT_KEYBINDINGS.esc.key;

    #[test]
    fn test_delete_movie_prompt_esc() {
      let mut app = App::default();
      app.push_navigation_stack(ActiveRadarrBlock::Movies.into());
      app.push_navigation_stack(ActiveRadarrBlock::DeleteMoviePrompt.into());
      app.data.radarr_data.prompt_confirm = true;
      app.data.radarr_data.delete_movie_files = true;
      app.data.radarr_data.add_list_exclusion = true;

      DeleteMovieHandler::with(
        &ESC_KEY,
        &mut app,
        &ActiveRadarrBlock::DeleteMoviePrompt,
        &None,
      )
      .handle();

      assert_eq!(app.get_current_route(), &ActiveRadarrBlock::Movies.into());
      assert!(!app.data.radarr_data.prompt_confirm);
      assert!(!app.data.radarr_data.delete_movie_files);
      assert!(!app.data.radarr_data.add_list_exclusion);
    }
  }
}
