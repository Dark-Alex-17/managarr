use crate::ui::styles::ManagarrStyle;
use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Style, Stylize};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Block, BorderType, Borders, LineGauge, Paragraph, Wrap};
use ratatui::{symbols, Frame};
use std::rc::Rc;

pub const COLOR_TEAL: Color = Color::Rgb(35, 50, 55);

#[cfg(test)]
#[path = "utils_tests.rs"]
mod utils_tests;

pub fn horizontal_chunks(constraints: Vec<Constraint>, area: Rect) -> Rc<[Rect]> {
  layout_with_constraints(constraints)
    .direction(Direction::Horizontal)
    .split(area)
}

pub fn horizontal_chunks_with_margin(
  constraints: Vec<Constraint>,
  area: Rect,
  margin: u16,
) -> Rc<[Rect]> {
  layout_with_constraints(constraints)
    .direction(Direction::Horizontal)
    .margin(margin)
    .split(area)
}

pub fn vertical_chunks(constraints: Vec<Constraint>, area: Rect) -> Rc<[Rect]> {
  layout_with_constraints(constraints)
    .direction(Direction::Vertical)
    .split(area)
}

pub fn vertical_chunks_with_margin(
  constraints: Vec<Constraint>,
  area: Rect,
  margin: u16,
) -> Rc<[Rect]> {
  layout_with_constraints(constraints)
    .direction(Direction::Vertical)
    .margin(margin)
    .split(area)
}

fn layout_with_constraints(constraints: Vec<Constraint>) -> Layout {
  Layout::default().constraints(<Vec<Constraint> as AsRef<[Constraint]>>::as_ref(
    &constraints,
  ))
}

pub fn background_block<'a>() -> Block<'a> {
  Block::new().white().bg(COLOR_TEAL)
}

pub fn layout_block<'a>() -> Block<'a> {
  Block::new()
    .borders(Borders::ALL)
    .border_type(BorderType::Rounded)
}

pub fn layout_block_with_title(title_span: Span<'_>) -> Block<'_> {
  layout_block().title(title_span)
}

pub fn layout_block_top_border_with_title(title_span: Span<'_>) -> Block<'_> {
  layout_block_top_border().title(title_span)
}

pub fn layout_block_top_border<'a>() -> Block<'a> {
  Block::new().borders(Borders::TOP)
}

pub fn layout_block_bottom_border<'a>() -> Block<'a> {
  Block::new().borders(Borders::BOTTOM)
}

pub fn layout_button_paragraph(
  is_selected: bool,
  label: &str,
  alignment: Alignment,
) -> Paragraph<'_> {
  Paragraph::new(Text::from(label))
    .block(layout_block())
    .alignment(alignment)
    .style(style_block_highlight(is_selected))
}

pub fn layout_button_paragraph_borderless(
  is_selected: bool,
  label: &str,
  alignment: Alignment,
) -> Paragraph<'_> {
  Paragraph::new(Text::from(label))
    .block(borderless_block())
    .alignment(alignment)
    .style(style_block_highlight(is_selected))
}

pub fn layout_paragraph_borderless(string: &str) -> Paragraph<'_> {
  Paragraph::new(Text::from(string))
    .block(borderless_block())
    .primary()
    .bold()
    .wrap(Wrap { trim: false })
    .alignment(Alignment::Center)
}

pub fn borderless_block<'a>() -> Block<'a> {
  Block::new()
}

pub fn style_block_highlight(is_selected: bool) -> Style {
  if is_selected {
    Style::new().system_function().bold()
  } else {
    Style::new().default().bold()
  }
}

pub fn title_style(title: &str) -> Span<'_> {
  format!("  {title}  ").bold()
}

pub fn title_block(title: &str) -> Block<'_> {
  layout_block_with_title(title_style(title))
}

pub fn title_block_centered(title: &str) -> Block<'_> {
  title_block(title).title_alignment(Alignment::Center)
}

pub fn logo_block<'a>() -> Block<'a> {
  layout_block().title(Span::styled(
    " Managarr - A Servarr management TUI ",
    Style::new().magenta().bold().italic(),
  ))
}

pub fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
  let popup_layout = vertical_chunks(
    vec![
      Constraint::Percentage((100 - percent_y) / 2),
      Constraint::Percentage(percent_y),
      Constraint::Percentage((100 - percent_y) / 2),
    ],
    r,
  );

  Layout::default()
    .direction(Direction::Horizontal)
    .constraints(
      [
        Constraint::Percentage((100 - percent_x) / 2),
        Constraint::Percentage(percent_x),
        Constraint::Percentage((100 - percent_x) / 2),
      ]
      .as_ref(),
    )
    .split(popup_layout[1])[1]
}

pub fn line_gauge_with_title(title: &str, ratio: f64) -> LineGauge<'_> {
  LineGauge::new()
    .block(Block::new().title(title))
    .gauge_style(Style::new().cyan())
    .line_set(symbols::line::THICK)
    .ratio(ratio)
    .label(Line::from(format!("{:.0}%", ratio * 100.0)))
}

pub fn line_gauge_with_label(title: &str, ratio: f64) -> LineGauge<'_> {
  LineGauge::new()
    .block(Block::new())
    .gauge_style(Style::new().cyan())
    .line_set(symbols::line::THICK)
    .ratio(ratio)
    .label(Line::from(format!("{title}: {:.0}%", ratio * 100.0)))
}

pub fn show_cursor(
  f: &mut Frame<'_>,
  area: Rect,
  offset: usize,
  string: &str,
  cursor_after_string: bool,
) {
  if cursor_after_string {
    f.set_cursor(area.x + (string.len() - offset) as u16 + 1, area.y + 1);
  } else {
    f.set_cursor(area.x + 1u16, area.y + 1);
  }
}

pub fn get_width_from_percentage(area: Rect, percentage: u16) -> usize {
  (area.width as f64 * (percentage as f64 / 100.0)) as usize
}
