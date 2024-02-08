use crate::app::context_clues::{build_context_clue_string, BARE_POPUP_CONTEXT_CLUES};
use crate::app::App;
use crate::models::servarr_data::radarr::modals::IndexerTestResultModalItem;
use crate::models::servarr_data::radarr::radarr_data::ActiveRadarrBlock;
use crate::models::Route;
use crate::ui::radarr_ui::indexers::draw_indexers;
use crate::ui::styles::ManagarrStyle;
use crate::ui::utils::{borderless_block, get_width_from_percentage, title_block};
use crate::ui::{
  draw_help_footer_and_get_content_area, draw_large_popup_over, draw_table, DrawUi, TableProps,
};
use ratatui::layout::{Constraint, Rect};
use ratatui::widgets::{Cell, Row};
use ratatui::Frame;

#[cfg(test)]
#[path = "test_all_indexers_ui_tests.rs"]
mod test_all_indexers_ui_tests;

pub(super) struct TestAllIndexersUi;

impl DrawUi for TestAllIndexersUi {
  fn accepts(route: Route) -> bool {
    if let Route::Radarr(active_radarr_block, _) = route {
      return active_radarr_block == ActiveRadarrBlock::TestAllIndexers;
    }

    false
  }

  fn draw(f: &mut Frame<'_>, app: &mut App<'_>, area: Rect) {
    draw_large_popup_over(
      f,
      app,
      area,
      draw_indexers,
      draw_test_all_indexers_test_results,
    );
  }
}

fn draw_test_all_indexers_test_results(f: &mut Frame<'_>, app: &mut App<'_>, area: Rect) {
  let current_selection =
    if let Some(test_all_results) = app.data.radarr_data.indexer_test_all_results.as_ref() {
      test_all_results.current_selection().clone()
    } else {
      IndexerTestResultModalItem::default()
    };
  f.render_widget(title_block("Test All Indexers"), area);
  let help = Some(format!(
    "<↑↓> scroll | {}",
    build_context_clue_string(&BARE_POPUP_CONTEXT_CLUES)
  ));
  let content_area = draw_help_footer_and_get_content_area(f, area, help);

  draw_table(
    f,
    content_area,
    borderless_block(),
    TableProps {
      content: app.data.radarr_data.indexer_test_all_results.as_mut(),
      wrapped_content: None,
      table_headers: vec!["Indexer", "Pass/Fail", "Failure Messages"],
      constraints: vec![
        Constraint::Percentage(20),
        Constraint::Percentage(10),
        Constraint::Percentage(70),
      ],
      help: None,
    },
    |result| {
      result.validation_failures.scroll_left_or_reset(
        get_width_from_percentage(area, 86),
        *result == current_selection,
        app.tick_count % app.ticks_until_scroll == 0,
      );
      let pass_fail = if result.is_valid { "✔" } else { "❌" };
      let row = Row::new(vec![
        Cell::from(result.name.to_owned()),
        Cell::from(pass_fail.to_owned()),
        Cell::from(result.validation_failures.to_string()),
      ]);

      if result.is_valid {
        row.success()
      } else {
        row.failure()
      }
    },
    app.is_loading,
    true,
  );
}
