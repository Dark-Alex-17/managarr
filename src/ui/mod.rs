use std::ops::Sub;
use chrono::{Duration, Utc};
use tui::backend::Backend;
use tui::layout::{Alignment, Constraint, Rect};
use tui::style::Color::Cyan;
use tui::style::Style;
use tui::text::{Spans, Text};
use tui::widgets::{Block, Borders, LineGauge, Paragraph};
use tui::{symbols, Frame};

use crate::app::radarr::RadarrData;
use crate::app::App;
use crate::logos::{
  BAZARR_LOGO, LIDARR_LOGO, PROWLARR_LOGO, RADARR_LOGO, READARR_LOGO, SONARR_LOGO,
};
use crate::ui::utils::{
  horizontal_chunks, horizontal_chunks_with_margin, vertical_chunks, vertical_chunks_with_margin,
};

mod utils;

pub fn ui<B: Backend>(f: &mut Frame<B>, app: &App) {
  let main_chunks = vertical_chunks(
    vec![Constraint::Length(20), Constraint::Length(0)],
    f.size(),
  );

  draw_context_row(f, app, main_chunks[0]);
  f.render_widget(Block::default().borders(Borders::ALL), main_chunks[1]);
}

fn draw_context_row<B: Backend>(f: &mut Frame<'_, B>, app: &App, area: Rect) {
  let chunks = horizontal_chunks(
    vec![
      Constraint::Percentage(23),
      Constraint::Percentage(23),
      Constraint::Percentage(23),
      Constraint::Percentage(23),
      Constraint::Length(20),
    ],
    area,
  );

  draw_stats(f, app, chunks[0]);
  f.render_widget(Block::default().borders(Borders::ALL), chunks[1]);
  f.render_widget(Block::default().borders(Borders::ALL), chunks[2]);
  f.render_widget(Block::default().borders(Borders::ALL), chunks[3]);

  draw_logo(f, chunks[4]);
}

fn draw_stats<B: Backend>(f: &mut Frame<'_, B>, app: &App, area: Rect) {
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

  let base_block = Block::default().title("Stats").borders(Borders::ALL);
  f.render_widget(base_block, area);

  let chunks =
    vertical_chunks_with_margin(vec![
      Constraint::Length(1),
      Constraint::Length(1),
      Constraint::Min(2)], area, 1);

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
  let seconds = hour_difference.sub(Duration::minutes(minutes)).num_seconds();

  let uptime_paragraph = Paragraph::new(Text::from(format!(
    "Uptime: {}d {}:{}:{}",
    days, hours, minutes, seconds
  ))).block(Block::default());

  let space_gauge = LineGauge::default()
      .block(Block::default().title("Storage:"))
      .gauge_style(Style::default().fg(Cyan))
      .line_set(symbols::line::THICK)
      .ratio(ratio)
      .label(Spans::from(format!("{:.0}%", ratio * 100.0)));

  f.render_widget(version_paragraph, chunks[0]);
  f.render_widget(uptime_paragraph, chunks[1]);
  f.render_widget(space_gauge, chunks[2]);
}

fn draw_logo<B: Backend>(f: &mut Frame<'_, B>, area: Rect) {
  let chunks = vertical_chunks(
    vec![Constraint::Percentage(60), Constraint::Percentage(40)],
    area,
  );
  let logo = Paragraph::new(Text::from(RADARR_LOGO))
    .block(Block::default())
    .alignment(Alignment::Center);

  f.render_widget(logo, chunks[0]);
}
