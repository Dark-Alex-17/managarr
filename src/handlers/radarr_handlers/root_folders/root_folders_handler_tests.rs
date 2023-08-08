#[cfg(test)]
mod tests {
  use pretty_assertions::assert_str_eq;
  use strum::IntoEnumIterator;

  use crate::app::key_binding::DEFAULT_KEYBINDINGS;
  use crate::app::radarr::{ActiveRadarrBlock, ROOT_FOLDERS_BLOCKS};
  use crate::app::App;
  use crate::event::Key;
  use crate::handlers::radarr_handlers::root_folders::RootFoldersHandler;
  use crate::handlers::KeyEventHandler;
  use crate::models::HorizontallyScrollableText;

  mod test_handle_scroll_up_and_down {
    use rstest::rstest;

    use crate::models::radarr_models::RootFolder;
    use crate::{simple_stateful_iterable_vec, test_iterable_scroll};

    use super::*;

    test_iterable_scroll!(
      test_root_folders_scroll,
      RootFoldersHandler,
      root_folders,
      simple_stateful_iterable_vec!(RootFolder, String, path),
      ActiveRadarrBlock::RootFolders,
      None,
      path
    );
  }

  mod test_handle_home_end {
    use pretty_assertions::assert_eq;

    use crate::models::radarr_models::RootFolder;
    use crate::{
      extended_stateful_iterable_vec, test_iterable_home_and_end, test_text_box_home_end_keys,
    };

    use super::*;

    test_iterable_home_and_end!(
      test_root_folders_home_end,
      RootFoldersHandler,
      root_folders,
      extended_stateful_iterable_vec!(RootFolder, String, path),
      ActiveRadarrBlock::RootFolders,
      None,
      path
    );

    #[test]
    fn test_add_root_folder_prompt_home_end_keys() {
      test_text_box_home_end_keys!(
        RootFoldersHandler,
        ActiveRadarrBlock::AddRootFolderPrompt,
        edit_path
      );
    }
  }

  mod test_handle_delete {
    use pretty_assertions::assert_eq;

    use crate::assert_delete_prompt;

    use super::*;

    const DELETE_KEY: Key = DEFAULT_KEYBINDINGS.delete.key;

    #[test]
    fn test_delete_root_folder_prompt() {
      assert_delete_prompt!(
        RootFoldersHandler,
        ActiveRadarrBlock::RootFolders,
        ActiveRadarrBlock::DeleteRootFolderPrompt
      );
    }
  }

  mod test_handle_left_right_action {
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    use crate::test_text_box_left_right_keys;

    use super::*;

    #[test]
    fn test_root_folders_tab_left() {
      let mut app = App::default();
      app.data.radarr_data.main_tabs.set_index(3);

      RootFoldersHandler::with(
        &DEFAULT_KEYBINDINGS.left.key,
        &mut app,
        &ActiveRadarrBlock::RootFolders,
        &None,
      )
      .handle();

      assert_eq!(
        app.data.radarr_data.main_tabs.get_active_route(),
        &ActiveRadarrBlock::Collections.into()
      );
      assert_eq!(
        app.get_current_route(),
        &ActiveRadarrBlock::Collections.into()
      );
    }

    #[test]
    fn test_root_folders_tab_right() {
      let mut app = App::default();
      app.data.radarr_data.main_tabs.set_index(3);

      RootFoldersHandler::with(
        &DEFAULT_KEYBINDINGS.right.key,
        &mut app,
        &ActiveRadarrBlock::RootFolders,
        &None,
      )
      .handle();

      assert_eq!(
        app.data.radarr_data.main_tabs.get_active_route(),
        &ActiveRadarrBlock::Indexers.into()
      );
      assert_eq!(app.get_current_route(), &ActiveRadarrBlock::Indexers.into());
    }

