#[cfg(test)]
mod tests {
  use pretty_assertions::assert_str_eq;
  use strum::IntoEnumIterator;

  use crate::app::key_binding::DEFAULT_KEYBINDINGS;
  use crate::app::App;
  use crate::event::Key;
  use crate::handlers::sonarr_handlers::root_folders::RootFoldersHandler;
  use crate::handlers::KeyEventHandler;
  use crate::models::servarr_data::sonarr::sonarr_data::{ActiveSonarrBlock, ROOT_FOLDERS_BLOCKS};
  use crate::models::servarr_models::RootFolder;
  use crate::models::HorizontallyScrollableText;

  mod test_handle_scroll_up_and_down {
    use rstest::rstest;

    use crate::models::servarr_models::RootFolder;
    use crate::{simple_stateful_iterable_vec, test_iterable_scroll};

    use super::*;

    test_iterable_scroll!(
      test_root_folders_scroll,
      RootFoldersHandler,
      sonarr_data,
      root_folders,
      simple_stateful_iterable_vec!(RootFolder, String, path),
      ActiveSonarrBlock::RootFolders,
      None,
      path
    );

    #[rstest]
    fn test_root_folders_scroll_no_op_when_not_ready(
      #[values(DEFAULT_KEYBINDINGS.up.key, DEFAULT_KEYBINDINGS.down.key)] key: Key,
    ) {
      let mut app = App::default();
      app.push_navigation_stack(ActiveSonarrBlock::RootFolders.into());
      app.is_loading = true;
      app
        .data
        .sonarr_data
        .root_folders
        .set_items(simple_stateful_iterable_vec!(RootFolder, String, path));

      RootFoldersHandler::with(key, &mut app, ActiveSonarrBlock::RootFolders, None).handle();

      assert_str_eq!(
        app.data.sonarr_data.root_folders.current_selection().path,
        "Test 1"
      );

      RootFoldersHandler::with(key, &mut app, ActiveSonarrBlock::RootFolders, None).handle();

      assert_str_eq!(
        app.data.sonarr_data.root_folders.current_selection().path,
        "Test 1"
      );
    }
  }

  mod test_handle_home_end {
    use std::sync::atomic::Ordering;

    use pretty_assertions::assert_eq;

    use crate::models::servarr_models::RootFolder;
    use crate::{extended_stateful_iterable_vec, test_iterable_home_and_end};

    use super::*;

    test_iterable_home_and_end!(
      test_root_folders_home_end,
      RootFoldersHandler,
      sonarr_data,
      root_folders,
      extended_stateful_iterable_vec!(RootFolder, String, path),
      ActiveSonarrBlock::RootFolders,
      None,
      path
    );

    #[test]
    fn test_root_folders_home_end_no_op_when_not_ready() {
      let mut app = App::default();
      app.push_navigation_stack(ActiveSonarrBlock::RootFolders.into());
      app.is_loading = true;
      app
        .data
        .sonarr_data
        .root_folders
        .set_items(extended_stateful_iterable_vec!(RootFolder, String, path));

      RootFoldersHandler::with(
        DEFAULT_KEYBINDINGS.end.key,
        &mut app,
        ActiveSonarrBlock::RootFolders,
        None,
      )
      .handle();

      assert_str_eq!(
        app.data.sonarr_data.root_folders.current_selection().path,
        "Test 1"
      );

      RootFoldersHandler::with(
        DEFAULT_KEYBINDINGS.home.key,
        &mut app,
        ActiveSonarrBlock::RootFolders,
        None,
      )
      .handle();

      assert_str_eq!(
        app.data.sonarr_data.root_folders.current_selection().path,
        "Test 1"
      );
    }

