use crate::app::App;
use crate::event::Key;
use crate::handlers::KeyEventHandler;
use crate::models::servarr_data::radarr::radarr_data::ActiveRadarrBlock;
use crate::models::{Route, Scrollable, ScrollableText};

#[cfg(test)]
#[path = "movie_overview_handler_tests.rs"]
mod movie_overview_handler_tests;

pub(super) struct MovieOverviewHandler<'a, 'b> {
  key: Key,
  app: &'a mut App<'b>,
  _active_radarr_block: ActiveRadarrBlock,
  _context: Option<ActiveRadarrBlock>,
}

impl MovieOverviewHandler<'_, '_> {
  fn overview(&mut self) -> &mut ScrollableText {
    &mut self
      .app
      .data
      .radarr_data
      .movie_overview_modal
      .as_mut()
      .expect("Movie overview modal is undefined")
      .overview
  }
}

impl<'a, 'b> KeyEventHandler<'a, 'b, ActiveRadarrBlock> for MovieOverviewHandler<'a, 'b> {
  fn accepts(active_block: ActiveRadarrBlock) -> bool {
    active_block == ActiveRadarrBlock::ViewMovieOverview
  }

  fn ignore_special_keys(&self) -> bool {
    self.app.ignore_special_keys_for_textbox_input
  }

  fn new(
    key: Key,
    app: &'a mut App<'b>,
    active_block: ActiveRadarrBlock,
    _context: Option<ActiveRadarrBlock>,
  ) -> MovieOverviewHandler<'a, 'b> {
    MovieOverviewHandler {
      key,
      app,
      _active_radarr_block: active_block,
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

    self.app.data.radarr_data.movie_overview_modal.is_some()
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
    self.app.data.radarr_data.movie_overview_modal = None;
  }

  fn handle_char_key_event(&mut self) {}

  fn app_mut(&mut self) -> &mut App<'b> {
    self.app
  }

  fn current_route(&self) -> Route {
    self.app.get_current_route()
  }
}
