use std::cmp::Ordering;

use serde_json::Number;
use strum::IntoEnumIterator;

use crate::app::key_binding::DEFAULT_KEYBINDINGS;
use crate::app::radarr::{ActiveRadarrBlock, EDIT_MOVIE_SELECTION_BLOCKS};
use crate::app::App;
use crate::event::Key;
use crate::handlers::{handle_prompt_toggle, KeyEventHandler};
use crate::models::radarr_models::{Language, Release, ReleaseField};
use crate::models::{BlockSelectionState, Scrollable};
use crate::network::radarr_network::RadarrEvent;

pub(super) struct MovieDetailsHandler<'a, 'b> {
  key: &'a Key,
  app: &'a mut App<'b>,
  active_radarr_block: &'a ActiveRadarrBlock,
  _context: &'a Option<ActiveRadarrBlock>,
}

impl<'a, 'b> KeyEventHandler<'a, 'b, ActiveRadarrBlock> for MovieDetailsHandler<'a, 'b> {
  fn with(
    key: &'a Key,
    app: &'a mut App<'b>,
    active_block: &'a ActiveRadarrBlock,
    _context: &'a Option<ActiveRadarrBlock>,
  ) -> MovieDetailsHandler<'a, 'b> {
    MovieDetailsHandler {
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
    match self.active_radarr_block {
      ActiveRadarrBlock::MovieDetails => self.app.data.radarr_data.movie_details.scroll_up(),
      ActiveRadarrBlock::MovieHistory => self.app.data.radarr_data.movie_history.scroll_up(),
      ActiveRadarrBlock::Cast => self.app.data.radarr_data.movie_cast.scroll_up(),
      ActiveRadarrBlock::Crew => self.app.data.radarr_data.movie_crew.scroll_up(),
      ActiveRadarrBlock::ManualSearch => self.app.data.radarr_data.movie_releases.scroll_up(),
      ActiveRadarrBlock::ManualSearchSortPrompt => {
        self.app.data.radarr_data.movie_releases_sort.scroll_up()
      }
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
      ActiveRadarrBlock::ManualSearchSortPrompt => {
        self.app.data.radarr_data.movie_releases_sort.scroll_down()
      }
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
      ActiveRadarrBlock::ManualSearchSortPrompt => self
        .app
        .data
        .radarr_data
        .movie_releases_sort
        .scroll_to_top(),
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
      ActiveRadarrBlock::ManualSearchSortPrompt => self
        .app
        .data
        .radarr_data
        .movie_releases_sort
        .scroll_to_bottom(),
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
      | ActiveRadarrBlock::UpdateAndScanPrompt
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
      ActiveRadarrBlock::UpdateAndScanPrompt => {
        if self.app.data.radarr_data.prompt_confirm {
          self.app.data.radarr_data.prompt_confirm_action = Some(RadarrEvent::UpdateAndScan);
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
      ActiveRadarrBlock::ManualSearchSortPrompt => {
        let movie_releases = self.app.data.radarr_data.movie_releases.items.clone();
        let field = self
          .app
          .data
          .radarr_data
          .movie_releases_sort
          .current_selection();
        let sort_ascending = !self.app.data.radarr_data.sort_ascending.unwrap();
        self.app.data.radarr_data.sort_ascending = Some(sort_ascending);

        self
          .app
          .data
          .radarr_data
          .movie_releases
          .set_items(sort_releases_by_selected_field(
            movie_releases,
            *field,
            sort_ascending,
          ));
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
      | ActiveRadarrBlock::UpdateAndScanPrompt
      | ActiveRadarrBlock::ManualSearchConfirmPrompt
      | ActiveRadarrBlock::ManualSearchSortPrompt => {
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
        _ if *key == DEFAULT_KEYBINDINGS.edit.key => {
          self.app.push_navigation_stack(
            (
              ActiveRadarrBlock::EditMoviePrompt,
              Some(*self.active_radarr_block),
            )
              .into(),
          );
          self.app.data.radarr_data.populate_edit_movie_fields();
          self.app.data.radarr_data.selected_block =
            BlockSelectionState::new(&EDIT_MOVIE_SELECTION_BLOCKS);
        }
        _ if *key == DEFAULT_KEYBINDINGS.update.key => {
          self
            .app
            .push_navigation_stack(ActiveRadarrBlock::UpdateAndScanPrompt.into());
        }
        _ if *key == DEFAULT_KEYBINDINGS.refresh.key => {
          self
            .app
            .pop_and_push_navigation_stack((*self.active_radarr_block).into());
        }
        _ if *key == DEFAULT_KEYBINDINGS.sort.key => {
          self
            .app
            .data
            .radarr_data
            .movie_releases_sort
            .set_items(Vec::from_iter(ReleaseField::iter()));
          let sort_ascending = self.app.data.radarr_data.sort_ascending;
          self.app.data.radarr_data.sort_ascending =
            Some(sort_ascending.is_some() && sort_ascending.unwrap());
          self
            .app
            .push_navigation_stack(ActiveRadarrBlock::ManualSearchSortPrompt.into());
        }
        _ => (),
      },
      _ => (),
    }
  }
}

fn sort_releases_by_selected_field(
  mut releases: Vec<Release>,
  field: ReleaseField,
  sort_ascending: bool,
) -> Vec<Release> {
  let cmp_fn: fn(&Release, &Release) -> Ordering = match field {
    ReleaseField::Source => |release_a, release_b| release_a.protocol.cmp(&release_b.protocol),
    ReleaseField::Age => |release_a, release_b| release_a.age.as_u64().cmp(&release_b.age.as_u64()),
    ReleaseField::Rejected => |release_a, release_b| release_a.rejected.cmp(&release_b.rejected),
    ReleaseField::Title => |release_a, release_b| release_a.title.text.cmp(&release_b.title.text),
    ReleaseField::Indexer => |release_a, release_b| release_a.indexer.cmp(&release_b.indexer),
    ReleaseField::Size => |release_a, release_b| {
      release_a
        .size
        .as_u64()
        .as_ref()
        .unwrap()
        .cmp(release_b.size.as_u64().as_ref().unwrap())
    },
    ReleaseField::Peers => |release_a, release_b| {
      let default_number = Number::from(i64::MAX);
      let seeder_a = release_a
        .seeders
        .as_ref()
        .unwrap_or(&default_number)
        .as_u64()
        .unwrap();
      let seeder_b = release_b
        .seeders
        .as_ref()
        .unwrap_or(&default_number)
        .as_u64()
        .unwrap();

      seeder_a.cmp(&seeder_b)
    },
    ReleaseField::Language => |release_a, release_b| {
      let default_language_vec = vec![Language {
        name: "_".to_owned(),
      }];
      let language_a = &release_a
        .languages
        .as_ref()
        .unwrap_or(&default_language_vec)[0];
      let language_b = &release_b
        .languages
        .as_ref()
        .unwrap_or(&default_language_vec)[0];

      language_a.cmp(language_b)
    },
    ReleaseField::Quality => |release_a, release_b| release_a.quality.cmp(&release_b.quality),
  };

  if !sort_ascending {
    releases.sort_by(|release_a, release_b| cmp_fn(release_a, release_b).reverse());
  } else {
    releases.sort_by(cmp_fn);
  }

  releases
}

#[cfg(test)]
mod tests {
  use pretty_assertions::assert_str_eq;
  use rstest::rstest;
  use serde_json::Number;

  use crate::app::key_binding::DEFAULT_KEYBINDINGS;
  use crate::app::radarr::ActiveRadarrBlock;
  use crate::app::App;
  use crate::event::Key;
  use crate::handlers::radarr_handlers::movie_details_handler::{
    sort_releases_by_selected_field, MovieDetailsHandler,
  };
  use crate::handlers::KeyEventHandler;
  use crate::models::radarr_models::{
    Credit, Language, MovieHistoryItem, Quality, QualityWrapper, Release, ReleaseField,
  };
  use crate::models::{HorizontallyScrollableText, ScrollableText};

  mod test_handle_scroll_up_and_down {
    use pretty_assertions::assert_eq;
    use rstest::rstest;
    use strum::IntoEnumIterator;

    use crate::models::radarr_models::ReleaseField;
    use crate::{simple_stateful_iterable_vec, test_enum_scroll, test_iterable_scroll};

    use super::*;

    #[test]
    fn test_movie_details_scroll() {
      let mut app = App::default();
      app.data.radarr_data.movie_details = ScrollableText::with_string("Test 1\nTest 2".to_owned());

      MovieDetailsHandler::with(
        &DEFAULT_KEYBINDINGS.up.key,
        &mut app,
        &ActiveRadarrBlock::MovieDetails,
        &None,
      )
      .handle();

      assert_eq!(app.data.radarr_data.movie_details.offset, 0);

      MovieDetailsHandler::with(
        &DEFAULT_KEYBINDINGS.down.key,
        &mut app,
        &ActiveRadarrBlock::MovieDetails,
        &None,
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
      None,
      source_title,
      to_string
    );

    test_iterable_scroll!(
      test_cast_scroll,
      MovieDetailsHandler,
      movie_cast,
      simple_stateful_iterable_vec!(Credit, String, person_name),
      ActiveRadarrBlock::Cast,
      None,
      person_name,
      to_owned
    );

    test_iterable_scroll!(
      test_crew_scroll,
      MovieDetailsHandler,
      movie_crew,
      simple_stateful_iterable_vec!(Credit, String, person_name),
      ActiveRadarrBlock::Crew,
      None,
      person_name,
      to_owned
    );

    test_iterable_scroll!(
      test_manual_search_scroll,
      MovieDetailsHandler,
      movie_releases,
      simple_stateful_iterable_vec!(Release, HorizontallyScrollableText),
      ActiveRadarrBlock::ManualSearch,
      None,
      title,
      to_string
    );

    test_enum_scroll!(
      test_manual_search_sort_scroll,
      MovieDetailsHandler,
      ReleaseField,
      movie_releases_sort,
      ActiveRadarrBlock::ManualSearchSortPrompt,
      None
    );
  }

  mod test_handle_home_end {
    use strum::IntoEnumIterator;

    use crate::models::radarr_models::ReleaseField;
    use crate::{
      extended_stateful_iterable_vec, test_enum_home_and_end, test_iterable_home_and_end,
    };

    use super::*;

    #[test]
    fn test_movie_details_home_end() {
      let mut app = App::default();
      app.data.radarr_data.movie_details = ScrollableText::with_string("Test 1\nTest 2".to_owned());

      MovieDetailsHandler::with(
        &DEFAULT_KEYBINDINGS.end.key,
        &mut app,
        &ActiveRadarrBlock::MovieDetails,
        &None,
      )
      .handle();

      assert_eq!(app.data.radarr_data.movie_details.offset, 1);

      MovieDetailsHandler::with(
        &DEFAULT_KEYBINDINGS.home.key,
        &mut app,
        &ActiveRadarrBlock::MovieDetails,
        &None,
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
      None,
      source_title,
      to_string
    );

    test_iterable_home_and_end!(
      test_cast_home_end,
      MovieDetailsHandler,
      movie_cast,
      extended_stateful_iterable_vec!(Credit, String, person_name),
      ActiveRadarrBlock::Cast,
      None,
      person_name,
      to_owned
    );

    test_iterable_home_and_end!(
      test_crew_home_end,
      MovieDetailsHandler,
      movie_crew,
      extended_stateful_iterable_vec!(Credit, String, person_name),
      ActiveRadarrBlock::Crew,
      None,
      person_name,
      to_owned
    );

    test_iterable_home_and_end!(
      test_manual_search_home_end,
      MovieDetailsHandler,
      movie_releases,
      extended_stateful_iterable_vec!(Release, HorizontallyScrollableText),
      ActiveRadarrBlock::ManualSearch,
      None,
      title,
      to_string
    );

    test_enum_home_and_end!(
      test_manual_search_sort_home_end,
      MovieDetailsHandler,
      ReleaseField,
      movie_releases_sort,
      ActiveRadarrBlock::ManualSearchSortPrompt,
      None
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
        ActiveRadarrBlock::UpdateAndScanPrompt,
        ActiveRadarrBlock::ManualSearchConfirmPrompt
      )]
      active_radarr_block: ActiveRadarrBlock,
      #[values(Key::Left, Key::Right)] key: Key,
    ) {
      let mut app = App::default();

      MovieDetailsHandler::with(&key, &mut app, &active_radarr_block, &None).handle();

      assert!(app.data.radarr_data.prompt_confirm);

      MovieDetailsHandler::with(&key, &mut app, &active_radarr_block, &None).handle();

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

      MovieDetailsHandler::with(&DEFAULT_KEYBINDINGS.left.key, &mut app, &right_block, &None)
        .handle();

      assert_eq!(
        app.get_current_route(),
        app.data.radarr_data.movie_info_tabs.get_active_route()
      );
      assert_eq!(app.get_current_route(), &left_block.into());

      MovieDetailsHandler::with(&DEFAULT_KEYBINDINGS.right.key, &mut app, &left_block, &None)
        .handle();

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

    use crate::models::radarr_models::ReleaseField;
    use crate::network::radarr_network::RadarrEvent;

    use super::*;

    const SUBMIT_KEY: Key = DEFAULT_KEYBINDINGS.submit.key;

    #[test]
    fn test_manual_search_submit() {
      let mut app = App::default();
      app.push_navigation_stack(ActiveRadarrBlock::ManualSearch.into());

      MovieDetailsHandler::with(
        &SUBMIT_KEY,
        &mut app,
        &ActiveRadarrBlock::ManualSearch,
        &None,
      )
      .handle();

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
    #[case(ActiveRadarrBlock::UpdateAndScanPrompt, RadarrEvent::UpdateAndScan)]
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

      MovieDetailsHandler::with(&SUBMIT_KEY, &mut app, &prompt_block, &None).handle();

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
        ActiveRadarrBlock::UpdateAndScanPrompt,
        ActiveRadarrBlock::ManualSearchConfirmPrompt
      )]
      prompt_block: ActiveRadarrBlock,
    ) {
      let mut app = App::default();
      app.push_navigation_stack(ActiveRadarrBlock::MovieDetails.into());
      app.push_navigation_stack(prompt_block.into());

      MovieDetailsHandler::with(&SUBMIT_KEY, &mut app, &prompt_block, &None).handle();

      assert!(!app.data.radarr_data.prompt_confirm);
      assert_eq!(
        app.get_current_route(),
        &ActiveRadarrBlock::MovieDetails.into()
      );
      assert_eq!(app.data.radarr_data.prompt_confirm_action, None);
    }

    #[test]
    fn test_manual_search_sort_prompt_submit() {
      let mut app = App::default();
      app.push_navigation_stack(ActiveRadarrBlock::ManualSearch.into());
      app.push_navigation_stack(ActiveRadarrBlock::ManualSearchSortPrompt.into());
      app.data.radarr_data.sort_ascending = Some(true);
      app
        .data
        .radarr_data
        .movie_releases_sort
        .set_items(vec![ReleaseField::default()]);
      app.data.radarr_data.movie_releases.set_items(release_vec());

      let mut expected_vec = release_vec();
      expected_vec.reverse();

      MovieDetailsHandler::with(
        &SUBMIT_KEY,
        &mut app,
        &ActiveRadarrBlock::ManualSearchSortPrompt,
        &None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        &ActiveRadarrBlock::ManualSearch.into()
      );
      assert_eq!(app.data.radarr_data.movie_releases.items, expected_vec);
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

      MovieDetailsHandler::with(&ESC_KEY, &mut app, &active_radarr_block, &None).handle();

      assert_eq!(app.get_current_route(), &ActiveRadarrBlock::Movies.into());
      assert_movie_info_tabs_reset!(app.data.radarr_data);
    }

    #[rstest]
    fn test_movie_info_prompts_esc(
      #[values(
        ActiveRadarrBlock::AutomaticallySearchMoviePrompt,
        ActiveRadarrBlock::UpdateAndScanPrompt,
        ActiveRadarrBlock::ManualSearchConfirmPrompt,
        ActiveRadarrBlock::ManualSearchSortPrompt
      )]
      prompt_block: ActiveRadarrBlock,
    ) {
      let mut app = App::default();
      app.data.radarr_data.prompt_confirm = true;
      app.push_navigation_stack(ActiveRadarrBlock::Movies.into());
      app.push_navigation_stack(prompt_block.into());

      MovieDetailsHandler::with(&ESC_KEY, &mut app, &prompt_block, &None).handle();

      assert!(!app.data.radarr_data.prompt_confirm);
      assert_eq!(app.get_current_route(), &ActiveRadarrBlock::Movies.into());
    }
  }

  mod test_handle_key_char {
    use bimap::BiMap;
    use pretty_assertions::{assert_eq, assert_str_eq};
    use rstest::rstest;
    use strum::IntoEnumIterator;

    use crate::app::radarr::radarr_test_utils::create_test_radarr_data;
    use crate::app::radarr::RadarrData;
    use crate::app::radarr::EDIT_MOVIE_SELECTION_BLOCKS;
    use crate::models::radarr_models::{MinimumAvailability, Movie};
    use crate::models::BlockSelectionState;
    use crate::models::HorizontallyScrollableText;
    use crate::models::StatefulTable;
    use crate::test_edit_movie_key;

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
        &None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        &ActiveRadarrBlock::AutomaticallySearchMoviePrompt.into()
      );
    }

