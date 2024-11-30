use crate::app::key_binding::DEFAULT_KEYBINDINGS;
use crate::app::App;
use crate::event::Key;
use crate::handlers::KeyEventHandler;
use crate::models::servarr_data::radarr::radarr_data::{
  ActiveRadarrBlock, ADD_MOVIE_SELECTION_BLOCKS, COLLECTION_DETAILS_BLOCKS,
  EDIT_COLLECTION_SELECTION_BLOCKS,
};
use crate::models::stateful_table::StatefulTable;
use crate::models::{BlockSelectionState, Scrollable};

#[cfg(test)]
#[path = "collection_details_handler_tests.rs"]
mod collection_details_handler_tests;

pub(super) struct CollectionDetailsHandler<'a, 'b> {
  key: Key,
  app: &'a mut App<'b>,
  active_radarr_block: ActiveRadarrBlock,
  _context: Option<ActiveRadarrBlock>,
}

impl<'a, 'b> KeyEventHandler<'a, 'b, ActiveRadarrBlock> for CollectionDetailsHandler<'a, 'b> {
  fn accepts(active_block: ActiveRadarrBlock) -> bool {
    COLLECTION_DETAILS_BLOCKS.contains(&active_block)
  }

  fn with(
    key: Key,
    app: &'a mut App<'b>,
    active_block: ActiveRadarrBlock,
    _context: Option<ActiveRadarrBlock>,
  ) -> CollectionDetailsHandler<'a, 'b> {
    CollectionDetailsHandler {
      key,
      app,
      active_radarr_block: active_block,
      _context,
    }
  }

  fn get_key(&self) -> Key {
    self.key
  }

  fn is_ready(&self) -> bool {
    !self.app.is_loading && !self.app.data.radarr_data.collection_movies.is_empty()
  }

  fn handle_scroll_up(&mut self) {
    if ActiveRadarrBlock::CollectionDetails == self.active_radarr_block {
      self.app.data.radarr_data.collection_movies.scroll_up()
    }
  }

  fn handle_scroll_down(&mut self) {
    if ActiveRadarrBlock::CollectionDetails == self.active_radarr_block {
      self.app.data.radarr_data.collection_movies.scroll_down()
    }
  }

  fn handle_home(&mut self) {
    if ActiveRadarrBlock::CollectionDetails == self.active_radarr_block {
      self.app.data.radarr_data.collection_movies.scroll_to_top();
    }
  }

  fn handle_end(&mut self) {
    if ActiveRadarrBlock::CollectionDetails == self.active_radarr_block {
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
    if ActiveRadarrBlock::CollectionDetails == self.active_radarr_block {
      let tmdb_id = self
        .app
        .data
        .radarr_data
        .collection_movies
        .current_selection()
        .tmdb_id;

      if self
        .app
        .data
        .radarr_data
        .movies
        .items
        .iter()
        .any(|movie| movie.tmdb_id == tmdb_id)
      {
        self
          .app
          .push_navigation_stack(ActiveRadarrBlock::ViewMovieOverview.into());
      } else {
        self.app.push_navigation_stack(
          (
            ActiveRadarrBlock::AddMoviePrompt,
            Some(ActiveRadarrBlock::CollectionDetails),
          )
            .into(),
        );
        self.app.data.radarr_data.selected_block =
          BlockSelectionState::new(ADD_MOVIE_SELECTION_BLOCKS);
        self.app.data.radarr_data.add_movie_modal = Some((&self.app.data.radarr_data).into());
      }
    }
  }

  fn handle_esc(&mut self) {
    match self.active_radarr_block {
      ActiveRadarrBlock::CollectionDetails => {
        self.app.data.radarr_data.collection_movies = StatefulTable::default();
        self.app.pop_navigation_stack();
      }
      ActiveRadarrBlock::ViewMovieOverview => self.app.pop_navigation_stack(),
      _ => (),
    }
  }

  fn handle_char_key_event(&mut self) {
    if self.active_radarr_block == ActiveRadarrBlock::CollectionDetails
      && self.key == DEFAULT_KEYBINDINGS.edit.key
    {
      self.app.push_navigation_stack(
        (
          ActiveRadarrBlock::EditCollectionPrompt,
          Some(self.active_radarr_block),
        )
          .into(),
      );
      self.app.data.radarr_data.edit_collection_modal = Some((&self.app.data.radarr_data).into());
      self.app.data.radarr_data.selected_block =
        BlockSelectionState::new(EDIT_COLLECTION_SELECTION_BLOCKS);
    }
  }
}
