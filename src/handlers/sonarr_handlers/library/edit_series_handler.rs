use crate::app::App;
use crate::event::Key;
use crate::handlers::{handle_prompt_toggle, KeyEventHandler};
use crate::models::servarr_data::sonarr::modals::EditSeriesModal;
use crate::models::servarr_data::sonarr::sonarr_data::{ActiveSonarrBlock, EDIT_SERIES_BLOCKS};
use crate::models::sonarr_models::EditSeriesParams;
use crate::models::Scrollable;
use crate::network::sonarr_network::SonarrEvent;
use crate::{handle_text_box_keys, handle_text_box_left_right_keys, matches_key};

#[cfg(test)]
#[path = "edit_series_handler_tests.rs"]
mod edit_series_handler_tests;

pub(super) struct EditSeriesHandler<'a, 'b> {
  key: Key,
  app: &'a mut App<'b>,
  active_sonarr_block: ActiveSonarrBlock,
  context: Option<ActiveSonarrBlock>,
}

impl EditSeriesHandler<'_, '_> {
  fn build_edit_series_params(&mut self) -> EditSeriesParams {
    let edit_series_modal = self
      .app
      .data
      .sonarr_data
      .edit_series_modal
      .take()
      .expect("EditSeriesModal is None");
    let series_id = self.app.data.sonarr_data.series.current_selection().id;
    let tags = edit_series_modal.tags.text;

    let EditSeriesModal {
      monitored,
      use_season_folders,
      path,
      series_type_list,
      quality_profile_list,
      language_profile_list,
      ..
    } = edit_series_modal;
    let quality_profile = quality_profile_list.current_selection();
    let quality_profile_id = *self
      .app
      .data
      .sonarr_data
      .quality_profile_map
      .iter()
      .filter(|(_, value)| *value == quality_profile)
      .map(|(key, _)| key)
      .next()
      .unwrap();
    let language_profile = language_profile_list.current_selection();
    let language_profile_id = *self
      .app
      .data
      .sonarr_data
      .language_profiles_map
      .iter()
      .filter(|(_, value)| *value == language_profile)
      .map(|(key, _)| key)
      .next()
      .unwrap();

    EditSeriesParams {
      series_id,
      monitored,
      use_season_folders,
      series_type: Some(*series_type_list.current_selection()),
      quality_profile_id: Some(quality_profile_id),
      language_profile_id: Some(language_profile_id),
      root_folder_path: Some(path.text),
      tag_input_string: Some(tags),
      ..EditSeriesParams::default()
    }
  }
}

impl<'a, 'b> KeyEventHandler<'a, 'b, ActiveSonarrBlock> for EditSeriesHandler<'a, 'b> {
  fn accepts(active_block: ActiveSonarrBlock) -> bool {
    EDIT_SERIES_BLOCKS.contains(&active_block)
  }

  fn ignore_alt_navigation(&self) -> bool {
    self.app.should_ignore_quit_key
  }

  fn new(
    key: Key,
    app: &'a mut App<'b>,
    active_block: ActiveSonarrBlock,
    context: Option<ActiveSonarrBlock>,
  ) -> EditSeriesHandler<'a, 'b> {
    EditSeriesHandler {
      key,
      app,
      active_sonarr_block: active_block,
      context,
    }
  }

  fn get_key(&self) -> Key {
    self.key
  }

  fn is_ready(&self) -> bool {
    !self.app.is_loading && self.app.data.sonarr_data.edit_series_modal.is_some()
  }

  fn handle_scroll_up(&mut self) {
    match self.active_sonarr_block {
      ActiveSonarrBlock::EditSeriesSelectSeriesType => self
        .app
        .data
        .sonarr_data
        .edit_series_modal
        .as_mut()
        .unwrap()
        .series_type_list
        .scroll_up(),
      ActiveSonarrBlock::EditSeriesSelectQualityProfile => self
        .app
        .data
        .sonarr_data
        .edit_series_modal
        .as_mut()
        .unwrap()
        .quality_profile_list
        .scroll_up(),
      ActiveSonarrBlock::EditSeriesSelectLanguageProfile => self
        .app
        .data
        .sonarr_data
        .edit_series_modal
        .as_mut()
        .unwrap()
        .language_profile_list
        .scroll_up(),
      ActiveSonarrBlock::EditSeriesPrompt => self.app.data.sonarr_data.selected_block.up(),
      _ => (),
    }
  }

