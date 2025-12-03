use crate::app::App;
use crate::event::Key;
use crate::handlers::KeyEventHandler;
use crate::handlers::table_handler::TableHandlingConfig;
use crate::models::BlockSelectionState;
use crate::models::radarr_models::CollectionMovie;
use crate::models::servarr_data::radarr::radarr_data::{
  ADD_MOVIE_SELECTION_BLOCKS, ActiveRadarrBlock, COLLECTION_DETAILS_BLOCKS,
  EDIT_COLLECTION_SELECTION_BLOCKS,
};
use crate::models::stateful_table::StatefulTable;
use crate::{handle_table_events, matches_key};

#[cfg(test)]
#[path = "collection_details_handler_tests.rs"]
mod collection_details_handler_tests;

pub(super) struct CollectionDetailsHandler<'a, 'b> {
  key: Key,
  app: &'a mut App<'b>,
  active_radarr_block: ActiveRadarrBlock,
  _context: Option<ActiveRadarrBlock>,
}

impl CollectionDetailsHandler<'_, '_> {
  handle_table_events!(
    self,
    collection_movies,
    self.app.data.radarr_data.collection_movies,
    CollectionMovie
  );
}

impl<'a, 'b> KeyEventHandler<'a, 'b, ActiveRadarrBlock> for CollectionDetailsHandler<'a, 'b> {
  fn handle(&mut self) {
    let collection_movies_table_handling_config =
      TableHandlingConfig::new(ActiveRadarrBlock::CollectionDetails.into());

    if !self.handle_collection_movies_table_events(collection_movies_table_handling_config) {
      self.handle_key_event();
    }
  }

  fn accepts(active_block: ActiveRadarrBlock) -> bool {
    COLLECTION_DETAILS_BLOCKS.contains(&active_block)
  }

  fn ignore_special_keys(&self) -> bool {
    self.app.ignore_special_keys_for_textbox_input
  }

  fn new(
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

  fn handle_scroll_up(&mut self) {}

  fn handle_scroll_down(&mut self) {}

  fn handle_home(&mut self) {}

  fn handle_end(&mut self) {}

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
      && matches_key!(edit, self.key)
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
