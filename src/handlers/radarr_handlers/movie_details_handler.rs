use crate::app::key_binding::DEFAULT_KEYBINDINGS;
use crate::app::radarr::ActiveRadarrBlock;
use crate::app::App;
use crate::event::Key;
use crate::handlers::{handle_prompt_toggle, KeyEventHandler};
use crate::models::Scrollable;
use crate::network::radarr_network::RadarrEvent;

pub(super) struct MovieDetailsHandler<'a> {
  key: &'a Key,
  app: &'a mut App,
  active_radarr_block: &'a ActiveRadarrBlock,
}

impl<'a> KeyEventHandler<'a, ActiveRadarrBlock> for MovieDetailsHandler<'a> {
  fn with(
    key: &'a Key,
    app: &'a mut App,
    active_block: &'a ActiveRadarrBlock,
  ) -> MovieDetailsHandler<'a> {
    MovieDetailsHandler {
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
      ActiveRadarrBlock::MovieDetails => self.app.data.radarr_data.movie_details.scroll_up(),
      ActiveRadarrBlock::MovieHistory => self.app.data.radarr_data.movie_history.scroll_up(),
      ActiveRadarrBlock::Cast => self.app.data.radarr_data.movie_cast.scroll_up(),
      ActiveRadarrBlock::Crew => self.app.data.radarr_data.movie_crew.scroll_up(),
      ActiveRadarrBlock::ManualSearch => self.app.data.radarr_data.movie_releases.scroll_up(),
      _ => (),
    }
  }

  fn handle_scroll_down(&mut self) {
    match self.active_radarr_block {
      ActiveRadarrBlock::MovieDetails => self.app.data.radarr_data.movie_details.scroll_down(),
      ActiveRadarrBlock::MovieHistory => self.app.data.radarr_data.movie_history.scroll_down(),
      ActiveRadarrBlock::Cast => self.app.data.radarr_data.movie_cast.scroll_down(),
      ActiveRadarrBlock::Crew => self.app.data.radarr_data.movie_crew.scroll_down(),
      ActiveRadarrBlock::ManualSearch => self.app.data.radarr_data.movie_releases.scroll_down(),
      _ => (),
    }
  }

  fn handle_home(&mut self) {
    match self.active_radarr_block {
      ActiveRadarrBlock::MovieDetails => self.app.data.radarr_data.movie_details.scroll_to_top(),
      ActiveRadarrBlock::MovieHistory => self.app.data.radarr_data.movie_history.scroll_to_top(),
      ActiveRadarrBlock::Cast => self.app.data.radarr_data.movie_cast.scroll_to_top(),
      ActiveRadarrBlock::Crew => self.app.data.radarr_data.movie_crew.scroll_to_top(),
      ActiveRadarrBlock::ManualSearch => self.app.data.radarr_data.movie_releases.scroll_to_top(),
      _ => (),
    }
  }

  fn handle_end(&mut self) {
    match self.active_radarr_block {
      ActiveRadarrBlock::MovieDetails => self.app.data.radarr_data.movie_details.scroll_to_bottom(),
      ActiveRadarrBlock::MovieHistory => self.app.data.radarr_data.movie_history.scroll_to_bottom(),
      ActiveRadarrBlock::Cast => self.app.data.radarr_data.movie_cast.scroll_to_bottom(),
      ActiveRadarrBlock::Crew => self.app.data.radarr_data.movie_crew.scroll_to_bottom(),
      ActiveRadarrBlock::ManualSearch => {
        self.app.data.radarr_data.movie_releases.scroll_to_bottom()
      }
      _ => (),
    }
  }

  fn handle_delete(&mut self) {}

  fn handle_left_right_action(&mut self) {
    match self.active_radarr_block {
      ActiveRadarrBlock::MovieDetails
      | ActiveRadarrBlock::MovieHistory
      | ActiveRadarrBlock::FileInfo
      | ActiveRadarrBlock::Cast
      | ActiveRadarrBlock::Crew
      | ActiveRadarrBlock::ManualSearch => match self.key {
        _ if *self.key == DEFAULT_KEYBINDINGS.left.key => {
          self.app.data.radarr_data.movie_info_tabs.previous();
          self.app.pop_and_push_navigation_stack(
            *self.app.data.radarr_data.movie_info_tabs.get_active_route(),
          );
        }
        _ if *self.key == DEFAULT_KEYBINDINGS.right.key => {
          self.app.data.radarr_data.movie_info_tabs.next();
          self.app.pop_and_push_navigation_stack(
            *self.app.data.radarr_data.movie_info_tabs.get_active_route(),
          );
        }
        _ => (),
      },
      ActiveRadarrBlock::AutomaticallySearchMoviePrompt
      | ActiveRadarrBlock::RefreshAndScanPrompt
      | ActiveRadarrBlock::ManualSearchConfirmPrompt => handle_prompt_toggle(self.app, self.key),
      _ => (),
    }
  }

  fn handle_submit(&mut self) {
    match self.active_radarr_block {
      ActiveRadarrBlock::AutomaticallySearchMoviePrompt => {
        if self.app.data.radarr_data.prompt_confirm {
          self.app.data.radarr_data.prompt_confirm_action =
            Some(RadarrEvent::TriggerAutomaticSearch);
        }

        self.app.pop_navigation_stack();
      }
      ActiveRadarrBlock::RefreshAndScanPrompt => {
        if self.app.data.radarr_data.prompt_confirm {
          self.app.data.radarr_data.prompt_confirm_action = Some(RadarrEvent::RefreshAndScan);
        }

        self.app.pop_navigation_stack();
      }
      ActiveRadarrBlock::ManualSearch => {
        self
          .app
          .push_navigation_stack(ActiveRadarrBlock::ManualSearchConfirmPrompt.into());
      }
      ActiveRadarrBlock::ManualSearchConfirmPrompt => {
        if self.app.data.radarr_data.prompt_confirm {
          self.app.data.radarr_data.prompt_confirm_action = Some(RadarrEvent::DownloadRelease);
        }

        self.app.pop_navigation_stack();
      }
      _ => (),
    }
  }

  fn handle_esc(&mut self) {
    match self.active_radarr_block {
      ActiveRadarrBlock::MovieDetails
      | ActiveRadarrBlock::MovieHistory
      | ActiveRadarrBlock::FileInfo
      | ActiveRadarrBlock::Cast
      | ActiveRadarrBlock::Crew
      | ActiveRadarrBlock::ManualSearch => {
        self.app.pop_navigation_stack();
        self.app.data.radarr_data.reset_movie_info_tabs();
      }
      ActiveRadarrBlock::AutomaticallySearchMoviePrompt
      | ActiveRadarrBlock::RefreshAndScanPrompt
      | ActiveRadarrBlock::ManualSearchConfirmPrompt => {
        self.app.pop_navigation_stack();
        self.app.data.radarr_data.prompt_confirm = false;
      }
      _ => (),
    }
  }

  fn handle_char_key_event(&mut self) {
    let key = self.key;
    match *self.active_radarr_block {
      ActiveRadarrBlock::MovieDetails
      | ActiveRadarrBlock::MovieHistory
      | ActiveRadarrBlock::FileInfo
      | ActiveRadarrBlock::Cast
      | ActiveRadarrBlock::Crew
      | ActiveRadarrBlock::ManualSearch => match self.key {
        _ if *key == DEFAULT_KEYBINDINGS.search.key => {
          self
            .app
            .push_navigation_stack(ActiveRadarrBlock::AutomaticallySearchMoviePrompt.into());
        }
        _ if *key == DEFAULT_KEYBINDINGS.refresh.key => {
          self
            .app
            .push_navigation_stack(ActiveRadarrBlock::RefreshAndScanPrompt.into());
        }
        _ => (),
      },
      _ => (),
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
  use crate::handlers::radarr_handlers::movie_details_handler::MovieDetailsHandler;
  use crate::handlers::KeyEventHandler;
  use crate::models::radarr_models::{Credit, MovieHistoryItem, Release};
  use crate::models::{HorizontallyScrollableText, ScrollableText};

  mod test_handle_scroll_up_and_down {
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    use crate::{simple_stateful_iterable_vec, test_iterable_scroll};

    use super::*;

    #[test]
    fn test_movie_details_scroll() {
      let mut app = App::default();
      app.data.radarr_data.movie_details = ScrollableText::with_string("Test 1\nTest 2".to_owned());

      MovieDetailsHandler::with(
        &DEFAULT_KEYBINDINGS.up.key,
        &mut app,
        &ActiveRadarrBlock::MovieDetails,
      )
      .handle();

      assert_eq!(app.data.radarr_data.movie_details.offset, 0);

      MovieDetailsHandler::with(
        &DEFAULT_KEYBINDINGS.down.key,
        &mut app,
        &ActiveRadarrBlock::MovieDetails,
      )
      .handle();

      assert_eq!(app.data.radarr_data.movie_details.offset, 1);
    }

    test_iterable_scroll!(
      test_movie_history_scroll,
      MovieDetailsHandler,
      movie_history,
      simple_stateful_iterable_vec!(MovieHistoryItem, HorizontallyScrollableText, source_title),
      ActiveRadarrBlock::MovieHistory,
      source_title,
      stationary_style
    );

    test_iterable_scroll!(
      test_cast_scroll,
      MovieDetailsHandler,
      movie_cast,
      simple_stateful_iterable_vec!(Credit, String, person_name),
      ActiveRadarrBlock::Cast,
      person_name,
      to_owned
    );

    test_iterable_scroll!(
      test_crew_scroll,
      MovieDetailsHandler,
      movie_crew,
      simple_stateful_iterable_vec!(Credit, String, person_name),
      ActiveRadarrBlock::Crew,
      person_name,
      to_owned
    );

    test_iterable_scroll!(
      test_manual_search_scroll,
      MovieDetailsHandler,
      movie_releases,
      simple_stateful_iterable_vec!(Release, HorizontallyScrollableText),
      ActiveRadarrBlock::ManualSearch,
      title,
      stationary_style
    );
  }

  mod test_handle_home_end {
    use crate::{extended_stateful_iterable_vec, test_iterable_home_and_end};

    use super::*;

    #[test]
    fn test_movie_details_home_end() {
      let mut app = App::default();
      app.data.radarr_data.movie_details = ScrollableText::with_string("Test 1\nTest 2".to_owned());

      MovieDetailsHandler::with(
        &DEFAULT_KEYBINDINGS.end.key,
        &mut app,
        &ActiveRadarrBlock::MovieDetails,
      )
      .handle();

      assert_eq!(app.data.radarr_data.movie_details.offset, 1);

      MovieDetailsHandler::with(
        &DEFAULT_KEYBINDINGS.home.key,
        &mut app,
        &ActiveRadarrBlock::MovieDetails,
      )
      .handle();

      assert_eq!(app.data.radarr_data.movie_details.offset, 0);
    }

    test_iterable_home_and_end!(
      test_movie_history_home_end,
      MovieDetailsHandler,
      movie_history,
      extended_stateful_iterable_vec!(MovieHistoryItem, HorizontallyScrollableText, source_title),
      ActiveRadarrBlock::MovieHistory,
      source_title,
      stationary_style
    );

    test_iterable_home_and_end!(
      test_cast_home_end,
      MovieDetailsHandler,
      movie_cast,
      extended_stateful_iterable_vec!(Credit, String, person_name),
      ActiveRadarrBlock::Cast,
      person_name,
      to_owned
    );

    test_iterable_home_and_end!(
      test_crew_home_end,
      MovieDetailsHandler,
      movie_crew,
      extended_stateful_iterable_vec!(Credit, String, person_name),
      ActiveRadarrBlock::Crew,
      person_name,
      to_owned
    );

    test_iterable_home_and_end!(
      test_manual_search_home_end,
      MovieDetailsHandler,
      movie_releases,
      extended_stateful_iterable_vec!(Release, HorizontallyScrollableText),
      ActiveRadarrBlock::ManualSearch,
      title,
      stationary_style
    );
  }

  mod test_handle_left_right_action {
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    use super::*;

    #[rstest]
    fn test_left_right_prompt_toggle(
      #[values(
        ActiveRadarrBlock::AutomaticallySearchMoviePrompt,
        ActiveRadarrBlock::RefreshAndScanPrompt,
        ActiveRadarrBlock::ManualSearchConfirmPrompt
      )]
      active_radarr_block: ActiveRadarrBlock,
      #[values(Key::Left, Key::Right)] key: Key,
    ) {
      let mut app = App::default();

      MovieDetailsHandler::with(&key, &mut app, &active_radarr_block).handle();

      assert!(app.data.radarr_data.prompt_confirm);

      MovieDetailsHandler::with(&key, &mut app, &active_radarr_block).handle();

      assert!(!app.data.radarr_data.prompt_confirm);
    }

    #[rstest]
    #[case(ActiveRadarrBlock::MovieDetails, ActiveRadarrBlock::MovieHistory)]
    #[case(ActiveRadarrBlock::MovieHistory, ActiveRadarrBlock::FileInfo)]
    #[case(ActiveRadarrBlock::FileInfo, ActiveRadarrBlock::Cast)]
    #[case(ActiveRadarrBlock::Cast, ActiveRadarrBlock::Crew)]
    #[case(ActiveRadarrBlock::Crew, ActiveRadarrBlock::ManualSearch)]
    #[case(ActiveRadarrBlock::ManualSearch, ActiveRadarrBlock::MovieDetails)]
    fn test_movie_info_tabs_left_right_action(
      #[case] left_block: ActiveRadarrBlock,
      #[case] right_block: ActiveRadarrBlock,
    ) {
      let mut app = App::default();
      app.push_navigation_stack(right_block.into());
      app.data.radarr_data.movie_info_tabs.index = app
        .data
        .radarr_data
        .movie_info_tabs
        .tabs
        .iter()
        .position(|tab_route| tab_route.route == right_block.into())
        .unwrap_or_default();

      MovieDetailsHandler::with(&DEFAULT_KEYBINDINGS.left.key, &mut app, &right_block).handle();

      assert_eq!(
        app.get_current_route(),
        app.data.radarr_data.movie_info_tabs.get_active_route()
      );
      assert_eq!(app.get_current_route(), &left_block.into());

      MovieDetailsHandler::with(&DEFAULT_KEYBINDINGS.right.key, &mut app, &left_block).handle();

      assert_eq!(
        app.get_current_route(),
        app.data.radarr_data.movie_info_tabs.get_active_route()
      );
      assert_eq!(app.get_current_route(), &right_block.into());
    }
  }

  mod test_handle_submit {
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    use crate::network::radarr_network::RadarrEvent;

    use super::*;

    const SUBMIT_KEY: Key = DEFAULT_KEYBINDINGS.submit.key;

    #[test]
    fn test_manual_search_submit() {
      let mut app = App::default();
      app.push_navigation_stack(ActiveRadarrBlock::ManualSearch.into());

      MovieDetailsHandler::with(&SUBMIT_KEY, &mut app, &ActiveRadarrBlock::ManualSearch).handle();

      assert_eq!(
        app.get_current_route(),
        &ActiveRadarrBlock::ManualSearchConfirmPrompt.into()
      );
    }

    #[rstest]
    #[case(
      ActiveRadarrBlock::AutomaticallySearchMoviePrompt,
      RadarrEvent::TriggerAutomaticSearch
    )]
    #[case(ActiveRadarrBlock::RefreshAndScanPrompt, RadarrEvent::RefreshAndScan)]
    #[case(
      ActiveRadarrBlock::ManualSearchConfirmPrompt,
      RadarrEvent::DownloadRelease
    )]
    fn test_movie_info_prompt_confirm_submit(
      #[case] prompt_block: ActiveRadarrBlock,
      #[case] expected_action: RadarrEvent,
    ) {
      let mut app = App::default();
      app.data.radarr_data.prompt_confirm = true;
      app.push_navigation_stack(ActiveRadarrBlock::MovieDetails.into());
      app.push_navigation_stack(prompt_block.into());

      MovieDetailsHandler::with(&SUBMIT_KEY, &mut app, &prompt_block).handle();

      assert!(app.data.radarr_data.prompt_confirm);
      assert_eq!(
        app.get_current_route(),
        &ActiveRadarrBlock::MovieDetails.into()
      );
      assert_eq!(
        app.data.radarr_data.prompt_confirm_action,
        Some(expected_action)
      );
    }

    #[rstest]
    fn test_movie_info_prompt_decline_submit(
      #[values(
        ActiveRadarrBlock::AutomaticallySearchMoviePrompt,
        ActiveRadarrBlock::RefreshAndScanPrompt,
        ActiveRadarrBlock::ManualSearchConfirmPrompt
      )]
      prompt_block: ActiveRadarrBlock,
    ) {
      let mut app = App::default();
      app.push_navigation_stack(ActiveRadarrBlock::MovieDetails.into());
      app.push_navigation_stack(prompt_block.into());

      MovieDetailsHandler::with(&SUBMIT_KEY, &mut app, &prompt_block).handle();

      assert!(!app.data.radarr_data.prompt_confirm);
      assert_eq!(
        app.get_current_route(),
        &ActiveRadarrBlock::MovieDetails.into()
      );
      assert_eq!(app.data.radarr_data.prompt_confirm_action, None);
    }
  }

  mod test_handle_esc {
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    use crate::app::radarr::radarr_test_utils::create_test_radarr_data;
    use crate::assert_movie_info_tabs_reset;

    use super::*;

    const ESC_KEY: Key = DEFAULT_KEYBINDINGS.esc.key;

    #[rstest]
    fn test_movie_info_tabs_esc(
      #[values(
        ActiveRadarrBlock::MovieDetails,
        ActiveRadarrBlock::MovieHistory,
        ActiveRadarrBlock::FileInfo,
        ActiveRadarrBlock::Cast,
        ActiveRadarrBlock::Crew,
        ActiveRadarrBlock::ManualSearch
      )]
      active_radarr_block: ActiveRadarrBlock,
    ) {
      let mut app = App::default();
      app.data.radarr_data = create_test_radarr_data();
      app.push_navigation_stack(ActiveRadarrBlock::Movies.into());
      app.push_navigation_stack(active_radarr_block.into());

      MovieDetailsHandler::with(&ESC_KEY, &mut app, &active_radarr_block).handle();

      assert_eq!(app.get_current_route(), &ActiveRadarrBlock::Movies.into());
      assert_movie_info_tabs_reset!(app.data.radarr_data);
    }

    #[rstest]
    fn test_movie_info_prompts_esc(
      #[values(
        ActiveRadarrBlock::AutomaticallySearchMoviePrompt,
        ActiveRadarrBlock::RefreshAndScanPrompt,
        ActiveRadarrBlock::ManualSearchConfirmPrompt
      )]
      prompt_block: ActiveRadarrBlock,
    ) {
      let mut app = App::default();
      app.data.radarr_data.prompt_confirm = true;
      app.push_navigation_stack(ActiveRadarrBlock::Movies.into());
      app.push_navigation_stack(prompt_block.into());

      MovieDetailsHandler::with(&ESC_KEY, &mut app, &prompt_block).handle();

      assert!(!app.data.radarr_data.prompt_confirm);
      assert_eq!(app.get_current_route(), &ActiveRadarrBlock::Movies.into());
    }
  }

  mod test_handle_key_char {
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    use super::*;

    #[rstest]
    fn test_search_key(
      #[values(
        ActiveRadarrBlock::MovieDetails,
        ActiveRadarrBlock::MovieHistory,
        ActiveRadarrBlock::FileInfo,
        ActiveRadarrBlock::Cast,
        ActiveRadarrBlock::Crew,
        ActiveRadarrBlock::ManualSearch
      )]
      active_radarr_block: ActiveRadarrBlock,
    ) {
      let mut app = App::default();

      MovieDetailsHandler::with(
        &DEFAULT_KEYBINDINGS.search.key,
        &mut app,
        &active_radarr_block,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        &ActiveRadarrBlock::AutomaticallySearchMoviePrompt.into()
      );
    }

    #[rstest]
    fn test_refresh_key(
      #[values(
        ActiveRadarrBlock::MovieDetails,
        ActiveRadarrBlock::MovieHistory,
        ActiveRadarrBlock::FileInfo,
        ActiveRadarrBlock::Cast,
        ActiveRadarrBlock::Crew,
        ActiveRadarrBlock::ManualSearch
      )]
      active_radarr_block: ActiveRadarrBlock,
    ) {
      let mut app = App::default();

      MovieDetailsHandler::with(
        &DEFAULT_KEYBINDINGS.refresh.key,
        &mut app,
        &active_radarr_block,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        &ActiveRadarrBlock::RefreshAndScanPrompt.into()
      );
    }
  }
}