    #[rstest]
    fn test_left_right_delete_root_folder_prompt_toggle(
      #[values(DEFAULT_KEYBINDINGS.left.key, DEFAULT_KEYBINDINGS.right.key)] key: Key,
    ) {
      let mut app = App::default();

      RootFoldersHandler::with(
        &key,
        &mut app,
        &ActiveRadarrBlock::DeleteRootFolderPrompt,
        &None,
      )
      .handle();

      assert!(app.data.radarr_data.prompt_confirm);

      RootFoldersHandler::with(
        &key,
        &mut app,
        &ActiveRadarrBlock::DeleteRootFolderPrompt,
        &None,
      )
      .handle();

      assert!(!app.data.radarr_data.prompt_confirm);
    }

    #[test]
    fn test_add_root_folder_prompt_left_right_keys() {
      test_text_box_left_right_keys!(
        RootFoldersHandler,
        ActiveRadarrBlock::AddRootFolderPrompt,
        edit_path
      );
    }
  }

  mod test_handle_submit {
    use pretty_assertions::assert_eq;

    use crate::network::radarr_network::RadarrEvent;

    use super::*;

    const SUBMIT_KEY: Key = DEFAULT_KEYBINDINGS.submit.key;

    #[test]
    fn test_add_root_folder_prompt_confirm_submit() {
      let mut app = App::default();
      app.data.radarr_data.prompt_confirm = true;
      app.should_ignore_quit_key = true;
      app.push_navigation_stack(ActiveRadarrBlock::RootFolders.into());
      app.push_navigation_stack(ActiveRadarrBlock::AddRootFolderPrompt.into());

      RootFoldersHandler::with(
        &SUBMIT_KEY,
        &mut app,
        &ActiveRadarrBlock::AddRootFolderPrompt,
        &None,
      )
      .handle();

      assert!(app.data.radarr_data.prompt_confirm);
      assert!(!app.should_ignore_quit_key);
      assert_eq!(
        app.data.radarr_data.prompt_confirm_action,
        Some(RadarrEvent::AddRootFolder)
      );
      assert_eq!(
        app.get_current_route(),
        &ActiveRadarrBlock::RootFolders.into()
      );
    }

    #[test]
    fn test_delete_root_folder_prompt_confirm_submit() {
      let mut app = App::default();
      app.data.radarr_data.prompt_confirm = true;
      app.push_navigation_stack(ActiveRadarrBlock::RootFolders.into());
      app.push_navigation_stack(ActiveRadarrBlock::DeleteRootFolderPrompt.into());

      RootFoldersHandler::with(
        &SUBMIT_KEY,
        &mut app,
        &ActiveRadarrBlock::DeleteRootFolderPrompt,
        &None,
      )
      .handle();

      assert!(app.data.radarr_data.prompt_confirm);
      assert_eq!(
        app.data.radarr_data.prompt_confirm_action,
        Some(RadarrEvent::DeleteRootFolder)
      );
      assert_eq!(
        app.get_current_route(),
        &ActiveRadarrBlock::RootFolders.into()
      );
    }

    #[test]
    fn test_delete_root_folder_prompt_decline_submit() {
      let mut app = App::default();
      app.push_navigation_stack(ActiveRadarrBlock::RootFolders.into());
      app.push_navigation_stack(ActiveRadarrBlock::DeleteRootFolderPrompt.into());

      RootFoldersHandler::with(
        &SUBMIT_KEY,
        &mut app,
        &ActiveRadarrBlock::DeleteRootFolderPrompt,
        &None,
      )
      .handle();

      assert!(!app.data.radarr_data.prompt_confirm);
      assert_eq!(app.data.radarr_data.prompt_confirm_action, None);
      assert_eq!(
        app.get_current_route(),
        &ActiveRadarrBlock::RootFolders.into()
      );
    }
  }

  mod test_handle_esc {
    use pretty_assertions::assert_eq;

    use super::*;

    const ESC_KEY: Key = DEFAULT_KEYBINDINGS.esc.key;

