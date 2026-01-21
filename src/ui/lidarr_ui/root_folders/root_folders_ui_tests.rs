#[cfg(test)]
mod tests {
  use strum::IntoEnumIterator;

  use crate::app::App;
  use crate::models::servarr_data::lidarr::lidarr_data::{
    ADD_ROOT_FOLDER_BLOCKS, ActiveLidarrBlock, ROOT_FOLDERS_BLOCKS,
  };
  use crate::ui::DrawUi;
  use crate::ui::lidarr_ui::root_folders::RootFoldersUi;
  use crate::ui::ui_test_utils::test_utils::render_to_string_with_app;

  #[test]
  fn test_root_folders_ui_accepts() {
    ActiveLidarrBlock::iter().for_each(|active_lidarr_block| {
      if ROOT_FOLDERS_BLOCKS.contains(&active_lidarr_block)
        || ADD_ROOT_FOLDER_BLOCKS.contains(&active_lidarr_block)
      {
        assert!(RootFoldersUi::accepts(active_lidarr_block.into()));
      } else {
        assert!(!RootFoldersUi::accepts(active_lidarr_block.into()));
      }
    });
  }

  mod snapshot_tests {
    use super::*;
    use crate::models::BlockSelectionState;
    use crate::models::servarr_data::lidarr::lidarr_data::ADD_ROOT_FOLDER_SELECTION_BLOCKS;
    use crate::ui::ui_test_utils::test_utils::TerminalSize;
    use rstest::rstest;

    #[test]
    fn test_root_folders_ui_renders_loading() {
      let mut app = App::test_default();
      app.is_loading = true;
      app.push_navigation_stack(ActiveLidarrBlock::RootFolders.into());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        RootFoldersUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }

    #[test]
    fn test_root_folders_ui_renders_empty_root_folders() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveLidarrBlock::RootFolders.into());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        RootFoldersUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }

    #[rstest]
    fn test_root_folders_ui_renders_root_folders_tab(
      #[values(
        ActiveLidarrBlock::RootFolders,
        ActiveLidarrBlock::DeleteRootFolderPrompt
      )]
      active_lidarr_block: ActiveLidarrBlock,
    ) {
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack(active_lidarr_block.into());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        RootFoldersUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(active_lidarr_block.to_string(), output);
    }

    #[rstest]
    fn test_root_folders_ui_renders_add_root_folders_popup_over_root_folders_table(
      #[values(
        ActiveLidarrBlock::AddRootFolderPrompt,
        ActiveLidarrBlock::AddRootFolderConfirmPrompt,
        ActiveLidarrBlock::AddRootFolderSelectMonitor,
        ActiveLidarrBlock::AddRootFolderSelectMonitorNewItems,
        ActiveLidarrBlock::AddRootFolderSelectQualityProfile,
        ActiveLidarrBlock::AddRootFolderSelectMetadataProfile
      )]
      active_lidarr_block: ActiveLidarrBlock,
    ) {
      let mut app = App::test_default_fully_populated();
      app.data.lidarr_data.selected_block =
        BlockSelectionState::new(ADD_ROOT_FOLDER_SELECTION_BLOCKS);
      app.push_navigation_stack(ActiveLidarrBlock::AddRootFolderPrompt.into());
      app.push_navigation_stack(active_lidarr_block.into());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        RootFoldersUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(active_lidarr_block.to_string(), output);
    }
  }
}
