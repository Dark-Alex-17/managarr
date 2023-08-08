use std::iter;
use std::ops::Sub;

use chrono::{Duration, Utc};
use tui::backend::Backend;
use tui::layout::{Alignment, Constraint, Rect};
use tui::style::Style;
use tui::text::{Span, Spans, Text};
use tui::widgets::{Block, Borders, Cell, Paragraph, Row, Table, Tabs, Wrap};
use tui::Frame;

use crate::app::radarr::{ActiveRadarrBlock, RadarrData};
use crate::app::{App, Route};
use crate::logos::RADARR_LOGO;
use crate::network::radarr_network::{DownloadRecord, Movie};
use crate::ui::utils::{
  horizontal_chunks_with_margin, line_gague, style_default, style_failure, style_highlight,
  style_secondary, style_success, style_warning, title_block, vertical_chunks_with_margin,
};
use crate::ui::{draw_small_popup_over, loading, HIGHLIGHT_SYMBOL};
use crate::utils::{convert_runtime, convert_to_gb};

pub(super) fn draw_radarr_ui<B: Backend>(f: &mut Frame<'_, B>, app: &mut App, area: Rect) {
  let chunks =
    vertical_chunks_with_margin(vec![Constraint::Length(2), Constraint::Min(0)], area, 1);
  let block = title_block(" Movies");

  let titles = app
    .data
    .radarr_data
    .main_tabs
    .tabs
    .iter()
    .map(|tab_route| Spans::from(Span::styled(&tab_route.title, style_default())))
    .collect();
  let tabs = Tabs::new(titles)
    .block(block)
    .highlight_style(style_secondary())
    .select(app.data.radarr_data.main_tabs.index);

  f.render_widget(tabs, area);

  if let Route::Radarr(active_radarr_block) = app.get_current_route() {
    match active_radarr_block {
      ActiveRadarrBlock::Movies => draw_radarr_library(f, app, chunks[1]),
      ActiveRadarrBlock::Downloads => draw_downloads(f, app, chunks[1]),
      ActiveRadarrBlock::MovieDetails => {
        draw_small_popup_over(f, app, chunks[1], draw_radarr_library, draw_movie_details)
      }
      _ => (),
    }
  }
}

pub(super) fn draw_radarr_context_row<B: Backend>(f: &mut Frame<'_, B>, app: &App, area: Rect) {
  let chunks = horizontal_chunks_with_margin(
    vec![
      Constraint::Ratio(1, 3),
      Constraint::Ratio(1, 3),
      Constraint::Ratio(1, 3),
    ],
    area,
    1,
  );

  draw_stats_context(f, app, chunks[0]);
  f.render_widget(Block::default().borders(Borders::ALL), chunks[1]);
  draw_downloads_context(f, app, chunks[2]);
}

fn draw_radarr_library<B: Backend>(f: &mut Frame<'_, B>, app: &mut App, area: Rect) {
  let block = Block::default().borders(Borders::TOP);
  let movies_vec = &app.data.radarr_data.movies.items;

  if !movies_vec.is_empty() {
    let rows = movies_vec.iter().map(|movie| {
      let (hours, minutes) = convert_runtime(movie.runtime.as_u64().unwrap());
      let file_size: f64 = convert_to_gb(movie.size_on_disk.as_u64().unwrap());

      Row::new(vec![
        Cell::from(movie.title.to_owned()),
        Cell::from(movie.year.to_string()),
        Cell::from(format!("{}h {}m", hours, minutes)),
        Cell::from(format!("{:.2} GB", file_size)),
        Cell::from(
          app
            .data
            .radarr_data
            .quality_profile_map
            .get(&movie.quality_profile_id.as_u64().unwrap())
            .unwrap()
            .to_owned(),
        ),
      ])
      .style(determine_row_style(app, movie))
    });

    let header_row = Row::new(vec!["Title", "Year", "Runtime", "Size", "Quality Profile"])
      .style(style_default())
      .bottom_margin(0);

    let constraints = vec![
      Constraint::Percentage(20),
      Constraint::Percentage(20),
      Constraint::Percentage(20),
      Constraint::Percentage(20),
      Constraint::Percentage(20),
    ];

    let table = Table::new(rows)
      .header(header_row)
      .block(block)
      .highlight_style(style_highlight())
      .highlight_symbol(HIGHLIGHT_SYMBOL)
      .widths(&constraints);

    f.render_stateful_widget(table, area, &mut app.data.radarr_data.movies.state);
  } else {
    loading(f, block, area, app.is_loading);
  }
}

fn draw_downloads_context<B: Backend>(f: &mut Frame<'_, B>, app: &App, area: Rect) {
  let block = title_block("Downloads");
  let downloads_vec = &app.data.radarr_data.downloads.items;

  if !downloads_vec.is_empty() {
    f.render_widget(block, area);

    let constraints = iter::repeat(Constraint::Min(2))
      .take(downloads_vec.len())
      .collect::<Vec<Constraint>>();

    let chunks = vertical_chunks_with_margin(constraints, area, 1);

    for i in 0..downloads_vec.len() {
      let DownloadRecord {
        title,
        sizeleft,
        size,
        ..
      } = &downloads_vec[i];
      let percent = 1f64 - (sizeleft.as_f64().unwrap() / size.as_f64().unwrap());
      let download_gague = line_gague(title, percent);

      f.render_widget(download_gague, chunks[i]);
    }
  } else {
    loading(f, block, area, app.is_loading);
  }
}

