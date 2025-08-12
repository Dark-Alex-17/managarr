use ratatui::layout::{Constraint, Rect};
use ratatui::widgets::{Cell, Row};
use ratatui::Frame;

use crate::app::App;
use crate::models::radarr_models::Movie;
use crate::models::servarr_data::radarr::radarr_data::{ActiveRadarrBlock, LIBRARY_BLOCKS};
use crate::models::Route;
use crate::ui::radarr_ui::decorate_with_row_style;
use crate::ui::radarr_ui::library::add_movie_ui::AddMovieUi;
use crate::ui::radarr_ui::library::delete_movie_ui::DeleteMovieUi;
use crate::ui::radarr_ui::library::edit_movie_ui::EditMovieUi;
use crate::ui::radarr_ui::library::movie_details_ui::MovieDetailsUi;
use crate::ui::utils::{get_width_from_percentage, layout_block_top_border};
use crate::ui::widgets::confirmation_prompt::ConfirmationPrompt;
use crate::ui::widgets::managarr_table::ManagarrTable;
use crate::ui::widgets::popup::{Popup, Size};
use crate::ui::DrawUi;
use crate::utils::{convert_runtime, convert_to_gb};

mod add_movie_ui;
mod delete_movie_ui;
mod edit_movie_ui;
mod movie_details_ui;

#[cfg(test)]
#[path = "library_ui_tests.rs"]
mod library_ui_tests;

pub(super) struct LibraryUi;

impl DrawUi for LibraryUi {
  fn accepts(route: Route) -> bool {
    if let Route::Radarr(active_radarr_block, _) = route {
      return MovieDetailsUi::accepts(route)
        || AddMovieUi::accepts(route)
        || EditMovieUi::accepts(route)
        || DeleteMovieUi::accepts(route)
        || LIBRARY_BLOCKS.contains(&active_radarr_block);
    }

    false
  }

  fn draw(f: &mut Frame<'_>, app: &mut App<'_>, area: Rect) {
    let route = app.get_current_route();
    if let Route::Radarr(_, context_option) = route {
      if context_option.is_some() && AddMovieUi::accepts(route) {
        AddMovieUi::draw(f, app, area);
        return;
      }

      draw_library(f, app, area);

      match route {
        _ if MovieDetailsUi::accepts(route) => MovieDetailsUi::draw(f, app, area),
        _ if AddMovieUi::accepts(route) => AddMovieUi::draw(f, app, area),
        _ if EditMovieUi::accepts(route) => EditMovieUi::draw(f, app, area),
        _ if DeleteMovieUi::accepts(route) => DeleteMovieUi::draw(f, app, area),
        Route::Radarr(ActiveRadarrBlock::UpdateAllMoviesPrompt, _) => {
          let confirmation_prompt = ConfirmationPrompt::new()
            .title("Update All Movies")
            .prompt("Do you want to update info and scan your disks for all of your movies?")
            .yes_no_value(app.data.radarr_data.prompt_confirm);

          f.render_widget(
            Popup::new(confirmation_prompt).size(Size::MediumPrompt),
            f.area(),
          );
        }
        _ => (),
      }
    }
  }
}

fn draw_library(f: &mut Frame<'_>, app: &mut App<'_>, area: Rect) {
  if let Route::Radarr(active_radarr_block, _) = app.get_current_route() {
    let current_selection = if !app.data.radarr_data.movies.items.is_empty() {
      app.data.radarr_data.movies.current_selection().clone()
    } else {
      Movie::default()
    };
    let quality_profile_map = &app.data.radarr_data.quality_profile_map;
    let tags_map = &app.data.radarr_data.tags_map;
    let downloads_vec = &app.data.radarr_data.downloads.items;
    let content = Some(&mut app.data.radarr_data.movies);

    let library_table_row_mapping = |movie: &Movie| {
      movie.title.scroll_left_or_reset(
        get_width_from_percentage(area, 27),
        *movie == current_selection,
        app.tick_count % app.ticks_until_scroll == 0,
      );
      let monitored = if movie.monitored { "🏷" } else { "" };
      let studio = movie.studio.clone().unwrap_or_default();
      let (hours, minutes) = convert_runtime(movie.runtime);
      let file_size: f64 = convert_to_gb(movie.size_on_disk);
      let certification = movie.certification.clone().unwrap_or_default();
      let quality_profile = quality_profile_map
        .get_by_left(&movie.quality_profile_id)
        .unwrap()
        .to_owned();
      let empty_tag = String::new();
      let tags = if !movie.tags.is_empty() {
        movie
          .tags
          .iter()
          .map(|tag_id| {
            tags_map
              .get_by_left(&tag_id.as_i64().unwrap())
              .unwrap_or(&empty_tag)
              .clone()
          })
          .collect::<Vec<String>>()
          .join(", ")
      } else {
        String::new()
      };

      decorate_with_row_style(
        downloads_vec,
        movie,
        Row::new(vec![
          Cell::from(movie.title.to_string()),
          Cell::from(movie.year.to_string()),
          Cell::from(studio),
          Cell::from(format!("{hours}h {minutes}m")),
          Cell::from(certification),
          Cell::from(movie.original_language.name.to_owned()),
          Cell::from(format!("{file_size:.2} GB")),
          Cell::from(quality_profile),
          Cell::from(monitored.to_owned()),
          Cell::from(tags),
        ]),
      )
    };
    let library_table = ManagarrTable::new(content, library_table_row_mapping)
      .block(layout_block_top_border())
      .loading(app.is_loading)
      .sorting(active_radarr_block == ActiveRadarrBlock::MoviesSortPrompt)
      .searching(active_radarr_block == ActiveRadarrBlock::SearchMovie)
      .search_produced_empty_results(active_radarr_block == ActiveRadarrBlock::SearchMovieError)
      .filtering(active_radarr_block == ActiveRadarrBlock::FilterMovies)
      .filter_produced_empty_results(active_radarr_block == ActiveRadarrBlock::FilterMoviesError)
      .headers([
        "Title",
        "Year",
        "Studio",
        "Runtime",
        "Rating",
        "Language",
        "Size",
        "Quality Profile",
        "Monitored",
        "Tags",
      ])
      .constraints([
        Constraint::Percentage(27),
        Constraint::Percentage(4),
        Constraint::Percentage(17),
        Constraint::Percentage(6),
        Constraint::Percentage(6),
        Constraint::Percentage(6),
        Constraint::Percentage(6),
        Constraint::Percentage(10),
        Constraint::Percentage(6),
        Constraint::Percentage(12),
      ]);

    if [
      ActiveRadarrBlock::SearchMovie,
      ActiveRadarrBlock::FilterMovies,
    ]
    .contains(&active_radarr_block)
    {
      library_table.show_cursor(f, area);
    }

    f.render_widget(library_table, area);
  }
}
