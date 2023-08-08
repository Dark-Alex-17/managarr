use crate::app::radarr::ActiveRadarrBlock;
use crate::app::App;
use crate::event::Key;
use crate::handlers::KeyEventHandler;
use crate::models::Scrollable;

pub(super) struct CollectionDetailsHandler<'a> {
  key: &'a Key,
  app: &'a mut App,
  active_radarr_block: &'a ActiveRadarrBlock,
}

impl<'a> KeyEventHandler<'a, ActiveRadarrBlock> for CollectionDetailsHandler<'a> {
  fn with(
    key: &'a Key,
    app: &'a mut App,
    active_block: &'a ActiveRadarrBlock,
  ) -> CollectionDetailsHandler<'a> {
    CollectionDetailsHandler {
      key,
      app,
      active_radarr_block: active_block,
    }
  }

  fn get_key(&self) -> &Key {
    self.key
  }

  fn handle_scroll_up(&mut self) {
    if ActiveRadarrBlock::CollectionDetails == *self.active_radarr_block {
      self.app.data.radarr_data.collection_movies.scroll_up()
    }
  }

  fn handle_scroll_down(&mut self) {
    if ActiveRadarrBlock::CollectionDetails == *self.active_radarr_block {
      self.app.data.radarr_data.collection_movies.scroll_down()
    }
  }

  fn handle_home(&mut self) {
    if ActiveRadarrBlock::CollectionDetails == *self.active_radarr_block {
      self.app.data.radarr_data.collection_movies.scroll_to_top();
    }
  }

  fn handle_end(&mut self) {
    if ActiveRadarrBlock::CollectionDetails == *self.active_radarr_block {
      self
        .app
        .data
        .radarr_data
        .collection_movies
        .scroll_to_bottom();
    }
  }

  fn handle_delete(&mut self) {}

  fn handle_left_right_action(&mut self) {}

  fn handle_submit(&mut self) {
    if ActiveRadarrBlock::CollectionDetails == *self.active_radarr_block {
      self
        .app
        .push_navigation_stack(ActiveRadarrBlock::ViewMovieOverview.into())
    }
  }

  fn handle_esc(&mut self) {
    match self.active_radarr_block {
      ActiveRadarrBlock::CollectionDetails => {
        self.app.pop_navigation_stack();
        self.app.data.radarr_data.reset_movie_collection_table();
      }
      ActiveRadarrBlock::ViewMovieOverview => self.app.pop_navigation_stack(),
      _ => (),
    }
  }

  fn handle_char_key_event(&mut self) {}
}
