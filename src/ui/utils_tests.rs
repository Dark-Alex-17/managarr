#[cfg(test)]
mod test {
  use pretty_assertions::assert_eq;
  use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
  use ratatui::style::{Color, Modifier, Style};
  use ratatui::text::Span;
  use ratatui::widgets::{Block, BorderType, Borders};

  use crate::ui::utils::{
    borderless_block, centered_rect, get_width_from_percentage, horizontal_chunks,
    horizontal_chunks_with_margin, layout_block, layout_block_bottom_border,
    layout_block_top_border, layout_block_top_border_with_title, layout_block_with_title,
    layout_with_constraints, logo_block, style_block_highlight, title_block, title_block_centered,
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
          height: 10,
        },
        30,
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
