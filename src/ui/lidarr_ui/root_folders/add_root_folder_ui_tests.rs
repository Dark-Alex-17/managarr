#[cfg(test)]
mod tests {
  use strum::IntoEnumIterator;

  use crate::app::App;
  use crate::models::BlockSelectionState;
  use crate::models::servarr_data::lidarr::lidarr_data::{
    ADD_ROOT_FOLDER_BLOCKS, ADD_ROOT_FOLDER_SELECTION_BLOCKS, ActiveLidarrBlock,
  };
  use crate::ui::DrawUi;
  use crate::ui::lidarr_ui::root_folders::add_root_folder_ui::AddRootFolderUi;
  use crate::ui::ui_test_utils::test_utils::render_to_string_with_app;

  #[test]
  fn test_add_root_folder_ui_accepts() {
    ActiveLidarrBlock::iter().for_each(|active_lidarr_block| {
      if ADD_ROOT_FOLDER_BLOCKS.contains(&active_lidarr_block) {
        assert!(AddRootFolderUi::accepts(active_lidarr_block.into()));
      } else {
        assert!(!AddRootFolderUi::accepts(active_lidarr_block.into()));
      }
    });
  }

  mod snapshot_tests {
    use crate::ui::ui_test_utils::test_utils::TerminalSize;
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(ActiveLidarrBlock::AddRootFolderPrompt)]
    #[case(ActiveLidarrBlock::AddRootFolderConfirmPrompt)]
    #[case(ActiveLidarrBlock::AddRootFolderSelectMonitor)]
    #[case(ActiveLidarrBlock::AddRootFolderSelectMonitorNewItems)]
    #[case(ActiveLidarrBlock::AddRootFolderSelectQualityProfile)]
    #[case(ActiveLidarrBlock::AddRootFolderSelectMetadataProfile)]
    fn test_add_root_folder_ui_renders(#[case] active_lidarr_block: ActiveLidarrBlock) {
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack(active_lidarr_block.into());
      app.data.lidarr_data.selected_block =
        BlockSelectionState::new(ADD_ROOT_FOLDER_SELECTION_BLOCKS);

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        AddRootFolderUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(format!("add_root_folder_{active_lidarr_block}"), output);
    }
  }
}
