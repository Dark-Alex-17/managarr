use tui::backend::Backend;
use tui::layout::{Constraint, Rect};
use tui::text::Text;
use tui::widgets::{Block, Borders, Clear, Paragraph};
use tui::Frame;

use crate::app::radarr::ActiveRadarrBlock;
use crate::app::{App, Route};
use crate::logos::{
  BAZARR_LOGO, LIDARR_LOGO, PROWLARR_LOGO, RADARR_LOGO, READARR_LOGO, SONARR_LOGO,
};
use crate::ui::utils::{
  centered_rect, horizontal_chunks, horizontal_chunks_with_margin, style_secondary,
  style_system_function, vertical_chunks, vertical_chunks_with_margin,
};

mod radarr_ui;
mod utils;

static HIGHLIGHT_SYMBOL: &str = "=> ";

pub fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
  let main_chunks = vertical_chunks_with_margin(
    vec![Constraint::Length(16), Constraint::Length(0)],
    f.size(),
    1,
  );

  draw_context_row(f, app, main_chunks[0]);
  match app.get_current_route().clone() {
    Route::Radarr(active_radarr_block) => match active_radarr_block {
      ActiveRadarrBlock::Movies => radarr_ui::draw_radarr_ui(f, app, main_chunks[1]),
      ActiveRadarrBlock::MovieDetails => draw_small_popup_over(
        f,
        app,
        main_chunks[1],
        radarr_ui::draw_radarr_ui,
        radarr_ui::draw_movie_details,
      ),
      _ => (),
    },
  }
}

pub fn draw_popup_over<B: Backend>(
  f: &mut Frame<'_, B>,
  app: &mut App,
  area: Rect,
  background_fn: fn(&mut Frame<'_, B>, &mut App, Rect),
  popup_fn: fn(&mut Frame<'_, B>, &App, Rect),
  percent_x: u16,
  percent_y: u16,
) {
  background_fn(f, app, area);

  let popup_area = centered_rect(percent_x, percent_y, f.size());
  f.render_widget(Clear, popup_area);
  popup_fn(f, app, popup_area);
}

pub fn draw_small_popup_over<B: Backend>(
  f: &mut Frame<'_, B>,
  app: &mut App,
  area: Rect,
  background_fn: fn(&mut Frame<'_, B>, &mut App, Rect),
  popup_fn: fn(&mut Frame<'_, B>, &App, Rect),
) {
  draw_popup_over(f, app, area, background_fn, popup_fn, 40, 40);
}

pub fn draw_medium_popup_over<B: Backend>(
  f: &mut Frame<'_, B>,
  app: &mut App,
  area: Rect,
  background_fn: fn(&mut Frame<'_, B>, &mut App, Rect),
  popup_fn: fn(&mut Frame<'_, B>, &App, Rect),
) {
  draw_popup_over(f, app, area, background_fn, popup_fn, 60, 60);
}

pub fn draw_large_popup_over<B: Backend>(
  f: &mut Frame<'_, B>,
  app: &mut App,
  area: Rect,
  background_fn: fn(&mut Frame<'_, B>, &mut App, Rect),
  popup_fn: fn(&mut Frame<'_, B>, &App, Rect),
) {
  draw_popup_over(f, app, area, background_fn, popup_fn, 75, 75);
}

fn draw_context_row<B: Backend>(f: &mut Frame<'_, B>, app: &App, area: Rect) {
  match app.get_current_route().clone() {
    Route::Radarr(_) => radarr_ui::draw_radarr_context_row(f, app, area),
  }
}

pub fn loading<B: Backend>(f: &mut Frame<'_, B>, block: Block<'_>, area: Rect, is_loading: bool) {
  if is_loading {
    let text = "\n\n Loading ...\n\n".to_owned();
    let mut text = Text::from(text);
    text.patch_style(style_system_function());

    let paragraph = Paragraph::new(text)
      .style(style_system_function())
      .block(block);
    f.render_widget(paragraph, area);
  } else {
    f.render_widget(block, area)
  }
}
