#[cfg(test)]
mod tests {
  use pretty_assertions::assert_eq;
  use strum::IntoEnumIterator;

  use crate::app::key_binding::DEFAULT_KEYBINDINGS;
  use crate::app::App;
  use crate::event::Key;
  use crate::handlers::radarr_handlers::radarr_handler_test_utils::utils::root_folder;
  use crate::handlers::radarr_handlers::root_folders::RootFoldersHandler;
  use crate::handlers::KeyEventHandler;
  use crate::models::servarr_data::radarr::radarr_data::{ActiveRadarrBlock, ROOT_FOLDERS_BLOCKS};
  use crate::models::servarr_models::{AddRootFolderBody, RootFolder};
  use crate::models::HorizontallyScrollableText;

  mod test_handle_home_end {
    use pretty_assertions::assert_eq;
    use std::sync::atomic::Ordering;

    use super::*;
    use crate::models::servarr_models::RootFolder;

    #[test]
    fn test_add_root_folder_prompt_home_end_keys() {
      let mut app = App::test_default();
      app
        .data
        .radarr_data
        .root_folders
        .set_items(vec![RootFolder::default()]);
      app.data.radarr_data.edit_root_folder = Some("Test".into());

      RootFoldersHandler::new(
        DEFAULT_KEYBINDINGS.home.key,
        &mut app,
        ActiveRadarrBlock::AddRootFolderPrompt,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .radarr_data
          .edit_root_folder
          .as_ref()
          .unwrap()
          .offset
          .load(Ordering::SeqCst),
        4
      );

      RootFoldersHandler::new(
        DEFAULT_KEYBINDINGS.end.key,
        &mut app,
        ActiveRadarrBlock::AddRootFolderPrompt,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .radarr_data
          .edit_root_folder
          .as_ref()
          .unwrap()
          .offset
          .load(Ordering::SeqCst),
        0
      );
    }
  }

  mod test_handle_delete {
    use pretty_assertions::assert_eq;

    use super::*;

    const DELETE_KEY: Key = DEFAULT_KEYBINDINGS.delete.key;

    #[test]
    fn test_delete_root_folder_prompt() {
      let mut app = App::test_default();
      app
        .data
        .radarr_data
        .root_folders
        .set_items(vec![RootFolder::default()]);

      RootFoldersHandler::new(DELETE_KEY, &mut app, ActiveRadarrBlock::RootFolders, None).handle();

      assert_eq!(
        app.get_current_route(),
        ActiveRadarrBlock::DeleteRootFolderPrompt.into()
      );
    }

    #[test]
    fn test_delete_root_folder_prompt_no_op_when_not_ready() {
      let mut app = App::test_default();
      app.is_loading = true;
      app.push_navigation_stack(ActiveRadarrBlock::RootFolders.into());
      app
        .data
        .radarr_data
        .root_folders
        .set_items(vec![RootFolder::default()]);

      RootFoldersHandler::new(DELETE_KEY, &mut app, ActiveRadarrBlock::RootFolders, None).handle();

      assert_eq!(
        app.get_current_route(),
        ActiveRadarrBlock::RootFolders.into()
      );
    }
  }

  mod test_handle_left_right_action {
    use std::sync::atomic::Ordering;

    use pretty_assertions::assert_eq;
    use rstest::rstest;

    use super::*;

    #[rstest]
    fn test_root_folders_tab_left(#[values(true, false)] is_ready: bool) {
      let mut app = App::test_default();
      app.is_loading = is_ready;
      app.data.radarr_data.main_tabs.set_index(4);

      RootFoldersHandler::new(
        DEFAULT_KEYBINDINGS.left.key,
        &mut app,
        ActiveRadarrBlock::RootFolders,
        None,
      )
      .handle();

      assert_eq!(
        app.data.radarr_data.main_tabs.get_active_route(),
        ActiveRadarrBlock::Blocklist.into()
      );
      assert_eq!(app.get_current_route(), ActiveRadarrBlock::Blocklist.into());
    }

    #[rstest]
    fn test_root_folders_tab_right(#[values(true, false)] is_ready: bool) {
      let mut app = App::test_default();
      app.is_loading = is_ready;
      app.data.radarr_data.main_tabs.set_index(4);

      RootFoldersHandler::new(
        DEFAULT_KEYBINDINGS.right.key,
        &mut app,
        ActiveRadarrBlock::RootFolders,
        None,
      )
      .handle();

      assert_eq!(
        app.data.radarr_data.main_tabs.get_active_route(),
        ActiveRadarrBlock::Indexers.into()
      );
      assert_eq!(app.get_current_route(), ActiveRadarrBlock::Indexers.into());
    }

