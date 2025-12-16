#[cfg(test)]
mod tests {
  use strum::IntoEnumIterator;

  use crate::app::App;
  use crate::models::servarr_data::sonarr::sonarr_data::{ActiveSonarrBlock, ROOT_FOLDERS_BLOCKS};
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
    use rstest::rstest;

    use super::*;

    #[test]
    fn test_root_folders_ui_renders_loading() {
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

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        RootFoldersUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }

    #[rstest]
    fn test_root_folders_ui_renders_root_folders_tab(
      #[values(
        ActiveSonarrBlock::RootFolders,
        ActiveSonarrBlock::AddRootFolderPrompt,
        ActiveSonarrBlock::DeleteRootFolderPrompt
      )]
      active_sonarr_block: ActiveSonarrBlock,
    ) {
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack(active_sonarr_block.into());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        RootFoldersUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(active_sonarr_block.to_string(), output);
    }
  }
}
