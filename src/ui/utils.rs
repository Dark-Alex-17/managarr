use tui::backend::Backend;
use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans, Text};
use tui::widgets::{Block, BorderType, Borders, LineGauge, Paragraph, Wrap};
use tui::{symbols, Frame};

pub const COLOR_TEAL: Color = Color::Rgb(35, 50, 55);
// pub const COLOR_CYAN: Color = Color::Rgb(0, 230, 230);
pub const COLOR_CYAN: Color = Color::Cyan;
// pub const COLOR_LIGHT_BLUE: Color = Color::Rgb(138, 196, 255);
pub const COLOR_LIGHT_BLUE: Color = Color::LightBlue;
// pub const COLOR_YELLOW: Color = Color::Rgb(249, 229, 113);
pub const COLOR_YELLOW: Color = Color::Yellow;
// pub const COLOR_GREEN: Color = Color::Rgb(72, 213, 150);
pub const COLOR_GREEN: Color = Color::Green;
// pub const COLOR_RED: Color = Color::Rgb(249, 140, 164);
pub const COLOR_RED: Color = Color::Red;
// pub const COLOR_ORANGE: Color = Color::Rgb(255, 170, 66);
// pub const COLOR_WHITE: Color = Color::Rgb(255, 255, 255);
pub const COLOR_WHITE: Color = Color::White;
// pub const COLOR_MAGENTA: Color = Color::Rgb(139, 0, 139);
pub const COLOR_MAGENTA: Color = Color::Magenta;

pub fn horizontal_chunks(constraints: Vec<Constraint>, area: Rect) -> Vec<Rect> {
  layout_with_constraints(constraints)
    .direction(Direction::Horizontal)
    .split(area)
}

pub fn horizontal_chunks_with_margin(
  constraints: Vec<Constraint>,
  area: Rect,
  margin: u16,
) -> Vec<Rect> {
  layout_with_constraints(constraints)
    .direction(Direction::Horizontal)
    .margin(margin)
    .split(area)
}

pub fn vertical_chunks(constraints: Vec<Constraint>, area: Rect) -> Vec<Rect> {
  layout_with_constraints(constraints)
    .direction(Direction::Vertical)
    .split(area)
}

pub fn vertical_chunks_with_margin(
  constraints: Vec<Constraint>,
  area: Rect,
  margin: u16,
) -> Vec<Rect> {
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

pub fn spans_info_with_style<'a>(
  title: String,
  content: String,
  title_style: Style,
  content_style: Style,
) -> Spans<'a> {
  Spans::from(vec![
    Span::styled(title, title_style),
    Span::styled(content, content_style),
  ])
}

pub fn spans_info_default<'a>(title: String, content: String) -> Spans<'a> {
  spans_info_with_style(title, content, style_bold(), style_default())
}

pub fn spans_info_primary<'a>(title: String, content: String) -> Spans<'a> {
  spans_info_with_style(
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
    .label(Spans::from(format!("{:.0}%", ratio * 100.0)))
}

pub fn line_gauge_with_label(title: &str, ratio: f64) -> LineGauge<'_> {
  LineGauge::default()
    .block(Block::default())
    .gauge_style(Style::default().fg(COLOR_CYAN))
    .line_set(symbols::line::THICK)
    .ratio(ratio)
    .label(Spans::from(format!("{}: {:.0}%", title, ratio * 100.0)))
}

pub fn show_cursor<B: Backend>(f: &mut Frame<'_, B>, area: Rect, string: &str) {
  f.set_cursor(area.x + string.len() as u16 + 1, area.y + 1);
}

pub fn get_width_from_percentage(area: Rect, percentage: u16) -> usize {
  (area.width as f64 * (percentage as f64 / 100.0)) as usize
}

#[cfg(test)]
mod test {
  use pretty_assertions::assert_eq;
  use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
  use tui::style::{Color, Modifier, Style};
  use tui::text::{Span, Spans};
  use tui::widgets::{Block, BorderType, Borders};

  use crate::ui::utils::{
    borderless_block, centered_rect, get_width_from_percentage, horizontal_chunks,
    horizontal_chunks_with_margin, layout_block, layout_block_bottom_border,
    layout_block_top_border, layout_block_top_border_with_title, layout_block_with_title,
    layout_with_constraints, logo_block, spans_info_default, spans_info_primary,
    spans_info_with_style, style_block_highlight, style_bold, style_default, style_default_bold,
    style_failure, style_help, style_highlight, style_primary, style_secondary, style_success,
    style_system_function, style_unmonitored, style_warning, title_block, title_block_centered,
    title_style, vertical_chunks, vertical_chunks_with_margin,
  };

