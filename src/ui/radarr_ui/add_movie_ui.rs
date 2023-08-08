use tui::backend::Backend;
use tui::layout::{Alignment, Constraint, Rect};
use tui::style::Modifier;
use tui::text::Text;
use tui::widgets::{Cell, ListItem, Paragraph, Row, Wrap};
use tui::Frame;

use crate::app::radarr::ActiveRadarrBlock;
use crate::models::radarr_models::AddMovieSearchResult;
use crate::models::Route;
use crate::ui::utils::{
  borderless_block, get_width, horizontal_chunks, layout_block, show_cursor, style_default,
  style_help, style_primary, title_block_centered, vertical_chunks_with_margin,
};
use crate::ui::{
  draw_button, draw_drop_down_list, draw_drop_down_menu_button, draw_drop_down_popup,
  draw_medium_popup_over, draw_table, TableProps,
};
use crate::utils::convert_runtime;
use crate::App;

pub(super) fn draw_add_movie_search_popup<B: Backend>(
  f: &mut Frame<'_, B>,
  app: &mut App,
  area: Rect,
) {
  if let Route::Radarr(active_radarr_block) = app.get_current_route().clone() {
    match active_radarr_block {
      ActiveRadarrBlock::AddMovieSearchInput | ActiveRadarrBlock::AddMovieSearchResults => {
        draw_add_movie_search(f, app, area);
      }
      ActiveRadarrBlock::AddMoviePrompt
      | ActiveRadarrBlock::AddMovieSelectMonitor
      | ActiveRadarrBlock::AddMovieSelectMinimumAvailability
      | ActiveRadarrBlock::AddMovieSelectQualityProfile => {
        draw_medium_popup_over(f, app, area, draw_add_movie_search, draw_confirmation_popup);
      }
      _ => (),
    }
  }
}

fn draw_add_movie_search<B: Backend>(f: &mut Frame<'_, B>, app: &mut App, area: Rect) {
  let current_selection = if app.data.radarr_data.add_searched_movies.items.is_empty() {
    AddMovieSearchResult::default()
  } else {
    app
      .data
      .radarr_data
      .add_searched_movies
      .current_selection_clone()
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
  let block_content = app.data.radarr_data.search.as_str();

  let search_paragraph = Paragraph::new(Text::from(block_content))
    .style(style_default())
    .block(title_block_centered("Add Movie"));

  if let Route::Radarr(active_radarr_block) = app.get_current_route().clone() {
    match active_radarr_block {
      ActiveRadarrBlock::AddMovieSearchInput => {
        show_cursor(f, chunks[0], block_content);
        f.render_widget(layout_block(), chunks[1]);

        let mut help_text = Text::from("<esc> close");
        help_text.patch_style(style_help());
        let help_paragraph = Paragraph::new(help_text)
          .block(borderless_block())
          .alignment(Alignment::Center);
        f.render_widget(help_paragraph, chunks[2]);
      }
      ActiveRadarrBlock::AddMovieSearchResults
      | ActiveRadarrBlock::AddMoviePrompt
      | ActiveRadarrBlock::AddMovieSelectMonitor
      | ActiveRadarrBlock::AddMovieSelectMinimumAvailability
      | ActiveRadarrBlock::AddMovieSelectQualityProfile => {
        let mut help_text = Text::from("<esc> edit search");
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
              "Title",
              "Year",
              "Runtime",
              "IMDB",
              "Rotten Tomatoes",
              "Genres",
            ],
            constraints: vec![
              Constraint::Percentage(27),
              Constraint::Percentage(8),
              Constraint::Percentage(10),
              Constraint::Percentage(8),
              Constraint::Percentage(14),
              Constraint::Percentage(30),
            ],
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
              String::default()
            } else {
              format!("{:.1}", imdb_rating)
            };
            let rotten_tomatoes_rating = if rotten_tomatoes_rating == 0 {
              String::default()
            } else {
              format!("{}%", rotten_tomatoes_rating)
            };

            movie
              .title
              .scroll_or_reset(get_width(area), *movie == current_selection);

            Row::new(vec![
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
        );
      }
      _ => (),
    }
  }

  f.render_widget(search_paragraph, chunks[0]);
}

