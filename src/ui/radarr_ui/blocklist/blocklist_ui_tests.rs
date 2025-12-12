#[cfg(test)]
mod tests {
  use strum::IntoEnumIterator;

  use crate::app::App;
  use crate::models::radarr_models::BlocklistItem;
  use crate::models::servarr_data::radarr::radarr_data::{ActiveRadarrBlock, BLOCKLIST_BLOCKS};
  use crate::models::stateful_table::StatefulTable;
  use crate::ui::DrawUi;
  use crate::ui::radarr_ui::blocklist::BlocklistUi;
  use crate::ui::ui_test_utils::test_utils::render_to_string_with_app;

  #[test]
  fn test_blocklist_ui_accepts() {
    ActiveRadarrBlock::iter().for_each(|active_radarr_block| {
      if BLOCKLIST_BLOCKS.contains(&active_radarr_block) {
        assert!(BlocklistUi::accepts(active_radarr_block.into()));
      } else {
        assert!(!BlocklistUi::accepts(active_radarr_block.into()));
      }
    });
  }

  #[test]
  fn test_blocklist_ui_renders_loading_state() {
    let mut app = App::test_default();
    app.is_loading = true;
    app.push_navigation_stack(ActiveRadarrBlock::Blocklist.into());

    let output = render_to_string_with_app(120, 30, &mut app, |f, app| {
      BlocklistUi::draw(f, app, f.area());
    });

    insta::assert_snapshot!(output);
  }

  #[test]
  fn test_blocklist_ui_renders_empty_blocklist() {
    let mut app = App::test_default();
    app.push_navigation_stack(ActiveRadarrBlock::Blocklist.into());
    app.data.radarr_data.blocklist = StatefulTable::default();

    let output = render_to_string_with_app(120, 30, &mut app, |f, app| {
      BlocklistUi::draw(f, app, f.area());
    });

    insta::assert_snapshot!(output);
  }

  #[test]
  fn test_blocklist_ui_renders_with_blocklist_items() {
    let mut app = App::test_default();
    app.push_navigation_stack(ActiveRadarrBlock::Blocklist.into());
    app.data.radarr_data.blocklist = StatefulTable::default();
    app.data.radarr_data.blocklist.set_items(vec![
      BlocklistItem {
        id: 1,
        source_title: "Test.Movie.2023.1080p".to_owned(),
        ..BlocklistItem::default()
      },
      BlocklistItem {
        id: 2,
        source_title: "Another.Movie.2023.720p".to_owned(),
        ..BlocklistItem::default()
      },
    ]);

    let output = render_to_string_with_app(120, 30, &mut app, |f, app| {
      BlocklistUi::draw(f, app, f.area());
    });

    insta::assert_snapshot!(output);
  }
}
