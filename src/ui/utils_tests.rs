#[cfg(test)]
mod test {
  use crate::app::{App, ServarrConfig};
  use crate::models::servarr_models::{DiskSpace, RootFolder};
  use crate::ui::styles::{ManagarrStyle, default_style, failure_style, secondary_style};
  use crate::ui::utils::{
    borderless_block, centered_rect, convert_to_minutes_hours_days, decorate_peer_style,
    extract_monitored_disk_space_vec, extract_monitored_root_folders, get_width_from_percentage,
    layout_block, layout_block_bottom_border, layout_block_top_border,
    layout_block_top_border_with_title, layout_block_with_title, logo_block, style_block_highlight,
    style_log_list_item, title_block, title_block_centered, title_style, unstyled_title_block,
  };
  use pretty_assertions::{assert_eq, assert_str_eq};
  use ratatui::layout::{Alignment, Rect};
  use ratatui::style::{Color, Modifier, Style, Stylize};
  use ratatui::text::{Span, Text};
  use ratatui::widgets::{Block, BorderType, Borders, ListItem};
  use rstest::rstest;

  #[test]
  fn test_layout_block() {
    assert_eq!(
      layout_block(),
      Block::new()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
    );
  }

  #[test]
  fn test_layout_block_with_title() {
    let title_span = Span::styled(
      "title",
      Style::new()
        .fg(Color::DarkGray)
        .add_modifier(Modifier::BOLD),
    );
    let expected_block = Block::new()
      .borders(Borders::ALL)
      .border_type(BorderType::Rounded)
      .title(title_span.clone());

    assert_eq!(layout_block_with_title(title_span), expected_block);
  }

