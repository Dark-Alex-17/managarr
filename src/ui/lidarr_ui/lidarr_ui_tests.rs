#[cfg(test)]
mod tests {
  use strum::IntoEnumIterator;

  use crate::models::Route;
  use crate::models::servarr_data::lidarr::lidarr_data::ActiveLidarrBlock;
  use crate::ui::DrawUi;
  use crate::ui::lidarr_ui::LidarrUi;

  #[test]
  fn test_lidarr_ui_accepts() {
    for lidarr_block in ActiveLidarrBlock::iter() {
      assert!(LidarrUi::accepts(Route::Lidarr(lidarr_block, None)));
    }
  }

  mod snapshot_tests {
    use super::*;
    use crate::app::App;
    use crate::ui::ui_test_utils::test_utils::{TerminalSize, render_to_string_with_app};
    use rstest::rstest;

    #[rstest]
    #[case(ActiveLidarrBlock::Artists, 0)]
    #[case(ActiveLidarrBlock::Downloads, 1)]
    #[case(ActiveLidarrBlock::History, 2)]
    #[case(ActiveLidarrBlock::RootFolders, 3)]
    fn test_lidarr_ui_renders_lidarr_tabs(
      #[case] active_lidarr_block: ActiveLidarrBlock,
      #[case] index: usize,
    ) {
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack(active_lidarr_block.into());
      app.data.lidarr_data.main_tabs.set_index(index);

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        LidarrUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(format!("lidarr_tabs_{active_lidarr_block}"), output);
    }
  }
}
