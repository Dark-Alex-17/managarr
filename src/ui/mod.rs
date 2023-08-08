use tui::backend::Backend;
use tui::layout::{Constraint, Rect};
use tui::text::{Span, Spans, Text};
use tui::widgets::Block;
use tui::widgets::Clear;
use tui::widgets::Paragraph;
use tui::widgets::Row;
use tui::widgets::Table;
use tui::widgets::Tabs;
use tui::Frame;

use crate::app::models::{StatefulTable, TabState};
use crate::app::{App, Route};
use crate::logos::{
  BAZARR_LOGO, LIDARR_LOGO, PROWLARR_LOGO, RADARR_LOGO, READARR_LOGO, SONARR_LOGO,
};
use crate::ui::utils::{
  centered_rect, layout_block_top_border, style_default_bold, style_highlight, style_secondary,
  style_system_function, title_block, vertical_chunks_with_margin,
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
  match app.get_current_route() {
    Route::Radarr(_) => radarr_ui::draw_radarr_ui(f, app, main_chunks[1]),
  }
}

pub fn draw_popup_over<B: Backend>(
  f: &mut Frame<'_, B>,
  app: &mut App,
  area: Rect,
  background_fn: fn(&mut Frame<'_, B>, &mut App, Rect),
  popup_fn: fn(&mut Frame<'_, B>, &mut App, Rect),
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
  popup_fn: fn(&mut Frame<'_, B>, &mut App, Rect),
) {
  draw_popup_over(f, app, area, background_fn, popup_fn, 40, 40);
}

pub fn draw_medium_popup_over<B: Backend>(
  f: &mut Frame<'_, B>,
  app: &mut App,
  area: Rect,
  background_fn: fn(&mut Frame<'_, B>, &mut App, Rect),
  popup_fn: fn(&mut Frame<'_, B>, &mut App, Rect),
) {
  draw_popup_over(f, app, area, background_fn, popup_fn, 60, 60);
}

pub fn draw_large_popup_over<B: Backend>(
  f: &mut Frame<'_, B>,
  app: &mut App,
  area: Rect,
  background_fn: fn(&mut Frame<'_, B>, &mut App, Rect),
  popup_fn: fn(&mut Frame<'_, B>, &mut App, Rect),
) {
  draw_popup_over(f, app, area, background_fn, popup_fn, 75, 75);
}

fn draw_context_row<B: Backend>(f: &mut Frame<'_, B>, app: &App, area: Rect) {
  match app.get_current_route() {
    Route::Radarr(_) => radarr_ui::draw_radarr_context_row(f, app, area),
  }
}

fn draw_tabs<'a, B: Backend>(
  f: &mut Frame<'_, B>,
  area: Rect,
  title: &str,
  tab_state: &TabState,
) -> (Rect, Block<'a>) {
  let chunks =
    vertical_chunks_with_margin(vec![Constraint::Length(2), Constraint::Min(0)], area, 1);
  let block = title_block(title);

  let titles = tab_state
    .tabs
    .iter()
    .map(|tab_route| Spans::from(Span::styled(&tab_route.title, style_default_bold())))
    .collect();
  let tabs = Tabs::new(titles)
    .block(block)
    .highlight_style(style_secondary())
    .select(tab_state.index);

  f.render_widget(tabs, area);

  (chunks[1], layout_block_top_border())
}

pub struct TableProps<'a, T> {
  pub content: &'a mut StatefulTable<T>,
  pub table_headers: Vec<&'a str>,
  pub constraints: Vec<Constraint>,
}

fn draw_table<'a, B, T, F>(
  f: &mut Frame<'_, B>,
  content_area: Rect,
  block: Block,
  table_props: TableProps<'a, T>,
  row_mapper: F,
  is_loading: bool,
) where
  B: Backend,
  F: Fn(&T) -> Row<'a>,
{
  let TableProps {
    content,
    table_headers,
    constraints,
  } = table_props;

  if !content.items.is_empty() {
    let rows = content.items.iter().map(row_mapper);

    let headers = Row::new(table_headers)
      .style(style_default_bold())
      .bottom_margin(0);

    let table = Table::new(rows)
      .header(headers)
      .block(block)
      .highlight_style(style_highlight())
      .highlight_symbol(HIGHLIGHT_SYMBOL)
      .widths(&constraints);

    f.render_stateful_widget(table, content_area, &mut content.state);
  } else {
    loading(f, block, content_area, is_loading);
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