  #[test]
  fn test_layout_block_top_border_with_title() {
    let title_span = Span::styled(
      "title",
      Style::new()
        .fg(Color::DarkGray)
        .add_modifier(Modifier::BOLD),
    );
    let expected_block = Block::new()
      .default_color()
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
      Block::new().borders(Borders::TOP).default_color()
    );
  }

  #[test]
  fn test_layout_block_bottom_border() {
    assert_eq!(
      layout_block_bottom_border(),
      Block::new().borders(Borders::BOTTOM).default_color()
    );
  }

  #[test]
  fn test_borderless_block() {
    assert_eq!(borderless_block(), Block::new().default_color());
  }

  #[test]
  fn test_style_button_highlight_selected() {
    let expected_style = Style::new().fg(Color::Yellow).add_modifier(Modifier::BOLD);

    assert_eq!(style_block_highlight(true), expected_style);
  }

  #[test]
  fn test_style_button_highlight_unselected() {
    let expected_style = Style::new().fg(Color::White).add_modifier(Modifier::BOLD);

    assert_eq!(style_block_highlight(false), expected_style);
  }

  #[test]
  fn test_title_style() {
    let expected_span = Span::styled("  test  ", Style::new().add_modifier(Modifier::BOLD));

    assert_eq!(title_style("test"), expected_span);
  }

  #[test]
  fn test_unstyled_title_block() {
    let expected_block = Block::new()
      .borders(Borders::ALL)
      .border_type(BorderType::Rounded)
      .title(Span::styled(
        "  test  ",
        Style::new().add_modifier(Modifier::BOLD),
      ));

    assert_eq!(unstyled_title_block("test"), expected_block);
  }

  #[test]
  fn test_title_block() {
    let expected_block = Block::new()
      .default_color()
      .borders(Borders::ALL)
      .border_type(BorderType::Rounded)
      .title(Span::styled(
        "  test  ",
        Style::new().add_modifier(Modifier::BOLD),
      ));

    assert_eq!(title_block("test"), expected_block);
  }

  #[test]
  fn test_title_block_centered() {
    let expected_block = Block::new()
      .default_color()
      .borders(Borders::ALL)
      .border_type(BorderType::Rounded)
      .title(Span::styled(
        "  test  ",
        Style::new().add_modifier(Modifier::BOLD),
      ))
      .title_alignment(Alignment::Center);

    assert_eq!(title_block_centered("test"), expected_block);
  }

  #[test]
  fn test_logo_block() {
    let expected_block = Block::new()
      .default_color()
      .borders(Borders::ALL)
      .border_type(BorderType::Rounded)
      .title(Span::styled(
        " Managarr - A Servarr management TUI ",
        Style::new()
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
      list_item.clone().style(default_style())
    );
    assert_eq!(
      style_log_list_item(list_item.clone(), "warn".to_string()),
      list_item.clone().style(secondary_style())
    );
    assert_eq!(
      style_log_list_item(list_item.clone(), "error".to_string()),
      list_item.clone().style(failure_style())
    );
    assert_eq!(
      style_log_list_item(list_item.clone(), "fatal".to_string()),
      list_item.clone().style(failure_style().bold())
    );
    assert_eq!(
      style_log_list_item(list_item.clone(), "".to_string()),
      list_item.style(default_style())
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

  #[test]
  fn test_extract_monitored_root_folders_collapses_subfolders() {
    let mut app = App::test_default();
    app.server_tabs.tabs[0].config = Some(ServarrConfig {
      monitored_storage_paths: Some(vec!["/nfs".to_owned()]),
      ..ServarrConfig::default()
    });
    let root_folders = vec![
      RootFolder {
        id: 1,
        path: "/nfs/cartoons".to_string(),
        accessible: true,
        free_space: 100,
        unmapped_folders: None,
      },
      RootFolder {
        id: 2,
        path: "/nfs/tv".to_string(),
        accessible: true,
        free_space: 100,
        unmapped_folders: None,
      },
      RootFolder {
        id: 3,
        path: "/nfs/reality".to_string(),
        accessible: true,
        free_space: 100,
        unmapped_folders: None,
      },
    ];

    let monitored_root_folders = extract_monitored_root_folders(&app, root_folders);

    assert_eq!(monitored_root_folders.len(), 1);
    assert_eq!(monitored_root_folders[0].path, "/nfs/[cartoons,reality,tv]");
    assert_eq!(monitored_root_folders[0].free_space, 100);
  }

  #[test]
  fn test_extract_monitored_root_folders_uses_most_specific_monitored_path() {
    let mut app = App::test_default();
    app.server_tabs.tabs[0].config = Some(ServarrConfig {
      monitored_storage_paths: Some(vec!["/nfs".to_owned(), "/".to_owned()]),
      ..ServarrConfig::default()
    });
    let root_folders = vec![
      RootFolder {
        id: 1,
        path: "/nfs/cartoons".to_string(),
        accessible: true,
        free_space: 100,
        unmapped_folders: None,
      },
      RootFolder {
        id: 2,
        path: "/nfs/tv".to_string(),
        accessible: true,
        free_space: 100,
        unmapped_folders: None,
      },
      RootFolder {
        id: 3,
        path: "/other/movies".to_string(),
        accessible: true,
        free_space: 200,
        unmapped_folders: None,
      },
    ];

    let monitored_root_folders = extract_monitored_root_folders(&app, root_folders);

    assert_eq!(monitored_root_folders.len(), 2);
    assert_eq!(monitored_root_folders[0].path, "/[other]");
    assert_eq!(monitored_root_folders[0].free_space, 200);
    assert_eq!(monitored_root_folders[1].path, "/nfs/[cartoons,tv]");
    assert_eq!(monitored_root_folders[1].free_space, 100);
  }

  #[test]
  fn test_extract_monitored_root_folders_preserves_unmatched_folders() {
    let mut app = App::test_default();
    app.server_tabs.tabs[0].config = Some(ServarrConfig {
      monitored_storage_paths: Some(vec!["/nfs".to_owned()]),
      ..ServarrConfig::default()
    });
    let root_folders = vec![
      RootFolder {
        id: 1,
        path: "/nfs/tv".to_string(),
        accessible: true,
        free_space: 100,
        unmapped_folders: None,
      },
      RootFolder {
        id: 2,
        path: "/other/movies".to_string(),
        accessible: true,
        free_space: 200,
        unmapped_folders: None,
      },
    ];

    let monitored_root_folders = extract_monitored_root_folders(&app, root_folders);

    assert_eq!(monitored_root_folders.len(), 2);
    assert_eq!(monitored_root_folders[0].path, "/nfs/[tv]");
    assert_eq!(monitored_root_folders[1].path, "/other/movies");
  }

  #[test]
  fn test_extract_monitored_root_folders_returns_all_when_monitored_storage_paths_is_empty() {
    let mut app = App::test_default();
    app.server_tabs.tabs[0].config = Some(ServarrConfig {
      monitored_storage_paths: Some(vec![]),
      ..ServarrConfig::default()
    });
    let root_folders = vec![
      RootFolder {
        id: 1,
        path: "/nfs".to_string(),
        accessible: true,
        free_space: 10,
        unmapped_folders: None,
      },
      RootFolder {
        id: 2,
        path: "/nfs/some/subpath".to_string(),
        accessible: true,
        free_space: 10,
        unmapped_folders: None,
      },
    ];

    let monitored_root_folders = extract_monitored_root_folders(&app, root_folders.clone());

    assert_eq!(monitored_root_folders, root_folders);
  }

  #[test]
  fn test_extract_monitored_root_folders_returns_all_when_monitored_storage_paths_is_none() {
    let app = App::test_default();
    let root_folders = vec![
      RootFolder {
        id: 1,
        path: "/nfs".to_string(),
        accessible: true,
        free_space: 10,
        unmapped_folders: None,
      },
      RootFolder {
        id: 2,
        path: "/nfs/some/subpath".to_string(),
        accessible: true,
        free_space: 10,
        unmapped_folders: None,
      },
    ];

    let monitored_root_folders = extract_monitored_root_folders(&app, root_folders.clone());

    assert_eq!(monitored_root_folders, root_folders);
  }

  #[test]
  fn test_extract_monitored_root_folders_exact_match_shows_no_brackets() {
    let mut app = App::test_default();
    app.server_tabs.tabs[0].config = Some(ServarrConfig {
      monitored_storage_paths: Some(vec!["/nfs/tv".to_owned()]),
      ..ServarrConfig::default()
    });
    let root_folders = vec![RootFolder {
      id: 1,
      path: "/nfs/tv".to_string(),
      accessible: true,
      free_space: 100,
      unmapped_folders: None,
    }];

    let monitored_root_folders = extract_monitored_root_folders(&app, root_folders);

    assert_eq!(monitored_root_folders.len(), 1);
    assert_eq!(monitored_root_folders[0].path, "/nfs/tv");
  }

  #[test]
  fn test_extract_monitored_disk_space_vec() {
    let mut app = App::test_default();
    app.server_tabs.tabs[0].config = Some(ServarrConfig {
      monitored_storage_paths: Some(vec!["/nfs".to_owned()]),
      ..ServarrConfig::default()
    });
    let disk_space = DiskSpace {
      path: Some("/nfs".to_string()),
      free_space: 10,
      total_space: 1000,
    };
    let disk_space_with_empty_path = DiskSpace {
      path: None,
      free_space: 10,
      total_space: 1000,
    };
    let disk_spaces = vec![
      disk_space.clone(),
      disk_space_with_empty_path.clone(),
      DiskSpace {
        path: Some("/nfs/some/subpath".to_string()),
        free_space: 10,
        total_space: 1000,
      },
    ];

    let monitored_disk_space = extract_monitored_disk_space_vec(&app, disk_spaces);

    assert_eq!(
      monitored_disk_space,
      vec![disk_space, disk_space_with_empty_path]
    );
  }

  #[test]
  fn test_extract_monitored_disk_space_vec_returns_all_when_monitored_storage_paths_is_empty() {
    let mut app = App::test_default();
    app.server_tabs.tabs[0].config = Some(ServarrConfig {
      monitored_storage_paths: Some(Vec::new()),
      ..ServarrConfig::default()
    });
    let disk_spaces = vec![
      DiskSpace {
        path: Some("/nfs".to_string()),
        free_space: 10,
        total_space: 1000,
      },
      DiskSpace {
        path: None,
        free_space: 10,
        total_space: 1000,
      },
      DiskSpace {
        path: Some("/nfs/some/subpath".to_string()),
        free_space: 10,
        total_space: 1000,
      },
    ];

    let monitored_disk_space = extract_monitored_disk_space_vec(&app, disk_spaces.clone());

    assert_eq!(monitored_disk_space, disk_spaces);
  }

  #[test]
  fn test_extract_monitored_disk_space_vec_returns_all_when_monitored_storage_paths_is_none() {
    let app = App::test_default();
    let disk_spaces = vec![
      DiskSpace {
        path: Some("/nfs".to_string()),
        free_space: 10,
        total_space: 1000,
      },
      DiskSpace {
        path: None,
        free_space: 10,
        total_space: 1000,
      },
      DiskSpace {
        path: Some("/nfs/some/subpath".to_string()),
        free_space: 10,
        total_space: 1000,
      },
    ];

    let monitored_disk_space = extract_monitored_disk_space_vec(&app, disk_spaces.clone());

    assert_eq!(monitored_disk_space, disk_spaces);
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
