#[cfg(test)]
mod test {
  use pretty_assertions::{assert_eq, assert_str_eq};
  use ratatui::layout::{Alignment, Rect};
  use ratatui::style::{Color, Modifier, Style, Stylize};
  use ratatui::text::{Span, Text};
  use ratatui::widgets::{Block, BorderType, Borders, ListItem};
  use rstest::rstest;
  use crate::ui::utils::{borderless_block, centered_rect, convert_to_minutes_hours_days, decorate_peer_style, get_width_from_percentage, layout_block, layout_block_bottom_border, layout_block_top_border, layout_block_top_border_with_title, layout_block_with_title, logo_block, style_block_highlight, style_log_list_item, title_block, title_block_centered, title_style};

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

  #[test]
  fn test_determine_log_style_by_level() {
    use crate::ui::styles::ManagarrStyle;
    let list_item = ListItem::new(Text::from(Span::raw("test")));

    assert_eq!(
      style_log_list_item(list_item.clone(), "trace".to_string()),
      list_item.clone().gray()
    );
    assert_eq!(
      style_log_list_item(list_item.clone(), "debug".to_string()),
      list_item.clone().blue()
    );
    assert_eq!(
      style_log_list_item(list_item.clone(), "info".to_string()),
      list_item.clone().style(Style::new().default())
    );
    assert_eq!(
      style_log_list_item(list_item.clone(), "warn".to_string()),
      list_item.clone().style(Style::new().secondary())
    );
    assert_eq!(
      style_log_list_item(list_item.clone(), "error".to_string()),
      list_item.clone().style(Style::new().failure())
    );
    assert_eq!(
      style_log_list_item(list_item.clone(), "fatal".to_string()),
      list_item.clone().style(Style::new().failure().bold())
    );
    assert_eq!(
      style_log_list_item(list_item.clone(), "".to_string()),
      list_item.style(Style::new().default())
    );
  }

  #[test]
  fn test_determine_log_style_by_level_case_insensitive() {
    let list_item = ListItem::new(Text::from(Span::raw("test")));

    assert_eq!(
      style_log_list_item(list_item.clone(), "TrAcE".to_string()),
      list_item.gray()
    );
  }

  #[test]
  fn test_convert_to_minutes_hours_days_minutes() {
    assert_str_eq!(convert_to_minutes_hours_days(0), "now");
    assert_str_eq!(convert_to_minutes_hours_days(1), "1 minute");
    assert_str_eq!(convert_to_minutes_hours_days(2), "2 minutes");
  }

  #[test]
  fn test_convert_to_minutes_hours_days_hours() {
    assert_str_eq!(convert_to_minutes_hours_days(60), "1 hour");
    assert_str_eq!(convert_to_minutes_hours_days(120), "2 hours");
  }

  #[test]
  fn test_convert_to_minutes_hours_days_days() {
    assert_str_eq!(convert_to_minutes_hours_days(1440), "1 day");
    assert_str_eq!(convert_to_minutes_hours_days(2880), "2 days");
  }

  #[rstest]
  #[case(0, 0, PeerStyle::Failure)]
  #[case(1, 2, PeerStyle::Warning)]
  #[case(4, 2, PeerStyle::Success)]
  fn test_decorate_peer_style(
    #[case] seeders: u64,
    #[case] leechers: u64,
    #[case] expected_style: PeerStyle,
  ) {
    use crate::ui::styles::ManagarrStyle;
    let text = Text::from("test");
    match expected_style {
      PeerStyle::Failure => assert_eq!(
        decorate_peer_style(seeders, leechers, text.clone()),
        text.failure()
      ),
      PeerStyle::Warning => assert_eq!(
        decorate_peer_style(seeders, leechers, text.clone()),
        text.warning()
      ),
      PeerStyle::Success => assert_eq!(
        decorate_peer_style(seeders, leechers, text.clone()),
        text.success()
      ),
    }
  }

  enum PeerStyle {
    Failure,
    Warning,
    Success,
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
