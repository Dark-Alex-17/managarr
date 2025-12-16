#[cfg(test)]
mod tests {
  use strum::IntoEnumIterator;

  use crate::app::App;
  use crate::models::servarr_data::sonarr::sonarr_data::{ActiveSonarrBlock, BLOCKLIST_BLOCKS};
  use crate::models::sonarr_models::BlocklistItem;
  use crate::models::stateful_table::StatefulTable;
  use crate::ui::DrawUi;
  use crate::ui::sonarr_ui::blocklist::BlocklistUi;
  use crate::ui::ui_test_utils::test_utils::render_to_string_with_app;

  #[test]
  fn test_blocklist_ui_accepts() {
    ActiveSonarrBlock::iter().for_each(|active_sonarr_block| {
      if BLOCKLIST_BLOCKS.contains(&active_sonarr_block) {
        assert!(BlocklistUi::accepts(active_sonarr_block.into()));
      } else {
        assert!(!BlocklistUi::accepts(active_sonarr_block.into()));
      }
    });
  }

  mod snapshot_tests {
    use crate::ui::ui_test_utils::test_utils::TerminalSize;

    use super::*;

    #[test]
    fn test_blocklist_ui_renders_loading_state() {
      let mut app = App::test_default();
      app.is_loading = true;
      app.push_navigation_stack(ActiveSonarrBlock::Blocklist.into());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        BlocklistUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }

    #[test]
    fn test_blocklist_ui_renders_empty_blocklist() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveSonarrBlock::Blocklist.into());
      app.data.sonarr_data.blocklist = StatefulTable::default();

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        BlocklistUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }

    #[test]
    fn test_blocklist_ui_renders_with_blocklist_items() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveSonarrBlock::Blocklist.into());
      app.data.sonarr_data.blocklist = StatefulTable::default();
      app.data.sonarr_data.blocklist.set_items(vec![
        BlocklistItem {
          id: 1,
          source_title: "Test.Series.S01E01.1080p".to_owned(),
          ..BlocklistItem::default()
        },
        BlocklistItem {
          id: 2,
          source_title: "Another.Series.S02E05.720p".to_owned(),
          ..BlocklistItem::default()
        },
      ]);

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        BlocklistUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }
  }
}
