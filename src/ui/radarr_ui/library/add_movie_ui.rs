use tui::backend::Backend;
use tui::layout::{Alignment, Constraint, Rect};
use tui::text::Text;
use tui::widgets::{Cell, ListItem, Paragraph, Row};
use tui::Frame;

use crate::app::radarr::{ActiveRadarrBlock, ADD_MOVIE_BLOCKS};
use crate::models::radarr_models::AddMovieSearchResult;
use crate::models::Route;
use crate::ui::radarr_ui::collections::{draw_collection_details, draw_collections};
use crate::ui::radarr_ui::library::draw_library;
use crate::ui::radarr_ui::{
  draw_select_minimum_availability_popup, draw_select_quality_profile_popup,
  draw_select_root_folder_popup,
};
use crate::ui::utils::{
  borderless_block, get_width_from_percentage, horizontal_chunks, layout_block,
  layout_paragraph_borderless, style_help, style_primary, title_block_centered,
  vertical_chunks_with_margin,
};
use crate::ui::{
  draw_button, draw_drop_down_menu_button, draw_drop_down_popup, draw_error_popup,
  draw_error_popup_over, draw_large_popup_over, draw_medium_popup_over, draw_selectable_list,
  draw_table, draw_text_box, draw_text_box_with_label, DrawUi, TableProps,
};
use crate::utils::convert_runtime;
use crate::App;

#[cfg(test)]
#[path = "add_movie_ui_tests.rs"]
mod add_movie_ui_tests;

pub(super) struct AddMovieUi {}

impl DrawUi for AddMovieUi {
  fn accepts(route: Route) -> bool {
    if let Route::Radarr(active_radarr_block, _) = route {
      return ADD_MOVIE_BLOCKS.contains(&active_radarr_block);
    }

    false
  }

  fn draw<B: Backend>(f: &mut Frame<'_, B>, app: &mut App<'_>, content_rect: Rect) {
    if let Route::Radarr(active_radarr_block, context_option) = *app.get_current_route() {
      let draw_add_movie_search_popup =
        |f: &mut Frame<'_, B>, app: &mut App<'_>, area: Rect| match active_radarr_block {
          ActiveRadarrBlock::AddMovieSearchInput
          | ActiveRadarrBlock::AddMovieSearchResults
          | ActiveRadarrBlock::AddMovieEmptySearchResults => {
            draw_add_movie_search(f, app, area);
          }
          ActiveRadarrBlock::AddMoviePrompt
          | ActiveRadarrBlock::AddMovieSelectMonitor
          | ActiveRadarrBlock::AddMovieSelectMinimumAvailability
          | ActiveRadarrBlock::AddMovieSelectQualityProfile
          | ActiveRadarrBlock::AddMovieSelectRootFolder
          | ActiveRadarrBlock::AddMovieTagsInput => {
            if context_option.is_some() {
              draw_medium_popup_over(
                f,
                app,
                area,
                draw_collection_details,
                draw_confirmation_popup,
              );
            } else {
              draw_medium_popup_over(f, app, area, draw_add_movie_search, draw_confirmation_popup);
            }
          }
          ActiveRadarrBlock::AddMovieAlreadyInLibrary => draw_error_popup_over(
            f,
            app,
            area,
            "This film is already in your library",
            draw_add_movie_search,
          ),
          _ => (),
        };

      match active_radarr_block {
        _ if ADD_MOVIE_BLOCKS.contains(&active_radarr_block) => {
          if context_option.is_some() {
            draw_large_popup_over(
              f,
              app,
              content_rect,
              draw_collections,
              draw_add_movie_search_popup,
            )
          } else {
            draw_large_popup_over(
              f,
              app,
              content_rect,
              draw_library,
              draw_add_movie_search_popup,
            )
          }
        }
        _ => (),
      }
    }
  }
}

