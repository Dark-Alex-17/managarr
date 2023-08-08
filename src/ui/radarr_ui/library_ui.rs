use tui::backend::Backend;
use tui::layout::{Constraint, Rect};
use tui::widgets::{Cell, Row};
use tui::Frame;

use crate::app::radarr::ActiveRadarrBlock;
use crate::app::App;
use crate::models::radarr_models::Movie;
use crate::models::Route;
use crate::ui::radarr_ui::{determine_row_style, draw_filter_box, draw_search_box};
use crate::ui::utils::{get_width_from_percentage, layout_block_top_border};
use crate::ui::{
  draw_popup_over, draw_prompt_box, draw_prompt_popup_over, draw_table, DrawUi, TableProps,
};
use crate::utils::{convert_runtime, convert_to_gb};

pub(super) struct LibraryUi {}

impl DrawUi for LibraryUi {
  fn draw<B: Backend>(f: &mut Frame<'_, B>, app: &mut App<'_>, content_rect: Rect) {
    if let Route::Radarr(active_radarr_block, _) = app.get_current_route() {
      match active_radarr_block {
        ActiveRadarrBlock::Movies => draw_library(f, app, content_rect),
        ActiveRadarrBlock::SearchMovie => {
          draw_popup_over(f, app, content_rect, draw_library, draw_search_box, 30, 11)
        }
        ActiveRadarrBlock::FilterMovies => {
          draw_popup_over(f, app, content_rect, draw_library, draw_filter_box, 30, 11)
        }
        ActiveRadarrBlock::UpdateAllMoviesPrompt => draw_prompt_popup_over(
          f,
          app,
          content_rect,
          draw_library,
          draw_update_all_movies_prompt,
        ),
        _ => (),
      }
    }
  }
}

pub(super) fn draw_library<B: Backend>(f: &mut Frame<'_, B>, app: &mut App<'_>, area: Rect) {
  let current_selection = if !app.data.radarr_data.filtered_movies.items.is_empty() {
    app
      .data
      .radarr_data
      .filtered_movies
      .current_selection()
      .clone()
  } else if !app.data.radarr_data.movies.items.is_empty() {
    app.data.radarr_data.movies.current_selection().clone()
  } else {
    Movie::default()
  };
  let quality_profile_map = &app.data.radarr_data.quality_profile_map;
  let tags_map = &app.data.radarr_data.tags_map;
  let downloads_vec = &app.data.radarr_data.downloads.items;
  let content = if !app.data.radarr_data.filtered_movies.items.is_empty()
    && !app.data.radarr_data.is_filtering
  {
    &mut app.data.radarr_data.filtered_movies
  } else {
    &mut app.data.radarr_data.movies
  };

  draw_table(
    f,
    area,
    layout_block_top_border(),
    TableProps {
      content,
      table_headers: vec![
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
      ],
      constraints: vec![
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
      ],
      help: app
        .data
        .radarr_data
        .main_tabs
        .get_active_tab_contextual_help(),
    },
    |movie| {
      movie.title.scroll_left_or_reset(
        get_width_from_percentage(area, 27),
        *movie == current_selection,
        app.tick_count % app.ticks_until_scroll == 0,
      );
      let monitored = if movie.monitored { "🏷" } else { "" };
      let (hours, minutes) = convert_runtime(movie.runtime.as_u64().unwrap());
      let file_size: f64 = convert_to_gb(movie.size_on_disk.as_u64().unwrap());
      let certification = movie.certification.clone().unwrap_or_else(|| "".to_owned());
      let quality_profile = quality_profile_map
        .get_by_left(&movie.quality_profile_id.as_u64().unwrap())
        .unwrap()
        .to_owned();
      let tags = movie
        .tags
        .iter()
        .map(|tag_id| {
          tags_map
            .get_by_left(&tag_id.as_u64().unwrap())
            .unwrap()
            .clone()
        })
        .collect::<Vec<String>>()
        .join(", ");

      Row::new(vec![
        Cell::from(movie.title.to_string()),
        Cell::from(movie.year.to_string()),
        Cell::from(movie.studio.to_string()),
        Cell::from(format!("{}h {}m", hours, minutes)),
        Cell::from(certification),
        Cell::from(movie.original_language.name.to_owned()),
        Cell::from(format!("{:.2} GB", file_size)),
        Cell::from(quality_profile),
        Cell::from(monitored.to_owned()),
        Cell::from(tags),
      ])
      .style(determine_row_style(downloads_vec, movie))
    },
    app.is_loading,
  );
}

fn draw_update_all_movies_prompt<B: Backend>(
  f: &mut Frame<'_, B>,
  app: &mut App<'_>,
  prompt_area: Rect,
) {
  draw_prompt_box(
    f,
    prompt_area,
    "Update All Movies",
    "Do you want to update info and scan your disks for all of your movies?",
    app.data.radarr_data.prompt_confirm,
  );
}
