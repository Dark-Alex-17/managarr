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
use crate::ui::widgets::error_message::ErrorMessage;
use crate::ui::widgets::managarr_table::ManagarrTable;
use crate::ui::widgets::popup::{Popup, Size};
use crate::ui::{draw_input_box_popup, draw_popup_over, draw_prompt_box, DrawUi};
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
    let route = *app.get_current_route();
    let mut library_ui_matchers = |active_radarr_block: ActiveRadarrBlock| match active_radarr_block
    {
      ActiveRadarrBlock::Movies | ActiveRadarrBlock::MoviesSortPrompt => draw_library(f, app, area),
      ActiveRadarrBlock::SearchMovie => draw_popup_over(
        f,
        app,
        area,
        draw_library,
        draw_movie_search_box,
        Size::InputBox,
      ),
      ActiveRadarrBlock::SearchMovieError => {
        draw_library(f, app, area);
        let popup = Popup::new(ErrorMessage::new("Movie not found!")).size(Size::Error);
        f.render_widget(popup, f.size());
      }
      ActiveRadarrBlock::FilterMovies => draw_popup_over(
        f,
        app,
        area,
        draw_library,
        draw_filter_movies_box,
        Size::InputBox,
      ),
      ActiveRadarrBlock::FilterMoviesError => {
        draw_library(f, app, area);
        let popup = Popup::new(ErrorMessage::new(
          "No movies found matching the given filter!",
        ))
        .size(Size::Error);
        f.render_widget(popup, f.size());
      }
      ActiveRadarrBlock::UpdateAllMoviesPrompt => draw_popup_over(
        f,
        app,
        area,
        draw_library,
        draw_update_all_movies_prompt,
        Size::Prompt,
      ),
      _ => (),
    };

    match route {
      _ if MovieDetailsUi::accepts(route) => MovieDetailsUi::draw(f, app, area),
      _ if AddMovieUi::accepts(route) => AddMovieUi::draw(f, app, area),
      _ if EditMovieUi::accepts(route) => EditMovieUi::draw(f, app, area),
      _ if DeleteMovieUi::accepts(route) => DeleteMovieUi::draw(f, app, area),
      Route::Radarr(active_radarr_block, _) if LIBRARY_BLOCKS.contains(&active_radarr_block) => {
        library_ui_matchers(active_radarr_block)
      }
      _ => (),
    }
  }
}

pub(super) fn draw_library(f: &mut Frame<'_>, app: &mut App<'_>, area: Rect) {
  if let Route::Radarr(active_radarr_block, _) = *app.get_current_route() {
    let current_selection = if !app.data.radarr_data.movies.items.is_empty() {
      app.data.radarr_data.movies.current_selection().clone()
    } else {
      Movie::default()
    };
    let quality_profile_map = &app.data.radarr_data.quality_profile_map;
    let tags_map = &app.data.radarr_data.tags_map;
    let downloads_vec = &app.data.radarr_data.downloads.items;
    let content = Some(&mut app.data.radarr_data.movies);
    let help_footer = app
      .data
      .radarr_data
      .main_tabs
      .get_active_tab_contextual_help();

    let library_table_row_mapping = |movie: &Movie| {
      movie.title.scroll_left_or_reset(
        get_width_from_percentage(area, 27),
        *movie == current_selection,
        app.tick_count % app.ticks_until_scroll == 0,
      );
      let monitored = if movie.monitored { "🏷" } else { "" };
      let (hours, minutes) = convert_runtime(movie.runtime);
      let file_size: f64 = convert_to_gb(movie.size_on_disk);
      let certification = movie.certification.clone().unwrap_or_default();
      let quality_profile = quality_profile_map
        .get_by_left(&movie.quality_profile_id)
        .unwrap()
        .to_owned();
      let tags = movie
        .tags
        .iter()
        .map(|tag_id| {
          tags_map
            .get_by_left(&tag_id.as_i64().unwrap())
            .unwrap()
            .clone()
        })
        .collect::<Vec<String>>()
        .join(", ");

      decorate_with_row_style(
        downloads_vec,
        movie,
        Row::new(vec![
          Cell::from(movie.title.to_string()),
          Cell::from(movie.year.to_string()),
          Cell::from(movie.studio.to_string()),
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
      .footer(help_footer)
      .sorting(active_radarr_block == ActiveRadarrBlock::MoviesSortPrompt)
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

    f.render_widget(library_table, area);
  }
}

fn draw_update_all_movies_prompt(f: &mut Frame<'_>, app: &mut App<'_>, area: Rect) {
  draw_prompt_box(
    f,
    area,
    "Update All Movies",
    "Do you want to update info and scan your disks for all of your movies?",
    app.data.radarr_data.prompt_confirm,
  );
}

fn draw_movie_search_box(f: &mut Frame<'_>, app: &mut App<'_>, area: Rect) {
  draw_input_box_popup(
    f,
    area,
    "Search",
    app.data.radarr_data.movies.search.as_ref().unwrap(),
  );
}

fn draw_filter_movies_box(f: &mut Frame<'_>, app: &mut App<'_>, area: Rect) {
  draw_input_box_popup(
    f,
    area,
    "Filter",
    app.data.radarr_data.movies.filter.as_ref().unwrap(),
  )
}
