#[cfg(test)]
mod tests {
  use strum::IntoEnumIterator;

  use crate::app::App;
  use crate::models::servarr_data::radarr::radarr_data::{ActiveRadarrBlock, HISTORY_BLOCKS};
  use crate::ui::DrawUi;
  use crate::ui::radarr_ui::history::HistoryUi;
  use crate::ui::ui_test_utils::test_utils::render_to_string_with_app;

  #[test]
  fn test_history_ui_accepts() {
    ActiveRadarrBlock::iter().for_each(|active_radarr_block| {
      if HISTORY_BLOCKS.contains(&active_radarr_block) {
        assert!(HistoryUi::accepts(active_radarr_block.into()));
      } else {
        assert!(!HistoryUi::accepts(active_radarr_block.into()));
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
      app.push_navigation_stack(ActiveRadarrBlock::History.into());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        HistoryUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }

    #[rstest]
    fn test_history_ui_renders_empty(
      #[values(ActiveRadarrBlock::History, ActiveRadarrBlock::HistoryItemDetails)]
      active_radarr_block: ActiveRadarrBlock,
    ) {
      let mut app = App::test_default();
      app.push_navigation_stack(active_radarr_block.into());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        HistoryUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(format!("loading_history_tab_{active_radarr_block}"), output);
    }

    #[rstest]
    fn test_history_ui_renders(
      #[values(
        ActiveRadarrBlock::History,
        ActiveRadarrBlock::HistoryItemDetails,
        ActiveRadarrBlock::HistorySortPrompt,
        ActiveRadarrBlock::FilterHistory,
        ActiveRadarrBlock::FilterHistoryError,
        ActiveRadarrBlock::SearchHistory,
        ActiveRadarrBlock::SearchHistoryError
      )]
      active_radarr_block: ActiveRadarrBlock,
    ) {
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack(active_radarr_block.into());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        HistoryUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(format!("history_tab_{active_radarr_block}"), output);
    }
  }
}