  fn handle_scroll_down(&mut self) {
    match self.active_sonarr_block {
      ActiveSonarrBlock::EditSeriesSelectSeriesType => self
        .app
        .data
        .sonarr_data
        .edit_series_modal
        .as_mut()
        .unwrap()
        .series_type_list
        .scroll_down(),
      ActiveSonarrBlock::EditSeriesSelectQualityProfile => self
        .app
        .data
        .sonarr_data
        .edit_series_modal
        .as_mut()
        .unwrap()
        .quality_profile_list
        .scroll_down(),
      ActiveSonarrBlock::EditSeriesSelectLanguageProfile => self
        .app
        .data
        .sonarr_data
        .edit_series_modal
        .as_mut()
        .unwrap()
        .language_profile_list
        .scroll_down(),
      ActiveSonarrBlock::EditSeriesPrompt => self.app.data.sonarr_data.selected_block.down(),
      _ => (),
    }
  }

  fn handle_home(&mut self) {
    match self.active_sonarr_block {
      ActiveSonarrBlock::EditSeriesSelectSeriesType => self
        .app
        .data
        .sonarr_data
        .edit_series_modal
        .as_mut()
        .unwrap()
        .series_type_list
        .scroll_to_top(),
      ActiveSonarrBlock::EditSeriesSelectQualityProfile => self
        .app
        .data
        .sonarr_data
        .edit_series_modal
        .as_mut()
        .unwrap()
        .quality_profile_list
        .scroll_to_top(),
      ActiveSonarrBlock::EditSeriesSelectLanguageProfile => self
        .app
        .data
        .sonarr_data
        .edit_series_modal
        .as_mut()
        .unwrap()
        .language_profile_list
        .scroll_to_top(),
      ActiveSonarrBlock::EditSeriesPathInput => self
        .app
        .data
        .sonarr_data
        .edit_series_modal
        .as_mut()
        .unwrap()
        .path
        .scroll_home(),
      ActiveSonarrBlock::EditSeriesTagsInput => self
        .app
        .data
        .sonarr_data
        .edit_series_modal
        .as_mut()
        .unwrap()
        .tags
        .scroll_home(),
      _ => (),
    }
  }

  fn handle_end(&mut self) {
    match self.active_sonarr_block {
      ActiveSonarrBlock::EditSeriesSelectSeriesType => self
        .app
        .data
        .sonarr_data
        .edit_series_modal
        .as_mut()
        .unwrap()
        .series_type_list
        .scroll_to_bottom(),
      ActiveSonarrBlock::EditSeriesSelectQualityProfile => self
        .app
        .data
        .sonarr_data
        .edit_series_modal
        .as_mut()
        .unwrap()
        .quality_profile_list
        .scroll_to_bottom(),
      ActiveSonarrBlock::EditSeriesSelectLanguageProfile => self
        .app
        .data
        .sonarr_data
        .edit_series_modal
        .as_mut()
        .unwrap()
        .language_profile_list
        .scroll_to_bottom(),
      ActiveSonarrBlock::EditSeriesPathInput => self
        .app
        .data
        .sonarr_data
        .edit_series_modal
        .as_mut()
        .unwrap()
        .path
        .reset_offset(),
      ActiveSonarrBlock::EditSeriesTagsInput => self
        .app
        .data
        .sonarr_data
        .edit_series_modal
        .as_mut()
        .unwrap()
        .tags
        .reset_offset(),
      _ => (),
    }
  }

  fn handle_delete(&mut self) {}

  fn handle_left_right_action(&mut self) {
    match self.active_sonarr_block {
      ActiveSonarrBlock::EditSeriesPrompt => handle_prompt_toggle(self.app, self.key),
      ActiveSonarrBlock::EditSeriesPathInput => {
        handle_text_box_left_right_keys!(
          self,
          self.key,
          self
            .app
            .data
            .sonarr_data
            .edit_series_modal
            .as_mut()
            .unwrap()
            .path
        )
      }
      ActiveSonarrBlock::EditSeriesTagsInput => {
        handle_text_box_left_right_keys!(
          self,
          self.key,
          self
            .app
            .data
            .sonarr_data
            .edit_series_modal
            .as_mut()
            .unwrap()
            .tags
        )
      }
      _ => (),
    }
  }