    #[test]
    fn test_sort_key() {
      let mut app = App::default();

      MovieDetailsHandler::with(
        &DEFAULT_KEYBINDINGS.sort.key,
        &mut app,
        &ActiveRadarrBlock::ManualSearch,
        &None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        &ActiveRadarrBlock::ManualSearchSortPrompt.into()
      );
      assert!(!app.data.radarr_data.movie_releases_sort.items.is_empty());
      assert!(app.data.radarr_data.sort_ascending.is_some());
      assert_eq!(app.data.radarr_data.sort_ascending, Some(false));
    }

    #[rstest]
    fn test_edit_key(
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
      test_edit_movie_key!(
        MovieDetailsHandler,
        active_radarr_block,
        active_radarr_block
      );
    }

    #[rstest]
    fn test_update_key(
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
        &DEFAULT_KEYBINDINGS.update.key,
        &mut app,
        &active_radarr_block,
        &None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        &ActiveRadarrBlock::UpdateAndScanPrompt.into()
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
        &None,
      )
      .handle();

      assert_eq!(app.get_current_route(), &active_radarr_block.into());
      assert!(app.is_routing);
    }
  }

  #[rstest]
  fn test_sort_releases_by_selected_field(
    #[values(
      ReleaseField::Source,
      ReleaseField::Age,
      ReleaseField::Title,
      ReleaseField::Indexer,
      ReleaseField::Size,
      ReleaseField::Peers,
      ReleaseField::Language,
      ReleaseField::Quality
    )]
    field: ReleaseField,
  ) {
    let mut expected_vec = release_vec();

    let sorted_releases = sort_releases_by_selected_field(release_vec(), field, true);

    assert_eq!(sorted_releases, expected_vec);

    let sorted_releases = sort_releases_by_selected_field(release_vec(), field, false);

    expected_vec.reverse();
    assert_eq!(sorted_releases, expected_vec);
  }

  #[test]
  fn test_sort_releases_by_selected_field_rejected() {
    let mut expected_vec = Vec::from(&release_vec()[1..]);
    expected_vec.push(release_vec()[0].clone());

    let sorted_releases =
      sort_releases_by_selected_field(release_vec(), ReleaseField::Rejected, true);

    assert_eq!(sorted_releases, expected_vec);

    let sorted_releases =
      sort_releases_by_selected_field(release_vec(), ReleaseField::Rejected, false);

    assert_eq!(sorted_releases, release_vec());
  }

  fn release_vec() -> Vec<Release> {
    let release_a = Release {
      protocol: "Protocol A".to_owned(),
      age: Number::from(1),
      title: HorizontallyScrollableText::from("Title A".to_owned()),
      indexer: "Indexer A".to_owned(),
      size: Number::from(1),
      rejected: true,
      seeders: Some(Number::from(1)),
      languages: Some(vec![Language {
        name: "Language A".to_owned(),
      }]),
      quality: QualityWrapper {
        quality: Quality {
          name: "Quality A".to_owned(),
        },
      },
      ..Release::default()
    };
    let release_b = Release {
      protocol: "Protocol B".to_owned(),
      age: Number::from(2),
      title: HorizontallyScrollableText::from("Title B".to_owned()),
      indexer: "Indexer B".to_owned(),
      size: Number::from(2),
      rejected: false,
      seeders: Some(Number::from(2)),
      languages: Some(vec![Language {
        name: "Language B".to_owned(),
      }]),
      quality: QualityWrapper {
        quality: Quality {
          name: "Quality B".to_owned(),
        },
      },
      ..Release::default()
    };
    let release_c = Release {
      protocol: "Protocol C".to_owned(),
      age: Number::from(3),
      title: HorizontallyScrollableText::from("Title C".to_owned()),
      indexer: "Indexer C".to_owned(),
      size: Number::from(3),
      rejected: false,
      seeders: None,
      languages: None,
      quality: QualityWrapper {
        quality: Quality {
          name: "Quality C".to_owned(),
        },
      },
      ..Release::default()
    };

    vec![release_a, release_b, release_c]
  }
}
