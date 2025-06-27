use std::sync::atomic::Ordering;

use ratatui::layout::{Constraint, Rect};
use ratatui::prelude::Layout;
use ratatui::text::Text;
use ratatui::widgets::{ListItem, Paragraph};
use ratatui::Frame;

use crate::app::context_clues::{build_context_clue_string, CONFIRMATION_PROMPT_CONTEXT_CLUES};
use crate::app::App;
use crate::models::servarr_data::lidarr::lidarr_data::{
  ActiveLidarrBlock, EDIT_ARTIST_BLOCKS, ARTIST_DETAILS_BLOCKS,
};
use crate::models::Route;
use crate::models::servarr_data::lidarr::modals::edit_artist_modal::EditArtistModal;
use crate::render_selectable_input_box;

use crate::ui::styles::ManagarrStyle;
use crate::ui::utils::{layout_paragraph_borderless, title_block_centered};
use crate::ui::widgets::button::Button;
use crate::ui::widgets::checkbox::Checkbox;
use crate::ui::widgets::input_box::InputBox;
use crate::ui::widgets::popup::{Popup, Size};
use crate::ui::widgets::selectable_list::SelectableList;
use crate::ui::{draw_popup, DrawUi};

use super::artist_details_ui::ArtistDetailsUi;

#[cfg(test)]
#[path = "edit_artist_ui_tests.rs"]
mod edit_artist_ui_tests;

pub(super) struct EditArtistUi;

impl DrawUi for EditArtistUi {
  fn accepts(route: Route) -> bool {
    if let Route::Lidarr(active_lidarr_block, _) = route {
      return EDIT_ARTIST_BLOCKS.contains(&active_lidarr_block);
    }

    false
  }

  fn draw(f: &mut Frame<'_>, app: &mut App<'_>, _area: Rect) {
    if let Route::Lidarr(active_lidarr_block, context_option) = app.get_current_route() {
      if let Some(context) = context_option {
        if ARTIST_DETAILS_BLOCKS.contains(&context) {
          draw_popup(f, app, ArtistDetailsUi::draw, Size::Large);
        }
      }

      let draw_edit_artist_prompt = |f: &mut Frame<'_>, app: &mut App<'_>, prompt_area: Rect| {
        draw_edit_artist_confirmation_prompt(f, app, prompt_area);

        match active_lidarr_block {
          ActiveLidarrBlock::EditArtistSelectQualityProfile => {
            draw_edit_artist_select_quality_profile_popup(f, app);
          }
          ActiveLidarrBlock::EditArtistSelectMetadataProfile => {
            draw_edit_artist_select_metadata_profile_popup(f, app);
          }
          _ => (),
        }
      };

      draw_popup(f, app, draw_edit_artist_prompt, Size::Long);
    }
  }
}

