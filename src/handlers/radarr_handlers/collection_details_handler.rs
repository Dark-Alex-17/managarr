use crate::app::key_binding::DEFAULT_KEYBINDINGS;
use crate::app::radarr::ActiveRadarrBlock;
use crate::app::App;
use crate::event::Key;
use crate::handlers::KeyEventHandler;
use crate::models::Scrollable;

pub(super) struct CollectionDetailsHandler<'a> {
  key: &'a Key,
  app: &'a mut App,
  active_radarr_block: &'a ActiveRadarrBlock,
  _context: &'a Option<ActiveRadarrBlock>,
}

impl<'a> KeyEventHandler<'a, ActiveRadarrBlock> for CollectionDetailsHandler<'a> {
  fn with(
    key: &'a Key,
    app: &'a mut App,
    active_block: &'a ActiveRadarrBlock,
    _context: &'a Option<ActiveRadarrBlock>,
  ) -> CollectionDetailsHandler<'a> {
    CollectionDetailsHandler {
      key,
      app,
      active_radarr_block: active_block,
      _context,
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
      let tmdb_id = self
        .app
        .data
        .radarr_data
        .collection_movies
        .current_selection()
        .tmdb_id
        .clone();

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
        self.app.data.radarr_data.selected_block = ActiveRadarrBlock::EditMovieToggleMonitored;
        self.app.data.radarr_data.populate_preferences_lists();
      }
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

  fn handle_char_key_event(&mut self) {
    if *self.active_radarr_block == ActiveRadarrBlock::CollectionDetails
      && *self.key == DEFAULT_KEYBINDINGS.edit.key
    {
      self.app.push_navigation_stack(
        (
          ActiveRadarrBlock::EditCollectionPrompt,
          Some(*self.active_radarr_block),
        )
          .into(),
      );
      self.app.data.radarr_data.populate_edit_collection_fields();
      self.app.data.radarr_data.selected_block = ActiveRadarrBlock::EditCollectionToggleMonitored;
    }
  }
}

#[cfg(test)]
mod tests {
  use pretty_assertions::assert_str_eq;

  use crate::app::key_binding::DEFAULT_KEYBINDINGS;
  use crate::app::radarr::ActiveRadarrBlock;
  use crate::app::App;
  use crate::event::Key;
  use crate::handlers::radarr_handlers::collection_details_handler::CollectionDetailsHandler;
  use crate::handlers::KeyEventHandler;
  use crate::models::radarr_models::CollectionMovie;
  use crate::models::HorizontallyScrollableText;

  mod test_handle_scroll_up_and_down {
    use rstest::rstest;

    use crate::{simple_stateful_iterable_vec, test_iterable_scroll};

    use super::*;

    test_iterable_scroll!(
      test_collection_details_scroll,
      CollectionDetailsHandler,
      collection_movies,
      simple_stateful_iterable_vec!(CollectionMovie, HorizontallyScrollableText),
      ActiveRadarrBlock::CollectionDetails,
      None,
      title,
      to_string
    );
  }

  mod test_handle_home_end {
    use crate::{extended_stateful_iterable_vec, test_iterable_home_and_end};

    use super::*;

    test_iterable_home_and_end!(
      test_collection_details_home_end,
      CollectionDetailsHandler,
      collection_movies,
      extended_stateful_iterable_vec!(CollectionMovie, HorizontallyScrollableText),
      ActiveRadarrBlock::CollectionDetails,
      None,
      title,
      to_string
    );
  }

  mod test_handle_submit {
    use bimap::BiMap;
    use pretty_assertions::assert_eq;

    use crate::models::radarr_models::Movie;

    use super::*;

    const SUBMIT_KEY: Key = DEFAULT_KEYBINDINGS.submit.key;

