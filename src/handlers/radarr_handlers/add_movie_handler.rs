use crate::app::key_binding::DEFAULT_KEYBINDINGS;
use crate::app::radarr::ActiveRadarrBlock;
use crate::handlers::{handle_prompt_toggle, KeyEventHandler};
use crate::models::{Scrollable, StatefulTable};
use crate::network::radarr_network::RadarrEvent;
use crate::{handle_text_box_keys, handle_text_box_left_right_keys, App, Key};

pub(super) struct AddMovieHandler<'a> {
  key: &'a Key,
  app: &'a mut App,
  active_radarr_block: &'a ActiveRadarrBlock,
  context: &'a Option<ActiveRadarrBlock>,
}

impl<'a> KeyEventHandler<'a, ActiveRadarrBlock> for AddMovieHandler<'a> {
  fn with(
    key: &'a Key,
    app: &'a mut App,
    active_block: &'a ActiveRadarrBlock,
    context: &'a Option<ActiveRadarrBlock>,
  ) -> AddMovieHandler<'a> {
    AddMovieHandler {
      key,
      app,
      active_radarr_block: active_block,
      context,
    }
  }

  fn get_key(&self) -> &Key {
    self.key
  }

  fn handle_scroll_up(&mut self) {
    match self.active_radarr_block {
      ActiveRadarrBlock::AddMovieSearchResults => {
        self.app.data.radarr_data.add_searched_movies.scroll_up()
      }
      ActiveRadarrBlock::AddMovieSelectMonitor => {
        self.app.data.radarr_data.movie_monitor_list.scroll_up()
      }
      ActiveRadarrBlock::AddMovieSelectMinimumAvailability => self
        .app
        .data
        .radarr_data
        .movie_minimum_availability_list
        .scroll_up(),
      ActiveRadarrBlock::AddMovieSelectQualityProfile => self
        .app
        .data
        .radarr_data
        .movie_quality_profile_list
        .scroll_up(),
      ActiveRadarrBlock::AddMoviePrompt => {
        self.app.data.radarr_data.selected_block = self
          .app
          .data
          .radarr_data
          .selected_block
          .clone()
          .previous_add_prompt_block()
      }
      _ => (),
    }
  }

  fn handle_scroll_down(&mut self) {
    match self.active_radarr_block {
      ActiveRadarrBlock::AddMovieSearchResults => {
        self.app.data.radarr_data.add_searched_movies.scroll_down()
      }
      ActiveRadarrBlock::AddMovieSelectMonitor => {
        self.app.data.radarr_data.movie_monitor_list.scroll_down()
      }
      ActiveRadarrBlock::AddMovieSelectMinimumAvailability => self
        .app
        .data
        .radarr_data
        .movie_minimum_availability_list
        .scroll_down(),
      ActiveRadarrBlock::AddMovieSelectQualityProfile => self
        .app
        .data
        .radarr_data
        .movie_quality_profile_list
        .scroll_down(),
      ActiveRadarrBlock::AddMoviePrompt => {
        self.app.data.radarr_data.selected_block = self
          .app
          .data
          .radarr_data
          .selected_block
          .next_add_prompt_block()
      }
      _ => (),
    }
  }

  fn handle_home(&mut self) {
    match self.active_radarr_block {
      ActiveRadarrBlock::AddMovieSearchResults => self
        .app
        .data
        .radarr_data
        .add_searched_movies
        .scroll_to_top(),
      ActiveRadarrBlock::AddMovieSelectMonitor => {
        self.app.data.radarr_data.movie_monitor_list.scroll_to_top()
      }
      ActiveRadarrBlock::AddMovieSelectMinimumAvailability => self
        .app
        .data
        .radarr_data
        .movie_minimum_availability_list
        .scroll_to_top(),
      ActiveRadarrBlock::AddMovieSelectQualityProfile => self
        .app
        .data
        .radarr_data
        .movie_quality_profile_list
        .scroll_to_top(),
      ActiveRadarrBlock::AddMovieSearchInput => self.app.data.radarr_data.search.scroll_home(),
      _ => (),
    }
  }

  fn handle_end(&mut self) {
    match self.active_radarr_block {
      ActiveRadarrBlock::AddMovieSearchResults => self
        .app
        .data
        .radarr_data
        .add_searched_movies
        .scroll_to_bottom(),
      ActiveRadarrBlock::AddMovieSelectMonitor => self
        .app
        .data
        .radarr_data
        .movie_monitor_list
        .scroll_to_bottom(),
      ActiveRadarrBlock::AddMovieSelectMinimumAvailability => self
        .app
        .data
        .radarr_data
        .movie_minimum_availability_list
        .scroll_to_bottom(),
      ActiveRadarrBlock::AddMovieSelectQualityProfile => self
        .app
        .data
        .radarr_data
        .movie_quality_profile_list
        .scroll_to_bottom(),
      ActiveRadarrBlock::AddMovieSearchInput => self.app.data.radarr_data.search.reset_offset(),
      _ => (),
    }
  }

  fn handle_delete(&mut self) {}

  fn handle_left_right_action(&mut self) {
    match self.active_radarr_block {
      ActiveRadarrBlock::AddMoviePrompt => handle_prompt_toggle(self.app, self.key),
      ActiveRadarrBlock::AddMovieSearchInput => {
        handle_text_box_left_right_keys!(self, self.key, self.app.data.radarr_data.search)
      }
      _ => (),
    }
  }

  fn handle_submit(&mut self) {
    match self.active_radarr_block {
      ActiveRadarrBlock::AddMovieSearchInput => {
        self
          .app
          .push_navigation_stack(ActiveRadarrBlock::AddMovieSearchResults.into());
        self.app.should_ignore_quit_key = false;
      }
      _ if *self.active_radarr_block == ActiveRadarrBlock::AddMovieSearchResults
        && !self
          .app
          .data
          .radarr_data
          .add_searched_movies
          .items
          .is_empty() =>
      {
        let tmdb_id = self
          .app
          .data
          .radarr_data
          .add_searched_movies
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
            .push_navigation_stack(ActiveRadarrBlock::AddMovieAlreadyInLibrary.into());
        } else {
          self
            .app
            .push_navigation_stack(ActiveRadarrBlock::AddMoviePrompt.into());
          self.app.data.radarr_data.populate_movie_preferences_lists();
          self.app.data.radarr_data.selected_block = ActiveRadarrBlock::AddMovieSelectMonitor
        }
      }
      ActiveRadarrBlock::AddMoviePrompt => match self.app.data.radarr_data.selected_block {
        ActiveRadarrBlock::AddMovieConfirmPrompt => {
          if self.app.data.radarr_data.prompt_confirm {
            self.app.data.radarr_data.prompt_confirm_action = Some(RadarrEvent::AddMovie);
            self.app.pop_navigation_stack();
          } else {
            self.app.pop_navigation_stack();
          }
        }
        ActiveRadarrBlock::AddMovieSelectMonitor
        | ActiveRadarrBlock::AddMovieSelectMinimumAvailability
        | ActiveRadarrBlock::AddMovieSelectQualityProfile => self
          .app
          .push_navigation_stack((self.app.data.radarr_data.selected_block, *self.context).into()),
        _ => (),
      },
      ActiveRadarrBlock::AddMovieSelectMonitor
      | ActiveRadarrBlock::AddMovieSelectMinimumAvailability
      | ActiveRadarrBlock::AddMovieSelectQualityProfile => self.app.pop_navigation_stack(),
      _ => (),
    }
  }

  fn handle_esc(&mut self) {
    match self.active_radarr_block {
      ActiveRadarrBlock::AddMovieSearchInput => {
        self.app.pop_navigation_stack();
        self.app.data.radarr_data.reset_search();
        self.app.should_ignore_quit_key = false;
      }
      ActiveRadarrBlock::AddMovieSearchResults => {
        self.app.pop_navigation_stack();
        self.app.data.radarr_data.add_searched_movies = StatefulTable::default();
        self.app.should_ignore_quit_key = true;
      }
      ActiveRadarrBlock::AddMoviePrompt => {
        self.app.pop_navigation_stack();
        self
          .app
          .data
          .radarr_data
          .reset_movie_preferences_selections();
        self.app.data.radarr_data.prompt_confirm = false;
      }
      ActiveRadarrBlock::AddMovieSelectMonitor
      | ActiveRadarrBlock::AddMovieSelectMinimumAvailability
      | ActiveRadarrBlock::AddMovieSelectQualityProfile
      | ActiveRadarrBlock::AddMovieAlreadyInLibrary => self.app.pop_navigation_stack(),
      _ => (),
    }
  }

  fn handle_char_key_event(&mut self) {
    let key = self.key;
    if self.active_radarr_block == &ActiveRadarrBlock::AddMovieSearchInput {
      handle_text_box_keys!(self, key, self.app.data.radarr_data.search)
    }
  }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
  use pretty_assertions::assert_str_eq;

  use crate::app::key_binding::DEFAULT_KEYBINDINGS;
  use crate::app::radarr::ActiveRadarrBlock;
  use crate::app::App;
  use crate::event::Key;
  use crate::handlers::radarr_handlers::add_movie_handler::AddMovieHandler;
  use crate::handlers::KeyEventHandler;
  use crate::models::radarr_models::{AddMovieSearchResult, MinimumAvailability, Monitor};
  use crate::models::HorizontallyScrollableText;

  mod test_handle_scroll_up_and_down {
    use pretty_assertions::assert_eq;
    use rstest::rstest;
    use strum::IntoEnumIterator;

    use crate::{simple_stateful_iterable_vec, test_enum_scroll, test_iterable_scroll};

    use super::*;

    test_iterable_scroll!(
      test_add_movie_search_results_scroll,
      AddMovieHandler,
      add_searched_movies,
      simple_stateful_iterable_vec!(AddMovieSearchResult, HorizontallyScrollableText),
      ActiveRadarrBlock::AddMovieSearchResults,
      None,
      title,
      to_string
    );

    test_enum_scroll!(
      test_add_movie_select_monitor_scroll,
      AddMovieHandler,
      Monitor,
      movie_monitor_list,
      ActiveRadarrBlock::AddMovieSelectMonitor,
      None
    );

    test_enum_scroll!(
      test_add_movie_select_minimuum_availability_scroll,
      AddMovieHandler,
      MinimumAvailability,
      movie_minimum_availability_list,
      ActiveRadarrBlock::AddMovieSelectMinimumAvailability,
      None
    );

    test_iterable_scroll!(
      test_add_movie_select_quality_profile_scroll,
      AddMovieHandler,
      movie_quality_profile_list,
      ActiveRadarrBlock::AddMovieSelectQualityProfile,
      None
    );

    #[rstest]
    fn test_add_movie_prompt_scroll(#[values(Key::Up, Key::Down)] key: Key) {
      let mut app = App::default();
      app.data.radarr_data.selected_block = ActiveRadarrBlock::AddMovieSelectMinimumAvailability;

      AddMovieHandler::with(&key, &mut app, &ActiveRadarrBlock::AddMoviePrompt, &None).handle();

      if key == Key::Up {
        assert_eq!(
          app.data.radarr_data.selected_block,
          ActiveRadarrBlock::AddMovieSelectMonitor
        );
      } else {
        assert_eq!(
          app.data.radarr_data.selected_block,
          ActiveRadarrBlock::AddMovieSelectQualityProfile
        );
      }
    }
  }

  mod test_handle_home_end {
    use rstest::rstest;
    use strum::IntoEnumIterator;

    use crate::{
      extended_stateful_iterable_vec, test_enum_home_and_end, test_iterable_home_and_end,
      test_text_box_home_end_keys,
    };

    use super::*;

    test_iterable_home_and_end!(
      test_add_movie_search_results_home_end,
      AddMovieHandler,
      add_searched_movies,
      extended_stateful_iterable_vec!(AddMovieSearchResult, HorizontallyScrollableText),
      ActiveRadarrBlock::AddMovieSearchResults,
      None,
      title,
      to_string
    );

    test_enum_home_and_end!(
      test_add_movie_select_monitor_home_end,
      AddMovieHandler,
      Monitor,
      movie_monitor_list,
      ActiveRadarrBlock::AddMovieSelectMonitor,
      None
    );

    test_enum_home_and_end!(
      test_add_movie_select_minimuum_availability_home_end,
      AddMovieHandler,
      MinimumAvailability,
      movie_minimum_availability_list,
      ActiveRadarrBlock::AddMovieSelectMinimumAvailability,
      None
    );

    test_iterable_home_and_end!(
      test_add_movie_select_quality_profile_scroll,
      AddMovieHandler,
      movie_quality_profile_list,
      ActiveRadarrBlock::AddMovieSelectQualityProfile,
      None
    );

    #[test]
    fn test_add_movie_search_input_home_end_keys() {
      test_text_box_home_end_keys!(
        AddMovieHandler,
        ActiveRadarrBlock::AddMovieSearchInput,
        search
      );
    }
  }

  mod test_handle_left_right_action {
    use rstest::rstest;

    use crate::test_text_box_left_right_keys;

    use super::*;

    #[rstest]
    fn test_left_right_prompt_toggle(#[values(Key::Left, Key::Right)] key: Key) {
      let mut app = App::default();

      AddMovieHandler::with(&key, &mut app, &ActiveRadarrBlock::AddMoviePrompt, &None).handle();

      assert!(app.data.radarr_data.prompt_confirm);

      AddMovieHandler::with(&key, &mut app, &ActiveRadarrBlock::AddMoviePrompt, &None).handle();

      assert!(!app.data.radarr_data.prompt_confirm);
    }

    #[test]
    fn test_add_movie_search_input_left_right_keys() {
      test_text_box_left_right_keys!(
        AddMovieHandler,
        ActiveRadarrBlock::AddMovieSearchInput,
        search
      );
    }
  }

  mod test_handle_submit {
    use std::collections::HashMap;

    use pretty_assertions::{assert_eq, assert_str_eq};
    use rstest::rstest;

    use crate::app::key_binding::DEFAULT_KEYBINDINGS;
    use crate::models::radarr_models::Movie;
    use crate::network::radarr_network::RadarrEvent;

    use super::*;

    const SUBMIT_KEY: Key = DEFAULT_KEYBINDINGS.submit.key;

    #[test]
    fn test_add_movie_search_input_submit() {
      let mut app = App::default();
      app.should_ignore_quit_key = true;

      AddMovieHandler::with(
        &SUBMIT_KEY,
        &mut app,
        &ActiveRadarrBlock::AddMovieSearchInput,
        &None,
      )
      .handle();

      assert!(!app.should_ignore_quit_key);
      assert_eq!(
        app.get_current_route(),
        &ActiveRadarrBlock::AddMovieSearchResults.into()
      );
    }

    #[test]
    fn test_add_movie_search_results_submit() {
      let mut app = App::default();
      app
        .data
        .radarr_data
        .add_searched_movies
        .set_items(vec![AddMovieSearchResult::default()]);
      app.data.radarr_data.quality_profile_map =
        HashMap::from([(1, "B - Test 2".to_owned()), (0, "A - Test 1".to_owned())]);

      AddMovieHandler::with(
        &SUBMIT_KEY,
        &mut app,
        &ActiveRadarrBlock::AddMovieSearchResults,
        &None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        &ActiveRadarrBlock::AddMoviePrompt.into()
      );
      assert_eq!(
        app.data.radarr_data.selected_block,
        ActiveRadarrBlock::AddMovieSelectMonitor
      );
      assert!(!app.data.radarr_data.movie_monitor_list.items.is_empty());
      assert!(!app
        .data
        .radarr_data
        .movie_minimum_availability_list
        .items
        .is_empty());
      assert!(!app
        .data
        .radarr_data
        .movie_quality_profile_list
        .items
        .is_empty());
      assert_str_eq!(
        app
          .data
          .radarr_data
          .movie_quality_profile_list
          .current_selection(),
        "A - Test 1"
      );
    }

    #[test]
    fn test_add_movie_search_results_submit_does_nothing_on_empty_table() {
      let mut app = App::default();
      app.push_navigation_stack(ActiveRadarrBlock::AddMovieSearchResults.into());
      AddMovieHandler::with(
        &SUBMIT_KEY,
        &mut app,
        &ActiveRadarrBlock::AddMovieSearchResults,
        &None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        &ActiveRadarrBlock::AddMovieSearchResults.into()
      );
    }

    #[test]
    fn test_add_movie_search_results_submit_movie_already_in_library() {
      let mut app = App::default();
      app
        .data
        .radarr_data
        .add_searched_movies
        .set_items(vec![AddMovieSearchResult::default()]);
      app
        .data
        .radarr_data
        .movies
        .set_items(vec![Movie::default()]);

      AddMovieHandler::with(
        &SUBMIT_KEY,
        &mut app,
        &ActiveRadarrBlock::AddMovieSearchResults,
        &None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        &ActiveRadarrBlock::AddMovieAlreadyInLibrary.into()
      );
    }

    #[test]
    fn test_add_movie_prompt_prompt_decline_submit() {
      let mut app = App::default();
      app.push_navigation_stack(ActiveRadarrBlock::Movies.into());
      app.push_navigation_stack(ActiveRadarrBlock::AddMoviePrompt.into());
      app.data.radarr_data.selected_block = ActiveRadarrBlock::AddMovieConfirmPrompt;

      AddMovieHandler::with(
        &SUBMIT_KEY,
        &mut app,
        &ActiveRadarrBlock::AddMoviePrompt,
        &None,
      )
      .handle();

      assert_eq!(app.get_current_route(), &ActiveRadarrBlock::Movies.into());
      assert_eq!(app.data.radarr_data.prompt_confirm_action, None);
    }

    #[test]
    fn test_add_movie_confirm_prompt_prompt_confirmation_submit() {
      let mut app = App::default();
      app.push_navigation_stack(ActiveRadarrBlock::Movies.into());
      app.push_navigation_stack(ActiveRadarrBlock::AddMoviePrompt.into());
      app.data.radarr_data.prompt_confirm = true;
      app.data.radarr_data.selected_block = ActiveRadarrBlock::AddMovieConfirmPrompt;

      AddMovieHandler::with(
        &SUBMIT_KEY,
        &mut app,
        &ActiveRadarrBlock::AddMoviePrompt,
        &None,
      )
      .handle();

      assert_eq!(app.get_current_route(), &ActiveRadarrBlock::Movies.into());
      assert_eq!(
        app.data.radarr_data.prompt_confirm_action,
        Some(RadarrEvent::AddMovie)
      );
    }

    #[rstest]
    fn test_add_movie_prompt_selected_block_submit(
      #[values(
        ActiveRadarrBlock::AddMovieSelectMonitor,
        ActiveRadarrBlock::AddMovieSelectMinimumAvailability,
        ActiveRadarrBlock::AddMovieSelectQualityProfile
      )]
      selected_block: ActiveRadarrBlock,
    ) {
      let mut app = App::default();
      app.push_navigation_stack(
        (
          ActiveRadarrBlock::AddMoviePrompt,
          Some(ActiveRadarrBlock::CollectionDetails),
        )
          .into(),
      );
      app.data.radarr_data.selected_block = selected_block;

      AddMovieHandler::with(
        &SUBMIT_KEY,
        &mut app,
        &ActiveRadarrBlock::AddMoviePrompt,
        &Some(ActiveRadarrBlock::CollectionDetails),
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        &(selected_block, Some(ActiveRadarrBlock::CollectionDetails)).into()
      );
      assert_eq!(app.data.radarr_data.prompt_confirm_action, None);
    }

    #[rstest]
    fn test_add_movie_prompt_selecting_preferences_blocks_submit(
      #[values(
        ActiveRadarrBlock::AddMovieSelectMonitor,
        ActiveRadarrBlock::AddMovieSelectMinimumAvailability,
        ActiveRadarrBlock::AddMovieSelectQualityProfile
      )]
      active_radarr_block: ActiveRadarrBlock,
    ) {
      let mut app = App::default();
      app.push_navigation_stack(ActiveRadarrBlock::AddMoviePrompt.into());
      app.push_navigation_stack(active_radarr_block.into());

      AddMovieHandler::with(
        &SUBMIT_KEY,
        &mut app,
        &active_radarr_block,
        &Some(ActiveRadarrBlock::CollectionDetails),
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        &ActiveRadarrBlock::AddMoviePrompt.into()
      );
    }
  }

  mod test_handle_esc {
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    use crate::app::radarr::radarr_test_utils::create_test_radarr_data;
    use crate::{
      assert_movie_preferences_selections_reset, assert_search_reset, simple_stateful_iterable_vec,
    };

    use super::*;

    const ESC_KEY: Key = DEFAULT_KEYBINDINGS.esc.key;

    #[test]
    fn test_add_movie_search_input_esc() {
      let mut app = App::default();
      app.data.radarr_data = create_test_radarr_data();
      app.should_ignore_quit_key = true;
      app.push_navigation_stack(ActiveRadarrBlock::AddMovieSearchInput.into());

      AddMovieHandler::with(
        &ESC_KEY,
        &mut app,
        &ActiveRadarrBlock::AddMovieSearchInput,
        &None,
      )
      .handle();

      assert!(!app.should_ignore_quit_key);
      assert_eq!(app.get_current_route(), &ActiveRadarrBlock::Movies.into());
      assert_search_reset!(app.data.radarr_data);
    }

    #[test]
    fn test_add_movie_search_results_esc() {
      let mut app = App::default();
      app.push_navigation_stack(ActiveRadarrBlock::AddMovieSearchInput.into());
      app.push_navigation_stack(ActiveRadarrBlock::AddMovieSearchResults.into());
      app
        .data
        .radarr_data
        .add_searched_movies
        .set_items(simple_stateful_iterable_vec!(
          AddMovieSearchResult,
          HorizontallyScrollableText
        ));

      AddMovieHandler::with(
        &ESC_KEY,
        &mut app,
        &ActiveRadarrBlock::AddMovieSearchResults,
        &None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        &ActiveRadarrBlock::AddMovieSearchInput.into()
      );
      assert!(app.data.radarr_data.add_searched_movies.items.is_empty());
      assert!(app.should_ignore_quit_key);
    }

    #[test]
    fn test_add_movie_already_in_library_esc() {
      let mut app = App::default();
      app.data.radarr_data = create_test_radarr_data();
      app.push_navigation_stack(ActiveRadarrBlock::AddMovieSearchResults.into());
      app.push_navigation_stack(ActiveRadarrBlock::AddMovieAlreadyInLibrary.into());

      AddMovieHandler::with(
        &ESC_KEY,
        &mut app,
        &ActiveRadarrBlock::AddMovieAlreadyInLibrary,
        &None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        &ActiveRadarrBlock::AddMovieSearchResults.into()
      );
    }

    #[test]
    fn test_add_movie_prompt_esc() {
      let mut app = App::default();
      app.data.radarr_data = create_test_radarr_data();
      app.push_navigation_stack(ActiveRadarrBlock::AddMovieSearchResults.into());
      app.push_navigation_stack(ActiveRadarrBlock::AddMoviePrompt.into());

      AddMovieHandler::with(
        &ESC_KEY,
        &mut app,
        &ActiveRadarrBlock::AddMoviePrompt,
        &None,
      )
      .handle();

      assert!(!app.data.radarr_data.prompt_confirm);
      assert_eq!(
        app.get_current_route(),
        &ActiveRadarrBlock::AddMovieSearchResults.into()
      );
      assert_movie_preferences_selections_reset!(app.data.radarr_data);
    }

    #[rstest]
    fn test_selecting_preferences_blocks_esc(
      #[values(
        ActiveRadarrBlock::AddMovieSelectMonitor,
        ActiveRadarrBlock::AddMovieSelectMinimumAvailability,
        ActiveRadarrBlock::AddMovieSelectQualityProfile
      )]
      active_radarr_block: ActiveRadarrBlock,
    ) {
      let mut app = App::default();
      app.push_navigation_stack(
        (
          ActiveRadarrBlock::AddMoviePrompt,
          Some(ActiveRadarrBlock::CollectionDetails),
        )
          .into(),
      );
      app.push_navigation_stack(
        (
          active_radarr_block,
          Some(ActiveRadarrBlock::CollectionDetails),
        )
          .into(),
      );

      AddMovieHandler::with(
        &ESC_KEY,
        &mut app,
        &active_radarr_block,
        &Some(ActiveRadarrBlock::CollectionDetails),
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        &(
          ActiveRadarrBlock::AddMoviePrompt,
          Some(ActiveRadarrBlock::CollectionDetails),
        )
          .into()
      );
    }
  }

  mod test_handle_key_char {
    use super::*;

    #[test]
    fn test_add_movie_search_input_backspace() {
      let mut app = App::default();
      app.data.radarr_data.search = "Test".to_owned().into();

      AddMovieHandler::with(
        &DEFAULT_KEYBINDINGS.backspace.key,
        &mut app,
        &ActiveRadarrBlock::AddMovieSearchInput,
        &None,
      )
      .handle();

      assert_str_eq!(app.data.radarr_data.search.text, "Tes");
    }

    #[test]
    fn test_add_movie_search_input_char_key() {
      let mut app = App::default();

      AddMovieHandler::with(
        &Key::Char('h'),
        &mut app,
        &ActiveRadarrBlock::AddMovieSearchInput,
        &None,
      )
      .handle();

      assert_str_eq!(app.data.radarr_data.search.text, "h");
    }
  }
}
