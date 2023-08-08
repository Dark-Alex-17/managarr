use std::rc::Rc;
use tui::backend::Backend;
use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::text::{Line, Span, Text};
use tui::widgets::{Block, BorderType, Borders, LineGauge, Paragraph, Wrap};
use tui::{symbols, Frame};

pub const COLOR_TEAL: Color = Color::Rgb(35, 50, 55);
pub const COLOR_CYAN: Color = Color::Cyan;
pub const COLOR_LIGHT_BLUE: Color = Color::LightBlue;
pub const COLOR_YELLOW: Color = Color::Yellow;
pub const COLOR_GREEN: Color = Color::Green;
pub const COLOR_RED: Color = Color::Red;
pub const COLOR_ORANGE: Color = Color::Rgb(255, 170, 66);
pub const COLOR_WHITE: Color = Color::White;
pub const COLOR_MAGENTA: Color = Color::Magenta;

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
  Block::default().style(Style::default().bg(COLOR_TEAL).fg(COLOR_WHITE))
}

pub fn layout_block<'a>() -> Block<'a> {
  Block::default()
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
  Block::default().borders(Borders::TOP)
}

pub fn layout_block_bottom_border<'a>() -> Block<'a> {
  Block::default().borders(Borders::BOTTOM)
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
    .style(style_primary().add_modifier(Modifier::BOLD))
    .wrap(Wrap { trim: false })
    .alignment(Alignment::Center)
}

pub fn borderless_block<'a>() -> Block<'a> {
  Block::default()
}

pub fn line_info_with_style<'a>(
  title: String,
  content: String,
  title_style: Style,
  content_style: Style,
) -> Line<'a> {
  Line::from(vec![
    Span::styled(title, title_style),
    Span::styled(content, content_style),
  ])
}

pub fn line_info_default<'a>(title: String, content: String) -> Line<'a> {
  line_info_with_style(title, content, style_bold(), style_default())
}

pub fn line_info_primary<'a>(title: String, content: String) -> Line<'a> {
  line_info_with_style(
    title,
    content,
    style_primary().add_modifier(Modifier::BOLD),
    style_default(),
  )
}

pub fn style_bold() -> Style {
  Style::default().add_modifier(Modifier::BOLD)
}

pub fn style_highlight() -> Style {
  Style::default().add_modifier(Modifier::REVERSED)
}

pub fn style_default() -> Style {
  Style::default().fg(COLOR_WHITE)
}

pub fn style_default_bold() -> Style {
  style_default().add_modifier(Modifier::BOLD)
}

pub fn style_primary() -> Style {
  Style::default().fg(COLOR_CYAN)
}

pub fn style_secondary() -> Style {
  Style::default().fg(COLOR_YELLOW)
}

pub fn style_system_function() -> Style {
  Style::default().fg(COLOR_YELLOW)
}

pub fn style_unmonitored() -> Style {
  Style::default().fg(COLOR_WHITE)
}

pub fn style_success() -> Style {
  Style::default().fg(COLOR_GREEN)
}

pub fn style_warning() -> Style {
  Style::default().fg(COLOR_MAGENTA)
}

pub fn style_failure() -> Style {
  Style::default().fg(COLOR_RED)
}

pub fn style_awaiting_import() -> Style {
  Style::default().fg(COLOR_ORANGE)
}

pub fn style_help() -> Style {
  Style::default().fg(COLOR_LIGHT_BLUE)
}

pub fn style_block_highlight(is_selected: bool) -> Style {
  if is_selected {
    style_system_function().add_modifier(Modifier::BOLD)
  } else {
    style_default_bold()
  }
}

pub fn title_style(title: &str) -> Span<'_> {
  Span::styled(format!("  {}  ", title), style_bold())
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
    Style::default()
      .fg(Color::Magenta)
      .add_modifier(Modifier::BOLD)
      .add_modifier(Modifier::ITALIC),
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
  LineGauge::default()
    .block(Block::default().title(title))
    .gauge_style(Style::default().fg(COLOR_CYAN))
    .line_set(symbols::line::THICK)
    .ratio(ratio)
    .label(Line::from(format!("{:.0}%", ratio * 100.0)))
}

pub fn line_gauge_with_label(title: &str, ratio: f64) -> LineGauge<'_> {
  LineGauge::default()
    .block(Block::default())
    .gauge_style(Style::default().fg(COLOR_CYAN))
    .line_set(symbols::line::THICK)
    .ratio(ratio)
    .label(Line::from(format!("{}: {:.0}%", title, ratio * 100.0)))
}

pub fn show_cursor<B: Backend>(f: &mut Frame<'_, B>, area: Rect, offset: usize, string: &str) {
  f.set_cursor(area.x + (string.len() - offset) as u16 + 1, area.y + 1);
}

pub fn get_width_from_percentage(area: Rect, percentage: u16) -> usize {
  (area.width as f64 * (percentage as f64 / 100.0)) as usize
}