    #[rstest]
    fn test_left_right_delete_root_folder_prompt_toggle(
      #[values(DEFAULT_KEYBINDINGS.left.key, DEFAULT_KEYBINDINGS.right.key)] key: Key,
    ) {
      let mut app = App::test_default();

      RootFoldersHandler::new(
        key,
        &mut app,
        ActiveRadarrBlock::DeleteRootFolderPrompt,
        None,
      )
      .handle();

      assert!(app.data.radarr_data.prompt_confirm);

      RootFoldersHandler::new(
        key,
        &mut app,
        ActiveRadarrBlock::DeleteRootFolderPrompt,
        None,
      )
      .handle();

      assert!(!app.data.radarr_data.prompt_confirm);
    }

    #[test]
    fn test_add_root_folder_prompt_left_right_keys() {
      let mut app = App::test_default();
      app.data.radarr_data.edit_root_folder = Some("Test".into());

      RootFoldersHandler::new(
        DEFAULT_KEYBINDINGS.left.key,
        &mut app,
        ActiveRadarrBlock::AddRootFolderPrompt,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .radarr_data
          .edit_root_folder
          .as_ref()
          .unwrap()
          .offset
          .load(Ordering::SeqCst),
        1
      );

      RootFoldersHandler::new(
        DEFAULT_KEYBINDINGS.right.key,
        &mut app,
        ActiveRadarrBlock::AddRootFolderPrompt,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .radarr_data
          .edit_root_folder
          .as_ref()
          .unwrap()
          .offset
          .load(Ordering::SeqCst),
        0
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
      let mut app = App::test_default();
      let expected_add_root_folder_body = AddRootFolderBody {
        path: "Test".to_owned(),
      };
      app
        .data
        .radarr_data
        .root_folders
        .set_items(vec![RootFolder::default()]);
      app.data.radarr_data.edit_root_folder = Some("Test".into());
      app.data.radarr_data.prompt_confirm = true;
      app.should_ignore_quit_key = true;
      app.push_navigation_stack(ActiveRadarrBlock::RootFolders.into());
      app.push_navigation_stack(ActiveRadarrBlock::AddRootFolderPrompt.into());

      RootFoldersHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveRadarrBlock::AddRootFolderPrompt,
        None,
      )
      .handle();

      assert!(app.data.radarr_data.prompt_confirm);
      assert!(!app.should_ignore_quit_key);
      assert_eq!(
        app.data.radarr_data.prompt_confirm_action,
        Some(RadarrEvent::AddRootFolder(expected_add_root_folder_body))
      );
      assert_eq!(
        app.get_current_route(),
        ActiveRadarrBlock::RootFolders.into()
      );
    }

    #[test]
    fn test_add_root_folder_prompt_confirm_submit_noop_on_empty_folder() {
      let mut app = App::test_default();
      app.data.radarr_data.edit_root_folder = Some(HorizontallyScrollableText::default());
      app.data.radarr_data.prompt_confirm = false;
      app.should_ignore_quit_key = true;
      app.push_navigation_stack(ActiveRadarrBlock::RootFolders.into());
      app.push_navigation_stack(ActiveRadarrBlock::AddRootFolderPrompt.into());

      RootFoldersHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveRadarrBlock::AddRootFolderPrompt,
        None,
      )
      .handle();

      assert!(!app.data.radarr_data.prompt_confirm);
      assert!(app.should_ignore_quit_key);
      assert!(app.data.radarr_data.prompt_confirm_action.is_none());
      assert_eq!(
        app.get_current_route(),
        ActiveRadarrBlock::AddRootFolderPrompt.into()
      );
    }

    #[test]
    fn test_delete_root_folder_prompt_confirm_submit() {
      let mut app = App::test_default();
      app
        .data
        .radarr_data
        .root_folders
        .set_items(vec![root_folder()]);
      app.data.radarr_data.prompt_confirm = true;
      app.push_navigation_stack(ActiveRadarrBlock::RootFolders.into());
      app.push_navigation_stack(ActiveRadarrBlock::DeleteRootFolderPrompt.into());

      RootFoldersHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveRadarrBlock::DeleteRootFolderPrompt,
        None,
      )
      .handle();

