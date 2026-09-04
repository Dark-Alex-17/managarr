#[cfg(test)]
mod tests {
  use pretty_assertions::assert_eq;
  use rstest::rstest;
  use strum::IntoEnumIterator;

  use crate::app::App;
  use crate::app::key_binding::DEFAULT_KEYBINDINGS;
  use crate::assert_modal_present;
  use crate::assert_navigation_pushed;
  use crate::event::Key;
  use crate::handlers::KeyEventHandler;
  use crate::handlers::readarr_handlers::root_folders::RootFoldersHandler;
  use crate::models::servarr_data::readarr::readarr_data::{
    ADD_ROOT_FOLDER_BLOCKS, ActiveReadarrBlock, ROOT_FOLDERS_BLOCKS,
  };
  use crate::models::servarr_models::RootFolder;

  mod test_handle_delete {
    use pretty_assertions::assert_eq;

    use super::*;

    const DELETE_KEY: Key = DEFAULT_KEYBINDINGS.delete.key;

    #[test]
    fn test_delete_root_folder_prompt() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveReadarrBlock::RootFolders.into());
      app
        .data
        .readarr_data
        .root_folders
        .set_items(root_folders_vec());

      RootFoldersHandler::new(DELETE_KEY, &mut app, ActiveReadarrBlock::RootFolders, None).handle();

