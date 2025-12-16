#[cfg(test)]
mod tests {
  use strum::IntoEnumIterator;

  use crate::app::App;
  use crate::models::servarr_data::radarr::radarr_data::{ActiveRadarrBlock, ROOT_FOLDERS_BLOCKS};
  use crate::models::stateful_table::StatefulTable;
  use crate::ui::DrawUi;
  use crate::ui::radarr_ui::root_folders::RootFoldersUi;
  use crate::ui::ui_test_utils::test_utils::{TerminalSize, render_to_string_with_app};

  #[test]
  fn test_root_folders_ui_accepts() {
    ActiveRadarrBlock::iter().for_each(|active_radarr_block| {
      if ROOT_FOLDERS_BLOCKS.contains(&active_radarr_block) {
        assert!(RootFoldersUi::accepts(active_radarr_block.into()));
      } else {
        assert!(!RootFoldersUi::accepts(active_radarr_block.into()));
      }
    });
  }

  mod snapshot_tests {
    use rstest::rstest;
    use super::*;

    #[test]
    fn test_root_folders_ui_renders_loading() {
      let mut app = App::test_default();
      app.is_loading = true;
      app.push_navigation_stack(ActiveRadarrBlock::RootFolders.into());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        RootFoldersUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }

    #[test]
    fn test_root_folders_ui_renders_empty_root_folders() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveRadarrBlock::RootFolders.into());
      app.data.radarr_data.root_folders = StatefulTable::default();

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        RootFoldersUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }

    #[rstest]
    fn test_root_folders_ui_renders_root_folders_tab(
      #[values(
        ActiveRadarrBlock::RootFolders,
        ActiveRadarrBlock::AddRootFolderPrompt,
        ActiveRadarrBlock::DeleteRootFolderPrompt,
      )] active_radarr_block: ActiveRadarrBlock
    ) {
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack(active_radarr_block.into());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        RootFoldersUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(active_radarr_block.to_string(), output);
    }
  }
}