  #[test]
  fn test_horizontal_chunks() {
    let constraints = [
      Constraint::Percentage(10),
      Constraint::Max(20),
      Constraint::Min(10),
      Constraint::Length(30),
      Constraint::Ratio(3, 4),
    ];
    let area = rect();
    let expected_layout = Layout::default()
      .constraints(constraints)
      .direction(Direction::Horizontal)
      .split(area);

    assert_eq!(horizontal_chunks(constraints.into(), area), expected_layout);
  }

  #[test]
  fn test_horizontal_chunks_with_margin() {
    let constraints = [
      Constraint::Percentage(10),
      Constraint::Max(20),
      Constraint::Min(10),
      Constraint::Length(30),
      Constraint::Ratio(3, 4),
    ];
    let area = rect();
    let expected_layout = Layout::default()
      .constraints(constraints)
      .direction(Direction::Horizontal)
      .margin(1)
      .split(area);

    assert_eq!(
      horizontal_chunks_with_margin(constraints.into(), area, 1),
      expected_layout
    );
  }

  #[test]
  fn test_vertical_chunks() {
    let constraints = [
      Constraint::Percentage(10),
      Constraint::Max(20),
      Constraint::Min(10),
      Constraint::Length(30),
      Constraint::Ratio(3, 4),
    ];
    let area = rect();
    let expected_layout = Layout::default()
      .constraints(constraints)
      .direction(Direction::Vertical)
      .split(area);

    assert_eq!(vertical_chunks(constraints.into(), area), expected_layout);
  }

  #[test]
  fn test_vertical_chunks_with_margin() {
    let constraints = [
      Constraint::Percentage(10),
      Constraint::Max(20),
      Constraint::Min(10),
      Constraint::Length(30),
      Constraint::Ratio(3, 4),
    ];
    let area = rect();
    let expected_layout = Layout::default()
      .constraints(constraints)
      .direction(Direction::Vertical)
      .margin(1)
      .split(area);

    assert_eq!(
      vertical_chunks_with_margin(constraints.into(), area, 1),
      expected_layout
    );
  }

  #[test]
  fn test_layout_with_constraints() {
    let constraints = [
      Constraint::Percentage(10),
      Constraint::Max(20),
      Constraint::Min(10),
      Constraint::Length(30),
      Constraint::Ratio(3, 4),
    ];
    let expected_layout = Layout::default().constraints(constraints);

    assert_eq!(layout_with_constraints(constraints.into()), expected_layout);
  }

  #[test]
  fn test_layout_block() {
    assert_eq!(
      layout_block(),
      Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
    );
  }

  #[test]
  fn test_layout_block_with_title() {
    let title_span = Span::styled(
      "title",
      Style::default()
        .fg(Color::DarkGray)
        .add_modifier(Modifier::BOLD),
    );
    let expected_block = Block::default()
      .borders(Borders::ALL)
      .border_type(BorderType::Rounded)
      .title(title_span.clone());

    assert_eq!(layout_block_with_title(title_span), expected_block);
  }

  #[test]
  fn test_layout_block_top_border_with_title() {
    let title_span = Span::styled(
      "title",
      Style::default()
        .fg(Color::DarkGray)
        .add_modifier(Modifier::BOLD),
    );
    let expected_block = Block::default()
      .borders(Borders::TOP)
      .title(title_span.clone());

    assert_eq!(
      layout_block_top_border_with_title(title_span),
      expected_block
    );
  }

  #[test]
  fn test_layout_block_top_border() {
    assert_eq!(
      layout_block_top_border(),
      Block::default().borders(Borders::TOP)
    );
  }

  #[test]
  fn test_layout_block_bottom_border() {
    assert_eq!(
      layout_block_bottom_border(),
      Block::default().borders(Borders::BOTTOM)
    );
  }

  #[test]
  fn test_borderless_block() {
    assert_eq!(borderless_block(), Block::default());
  }

  #[test]
  fn test_spans_info_with_style() {
    let first_style = Style::default()
      .fg(Color::DarkGray)
      .add_modifier(Modifier::BOLD);
    let second_style = Style::default()
      .fg(Color::LightYellow)
      .add_modifier(Modifier::ITALIC);
    let expected_spans = Spans::from(vec![
      Span::styled("title".to_owned(), first_style),
      Span::styled("content".to_owned(), second_style),
    ]);

    assert_eq!(
      spans_info_with_style(
        "title".to_owned(),
        "content".to_owned(),
        first_style,
        second_style
      ),
      expected_spans
    );
  }

  #[test]
  fn test_spans_info_default() {
    let expected_spans = Spans::from(vec![
      Span::styled(
        "title".to_owned(),
        Style::default().add_modifier(Modifier::BOLD),
      ),
      Span::styled("content".to_owned(), Style::default().fg(Color::White)),
    ]);

    assert_eq!(
      spans_info_default("title".to_owned(), "content".to_owned()),
      expected_spans
    );
  }

