use crate::app::key_binding::DEFAULT_KEYBINDINGS;
use crate::app::radarr::ActiveRadarrBlock;
use crate::app::App;
use crate::event::Key;
use crate::handlers::{handle_prompt_toggle, KeyEventHandler};
use crate::models::Scrollable;
use crate::network::radarr_network::RadarrEvent;
use crate::{handle_text_box_keys, handle_text_box_left_right_keys};

pub(super) struct EditCollectionHandler<'a, 'b> {
  key: &'a Key,
  app: &'a mut App<'b>,
  active_radarr_block: &'a ActiveRadarrBlock,
  context: &'a Option<ActiveRadarrBlock>,
}

impl<'a, 'b> KeyEventHandler<'a, 'b, ActiveRadarrBlock> for EditCollectionHandler<'a, 'b> {
  fn with(
    key: &'a Key,
    app: &'a mut App<'b>,
    active_block: &'a ActiveRadarrBlock,
    context: &'a Option<ActiveRadarrBlock>,
  ) -> EditCollectionHandler<'a, 'b> {
    EditCollectionHandler {
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
      ActiveRadarrBlock::EditCollectionSelectMinimumAvailability => self
        .app
        .data
        .radarr_data
        .minimum_availability_list
        .scroll_up(),
      ActiveRadarrBlock::EditCollectionSelectQualityProfile => {
        self.app.data.radarr_data.quality_profile_list.scroll_up()
      }
      ActiveRadarrBlock::EditCollectionPrompt => {
        self.app.data.radarr_data.selected_block.previous()
      }
      _ => (),
    }
  }

  fn handle_scroll_down(&mut self) {
    match self.active_radarr_block {
      ActiveRadarrBlock::EditCollectionSelectMinimumAvailability => self
        .app
        .data
        .radarr_data
        .minimum_availability_list
        .scroll_down(),
      ActiveRadarrBlock::EditCollectionSelectQualityProfile => {
        self.app.data.radarr_data.quality_profile_list.scroll_down()
      }
      ActiveRadarrBlock::EditCollectionPrompt => self.app.data.radarr_data.selected_block.next(),
      _ => (),
    }
  }

  fn handle_home(&mut self) {
    match self.active_radarr_block {
      ActiveRadarrBlock::EditCollectionSelectMinimumAvailability => self
        .app
        .data
        .radarr_data
        .minimum_availability_list
        .scroll_to_top(),
      ActiveRadarrBlock::EditCollectionSelectQualityProfile => self
        .app
        .data
        .radarr_data
        .quality_profile_list
        .scroll_to_top(),
      ActiveRadarrBlock::EditCollectionRootFolderPathInput => {
        self.app.data.radarr_data.edit_path.scroll_home()
      }
      _ => (),
    }
  }

  fn handle_end(&mut self) {
    match self.active_radarr_block {
      ActiveRadarrBlock::EditCollectionSelectMinimumAvailability => self
        .app
        .data
        .radarr_data
        .minimum_availability_list
        .scroll_to_bottom(),
      ActiveRadarrBlock::EditCollectionSelectQualityProfile => self
        .app
        .data
        .radarr_data
        .quality_profile_list
        .scroll_to_bottom(),
      ActiveRadarrBlock::EditCollectionRootFolderPathInput => {
        self.app.data.radarr_data.edit_path.reset_offset()
      }
      _ => (),
    }
  }

  fn handle_delete(&mut self) {}

  fn handle_left_right_action(&mut self) {
    match self.active_radarr_block {
      ActiveRadarrBlock::EditCollectionPrompt => handle_prompt_toggle(self.app, self.key),
      ActiveRadarrBlock::EditCollectionRootFolderPathInput => {
        handle_text_box_left_right_keys!(self, self.key, self.app.data.radarr_data.edit_path)
      }
      _ => (),
    }
  }