    #[test]
    fn test_add_root_folder_prompt_home_end_keys() {
      let mut app = App::default();
      app.push_navigation_stack(ActiveSonarrBlock::RootFolders.into());
      app
        .data
        .sonarr_data
        .root_folders
        .set_items(vec![RootFolder::default()]);
      app.data.sonarr_data.edit_root_folder = Some("Test".into());

      RootFoldersHandler::with(
        DEFAULT_KEYBINDINGS.home.key,
        &mut app,
        ActiveSonarrBlock::AddRootFolderPrompt,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .sonarr_data
          .edit_root_folder
          .as_ref()
          .unwrap()
          .offset
          .load(Ordering::SeqCst),
        4
      );

      RootFoldersHandler::with(
        DEFAULT_KEYBINDINGS.end.key,
        &mut app,
        ActiveSonarrBlock::AddRootFolderPrompt,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .sonarr_data
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
      let mut app = App::default();
      app.push_navigation_stack(ActiveSonarrBlock::RootFolders.into());
      app
        .data
        .sonarr_data
        .root_folders
        .set_items(vec![RootFolder::default()]);

      RootFoldersHandler::with(DELETE_KEY, &mut app, ActiveSonarrBlock::RootFolders, None).handle();

      assert_eq!(
        app.get_current_route(),
        ActiveSonarrBlock::DeleteRootFolderPrompt.into()
      );
    }

    #[test]
    fn test_delete_root_folder_prompt_no_op_when_not_ready() {
      let mut app = App::default();
      app.is_loading = true;
      app.push_navigation_stack(ActiveSonarrBlock::RootFolders.into());
      app
        .data
        .sonarr_data
        .root_folders
        .set_items(vec![RootFolder::default()]);

      RootFoldersHandler::with(DELETE_KEY, &mut app, ActiveSonarrBlock::RootFolders, None).handle();

      assert_eq!(
        app.get_current_route(),
        ActiveSonarrBlock::RootFolders.into()
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
      let mut app = App::default();
      app.push_navigation_stack(ActiveSonarrBlock::RootFolders.into());
      app.is_loading = is_ready;
      app.data.sonarr_data.main_tabs.set_index(4);

      RootFoldersHandler::with(
        DEFAULT_KEYBINDINGS.left.key,
        &mut app,
        ActiveSonarrBlock::RootFolders,
        None,
      )
      .handle();

      assert_eq!(
        app.data.sonarr_data.main_tabs.get_active_route(),
        ActiveSonarrBlock::History.into()
      );
      assert_eq!(app.get_current_route(), ActiveSonarrBlock::History.into());
    }

    #[rstest]
    fn test_root_folders_tab_right(#[values(true, false)] is_ready: bool) {
      let mut app = App::default();
      app.push_navigation_stack(ActiveSonarrBlock::RootFolders.into());
      app.is_loading = is_ready;
      app.data.sonarr_data.main_tabs.set_index(4);

      RootFoldersHandler::with(
        DEFAULT_KEYBINDINGS.right.key,
        &mut app,
        ActiveSonarrBlock::RootFolders,
        None,
      )
      .handle();

      assert_eq!(
        app.data.sonarr_data.main_tabs.get_active_route(),
        ActiveSonarrBlock::Indexers.into()
      );
      assert_eq!(app.get_current_route(), ActiveSonarrBlock::Indexers.into());
    }

    #[rstest]
    fn test_left_right_delete_root_folder_prompt_toggle(
      #[values(DEFAULT_KEYBINDINGS.left.key, DEFAULT_KEYBINDINGS.right.key)] key: Key,
    ) {
      let mut app = App::default();
      app.push_navigation_stack(ActiveSonarrBlock::RootFolders.into());

      RootFoldersHandler::with(
        key,
        &mut app,
        ActiveSonarrBlock::DeleteRootFolderPrompt,
        None,
      )
      .handle();

      assert!(app.data.sonarr_data.prompt_confirm);

      RootFoldersHandler::with(
        key,
        &mut app,
        ActiveSonarrBlock::DeleteRootFolderPrompt,
        None,
      )
      .handle();

      assert!(!app.data.sonarr_data.prompt_confirm);
    }

    #[test]
    fn test_add_root_folder_prompt_left_right_keys() {
      let mut app = App::default();
      app.push_navigation_stack(ActiveSonarrBlock::RootFolders.into());
      app.data.sonarr_data.edit_root_folder = Some("Test".into());

      RootFoldersHandler::with(
        DEFAULT_KEYBINDINGS.left.key,
        &mut app,
        ActiveSonarrBlock::AddRootFolderPrompt,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .sonarr_data
          .edit_root_folder
          .as_ref()
          .unwrap()
          .offset
          .load(Ordering::SeqCst),
        1
      );

      RootFoldersHandler::with(
        DEFAULT_KEYBINDINGS.right.key,
        &mut app,
        ActiveSonarrBlock::AddRootFolderPrompt,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .sonarr_data
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

    use crate::network::sonarr_network::SonarrEvent;

    use super::*;

    const SUBMIT_KEY: Key = DEFAULT_KEYBINDINGS.submit.key;

    #[test]
    fn test_add_root_folder_prompt_confirm_submit() {
      let mut app = App::default();
      app
        .data
        .sonarr_data
        .root_folders
        .set_items(vec![RootFolder::default()]);
      app.data.sonarr_data.edit_root_folder = Some("Test".into());
      app.data.sonarr_data.prompt_confirm = true;
      app.should_ignore_quit_key = true;
      app.push_navigation_stack(ActiveSonarrBlock::RootFolders.into());
      app.push_navigation_stack(ActiveSonarrBlock::AddRootFolderPrompt.into());

      RootFoldersHandler::with(
        SUBMIT_KEY,
        &mut app,
        ActiveSonarrBlock::AddRootFolderPrompt,
        None,
      )
      .handle();

      assert!(app.data.sonarr_data.prompt_confirm);
      assert!(!app.should_ignore_quit_key);
      assert_eq!(
        app.data.sonarr_data.prompt_confirm_action,
        Some(SonarrEvent::AddRootFolder(None))
      );
      assert_eq!(
        app.get_current_route(),
        ActiveSonarrBlock::RootFolders.into()
      );
    }

    #[test]
    fn test_add_root_folder_prompt_confirm_submit_noop_on_empty_folder() {
      let mut app = App::default();
      app.data.sonarr_data.edit_root_folder = Some(HorizontallyScrollableText::default());
      app.data.sonarr_data.prompt_confirm = false;
      app.should_ignore_quit_key = true;
      app.push_navigation_stack(ActiveSonarrBlock::RootFolders.into());
      app.push_navigation_stack(ActiveSonarrBlock::AddRootFolderPrompt.into());

      RootFoldersHandler::with(
        SUBMIT_KEY,
        &mut app,
        ActiveSonarrBlock::AddRootFolderPrompt,
        None,
      )
      .handle();

      assert!(!app.data.sonarr_data.prompt_confirm);
      assert!(app.should_ignore_quit_key);
      assert!(app.data.sonarr_data.prompt_confirm_action.is_none());
      assert_eq!(
        app.get_current_route(),
        ActiveSonarrBlock::AddRootFolderPrompt.into()
      );
    }

    #[test]
    fn test_delete_root_folder_prompt_confirm_submit() {
      let mut app = App::default();
      app
        .data
        .sonarr_data
        .root_folders
        .set_items(vec![RootFolder::default()]);
      app.data.sonarr_data.prompt_confirm = true;
      app.push_navigation_stack(ActiveSonarrBlock::RootFolders.into());
      app.push_navigation_stack(ActiveSonarrBlock::DeleteRootFolderPrompt.into());

      RootFoldersHandler::with(
        SUBMIT_KEY,
        &mut app,
        ActiveSonarrBlock::DeleteRootFolderPrompt,
        None,
      )
      .handle();

      assert!(app.data.sonarr_data.prompt_confirm);
      assert_eq!(
        app.data.sonarr_data.prompt_confirm_action,
        Some(SonarrEvent::DeleteRootFolder(None))
      );
      assert_eq!(
        app.get_current_route(),
        ActiveSonarrBlock::RootFolders.into()
      );
    }

    #[test]
    fn test_delete_root_folder_prompt_decline_submit() {
      let mut app = App::default();
      app
        .data
        .sonarr_data
        .root_folders
        .set_items(vec![RootFolder::default()]);
      app.push_navigation_stack(ActiveSonarrBlock::RootFolders.into());
      app.push_navigation_stack(ActiveSonarrBlock::DeleteRootFolderPrompt.into());

      RootFoldersHandler::with(
        SUBMIT_KEY,
        &mut app,
        ActiveSonarrBlock::DeleteRootFolderPrompt,
        None,
      )
      .handle();

      assert!(!app.data.sonarr_data.prompt_confirm);
      assert_eq!(app.data.sonarr_data.prompt_confirm_action, None);
      assert_eq!(
        app.get_current_route(),
        ActiveSonarrBlock::RootFolders.into()
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
      let mut app = App::default();
      app.push_navigation_stack(ActiveSonarrBlock::RootFolders.into());
      app.push_navigation_stack(ActiveSonarrBlock::DeleteRootFolderPrompt.into());
      app.data.sonarr_data.prompt_confirm = true;

      RootFoldersHandler::with(
        ESC_KEY,
        &mut app,
        ActiveSonarrBlock::DeleteRootFolderPrompt,
        None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveSonarrBlock::RootFolders.into()
      );
      assert!(!app.data.sonarr_data.prompt_confirm);
    }

    #[test]
    fn test_add_root_folder_prompt_esc() {
      let mut app = App::default();
      app.push_navigation_stack(ActiveSonarrBlock::RootFolders.into());
      app.push_navigation_stack(ActiveSonarrBlock::AddRootFolderPrompt.into());
      app.data.sonarr_data.edit_root_folder = Some("/nfs/test".into());
      app.should_ignore_quit_key = true;

      RootFoldersHandler::with(
        ESC_KEY,
        &mut app,
        ActiveSonarrBlock::AddRootFolderPrompt,
        None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveSonarrBlock::RootFolders.into()
      );

      assert!(app.data.sonarr_data.edit_root_folder.is_none());
      assert!(!app.data.sonarr_data.prompt_confirm);
      assert!(!app.should_ignore_quit_key);
    }

    #[rstest]
    fn test_default_esc(#[values(true, false)] is_ready: bool) {
      let mut app = App::default();
      app.is_loading = is_ready;
      app.error = "test error".to_owned().into();
      app.push_navigation_stack(ActiveSonarrBlock::RootFolders.into());
      app.push_navigation_stack(ActiveSonarrBlock::RootFolders.into());

      RootFoldersHandler::with(ESC_KEY, &mut app, ActiveSonarrBlock::RootFolders, None).handle();

      assert_eq!(
        app.get_current_route(),
        ActiveSonarrBlock::RootFolders.into()
      );
      assert!(app.error.text.is_empty());
    }
  }

  mod test_handle_key_char {
    use pretty_assertions::{assert_eq, assert_str_eq};

    use crate::network::sonarr_network::SonarrEvent;

    use super::*;

    #[test]
    fn test_root_folder_add() {
      let mut app = App::default();
      app.push_navigation_stack(ActiveSonarrBlock::RootFolders.into());
      app
        .data
        .sonarr_data
        .root_folders
        .set_items(vec![RootFolder::default()]);

      RootFoldersHandler::with(
        DEFAULT_KEYBINDINGS.add.key,
        &mut app,
        ActiveSonarrBlock::RootFolders,
        None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveSonarrBlock::AddRootFolderPrompt.into()
      );
      assert!(app.should_ignore_quit_key);
      assert!(app.data.sonarr_data.edit_root_folder.is_some());
    }

    #[test]
    fn test_root_folder_add_no_op_when_not_ready() {
      let mut app = App::default();
      app.is_loading = true;
      app.push_navigation_stack(ActiveSonarrBlock::RootFolders.into());
      app
        .data
        .sonarr_data
        .root_folders
        .set_items(vec![RootFolder::default()]);

      RootFoldersHandler::with(
        DEFAULT_KEYBINDINGS.add.key,
        &mut app,
        ActiveSonarrBlock::RootFolders,
        None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveSonarrBlock::RootFolders.into()
      );
      assert!(!app.should_ignore_quit_key);
      assert!(app.data.sonarr_data.edit_root_folder.is_none());
    }

    #[test]
    fn test_refresh_root_folders_key() {
      let mut app = App::default();
      app
        .data
        .sonarr_data
        .root_folders
        .set_items(vec![RootFolder::default()]);
      app.push_navigation_stack(ActiveSonarrBlock::RootFolders.into());

      RootFoldersHandler::with(
        DEFAULT_KEYBINDINGS.refresh.key,
        &mut app,
        ActiveSonarrBlock::RootFolders,
        None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveSonarrBlock::RootFolders.into()
      );
      assert!(app.should_refresh);
    }

    #[test]
    fn test_refresh_root_folders_key_no_op_when_not_ready() {
      let mut app = App::default();
      app.is_loading = true;
      app
        .data
        .sonarr_data
        .root_folders
        .set_items(vec![RootFolder::default()]);
      app.push_navigation_stack(ActiveSonarrBlock::RootFolders.into());

      RootFoldersHandler::with(
        DEFAULT_KEYBINDINGS.refresh.key,
        &mut app,
        ActiveSonarrBlock::RootFolders,
        None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveSonarrBlock::RootFolders.into()
      );
      assert!(!app.should_refresh);
    }

    #[test]
    fn test_add_root_folder_prompt_backspace_key() {
      let mut app = App::default();
      app.push_navigation_stack(ActiveSonarrBlock::RootFolders.into());
      app
        .data
        .sonarr_data
        .root_folders
        .set_items(vec![RootFolder::default()]);
      app.data.sonarr_data.edit_root_folder = Some("/nfs/test".into());

      RootFoldersHandler::with(
        DEFAULT_KEYBINDINGS.backspace.key,
        &mut app,
        ActiveSonarrBlock::AddRootFolderPrompt,
        None,
      )
      .handle();

      assert_str_eq!(
        app.data.sonarr_data.edit_root_folder.as_ref().unwrap().text,
        "/nfs/tes"
      );
    }

    #[test]
    fn test_add_root_folder_prompt_char_key() {
      let mut app = App::default();
      app.push_navigation_stack(ActiveSonarrBlock::RootFolders.into());
      app
        .data
        .sonarr_data
        .root_folders
        .set_items(vec![RootFolder::default()]);
      app.data.sonarr_data.edit_root_folder = Some(HorizontallyScrollableText::default());

      RootFoldersHandler::with(
        Key::Char('h'),
        &mut app,
        ActiveSonarrBlock::AddRootFolderPrompt,
        None,
      )
      .handle();

      assert_str_eq!(
        app.data.sonarr_data.edit_root_folder.as_ref().unwrap().text,
        "h"
      );
    }

    #[test]
    fn test_delete_root_folder_prompt_confirm() {
      let mut app = App::default();
      app
        .data
        .sonarr_data
        .root_folders
        .set_items(vec![RootFolder::default()]);
      app.push_navigation_stack(ActiveSonarrBlock::RootFolders.into());
      app.push_navigation_stack(ActiveSonarrBlock::DeleteRootFolderPrompt.into());

      RootFoldersHandler::with(
        DEFAULT_KEYBINDINGS.confirm.key,
        &mut app,
        ActiveSonarrBlock::DeleteRootFolderPrompt,
        None,
      )
      .handle();

      assert!(app.data.sonarr_data.prompt_confirm);
      assert_eq!(
        app.data.sonarr_data.prompt_confirm_action,
        Some(SonarrEvent::DeleteRootFolder(None))
      );
      assert_eq!(
        app.get_current_route(),
        ActiveSonarrBlock::RootFolders.into()
      );
    }
  }

  #[test]
  fn test_root_folders_handler_accepts() {
    ActiveSonarrBlock::iter().for_each(|active_sonarr_block| {
      if ROOT_FOLDERS_BLOCKS.contains(&active_sonarr_block) {
        assert!(RootFoldersHandler::accepts(active_sonarr_block));
      } else {
        assert!(!RootFoldersHandler::accepts(active_sonarr_block));
      }
    })
  }

  #[test]
  fn test_root_folders_handler_not_ready_when_loading() {
    let mut app = App::default();
    app.push_navigation_stack(ActiveSonarrBlock::RootFolders.into());
    app.is_loading = true;

    let handler = RootFoldersHandler::with(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveSonarrBlock::RootFolders,
      None,
    );

    assert!(!handler.is_ready());
  }

  #[test]
  fn test_root_folders_handler_not_ready_when_root_folders_is_empty() {
    let mut app = App::default();
    app.push_navigation_stack(ActiveSonarrBlock::RootFolders.into());
    app.is_loading = false;

    let handler = RootFoldersHandler::with(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveSonarrBlock::RootFolders,
      None,
    );

    assert!(!handler.is_ready());
  }

  #[test]
  fn test_root_folders_handler_ready_when_not_loading_and_root_folders_is_not_empty() {
    let mut app = App::default();
    app.push_navigation_stack(ActiveSonarrBlock::RootFolders.into());
    app.is_loading = false;

    app
      .data
      .sonarr_data
      .root_folders
      .set_items(vec![RootFolder::default()]);
    let handler = RootFoldersHandler::with(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveSonarrBlock::RootFolders,
      None,
    );

    assert!(handler.is_ready());
  }
}