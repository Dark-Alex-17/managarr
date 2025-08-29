#[cfg(test)]
mod tests {
  use pretty_assertions::assert_eq;
  use rstest::rstest;
  use strum::IntoEnumIterator;

  use crate::app::key_binding::DEFAULT_KEYBINDINGS;
  use crate::app::App;
  use crate::event::Key;
  use crate::handlers::radarr_handlers::library::delete_movie_handler::DeleteMovieHandler;
  use crate::handlers::radarr_handlers::radarr_handler_test_utils::utils::movie;
  use crate::handlers::KeyEventHandler;
  use crate::models::radarr_models::DeleteMovieParams;
  use crate::models::servarr_data::radarr::radarr_data::{ActiveRadarrBlock, DELETE_MOVIE_BLOCKS};

  mod test_handle_scroll_up_and_down {
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    use crate::models::servarr_data::radarr::radarr_data::DELETE_MOVIE_SELECTION_BLOCKS;
    use crate::models::BlockSelectionState;

    use super::*;

    #[rstest]
    fn test_delete_movie_prompt_scroll(#[values(Key::Up, Key::Down)] key: Key) {
      let mut app = App::test_default();
      app.data.radarr_data.selected_block = BlockSelectionState::new(DELETE_MOVIE_SELECTION_BLOCKS);
      app.data.radarr_data.selected_block.down();

      DeleteMovieHandler::new(key, &mut app, ActiveRadarrBlock::DeleteMoviePrompt, None).handle();

      if key == Key::Up {
        assert_eq!(
          app.data.radarr_data.selected_block.get_active_block(),
          ActiveRadarrBlock::DeleteMovieToggleDeleteFile
        );
      } else {
        assert_eq!(
          app.data.radarr_data.selected_block.get_active_block(),
          ActiveRadarrBlock::DeleteMovieConfirmPrompt
        );
      }
    }

    #[rstest]
    fn test_delete_movie_prompt_scroll_no_op_when_not_ready(
      #[values(Key::Up, Key::Down)] key: Key,
    ) {
      let mut app = App::test_default();
      app.is_loading = true;
      app.data.radarr_data.selected_block = BlockSelectionState::new(DELETE_MOVIE_SELECTION_BLOCKS);
      app.data.radarr_data.selected_block.down();

      DeleteMovieHandler::new(key, &mut app, ActiveRadarrBlock::DeleteMoviePrompt, None).handle();

      assert_eq!(
        app.data.radarr_data.selected_block.get_active_block(),
        ActiveRadarrBlock::DeleteMovieToggleAddListExclusion
      );
    }
  }

  mod test_handle_left_right_action {
    use rstest::rstest;

    use super::*;

    #[rstest]
    fn test_left_right_prompt_toggle(#[values(Key::Left, Key::Right)] key: Key) {
      let mut app = App::test_default();

      DeleteMovieHandler::new(key, &mut app, ActiveRadarrBlock::DeleteMoviePrompt, None).handle();

      assert!(app.data.radarr_data.prompt_confirm);

      DeleteMovieHandler::new(key, &mut app, ActiveRadarrBlock::DeleteMoviePrompt, None).handle();

      assert!(!app.data.radarr_data.prompt_confirm);
    }
  }

  mod test_handle_submit {
    use pretty_assertions::assert_eq;

    use crate::models::servarr_data::radarr::radarr_data::DELETE_MOVIE_SELECTION_BLOCKS;
    use crate::models::BlockSelectionState;
    use crate::network::radarr_network::RadarrEvent;

    use super::*;

    const SUBMIT_KEY: Key = DEFAULT_KEYBINDINGS.submit.key;

    #[test]
    fn test_delete_movie_prompt_prompt_decline_submit() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveRadarrBlock::Movies.into());
      app.push_navigation_stack(ActiveRadarrBlock::DeleteMoviePrompt.into());
      app.data.radarr_data.selected_block = BlockSelectionState::new(DELETE_MOVIE_SELECTION_BLOCKS);
      app
        .data
        .radarr_data
        .selected_block
        .set_index(0, DELETE_MOVIE_SELECTION_BLOCKS.len() - 1);
      app.data.radarr_data.delete_movie_files = true;
      app.data.radarr_data.add_list_exclusion = true;

      DeleteMovieHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveRadarrBlock::DeleteMoviePrompt,
        None,
      )
      .handle();

      assert_eq!(app.get_current_route(), ActiveRadarrBlock::Movies.into());
      assert_eq!(app.data.radarr_data.prompt_confirm_action, None);
      assert!(!app.data.radarr_data.prompt_confirm);
      assert!(!app.data.radarr_data.delete_movie_files);
      assert!(!app.data.radarr_data.add_list_exclusion);
    }

    #[test]
    fn test_delete_movie_confirm_prompt_prompt_confirmation_submit() {
      let mut app = App::test_default();
      let expected_delete_movie_params = DeleteMovieParams {
        id: 1,
        delete_movie_files: true,
        add_list_exclusion: true,
      };
      app.push_navigation_stack(ActiveRadarrBlock::Movies.into());
      app.push_navigation_stack(ActiveRadarrBlock::DeleteMoviePrompt.into());
      app.data.radarr_data.movies.set_items(vec![movie()]);
      app.data.radarr_data.prompt_confirm = true;
      app.data.radarr_data.delete_movie_files = true;
      app.data.radarr_data.add_list_exclusion = true;
      app.data.radarr_data.selected_block = BlockSelectionState::new(DELETE_MOVIE_SELECTION_BLOCKS);
      app
        .data
        .radarr_data
        .selected_block
        .set_index(0, DELETE_MOVIE_SELECTION_BLOCKS.len() - 1);

      DeleteMovieHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveRadarrBlock::DeleteMoviePrompt,
        None,
      )
      .handle();

      assert_eq!(app.get_current_route(), ActiveRadarrBlock::Movies.into());
      assert_eq!(
        app.data.radarr_data.prompt_confirm_action,
        Some(RadarrEvent::DeleteMovie(expected_delete_movie_params))
      );
      assert!(app.should_refresh);
      assert!(app.data.radarr_data.prompt_confirm);
      assert!(!app.data.radarr_data.delete_movie_files);
      assert!(!app.data.radarr_data.add_list_exclusion);
    }

    #[test]
    fn test_delete_movie_confirm_prompt_prompt_confirmation_submit_no_op_when_not_ready() {
      let mut app = App::test_default();
      app.is_loading = true;
      app.push_navigation_stack(ActiveRadarrBlock::Movies.into());
      app.push_navigation_stack(ActiveRadarrBlock::DeleteMoviePrompt.into());
      app.data.radarr_data.prompt_confirm = true;
      app.data.radarr_data.delete_movie_files = true;
      app.data.radarr_data.add_list_exclusion = true;

      DeleteMovieHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveRadarrBlock::DeleteMoviePrompt,
        None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveRadarrBlock::DeleteMoviePrompt.into()
      );
      assert_eq!(app.data.radarr_data.prompt_confirm_action, None);
      assert!(!app.should_refresh);
      assert!(app.data.radarr_data.prompt_confirm);
      assert!(app.data.radarr_data.delete_movie_files);
      assert!(app.data.radarr_data.add_list_exclusion);
    }

    #[test]
    fn test_delete_movie_toggle_delete_files_submit() {
      let current_route = ActiveRadarrBlock::DeleteMoviePrompt.into();
      let mut app = App::test_default();
      app.data.radarr_data.selected_block = BlockSelectionState::new(DELETE_MOVIE_SELECTION_BLOCKS);
      app.push_navigation_stack(ActiveRadarrBlock::DeleteMoviePrompt.into());

      DeleteMovieHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveRadarrBlock::DeleteMoviePrompt,
        None,
      )
      .handle();

      assert_eq!(app.get_current_route(), current_route);
      assert_eq!(app.data.radarr_data.delete_movie_files, true);

      DeleteMovieHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveRadarrBlock::DeleteMoviePrompt,
        None,
      )
      .handle();

      assert_eq!(app.get_current_route(), current_route);
      assert_eq!(app.data.radarr_data.delete_movie_files, false);
    }
  }

  mod test_handle_esc {
    use super::*;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    const ESC_KEY: Key = DEFAULT_KEYBINDINGS.esc.key;

    #[rstest]
    fn test_delete_movie_prompt_esc(#[values(true, false)] is_ready: bool) {
      let mut app = App::test_default();
      app.is_loading = is_ready;
      app.push_navigation_stack(ActiveRadarrBlock::Movies.into());
      app.push_navigation_stack(ActiveRadarrBlock::DeleteMoviePrompt.into());
      app.data.radarr_data.prompt_confirm = true;
      app.data.radarr_data.delete_movie_files = true;
      app.data.radarr_data.add_list_exclusion = true;

      DeleteMovieHandler::new(
        ESC_KEY,
        &mut app,
        ActiveRadarrBlock::DeleteMoviePrompt,
        None,
      )
      .handle();

      assert_eq!(app.get_current_route(), ActiveRadarrBlock::Movies.into());
      assert!(!app.data.radarr_data.prompt_confirm);
      assert!(!app.data.radarr_data.delete_movie_files);
      assert!(!app.data.radarr_data.add_list_exclusion);
    }
  }

  mod test_handle_key_char {
    use crate::{
      models::{
        servarr_data::radarr::radarr_data::DELETE_MOVIE_SELECTION_BLOCKS, BlockSelectionState,
      },
      network::radarr_network::RadarrEvent,
    };
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_delete_movie_confirm_prompt_prompt_confirm() {
      let mut app = App::test_default();
      let expected_delete_movie_params = DeleteMovieParams {
        id: 1,
        delete_movie_files: true,
        add_list_exclusion: true,
      };
      app.push_navigation_stack(ActiveRadarrBlock::Movies.into());
      app.push_navigation_stack(ActiveRadarrBlock::DeleteMoviePrompt.into());
      app.data.radarr_data.movies.set_items(vec![movie()]);
      app.data.radarr_data.delete_movie_files = true;
      app.data.radarr_data.add_list_exclusion = true;
      app.data.radarr_data.selected_block = BlockSelectionState::new(DELETE_MOVIE_SELECTION_BLOCKS);
      app
        .data
        .radarr_data
        .selected_block
        .set_index(0, DELETE_MOVIE_SELECTION_BLOCKS.len() - 1);

      DeleteMovieHandler::new(
        DEFAULT_KEYBINDINGS.confirm.key,
        &mut app,
        ActiveRadarrBlock::DeleteMoviePrompt,
        None,
      )
      .handle();

      assert_eq!(app.get_current_route(), ActiveRadarrBlock::Movies.into());
      assert_eq!(
        app.data.radarr_data.prompt_confirm_action,
        Some(RadarrEvent::DeleteMovie(expected_delete_movie_params))
      );
      assert!(app.should_refresh);
      assert!(app.data.radarr_data.prompt_confirm);
      assert!(!app.data.radarr_data.delete_movie_files);
      assert!(!app.data.radarr_data.add_list_exclusion);
    }
  }

  #[test]
  fn test_delete_movie_handler_accepts() {
    ActiveRadarrBlock::iter().for_each(|active_radarr_block| {
      if DELETE_MOVIE_BLOCKS.contains(&active_radarr_block) {
        assert!(DeleteMovieHandler::accepts(active_radarr_block));
      } else {
        assert!(!DeleteMovieHandler::accepts(active_radarr_block));
      }
    });
  }

  #[rstest]
  fn test_delete_movie_handler_ignore_special_keys(
    #[values(true, false)] ignore_special_keys_for_textbox_input: bool,
  ) {
    let mut app = App::test_default();
    app.ignore_special_keys_for_textbox_input = ignore_special_keys_for_textbox_input;
    let handler = DeleteMovieHandler::new(
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
  fn test_build_delete_movie_params() {
    let mut app = App::test_default();
    app.data.radarr_data.movies.set_items(vec![movie()]);
    app.data.radarr_data.delete_movie_files = true;
    app.data.radarr_data.add_list_exclusion = true;
    let expected_delete_movie_params = DeleteMovieParams {
      id: 1,
      delete_movie_files: true,
      add_list_exclusion: true,
    };

    let delete_movie_params = DeleteMovieHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveRadarrBlock::DeleteMoviePrompt,
      None,
    )
    .build_delete_movie_params();

    assert_eq!(delete_movie_params, expected_delete_movie_params);
    assert!(!app.data.radarr_data.delete_movie_files);
    assert!(!app.data.radarr_data.add_list_exclusion);
  }

  #[test]
  fn test_delete_movie_handler_not_ready_when_loading() {
    let mut app = App::test_default();
    app.is_loading = true;

    let handler = DeleteMovieHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveRadarrBlock::DeleteMoviePrompt,
      None,
    );

    assert!(!handler.is_ready());
  }

  #[test]
  fn test_delete_movie_handler_ready_when_not_loading() {
    let mut app = App::test_default();
    app.is_loading = false;

    let handler = DeleteMovieHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveRadarrBlock::DeleteMoviePrompt,
      None,
    );

    assert!(handler.is_ready());
  }
}
