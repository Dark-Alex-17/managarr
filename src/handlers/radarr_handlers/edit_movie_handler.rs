use crate::app::key_binding::DEFAULT_KEYBINDINGS;
use crate::app::radarr::ActiveRadarrBlock;
use crate::app::App;
use crate::event::Key;
use crate::handlers::{handle_prompt_toggle, KeyEventHandler};
use crate::models::Scrollable;
use crate::network::radarr_network::RadarrEvent;
use crate::{handle_text_box_keys, handle_text_box_left_right_keys};

pub(super) struct EditMovieHandler<'a> {
  key: &'a Key,
  app: &'a mut App,
  active_radarr_block: &'a ActiveRadarrBlock,
  context: &'a Option<ActiveRadarrBlock>,
}

impl<'a> KeyEventHandler<'a, ActiveRadarrBlock> for EditMovieHandler<'a> {
  fn with(
    key: &'a Key,
    app: &'a mut App,
    active_block: &'a ActiveRadarrBlock,
    context: &'a Option<ActiveRadarrBlock>,
  ) -> EditMovieHandler<'a> {
    EditMovieHandler {
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
      ActiveRadarrBlock::EditMovieSelectMinimumAvailability => self
        .app
        .data
        .radarr_data
        .movie_minimum_availability_list
        .scroll_up(),
      ActiveRadarrBlock::EditMovieSelectQualityProfile => self
        .app
        .data
        .radarr_data
        .movie_quality_profile_list
        .scroll_up(),
      ActiveRadarrBlock::EditMoviePrompt => {
        self.app.data.radarr_data.selected_block = self
          .app
          .data
          .radarr_data
          .selected_block
          .clone()
          .previous_edit_prompt_block()
      }
      _ => (),
    }
  }

  fn handle_scroll_down(&mut self) {
    match self.active_radarr_block {
      ActiveRadarrBlock::EditMovieSelectMinimumAvailability => self
        .app
        .data
        .radarr_data
        .movie_minimum_availability_list
        .scroll_down(),
      ActiveRadarrBlock::EditMovieSelectQualityProfile => self
        .app
        .data
        .radarr_data
        .movie_quality_profile_list
        .scroll_down(),
      ActiveRadarrBlock::EditMoviePrompt => {
        self.app.data.radarr_data.selected_block = self
          .app
          .data
          .radarr_data
          .selected_block
          .next_edit_prompt_block()
      }
      _ => (),
    }
  }

  fn handle_home(&mut self) {
    match self.active_radarr_block {
      ActiveRadarrBlock::EditMovieSelectMinimumAvailability => self
        .app
        .data
        .radarr_data
        .movie_minimum_availability_list
        .scroll_to_top(),
      ActiveRadarrBlock::EditMovieSelectQualityProfile => self
        .app
        .data
        .radarr_data
        .movie_quality_profile_list
        .scroll_to_top(),
      ActiveRadarrBlock::EditMoviePathInput => self.app.data.radarr_data.edit_path.scroll_home(),
      ActiveRadarrBlock::EditMovieTagsInput => self.app.data.radarr_data.edit_tags.scroll_home(),
      _ => (),
    }
  }

  fn handle_end(&mut self) {
    match self.active_radarr_block {
      ActiveRadarrBlock::EditMovieSelectMinimumAvailability => self
        .app
        .data
        .radarr_data
        .movie_minimum_availability_list
        .scroll_to_bottom(),
      ActiveRadarrBlock::EditMovieSelectQualityProfile => self
        .app
        .data
        .radarr_data
        .movie_quality_profile_list
        .scroll_to_bottom(),
      ActiveRadarrBlock::EditMoviePathInput => self.app.data.radarr_data.edit_path.reset_offset(),
      ActiveRadarrBlock::EditMovieTagsInput => self.app.data.radarr_data.edit_tags.reset_offset(),
      _ => (),
    }
  }

  fn handle_delete(&mut self) {}

  fn handle_left_right_action(&mut self) {
    match self.active_radarr_block {
      ActiveRadarrBlock::EditMoviePrompt => handle_prompt_toggle(self.app, self.key),
      ActiveRadarrBlock::EditMoviePathInput => {
        handle_text_box_left_right_keys!(self, self.key, self.app.data.radarr_data.edit_path)
      }
      ActiveRadarrBlock::EditMovieTagsInput => {
        handle_text_box_left_right_keys!(self, self.key, self.app.data.radarr_data.edit_tags)
      }
      _ => (),
    }
  }

  fn handle_submit(&mut self) {
    match self.active_radarr_block {
      ActiveRadarrBlock::EditMoviePrompt => match self.app.data.radarr_data.selected_block {
        ActiveRadarrBlock::EditMovieConfirmPrompt => {
          if self.app.data.radarr_data.prompt_confirm {
            self.app.data.radarr_data.prompt_confirm_action = Some(RadarrEvent::EditMovie);
            self.app.pop_navigation_stack();
          } else {
            self.app.pop_navigation_stack();
          }
        }
        ActiveRadarrBlock::EditMovieSelectMinimumAvailability
        | ActiveRadarrBlock::EditMovieSelectQualityProfile => self
          .app
          .push_navigation_stack((self.app.data.radarr_data.selected_block, *self.context).into()),
        ActiveRadarrBlock::EditMoviePathInput | ActiveRadarrBlock::EditMovieTagsInput => {
          self.app.push_navigation_stack(
            (self.app.data.radarr_data.selected_block, *self.context).into(),
          );
          self.app.should_ignore_quit_key = true;
        }
        ActiveRadarrBlock::EditMovieToggleMonitored => {
          self.app.data.radarr_data.edit_monitored =
            Some(!self.app.data.radarr_data.edit_monitored.unwrap_or_default())
        }
        _ => (),
      },
      ActiveRadarrBlock::EditMovieSelectMinimumAvailability
      | ActiveRadarrBlock::EditMovieSelectQualityProfile => self.app.pop_navigation_stack(),
      ActiveRadarrBlock::EditMoviePathInput | ActiveRadarrBlock::EditMovieTagsInput => {
        self.app.pop_navigation_stack();
        self.app.should_ignore_quit_key = false;
      }
      _ => (),
    }
  }

  fn handle_esc(&mut self) {
    match self.active_radarr_block {
      ActiveRadarrBlock::EditMovieTagsInput | ActiveRadarrBlock::EditMoviePathInput => {
        self.app.pop_navigation_stack();
        self.app.should_ignore_quit_key = false;
      }
      ActiveRadarrBlock::EditMoviePrompt => {
        self.app.pop_navigation_stack();
        self.app.data.radarr_data.reset_add_edit_movie_fields();
        self.app.data.radarr_data.prompt_confirm = false;
      }
      ActiveRadarrBlock::EditMovieToggleMonitored
      | ActiveRadarrBlock::EditMovieSelectMinimumAvailability
      | ActiveRadarrBlock::EditMovieSelectQualityProfile => self.app.pop_navigation_stack(),
      _ => (),
    }
  }

  fn handle_char_key_event(&mut self) {
    let key = self.key;
    match self.active_radarr_block {
      ActiveRadarrBlock::EditMoviePathInput => {
        handle_text_box_keys!(self, key, self.app.data.radarr_data.edit_path)
      }
      ActiveRadarrBlock::EditMovieTagsInput => {
        handle_text_box_keys!(self, key, self.app.data.radarr_data.edit_tags)
      }
      _ => (),
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
  use crate::handlers::radarr_handlers::edit_movie_handler::EditMovieHandler;
  use crate::handlers::KeyEventHandler;
  use crate::models::radarr_models::{MinimumAvailability, Monitor};

  mod test_handle_scroll_up_and_down {
    use pretty_assertions::assert_eq;
    use rstest::rstest;
    use strum::IntoEnumIterator;

    use crate::{test_enum_scroll, test_iterable_scroll};

    use super::*;

    test_enum_scroll!(
      test_edit_movie_select_minimuum_availability_scroll,
      EditMovieHandler,
      MinimumAvailability,
      movie_minimum_availability_list,
      ActiveRadarrBlock::EditMovieSelectMinimumAvailability,
      None
    );

    test_iterable_scroll!(
      test_edit_movie_select_quality_profile_scroll,
      EditMovieHandler,
      movie_quality_profile_list,
      ActiveRadarrBlock::EditMovieSelectQualityProfile,
      None
    );

    #[rstest]
    fn test_edit_movie_prompt_scroll(#[values(Key::Up, Key::Down)] key: Key) {
      let mut app = App::default();
      app.data.radarr_data.selected_block = ActiveRadarrBlock::EditMovieSelectMinimumAvailability;

      EditMovieHandler::with(&key, &mut app, &ActiveRadarrBlock::EditMoviePrompt, &None).handle();

      if key == Key::Up {
        assert_eq!(
          app.data.radarr_data.selected_block,
          ActiveRadarrBlock::EditMovieToggleMonitored
        );
      } else {
        assert_eq!(
          app.data.radarr_data.selected_block,
          ActiveRadarrBlock::EditMovieSelectQualityProfile
        );
      }
    }
  }

  mod test_handle_home_end {
    use strum::IntoEnumIterator;

    use crate::{test_enum_home_and_end, test_iterable_home_and_end, test_text_box_home_end_keys};

    use super::*;

    test_enum_home_and_end!(
      test_edit_movie_select_minimuum_availability_home_end,
      EditMovieHandler,
      MinimumAvailability,
      movie_minimum_availability_list,
      ActiveRadarrBlock::EditMovieSelectMinimumAvailability,
      None
    );

    test_iterable_home_and_end!(
      test_edit_movie_select_quality_profile_scroll,
      EditMovieHandler,
      movie_quality_profile_list,
      ActiveRadarrBlock::EditMovieSelectQualityProfile,
      None
    );

    #[test]
    fn test_edit_movie_path_input_home_end_keys() {
      test_text_box_home_end_keys!(
        EditMovieHandler,
        ActiveRadarrBlock::EditMoviePathInput,
        edit_path
      );
    }

    #[test]
    fn test_edit_movie_tags_input_home_end_keys() {
      test_text_box_home_end_keys!(
        EditMovieHandler,
        ActiveRadarrBlock::EditMovieTagsInput,
        edit_tags
      );
    }
  }

  mod test_handle_left_right_action {
    use rstest::rstest;

    use crate::{test_text_box_home_end_keys, test_text_box_left_right_keys};

    use super::*;

    #[rstest]
    fn test_left_right_prompt_toggle(#[values(Key::Left, Key::Right)] key: Key) {
      let mut app = App::default();

      EditMovieHandler::with(&key, &mut app, &ActiveRadarrBlock::EditMoviePrompt, &None).handle();

      assert!(app.data.radarr_data.prompt_confirm);

      EditMovieHandler::with(&key, &mut app, &ActiveRadarrBlock::EditMoviePrompt, &None).handle();

      assert!(!app.data.radarr_data.prompt_confirm);
    }

    #[test]
    fn test_edit_movie_path_input_left_right_keys() {
      test_text_box_left_right_keys!(
        EditMovieHandler,
        ActiveRadarrBlock::EditMoviePathInput,
        edit_path
      );
    }

    #[test]
    fn test_edit_movie_tags_input_left_right_keys() {
      test_text_box_left_right_keys!(
        EditMovieHandler,
        ActiveRadarrBlock::EditMovieTagsInput,
        edit_tags
      );
    }
  }

  mod test_handle_submit {
    use std::collections::HashMap;

    use pretty_assertions::{assert_eq, assert_str_eq};
    use rstest::rstest;

    use crate::app::key_binding::DEFAULT_KEYBINDINGS;
    use crate::models::Route;
    use crate::network::radarr_network::RadarrEvent;

    use super::*;

    const SUBMIT_KEY: Key = DEFAULT_KEYBINDINGS.submit.key;

    #[test]
    fn test_edit_movie_path_input_submit() {
      let mut app = App::default();
      app.should_ignore_quit_key = true;
      app.data.radarr_data.edit_path = "Test Path".to_owned().into();
      app.push_navigation_stack(ActiveRadarrBlock::EditMoviePrompt.into());
      app.push_navigation_stack(ActiveRadarrBlock::EditMoviePathInput.into());

      EditMovieHandler::with(
        &SUBMIT_KEY,
        &mut app,
        &ActiveRadarrBlock::EditMoviePathInput,
        &None,
      )
      .handle();

      assert!(!app.should_ignore_quit_key);
      assert!(!app.data.radarr_data.edit_path.text.is_empty());
      assert_eq!(
        app.get_current_route(),
        &ActiveRadarrBlock::EditMoviePrompt.into()
      );
    }

    #[test]
    fn test_edit_movie_tags_input_submit() {
      let mut app = App::default();
      app.should_ignore_quit_key = true;
      app.data.radarr_data.edit_tags = "Test Tags".to_owned().into();
      app.push_navigation_stack(ActiveRadarrBlock::EditMoviePrompt.into());
      app.push_navigation_stack(ActiveRadarrBlock::EditMoviePathInput.into());

      EditMovieHandler::with(
        &SUBMIT_KEY,
        &mut app,
        &ActiveRadarrBlock::EditMovieTagsInput,
        &None,
      )
      .handle();

      assert!(!app.should_ignore_quit_key);
      assert!(!app.data.radarr_data.edit_tags.text.is_empty());
      assert_eq!(
        app.get_current_route(),
        &ActiveRadarrBlock::EditMoviePrompt.into()
      );
    }

    #[test]
    fn test_edit_movie_prompt_prompt_decline_submit() {
      let mut app = App::default();
      app.push_navigation_stack(ActiveRadarrBlock::Movies.into());
      app.push_navigation_stack(ActiveRadarrBlock::EditMoviePrompt.into());
      app.data.radarr_data.selected_block = ActiveRadarrBlock::EditMovieConfirmPrompt;

      EditMovieHandler::with(
        &SUBMIT_KEY,
        &mut app,
        &ActiveRadarrBlock::EditMoviePrompt,
        &None,
      )
      .handle();

      assert_eq!(app.get_current_route(), &ActiveRadarrBlock::Movies.into());
      assert_eq!(app.data.radarr_data.prompt_confirm_action, None);
    }

    #[test]
    fn test_edit_movie_confirm_prompt_prompt_confirmation_submit() {
      let mut app = App::default();
      app.push_navigation_stack(ActiveRadarrBlock::Movies.into());
      app.push_navigation_stack(ActiveRadarrBlock::EditMoviePrompt.into());
      app.data.radarr_data.prompt_confirm = true;
      app.data.radarr_data.selected_block = ActiveRadarrBlock::EditMovieConfirmPrompt;

      EditMovieHandler::with(
        &SUBMIT_KEY,
        &mut app,
        &ActiveRadarrBlock::EditMoviePrompt,
        &None,
      )
      .handle();

      assert_eq!(app.get_current_route(), &ActiveRadarrBlock::Movies.into());
      assert_eq!(
        app.data.radarr_data.prompt_confirm_action,
        Some(RadarrEvent::EditMovie)
      );
    }

    #[test]
    fn test_edit_movie_toggle_monitored_submit() {
      let current_route = Route::from((
        ActiveRadarrBlock::EditMoviePrompt,
        Some(ActiveRadarrBlock::Movies),
      ));
      let mut app = App::default();
      app.data.radarr_data.selected_block = ActiveRadarrBlock::EditMovieToggleMonitored;
      app.push_navigation_stack(current_route);

      EditMovieHandler::with(
        &SUBMIT_KEY,
        &mut app,
        &ActiveRadarrBlock::EditMoviePrompt,
        &Some(ActiveRadarrBlock::Movies),
      )
      .handle();

      assert_eq!(app.get_current_route(), &current_route);
      assert_eq!(app.data.radarr_data.edit_monitored, Some(true));

      EditMovieHandler::with(
        &SUBMIT_KEY,
        &mut app,
        &ActiveRadarrBlock::EditMoviePrompt,
        &Some(ActiveRadarrBlock::Movies),
      )
      .handle();

      assert_eq!(app.get_current_route(), &current_route);
      assert_eq!(app.data.radarr_data.edit_monitored, Some(false));
    }

    #[rstest]
    fn test_edit_movie_prompt_selected_block_submit(
      #[values(
        ActiveRadarrBlock::EditMovieSelectMinimumAvailability,
        ActiveRadarrBlock::EditMovieSelectQualityProfile,
        ActiveRadarrBlock::EditMoviePathInput,
        ActiveRadarrBlock::EditMovieTagsInput
      )]
      selected_block: ActiveRadarrBlock,
    ) {
      let mut app = App::default();
      app.push_navigation_stack(
        (
          ActiveRadarrBlock::EditMoviePrompt,
          Some(ActiveRadarrBlock::Movies),
        )
          .into(),
      );
      app.data.radarr_data.selected_block = selected_block;

      EditMovieHandler::with(
        &SUBMIT_KEY,
        &mut app,
        &ActiveRadarrBlock::EditMoviePrompt,
        &Some(ActiveRadarrBlock::Movies),
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        &(selected_block, Some(ActiveRadarrBlock::Movies)).into()
      );
      assert_eq!(app.data.radarr_data.prompt_confirm_action, None);

      if selected_block == ActiveRadarrBlock::EditMoviePathInput
        || selected_block == ActiveRadarrBlock::EditMovieTagsInput
      {
        assert!(app.should_ignore_quit_key);
      }
    }

    #[rstest]
    fn test_edit_movie_prompt_selecting_preferences_blocks_submit(
      #[values(
        ActiveRadarrBlock::EditMovieSelectMinimumAvailability,
        ActiveRadarrBlock::EditMovieSelectQualityProfile,
        ActiveRadarrBlock::EditMoviePathInput,
        ActiveRadarrBlock::EditMovieTagsInput
      )]
      active_radarr_block: ActiveRadarrBlock,
    ) {
      let mut app = App::default();
      app.push_navigation_stack(ActiveRadarrBlock::EditMoviePrompt.into());
      app.push_navigation_stack(active_radarr_block.into());

      EditMovieHandler::with(
        &SUBMIT_KEY,
        &mut app,
        &active_radarr_block,
        &Some(ActiveRadarrBlock::Movies),
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        &ActiveRadarrBlock::EditMoviePrompt.into()
      );

      if active_radarr_block == ActiveRadarrBlock::EditMoviePathInput
        || active_radarr_block == ActiveRadarrBlock::EditMovieTagsInput
      {
        assert!(!app.should_ignore_quit_key);
      }
    }
  }

  mod test_handle_esc {
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    use crate::app::radarr::radarr_test_utils::create_test_radarr_data;
    use crate::{assert_edit_movie_reset, assert_movie_preferences_selections_reset};

    use super::*;

    const ESC_KEY: Key = DEFAULT_KEYBINDINGS.esc.key;

    #[rstest]
    fn test_edit_movie_input_esc(
      #[values(
        ActiveRadarrBlock::EditMovieTagsInput,
        ActiveRadarrBlock::EditMoviePathInput
      )]
      active_radarr_block: ActiveRadarrBlock,
    ) {
      let mut app = App::default();
      app.data.radarr_data = create_test_radarr_data();
      app.should_ignore_quit_key = true;
      app.push_navigation_stack(ActiveRadarrBlock::EditMoviePrompt.into());
      app.push_navigation_stack(active_radarr_block.into());

      EditMovieHandler::with(&ESC_KEY, &mut app, &active_radarr_block, &None).handle();

      assert!(!app.should_ignore_quit_key);
      assert_eq!(
        app.get_current_route(),
        &ActiveRadarrBlock::EditMoviePrompt.into()
      );
    }

    #[test]
    fn test_edit_movie_prompt_esc() {
      let mut app = App::default();
      app.push_navigation_stack(ActiveRadarrBlock::EditMoviePrompt.into());
      app.data.radarr_data = create_test_radarr_data();

      EditMovieHandler::with(
        &ESC_KEY,
        &mut app,
        &ActiveRadarrBlock::EditMoviePrompt,
        &None,
      )
      .handle();

      assert_eq!(app.get_current_route(), &ActiveRadarrBlock::Movies.into());
      {
        let radarr_data = &app.data.radarr_data;

        assert_movie_preferences_selections_reset!(radarr_data);
        assert_edit_movie_reset!(radarr_data);
        assert!(!radarr_data.prompt_confirm);
      }
    }

    #[rstest]
    fn test_edit_movie_esc(
      #[values(
        ActiveRadarrBlock::EditMovieToggleMonitored,
        ActiveRadarrBlock::EditMovieSelectMinimumAvailability,
        ActiveRadarrBlock::EditMovieSelectQualityProfile
      )]
      active_radarr_block: ActiveRadarrBlock,
    ) {
      let mut app = App::default();
      app.data.radarr_data = create_test_radarr_data();
      app.push_navigation_stack(active_radarr_block.into());

      EditMovieHandler::with(&ESC_KEY, &mut app, &active_radarr_block, &None).handle();

      assert_eq!(app.get_current_route(), &ActiveRadarrBlock::Movies.into());
    }
  }

  mod test_handle_key_char {
    use super::*;

    #[test]
    fn test_edit_movie_path_input_backspace() {
      let mut app = App::default();
      app.data.radarr_data.edit_path = "Test".to_owned().into();

      EditMovieHandler::with(
        &DEFAULT_KEYBINDINGS.backspace.key,
        &mut app,
        &ActiveRadarrBlock::EditMoviePathInput,
        &None,
      )
      .handle();

      assert_str_eq!(app.data.radarr_data.edit_path.text, "Tes");
    }

    #[test]
    fn test_edit_movie_tags_input_backspace() {
      let mut app = App::default();
      app.data.radarr_data.edit_tags = "Test".to_owned().into();

      EditMovieHandler::with(
        &DEFAULT_KEYBINDINGS.backspace.key,
        &mut app,
        &ActiveRadarrBlock::EditMovieTagsInput,
        &None,
      )
      .handle();

      assert_str_eq!(app.data.radarr_data.edit_tags.text, "Tes");
    }

    #[test]
    fn test_edit_movie_path_input_char_key() {
      let mut app = App::default();

      EditMovieHandler::with(
        &Key::Char('h'),
        &mut app,
        &ActiveRadarrBlock::EditMoviePathInput,
        &None,
      )
      .handle();

      assert_str_eq!(app.data.radarr_data.edit_path.text, "h");
    }

    #[test]
    fn test_edit_movie_tags_input_char_key() {
      let mut app = App::default();

      EditMovieHandler::with(
        &Key::Char('h'),
        &mut app,
        &ActiveRadarrBlock::EditMovieTagsInput,
        &None,
      )
      .handle();

      assert_str_eq!(app.data.radarr_data.edit_tags.text, "h");
    }
  }
}