  fn handle_submit(&mut self) {
    match self.active_radarr_block {
      ActiveRadarrBlock::EditCollectionPrompt => {
        match self.app.data.radarr_data.selected_block.get_active_block() {
          ActiveRadarrBlock::EditCollectionConfirmPrompt => {
            if self.app.data.radarr_data.prompt_confirm {
              self.app.data.radarr_data.prompt_confirm_action = Some(RadarrEvent::EditCollection);
              self.app.should_refresh = true;
            }

            self.app.pop_navigation_stack();
          }
          ActiveRadarrBlock::EditCollectionSelectMinimumAvailability
          | ActiveRadarrBlock::EditCollectionSelectQualityProfile => {
            self.app.push_navigation_stack(
              (
                *self.app.data.radarr_data.selected_block.get_active_block(),
                *self.context,
              )
                .into(),
            )
          }
          ActiveRadarrBlock::EditCollectionRootFolderPathInput => {
            self.app.push_navigation_stack(
              (
                *self.app.data.radarr_data.selected_block.get_active_block(),
                *self.context,
              )
                .into(),
            );
            self.app.should_ignore_quit_key = true;
          }
          ActiveRadarrBlock::EditCollectionToggleMonitored => {
            self.app.data.radarr_data.edit_monitored =
              Some(!self.app.data.radarr_data.edit_monitored.unwrap_or_default())
          }
          ActiveRadarrBlock::EditCollectionToggleSearchOnAdd => {
            self.app.data.radarr_data.edit_search_on_add = Some(
              !self
                .app
                .data
                .radarr_data
                .edit_search_on_add
                .unwrap_or_default(),
            )
          }
          _ => (),
        }
      }
      ActiveRadarrBlock::EditCollectionSelectMinimumAvailability
      | ActiveRadarrBlock::EditCollectionSelectQualityProfile => self.app.pop_navigation_stack(),
      ActiveRadarrBlock::EditCollectionRootFolderPathInput => {
        self.app.pop_navigation_stack();
        self.app.should_ignore_quit_key = false;
      }
      _ => (),
    }
  }

  fn handle_esc(&mut self) {
    match self.active_radarr_block {
      ActiveRadarrBlock::EditCollectionRootFolderPathInput => {
        self.app.pop_navigation_stack();
        self.app.should_ignore_quit_key = false;
      }
      ActiveRadarrBlock::EditCollectionPrompt => {
        self.app.pop_navigation_stack();
        self.app.data.radarr_data.reset_add_edit_media_fields();
        self.app.data.radarr_data.prompt_confirm = false;
      }
      ActiveRadarrBlock::EditCollectionSelectMinimumAvailability
      | ActiveRadarrBlock::EditCollectionSelectQualityProfile => self.app.pop_navigation_stack(),
      _ => (),
    }
  }

