#[cfg(test)]
mod tests {
  use strum::IntoEnumIterator;

  use crate::app::App;
  use crate::models::servarr_data::lidarr::lidarr_data::{ActiveLidarrBlock, HISTORY_BLOCKS};
  use crate::ui::DrawUi;
  use crate::ui::lidarr_ui::history::HistoryUi;
  use crate::ui::ui_test_utils::test_utils::render_to_string_with_app;

  #[test]
  fn test_history_ui_accepts() {
    ActiveLidarrBlock::iter().for_each(|active_lidarr_block| {
      if HISTORY_BLOCKS.contains(&active_lidarr_block) {
        assert!(HistoryUi::accepts(active_lidarr_block.into()));
      } else {
        assert!(!HistoryUi::accepts(active_lidarr_block.into()));
      }
    });
  }

  mod snapshot_tests {
    use crate::ui::ui_test_utils::test_utils::TerminalSize;
    use rstest::rstest;

    use super::*;

    #[test]
    fn test_history_ui_renders_loading() {
      let mut app = App::test_default();
      app.is_loading = true;
      app.push_navigation_stack(ActiveLidarrBlock::History.into());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        HistoryUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }

    #[rstest]
    fn test_history_ui_renders_empty(
      #[values(ActiveLidarrBlock::History, ActiveLidarrBlock::HistoryItemDetails)]
      active_lidarr_block: ActiveLidarrBlock,
    ) {
      let mut app = App::test_default();
      app.push_navigation_stack(active_lidarr_block.into());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        HistoryUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(format!("loading_history_tab_{active_lidarr_block}"), output);
    }

    #[rstest]
    fn test_history_ui_renders(
      #[values(
        ActiveLidarrBlock::History,
        ActiveLidarrBlock::HistoryItemDetails,
        ActiveLidarrBlock::HistorySortPrompt,
        ActiveLidarrBlock::FilterHistory,
        ActiveLidarrBlock::FilterHistoryError,
        ActiveLidarrBlock::SearchHistory,
        ActiveLidarrBlock::SearchHistoryError
      )]
      active_lidarr_block: ActiveLidarrBlock,
    ) {
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack(active_lidarr_block.into());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        HistoryUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(format!("history_tab_{active_lidarr_block}"), output);
    }
  }
}
