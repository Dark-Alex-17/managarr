use tui::backend::Backend;
use tui::layout::{Alignment, Constraint, Rect};
use tui::text::Text;
use tui::widgets::{Cell, Paragraph, Row, Wrap};
use tui::Frame;

use crate::app::radarr::ActiveRadarrBlock;
use crate::app::App;
use crate::models::radarr_models::CollectionMovie;
use crate::models::Route;
use crate::ui::radarr_ui::collections_ui::draw_collections;
use crate::ui::utils::{
  borderless_block, get_width_from_percentage, layout_block_top_border_with_title,
  spans_info_primary, style_default, style_help, style_primary, title_block, title_style,
  vertical_chunks_with_margin,
};
use crate::ui::{draw_large_popup_over, draw_small_popup_over, draw_table, DrawUi, TableProps};
use crate::utils::convert_runtime;

pub(super) struct CollectionDetailsUi {}

impl DrawUi for CollectionDetailsUi {
  fn draw<B: Backend>(f: &mut Frame<'_, B>, app: &mut App<'_>, content_rect: Rect) {
    if let Route::Radarr(active_radarr_block, context_option) = *app.get_current_route() {
      let draw_collection_details_popup =
        |f: &mut Frame<'_, B>, app: &mut App<'_>, popup_area: Rect| match context_option
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
        content_rect,
        draw_collections,
        draw_collection_details_popup,
      );
    }
  }
}

pub(super) fn draw_collection_details<B: Backend>(
  f: &mut Frame<'_, B>,
  app: &mut App<'_>,
  content_area: Rect,
) {
  let chunks = vertical_chunks_with_margin(
    vec![
      Constraint::Percentage(25),
      Constraint::Percentage(70),
      Constraint::Percentage(5),
    ],
    content_area,
    1,
  );
  let collection_selection = if !app.data.radarr_data.filtered_collections.items.is_empty() {
    app
      .data
      .radarr_data
      .filtered_collections
      .current_selection()
  } else {
    app.data.radarr_data.collections.current_selection()
  };
  let quality_profile = app
    .data
    .radarr_data
    .quality_profile_map
    .get_by_left(&collection_selection.quality_profile_id.as_u64().unwrap())
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
  let mut help_text =
    Text::from("<↑↓> scroll table | <enter> show overview/add movie | <esc> close");
  help_text.patch_style(style_help());
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
    spans_info_primary(
      "Overview: ".to_owned(),
      collection_selection.overview.clone().unwrap_or_default(),
    ),
    spans_info_primary(
      "Root Folder Path: ".to_owned(),
      collection_selection
        .root_folder_path
        .clone()
        .unwrap_or_default(),
    ),
    spans_info_primary("Quality Profile: ".to_owned(), quality_profile),
    spans_info_primary(
      "Minimum Availability: ".to_owned(),
      minimum_availability.to_owned(),
    ),
    spans_info_primary("Monitored: ".to_owned(), monitored.to_owned()),
    spans_info_primary("Search on Add: ".to_owned(), search_on_add.to_owned()),
  ]);

  let description_paragraph = Paragraph::new(collection_description)
    .block(borderless_block())
    .wrap(Wrap { trim: false });
  let help_paragraph = Paragraph::new(help_text)
    .block(borderless_block())
    .alignment(Alignment::Center);

  f.render_widget(title_block(&collection_selection.title.text), content_area);

  f.render_widget(description_paragraph, chunks[0]);
  f.render_widget(help_paragraph, chunks[2]);

  draw_table(
    f,
    chunks[1],
    layout_block_top_border_with_title(title_style("Movies")),
    TableProps {
      content: &mut app.data.radarr_data.collection_movies,
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
        get_width_from_percentage(chunks[1], 20),
        current_selection == *movie,
        app.tick_count % app.ticks_until_scroll == 0,
      );
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
  );
}

fn draw_movie_overview<B: Backend>(f: &mut Frame<'_, B>, app: &mut App<'_>, content_area: Rect) {
  let title_block = title_block("Overview");
  f.render_widget(title_block, content_area);

  let chunks = vertical_chunks_with_margin(
    vec![Constraint::Percentage(95), Constraint::Percentage(5)],
    content_area,
    1,
  );
  let mut overview = Text::from(
    app
      .data
      .radarr_data
      .collection_movies
      .current_selection()
      .clone()
      .overview,
  );
  overview.patch_style(style_default());
  let mut help_text = Text::from("<esc> close");
  help_text.patch_style(style_help());

  let paragraph = Paragraph::new(overview)
    .block(borderless_block())
    .wrap(Wrap { trim: false });
  let help_paragraph = Paragraph::new(help_text)
    .block(borderless_block())
    .alignment(Alignment::Center);

  f.render_widget(paragraph, chunks[0]);
  f.render_widget(help_paragraph, chunks[1]);
}
