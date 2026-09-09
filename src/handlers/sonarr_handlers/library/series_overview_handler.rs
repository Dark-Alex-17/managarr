use crate::app::App;
use crate::event::Key;
use crate::handlers::KeyEventHandler;
use crate::matches_key;
use crate::models::servarr_data::sonarr::sonarr_data::{ActiveSonarrBlock, SERIES_OVERVIEW_BLOCKS};
use crate::models::{Route, Scrollable, ScrollableText};

#[cfg(test)]
#[path = "series_overview_handler_tests.rs"]
mod series_overview_handler_tests;

pub(super) struct SeriesOverviewHandler<'a, 'b> {
  key: Key,
  app: &'a mut App<'b>,
  active_sonarr_block: ActiveSonarrBlock,
  _context: Option<ActiveSonarrBlock>,
}

impl SeriesOverviewHandler<'_, '_> {
  fn overview(&mut self) -> &mut ScrollableText {
    &mut self
      .app
      .data
      .sonarr_data
      .series_overview_modal
      .as_mut()
      .expect("Series overview modal is undefined")
      .overview
  }
}

impl<'a, 'b> KeyEventHandler<'a, 'b, ActiveSonarrBlock> for SeriesOverviewHandler<'a, 'b> {
  fn accepts(active_block: ActiveSonarrBlock) -> bool {
    SERIES_OVERVIEW_BLOCKS.contains(&active_block)
  }

  fn ignore_special_keys(&self) -> bool {
    self.app.ignore_special_keys_for_textbox_input
  }

  fn new(
    key: Key,
    app: &'a mut App<'b>,
    active_block: ActiveSonarrBlock,
    _context: Option<ActiveSonarrBlock>,
  ) -> SeriesOverviewHandler<'a, 'b> {
    SeriesOverviewHandler {
      key,
      app,
      active_sonarr_block: active_block,
      _context,
    }
  }

  fn get_key(&self) -> Key {
    self.key
  }

  fn is_ready(&self) -> bool {
    if self.app.is_loading {
      return false;
    }

    self.app.data.sonarr_data.series_overview_modal.is_some()
  }

  fn handle_scroll_up(&mut self) {
    self.overview().scroll_up();
  }

  fn handle_scroll_down(&mut self) {
    self.overview().scroll_down();
  }

  fn handle_home(&mut self) {
    self.overview().scroll_to_top();
  }

  fn handle_end(&mut self) {
    self.overview().scroll_to_bottom();
  }

  fn handle_delete(&mut self) {}

  fn handle_left_right_action(&mut self) {}

  fn handle_submit(&mut self) {}

  fn handle_esc(&mut self) {
    self.app.pop_navigation_stack();
    self.app.data.sonarr_data.series_overview_modal = None;
  }

  fn handle_char_key_event(&mut self) {
    if matches_key!(refresh, self.key) {
      self
        .app
        .pop_and_push_navigation_stack(self.active_sonarr_block.into());
    }
  }

  fn app_mut(&mut self) -> &mut App<'b> {
    self.app
  }

  fn current_route(&self) -> Route {
    self.app.get_current_route()
  }
}
