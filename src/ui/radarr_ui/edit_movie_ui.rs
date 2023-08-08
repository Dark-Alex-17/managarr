use tui::backend::Backend;
use tui::layout::{Constraint, Rect};
use tui::Frame;

use crate::app::radarr::{ActiveRadarrBlock, MOVIE_DETAILS_BLOCKS};
use crate::app::App;
use crate::models::Route;
use crate::ui::radarr_ui::library_ui::draw_library;
use crate::ui::radarr_ui::movie_details_ui::MovieDetailsUi;
use crate::ui::radarr_ui::{
  draw_select_minimum_availability_popup, draw_select_quality_profile_popup,
};
use crate::ui::utils::{
  horizontal_chunks, layout_paragraph_borderless, title_block_centered, vertical_chunks_with_margin,
};
use crate::ui::{
  draw_button, draw_checkbox_with_label, draw_drop_down_menu_button, draw_drop_down_popup,
  draw_large_popup_over_background_fn_with_ui, draw_medium_popup_over, draw_popup,
  draw_text_box_with_label, DrawUi,
};

pub(super) struct EditMovieUi {}

impl DrawUi for EditMovieUi {
  fn draw<B: Backend>(f: &mut Frame<'_, B>, app: &mut App<'_>, content_rect: Rect) {
    if let Route::Radarr(active_radarr_block, context_option) = *app.get_current_route() {
      let draw_edit_movie_prompt =
        |f: &mut Frame<'_, B>, app: &mut App<'_>, prompt_area: Rect| match active_radarr_block {
          ActiveRadarrBlock::EditMovieSelectMinimumAvailability => {
            draw_drop_down_popup(
              f,
              app,
              prompt_area,
              draw_edit_movie_confirmation_prompt,
              draw_select_minimum_availability_popup,
            );
          }
          ActiveRadarrBlock::EditMovieSelectQualityProfile => {
            draw_drop_down_popup(
              f,
              app,
              prompt_area,
              draw_edit_movie_confirmation_prompt,
              draw_select_quality_profile_popup,
            );
          }
          ActiveRadarrBlock::EditMoviePrompt
          | ActiveRadarrBlock::EditMovieToggleMonitored
          | ActiveRadarrBlock::EditMoviePathInput
          | ActiveRadarrBlock::EditMovieTagsInput => {
            draw_edit_movie_confirmation_prompt(f, app, prompt_area)
          }
          _ => (),
        };

      if let Some(context) = context_option {
        match context {
          ActiveRadarrBlock::Movies => {
            draw_medium_popup_over(f, app, content_rect, draw_library, draw_edit_movie_prompt);
          }
          _ if MOVIE_DETAILS_BLOCKS.contains(&context) => {
            draw_large_popup_over_background_fn_with_ui::<B, MovieDetailsUi>(
              f,
              app,
              content_rect,
              draw_library,
            );
            draw_popup(f, app, draw_edit_movie_prompt, 60, 60);
          }
          _ => (),
        }
      }
    }
  }
}

fn draw_edit_movie_confirmation_prompt<B: Backend>(
  f: &mut Frame<'_, B>,
  app: &mut App<'_>,
  prompt_area: Rect,
) {
  let (movie_title, movie_overview) = if app.data.radarr_data.filtered_movies.items.is_empty() {
    (
      app
        .data
        .radarr_data
        .movies
        .current_selection()
        .title
        .to_string(),
      app
        .data
        .radarr_data
        .movies
        .current_selection()
        .overview
        .clone(),
    )
  } else {
    (
      app
        .data
        .radarr_data
        .filtered_movies
        .current_selection()
        .title
        .to_string(),
      app
        .data
        .radarr_data
        .filtered_movies
        .current_selection()
        .overview
        .clone(),
    )
  };
  let title = format!("Edit - {}", movie_title);
  let yes_no_value = app.data.radarr_data.prompt_confirm;
  let selected_block = app.data.radarr_data.selected_block.get_active_block();
  let highlight_yes_no = selected_block == &ActiveRadarrBlock::EditMovieConfirmPrompt;

  let selected_minimum_availability = app
    .data
    .radarr_data
    .minimum_availability_list
    .current_selection();
  let selected_quality_profile = app
    .data
    .radarr_data
    .quality_profile_list
    .current_selection();

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

  let prompt_paragraph = layout_paragraph_borderless(&movie_overview);
  f.render_widget(prompt_paragraph, chunks[0]);

  let horizontal_chunks = horizontal_chunks(
    vec![Constraint::Percentage(50), Constraint::Percentage(50)],
    chunks[7],
  );

  draw_checkbox_with_label(
    f,
    chunks[1],
    "Monitored",
    app.data.radarr_data.edit_monitored.unwrap_or_default(),
    selected_block == &ActiveRadarrBlock::EditMovieToggleMonitored,
  );

  draw_drop_down_menu_button(
    f,
    chunks[2],
    "Minimum Availability",
    selected_minimum_availability.to_display_str(),
    selected_block == &ActiveRadarrBlock::EditMovieSelectMinimumAvailability,
  );
  draw_drop_down_menu_button(
    f,
    chunks[3],
    "Quality Profile",
    selected_quality_profile,
    selected_block == &ActiveRadarrBlock::EditMovieSelectQualityProfile,
  );

  if let Route::Radarr(active_radarr_block, _) = *app.get_current_route() {
    draw_text_box_with_label(
      f,
      chunks[4],
      "Path",
      &app.data.radarr_data.edit_path.text,
      *app.data.radarr_data.edit_path.offset.borrow(),
      selected_block == &ActiveRadarrBlock::EditMoviePathInput,
      active_radarr_block == ActiveRadarrBlock::EditMoviePathInput,
    );
    draw_text_box_with_label(
      f,
      chunks[5],
      "Tags",
      &app.data.radarr_data.edit_tags.text,
      *app.data.radarr_data.edit_tags.offset.borrow(),
      selected_block == &ActiveRadarrBlock::EditMovieTagsInput,
      active_radarr_block == ActiveRadarrBlock::EditMovieTagsInput,
    );
  }

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