    #[test]
    fn test_delete_root_folder_prompt_block_esc() {
      let mut app = App::default();
      app.push_navigation_stack(ActiveRadarrBlock::RootFolders.into());
      app.push_navigation_stack(ActiveRadarrBlock::DeleteRootFolderPrompt.into());
      app.data.radarr_data.prompt_confirm = true;

      RootFoldersHandler::with(
        &ESC_KEY,
        &mut app,
        &ActiveRadarrBlock::DeleteRootFolderPrompt,
        &None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        &ActiveRadarrBlock::RootFolders.into()
      );
      assert!(!app.data.radarr_data.prompt_confirm);
    }

    #[test]
    fn test_add_root_folder_prompt_esc() {
      let mut app = App::default();
      app.push_navigation_stack(ActiveRadarrBlock::RootFolders.into());
      app.push_navigation_stack(ActiveRadarrBlock::AddRootFolderPrompt.into());
      app.data.radarr_data.edit_path = HorizontallyScrollableText::from("/nfs/test");
      app.should_ignore_quit_key = true;

      RootFoldersHandler::with(
        &ESC_KEY,
        &mut app,
        &ActiveRadarrBlock::AddRootFolderPrompt,
        &None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        &ActiveRadarrBlock::RootFolders.into()
      );

      assert!(app.data.radarr_data.edit_path.text.is_empty());
      assert!(!app.data.radarr_data.prompt_confirm);
      assert!(!app.should_ignore_quit_key);
    }

    #[test]
    fn test_default_esc() {
      let mut app = App::default();
      app.error = "test error".to_owned().into();
      app.push_navigation_stack(ActiveRadarrBlock::RootFolders.into());
      app.push_navigation_stack(ActiveRadarrBlock::RootFolders.into());

      RootFoldersHandler::with(&ESC_KEY, &mut app, &ActiveRadarrBlock::RootFolders, &None).handle();

      assert_eq!(
        app.get_current_route(),
        &ActiveRadarrBlock::RootFolders.into()
      );
      assert!(app.error.text.is_empty());
    }
  }

  mod test_handle_key_char {
    use pretty_assertions::{assert_eq, assert_str_eq};

    use crate::assert_refresh_key;

    use super::*;

    #[test]
    fn test_root_folder_add() {
      let mut app = App::default();

      RootFoldersHandler::with(
        &DEFAULT_KEYBINDINGS.add.key,
        &mut app,
        &ActiveRadarrBlock::RootFolders,
        &None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        &ActiveRadarrBlock::AddRootFolderPrompt.into()
      );
      assert!(app.should_ignore_quit_key);
    }

    #[test]
    fn test_refresh_root_folders_key() {
      assert_refresh_key!(RootFoldersHandler, ActiveRadarrBlock::RootFolders);
    }

    #[test]
    fn test_add_root_folder_prompt_backspace_key() {
      let mut app = App::default();
      app.data.radarr_data.edit_path = "/nfs/test".to_owned().into();

      RootFoldersHandler::with(
        &DEFAULT_KEYBINDINGS.backspace.key,
        &mut app,
        &ActiveRadarrBlock::AddRootFolderPrompt,
        &None,
      )
      .handle();

      assert_str_eq!(app.data.radarr_data.edit_path.text, "/nfs/tes");
    }

    #[test]
    fn test_add_root_folder_prompt_char_key() {
      let mut app = App::default();

      RootFoldersHandler::with(
        &Key::Char('h'),
        &mut app,
        &ActiveRadarrBlock::AddRootFolderPrompt,
        &None,
      )
      .handle();

      assert_str_eq!(app.data.radarr_data.edit_path.text, "h");
    }
  }

  #[test]
  fn test_root_folders_handler_accepts() {
    ActiveRadarrBlock::iter().for_each(|active_radarr_block| {
      if ROOT_FOLDERS_BLOCKS.contains(&active_radarr_block) {
        assert!(RootFoldersHandler::accepts(&active_radarr_block));
      } else {
        assert!(!RootFoldersHandler::accepts(&active_radarr_block));
      }
    })
  }
}
