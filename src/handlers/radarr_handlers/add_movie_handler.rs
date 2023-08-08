use strum::IntoEnumIterator;

use crate::app::key_binding::DEFAULT_KEYBINDINGS;
use crate::app::radarr::ActiveRadarrBlock;
use crate::handlers::{handle_prompt_toggle, KeyEventHandler};
use crate::models::radarr_models::{MinimumAvailability, Monitor};
use crate::models::{Scrollable, StatefulTable};
use crate::network::radarr_network::RadarrEvent;
use crate::{handle_text_box_keys, App, Key};

pub(super) struct AddMovieHandler<'a> {
  key: &'a Key,
  app: &'a mut App,
  active_radarr_block: &'a ActiveRadarrBlock,
}

impl<'a> KeyEventHandler<'a, ActiveRadarrBlock> for AddMovieHandler<'a> {
  fn with(
    key: &'a Key,
    app: &'a mut App,
    active_block: &'a ActiveRadarrBlock,
  ) -> AddMovieHandler<'a> {
    AddMovieHandler {
      key,
      app,
      active_radarr_block: active_block,
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
        self.app.data.radarr_data.add_movie_monitor_list.scroll_up()
      }
      ActiveRadarrBlock::AddMovieSelectMinimumAvailability => self
        .app
        .data
        .radarr_data
        .add_movie_minimum_availability_list
        .scroll_up(),
      ActiveRadarrBlock::AddMovieSelectQualityProfile => self
        .app
        .data
        .radarr_data
        .add_movie_quality_profile_list
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
      ActiveRadarrBlock::AddMovieSelectMonitor => self
        .app
        .data
        .radarr_data
        .add_movie_monitor_list
        .scroll_down(),
      ActiveRadarrBlock::AddMovieSelectMinimumAvailability => self
        .app
        .data
        .radarr_data
        .add_movie_minimum_availability_list
        .scroll_down(),
      ActiveRadarrBlock::AddMovieSelectQualityProfile => self
        .app
        .data
        .radarr_data
        .add_movie_quality_profile_list
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
      ActiveRadarrBlock::AddMovieSelectMonitor => self
        .app
        .data
        .radarr_data
        .add_movie_monitor_list
        .scroll_to_top(),
      ActiveRadarrBlock::AddMovieSelectMinimumAvailability => self
        .app
        .data
        .radarr_data
        .add_movie_minimum_availability_list
        .scroll_to_top(),
      ActiveRadarrBlock::AddMovieSelectQualityProfile => self
        .app
        .data
        .radarr_data
        .add_movie_quality_profile_list
        .scroll_to_top(),
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
        .add_movie_monitor_list
        .scroll_to_bottom(),
      ActiveRadarrBlock::AddMovieSelectMinimumAvailability => self
        .app
        .data
        .radarr_data
        .add_movie_minimum_availability_list
        .scroll_to_bottom(),
      ActiveRadarrBlock::AddMovieSelectQualityProfile => self
        .app
        .data
        .radarr_data
        .add_movie_quality_profile_list
        .scroll_to_bottom(),
      _ => (),
    }
  }

  fn handle_delete(&mut self) {}

  fn handle_left_right_action(&mut self) {
    if let ActiveRadarrBlock::AddMoviePrompt = self.active_radarr_block {
      handle_prompt_toggle(self.app, self.key)
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
      ActiveRadarrBlock::AddMovieSearchResults => {
        self
          .app
          .push_navigation_stack(ActiveRadarrBlock::AddMoviePrompt.into());
        self
          .app
          .data
          .radarr_data
          .add_movie_monitor_list
          .set_items(Vec::from_iter(Monitor::iter()));
        self
          .app
          .data
          .radarr_data
          .add_movie_minimum_availability_list
          .set_items(Vec::from_iter(MinimumAvailability::iter()));
        let mut quality_profile_names: Vec<String> = self
          .app
          .data
          .radarr_data
          .quality_profile_map
          .values()
          .cloned()
          .collect();
        quality_profile_names.sort();
        self
          .app
          .data
          .radarr_data
          .add_movie_quality_profile_list
          .set_items(quality_profile_names);
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
        ActiveRadarrBlock::AddMovieSelectMonitor => self
          .app
          .push_navigation_stack(ActiveRadarrBlock::AddMovieSelectMonitor.into()),
        ActiveRadarrBlock::AddMovieSelectMinimumAvailability => self
          .app
          .push_navigation_stack(ActiveRadarrBlock::AddMovieSelectMinimumAvailability.into()),
        ActiveRadarrBlock::AddMovieSelectQualityProfile => self
          .app
          .push_navigation_stack(ActiveRadarrBlock::AddMovieSelectQualityProfile.into()),
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
        self.app.data.radarr_data.reset_add_movie_selections();
        self.app.data.radarr_data.prompt_confirm = false;
      }
      ActiveRadarrBlock::AddMovieSelectMonitor
      | ActiveRadarrBlock::AddMovieSelectMinimumAvailability
      | ActiveRadarrBlock::AddMovieSelectQualityProfile => self.app.pop_navigation_stack(),
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
      title,
      stationary_style
    );

    test_enum_scroll!(
      test_add_movie_select_monitor_scroll,
      AddMovieHandler,
      Monitor,
      add_movie_monitor_list,
      ActiveRadarrBlock::AddMovieSelectMonitor
    );

    test_enum_scroll!(
      test_add_movie_select_minimuum_availability_scroll,
      AddMovieHandler,
      MinimumAvailability,
      add_movie_minimum_availability_list,
      ActiveRadarrBlock::AddMovieSelectMinimumAvailability
    );

    test_iterable_scroll!(
      test_add_movie_select_quality_profile_scroll,
      AddMovieHandler,
      add_movie_quality_profile_list,
      ActiveRadarrBlock::AddMovieSelectQualityProfile
    );

    #[rstest]
    fn test_add_movie_prompt_scroll(#[values(Key::Up, Key::Down)] key: Key) {
      let mut app = App::default();
      app.data.radarr_data.selected_block = ActiveRadarrBlock::AddMovieSelectMinimumAvailability;

      AddMovieHandler::with(&key, &mut app, &ActiveRadarrBlock::AddMoviePrompt).handle();

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
    use strum::IntoEnumIterator;

    use crate::{
      extended_stateful_iterable_vec, test_enum_home_and_end, test_iterable_home_and_end,
    };

    use super::*;

    test_iterable_home_and_end!(
      test_add_movie_search_results_home_end,
      AddMovieHandler,
      add_searched_movies,
      extended_stateful_iterable_vec!(AddMovieSearchResult, HorizontallyScrollableText),
      ActiveRadarrBlock::AddMovieSearchResults,
      title,
      stationary_style
    );

    test_enum_home_and_end!(
      test_add_movie_select_monitor_home_end,
      AddMovieHandler,
      Monitor,
      add_movie_monitor_list,
      ActiveRadarrBlock::AddMovieSelectMonitor
    );

    test_enum_home_and_end!(
      test_add_movie_select_minimuum_availability_home_end,
      AddMovieHandler,
      MinimumAvailability,
      add_movie_minimum_availability_list,
      ActiveRadarrBlock::AddMovieSelectMinimumAvailability
    );

    test_iterable_home_and_end!(
      test_add_movie_select_quality_profile_scroll,
      AddMovieHandler,
      add_movie_quality_profile_list,
      ActiveRadarrBlock::AddMovieSelectQualityProfile
    );
  }

  mod test_handle_left_right_action {
    use rstest::rstest;

    use super::*;

    #[rstest]
    fn test_left_right_prompt_toggle(#[values(Key::Left, Key::Right)] key: Key) {
      let mut app = App::default();

      AddMovieHandler::with(&key, &mut app, &ActiveRadarrBlock::AddMoviePrompt).handle();

      assert!(app.data.radarr_data.prompt_confirm);

      AddMovieHandler::with(&key, &mut app, &ActiveRadarrBlock::AddMoviePrompt).handle();

      assert!(!app.data.radarr_data.prompt_confirm);
    }
  }

  mod test_handle_submit {
    use std::collections::HashMap;

    use pretty_assertions::{assert_eq, assert_str_eq};
    use rstest::rstest;

    use crate::app::key_binding::DEFAULT_KEYBINDINGS;
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
      app.data.radarr_data.quality_profile_map =
        HashMap::from([(1, "B - Test 2".to_owned()), (0, "A - Test 1".to_owned())]);

      AddMovieHandler::with(
        &SUBMIT_KEY,
        &mut app,
        &ActiveRadarrBlock::AddMovieSearchResults,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        &ActiveRadarrBlock::AddMoviePrompt.into()
      );
      assert!(!app.data.radarr_data.add_movie_monitor_list.items.is_empty());
      assert!(!app
        .data
        .radarr_data
        .add_movie_minimum_availability_list
        .items
        .is_empty());
      assert!(!app
        .data
        .radarr_data
        .add_movie_quality_profile_list
        .items
        .is_empty());
      assert_str_eq!(
        app
          .data
          .radarr_data
          .add_movie_quality_profile_list
          .current_selection(),
        "A - Test 1"
      );
    }

    #[test]
    fn test_add_movie_prompt_prompt_decline() {
      let mut app = App::default();
      app.push_navigation_stack(ActiveRadarrBlock::Movies.into());
      app.push_navigation_stack(ActiveRadarrBlock::AddMoviePrompt.into());
      app.data.radarr_data.selected_block = ActiveRadarrBlock::AddMovieConfirmPrompt;

      AddMovieHandler::with(&SUBMIT_KEY, &mut app, &ActiveRadarrBlock::AddMoviePrompt).handle();

      assert_eq!(app.get_current_route(), &ActiveRadarrBlock::Movies.into());
      assert_eq!(app.data.radarr_data.prompt_confirm_action, None);
    }

    #[test]
    fn test_add_movie_confirm_prompt_prompt_confirmation() {
      let mut app = App::default();
      app.push_navigation_stack(ActiveRadarrBlock::Movies.into());
      app.push_navigation_stack(ActiveRadarrBlock::AddMoviePrompt.into());
      app.data.radarr_data.prompt_confirm = true;
      app.data.radarr_data.selected_block = ActiveRadarrBlock::AddMovieConfirmPrompt;

      AddMovieHandler::with(&SUBMIT_KEY, &mut app, &ActiveRadarrBlock::AddMoviePrompt).handle();

      assert_eq!(app.get_current_route(), &ActiveRadarrBlock::Movies.into());
      assert_eq!(
        app.data.radarr_data.prompt_confirm_action,
        Some(RadarrEvent::AddMovie)
      );
    }

    #[rstest]
    fn test_add_movie_prompt_selected_block(
      #[values(
        ActiveRadarrBlock::AddMovieSelectMonitor,
        ActiveRadarrBlock::AddMovieSelectMinimumAvailability,
        ActiveRadarrBlock::AddMovieSelectQualityProfile
      )]
      selected_block: ActiveRadarrBlock,
    ) {
      let mut app = App::default();
      app.push_navigation_stack(ActiveRadarrBlock::AddMoviePrompt.into());
      app.data.radarr_data.selected_block = selected_block.clone();

      AddMovieHandler::with(&SUBMIT_KEY, &mut app, &ActiveRadarrBlock::AddMoviePrompt).handle();

      assert_eq!(app.get_current_route(), &selected_block.into());
      assert_eq!(app.data.radarr_data.prompt_confirm_action, None);
    }

    #[rstest]
    fn test_add_movie_prompt_selecting_preferences_blocks(
      #[values(
        ActiveRadarrBlock::AddMovieSelectMonitor,
        ActiveRadarrBlock::AddMovieSelectMinimumAvailability,
        ActiveRadarrBlock::AddMovieSelectQualityProfile
      )]
      active_radarr_block: ActiveRadarrBlock,
    ) {
      let mut app = App::default();
      app.push_navigation_stack(ActiveRadarrBlock::AddMoviePrompt.into());
      app.push_navigation_stack(active_radarr_block.clone().into());

      AddMovieHandler::with(&SUBMIT_KEY, &mut app, &active_radarr_block).handle();

      assert_eq!(
        app.get_current_route(),
        &ActiveRadarrBlock::AddMoviePrompt.into()
      );
    }
  }

  mod test_handle_esc {
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    use crate::app::radarr::RadarrData;
    use crate::models::radarr_models::{Collection, Movie};
    use crate::simple_stateful_iterable_vec;

    use super::*;

    const ESC_KEY: Key = DEFAULT_KEYBINDINGS.esc.key;

    #[test]
    fn test_add_movie_search_input_esc() {
      let mut radarr_data = RadarrData {
        is_searching: true,
        search: "test search".to_owned(),
        filter: "test filter".to_owned(),
        ..RadarrData::default()
      };
      radarr_data
        .filtered_movies
        .set_items(vec![Movie::default()]);
      radarr_data
        .filtered_collections
        .set_items(vec![Collection::default()]);
      radarr_data
        .add_searched_movies
        .set_items(vec![AddMovieSearchResult::default()]);
      let mut app = App::default();
      app.data.radarr_data = radarr_data;
      app.should_ignore_quit_key = true;
      app.push_navigation_stack(ActiveRadarrBlock::AddMovieSearchInput.into());

      AddMovieHandler::with(&ESC_KEY, &mut app, &ActiveRadarrBlock::AddMovieSearchInput).handle();

      assert!(!app.should_ignore_quit_key);
      assert_eq!(app.get_current_route(), &ActiveRadarrBlock::Movies.into());
      assert!(!app.data.radarr_data.is_searching);
      assert!(app.data.radarr_data.search.is_empty());
      assert!(app.data.radarr_data.filter.is_empty());
      assert!(app.data.radarr_data.filtered_movies.items.is_empty());
      assert!(app.data.radarr_data.filtered_collections.items.is_empty());
      assert!(app.data.radarr_data.add_searched_movies.items.is_empty());
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
    fn test_add_movie_prompt_esc() {
      let mut radarr_data = RadarrData::default();
      radarr_data
        .add_movie_monitor_list
        .set_items(vec![Monitor::default()]);
      radarr_data
        .add_movie_minimum_availability_list
        .set_items(vec![MinimumAvailability::default()]);
      radarr_data
        .add_movie_quality_profile_list
        .set_items(vec![String::default()]);
      radarr_data.prompt_confirm = true;
      let mut app = App::default();
      app.data.radarr_data = radarr_data;
      app.push_navigation_stack(ActiveRadarrBlock::AddMovieSearchResults.into());
      app.push_navigation_stack(ActiveRadarrBlock::AddMoviePrompt.into());

      AddMovieHandler::with(&ESC_KEY, &mut app, &ActiveRadarrBlock::AddMoviePrompt).handle();

      assert!(!app.data.radarr_data.prompt_confirm);
      assert_eq!(
        app.get_current_route(),
        &ActiveRadarrBlock::AddMovieSearchResults.into()
      );
      assert!(app.data.radarr_data.add_movie_monitor_list.items.is_empty());
      assert!(app
        .data
        .radarr_data
        .add_movie_minimum_availability_list
        .items
        .is_empty());
      assert!(app
        .data
        .radarr_data
        .add_movie_quality_profile_list
        .items
        .is_empty());
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
      app.push_navigation_stack(ActiveRadarrBlock::AddMoviePrompt.into());
      app.push_navigation_stack(active_radarr_block.into());

      AddMovieHandler::with(&ESC_KEY, &mut app, &ActiveRadarrBlock::AddMoviePrompt).handle();

      assert_eq!(
        app.get_current_route(),
        &ActiveRadarrBlock::AddMoviePrompt.into()
      );
    }
  }

  mod test_handle_key_char {
    use super::*;

    #[test]
    fn test_add_movie_search_input_backspace() {
      let mut app = App::default();
      app.data.radarr_data.search = "Test".to_owned();

      AddMovieHandler::with(
        &DEFAULT_KEYBINDINGS.backspace.key,
        &mut app,
        &ActiveRadarrBlock::AddMovieSearchInput,
      )
      .handle();

      assert_str_eq!(app.data.radarr_data.search, "Tes");
    }

    #[test]
    fn test_add_movie_search_input_char_key() {
      let mut app = App::default();

      AddMovieHandler::with(
        &Key::Char('h'),
        &mut app,
        &ActiveRadarrBlock::AddMovieSearchInput,
      )
      .handle();

      assert_str_eq!(app.data.radarr_data.search, "h");
    }
  }
}
