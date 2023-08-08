use tui::backend::Backend;
use tui::layout::{Alignment, Constraint, Rect};
use tui::style::Modifier;
use tui::text::{Span, Spans, Text};
use tui::widgets::Paragraph;
use tui::widgets::Row;
use tui::widgets::Table;
use tui::widgets::Tabs;
use tui::widgets::{Block, Borders, Wrap};
use tui::widgets::{Clear, List, ListItem};
use tui::Frame;

use crate::app::App;
use crate::models::{Route, StatefulList, StatefulTable, TabState};
use crate::ui::utils::{
  borderless_block, centered_rect, horizontal_chunks, horizontal_chunks_with_margin, layout_block,
  layout_block_top_border, layout_button_paragraph, layout_button_paragraph_borderless,
  layout_paragraph_borderless, logo_block, style_button_highlight, style_default_bold,
  style_failure, style_help, style_highlight, style_primary, style_secondary,
  style_system_function, title_block, title_block_centered, vertical_chunks,
  vertical_chunks_with_margin,
};

mod radarr_ui;
mod utils;

static HIGHLIGHT_SYMBOL: &str = "=> ";

pub fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
  let main_chunks = if !app.error.text.is_empty() {
    let chunks = vertical_chunks_with_margin(
      vec![
        Constraint::Length(3),
        Constraint::Length(3),
        Constraint::Length(16),
        Constraint::Length(0),
      ],
      f.size(),
      1,
    );

    draw_error(f, app, chunks[1]);

    vec![chunks[0], chunks[2], chunks[3]]
  } else {
    vertical_chunks_with_margin(
      vec![
        Constraint::Length(3),
        Constraint::Length(16),
        Constraint::Length(0),
      ],
      f.size(),
      1,
    )
  };

  draw_header_row(f, app, main_chunks[0]);
  draw_context_row(f, app, main_chunks[1]);
  if let Route::Radarr(_) = app.get_current_route() {
    radarr_ui::draw_radarr_ui(f, app, main_chunks[2])
  }
}

fn draw_header_row<B: Backend>(f: &mut Frame<'_, B>, app: &mut App, area: Rect) {
  let chunks =
    horizontal_chunks_with_margin(vec![Constraint::Length(75), Constraint::Min(0)], area, 1);
  let help_text = Text::from(app.server_tabs.get_active_tab_help());

  let titles = app
    .server_tabs
    .tabs
    .iter()
    .map(|tab| Spans::from(Span::styled(&tab.title, style_default_bold())))
    .collect();
  let tabs = Tabs::new(titles)
    .block(logo_block())
    .highlight_style(style_secondary())
    .select(app.server_tabs.index);
  let help = Paragraph::new(help_text)
    .block(borderless_block())
    .style(style_help())
    .alignment(Alignment::Right);

  f.render_widget(tabs, area);
  f.render_widget(help, chunks[1]);
}

