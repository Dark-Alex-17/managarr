use crate::app::App;
use crate::event::Key;
use crate::handlers::KeyEventHandler;
use crate::matches_key;
use crate::models::servarr_data::readarr::readarr_data::{
  ActiveReadarrBlock, EDITION_DETAILS_BLOCKS,
};
use crate::models::{Route, Scrollable, ScrollableText};

#[cfg(test)]
#[path = "edition_details_handler_tests.rs"]
mod edition_details_handler_tests;

pub(super) struct EditionDetailsHandler<'a, 'b> {
  key: Key,
  app: &'a mut App<'b>,
  active_readarr_block: ActiveReadarrBlock,
  _context: Option<ActiveReadarrBlock>,
}

impl EditionDetailsHandler<'_, '_> {
  fn edition_details(&mut self) -> &mut ScrollableText {
    &mut self
      .app
      .data
      .readarr_data
      .book_details_modal
      .as_mut()
      .expect("Book details modal is undefined")
      .edition_details_modal
      .as_mut()
      .expect("Edition details modal is undefined")
      .edition_details
  }
}

impl<'a, 'b> KeyEventHandler<'a, 'b, ActiveReadarrBlock> for EditionDetailsHandler<'a, 'b> {
  fn accepts(active_block: ActiveReadarrBlock) -> bool {
    EDITION_DETAILS_BLOCKS.contains(&active_block)
  }

  fn ignore_special_keys(&self) -> bool {
    self.app.ignore_special_keys_for_textbox_input
  }

  fn new(
    key: Key,
    app: &'a mut App<'b>,
    active_block: ActiveReadarrBlock,
    _context: Option<ActiveReadarrBlock>,
  ) -> EditionDetailsHandler<'a, 'b> {
    EditionDetailsHandler {
      key,
      app,
      active_readarr_block: active_block,
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

    let Some(book_details_modal) = &self.app.data.readarr_data.book_details_modal else {
      return false;
    };

    book_details_modal.edition_details_modal.is_some()
  }

  fn handle_scroll_up(&mut self) {
    self.edition_details().scroll_up();
  }

  fn handle_scroll_down(&mut self) {
    self.edition_details().scroll_down();
  }

  fn handle_home(&mut self) {
    self.edition_details().scroll_to_top();
  }

  fn handle_end(&mut self) {
    self.edition_details().scroll_to_bottom();
  }

  fn handle_delete(&mut self) {}

  fn handle_left_right_action(&mut self) {}

  fn handle_submit(&mut self) {}

  fn handle_esc(&mut self) {
    self.app.pop_navigation_stack();
    self
      .app
      .data
      .readarr_data
      .book_details_modal
      .as_mut()
      .expect("Book details modal is undefined")
      .edition_details_modal = None;
  }

  fn handle_char_key_event(&mut self) {
    if matches_key!(refresh, self.key) {
      self
        .app
        .pop_and_push_navigation_stack(self.active_readarr_block.into());
    }
  }

  fn app_mut(&mut self) -> &mut App<'b> {
    self.app
  }

  fn current_route(&self) -> Route {
    self.app.get_current_route()
  }
}
