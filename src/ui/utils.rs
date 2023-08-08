use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::symbols;
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, LineGauge};

pub fn horizontal_chunks(constraints: Vec<Constraint>, size: Rect) -> Vec<Rect> {
  Layout::default()
    .constraints(<Vec<Constraint> as AsRef<[Constraint]>>::as_ref(
      &constraints,
    ))
    .direction(Direction::Horizontal)
    .split(size)
}

pub fn horizontal_chunks_with_margin(
  constraints: Vec<Constraint>,
  size: Rect,
  margin: u16,
) -> Vec<Rect> {
  Layout::default()
    .constraints(<Vec<Constraint> as AsRef<[Constraint]>>::as_ref(
      &constraints,
    ))
    .direction(Direction::Horizontal)
    .margin(margin)
    .split(size)
}

pub fn vertical_chunks(constraints: Vec<Constraint>, size: Rect) -> Vec<Rect> {
  Layout::default()
    .constraints(<Vec<Constraint> as AsRef<[Constraint]>>::as_ref(
      &constraints,
    ))
    .direction(Direction::Vertical)
    .split(size)
}

pub fn vertical_chunks_with_margin(
  constraints: Vec<Constraint>,
  size: Rect,
  margin: u16,
) -> Vec<Rect> {
  Layout::default()
    .constraints(<Vec<Constraint> as AsRef<[Constraint]>>::as_ref(
      &constraints,
    ))
    .direction(Direction::Vertical)
    .margin(margin)
    .split(size)
}

pub fn layout_block<'a>() -> Block<'a> {
  Block::default().borders(Borders::ALL)
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

pub fn style_bold() -> Style {
  Style::default().add_modifier(Modifier::BOLD)
}

pub fn style_highlight() -> Style {
  Style::default().add_modifier(Modifier::REVERSED)
}

pub fn style_default() -> Style {
  Style::default().fg(Color::White)
}

pub fn style_default_bold() -> Style {
  style_default().add_modifier(Modifier::BOLD)
}

pub fn style_primary() -> Style {
  Style::default().fg(Color::Cyan)
}

pub fn style_secondary() -> Style {
  Style::default().fg(Color::Yellow)
}

pub fn style_system_function() -> Style {
  Style::default().fg(Color::Yellow)
}

pub fn style_success() -> Style {
  Style::default().fg(Color::Green)
}

pub fn style_warning() -> Style {
  Style::default().fg(Color::Magenta)
}

pub fn style_failure() -> Style {
  Style::default().fg(Color::Red)
}

pub fn title_style(title: &str) -> Span<'_> {
  Span::styled(title, style_bold())
}

pub fn title_block(title: &str) -> Block<'_> {
  layout_block_with_title(title_style(title))
}

pub fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
  let popup_layout = Layout::default()
    .direction(Direction::Vertical)
    .constraints(
      [
        Constraint::Percentage((100 - percent_y) / 2),
        Constraint::Percentage(percent_y),
        Constraint::Percentage((100 - percent_y) / 2),
      ]
      .as_ref(),
    )
    .split(r);

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

pub fn line_gague_with_title(title: &str, ratio: f64) -> LineGauge {
  LineGauge::default()
    .block(Block::default().title(title))
    .gauge_style(Style::default().fg(Color::Cyan))
    .line_set(symbols::line::THICK)
    .ratio(ratio)
    .label(Spans::from(format!("{:.0}%", ratio * 100.0)))
}

pub fn line_gague_with_label(title: &str, ratio: f64) -> LineGauge {
  LineGauge::default()
    .block(Block::default())
    .gauge_style(Style::default().fg(Color::Cyan))
    .line_set(symbols::line::THICK)
    .ratio(ratio)
    .label(Spans::from(format!("{}: {:.0}%", title, ratio * 100.0)))
}