      assert_navigation_pushed!(app, ActiveReadarrBlock::DeleteRootFolderPrompt.into());
    }

    #[test]
    fn test_delete_root_folder_prompt_no_op_when_not_ready() {
      let mut app = App::test_default();
      app.is_loading = true;
      app.push_navigation_stack(ActiveReadarrBlock::RootFolders.into());
      app
        .data
        .readarr_data
        .root_folders
        .set_items(root_folders_vec());

      RootFoldersHandler::new(DELETE_KEY, &mut app, ActiveReadarrBlock::RootFolders, None).handle();

      assert_eq!(
        app.get_current_route(),
        ActiveReadarrBlock::RootFolders.into()
      );
    }
  }

  mod test_handle_left_right_action {
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    use super::*;
    use crate::assert_navigation_pushed;

    #[rstest]
    fn test_root_folders_tab_left(#[values(true, false)] is_ready: bool) {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveReadarrBlock::RootFolders.into());
      app.is_loading = is_ready;
      app.data.readarr_data.main_tabs.set_index(4);

      RootFoldersHandler::new(
        DEFAULT_KEYBINDINGS.left.key,
        &mut app,
        ActiveReadarrBlock::RootFolders,
        None,
      )
      .handle();

      assert_eq!(
        app.data.readarr_data.main_tabs.get_active_route(),
        ActiveReadarrBlock::History.into()
      );
      assert_navigation_pushed!(app, ActiveReadarrBlock::History.into());
    }

    #[rstest]
    fn test_root_folders_tab_right(#[values(true, false)] is_ready: bool) {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveReadarrBlock::RootFolders.into());
      app.is_loading = is_ready;
      app.data.readarr_data.main_tabs.set_index(4);

      RootFoldersHandler::new(
        DEFAULT_KEYBINDINGS.right.key,
        &mut app,
        ActiveReadarrBlock::RootFolders,
        None,
      )
      .handle();

      assert_eq!(
        app.data.readarr_data.main_tabs.get_active_route(),
        ActiveReadarrBlock::Indexers.into()
      );
      assert_navigation_pushed!(app, ActiveReadarrBlock::Indexers.into());
    }

    #[rstest]
    fn test_left_right_delete_root_folder_prompt_toggle(
      #[values(DEFAULT_KEYBINDINGS.left.key, DEFAULT_KEYBINDINGS.right.key)] key: Key,
    ) {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveReadarrBlock::RootFolders.into());

      RootFoldersHandler::new(
        key,
        &mut app,
        ActiveReadarrBlock::DeleteRootFolderPrompt,
        None,
      )
      .handle();

      assert!(app.data.readarr_data.prompt_confirm);

      RootFoldersHandler::new(
        key,
        &mut app,
        ActiveReadarrBlock::DeleteRootFolderPrompt,
        None,
      )
      .handle();

      assert!(!app.data.readarr_data.prompt_confirm);
    }
  }

  mod test_handle_submit {
    use pretty_assertions::assert_eq;

    use super::*;
    use crate::assert_navigation_popped;
    use crate::network::readarr_network::ReadarrEvent;

    const SUBMIT_KEY: Key = DEFAULT_KEYBINDINGS.submit.key;

    #[test]
    fn test_delete_root_folder_prompt_confirm_submit() {
      let mut app = App::test_default();
      app
        .data
        .readarr_data
        .root_folders
        .set_items(root_folders_vec());
      app.data.readarr_data.root_folders.select_index(Some(1));
      app.data.readarr_data.prompt_confirm = true;
      app.push_navigation_stack(ActiveReadarrBlock::RootFolders.into());
      app.push_navigation_stack(ActiveReadarrBlock::DeleteRootFolderPrompt.into());

      RootFoldersHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveReadarrBlock::DeleteRootFolderPrompt,
        None,
      )
      .handle();

      assert!(app.data.readarr_data.prompt_confirm);
      assert_some_eq_x!(
        &app.data.readarr_data.prompt_confirm_action,
        &ReadarrEvent::DeleteRootFolder(2)
      );
      assert_navigation_popped!(app, ActiveReadarrBlock::RootFolders.into());
    }

    #[test]
    fn test_delete_root_folder_prompt_decline_submit() {
      let mut app = App::test_default();
      app
        .data
        .readarr_data
        .root_folders
        .set_items(root_folders_vec());
      app.push_navigation_stack(ActiveReadarrBlock::RootFolders.into());
      app.push_navigation_stack(ActiveReadarrBlock::DeleteRootFolderPrompt.into());

      RootFoldersHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveReadarrBlock::DeleteRootFolderPrompt,
        None,
      )
      .handle();

      assert!(!app.data.readarr_data.prompt_confirm);
      assert_none!(app.data.readarr_data.prompt_confirm_action);
      assert_navigation_popped!(app, ActiveReadarrBlock::RootFolders.into());
    }

    #[test]
    fn test_delete_root_folder_prompt_submit_no_op_when_not_ready() {
      let mut app = App::test_default();
      app.is_loading = true;
      app
        .data
        .readarr_data
        .root_folders
        .set_items(root_folders_vec());
      app.data.readarr_data.prompt_confirm = true;
      app.push_navigation_stack(ActiveReadarrBlock::RootFolders.into());
      app.push_navigation_stack(ActiveReadarrBlock::DeleteRootFolderPrompt.into());

      RootFoldersHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveReadarrBlock::DeleteRootFolderPrompt,
        None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveReadarrBlock::DeleteRootFolderPrompt.into()
      );
      assert_none!(app.data.readarr_data.prompt_confirm_action);
    }
  }

  mod test_handle_esc {
    use super::*;
    use crate::assert_navigation_popped;
    use rstest::rstest;

    const ESC_KEY: Key = DEFAULT_KEYBINDINGS.esc.key;

    #[test]
    fn test_delete_root_folder_prompt_block_esc() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveReadarrBlock::RootFolders.into());
      app.push_navigation_stack(ActiveReadarrBlock::DeleteRootFolderPrompt.into());
      app.data.readarr_data.prompt_confirm = true;

      RootFoldersHandler::new(
        ESC_KEY,
        &mut app,
        ActiveReadarrBlock::DeleteRootFolderPrompt,
        None,
      )
      .handle();

      assert_navigation_popped!(app, ActiveReadarrBlock::RootFolders.into());
      assert!(!app.data.readarr_data.prompt_confirm);
    }

    #[rstest]
    fn test_default_esc(#[values(true, false)] is_ready: bool) {
      let mut app = App::test_default();
      app.is_loading = is_ready;
      app.error = "test error".to_owned().into();
      app.push_navigation_stack(ActiveReadarrBlock::RootFolders.into());
      app.push_navigation_stack(ActiveReadarrBlock::RootFolders.into());

      RootFoldersHandler::new(ESC_KEY, &mut app, ActiveReadarrBlock::RootFolders, None).handle();

      assert_navigation_popped!(app, ActiveReadarrBlock::RootFolders.into());
      assert_is_empty!(app.error.text);
    }
  }

  mod test_handle_key_char {
    use pretty_assertions::assert_eq;

    use super::*;
    use crate::assert_navigation_popped;
    use crate::network::readarr_network::ReadarrEvent;

    #[test]
    fn test_root_folder_add() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveReadarrBlock::RootFolders.into());
      app
        .data
        .readarr_data
        .root_folders
        .set_items(root_folders_vec());

      RootFoldersHandler::new(
        DEFAULT_KEYBINDINGS.add.key,
        &mut app,
        ActiveReadarrBlock::RootFolders,
        None,
      )
      .handle();

      assert_navigation_pushed!(app, ActiveReadarrBlock::AddRootFolderPrompt.into());
      assert_modal_present!(app.data.readarr_data.add_root_folder_modal);
      assert_eq!(
        app.data.readarr_data.selected_block.get_active_block(),
        ActiveReadarrBlock::AddRootFolderNameInput
      );
    }

    #[test]
    fn test_root_folder_add_no_op_when_not_ready() {
      let mut app = App::test_default();
      app.is_loading = true;
      app.push_navigation_stack(ActiveReadarrBlock::RootFolders.into());
      app
        .data
        .readarr_data
        .root_folders
        .set_items(root_folders_vec());

      RootFoldersHandler::new(
        DEFAULT_KEYBINDINGS.add.key,
        &mut app,
        ActiveReadarrBlock::RootFolders,
        None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveReadarrBlock::RootFolders.into()
      );
      assert_none!(app.data.readarr_data.add_root_folder_modal);
    }

    #[test]
    fn test_refresh_root_folders_key() {
      let mut app = App::test_default();
      app
        .data
        .readarr_data
        .root_folders
        .set_items(root_folders_vec());
      app.push_navigation_stack(ActiveReadarrBlock::RootFolders.into());

      RootFoldersHandler::new(
        DEFAULT_KEYBINDINGS.refresh.key,
        &mut app,
        ActiveReadarrBlock::RootFolders,
        None,
      )
      .handle();

      assert_navigation_pushed!(app, ActiveReadarrBlock::RootFolders.into());
      assert!(app.should_refresh);
    }

    #[test]
    fn test_refresh_root_folders_key_no_op_when_not_ready() {
      let mut app = App::test_default();
      app.is_loading = true;
      app
        .data
        .readarr_data
        .root_folders
        .set_items(root_folders_vec());
      app.push_navigation_stack(ActiveReadarrBlock::RootFolders.into());

      RootFoldersHandler::new(
        DEFAULT_KEYBINDINGS.refresh.key,
        &mut app,
        ActiveReadarrBlock::RootFolders,
        None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveReadarrBlock::RootFolders.into()
      );
      assert!(!app.should_refresh);
    }

    #[test]
    fn test_delete_root_folder_prompt_confirm() {
      let mut app = App::test_default();
      app
        .data
        .readarr_data
        .root_folders
        .set_items(root_folders_vec());
      app.data.readarr_data.root_folders.select_index(Some(1));
      app.push_navigation_stack(ActiveReadarrBlock::RootFolders.into());
      app.push_navigation_stack(ActiveReadarrBlock::DeleteRootFolderPrompt.into());

      RootFoldersHandler::new(
        DEFAULT_KEYBINDINGS.confirm.key,
        &mut app,
        ActiveReadarrBlock::DeleteRootFolderPrompt,
        None,
      )
      .handle();

      assert!(app.data.readarr_data.prompt_confirm);
      assert_some_eq_x!(
        &app.data.readarr_data.prompt_confirm_action,
        &ReadarrEvent::DeleteRootFolder(2)
      );
      assert_navigation_popped!(app, ActiveReadarrBlock::RootFolders.into());
    }
  }

  #[test]
  fn test_root_folders_handler_accepts() {
    ActiveReadarrBlock::iter().for_each(|active_readarr_block| {
      if ROOT_FOLDERS_BLOCKS.contains(&active_readarr_block)
        || ADD_ROOT_FOLDER_BLOCKS.contains(&active_readarr_block)
      {
        assert!(
          RootFoldersHandler::accepts(active_readarr_block),
          "{active_readarr_block} is not accepted by the RootFoldersHandler"
        );
      } else {
        assert!(
          !RootFoldersHandler::accepts(active_readarr_block),
          "{active_readarr_block} is accepted by the RootFoldersHandler"
        );
      }
    })
  }

  #[rstest]
  fn test_root_folders_handler_ignore_special_keys(
    #[values(true, false)] ignore_special_keys_for_textbox_input: bool,
  ) {
    let mut app = App::test_default();
    app.ignore_special_keys_for_textbox_input = ignore_special_keys_for_textbox_input;
    let handler = RootFoldersHandler::new(
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
  fn test_extract_root_folder_id() {
    let mut app = App::test_default();
    app
      .data
      .readarr_data
      .root_folders
      .set_items(root_folders_vec());
    app.data.readarr_data.root_folders.select_index(Some(1));

    let root_folder_id = RootFoldersHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveReadarrBlock::DeleteRootFolderPrompt,
      None,
    )
    .extract_root_folder_id();

    assert_eq!(root_folder_id, 2);
  }

  #[test]
  fn test_root_folders_handler_not_ready_when_loading() {
    let mut app = App::test_default();
    app.push_navigation_stack(ActiveReadarrBlock::RootFolders.into());
    app.is_loading = true;
    app
      .data
      .readarr_data
      .root_folders
      .set_items(root_folders_vec());

    let handler = RootFoldersHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveReadarrBlock::RootFolders,
      None,
    );

    assert!(!handler.is_ready());
  }

  #[test]
  fn test_root_folders_handler_not_ready_when_root_folders_is_empty() {
    let mut app = App::test_default();
    app.push_navigation_stack(ActiveReadarrBlock::RootFolders.into());
    app.is_loading = false;

    let handler = RootFoldersHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveReadarrBlock::RootFolders,
      None,
    );

    assert!(!handler.is_ready());
  }

  #[test]
  fn test_root_folders_handler_ready_when_not_loading_and_root_folders_is_not_empty() {
    let mut app = App::test_default();
    app.push_navigation_stack(ActiveReadarrBlock::RootFolders.into());
    app.is_loading = false;
    app
      .data
      .readarr_data
      .root_folders
      .set_items(root_folders_vec());

    let handler = RootFoldersHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveReadarrBlock::RootFolders,
      None,
    );

    assert!(handler.is_ready());
  }

  fn root_folders_vec() -> Vec<RootFolder> {
    vec![
      RootFolder {
        id: 1,
        path: "/nfs/books".to_owned(),
        accessible: true,
        free_space: 219902325555200,
        unmapped_folders: None,
      },
      RootFolder {
        id: 2,
        path: "/nfs/audiobooks".to_owned(),
        accessible: false,
        free_space: 1024,
        unmapped_folders: None,
      },
    ]
  }
}