fn draw_confirmation_popup<B: Backend>(f: &mut Frame<'_, B>, app: &mut App, prompt_area: Rect) {
  if let Route::Radarr(active_radarr_block) = app.get_current_route().clone() {
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
      ActiveRadarrBlock::AddMoviePrompt => draw_confirmation_prompt(f, app, prompt_area),
      _ => (),
    }
  }
}

fn draw_select_monitor_popup<B: Backend>(f: &mut Frame<'_, B>, app: &mut App, popup_area: Rect) {
  draw_drop_down_list(
    f,
    popup_area,
    &mut app.data.radarr_data.add_movie_monitor_list,
    |monitor| ListItem::new(monitor.to_display_str().to_owned()),
  );
}

fn draw_select_minimum_availability_popup<B: Backend>(
  f: &mut Frame<'_, B>,
  app: &mut App,
  popup_area: Rect,
) {
  draw_drop_down_list(
    f,
    popup_area,
    &mut app.data.radarr_data.add_movie_minimum_availability_list,
    |minimum_availability| ListItem::new(minimum_availability.to_display_str().to_owned()),
  );
}

fn draw_select_quality_profile_popup<B: Backend>(
  f: &mut Frame<'_, B>,
  app: &mut App,
  popup_area: Rect,
) {
  draw_drop_down_list(
    f,
    popup_area,
    &mut app.data.radarr_data.add_movie_quality_profile_list,
    |quality_profile| ListItem::new(quality_profile.clone()),
  );
}

fn draw_confirmation_prompt<B: Backend>(f: &mut Frame<'_, B>, app: &mut App, prompt_area: Rect) {
  let title = "Confirm Add Movie?";
  let prompt = format!(
    "{}:\n\n{}",
    app
      .data
      .radarr_data
      .add_searched_movies
      .current_selection()
      .title
      .to_string()
      .trim(),
    app
      .data
      .radarr_data
      .add_searched_movies
      .current_selection()
      .overview
  );
  let yes_no_value = &app.data.radarr_data.prompt_confirm;
  let selected_block = &app.data.radarr_data.selected_block;
  let highlight_yes_no = *selected_block == ActiveRadarrBlock::AddMovieConfirmPrompt;

  let selected_monitor = app
    .data
    .radarr_data
    .add_movie_monitor_list
    .current_selection();
  let selected_minimum_availability = app
    .data
    .radarr_data
    .add_movie_minimum_availability_list
    .current_selection();
  let selected_quality_profile = app
    .data
    .radarr_data
    .add_movie_quality_profile_list
    .current_selection();

  f.render_widget(title_block_centered(title), prompt_area);

  let chunks = vertical_chunks_with_margin(
    vec![
      Constraint::Percentage(40),
      Constraint::Length(3),
      Constraint::Length(3),
      Constraint::Length(3),
      Constraint::Min(5),
      Constraint::Length(3),
    ],
    prompt_area,
    1,
  );

  let prompt_paragraph = Paragraph::new(Text::from(prompt))
    .block(borderless_block())
    .style(style_primary().add_modifier(Modifier::BOLD))
    .wrap(Wrap { trim: false })
    .alignment(Alignment::Center);
  f.render_widget(prompt_paragraph, chunks[0]);

  let horizontal_chunks = horizontal_chunks(
    vec![Constraint::Percentage(50), Constraint::Percentage(50)],
    chunks[5],
  );

  draw_drop_down_menu_button(
    f,
    chunks[1],
    "Monitor",
    selected_monitor.to_display_str(),
    *selected_block == ActiveRadarrBlock::AddMovieSelectMonitor,
  );

  draw_drop_down_menu_button(
    f,
    chunks[2],
    "Minimum Availability",
    selected_minimum_availability.to_display_str(),
    *selected_block == ActiveRadarrBlock::AddMovieSelectMinimumAvailability,
  );
  draw_drop_down_menu_button(
    f,
    chunks[3],
    "Quality Profile",
    selected_quality_profile,
    *selected_block == ActiveRadarrBlock::AddMovieSelectQualityProfile,
  );

  draw_button(
    f,
    horizontal_chunks[0],
    "Yes",
    *yes_no_value && highlight_yes_no,
  );
  draw_button(
    f,
    horizontal_chunks[1],
    "No",
    !*yes_no_value && highlight_yes_no,
  );
}
