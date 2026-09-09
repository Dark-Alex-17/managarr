#[cfg(test)]
mod tests {
  use pretty_assertions::assert_eq;
  use rstest::rstest;
  use strum::IntoEnumIterator;

  use crate::app::App;
  use crate::app::key_binding::DEFAULT_KEYBINDINGS;
  use crate::event::Key;
  use crate::handlers::KeyEventHandler;
  use crate::handlers::readarr_handlers::library::delete_author_handler::DeleteAuthorHandler;
  use crate::models::readarr_models::{Author, DeleteParams};
  use crate::models::servarr_data::readarr::readarr_data::{
    ActiveReadarrBlock, DELETE_AUTHOR_BLOCKS,
  };

  mod test_handle_scroll_up_and_down {
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    use crate::models::BlockSelectionState;
    use crate::models::servarr_data::readarr::readarr_data::DELETE_AUTHOR_SELECTION_BLOCKS;

    use super::*;

    #[rstest]
    fn test_delete_author_prompt_scroll(#[values(Key::Up, Key::Down)] key: Key) {
      let mut app = App::test_default();
      app.data.readarr_data.selected_block =
        BlockSelectionState::new(DELETE_AUTHOR_SELECTION_BLOCKS);
      app.data.readarr_data.selected_block.down();

      DeleteAuthorHandler::new(key, &mut app, ActiveReadarrBlock::DeleteAuthorPrompt, None)
        .handle();

      if key == Key::Up {
        assert_eq!(
          app.data.readarr_data.selected_block.get_active_block(),
          ActiveReadarrBlock::DeleteAuthorToggleDeleteFile
        );
      } else {
        assert_eq!(
          app.data.readarr_data.selected_block.get_active_block(),
          ActiveReadarrBlock::DeleteAuthorConfirmPrompt
        );
      }
    }

    #[rstest]
    fn test_delete_author_prompt_scroll_no_op_when_not_ready(
      #[values(Key::Up, Key::Down)] key: Key,
    ) {
      let mut app = App::test_default();
      app.is_loading = true;
      app.data.readarr_data.selected_block =
        BlockSelectionState::new(DELETE_AUTHOR_SELECTION_BLOCKS);
      app.data.readarr_data.selected_block.down();

      DeleteAuthorHandler::new(key, &mut app, ActiveReadarrBlock::DeleteAuthorPrompt, None)
        .handle();

      assert_eq!(
        app.data.readarr_data.selected_block.get_active_block(),
        ActiveReadarrBlock::DeleteAuthorToggleAddListExclusion
      );
    }
  }

  mod test_handle_left_right_action {
    use rstest::rstest;

    use super::*;

    #[rstest]
    fn test_left_right_prompt_toggle(#[values(Key::Left, Key::Right)] key: Key) {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveReadarrBlock::DeleteAuthorPrompt.into());

      DeleteAuthorHandler::new(key, &mut app, ActiveReadarrBlock::DeleteAuthorPrompt, None)
        .handle();

      assert!(app.data.readarr_data.prompt_confirm);

      DeleteAuthorHandler::new(key, &mut app, ActiveReadarrBlock::DeleteAuthorPrompt, None)
        .handle();

      assert!(!app.data.readarr_data.prompt_confirm);
    }
  }

  mod test_handle_submit {
    use pretty_assertions::assert_eq;

    use crate::assert_navigation_popped;
    use crate::models::BlockSelectionState;
    use crate::models::servarr_data::readarr::readarr_data::DELETE_AUTHOR_SELECTION_BLOCKS;
    use crate::network::readarr_network::ReadarrEvent;

    use super::*;

    const SUBMIT_KEY: Key = DEFAULT_KEYBINDINGS.submit.key;

    #[test]
    fn test_delete_author_prompt_prompt_decline_submit() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveReadarrBlock::Authors.into());
      app.push_navigation_stack(ActiveReadarrBlock::DeleteAuthorPrompt.into());
      app.data.readarr_data.selected_block =
        BlockSelectionState::new(DELETE_AUTHOR_SELECTION_BLOCKS);
      app
        .data
        .readarr_data
        .selected_block
        .set_index(0, DELETE_AUTHOR_SELECTION_BLOCKS.len() - 1);
      app.data.readarr_data.delete_files = true;
      app.data.readarr_data.add_import_list_exclusion = true;

      DeleteAuthorHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveReadarrBlock::DeleteAuthorPrompt,
        None,
      )
      .handle();

      assert_navigation_popped!(app, ActiveReadarrBlock::Authors.into());
      assert_none!(app.data.readarr_data.prompt_confirm_action);
      assert!(!app.data.readarr_data.prompt_confirm);
      assert!(!app.data.readarr_data.delete_files);
      assert!(!app.data.readarr_data.add_import_list_exclusion);
    }

    #[test]
    fn test_delete_author_confirm_prompt_prompt_confirmation_submit() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveReadarrBlock::Authors.into());
      app.push_navigation_stack(ActiveReadarrBlock::DeleteAuthorPrompt.into());
      app.data.readarr_data.prompt_confirm = true;
      app.data.readarr_data.delete_files = false;
      app.data.readarr_data.add_import_list_exclusion = true;
      app.data.readarr_data.authors.set_items(authors_vec());
      app.data.readarr_data.authors.select_index(Some(1));
      let expected_delete_author_params = DeleteParams {
        id: 42,
        delete_files: false,
        add_import_list_exclusion: true,
      };
      app.data.readarr_data.selected_block =
        BlockSelectionState::new(DELETE_AUTHOR_SELECTION_BLOCKS);
      app
        .data
        .readarr_data
        .selected_block
        .set_index(0, DELETE_AUTHOR_SELECTION_BLOCKS.len() - 1);

      DeleteAuthorHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveReadarrBlock::DeleteAuthorPrompt,
        None,
      )
      .handle();

      assert_navigation_popped!(app, ActiveReadarrBlock::Authors.into());
      assert_eq!(
        app.data.readarr_data.prompt_confirm_action,
        Some(ReadarrEvent::DeleteAuthor(expected_delete_author_params))
      );
      assert!(app.should_refresh);
      assert!(app.data.readarr_data.prompt_confirm);
      assert!(!app.data.readarr_data.delete_files);
      assert!(!app.data.readarr_data.add_import_list_exclusion);
    }

    #[test]
    fn test_delete_author_confirm_prompt_prompt_confirmation_submit_no_op_when_not_ready() {
      let mut app = App::test_default();
      app.is_loading = true;
      app.push_navigation_stack(ActiveReadarrBlock::Authors.into());
      app.push_navigation_stack(ActiveReadarrBlock::DeleteAuthorPrompt.into());
      app.data.readarr_data.prompt_confirm = true;
      app.data.readarr_data.delete_files = true;
      app.data.readarr_data.add_import_list_exclusion = false;

      DeleteAuthorHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveReadarrBlock::DeleteAuthorPrompt,
        None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveReadarrBlock::DeleteAuthorPrompt.into()
      );
      assert_none!(app.data.readarr_data.prompt_confirm_action);
      assert!(!app.should_refresh);
      assert!(app.data.readarr_data.prompt_confirm);
      assert!(app.data.readarr_data.delete_files);
      assert!(!app.data.readarr_data.add_import_list_exclusion);
    }

    #[test]
    fn test_delete_author_toggle_delete_files_submit() {
      let current_route = ActiveReadarrBlock::DeleteAuthorPrompt.into();
      let mut app = App::test_default();
      app.data.readarr_data.selected_block =
        BlockSelectionState::new(DELETE_AUTHOR_SELECTION_BLOCKS);
      app.push_navigation_stack(ActiveReadarrBlock::DeleteAuthorPrompt.into());

      DeleteAuthorHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveReadarrBlock::DeleteAuthorPrompt,
        None,
      )
      .handle();

      assert_eq!(app.get_current_route(), current_route);
      assert!(app.data.readarr_data.delete_files);
      assert!(!app.data.readarr_data.add_import_list_exclusion);

      DeleteAuthorHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveReadarrBlock::DeleteAuthorPrompt,
        None,
      )
      .handle();

      assert_eq!(app.get_current_route(), current_route);
      assert!(!app.data.readarr_data.delete_files);
      assert!(!app.data.readarr_data.add_import_list_exclusion);
    }

    #[test]
    fn test_delete_author_toggle_add_list_exclusion_submit() {
      let current_route = ActiveReadarrBlock::DeleteAuthorPrompt.into();
      let mut app = App::test_default();
      app.data.readarr_data.selected_block =
        BlockSelectionState::new(DELETE_AUTHOR_SELECTION_BLOCKS);
      app.data.readarr_data.selected_block.down();
      app.push_navigation_stack(ActiveReadarrBlock::DeleteAuthorPrompt.into());

      DeleteAuthorHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveReadarrBlock::DeleteAuthorPrompt,
        None,
      )
      .handle();

      assert_eq!(app.get_current_route(), current_route);
      assert!(app.data.readarr_data.add_import_list_exclusion);
      assert!(!app.data.readarr_data.delete_files);

      DeleteAuthorHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveReadarrBlock::DeleteAuthorPrompt,
        None,
      )
      .handle();

      assert_eq!(app.get_current_route(), current_route);
      assert!(!app.data.readarr_data.add_import_list_exclusion);
      assert!(!app.data.readarr_data.delete_files);
    }
  }

  mod test_handle_esc {
    use rstest::rstest;

    use crate::assert_navigation_popped;

    use super::*;

    const ESC_KEY: Key = DEFAULT_KEYBINDINGS.esc.key;

    #[rstest]
    fn test_delete_author_prompt_esc(#[values(true, false)] is_loading: bool) {
      let mut app = App::test_default();
      app.is_loading = is_loading;
      app.push_navigation_stack(ActiveReadarrBlock::Authors.into());
      app.push_navigation_stack(ActiveReadarrBlock::DeleteAuthorPrompt.into());
      app.data.readarr_data.prompt_confirm = true;
      app.data.readarr_data.delete_files = true;
      app.data.readarr_data.add_import_list_exclusion = true;

      DeleteAuthorHandler::new(
        ESC_KEY,
        &mut app,
        ActiveReadarrBlock::DeleteAuthorPrompt,
        None,
      )
      .handle();

      assert_navigation_popped!(app, ActiveReadarrBlock::Authors.into());
      assert!(!app.data.readarr_data.prompt_confirm);
      assert!(!app.data.readarr_data.delete_files);
      assert!(!app.data.readarr_data.add_import_list_exclusion);
    }
  }

  mod test_handle_key_char {
    use pretty_assertions::assert_eq;

    use crate::assert_navigation_popped;
    use crate::models::BlockSelectionState;
    use crate::models::servarr_data::readarr::readarr_data::DELETE_AUTHOR_SELECTION_BLOCKS;
    use crate::network::readarr_network::ReadarrEvent;

    use super::*;

    const CONFIRM_KEY: Key = DEFAULT_KEYBINDINGS.confirm.key;

    #[test]
    fn test_delete_author_confirm_prompt_prompt_confirm() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveReadarrBlock::Authors.into());
      app.push_navigation_stack(ActiveReadarrBlock::DeleteAuthorPrompt.into());
      app.data.readarr_data.delete_files = true;
      app.data.readarr_data.add_import_list_exclusion = false;
      app.data.readarr_data.authors.set_items(authors_vec());
      app.data.readarr_data.authors.select_index(Some(1));
      let expected_delete_author_params = DeleteParams {
        id: 42,
        delete_files: true,
        add_import_list_exclusion: false,
      };
      app.data.readarr_data.selected_block =
        BlockSelectionState::new(DELETE_AUTHOR_SELECTION_BLOCKS);
      app
        .data
        .readarr_data
        .selected_block
        .set_index(0, DELETE_AUTHOR_SELECTION_BLOCKS.len() - 1);

      DeleteAuthorHandler::new(
        CONFIRM_KEY,
        &mut app,
        ActiveReadarrBlock::DeleteAuthorPrompt,
        None,
      )
      .handle();

      assert_navigation_popped!(app, ActiveReadarrBlock::Authors.into());
      assert_eq!(
        app.data.readarr_data.prompt_confirm_action,
        Some(ReadarrEvent::DeleteAuthor(expected_delete_author_params))
      );
      assert!(app.should_refresh);
      assert!(app.data.readarr_data.prompt_confirm);
      assert!(!app.data.readarr_data.delete_files);
      assert!(!app.data.readarr_data.add_import_list_exclusion);
    }

    #[test]
    fn test_delete_author_confirm_prompt_prompt_confirm_no_op_when_not_ready() {
      let mut app = App::test_default();
      app.is_loading = true;
      app.push_navigation_stack(ActiveReadarrBlock::Authors.into());
      app.push_navigation_stack(ActiveReadarrBlock::DeleteAuthorPrompt.into());
      app.data.readarr_data.delete_files = true;
      app.data.readarr_data.add_import_list_exclusion = false;
      app.data.readarr_data.authors.set_items(authors_vec());
      app.data.readarr_data.authors.select_index(Some(1));
      app.data.readarr_data.selected_block =
        BlockSelectionState::new(DELETE_AUTHOR_SELECTION_BLOCKS);
      app
        .data
        .readarr_data
        .selected_block
        .set_index(0, DELETE_AUTHOR_SELECTION_BLOCKS.len() - 1);

      DeleteAuthorHandler::new(
        CONFIRM_KEY,
        &mut app,
        ActiveReadarrBlock::DeleteAuthorPrompt,
        None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveReadarrBlock::DeleteAuthorPrompt.into()
      );
      assert_none!(app.data.readarr_data.prompt_confirm_action);
      assert!(!app.should_refresh);
      assert!(!app.data.readarr_data.prompt_confirm);
      assert!(app.data.readarr_data.delete_files);
    }
  }

  #[test]
  fn test_delete_author_handler_accepts() {
    ActiveReadarrBlock::iter().for_each(|active_readarr_block| {
      if DELETE_AUTHOR_BLOCKS.contains(&active_readarr_block) {
        assert!(DeleteAuthorHandler::accepts(active_readarr_block));
      } else {
        assert!(!DeleteAuthorHandler::accepts(active_readarr_block));
      }
    });
  }

  #[rstest]
  fn test_delete_author_handler_ignore_special_keys(
    #[values(true, false)] ignore_special_keys_for_textbox_input: bool,
  ) {
    let mut app = App::test_default();
    app.ignore_special_keys_for_textbox_input = ignore_special_keys_for_textbox_input;
    let handler = DeleteAuthorHandler::new(
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
  fn test_build_delete_author_params() {
    let mut app = App::test_default();
    app.data.readarr_data.authors.set_items(authors_vec());
    app.data.readarr_data.authors.select_index(Some(1));
    app.data.readarr_data.delete_files = true;
    app.data.readarr_data.add_import_list_exclusion = false;
    let expected_delete_author_params = DeleteParams {
      id: 42,
      delete_files: true,
      add_import_list_exclusion: false,
    };

    let delete_author_params = DeleteAuthorHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveReadarrBlock::DeleteAuthorPrompt,
      None,
    )
    .build_delete_author_params();

    assert_eq!(delete_author_params, expected_delete_author_params);
    assert!(!app.data.readarr_data.delete_files);
    assert!(!app.data.readarr_data.add_import_list_exclusion);
  }

  #[test]
  fn test_delete_author_handler_not_ready_when_loading() {
    let mut app = App::test_default();
    app.is_loading = true;

    let handler = DeleteAuthorHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveReadarrBlock::DeleteAuthorPrompt,
      None,
    );

    assert!(!handler.is_ready());
  }

  #[test]
  fn test_delete_author_handler_ready_when_not_loading() {
    let mut app = App::test_default();
    app.is_loading = false;

    let handler = DeleteAuthorHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveReadarrBlock::DeleteAuthorPrompt,
      None,
    );

    assert!(handler.is_ready());
  }

  fn author() -> Author {
    Author {
      id: 42,
      ..Author::default()
    }
  }

  fn authors_vec() -> Vec<Author> {
    vec![
      Author {
        id: 999,
        ..Author::default()
      },
      author(),
    ]
  }
}
