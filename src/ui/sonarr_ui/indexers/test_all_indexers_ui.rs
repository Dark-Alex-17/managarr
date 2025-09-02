use crate::app::App;
use crate::models::servarr_data::modals::IndexerTestResultModalItem;
use crate::models::servarr_data::sonarr::sonarr_data::ActiveSonarrBlock;
use crate::models::Route;
use crate::ui::styles::ManagarrStyle;
use crate::ui::utils::{get_width_from_percentage, title_block};
use crate::ui::widgets::managarr_table::ManagarrTable;
use crate::ui::widgets::popup::Size;
use crate::ui::{draw_popup, DrawUi};
use ratatui::layout::{Constraint, Rect};
use ratatui::widgets::{Cell, Row};
use ratatui::Frame;

#[cfg(test)]
#[path = "test_all_indexers_ui_tests.rs"]
mod test_all_indexers_ui_tests;

pub(super) struct TestAllIndexersUi;

impl DrawUi for TestAllIndexersUi {
  fn accepts(route: Route) -> bool {
    if let Route::Sonarr(active_sonarr_block, _) = route {
      return active_sonarr_block == ActiveSonarrBlock::TestAllIndexers;
    }

    false
  }

  fn draw(f: &mut Frame<'_>, app: &mut App<'_>, _area: Rect) {
    draw_popup(f, app, draw_test_all_indexers_test_results, Size::Large);
  }
}

fn draw_test_all_indexers_test_results(f: &mut Frame<'_>, app: &mut App<'_>, area: Rect) {
  let is_loading = app.is_loading || app.data.sonarr_data.indexer_test_all_results.is_none();
  let current_selection =
    if let Some(test_all_results) = app.data.sonarr_data.indexer_test_all_results.as_ref() {
      test_all_results.current_selection().clone()
    } else {
      IndexerTestResultModalItem::default()
    };
  f.render_widget(title_block("Test All Indexers"), area);
  let test_results_row_mapping = |result: &IndexerTestResultModalItem| {
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
  };

  let indexers_test_results_table = ManagarrTable::new(
    app.data.sonarr_data.indexer_test_all_results.as_mut(),
    test_results_row_mapping,
  )
  .loading(is_loading)
  .margin(1)
  .headers(["Indexer", "Pass/Fail", "Failure Messages"])
  .constraints([
    Constraint::Percentage(20),
    Constraint::Percentage(10),
    Constraint::Percentage(70),
  ]);

  f.render_widget(indexers_test_results_table, area);
}
