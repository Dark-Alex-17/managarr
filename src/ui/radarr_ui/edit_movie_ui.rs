use tui::backend::Backend;
use tui::layout::{Constraint, Rect};
use tui::Frame;

use crate::app::radarr::ActiveRadarrBlock;
use crate::app::App;
use crate::models::Route;
use crate::ui::radarr_ui::{
  draw_select_minimum_availability_popup, draw_select_quality_profile_popup,
};
use crate::ui::utils::{
  horizontal_chunks, layout_paragraph_borderless, title_block_centered, vertical_chunks_with_margin,
};
use crate::ui::{
  draw_button, draw_checkbox_with_label, draw_drop_down_menu_button, draw_drop_down_popup,
  draw_text_box_with_label,
};

pub(super) fn draw_edit_movie_prompt<B: Backend>(
  f: &mut Frame<'_, B>,
  app: &mut App,
  prompt_area: Rect,
) {
  if let Route::Radarr(active_radarr_block, _) = *app.get_current_route() {
    match active_radarr_block {
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
    }
  }
}

fn draw_edit_movie_confirmation_prompt<B: Backend>(
  f: &mut Frame<'_, B>,
  app: &mut App,
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
  let yes_no_value = &app.data.radarr_data.prompt_confirm;
  let selected_block = &app.data.radarr_data.selected_block;
  let highlight_yes_no = *selected_block == ActiveRadarrBlock::EditMovieConfirmPrompt;

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
      Constraint::Percentage(25),
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
    *selected_block == ActiveRadarrBlock::EditMovieToggleMonitored,
  );

  draw_drop_down_menu_button(
    f,
    chunks[2],
    "Minimum Availability",
    selected_minimum_availability.to_display_str(),
    *selected_block == ActiveRadarrBlock::EditMovieSelectMinimumAvailability,
  );
  draw_drop_down_menu_button(
    f,
    chunks[3],
    "Quality Profile",
    selected_quality_profile,
    *selected_block == ActiveRadarrBlock::EditMovieSelectQualityProfile,
  );

  if let Route::Radarr(active_radarr_block, _) = *app.get_current_route() {
    draw_text_box_with_label(
      f,
      chunks[4],
      "Path",
      &app.data.radarr_data.edit_path.text,
      *app.data.radarr_data.edit_path.offset.borrow(),
      *selected_block == ActiveRadarrBlock::EditMoviePathInput,
      active_radarr_block == ActiveRadarrBlock::EditMoviePathInput,
    );
    draw_text_box_with_label(
      f,
      chunks[5],
      "Tags",
      &app.data.radarr_data.edit_tags.text,
      *app.data.radarr_data.edit_tags.offset.borrow(),
      *selected_block == ActiveRadarrBlock::EditMovieTagsInput,
      active_radarr_block == ActiveRadarrBlock::EditMovieTagsInput,
    );
  }

  draw_button(
    f,
    horizontal_chunks[0],
    "Save",
    *yes_no_value && highlight_yes_no,
  );
  draw_button(
    f,
    horizontal_chunks[1],
    "Cancel",
    !*yes_no_value && highlight_yes_no,
  );
}
