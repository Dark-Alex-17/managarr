use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::widgets::ListItem;
use ratatui::Frame;

use crate::app::App;
use crate::models::servarr_data::radarr::modals::EditCollectionModal;
use crate::models::servarr_data::radarr::radarr_data::{
  ActiveRadarrBlock, COLLECTION_DETAILS_BLOCKS, EDIT_COLLECTION_BLOCKS,
};
use crate::models::Route;
use crate::ui::radarr_ui::collections::collection_details_ui::CollectionDetailsUi;
use crate::ui::radarr_ui::collections::draw_collections;
use crate::ui::utils::{layout_paragraph_borderless, title_block_centered};
use crate::ui::{
  draw_button, draw_checkbox_with_label, draw_drop_down_menu_button, draw_drop_down_popup,
  draw_large_popup_over_background_fn_with_ui, draw_medium_popup_over, draw_popup,
  draw_selectable_list, draw_text_box_with_label, DrawUi, LabeledTextBoxProps,
};

#[cfg(test)]
#[path = "edit_collection_ui_tests.rs"]
mod edit_collection_ui_tests;

pub(super) struct EditCollectionUi;

impl DrawUi for EditCollectionUi {
  fn accepts(route: Route) -> bool {
    if let Route::Radarr(active_radarr_block, _) = route {
      return EDIT_COLLECTION_BLOCKS.contains(&active_radarr_block);
    }

    false
  }

  fn draw(f: &mut Frame<'_>, app: &mut App<'_>, area: Rect) {
    if let Route::Radarr(active_radarr_block, context_option) = *app.get_current_route() {
      let draw_edit_collection_prompt =
        |f: &mut Frame<'_>, app: &mut App<'_>, prompt_area: Rect| match active_radarr_block {
          ActiveRadarrBlock::EditCollectionSelectMinimumAvailability => {
            draw_drop_down_popup(
              f,
              app,
              prompt_area,
              draw_edit_collection_confirmation_prompt,
              draw_edit_collection_select_minimum_availability_popup,
            );
          }
          ActiveRadarrBlock::EditCollectionSelectQualityProfile => {
            draw_drop_down_popup(
              f,
              app,
              prompt_area,
              draw_edit_collection_confirmation_prompt,
              draw_edit_collection_select_quality_profile_popup,
            );
          }
          ActiveRadarrBlock::EditCollectionPrompt
          | ActiveRadarrBlock::EditCollectionToggleMonitored
          | ActiveRadarrBlock::EditCollectionRootFolderPathInput
          | ActiveRadarrBlock::EditCollectionToggleSearchOnAdd => {
            draw_edit_collection_confirmation_prompt(f, app, prompt_area)
          }
          _ => (),
        };

      if let Some(context) = context_option {
        match context {
          ActiveRadarrBlock::Collections => {
            draw_medium_popup_over(f, app, area, draw_collections, draw_edit_collection_prompt)
          }
          _ if COLLECTION_DETAILS_BLOCKS.contains(&context) => {
            draw_large_popup_over_background_fn_with_ui::<CollectionDetailsUi>(
              f,
              app,
              area,
              draw_collections,
            );
            draw_popup(f, app, draw_edit_collection_prompt, 60, 60);
          }
          _ => (),
        }
      }
    }
  }
}