fn draw_add_movie_search<B: Backend>(f: &mut Frame<'_, B>, app: &mut App<'_>, area: Rect) {
  let current_selection = if app.data.radarr_data.add_searched_movies.items.is_empty() {
    AddMovieSearchResult::default()
  } else {
    app
      .data
      .radarr_data
      .add_searched_movies
      .current_selection()
      .clone()
  };

  let chunks = vertical_chunks_with_margin(
    vec![
      Constraint::Length(3),
      Constraint::Min(0),
      Constraint::Length(3),
    ],
    area,
    1,
  );
  let block_content = &app.data.radarr_data.search.text;
  let offset = *app.data.radarr_data.search.offset.borrow();

  if let Route::Radarr(active_radarr_block, _) = *app.get_current_route() {
    match active_radarr_block {
      ActiveRadarrBlock::AddMovieSearchInput => {
        draw_text_box(
          f,
          chunks[0],
          Some("Add Movie"),
          block_content,
          offset,
          true,
          false,
        );
        f.render_widget(layout_block(), chunks[1]);

        let mut help_text = Text::from("<esc> close");
        help_text.patch_style(style_help());
        let help_paragraph = Paragraph::new(help_text)
          .block(borderless_block())
          .alignment(Alignment::Center);
        f.render_widget(help_paragraph, chunks[2]);
      }
      ActiveRadarrBlock::AddMovieEmptySearchResults => {
        f.render_widget(layout_block(), chunks[1]);
        draw_error_popup(f, "No movies found matching your query!");
      }
      ActiveRadarrBlock::AddMovieSearchResults
      | ActiveRadarrBlock::AddMoviePrompt
      | ActiveRadarrBlock::AddMovieSelectMonitor
      | ActiveRadarrBlock::AddMovieSelectMinimumAvailability
      | ActiveRadarrBlock::AddMovieSelectQualityProfile
      | ActiveRadarrBlock::AddMovieSelectRootFolder
      | ActiveRadarrBlock::AddMovieAlreadyInLibrary
      | ActiveRadarrBlock::AddMovieTagsInput => {
        let mut help_text = Text::from("<enter> details | <esc> edit search");
        help_text.patch_style(style_help());
        let help_paragraph = Paragraph::new(help_text)
          .block(borderless_block())
          .alignment(Alignment::Center);
        f.render_widget(help_paragraph, chunks[2]);

        draw_table(
          f,
          chunks[1],
          layout_block(),
          TableProps {
            content: &mut app.data.radarr_data.add_searched_movies,
            table_headers: vec![
              "✔",
              "Title",
              "Year",
              "Runtime",
              "IMDB",
              "Rotten Tomatoes",
              "Genres",
            ],
            constraints: vec![
              Constraint::Percentage(2),
              Constraint::Percentage(27),
              Constraint::Percentage(8),
              Constraint::Percentage(10),
              Constraint::Percentage(8),
              Constraint::Percentage(14),
              Constraint::Percentage(28),
            ],
            help: None,
          },
          |movie| {
            let (hours, minutes) = convert_runtime(movie.runtime.as_u64().unwrap());
            let imdb_rating = movie
              .ratings
              .imdb
              .clone()
              .unwrap_or_default()
              .value
              .as_f64()
              .unwrap();
            let rotten_tomatoes_rating = movie
              .ratings
              .rotten_tomatoes
              .clone()
              .unwrap_or_default()
              .value
              .as_u64()
              .unwrap();
            let imdb_rating = if imdb_rating == 0.0 {
              String::new()
            } else {
              format!("{:.1}", imdb_rating)
            };
            let rotten_tomatoes_rating = if rotten_tomatoes_rating == 0 {
              String::new()
            } else {
              format!("{}%", rotten_tomatoes_rating)
            };
            let in_library = if app
              .data
              .radarr_data
              .movies
              .items
              .iter()
              .any(|mov| mov.tmdb_id == movie.tmdb_id)
            {
              "✔"
            } else {
              ""
            };

            movie.title.scroll_left_or_reset(
              get_width_from_percentage(area, 27),
              *movie == current_selection,
              app.tick_count % app.ticks_until_scroll == 0,
            );

            Row::new(vec![
              Cell::from(in_library),
              Cell::from(movie.title.to_string()),
              Cell::from(movie.year.as_u64().unwrap().to_string()),
              Cell::from(format!("{}h {}m", hours, minutes)),
              Cell::from(imdb_rating),
              Cell::from(rotten_tomatoes_rating),
              Cell::from(movie.genres.join(", ")),
            ])
            .style(style_primary())
          },
          app.is_loading,
          true,
        );
      }
      _ => (),
    }
  }

  draw_text_box(
    f,
    chunks[0],
    Some("Add Movie"),
    block_content,
    offset,
    false,
    false,
  );
}