fn draw_edit_artist_confirmation_prompt(f: &mut Frame<'_>, app: &mut App<'_>, area: Rect) {
  let artist_name = app
    .data
    .lidarr_data
    .artists
    .current_selection()
    .artist_name
    .text
    .clone();
  let artist_overview = app
    .data
    .lidarr_data
    .artists
    .current_selection()
    .overview
    .clone()
    .unwrap_or_default();
  let title = format!("Edit - {artist_name}");
  f.render_widget(title_block_centered(&title), area);

  let yes_no_value = app.data.lidarr_data.prompt_confirm;
  let selected_block = app.data.lidarr_data.selected_block.get_active_block();
  let highlight_yes_no = selected_block == ActiveLidarrBlock::EditArtistConfirmPrompt;
  let EditArtistModal {
    quality_profile_list,
    metadata_profile_list,
    monitored,
    path,
    tags,
  } = app.data.lidarr_data.edit_artist_modal.as_ref().unwrap();
  let selected_quality_profile = quality_profile_list.current_selection();
  let selected_metadata_profile = metadata_profile_list.current_selection();

  let [paragraph_area, monitored_area, quality_profile_area, metadata_profile_area, path_area, tags_area, _, buttons_area, help_area] =
    Layout::vertical([
      Constraint::Length(6),
      Constraint::Length(3),
      Constraint::Length(3),
      Constraint::Length(3),
      Constraint::Length(3),
      Constraint::Length(3),
      Constraint::Fill(1),
      Constraint::Length(3),
      Constraint::Length(1),
    ])
    .margin(1)
    .areas(area);
  let [save_area, cancel_area] =
    Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
      .areas(buttons_area);

  let help_text = Text::from(build_context_clue_string(&CONFIRMATION_PROMPT_CONTEXT_CLUES).help());
  let help_paragraph = Paragraph::new(help_text).centered();
  let prompt_paragraph = layout_paragraph_borderless(&artist_overview);
  let monitored_checkbox = Checkbox::new("Monitored")
    .checked(monitored.unwrap_or_default())
    .highlighted(selected_block == ActiveLidarrBlock::EditArtistToggleMonitored);
  let quality_profile_drop_down_button = Button::new()
    .title(selected_quality_profile)
    .label("Quality Profile")
    .icon("▼")
    .selected(selected_block == ActiveLidarrBlock::EditArtistSelectQualityProfile);
  let metadata_profile_drop_down_button = Button::new()
    .title(selected_metadata_profile)
    .label("Metadata Profile")
    .icon("▼")
    .selected(selected_block == ActiveLidarrBlock::EditArtistSelectMetadataProfile);

  if let Route::Lidarr(active_lidarr_block, _) = app.get_current_route() {
    let path_input_box = InputBox::new(&path.text)
      .offset(path.offset.load(Ordering::SeqCst))
      .label("Path")
      .highlighted(selected_block == ActiveLidarrBlock::EditArtistPathInput)
      .selected(active_lidarr_block == ActiveLidarrBlock::EditArtistPathInput);
    let tags_input_box = InputBox::new(&tags.text)
      .offset(tags.offset.load(Ordering::SeqCst))
      .label("Tags")
      .highlighted(selected_block == ActiveLidarrBlock::EditArtistTagsInput)
      .selected(active_lidarr_block == ActiveLidarrBlock::EditArtistTagsInput);

    match active_lidarr_block {
      ActiveLidarrBlock::EditArtistPathInput => path_input_box.show_cursor(f, path_area),
      ActiveLidarrBlock::EditArtistTagsInput => tags_input_box.show_cursor(f, tags_area),
      _ => (),
    }

    render_selectable_input_box!(path_input_box, f, path_area);
    render_selectable_input_box!(tags_input_box, f, tags_area);
  }

  let save_button = Button::new()
    .title("Save")
    .selected(yes_no_value && highlight_yes_no);
  let cancel_button = Button::new()
    .title("Cancel")
    .selected(!yes_no_value && highlight_yes_no);

  f.render_widget(prompt_paragraph, paragraph_area);
  f.render_widget(monitored_checkbox, monitored_area);
  f.render_widget(quality_profile_drop_down_button, quality_profile_area);
  f.render_widget(metadata_profile_drop_down_button, metadata_profile_area);
  f.render_widget(save_button, save_area);
  f.render_widget(cancel_button, cancel_area);
  f.render_widget(help_paragraph, help_area);
}

fn draw_edit_artist_select_quality_profile_popup(f: &mut Frame<'_>, app: &mut App<'_>) {
  let quality_profile_list = SelectableList::new(
    &mut app
      .data
      .lidarr_data
      .edit_artist_modal
      .as_mut()
      .unwrap()
      .quality_profile_list,
    |quality_profile| ListItem::new(quality_profile.clone()),
  );
  let popup = Popup::new(quality_profile_list).size(Size::Dropdown);

  f.render_widget(popup, f.area());
}

fn draw_edit_artist_select_metadata_profile_popup(f: &mut Frame<'_>, app: &mut App<'_>) {
  let metadata_profile_list = SelectableList::new(
    &mut app
      .data
      .lidarr_data
      .edit_artist_modal
      .as_mut()
      .unwrap()
      .metadata_profile_list,
    |metadata_profile| ListItem::new(metadata_profile.clone()),
  );
  let popup = Popup::new(metadata_profile_list).size(Size::Dropdown);

  f.render_widget(popup, f.area());
}
