use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::text::Text;
use ratatui::widgets::{ListItem, Paragraph};
use ratatui::Frame;
use std::sync::atomic::Ordering;

use crate::app::context_clues::{build_context_clue_string, CONFIRMATION_PROMPT_CONTEXT_CLUES};
use crate::app::App;
use crate::models::servarr_data::radarr::modals::EditCollectionModal;
use crate::models::servarr_data::radarr::radarr_data::{
  ActiveRadarrBlock, COLLECTION_DETAILS_BLOCKS, EDIT_COLLECTION_BLOCKS,
};
use crate::models::Route;
use crate::render_selectable_input_box;
use crate::ui::radarr_ui::collections::collection_details_ui::CollectionDetailsUi;
use crate::ui::styles::ManagarrStyle;
use crate::ui::utils::{layout_paragraph_borderless, title_block_centered};
use crate::ui::widgets::button::Button;
use crate::ui::widgets::checkbox::Checkbox;
use crate::ui::widgets::input_box::InputBox;
use crate::ui::widgets::popup::{Popup, Size};
use crate::ui::widgets::selectable_list::SelectableList;
use crate::ui::{draw_popup, DrawUi};

#[cfg(test)]
#[path = "edit_collection_ui_tests.rs"]
mod edit_collection_ui_tests;

pub(super) struct EditCollectionUi;

impl DrawUi for EditCollectionUi {
  fn accepts(route: Route) -> bool {
    if let Route::Radarr(active_radarr_block, context_option) = route {
      if let Some(context) = context_option {
        return EDIT_COLLECTION_BLOCKS.contains(&active_radarr_block)
          && context == ActiveRadarrBlock::CollectionDetails;
      }

      return EDIT_COLLECTION_BLOCKS.contains(&active_radarr_block);
    }

    false
  }

  fn draw(f: &mut Frame<'_>, app: &mut App<'_>, _area: Rect) {
    if let Route::Radarr(active_radarr_block, context_option) = app.get_current_route() {
      if let Some(context) = context_option {
        if COLLECTION_DETAILS_BLOCKS.contains(&context) {
          draw_popup(f, app, CollectionDetailsUi::draw, Size::Large);
        }
      }

      draw_popup(
        f,
        app,
        draw_edit_collection_confirmation_prompt,
        Size::Medium,
      );

      match active_radarr_block {
        ActiveRadarrBlock::EditCollectionSelectMinimumAvailability => {
          draw_edit_collection_select_minimum_availability_popup(f, app);
        }
        ActiveRadarrBlock::EditCollectionSelectQualityProfile => {
          draw_edit_collection_select_quality_profile_popup(f, app);
        }
        _ => (),
      };
    }
  }
}

fn draw_edit_collection_confirmation_prompt(f: &mut Frame<'_>, app: &mut App<'_>, area: Rect) {
  let collection_title = app
    .data
    .radarr_data
    .collections
    .current_selection()
    .title
    .text
    .clone();
  let collection_overview = app
    .data
    .radarr_data
    .collections
    .current_selection()
    .overview
    .clone()
    .unwrap_or_default();
  let title = format!("Edit - {collection_title}");
  f.render_widget(title_block_centered(&title), area);
  let yes_no_value = app.data.radarr_data.prompt_confirm;
  let selected_block = app.data.radarr_data.selected_block.get_active_block();
  let highlight_yes_no = selected_block == ActiveRadarrBlock::EditCollectionConfirmPrompt;
  let EditCollectionModal {
    minimum_availability_list,
    quality_profile_list,
    monitored,
    search_on_add,
    path,
  } = app.data.radarr_data.edit_collection_modal.as_ref().unwrap();
  let selected_minimum_availability = minimum_availability_list.current_selection();
  let selected_quality_profile = quality_profile_list.current_selection();

  let [paragraph_area, monitored_area, min_availability_area, quality_profile_area, root_folder_area, search_on_add_area, _, buttons_area, help_area] =
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
  let prompt_paragraph = layout_paragraph_borderless(&collection_overview);
  let monitored_checkbox = Checkbox::new("Monitored")
    .highlighted(selected_block == ActiveRadarrBlock::EditCollectionToggleMonitored)
    .checked(monitored.unwrap_or_default());
  let min_availability_drop_down_button = Button::new()
    .title(selected_minimum_availability.to_display_str())
    .label("Minimum Availability")
    .icon("▼")
    .selected(selected_block == ActiveRadarrBlock::EditCollectionSelectMinimumAvailability);
  let quality_profile_drop_down_button = Button::new()
    .title(selected_quality_profile)
    .label("Quality Profile")
    .icon("▼")
    .selected(selected_block == ActiveRadarrBlock::EditCollectionSelectQualityProfile);

  if let Route::Radarr(active_radarr_block, _) = app.get_current_route() {
    let root_folder_input_box = InputBox::new(&path.text)
      .offset(path.offset.load(Ordering::SeqCst))
      .label("Root Folder")
      .highlighted(selected_block == ActiveRadarrBlock::EditCollectionRootFolderPathInput)
      .selected(active_radarr_block == ActiveRadarrBlock::EditCollectionRootFolderPathInput);
    render_selectable_input_box!(root_folder_input_box, f, root_folder_area);
  }

  let search_on_add_checkbox = Checkbox::new("Search on Add")
    .highlighted(selected_block == ActiveRadarrBlock::EditCollectionToggleSearchOnAdd)
    .checked(search_on_add.unwrap_or_default());
  let save_button = Button::new()
    .title("Save")
    .selected(yes_no_value && highlight_yes_no);
  let cancel_button = Button::new()
    .title("Cancel")
    .selected(!yes_no_value && highlight_yes_no);

  f.render_widget(prompt_paragraph, paragraph_area);
  f.render_widget(monitored_checkbox, monitored_area);
  f.render_widget(min_availability_drop_down_button, min_availability_area);
  f.render_widget(quality_profile_drop_down_button, quality_profile_area);
  f.render_widget(search_on_add_checkbox, search_on_add_area);
  f.render_widget(save_button, save_area);
  f.render_widget(cancel_button, cancel_area);
  f.render_widget(help_paragraph, help_area);
}

fn draw_edit_collection_select_minimum_availability_popup(f: &mut Frame<'_>, app: &mut App<'_>) {
  let min_availability_list = SelectableList::new(
    &mut app
      .data
      .radarr_data
      .edit_collection_modal
      .as_mut()
      .unwrap()
      .minimum_availability_list,
    |minimum_availability| ListItem::new(minimum_availability.to_display_str().to_owned()),
  );
  let popup = Popup::new(min_availability_list).size(Size::Dropdown);

  f.render_widget(popup, f.area());
}

fn draw_edit_collection_select_quality_profile_popup(f: &mut Frame<'_>, app: &mut App<'_>) {
  let quality_profile_list = SelectableList::new(
    &mut app
      .data
      .radarr_data
      .edit_collection_modal
      .as_mut()
      .unwrap()
      .quality_profile_list,
    |quality_profile| ListItem::new(quality_profile.clone()),
  );
  let popup = Popup::new(quality_profile_list).size(Size::Dropdown);

  f.render_widget(popup, f.area());
}