  fn handle_submit(&mut self) {
    match self.active_sonarr_block {
      ActiveSonarrBlock::EditSeriesPrompt => {
        match self.app.data.sonarr_data.selected_block.get_active_block() {
          ActiveSonarrBlock::EditSeriesConfirmPrompt => {
            if self.app.data.sonarr_data.prompt_confirm {
              self.app.data.sonarr_data.prompt_confirm_action =
                Some(SonarrEvent::EditSeries(self.build_edit_series_params()));
              self.app.should_refresh = true;
            }

            self.app.pop_navigation_stack();
          }
          ActiveSonarrBlock::EditSeriesSelectSeriesType
          | ActiveSonarrBlock::EditSeriesSelectQualityProfile
          | ActiveSonarrBlock::EditSeriesSelectLanguageProfile => self.app.push_navigation_stack(
            (
              self.app.data.sonarr_data.selected_block.get_active_block(),
              self.context,
            )
              .into(),
          ),
          ActiveSonarrBlock::EditSeriesPathInput | ActiveSonarrBlock::EditSeriesTagsInput => {
            self.app.push_navigation_stack(
              (
                self.app.data.sonarr_data.selected_block.get_active_block(),
                self.context,
              )
                .into(),
            );
            self.app.should_ignore_quit_key = true;
          }
          ActiveSonarrBlock::EditSeriesToggleMonitored => {
            self
              .app
              .data
              .sonarr_data
              .edit_series_modal
              .as_mut()
              .unwrap()
              .monitored = Some(
              !self
                .app
                .data
                .sonarr_data
                .edit_series_modal
                .as_mut()
                .unwrap()
                .monitored
                .unwrap_or_default(),
            )
          }
          ActiveSonarrBlock::EditSeriesToggleSeasonFolder => {
            self
              .app
              .data
              .sonarr_data
              .edit_series_modal
              .as_mut()
              .unwrap()
              .use_season_folders = Some(
              !self
                .app
                .data
                .sonarr_data
                .edit_series_modal
                .as_mut()
                .unwrap()
                .use_season_folders
                .unwrap_or_default(),
            )
          }
          _ => (),
        }
      }
      ActiveSonarrBlock::EditSeriesSelectSeriesType
      | ActiveSonarrBlock::EditSeriesSelectQualityProfile
      | ActiveSonarrBlock::EditSeriesSelectLanguageProfile => self.app.pop_navigation_stack(),
      ActiveSonarrBlock::EditSeriesPathInput | ActiveSonarrBlock::EditSeriesTagsInput => {
        self.app.pop_navigation_stack();
        self.app.should_ignore_quit_key = false;
      }
      _ => (),
    }
  }

  fn handle_esc(&mut self) {
    match self.active_sonarr_block {
      ActiveSonarrBlock::EditSeriesTagsInput | ActiveSonarrBlock::EditSeriesPathInput => {
        self.app.pop_navigation_stack();
        self.app.should_ignore_quit_key = false;
      }
      ActiveSonarrBlock::EditSeriesPrompt => {
        self.app.pop_navigation_stack();
        self.app.data.sonarr_data.edit_series_modal = None;
        self.app.data.sonarr_data.prompt_confirm = false;
      }
      ActiveSonarrBlock::EditSeriesSelectSeriesType
      | ActiveSonarrBlock::EditSeriesSelectQualityProfile
      | ActiveSonarrBlock::EditSeriesSelectLanguageProfile => self.app.pop_navigation_stack(),
      _ => (),
    }
  }

  fn handle_char_key_event(&mut self) {
    let key = self.key;
    match self.active_sonarr_block {
      ActiveSonarrBlock::EditSeriesPathInput => {
        handle_text_box_keys!(
          self,
          key,
          self
            .app
            .data
            .sonarr_data
            .edit_series_modal
            .as_mut()
            .unwrap()
            .path
        )
      }
      ActiveSonarrBlock::EditSeriesTagsInput => {
        handle_text_box_keys!(
          self,
          key,
          self
            .app
            .data
            .sonarr_data
            .edit_series_modal
            .as_mut()
            .unwrap()
            .tags
        )
      }
      ActiveSonarrBlock::EditSeriesPrompt => {
        if self.app.data.sonarr_data.selected_block.get_active_block()
          == ActiveSonarrBlock::EditSeriesConfirmPrompt
          && matches_key!(confirm, key)
        {
          self.app.data.sonarr_data.prompt_confirm = true;
          self.app.data.sonarr_data.prompt_confirm_action =
            Some(SonarrEvent::EditSeries(self.build_edit_series_params()));
          self.app.should_refresh = true;

          self.app.pop_navigation_stack();
        }
      }
      _ => (),
    }
  }
}