    #[test]
    fn test_collection_details_submit() {
      let mut app = App::default();
      app
        .data
        .radarr_data
        .collection_movies
        .set_items(vec![CollectionMovie::default()]);
      app.data.radarr_data.quality_profile_map =
        BiMap::from_iter([(1, "B - Test 2".to_owned()), (0, "A - Test 1".to_owned())]);
      app.data.radarr_data.selected_block = ActiveRadarrBlock::AddMovieConfirmPrompt;

      CollectionDetailsHandler::with(
        &SUBMIT_KEY,
        &mut app,
        &ActiveRadarrBlock::CollectionDetails,
        &None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        &(
          ActiveRadarrBlock::AddMoviePrompt,
          Some(ActiveRadarrBlock::CollectionDetails)
        )
          .into()
      );
      assert!(!app.data.radarr_data.monitor_list.items.is_empty());
      assert_eq!(
        app.data.radarr_data.selected_block,
        ActiveRadarrBlock::EditMovieToggleMonitored
      );
      assert!(!app
        .data
        .radarr_data
        .minimum_availability_list
        .items
        .is_empty());
      assert!(!app.data.radarr_data.quality_profile_list.items.is_empty());
      assert_str_eq!(
        app
          .data
          .radarr_data
          .quality_profile_list
          .current_selection(),
        "A - Test 1"
      );
    }

    #[test]
    fn test_collection_details_submit_movie_already_in_library() {
      let mut app = App::default();
      app
        .data
        .radarr_data
        .collection_movies
        .set_items(vec![CollectionMovie::default()]);
      app
        .data
        .radarr_data
        .movies
        .set_items(vec![Movie::default()]);

      CollectionDetailsHandler::with(
        &SUBMIT_KEY,
        &mut app,
        &ActiveRadarrBlock::CollectionDetails,
        &None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        &ActiveRadarrBlock::ViewMovieOverview.into()
      );
    }
  }

  mod test_handle_esc {
    use pretty_assertions::assert_eq;

    use super::*;

    const ESC_KEY: Key = DEFAULT_KEYBINDINGS.esc.key;

    #[test]
    fn test_esc_collection_details() {
      let mut app = App::default();
      app.push_navigation_stack(ActiveRadarrBlock::Collections.into());
      app.push_navigation_stack(ActiveRadarrBlock::CollectionDetails.into());
      app
        .data
        .radarr_data
        .collection_movies
        .set_items(vec![CollectionMovie::default()]);

      CollectionDetailsHandler::with(
        &ESC_KEY,
        &mut app,
        &ActiveRadarrBlock::CollectionDetails,
        &None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        &ActiveRadarrBlock::Collections.into()
      );
      assert!(app.data.radarr_data.collection_movies.items.is_empty());
    }

    #[test]
    fn test_esc_view_movie_overview() {
      let mut app = App::default();
      app.push_navigation_stack(ActiveRadarrBlock::CollectionDetails.into());
      app.push_navigation_stack(ActiveRadarrBlock::ViewMovieOverview.into());

      CollectionDetailsHandler::with(
        &ESC_KEY,
        &mut app,
        &ActiveRadarrBlock::ViewMovieOverview,
        &None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        &ActiveRadarrBlock::CollectionDetails.into()
      );
    }
  }

  mod test_handle_key_char {
    use bimap::BiMap;
    use pretty_assertions::{assert_eq, assert_str_eq};
    use serde_json::Number;
    use strum::IntoEnumIterator;

    use crate::app::radarr::radarr_test_utils::create_test_radarr_data;
    use crate::app::radarr::RadarrData;
    use crate::models::radarr_models::{Collection, MinimumAvailability};
    use crate::models::HorizontallyScrollableText;
    use crate::models::StatefulTable;
    use crate::test_edit_collection_key;

    use super::*;

    #[test]
    fn test_edit_key() {
      test_edit_collection_key!(
        CollectionDetailsHandler,
        ActiveRadarrBlock::CollectionDetails,
        ActiveRadarrBlock::CollectionDetails
      );
    }
  }
}
