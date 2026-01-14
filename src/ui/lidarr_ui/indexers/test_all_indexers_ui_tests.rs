#[cfg(test)]
mod tests {
  use strum::IntoEnumIterator;

  use crate::app::App;
  use crate::models::servarr_data::lidarr::lidarr_data::ActiveLidarrBlock;
  use crate::ui::DrawUi;
  use crate::ui::lidarr_ui::indexers::test_all_indexers_ui::TestAllIndexersUi;
  use crate::ui::ui_test_utils::test_utils::render_to_string_with_app;

  #[test]
  fn test_test_all_indexers_ui_accepts() {
    ActiveLidarrBlock::iter().for_each(|active_lidarr_block| {
      if active_lidarr_block == ActiveLidarrBlock::TestAllIndexers {
        assert!(TestAllIndexersUi::accepts(active_lidarr_block.into()));
      } else {
        assert!(!TestAllIndexersUi::accepts(active_lidarr_block.into()));
      }
    });
  }

  mod snapshot_tests {
    use crate::ui::ui_test_utils::test_utils::TerminalSize;

    use super::*;

    #[test]
    fn test_test_all_indexers_ui_renders_loading() {
      let mut app = App::test_default_fully_populated();
      app.is_loading = true;
      app.push_navigation_stack(ActiveLidarrBlock::TestAllIndexers.into());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        TestAllIndexersUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }

    #[test]
    fn test_test_all_indexers_ui_renders() {
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack(ActiveLidarrBlock::TestAllIndexers.into());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        TestAllIndexersUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }
  }
}
