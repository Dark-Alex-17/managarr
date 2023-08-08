use tui::backend::Backend;
use tui::layout::{Constraint, Rect};
use tui::widgets::ListItem;
use tui::Frame;

use crate::app::App;
use crate::models::servarr_data::radarr::modals::EditCollectionModal;
use crate::models::servarr_data::radarr::radarr_data::{
  ActiveRadarrBlock, COLLECTION_DETAILS_BLOCKS, EDIT_COLLECTION_BLOCKS,
};
use crate::models::Route;
use crate::ui::radarr_ui::collections::collection_details_ui::CollectionDetailsUi;
use crate::ui::radarr_ui::collections::draw_collections;
use crate::ui::utils::{
  horizontal_chunks, layout_paragraph_borderless, title_block_centered, vertical_chunks_with_margin,
};
use crate::ui::{
  draw_button, draw_checkbox_with_label, draw_drop_down_menu_button, draw_drop_down_popup,
  draw_large_popup_over_background_fn_with_ui, draw_medium_popup_over, draw_popup,
  draw_selectable_list, draw_text_box_with_label, DrawUi,
};

#[cfg(test)]
#[path = "edit_collection_ui_tests.rs"]
mod edit_collection_ui_tests;

pub(super) struct EditCollectionUi {}

impl DrawUi for EditCollectionUi {
  fn accepts(route: Route) -> bool {
    if let Route::Radarr(active_radarr_block, _) = route {
      return EDIT_COLLECTION_BLOCKS.contains(&active_radarr_block);
    }

    false
  }

  fn draw<B: Backend>(f: &mut Frame<'_, B>, app: &mut App<'_>, content_rect: Rect) {
    if let Route::Radarr(active_radarr_block, context_option) = *app.get_current_route() {
      let draw_edit_collection_prompt =
        |f: &mut Frame<'_, B>, app: &mut App<'_>, prompt_area: Rect| match active_radarr_block {
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
          ActiveRadarrBlock::Collections => draw_medium_popup_over(
            f,
            app,
            content_rect,
            draw_collections,
            draw_edit_collection_prompt,
          ),
          _ if COLLECTION_DETAILS_BLOCKS.contains(&context) => {
            draw_large_popup_over_background_fn_with_ui::<B, CollectionDetailsUi>(
              f,
              app,
              content_rect,
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

fn draw_edit_collection_confirmation_prompt<B: Backend>(
  f: &mut Frame<'_, B>,
  app: &mut App<'_>,
  prompt_area: Rect,
) {
  let (collection_title, collection_overview) =
    if app.data.radarr_data.filtered_collections.items.is_empty() {
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
    } else {
      (
        app
          .data
          .radarr_data
          .filtered_collections
          .current_selection()
          .title
          .text
          .clone(),
        app
          .data
          .radarr_data
          .filtered_collections
          .current_selection()
          .overview
          .clone()
          .unwrap_or_default(),
      )
    };
  let title = format!("Edit - {}", collection_title);
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

  f.render_widget(title_block_centered(&title), prompt_area);

  let chunks = vertical_chunks_with_margin(
    vec![
      Constraint::Length(6),
      Constraint::Length(3),
      Constraint::Length(3),
      Constraint::Length(3),
      Constraint::Length(3),
      Constraint::Length(3),
      Constraint::Min(0),
      Constraint::Length(3),
    ],
    prompt_area,
    1,
  );

  let prompt_paragraph = layout_paragraph_borderless(&collection_overview);
  f.render_widget(prompt_paragraph, chunks[0]);

  let horizontal_chunks = horizontal_chunks(
    vec![Constraint::Percentage(50), Constraint::Percentage(50)],
    chunks[7],
  );

  draw_checkbox_with_label(
    f,
    chunks[1],
    "Monitored",
    monitored.unwrap_or_default(),
    selected_block == &ActiveRadarrBlock::EditCollectionToggleMonitored,
  );

  draw_drop_down_menu_button(
    f,
    chunks[2],
    "Minimum Availability",
    selected_minimum_availability.to_display_str(),
    selected_block == &ActiveRadarrBlock::EditCollectionSelectMinimumAvailability,
  );
  draw_drop_down_menu_button(
    f,
    chunks[3],
    "Quality Profile",
    selected_quality_profile,
    selected_block == &ActiveRadarrBlock::EditCollectionSelectQualityProfile,
  );

  if let Route::Radarr(active_radarr_block, _) = *app.get_current_route() {
    draw_text_box_with_label(
      f,
      chunks[4],
      "Root Folder",
      &path.text,
      *path.offset.borrow(),
      selected_block == &ActiveRadarrBlock::EditCollectionRootFolderPathInput,
      active_radarr_block == ActiveRadarrBlock::EditCollectionRootFolderPathInput,
    );
  }

  draw_checkbox_with_label(
    f,
    chunks[5],
    "Search on Add",
    search_on_add.unwrap_or_default(),
    selected_block == &ActiveRadarrBlock::EditCollectionToggleSearchOnAdd,
  );

  draw_button(
    f,
    horizontal_chunks[0],
    "Save",
    yes_no_value && highlight_yes_no,
  );
  draw_button(
    f,
    horizontal_chunks[1],
    "Cancel",
    !yes_no_value && highlight_yes_no,
  );
}

fn draw_edit_collection_select_minimum_availability_popup<B: Backend>(
  f: &mut Frame<'_, B>,
  app: &mut App<'_>,
  popup_area: Rect,
) {
  draw_selectable_list(
    f,
    popup_area,
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

fn draw_edit_collection_select_quality_profile_popup<B: Backend>(
  f: &mut Frame<'_, B>,
  app: &mut App<'_>,
  popup_area: Rect,
) {
  draw_selectable_list(
    f,
    popup_area,
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