fn draw_downloads<B: Backend>(f: &mut Frame<'_, B>, app: &mut App, area: Rect) {
  let block = Block::default().borders(Borders::TOP);
  let downloads_vec = &app.data.radarr_data.downloads.items;

  if !downloads_vec.is_empty() {
    let rows = downloads_vec.iter().map(|download_record| {
      let DownloadRecord {
        title,
        size,
        sizeleft,
        download_client,
        indexer,
        output_path,
        ..
      } = download_record;
      let percent = 1f64 - (sizeleft.as_f64().unwrap() / size.as_f64().unwrap());
      let file_size: f64 = convert_to_gb(size.as_u64().unwrap());

      Row::new(vec![
        Cell::from(title.to_owned()),
        Cell::from(format!("{:.0}%", percent * 100.0)),
        Cell::from(format!("{:.2} GB", file_size)),
        Cell::from(output_path.to_owned()),
        Cell::from(indexer.to_owned()),
        Cell::from(download_client.to_owned()),
      ])
      .style(style_success())
    });

    let header_row = Row::new(vec![
      "Title",
      "Percent Complete",
      "Size",
      "Output Path",
      "Indexer",
      "Download Client",
    ])
    .style(style_default())
    .bottom_margin(0);

    let constraints = vec![
      Constraint::Percentage(30),
      Constraint::Percentage(11),
      Constraint::Percentage(11),
      Constraint::Percentage(18),
      Constraint::Percentage(17),
      Constraint::Percentage(13),
    ];

    let table = Table::new(rows)
      .header(header_row)
      .block(block)
      .highlight_style(style_highlight())
      .highlight_symbol(HIGHLIGHT_SYMBOL)
      .widths(&constraints);

    f.render_stateful_widget(table, area, &mut app.data.radarr_data.downloads.state);
  } else {
    loading(f, block, area, app.is_loading);
  }
}

fn draw_movie_details<B: Backend>(f: &mut Frame<'_, B>, app: &App, area: Rect) {
  let block = title_block("Movie Details");
  let movie_details = app.data.radarr_data.movie_details.get_text();

  if !movie_details.is_empty() {
    let download_status = app
      .data
      .radarr_data
      .movie_details
      .items
      .iter()
      .find(|&line| line.starts_with("Status: "))
      .unwrap()
      .split(": ")
      .collect::<Vec<&str>>()[1];
    let mut text = Text::from(movie_details);
    text.patch_style(determine_style_from_download_status(download_status));

    let paragraph = Paragraph::new(text)
      .block(block)
      .wrap(Wrap { trim: false })
      .scroll((app.data.radarr_data.movie_details.offset, 0));

    f.render_widget(paragraph, area);
  } else {
    loading(f, block, area, app.is_loading);
  }
}

fn draw_stats_context<B: Backend>(f: &mut Frame<'_, B>, app: &App, area: Rect) {
  let block = title_block("Stats");

  if !app.data.radarr_data.version.is_empty() {
    let RadarrData {
      free_space,
      total_space,
      start_time,
      ..
    } = app.data.radarr_data;
    let ratio = if total_space == 0 {
      0f64
    } else {
      1f64 - (free_space as f64 / total_space as f64)
    };

    f.render_widget(block, area);

    let chunks = vertical_chunks_with_margin(
      vec![
        Constraint::Percentage(60),
        Constraint::Length(1),
        Constraint::Length(1),
        Constraint::Min(2),
      ],
      area,
      1,
    );

    let version_paragraph = Paragraph::new(Text::from(format!(
      "Radarr Version:  {}",
      app.data.radarr_data.version
    )))
    .block(Block::default());

    let uptime = Utc::now().sub(start_time);
    let days = uptime.num_days();
    let day_difference = uptime.sub(Duration::days(days));
    let hours = day_difference.num_hours();
    let hour_difference = day_difference.sub(Duration::hours(hours));
    let minutes = hour_difference.num_minutes();
    let seconds = hour_difference
      .sub(Duration::minutes(minutes))
      .num_seconds();

    let uptime_paragraph = Paragraph::new(Text::from(format!(
      "Uptime: {}d {:0width$}:{:0width$}:{:0width$}",
      days,
      hours,
      minutes,
      seconds,
      width = 2
    )))
    .block(Block::default());

    let space_gauge = line_gague("Storage:", ratio);
    let logo = Paragraph::new(Text::from(RADARR_LOGO))
      .block(Block::default())
      .alignment(Alignment::Center);

    f.render_widget(logo, chunks[0]);
    f.render_widget(version_paragraph, chunks[1]);
    f.render_widget(uptime_paragraph, chunks[2]);
    f.render_widget(space_gauge, chunks[3]);
  } else {
    loading(f, block, area, app.is_loading);
  }
}

fn determine_row_style(app: &App, movie: &Movie) -> Style {
  let downloads_vec = &app.data.radarr_data.downloads.items;

  if !movie.has_file {
    if let Some(download) = downloads_vec
      .iter()
      .find(|&download| download.movie_id == movie.id)
    {
      if download.status == "downloading" {
        return style_warning();
      }
    }

    return style_failure();
  }

  style_success()
}

fn determine_style_from_download_status(download_status: &str) -> Style {
  match download_status {
    "Downloaded" => style_success(),
    "Downloading" => style_warning(),
    "Missing" => style_failure(),
    _ => style_success(),
  }
}
