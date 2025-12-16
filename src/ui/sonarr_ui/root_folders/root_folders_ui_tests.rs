#[cfg(test)]
mod tests {
  use strum::IntoEnumIterator;

  use crate::app::App;
  use crate::models::HorizontallyScrollableText;
  use crate::models::servarr_data::sonarr::sonarr_data::{ActiveSonarrBlock, ROOT_FOLDERS_BLOCKS};
  use crate::models::servarr_models::RootFolder;
  use crate::models::stateful_table::StatefulTable;
  use crate::ui::DrawUi;
  use crate::ui::sonarr_ui::root_folders::RootFoldersUi;
  use crate::ui::ui_test_utils::test_utils::render_to_string_with_app;

  #[test]
  fn test_root_folders_ui_accepts() {
    ActiveSonarrBlock::iter().for_each(|active_sonarr_block| {
      if ROOT_FOLDERS_BLOCKS.contains(&active_sonarr_block) {
        assert!(RootFoldersUi::accepts(active_sonarr_block.into()));
      } else {
        assert!(!RootFoldersUi::accepts(active_sonarr_block.into()));
      }
    });
  }

  mod snapshot_tests {
    use crate::ui::ui_test_utils::test_utils::TerminalSize;

    use super::*;

    #[test]
    fn test_root_folders_ui_renders_loading_state() {
      let mut app = App::test_default();
      app.is_loading = true;
      app.push_navigation_stack(ActiveSonarrBlock::RootFolders.into());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        RootFoldersUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }

    #[test]
    fn test_root_folders_ui_renders_empty_root_folders() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveSonarrBlock::RootFolders.into());
      app.data.sonarr_data.root_folders = StatefulTable::default();

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        RootFoldersUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }

    #[test]
    fn test_root_folders_ui_renders_with_root_folders() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveSonarrBlock::RootFolders.into());
      app.data.sonarr_data.root_folders = StatefulTable::default();
      app.data.sonarr_data.root_folders.set_items(vec![
        RootFolder {
          path: "/tv".to_owned(),
          accessible: true,
          free_space: 1024 * 1024 * 1024 * 100,
          ..RootFolder::default()
        },
        RootFolder {
          path: "/media/tv".to_owned(),
          accessible: true,
          free_space: 1024 * 1024 * 1024 * 50,
          ..RootFolder::default()
        },
      ]);

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        RootFoldersUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }

    #[test]
    fn test_root_folders_ui_renders_add_root_folder() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveSonarrBlock::AddRootFolderPrompt.into());
      app.data.sonarr_data.root_folders = StatefulTable::default();
      app.data.sonarr_data.edit_root_folder = Some(HorizontallyScrollableText::default());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        RootFoldersUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }
  }
}