fn draw_edit_collection_confirmation_prompt(f: &mut Frame<'_>, app: &mut App<'_>, area: Rect) {
  let (collection_title, collection_overview) =
    if let Some(filtered_collections) = app.data.radarr_data.filtered_collections.as_ref() {
      (
        filtered_collections.current_selection().title.text.clone(),
        filtered_collections
          .current_selection()
          .overview
          .clone()
          .unwrap_or_default(),
      )
    } else {
      (
        app
          .data
          .radarr_data
          .collections
          .current_selection()
          .title
          .text
          .clone(),
        app
          .data
          .radarr_data
          .collections
          .current_selection()
          .overview
          .clone()
          .unwrap_or_default(),
      )
    };
  let title = format!("Edit - {collection_title}");
  let yes_no_value = app.data.radarr_data.prompt_confirm;
  let selected_block = app.data.radarr_data.selected_block.get_active_block();
  let highlight_yes_no = selected_block == &ActiveRadarrBlock::EditCollectionConfirmPrompt;
  let EditCollectionModal {
    minimum_availability_list,
    quality_profile_list,
    monitored,
    search_on_add,
    path,
  } = app.data.radarr_data.edit_collection_modal.as_ref().unwrap();

  let selected_minimum_availability = minimum_availability_list.current_selection();
  let selected_quality_profile = quality_profile_list.current_selection();

  f.render_widget(title_block_centered(&title), area);

  let [paragraph_area, monitored_area, min_availability_area, quality_profile_area, root_folder_area, search_on_add_area, _, buttons_area] =
    Layout::vertical([
      Constraint::Length(6),
      Constraint::Length(3),
      Constraint::Length(3),
      Constraint::Length(3),
      Constraint::Length(3),
      Constraint::Length(3),
      Constraint::Fill(0),
      Constraint::Length(3),
    ])
    .margin(1)
    .areas(area);

  let prompt_paragraph = layout_paragraph_borderless(&collection_overview);
  f.render_widget(prompt_paragraph, paragraph_area);

  let [save_area, cancel_area] =
    Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
      .areas(buttons_area);

  draw_checkbox_with_label(
    f,
    monitored_area,
    "Monitored",
    monitored.unwrap_or_default(),
    selected_block == &ActiveRadarrBlock::EditCollectionToggleMonitored,
  );

  draw_drop_down_menu_button(
    f,
    min_availability_area,
    "Minimum Availability",
    selected_minimum_availability.to_display_str(),
    selected_block == &ActiveRadarrBlock::EditCollectionSelectMinimumAvailability,
  );
  draw_drop_down_menu_button(
    f,
    quality_profile_area,
    "Quality Profile",
    selected_quality_profile,
    selected_block == &ActiveRadarrBlock::EditCollectionSelectQualityProfile,
  );

  if let Route::Radarr(active_radarr_block, _) = *app.get_current_route() {
    draw_text_box_with_label(
      f,
      LabeledTextBoxProps {
        area: root_folder_area,
        label: "Root Folder",
        text: &path.text,
        offset: *path.offset.borrow(),
        is_selected: selected_block == &ActiveRadarrBlock::EditCollectionRootFolderPathInput,
        should_show_cursor: active_radarr_block
          == ActiveRadarrBlock::EditCollectionRootFolderPathInput,
        cursor_after_string: true,
      },
    );
  }

  draw_checkbox_with_label(
    f,
    search_on_add_area,
    "Search on Add",
    search_on_add.unwrap_or_default(),
    selected_block == &ActiveRadarrBlock::EditCollectionToggleSearchOnAdd,
  );

  draw_button(f, save_area, "Save", yes_no_value && highlight_yes_no);
  draw_button(f, cancel_area, "Cancel", !yes_no_value && highlight_yes_no);
}

fn draw_edit_collection_select_minimum_availability_popup(
  f: &mut Frame<'_>,
  app: &mut App<'_>,
  area: Rect,
) {
  draw_selectable_list(
    f,
    area,
    &mut app
      .data
      .radarr_data
      .edit_collection_modal
      .as_mut()
      .unwrap()
      .minimum_availability_list,
    |minimum_availability| ListItem::new(minimum_availability.to_display_str().to_owned()),
  );
}

fn draw_edit_collection_select_quality_profile_popup(
  f: &mut Frame<'_>,
  app: &mut App<'_>,
  area: Rect,
) {
  draw_selectable_list(
    f,
    area,
    &mut app
      .data
      .radarr_data
      .edit_collection_modal
      .as_mut()
      .unwrap()
      .quality_profile_list,
    |quality_profile| ListItem::new(quality_profile.clone()),
  );
}
