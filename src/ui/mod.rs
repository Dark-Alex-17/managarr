use tui::backend::Backend;
use tui::Frame;
use tui::layout::{Constraint, Rect};
use tui::widgets::{Block, Borders};

use crate::app::App;
use crate::logos::{
  BAZARR_LOGO, LIDARR_LOGO, PROWLARR_LOGO, RADARR_LOGO, READARR_LOGO, SONARR_LOGO,
};
use crate::ui::utils::{
  horizontal_chunks, horizontal_chunks_with_margin, vertical_chunks, vertical_chunks_with_margin,
};

mod utils;
mod radarr_ui;

static HIGHLIGHT_SYMBOL: &str = "=> ";

pub fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
  let main_chunks = vertical_chunks_with_margin(
    vec![Constraint::Length(20), Constraint::Length(0)],
    f.size(),
    1
  );

  draw_context_row(f, app, main_chunks[0]);
  radarr_ui::draw_radarr_ui(f, app, main_chunks[1]);
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

  radarr_ui::draw_stats(f, app, chunks[0]);
  f.render_widget(Block::default().borders(Borders::ALL), chunks[1]);
  f.render_widget(Block::default().borders(Borders::ALL), chunks[2]);
  f.render_widget(Block::default().borders(Borders::ALL), chunks[3]);

  radarr_ui::draw_logo(f, chunks[4]);
}
