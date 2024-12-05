use crate::ui::styles::ManagarrStyle;
use ratatui::layout::{Alignment, Constraint, Layout, Rect};
use ratatui::style::{Color, Style, Stylize};
use ratatui::symbols;
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Block, BorderType, Borders, LineGauge, ListItem, Paragraph, Wrap};

pub const COLOR_TEAL: Color = Color::Rgb(35, 50, 55);

#[cfg(test)]
#[path = "utils_tests.rs"]
mod utils_tests;

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

pub fn layout_paragraph_borderless(string: &str) -> Paragraph<'_> {
  Paragraph::new(Text::from(string))
    .block(borderless_block())
    .primary()
    .bold()
    .wrap(Wrap { trim: false })
    .centered()
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

pub fn centered_rect(percent_x: u16, percent_y: u16, area: Rect) -> Rect {
  let [_, vertical_area, _] = Layout::vertical([
    Constraint::Percentage((100 - percent_y) / 2),
    Constraint::Percentage(percent_y),
    Constraint::Percentage((100 - percent_y) / 2),
  ])
  .areas(area);

  let [_, horizontal_layout, _] = Layout::horizontal([
    Constraint::Percentage((100 - percent_x) / 2),
    Constraint::Percentage(percent_x),
    Constraint::Percentage((100 - percent_x) / 2),
  ])
  .areas(vertical_area);

  horizontal_layout
}

pub fn line_gauge_with_title(title: &str, ratio: f64) -> LineGauge<'_> {
  LineGauge::new()
    .block(Block::new().title(title))
    .filled_style(Style::new().cyan())
    .line_set(symbols::line::THICK)
    .ratio(ratio)
    .label(Line::from(format!("{:.0}%", ratio * 100.0)))
}

pub fn line_gauge_with_label(title: &str, ratio: f64) -> LineGauge<'_> {
  LineGauge::new()
    .block(Block::new())
    .filled_style(Style::new().cyan())
    .line_set(symbols::line::THICK)
    .ratio(ratio)
    .label(Line::from(format!("{title}: {:.0}%", ratio * 100.0)))
}

pub fn get_width_from_percentage(area: Rect, percentage: u16) -> usize {
  (area.width as f64 * (percentage as f64 / 100.0)) as usize
}

pub(super) fn style_log_list_item(list_item: ListItem<'_>, level: String) -> ListItem<'_> {
  match level.to_lowercase().as_str() {
    "trace" => list_item.gray(),
    "debug" => list_item.blue(),
    "info" => list_item.style(Style::new().default()),
    "warn" => list_item.style(Style::new().secondary()),
    "error" => list_item.style(Style::new().failure()),
    "fatal" => list_item.style(Style::new().failure().bold()),
    _ => list_item.style(Style::new().default()),
  }
}

pub(super) fn convert_to_minutes_hours_days(time: i64) -> String {
  if time < 60 {
    if time == 0 {
      "now".to_owned()
    } else if time == 1 {
      format!("{time} minute")
    } else {
      format!("{time} minutes")
    }
  } else if time / 60 < 24 {
    let hours = time / 60;
    if hours == 1 {
      format!("{hours} hour")
    } else {
      format!("{hours} hours")
    }
  } else {
    let days = time / (60 * 24);
    if days == 1 {
      format!("{days} day")
    } else {
      format!("{days} days")
    }
  }
}
