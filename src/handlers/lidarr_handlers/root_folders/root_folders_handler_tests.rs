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
  use crate::handlers::lidarr_handlers::root_folders::RootFoldersHandler;
  use crate::models::servarr_data::lidarr::lidarr_data::{
    ADD_ROOT_FOLDER_BLOCKS, ActiveLidarrBlock, ROOT_FOLDERS_BLOCKS,
  };
  use crate::models::servarr_models::RootFolder;
  use crate::network::lidarr_network::lidarr_network_test_utils::test_utils::root_folder;

  mod test_handle_delete {
    use pretty_assertions::assert_eq;

    use super::*;

    const DELETE_KEY: Key = DEFAULT_KEYBINDINGS.delete.key;

    #[test]
    fn test_delete_root_folder_prompt() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveLidarrBlock::RootFolders.into());
      app
        .data
        .lidarr_data
        .root_folders
        .set_items(vec![RootFolder::default()]);

      RootFoldersHandler::new(DELETE_KEY, &mut app, ActiveLidarrBlock::RootFolders, None).handle();

      assert_navigation_pushed!(app, ActiveLidarrBlock::DeleteRootFolderPrompt.into());
    }

    #[test]
    fn test_delete_root_folder_prompt_no_op_when_not_ready() {
      let mut app = App::test_default();
      app.is_loading = true;
      app.push_navigation_stack(ActiveLidarrBlock::RootFolders.into());
      app
        .data
        .lidarr_data
        .root_folders
        .set_items(vec![RootFolder::default()]);

      RootFoldersHandler::new(DELETE_KEY, &mut app, ActiveLidarrBlock::RootFolders, None).handle();

      assert_eq!(
        app.get_current_route(),
        ActiveLidarrBlock::RootFolders.into()
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
      app.push_navigation_stack(ActiveLidarrBlock::RootFolders.into());
      app.is_loading = is_ready;
      app.data.lidarr_data.main_tabs.set_index(3);

      RootFoldersHandler::new(
        DEFAULT_KEYBINDINGS.left.key,
        &mut app,
        ActiveLidarrBlock::RootFolders,
        None,
      )
      .handle();

      assert_eq!(
        app.data.lidarr_data.main_tabs.get_active_route(),
        ActiveLidarrBlock::History.into()
      );
      assert_navigation_pushed!(app, ActiveLidarrBlock::History.into());
    }

    #[rstest]
    fn test_root_folders_tab_right(#[values(true, false)] is_ready: bool) {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveLidarrBlock::RootFolders.into());
      app.is_loading = is_ready;
      app.data.lidarr_data.main_tabs.set_index(3);

      RootFoldersHandler::new(
        DEFAULT_KEYBINDINGS.right.key,
        &mut app,
        ActiveLidarrBlock::RootFolders,
        None,
      )
      .handle();

      assert_eq!(
        app.data.lidarr_data.main_tabs.get_active_route(),
        ActiveLidarrBlock::Artists.into()
      );
      assert_navigation_pushed!(app, ActiveLidarrBlock::Artists.into());
    }

    #[rstest]
    fn test_left_right_delete_root_folder_prompt_toggle(
      #[values(DEFAULT_KEYBINDINGS.left.key, DEFAULT_KEYBINDINGS.right.key)] key: Key,
    ) {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveLidarrBlock::RootFolders.into());

      RootFoldersHandler::new(
        key,
        &mut app,
        ActiveLidarrBlock::DeleteRootFolderPrompt,
        None,
      )
      .handle();

      assert!(app.data.lidarr_data.prompt_confirm);

      RootFoldersHandler::new(
        key,
        &mut app,
        ActiveLidarrBlock::DeleteRootFolderPrompt,
        None,
      )
      .handle();

      assert!(!app.data.lidarr_data.prompt_confirm);
    }
  }

  mod test_handle_submit {
    use crate::network::lidarr_network::LidarrEvent;

    use super::*;
    use crate::assert_navigation_popped;
    use crate::network::lidarr_network::lidarr_network_test_utils::test_utils::root_folder;

    const SUBMIT_KEY: Key = DEFAULT_KEYBINDINGS.submit.key;

    #[test]
    fn test_delete_root_folder_prompt_confirm_submit() {
      let mut app = App::test_default();
      app
        .data
        .lidarr_data
        .root_folders
        .set_items(vec![root_folder()]);
      app.data.lidarr_data.prompt_confirm = true;
      app.push_navigation_stack(ActiveLidarrBlock::RootFolders.into());
      app.push_navigation_stack(ActiveLidarrBlock::DeleteRootFolderPrompt.into());

      RootFoldersHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveLidarrBlock::DeleteRootFolderPrompt,
        None,
      )
      .handle();

      assert!(app.data.lidarr_data.prompt_confirm);
      assert_some_eq_x!(
        &app.data.lidarr_data.prompt_confirm_action,
        &LidarrEvent::DeleteRootFolder(1)
      );
      assert_navigation_popped!(app, ActiveLidarrBlock::RootFolders.into());
    }

    #[test]
    fn test_delete_root_folder_prompt_decline_submit() {
      let mut app = App::test_default();
      app
        .data
        .lidarr_data
        .root_folders
        .set_items(vec![RootFolder::default()]);
      app.push_navigation_stack(ActiveLidarrBlock::RootFolders.into());
      app.push_navigation_stack(ActiveLidarrBlock::DeleteRootFolderPrompt.into());

      RootFoldersHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveLidarrBlock::DeleteRootFolderPrompt,
        None,
      )
      .handle();

      assert!(!app.data.lidarr_data.prompt_confirm);
      assert_none!(app.data.lidarr_data.prompt_confirm_action);
      assert_navigation_popped!(app, ActiveLidarrBlock::RootFolders.into());
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
      app.push_navigation_stack(ActiveLidarrBlock::RootFolders.into());
      app.push_navigation_stack(ActiveLidarrBlock::DeleteRootFolderPrompt.into());
      app.data.lidarr_data.prompt_confirm = true;

      RootFoldersHandler::new(
        ESC_KEY,
        &mut app,
        ActiveLidarrBlock::DeleteRootFolderPrompt,
        None,
      )
      .handle();

      assert_navigation_popped!(app, ActiveLidarrBlock::RootFolders.into());
      assert!(!app.data.lidarr_data.prompt_confirm);
    }

    #[rstest]
    fn test_default_esc(#[values(true, false)] is_ready: bool) {
      let mut app = App::test_default();
      app.is_loading = is_ready;
      app.error = "test error".to_owned().into();
      app.push_navigation_stack(ActiveLidarrBlock::RootFolders.into());
      app.push_navigation_stack(ActiveLidarrBlock::RootFolders.into());

      RootFoldersHandler::new(ESC_KEY, &mut app, ActiveLidarrBlock::RootFolders, None).handle();

      assert_navigation_popped!(app, ActiveLidarrBlock::RootFolders.into());
      assert_is_empty!(app.error.text);
    }
  }

  mod test_handle_key_char {
    use super::*;
    use crate::assert_navigation_popped;
    use crate::network::lidarr_network::LidarrEvent;
    use crate::network::lidarr_network::lidarr_network_test_utils::test_utils::root_folder;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_root_folder_add() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveLidarrBlock::RootFolders.into());
      app
        .data
        .lidarr_data
        .root_folders
        .set_items(vec![RootFolder::default()]);

      RootFoldersHandler::new(
        DEFAULT_KEYBINDINGS.add.key,
        &mut app,
        ActiveLidarrBlock::RootFolders,
        None,
      )
      .handle();

      assert_navigation_pushed!(app, ActiveLidarrBlock::AddRootFolderPrompt.into());
      assert_modal_present!(app.data.lidarr_data.add_root_folder_modal);
    }

    #[test]
    fn test_root_folder_add_no_op_when_not_ready() {
      let mut app = App::test_default();
      app.is_loading = true;
      app.push_navigation_stack(ActiveLidarrBlock::RootFolders.into());
      app
        .data
        .lidarr_data
        .root_folders
        .set_items(vec![RootFolder::default()]);

      RootFoldersHandler::new(
        DEFAULT_KEYBINDINGS.add.key,
        &mut app,
        ActiveLidarrBlock::RootFolders,
        None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveLidarrBlock::RootFolders.into()
      );
      assert_none!(app.data.lidarr_data.add_root_folder_modal);
    }

    #[test]
    fn test_refresh_root_folders_key() {
      let mut app = App::test_default();
      app
        .data
        .lidarr_data
        .root_folders
        .set_items(vec![RootFolder::default()]);
      app.push_navigation_stack(ActiveLidarrBlock::RootFolders.into());

      RootFoldersHandler::new(
        DEFAULT_KEYBINDINGS.refresh.key,
        &mut app,
        ActiveLidarrBlock::RootFolders,
        None,
      )
      .handle();

      assert_navigation_pushed!(app, ActiveLidarrBlock::RootFolders.into());
      assert!(app.should_refresh);
    }

    #[test]
    fn test_refresh_root_folders_key_no_op_when_not_ready() {
      let mut app = App::test_default();
      app.is_loading = true;
      app
        .data
        .lidarr_data
        .root_folders
        .set_items(vec![RootFolder::default()]);
      app.push_navigation_stack(ActiveLidarrBlock::RootFolders.into());

      RootFoldersHandler::new(
        DEFAULT_KEYBINDINGS.refresh.key,
        &mut app,
        ActiveLidarrBlock::RootFolders,
        None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveLidarrBlock::RootFolders.into()
      );
      assert!(!app.should_refresh);
    }

    #[test]
    fn test_delete_root_folder_prompt_confirm() {
      let mut app = App::test_default();
      app
        .data
        .lidarr_data
        .root_folders
        .set_items(vec![root_folder()]);
      app.push_navigation_stack(ActiveLidarrBlock::RootFolders.into());
      app.push_navigation_stack(ActiveLidarrBlock::DeleteRootFolderPrompt.into());

      RootFoldersHandler::new(
        DEFAULT_KEYBINDINGS.confirm.key,
        &mut app,
        ActiveLidarrBlock::DeleteRootFolderPrompt,
        None,
      )
      .handle();

      assert!(app.data.lidarr_data.prompt_confirm);
      assert_some_eq_x!(
        &app.data.lidarr_data.prompt_confirm_action,
        &LidarrEvent::DeleteRootFolder(1)
      );
      assert_navigation_popped!(app, ActiveLidarrBlock::RootFolders.into());
    }
  }

  #[test]
  fn test_root_folders_handler_accepts() {
    ActiveLidarrBlock::iter().for_each(|active_lidarr_block| {
      if ROOT_FOLDERS_BLOCKS.contains(&active_lidarr_block)
        || ADD_ROOT_FOLDER_BLOCKS.contains(&active_lidarr_block)
      {
        assert!(RootFoldersHandler::accepts(active_lidarr_block));
      } else {
        assert!(!RootFoldersHandler::accepts(active_lidarr_block));
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
      ActiveLidarrBlock::default(),
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
      .lidarr_data
      .root_folders
      .set_items(vec![root_folder()]);

    let root_folder_id = RootFoldersHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveLidarrBlock::DeleteRootFolderPrompt,
      None,
    )
    .extract_root_folder_id();

    assert_eq!(root_folder_id, 1);
  }

  #[test]
  fn test_root_folders_handler_not_ready_when_loading() {
    let mut app = App::test_default();
    app.push_navigation_stack(ActiveLidarrBlock::RootFolders.into());
    app.is_loading = true;

    let handler = RootFoldersHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveLidarrBlock::RootFolders,
      None,
    );

    assert!(!handler.is_ready());
  }

  #[test]
  fn test_root_folders_handler_not_ready_when_root_folders_is_empty() {
    let mut app = App::test_default();
    app.push_navigation_stack(ActiveLidarrBlock::RootFolders.into());
    app.is_loading = false;

    let handler = RootFoldersHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveLidarrBlock::RootFolders,
      None,
    );

    assert!(!handler.is_ready());
  }

  #[test]
  fn test_root_folders_handler_ready_when_not_loading_and_root_folders_is_not_empty() {
    let mut app = App::test_default();
    app.push_navigation_stack(ActiveLidarrBlock::RootFolders.into());
    app.is_loading = false;

    app
      .data
      .lidarr_data
      .root_folders
      .set_items(vec![RootFolder::default()]);
    let handler = RootFoldersHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveLidarrBlock::RootFolders,
      None,
    );

    assert!(handler.is_ready());
  }
}