  fn handle_char_key_event(&mut self) {
    let key = self.key;
    if self.active_radarr_block == &ActiveRadarrBlock::EditCollectionRootFolderPathInput {
      handle_text_box_keys!(self, key, self.app.data.radarr_data.edit_path)
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
  use crate::handlers::radarr_handlers::edit_collection_handler::EditCollectionHandler;
  use crate::handlers::KeyEventHandler;
  use crate::models::radarr_models::MinimumAvailability;

  mod test_handle_scroll_up_and_down {
    use pretty_assertions::assert_eq;
    use rstest::rstest;
    use strum::IntoEnumIterator;

    use crate::app::radarr::EDIT_COLLECTION_SELECTION_BLOCKS;
    use crate::models::BlockSelectionState;
    use crate::{test_enum_scroll, test_iterable_scroll};

    use super::*;

    test_enum_scroll!(
      test_edit_collection_select_minimum_availability_scroll,
      EditCollectionHandler,
      MinimumAvailability,
      minimum_availability_list,
      ActiveRadarrBlock::EditCollectionSelectMinimumAvailability,
      None
    );

    test_iterable_scroll!(
      test_edit_collection_select_quality_profile_scroll,
      EditCollectionHandler,
      quality_profile_list,
      ActiveRadarrBlock::EditCollectionSelectQualityProfile,
      None
    );

    #[rstest]
    fn test_edit_collection_prompt_scroll(#[values(Key::Up, Key::Down)] key: Key) {
      let mut app = App::default();
      app.data.radarr_data.selected_block =
        BlockSelectionState::new(&EDIT_COLLECTION_SELECTION_BLOCKS);
      app.data.radarr_data.selected_block.next();

      EditCollectionHandler::with(
        &key,
        &mut app,
        &ActiveRadarrBlock::EditCollectionPrompt,
        &None,
      )
      .handle();

      if key == Key::Up {
        assert_eq!(
          app.data.radarr_data.selected_block.get_active_block(),
          &ActiveRadarrBlock::EditCollectionToggleMonitored
        );
      } else {
        assert_eq!(
          app.data.radarr_data.selected_block.get_active_block(),
          &ActiveRadarrBlock::EditCollectionSelectQualityProfile
        );
      }
    }
  }

  mod test_handle_home_end {
    use strum::IntoEnumIterator;

    use crate::{test_enum_home_and_end, test_iterable_home_and_end, test_text_box_home_end_keys};

    use super::*;

    test_enum_home_and_end!(
      test_edit_collection_select_minimum_availability_home_end,
      EditCollectionHandler,
      MinimumAvailability,
      minimum_availability_list,
      ActiveRadarrBlock::EditCollectionSelectMinimumAvailability,
      None
    );

    test_iterable_home_and_end!(
      test_edit_collection_select_quality_profile_scroll,
      EditCollectionHandler,
      quality_profile_list,
      ActiveRadarrBlock::EditCollectionSelectQualityProfile,
      None
    );

    #[test]
    fn test_edit_collection_root_folder_path_input_home_end_keys() {
      test_text_box_home_end_keys!(
        EditCollectionHandler,
        ActiveRadarrBlock::EditCollectionRootFolderPathInput,
        edit_path
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

      EditCollectionHandler::with(
        &key,
        &mut app,
        &ActiveRadarrBlock::EditCollectionPrompt,
        &None,
      )
      .handle();

      assert!(app.data.radarr_data.prompt_confirm);

      EditCollectionHandler::with(
        &key,
        &mut app,
        &ActiveRadarrBlock::EditCollectionPrompt,
        &None,
      )
      .handle();

      assert!(!app.data.radarr_data.prompt_confirm);
    }

    #[test]
    fn test_edit_collection_root_folder_path_input_left_right_keys() {
      test_text_box_left_right_keys!(
        EditCollectionHandler,
        ActiveRadarrBlock::EditCollectionRootFolderPathInput,
        edit_path
      );
    }
  }

  mod test_handle_submit {
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    use crate::app::radarr::EDIT_COLLECTION_SELECTION_BLOCKS;
    use crate::models::{BlockSelectionState, Route};
    use crate::network::radarr_network::RadarrEvent;

    use super::*;

    const SUBMIT_KEY: Key = DEFAULT_KEYBINDINGS.submit.key;

    #[test]
    fn test_edit_collection_root_folder_path_input_submit() {
      let mut app = App::default();
      app.should_ignore_quit_key = true;
      app.data.radarr_data.edit_path = "Test Path".to_owned().into();
      app.push_navigation_stack(ActiveRadarrBlock::EditCollectionPrompt.into());
      app.push_navigation_stack(ActiveRadarrBlock::EditCollectionRootFolderPathInput.into());

      EditCollectionHandler::with(
        &SUBMIT_KEY,
        &mut app,
        &ActiveRadarrBlock::EditCollectionRootFolderPathInput,
        &None,
      )
      .handle();

      assert!(!app.should_ignore_quit_key);
      assert!(!app.data.radarr_data.edit_path.text.is_empty());
      assert_eq!(
        app.get_current_route(),
        &ActiveRadarrBlock::EditCollectionPrompt.into()
      );
    }

    #[test]
    fn test_edit_collection_prompt_prompt_decline_submit() {
      let mut app = App::default();
      app.push_navigation_stack(ActiveRadarrBlock::Collections.into());
      app.push_navigation_stack(ActiveRadarrBlock::EditCollectionPrompt.into());
      app.data.radarr_data.selected_block =
        BlockSelectionState::new(&EDIT_COLLECTION_SELECTION_BLOCKS);
      app
        .data
        .radarr_data
        .selected_block
        .set_index(EDIT_COLLECTION_SELECTION_BLOCKS.len() - 1);

      EditCollectionHandler::with(
        &SUBMIT_KEY,
        &mut app,
        &ActiveRadarrBlock::EditCollectionPrompt,
        &None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        &ActiveRadarrBlock::Collections.into()
      );
      assert_eq!(app.data.radarr_data.prompt_confirm_action, None);
    }

    #[test]
    fn test_edit_collection_confirm_prompt_prompt_confirmation_submit() {
      let mut app = App::default();
      app.push_navigation_stack(ActiveRadarrBlock::Collections.into());
      app.push_navigation_stack(ActiveRadarrBlock::EditCollectionPrompt.into());
      app.data.radarr_data.prompt_confirm = true;
      app.data.radarr_data.selected_block =
        BlockSelectionState::new(&EDIT_COLLECTION_SELECTION_BLOCKS);
      app
        .data
        .radarr_data
        .selected_block
        .set_index(EDIT_COLLECTION_SELECTION_BLOCKS.len() - 1);

      EditCollectionHandler::with(
        &SUBMIT_KEY,
        &mut app,
        &ActiveRadarrBlock::EditCollectionPrompt,
        &None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        &ActiveRadarrBlock::Collections.into()
      );
      assert_eq!(
        app.data.radarr_data.prompt_confirm_action,
        Some(RadarrEvent::EditCollection)
      );
      assert!(app.should_refresh);
    }

    #[test]
    fn test_edit_collection_toggle_monitored_submit() {
      let current_route = Route::from((
        ActiveRadarrBlock::EditCollectionPrompt,
        Some(ActiveRadarrBlock::Collections),
      ));
      let mut app = App::default();
      app.data.radarr_data.selected_block =
        BlockSelectionState::new(&EDIT_COLLECTION_SELECTION_BLOCKS);
      app.push_navigation_stack(current_route);

      EditCollectionHandler::with(
        &SUBMIT_KEY,
        &mut app,
        &ActiveRadarrBlock::EditCollectionPrompt,
        &Some(ActiveRadarrBlock::Collections),
      )
      .handle();

      assert_eq!(app.get_current_route(), &current_route);
      assert_eq!(app.data.radarr_data.edit_monitored, Some(true));

      EditCollectionHandler::with(
        &SUBMIT_KEY,
        &mut app,
        &ActiveRadarrBlock::EditCollectionPrompt,
        &Some(ActiveRadarrBlock::Collections),
      )
      .handle();

      assert_eq!(app.get_current_route(), &current_route);
      assert_eq!(app.data.radarr_data.edit_monitored, Some(false));
    }

    #[test]
    fn test_edit_collection_toggle_search_on_add_submit() {
      let current_route = Route::from((
        ActiveRadarrBlock::EditCollectionPrompt,
        Some(ActiveRadarrBlock::Collections),
      ));
      let mut app = App::default();
      app.data.radarr_data.selected_block =
        BlockSelectionState::new(&EDIT_COLLECTION_SELECTION_BLOCKS);
      app
        .data
        .radarr_data
        .selected_block
        .set_index(EDIT_COLLECTION_SELECTION_BLOCKS.len() - 2);
      app.push_navigation_stack(current_route);

      EditCollectionHandler::with(
        &SUBMIT_KEY,
        &mut app,
        &ActiveRadarrBlock::EditCollectionPrompt,
        &Some(ActiveRadarrBlock::Collections),
      )
      .handle();

      assert_eq!(app.get_current_route(), &current_route);
      assert_eq!(app.data.radarr_data.edit_search_on_add, Some(true));

      EditCollectionHandler::with(
        &SUBMIT_KEY,
        &mut app,
        &ActiveRadarrBlock::EditCollectionPrompt,
        &Some(ActiveRadarrBlock::Collections),
      )
      .handle();

      assert_eq!(app.get_current_route(), &current_route);
      assert_eq!(app.data.radarr_data.edit_search_on_add, Some(false));
    }

    #[rstest]
    #[case(ActiveRadarrBlock::EditCollectionSelectMinimumAvailability, 1)]
    #[case(ActiveRadarrBlock::EditCollectionSelectQualityProfile, 2)]
    #[case(ActiveRadarrBlock::EditCollectionRootFolderPathInput, 3)]
    fn test_edit_collection_prompt_selected_block_submit(
      #[case] selected_block: ActiveRadarrBlock,
      #[case] index: usize,
    ) {
      let mut app = App::default();
      app.push_navigation_stack(
        (
          ActiveRadarrBlock::EditCollectionPrompt,
          Some(ActiveRadarrBlock::Collections),
        )
          .into(),
      );
      app.data.radarr_data.selected_block =
        BlockSelectionState::new(&EDIT_COLLECTION_SELECTION_BLOCKS);
      app.data.radarr_data.selected_block.set_index(index);

      EditCollectionHandler::with(
        &SUBMIT_KEY,
        &mut app,
        &ActiveRadarrBlock::EditCollectionPrompt,
        &Some(ActiveRadarrBlock::Collections),
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        &(selected_block, Some(ActiveRadarrBlock::Collections)).into()
      );
      assert_eq!(app.data.radarr_data.prompt_confirm_action, None);

      if selected_block == ActiveRadarrBlock::EditCollectionRootFolderPathInput {
        assert!(app.should_ignore_quit_key);
      }
    }

    #[rstest]
    fn test_edit_collection_prompt_selecting_preferences_blocks_submit(
      #[values(
        ActiveRadarrBlock::EditCollectionSelectMinimumAvailability,
        ActiveRadarrBlock::EditCollectionSelectQualityProfile,
        ActiveRadarrBlock::EditCollectionRootFolderPathInput
      )]
      active_radarr_block: ActiveRadarrBlock,
    ) {
      let mut app = App::default();
      app.push_navigation_stack(ActiveRadarrBlock::EditCollectionPrompt.into());
      app.push_navigation_stack(active_radarr_block.into());

      EditCollectionHandler::with(
        &SUBMIT_KEY,
        &mut app,
        &active_radarr_block,
        &Some(ActiveRadarrBlock::Collections),
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        &ActiveRadarrBlock::EditCollectionPrompt.into()
      );

      if active_radarr_block == ActiveRadarrBlock::EditCollectionRootFolderPathInput {
        assert!(!app.should_ignore_quit_key);
      }
    }
  }

  mod test_handle_esc {
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    use crate::app::radarr::radarr_test_utils::create_test_radarr_data;
    use crate::{assert_edit_media_reset, assert_preferences_selections_reset};

    use super::*;

    const ESC_KEY: Key = DEFAULT_KEYBINDINGS.esc.key;

    #[test]
    fn test_edit_collection_root_folder_path_input_esc() {
      let mut app = App::default();
      app.data.radarr_data = create_test_radarr_data();
      app.should_ignore_quit_key = true;
      app.push_navigation_stack(ActiveRadarrBlock::EditCollectionPrompt.into());
      app.push_navigation_stack(ActiveRadarrBlock::EditCollectionRootFolderPathInput.into());

      EditCollectionHandler::with(
        &ESC_KEY,
        &mut app,
        &ActiveRadarrBlock::EditCollectionRootFolderPathInput,
        &None,
      )
      .handle();

      assert!(!app.should_ignore_quit_key);
      assert_eq!(
        app.get_current_route(),
        &ActiveRadarrBlock::EditCollectionPrompt.into()
      );
    }

    #[test]
    fn test_edit_collection_prompt_esc() {
      let mut app = App::default();
      app.push_navigation_stack(ActiveRadarrBlock::Collections.into());
      app.push_navigation_stack(ActiveRadarrBlock::EditCollectionPrompt.into());
      app.data.radarr_data = create_test_radarr_data();

      EditCollectionHandler::with(
        &ESC_KEY,
        &mut app,
        &ActiveRadarrBlock::EditCollectionPrompt,
        &None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        &ActiveRadarrBlock::Collections.into()
      );
      let radarr_data = &app.data.radarr_data;

      assert_preferences_selections_reset!(radarr_data);
      assert_edit_media_reset!(radarr_data);
      assert!(!radarr_data.prompt_confirm);
    }

    #[rstest]
    fn test_edit_collection_esc(
      #[values(
        ActiveRadarrBlock::EditCollectionSelectMinimumAvailability,
        ActiveRadarrBlock::EditCollectionSelectQualityProfile
      )]
      active_radarr_block: ActiveRadarrBlock,
    ) {
      let mut app = App::default();
      app.data.radarr_data = create_test_radarr_data();
      app.push_navigation_stack(ActiveRadarrBlock::Collections.into());
      app.push_navigation_stack(active_radarr_block.into());

      EditCollectionHandler::with(&ESC_KEY, &mut app, &active_radarr_block, &None).handle();

      assert_eq!(
        app.get_current_route(),
        &ActiveRadarrBlock::Collections.into()
      );
    }
  }

  mod test_handle_key_char {
    use super::*;

    #[test]
    fn test_edit_collection_root_folder_path_input_backspace() {
      let mut app = App::default();
      app.data.radarr_data.edit_path = "Test".to_owned().into();

      EditCollectionHandler::with(
        &DEFAULT_KEYBINDINGS.backspace.key,
        &mut app,
        &ActiveRadarrBlock::EditCollectionRootFolderPathInput,
        &None,
      )
      .handle();

      assert_str_eq!(app.data.radarr_data.edit_path.text, "Tes");
    }

    #[test]
    fn test_edit_collection_root_folder_path_input_char_key() {
      let mut app = App::default();

      EditCollectionHandler::with(
        &Key::Char('h'),
        &mut app,
        &ActiveRadarrBlock::EditCollectionRootFolderPathInput,
        &None,
      )
      .handle();

      assert_str_eq!(app.data.radarr_data.edit_path.text, "h");
    }
  }
}
