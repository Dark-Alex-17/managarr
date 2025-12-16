#[cfg(test)]
mod tests {
  use strum::IntoEnumIterator;

  use crate::app::App;
  use crate::models::servarr_data::sonarr::sonarr_data::{ActiveSonarrBlock, HISTORY_BLOCKS};
  use crate::models::sonarr_models::SonarrHistoryItem;
  use crate::models::stateful_table::StatefulTable;
  use crate::ui::DrawUi;
  use crate::ui::sonarr_ui::history::HistoryUi;
  use crate::ui::ui_test_utils::test_utils::render_to_string_with_app;

  #[test]
  fn test_history_ui_accepts() {
    ActiveSonarrBlock::iter().for_each(|active_sonarr_block| {
      if HISTORY_BLOCKS.contains(&active_sonarr_block) {
        assert!(HistoryUi::accepts(active_sonarr_block.into()));
      } else {
        assert!(!HistoryUi::accepts(active_sonarr_block.into()));
      }
    });
  }

  mod snapshot_tests {
    use crate::ui::ui_test_utils::test_utils::TerminalSize;

    use super::*;

    #[test]
    fn test_history_ui_renders_loading_state() {
      let mut app = App::test_default();
      app.is_loading = true;
      app.push_navigation_stack(ActiveSonarrBlock::History.into());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        HistoryUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }

    #[test]
    fn test_history_ui_renders_empty_history() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveSonarrBlock::History.into());
      app.data.sonarr_data.history = StatefulTable::default();

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        HistoryUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }

    #[test]
    fn test_history_ui_renders_with_history_items() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveSonarrBlock::History.into());
      app.data.sonarr_data.history = StatefulTable::default();
      app.data.sonarr_data.history.set_items(vec![
        SonarrHistoryItem {
          id: 1,
          source_title: "Test.Series.S01E01".to_owned().into(),
          ..SonarrHistoryItem::default()
        },
        SonarrHistoryItem {
          id: 2,
          source_title: "Another.Series.S02E05".to_owned().into(),
          ..SonarrHistoryItem::default()
        },
      ]);

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        HistoryUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }
  }
}