fn draw_error<B: Backend>(f: &mut Frame<'_, B>, app: &mut App, area: Rect) {
  let block = borderless_block()
    .title("Error | <esc> to close")
    .style(style_failure())
    .borders(Borders::ALL);

  if app.error.text.len() > area.width as usize {
    app.error.scroll_text();
  }

  let mut text = Text::from(app.error.to_string());
  text.patch_style(style_failure());

  let paragraph = Paragraph::new(text)
    .block(block)
    .wrap(Wrap { trim: true })
    .style(style_primary());

  f.render_widget(paragraph, area);
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

pub fn draw_prompt_popup_over<B: Backend>(
  f: &mut Frame<'_, B>,
  app: &mut App,
  area: Rect,
  background_fn: fn(&mut Frame<'_, B>, &mut App, Rect),
  popup_fn: fn(&mut Frame<'_, B>, &mut App, Rect),
) {
  draw_popup_over(f, app, area, background_fn, popup_fn, 30, 30);
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

pub fn draw_drop_down_popup<B: Backend>(
  f: &mut Frame<'_, B>,
  app: &mut App,
  area: Rect,
  background_fn: fn(&mut Frame<'_, B>, &mut App, Rect),
  drop_down_fn: fn(&mut Frame<'_, B>, &mut App, Rect),
) {
  draw_popup_over(f, app, area, background_fn, drop_down_fn, 20, 30);
}

fn draw_context_row<B: Backend>(f: &mut Frame<'_, B>, app: &App, area: Rect) {
  if let Route::Radarr(_) = app.get_current_route() {
    radarr_ui::draw_radarr_context_row(f, app, area)
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
  let horizontal_chunks = horizontal_chunks_with_margin(
    vec![Constraint::Percentage(10), Constraint::Min(0)],
    area,
    1,
  );
  let block = title_block(title);
  let mut help_text = Text::from(tab_state.get_active_tab_help());
  help_text.patch_style(style_help());

  let titles = tab_state
    .tabs
    .iter()
    .map(|tab_route| Spans::from(Span::styled(&tab_route.title, style_default_bold())))
    .collect();
  let tabs = Tabs::new(titles)
    .block(block)
    .highlight_style(style_secondary())
    .select(tab_state.index);
  let help = Paragraph::new(help_text)
    .block(borderless_block())
    .alignment(Alignment::Right);

  f.render_widget(tabs, area);
  f.render_widget(help, horizontal_chunks[1]);

  (chunks[1], layout_block_top_border())
}

pub struct TableProps<'a, T> {
  pub content: &'a mut StatefulTable<T>,
  pub table_headers: Vec<&'a str>,
  pub constraints: Vec<Constraint>,
  pub help: Option<String>,
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
    help,
  } = table_props;

  let content_area = if let Some(help_string) = help {
    let chunks = vertical_chunks(
      vec![Constraint::Min(0), Constraint::Length(3)],
      content_area,
    );
    let mut help_text = Text::from(format!(" {}", help_string));
    help_text.patch_style(style_help());
    let help_paragraph = Paragraph::new(help_text)
      .block(layout_block_top_border())
      .alignment(Alignment::Left);

    f.render_widget(help_paragraph, chunks[1]);

    chunks[0]
  } else {
    content_area
  };

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

pub fn draw_error_popup_over<B: Backend>(
  f: &mut Frame<'_, B>,
  app: &mut App,
  area: Rect,
  message: &str,
  background_fn: fn(&mut Frame<'_, B>, &mut App, Rect),
) {
  background_fn(f, app, area);
  draw_error_popup(f, message);
}

pub fn draw_error_popup<B: Backend>(f: &mut Frame<'_, B>, message: &str) {
  let prompt_area = centered_rect(25, 8, f.size());
  f.render_widget(Clear, prompt_area);

  let error_message = Paragraph::new(Text::from(message))
    .block(title_block_centered("Error").style(style_failure()))
    .style(style_failure().add_modifier(Modifier::BOLD))
    .wrap(Wrap { trim: false })
    .alignment(Alignment::Center);

  f.render_widget(error_message, prompt_area);
}

pub fn draw_prompt_box<B: Backend>(
  f: &mut Frame<'_, B>,
  prompt_area: Rect,
  title: &str,
  prompt: &str,
  yes_no_value: &bool,
) {
  draw_prompt_box_with_content(f, prompt_area, title, prompt, None, yes_no_value);
}

pub fn draw_prompt_box_with_content<B: Backend>(
  f: &mut Frame<'_, B>,
  prompt_area: Rect,
  title: &str,
  prompt: &str,
  content: Option<Paragraph>,
  yes_no_value: &bool,
) {
  f.render_widget(title_block_centered(title), prompt_area);

  let chunks = if let Some(content_paragraph) = content {
    let vertical_chunks = vertical_chunks_with_margin(
      vec![
        Constraint::Length(4),
        Constraint::Length(7),
        Constraint::Min(0),
        Constraint::Length(3),
      ],
      prompt_area,
      1,
    );

    f.render_widget(content_paragraph, vertical_chunks[1]);

    vec![vertical_chunks[0], vertical_chunks[2], vertical_chunks[3]]
  } else {
    vertical_chunks_with_margin(
      vec![
        Constraint::Percentage(72),
        Constraint::Min(0),
        Constraint::Length(3),
      ],
      prompt_area,
      1,
    )
  };

  let prompt_paragraph = layout_paragraph_borderless(prompt);
  f.render_widget(prompt_paragraph, chunks[0]);

  let horizontal_chunks = horizontal_chunks(
    vec![Constraint::Percentage(50), Constraint::Percentage(50)],
    chunks[2],
  );

  draw_button(f, horizontal_chunks[0], "Yes", *yes_no_value);
  draw_button(f, horizontal_chunks[1], "No", !*yes_no_value);
}

pub fn draw_button<B: Backend>(f: &mut Frame<'_, B>, area: Rect, label: &str, is_selected: bool) {
  let label_paragraph = layout_button_paragraph(is_selected, label, Alignment::Center);

  f.render_widget(label_paragraph, area);
}

pub fn draw_button_with_icon<B: Backend>(
  f: &mut Frame<'_, B>,
  area: Rect,
  label: &str,
  icon: &str,
  is_selected: bool,
) {
  let label_paragraph = layout_button_paragraph_borderless(is_selected, label, Alignment::Left);
  let icon_paragraph = layout_button_paragraph_borderless(is_selected, icon, Alignment::Right);

  let horizontal_chunks = horizontal_chunks_with_margin(
    vec![Constraint::Percentage(50), Constraint::Percentage(50)],
    area,
    1,
  );

  f.render_widget(
    layout_block().style(style_button_highlight(is_selected)),
    area,
  );
  f.render_widget(label_paragraph, horizontal_chunks[0]);
  f.render_widget(icon_paragraph, horizontal_chunks[1]);
}

pub fn draw_drop_down_menu_button<B: Backend>(
  f: &mut Frame<'_, B>,
  area: Rect,
  description: &str,
  selection: &str,
  is_selected: bool,
) {
  let horizontal_chunks = horizontal_chunks(
    vec![Constraint::Percentage(50), Constraint::Percentage(50)],
    area,
  );

  let description_paragraph = Paragraph::new(Text::from(format!("\n{}: ", description)))
    .block(borderless_block())
    .alignment(Alignment::Right)
    .style(style_primary());

  f.render_widget(description_paragraph, horizontal_chunks[0]);

  draw_button_with_icon(f, horizontal_chunks[1], selection, "▼", is_selected);
}

pub fn draw_drop_down_list<'a, B: Backend, T>(
  f: &mut Frame<'_, B>,
  area: Rect,
  content: &'a mut StatefulList<T>,
  item_mapper: impl Fn(&T) -> ListItem<'a>,
) {
  let items: Vec<ListItem<'_>> = content.items.iter().map(item_mapper).collect();
  let list = List::new(items)
    .block(layout_block())
    .highlight_style(style_highlight());

  f.render_stateful_widget(list, area, &mut content.state);
}