  #[test]
  fn test_spans_info_primary() {
    let expected_spans = Spans::from(vec![
      Span::styled(
        "title".to_owned(),
        Style::default()
          .fg(Color::Cyan)
          .add_modifier(Modifier::BOLD),
      ),
      Span::styled("content".to_owned(), Style::default().fg(Color::White)),
    ]);

    assert_eq!(
      spans_info_primary("title".to_owned(), "content".to_owned()),
      expected_spans
    );
  }

  #[test]
  fn test_style_bold() {
    assert_eq!(style_bold(), Style::default().add_modifier(Modifier::BOLD));
  }

  #[test]
  fn test_style_highlight() {
    assert_eq!(
      style_highlight(),
      Style::default().add_modifier(Modifier::REVERSED)
    );
  }

  #[test]
  fn test_style_unmonitored() {
    assert_eq!(style_unmonitored(), Style::default().fg(Color::White));
  }

  #[test]
  fn test_style_default() {
    assert_eq!(style_default(), Style::default().fg(Color::White));
  }

  #[test]
  fn test_style_default_bold() {
    assert_eq!(
      style_default_bold(),
      Style::default()
        .fg(Color::White)
        .add_modifier(Modifier::BOLD)
    );
  }

  #[test]
  fn test_style_primary() {
    assert_eq!(style_primary(), Style::default().fg(Color::Cyan));
  }

  #[test]
  fn test_style_secondary() {
    assert_eq!(style_secondary(), Style::default().fg(Color::Yellow));
  }

  #[test]
  fn test_style_system_function() {
    assert_eq!(style_system_function(), Style::default().fg(Color::Yellow));
  }

  #[test]
  fn test_style_success() {
    assert_eq!(style_success(), Style::default().fg(Color::Green));
  }

  #[test]
  fn test_style_warning() {
    assert_eq!(style_warning(), Style::default().fg(Color::Magenta));
  }

  #[test]
  fn test_style_failure() {
    assert_eq!(style_failure(), Style::default().fg(Color::Red));
  }

  #[test]
  fn test_style_help() {
    assert_eq!(style_help(), Style::default().fg(Color::LightBlue));
  }

  #[test]
  fn test_style_button_highlight_selected() {
    let expected_style = Style::default()
      .fg(Color::Yellow)
      .add_modifier(Modifier::BOLD);

    assert_eq!(style_block_highlight(true), expected_style);
  }

  #[test]
  fn test_style_button_highlight_unselected() {
    let expected_style = Style::default()
      .fg(Color::White)
      .add_modifier(Modifier::BOLD);

    assert_eq!(style_block_highlight(false), expected_style);
  }

  #[test]
  fn test_title_style() {
    let expected_span = Span::styled("  test  ", Style::default().add_modifier(Modifier::BOLD));

    assert_eq!(title_style("test"), expected_span);
  }

  #[test]
  fn test_title_block() {
    let expected_block = Block::default()
      .borders(Borders::ALL)
      .border_type(BorderType::Rounded)
      .title(Span::styled(
        "  test  ",
        Style::default().add_modifier(Modifier::BOLD),
      ));

    assert_eq!(title_block("test"), expected_block);
  }

  #[test]
  fn test_title_block_centered() {
    let expected_block = Block::default()
      .borders(Borders::ALL)
      .border_type(BorderType::Rounded)
      .title(Span::styled(
        "  test  ",
        Style::default().add_modifier(Modifier::BOLD),
      ))
      .title_alignment(Alignment::Center);

    assert_eq!(title_block_centered("test"), expected_block);
  }

  #[test]
  fn test_logo_block() {
    let expected_block = Block::default()
      .borders(Borders::ALL)
      .border_type(BorderType::Rounded)
      .title(Span::styled(
        " Managarr - A Servarr management TUI ",
        Style::default()
          .fg(Color::Magenta)
          .add_modifier(Modifier::BOLD)
          .add_modifier(Modifier::ITALIC),
      ));

    assert_eq!(logo_block(), expected_block);
  }

  #[test]
  fn test_centered_rect() {
    let expected_rect = Rect {
      x: 30,
      y: 45,
      width: 60,
      height: 90,
    };

    assert_eq!(centered_rect(50, 50, rect()), expected_rect);
  }

  #[test]
  fn test_get_width_from_percentage() {
    assert_eq!(
      get_width_from_percentage(
        Rect {
          x: 0,
          y: 0,
          width: 100,
          height: 10
        },
        30
      ),
      30
    );
  }

  fn rect() -> Rect {
    Rect {
      x: 0,
      y: 0,
      width: 120,
      height: 180,
    }
  }
}