      assert!(app.data.radarr_data.prompt_confirm);
      assert_eq!(
        app.data.radarr_data.prompt_confirm_action,
        Some(RadarrEvent::DeleteRootFolder(1))
      );
      assert_eq!(
        app.get_current_route(),
        ActiveRadarrBlock::RootFolders.into()
      );
    }

    #[test]
    fn test_delete_root_folder_prompt_decline_submit() {
      let mut app = App::test_default();
      app
        .data
        .radarr_data
        .root_folders
        .set_items(vec![RootFolder::default()]);
      app.push_navigation_stack(ActiveRadarrBlock::RootFolders.into());
      app.push_navigation_stack(ActiveRadarrBlock::DeleteRootFolderPrompt.into());

      RootFoldersHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveRadarrBlock::DeleteRootFolderPrompt,
        None,
      )
      .handle();

      assert!(!app.data.radarr_data.prompt_confirm);
      assert_eq!(app.data.radarr_data.prompt_confirm_action, None);
      assert_eq!(
        app.get_current_route(),
        ActiveRadarrBlock::RootFolders.into()
      );
    }
  }

  mod test_handle_esc {
    use super::*;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    const ESC_KEY: Key = DEFAULT_KEYBINDINGS.esc.key;

    #[test]
    fn test_delete_root_folder_prompt_block_esc() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveRadarrBlock::RootFolders.into());
      app.push_navigation_stack(ActiveRadarrBlock::DeleteRootFolderPrompt.into());
      app.data.radarr_data.prompt_confirm = true;

      RootFoldersHandler::new(
        ESC_KEY,
        &mut app,
        ActiveRadarrBlock::DeleteRootFolderPrompt,
        None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveRadarrBlock::RootFolders.into()
      );
      assert!(!app.data.radarr_data.prompt_confirm);
    }

    #[test]
    fn test_add_root_folder_prompt_esc() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveRadarrBlock::RootFolders.into());
      app.push_navigation_stack(ActiveRadarrBlock::AddRootFolderPrompt.into());
      app.data.radarr_data.edit_root_folder = Some("/nfs/test".into());
      app.should_ignore_quit_key = true;

      RootFoldersHandler::new(
        ESC_KEY,
        &mut app,
        ActiveRadarrBlock::AddRootFolderPrompt,
        None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveRadarrBlock::RootFolders.into()
      );

      assert!(app.data.radarr_data.edit_root_folder.is_none());
      assert!(!app.data.radarr_data.prompt_confirm);
      assert!(!app.should_ignore_quit_key);
    }

    #[rstest]
    fn test_default_esc(#[values(true, false)] is_ready: bool) {
      let mut app = App::test_default();
      app.is_loading = is_ready;
      app.error = "test error".to_owned().into();
      app.push_navigation_stack(ActiveRadarrBlock::RootFolders.into());
      app.push_navigation_stack(ActiveRadarrBlock::RootFolders.into());

      RootFoldersHandler::new(ESC_KEY, &mut app, ActiveRadarrBlock::RootFolders, None).handle();

      assert_eq!(
        app.get_current_route(),
        ActiveRadarrBlock::RootFolders.into()
      );
      assert!(app.error.text.is_empty());
    }
  }

  mod test_handle_key_char {
    use pretty_assertions::{assert_eq, assert_str_eq};

    use crate::network::radarr_network::RadarrEvent;

    use super::*;

    #[test]
    fn test_root_folder_add() {
      let mut app = App::test_default();
      app
        .data
        .radarr_data
        .root_folders
        .set_items(vec![RootFolder::default()]);

      RootFoldersHandler::new(
        DEFAULT_KEYBINDINGS.add.key,
        &mut app,
        ActiveRadarrBlock::RootFolders,
        None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveRadarrBlock::AddRootFolderPrompt.into()
      );
      assert!(app.should_ignore_quit_key);
      assert!(app.data.radarr_data.edit_root_folder.is_some());
    }

    #[test]
    fn test_root_folder_add_no_op_when_not_ready() {
      let mut app = App::test_default();
      app.is_loading = true;
      app.push_navigation_stack(ActiveRadarrBlock::RootFolders.into());
      app
        .data
        .radarr_data
        .root_folders
        .set_items(vec![RootFolder::default()]);

      RootFoldersHandler::new(
        DEFAULT_KEYBINDINGS.add.key,
        &mut app,
        ActiveRadarrBlock::RootFolders,
        None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveRadarrBlock::RootFolders.into()
      );
      assert!(!app.should_ignore_quit_key);
      assert!(app.data.radarr_data.edit_root_folder.is_none());
    }

    #[test]
    fn test_refresh_root_folders_key() {
      let mut app = App::test_default();
      app
        .data
        .radarr_data
        .root_folders
        .set_items(vec![RootFolder::default()]);
      app.push_navigation_stack(ActiveRadarrBlock::RootFolders.into());

      RootFoldersHandler::new(
        DEFAULT_KEYBINDINGS.refresh.key,
        &mut app,
        ActiveRadarrBlock::RootFolders,
        None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveRadarrBlock::RootFolders.into()
      );
      assert!(app.should_refresh);
    }

    #[test]
    fn test_refresh_root_folders_key_no_op_when_not_ready() {
      let mut app = App::test_default();
      app.is_loading = true;
      app
        .data
        .radarr_data
        .root_folders
        .set_items(vec![RootFolder::default()]);
      app.push_navigation_stack(ActiveRadarrBlock::RootFolders.into());

      RootFoldersHandler::new(
        DEFAULT_KEYBINDINGS.refresh.key,
        &mut app,
        ActiveRadarrBlock::RootFolders,
        None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveRadarrBlock::RootFolders.into()
      );
      assert!(!app.should_refresh);
    }

    #[test]
    fn test_add_root_folder_prompt_backspace_key() {
      let mut app = App::test_default();
      app
        .data
        .radarr_data
        .root_folders
        .set_items(vec![RootFolder::default()]);
      app.data.radarr_data.edit_root_folder = Some("/nfs/test".into());

      RootFoldersHandler::new(
        DEFAULT_KEYBINDINGS.backspace.key,
        &mut app,
        ActiveRadarrBlock::AddRootFolderPrompt,
        None,
      )
      .handle();

      assert_str_eq!(
        app.data.radarr_data.edit_root_folder.as_ref().unwrap().text,
        "/nfs/tes"
      );
    }

    #[test]
    fn test_add_root_folder_prompt_char_key() {
      let mut app = App::test_default();
      app
        .data
        .radarr_data
        .root_folders
        .set_items(vec![RootFolder::default()]);
      app.data.radarr_data.edit_root_folder = Some(HorizontallyScrollableText::default());

      RootFoldersHandler::new(
        Key::Char('h'),
        &mut app,
        ActiveRadarrBlock::AddRootFolderPrompt,
        None,
      )
      .handle();

      assert_str_eq!(
        app.data.radarr_data.edit_root_folder.as_ref().unwrap().text,
        "h"
      );
    }

    #[test]
    fn test_delete_root_folder_prompt_confirm() {
      let mut app = App::test_default();
      app
        .data
        .radarr_data
        .root_folders
        .set_items(vec![root_folder()]);
      app.push_navigation_stack(ActiveRadarrBlock::RootFolders.into());
      app.push_navigation_stack(ActiveRadarrBlock::DeleteRootFolderPrompt.into());

      RootFoldersHandler::new(
        DEFAULT_KEYBINDINGS.confirm.key,
        &mut app,
        ActiveRadarrBlock::DeleteRootFolderPrompt,
        None,
      )
      .handle();

      assert!(app.data.radarr_data.prompt_confirm);
      assert_eq!(
        app.data.radarr_data.prompt_confirm_action,
        Some(RadarrEvent::DeleteRootFolder(1))
      );
      assert_eq!(
        app.get_current_route(),
        ActiveRadarrBlock::RootFolders.into()
      );
    }
  }

  #[test]
  fn test_root_folders_handler_accepts() {
    ActiveRadarrBlock::iter().for_each(|active_radarr_block| {
      if ROOT_FOLDERS_BLOCKS.contains(&active_radarr_block) {
        assert!(RootFoldersHandler::accepts(active_radarr_block));
      } else {
        assert!(!RootFoldersHandler::accepts(active_radarr_block));
      }
    })
  }

  #[test]
  fn test_build_add_root_folder_body() {
    let mut app = App::test_default();
    app.data.radarr_data.edit_root_folder = Some("/nfs/test".into());
    let expected_add_root_folder_body = AddRootFolderBody {
      path: "/nfs/test".to_owned(),
    };

    let actual_add_root_folder_body = RootFoldersHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveRadarrBlock::RootFolders,
      None,
    )
    .build_add_root_folder_body();

    assert_eq!(actual_add_root_folder_body, expected_add_root_folder_body);
    assert!(app.data.radarr_data.edit_root_folder.is_none());
  }

  #[test]
  fn test_extract_root_folder_id() {
    let mut app = App::test_default();
    app
      .data
      .radarr_data
      .root_folders
      .set_items(vec![root_folder()]);

    let root_folder_id = RootFoldersHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveRadarrBlock::RootFolders,
      None,
    )
    .extract_root_folder_id();

    assert_eq!(root_folder_id, 1);
  }

  #[test]
  fn test_root_folders_handler_not_ready_when_loading() {
    let mut app = App::test_default();
    app.is_loading = true;

    let handler = RootFoldersHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveRadarrBlock::RootFolders,
      None,
    );

    assert!(!handler.is_ready());
  }

  #[test]
  fn test_root_folders_handler_not_ready_when_root_folders_is_empty() {
    let mut app = App::test_default();
    app.is_loading = false;

    let handler = RootFoldersHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveRadarrBlock::RootFolders,
      None,
    );

    assert!(!handler.is_ready());
  }

  #[test]
  fn test_root_folders_handler_ready_when_not_loading_and_root_folders_is_not_empty() {
    let mut app = App::test_default();
    app.is_loading = false;

    app
      .data
      .radarr_data
      .root_folders
      .set_items(vec![RootFolder::default()]);
    let handler = RootFoldersHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveRadarrBlock::RootFolders,
      None,
    );

    assert!(handler.is_ready());
  }
}
