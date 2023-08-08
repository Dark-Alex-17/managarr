use tui::backend::Backend;
use tui::layout::{Alignment, Constraint, Rect};
use tui::text::Text;
use tui::widgets::{Cell, Paragraph, Row};
use tui::Frame;

use crate::app::radarr::ActiveRadarrBlock;
use crate::models::Route;
use crate::ui::utils::{
  borderless_block, layout_block, show_cursor, style_default, style_help, style_primary,
  title_block_centered, vertical_chunks_with_margin,
};
use crate::ui::{draw_medium_popup_over, draw_prompt_box, draw_table, TableProps};
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
      ActiveRadarrBlock::AddMoviePrompt => {
        draw_medium_popup_over(
          f,
          app,
          area,
          draw_add_movie_search,
          draw_add_movie_confirmation_prompt,
        );
      }
      _ => (),
    }
  }
}

fn draw_add_movie_search<B: Backend>(f: &mut Frame<'_, B>, app: &mut App, area: Rect) {
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
    .block(title_block_centered("  Add Movie  "));

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
      ActiveRadarrBlock::AddMovieSearchResults | ActiveRadarrBlock::AddMoviePrompt => {
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
              "IMDB Rating",
              "Rotten Tomatoes Rating",
              "Genres",
            ],
            constraints: vec![
              Constraint::Percentage(20),
              Constraint::Percentage(8),
              Constraint::Percentage(10),
              Constraint::Percentage(10),
              Constraint::Percentage(18),
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

            Row::new(vec![
              Cell::from(movie.title.to_owned()),
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

fn draw_add_movie_confirmation_prompt<B: Backend>(
  f: &mut Frame<'_, B>,
  app: &mut App,
  prompt_area: Rect,
) {
  draw_prompt_box(
    f,
    prompt_area,
    "  Confirm Add Movie?  ",
    format!(
      "{}:\n\n{}",
      app
        .data
        .radarr_data
        .add_searched_movies
        .current_selection()
        .title,
      app
        .data
        .radarr_data
        .add_searched_movies
        .current_selection()
        .overview
    )
    .as_str(),
    &app.data.radarr_data.prompt_confirm,
  );
}
