use ratatui::layout::{Alignment, Constraint, Flex, Layout, Rect};
use ratatui::style::Stylize;
use ratatui::text::{Line, Text};
use ratatui::widgets::{Cell, Paragraph, Row, Wrap};
use ratatui::Frame;

use crate::app::context_clues::{build_context_clue_string, BARE_POPUP_CONTEXT_CLUES};
use crate::app::radarr::radarr_context_clues::COLLECTION_DETAILS_CONTEXT_CLUES;
use crate::app::App;
use crate::models::radarr_models::CollectionMovie;
use crate::models::servarr_data::radarr::radarr_data::{
  ActiveRadarrBlock, COLLECTION_DETAILS_BLOCKS,
};
use crate::models::Route;
use crate::ui::radarr_ui::collections::draw_collections;
use crate::ui::styles::ManagarrStyle;
use crate::ui::utils::{
  borderless_block, get_width_from_percentage, layout_block_top_border_with_title, title_block,
  title_style,
};
use crate::ui::{draw_large_popup_over, draw_small_popup_over, draw_table, DrawUi, TableProps};
use crate::utils::convert_runtime;

#[cfg(test)]
#[path = "collection_details_ui_tests.rs"]
mod collection_details_ui_tests;

pub(super) struct CollectionDetailsUi;

impl DrawUi for CollectionDetailsUi {
  fn accepts(route: Route) -> bool {
    if let Route::Radarr(active_radarr_block, _) = route {
      return COLLECTION_DETAILS_BLOCKS.contains(&active_radarr_block);
    }

    false
  }

  fn draw(f: &mut Frame<'_>, app: &mut App<'_>, area: Rect) {
    if let Route::Radarr(active_radarr_block, context_option) = *app.get_current_route() {
      let draw_collection_details_popup =
        |f: &mut Frame<'_>, app: &mut App<'_>, popup_area: Rect| match context_option
          .unwrap_or(active_radarr_block)
        {
          ActiveRadarrBlock::ViewMovieOverview => {
            draw_small_popup_over(
              f,
              app,
              popup_area,
              draw_collection_details,
              draw_movie_overview,
            );
          }
          ActiveRadarrBlock::CollectionDetails => draw_collection_details(f, app, popup_area),
          _ => (),
        };

      draw_large_popup_over(
        f,
        app,
        area,
        draw_collections,
        draw_collection_details_popup,
      );
    }
  }
}

pub fn draw_collection_details(f: &mut Frame<'_>, app: &mut App<'_>, area: Rect) {
  let [description_area, table_area, help_footer_area] = Layout::vertical([
    Constraint::Percentage(25),
    Constraint::Percentage(70),
    Constraint::Percentage(5),
  ])
  .margin(1)
  .areas(area);
  let collection_selection =
    if let Some(filtered_collections) = app.data.radarr_data.filtered_collections.as_ref() {
      filtered_collections.current_selection()
    } else {
      app.data.radarr_data.collections.current_selection()
    };
  let quality_profile = app
    .data
    .radarr_data
    .quality_profile_map
    .get_by_left(&collection_selection.quality_profile_id)
    .unwrap()
    .to_owned();
  let current_selection = if app.data.radarr_data.collection_movies.items.is_empty() {
    CollectionMovie::default()
  } else {
    app
      .data
      .radarr_data
      .collection_movies
      .current_selection()
      .clone()
  };
  let help_text = Text::from(
    format!(
      "<↑↓> scroll table | {}",
      build_context_clue_string(&COLLECTION_DETAILS_CONTEXT_CLUES)
    )
    .help(),
  );
  let monitored = if collection_selection.monitored {
    "Yes"
  } else {
    "No"
  };
  let search_on_add = if collection_selection.search_on_add {
    "Yes"
  } else {
    "No"
  };
  let minimum_availability = collection_selection.minimum_availability.to_display_str();

  let collection_description = Text::from(vec![
    Line::from(vec![
      "Overview ".primary().bold(),
      collection_selection
        .overview
        .clone()
        .unwrap_or_default()
        .default(),
    ]),
    Line::from(vec![
      "Root Folder Path: ".primary().bold(),
      collection_selection
        .root_folder_path
        .clone()
        .unwrap_or_default()
        .default(),
    ]),
    Line::from(vec![
      "Quality Profile: ".primary().bold(),
      quality_profile.default(),
    ]),
    Line::from(vec![
      "Minimum Availability: ".primary().bold(),
      minimum_availability.default(),
    ]),
    Line::from(vec!["Monitored: ".primary().bold(), monitored.default()]),
    Line::from(vec![
      "Search on Add: ".primary().bold(),
      search_on_add.default(),
    ]),
  ]);

  let description_paragraph = Paragraph::new(collection_description)
    .block(borderless_block())
    .wrap(Wrap { trim: false });
  let help_paragraph = Paragraph::new(help_text)
    .block(borderless_block())
    .alignment(Alignment::Center);

  f.render_widget(title_block(&collection_selection.title.text), area);

  f.render_widget(description_paragraph, description_area);
  f.render_widget(help_paragraph, help_footer_area);

  draw_table(
    f,
    table_area,
    layout_block_top_border_with_title(title_style("Movies")),
    TableProps {
      content: Some(&mut app.data.radarr_data.collection_movies),
      wrapped_content: None,
      table_headers: vec![
        "✔",
        "Title",
        "Year",
        "Runtime",
        "IMDB Rating",
        "Rotten Tomatoes Rating",
        "Genres",
      ],
      constraints: vec![
        Constraint::Percentage(2),
        Constraint::Percentage(20),
        Constraint::Percentage(8),
        Constraint::Percentage(10),
        Constraint::Percentage(10),
        Constraint::Percentage(18),
        Constraint::Percentage(28),
      ],
      help: None,
    },
    |movie| {
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
        get_width_from_percentage(table_area, 20),
        current_selection == *movie,
        app.tick_count % app.ticks_until_scroll == 0,
      );
      let (hours, minutes) = convert_runtime(movie.runtime);
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
        format!("{imdb_rating:.1}")
      };
      let rotten_tomatoes_rating = if rotten_tomatoes_rating == 0 {
        String::new()
      } else {
        format!("{rotten_tomatoes_rating}%")
      };

      Row::new(vec![
        Cell::from(in_library),
        Cell::from(movie.title.to_string()),
        Cell::from(movie.year.to_string()),
        Cell::from(format!("{hours}h {minutes}m")),
        Cell::from(imdb_rating),
        Cell::from(rotten_tomatoes_rating),
        Cell::from(movie.genres.join(", ")),
      ])
      .primary()
    },
    app.is_loading,
    true,
  );
}

fn draw_movie_overview(f: &mut Frame<'_>, app: &mut App<'_>, area: Rect) {
  let title_block = title_block("Overview");
  f.render_widget(title_block, area);

  let [paragraph_area, help_area] =
    Layout::vertical([Constraint::Percentage(95), Constraint::Length(1)])
      .flex(Flex::SpaceBetween)
      .margin(1)
      .areas(area);
  let overview = Text::from(
    app
      .data
      .radarr_data
      .collection_movies
      .current_selection()
      .clone()
      .overview,
  )
  .default();
  let help_text = Text::from(build_context_clue_string(&BARE_POPUP_CONTEXT_CLUES).help());

  let paragraph = Paragraph::new(overview)
    .block(borderless_block())
    .wrap(Wrap { trim: false });
  let help_paragraph = Paragraph::new(help_text)
    .block(borderless_block())
    .alignment(Alignment::Center);

  f.render_widget(paragraph, paragraph_area);
  f.render_widget(help_paragraph, help_area);
}