fn draw_confirmation_popup<B: Backend>(f: &mut Frame<'_, B>, app: &mut App<'_>, prompt_area: Rect) {
  if let Route::Radarr(active_radarr_block, _) = *app.get_current_route() {
    match active_radarr_block {
      ActiveRadarrBlock::AddMovieSelectMonitor => {
        draw_drop_down_popup(
          f,
          app,
          prompt_area,
          draw_confirmation_prompt,
          draw_select_monitor_popup,
        );
      }
      ActiveRadarrBlock::AddMovieSelectMinimumAvailability => {
        draw_drop_down_popup(
          f,
          app,
          prompt_area,
          draw_confirmation_prompt,
          draw_select_minimum_availability_popup,
        );
      }
      ActiveRadarrBlock::AddMovieSelectQualityProfile => {
        draw_drop_down_popup(
          f,
          app,
          prompt_area,
          draw_confirmation_prompt,
          draw_select_quality_profile_popup,
        );
      }
      ActiveRadarrBlock::AddMovieSelectRootFolder => {
        draw_drop_down_popup(
          f,
          app,
          prompt_area,
          draw_confirmation_prompt,
          draw_select_root_folder_popup,
        );
      }
      ActiveRadarrBlock::AddMoviePrompt | ActiveRadarrBlock::AddMovieTagsInput => {
        draw_confirmation_prompt(f, app, prompt_area)
      }
      _ => (),
    }
  }
}

fn draw_select_monitor_popup<B: Backend>(
  f: &mut Frame<'_, B>,
  app: &mut App<'_>,
  popup_area: Rect,
) {
  draw_selectable_list(
    f,
    popup_area,
    &mut app.data.radarr_data.monitor_list,
    |monitor| ListItem::new(monitor.to_display_str().to_owned()),
  );
}

fn draw_confirmation_prompt<B: Backend>(
  f: &mut Frame<'_, B>,
  app: &mut App<'_>,
  prompt_area: Rect,
) {
  let (movie_title, movie_overview) = if let Route::Radarr(_, Some(_)) = app.get_current_route() {
    (
      &app
        .data
        .radarr_data
        .collection_movies
        .current_selection()
        .title
        .text,
      app
        .data
        .radarr_data
        .collection_movies
        .current_selection()
        .overview
        .clone(),
    )
  } else {
    (
      &app
        .data
        .radarr_data
        .add_searched_movies
        .current_selection()
        .title
        .text,
      app
        .data
        .radarr_data
        .add_searched_movies
        .current_selection()
        .overview
        .clone(),
    )
  };
  let title = format!("Add Movie - {}", movie_title);
  let prompt = movie_overview;
  let yes_no_value = app.data.radarr_data.prompt_confirm;
  let selected_block = app.data.radarr_data.selected_block.get_active_block();
  let highlight_yes_no = selected_block == &ActiveRadarrBlock::AddMovieConfirmPrompt;

  let selected_monitor = app.data.radarr_data.monitor_list.current_selection();
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
  let selected_root_folder = app.data.radarr_data.root_folder_list.current_selection();

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

  let prompt_paragraph = layout_paragraph_borderless(&prompt);
  f.render_widget(prompt_paragraph, chunks[0]);

  let horizontal_chunks = horizontal_chunks(
    vec![Constraint::Percentage(50), Constraint::Percentage(50)],
    chunks[7],
  );

  draw_drop_down_menu_button(
    f,
    chunks[1],
    "Root Folder",
    &selected_root_folder.path,
    selected_block == &ActiveRadarrBlock::AddMovieSelectRootFolder,
  );

  draw_drop_down_menu_button(
    f,
    chunks[2],
    "Monitor",
    selected_monitor.to_display_str(),
    selected_block == &ActiveRadarrBlock::AddMovieSelectMonitor,
  );

  draw_drop_down_menu_button(
    f,
    chunks[3],
    "Minimum Availability",
    selected_minimum_availability.to_display_str(),
    selected_block == &ActiveRadarrBlock::AddMovieSelectMinimumAvailability,
  );
  draw_drop_down_menu_button(
    f,
    chunks[4],
    "Quality Profile",
    selected_quality_profile,
    selected_block == &ActiveRadarrBlock::AddMovieSelectQualityProfile,
  );

  if let Route::Radarr(active_radarr_block, _) = *app.get_current_route() {
    draw_text_box_with_label(
      f,
      chunks[5],
      "Tags",
      &app.data.radarr_data.edit_tags.text,
      *app.data.radarr_data.edit_tags.offset.borrow(),
      selected_block == &ActiveRadarrBlock::AddMovieTagsInput,
      active_radarr_block == ActiveRadarrBlock::AddMovieTagsInput,
    );
  }

  draw_button(
    f,
    horizontal_chunks[0],
    "Add",
    yes_no_value && highlight_yes_no,
  );
  draw_button(
    f,
    horizontal_chunks[1],
    "Cancel",
    !yes_no_value && highlight_yes_no,
  );
}