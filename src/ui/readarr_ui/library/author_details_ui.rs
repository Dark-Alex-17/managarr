use chrono::Utc;
use deunicode::deunicode;
use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::Stylize;
use ratatui::text::Line;
use ratatui::widgets::{Cell, Paragraph, Row, Wrap};

use crate::app::App;
use crate::models::Route;
use crate::models::readarr_models::{Book, ReadarrHistoryItem, ReadarrRelease};
use crate::models::servarr_data::readarr::readarr_data::{
  AUTHOR_DETAILS_BLOCKS, ActiveReadarrBlock,
};
use crate::ui::readarr_ui::library::author_overview_ui::AuthorOverviewUi;
use crate::ui::readarr_ui::library::book_details_ui::BookDetailsUi;
use crate::ui::readarr_ui::library::delete_book_ui::DeleteBookUi;
use crate::ui::readarr_ui::readarr_ui_utils::create_history_event_details;
use crate::ui::styles::{ManagarrStyle, secondary_style};
use crate::ui::utils::decorate_peer_style;
use crate::ui::utils::{
  borderless_block, collapse_whitespace, get_width_from_percentage, layout_block_top_border,
  title_block,
};
use crate::ui::widgets::confirmation_prompt::ConfirmationPrompt;
use crate::ui::widgets::loading_block::LoadingBlock;
use crate::ui::widgets::managarr_table::ManagarrTable;
use crate::ui::widgets::message::Message;
use crate::ui::widgets::popup::{Popup, Size};
use crate::ui::{DrawUi, draw_popup, draw_tabs};
use crate::utils::convert_to_gb;
use ratatui::layout::Alignment;
use ratatui::text::Text;
use serde_json::Number;

#[cfg(test)]
#[path = "author_details_ui_tests.rs"]
mod author_details_ui_tests;

pub(super) struct AuthorDetailsUi;

impl DrawUi for AuthorDetailsUi {
  fn accepts(route: Route) -> bool {
    let Route::Readarr(active_readarr_block, _) = route else {
      return false;
    };
    BookDetailsUi::accepts(route)
      || DeleteBookUi::accepts(route)
      || AuthorOverviewUi::accepts(route)
      || AUTHOR_DETAILS_BLOCKS.contains(&active_readarr_block)
  }

  fn draw(f: &mut Frame<'_>, app: &mut App<'_>, _area: Rect) {
    let route = app.get_current_route();
    if let Route::Readarr(active_readarr_block, _) = route {
      let draw_author_details_popup = |f: &mut Frame<'_>, app: &mut App<'_>, popup_area: Rect| {
        f.render_widget(
          title_block(
            &app
              .data
              .readarr_data
              .authors
              .current_selection()
              .author_name
              .text,
          ),
          popup_area,
        );
        let [description_area, detail_area] =
          Layout::vertical([Constraint::Length(12), Constraint::Fill(0)])
            .margin(1)
            .areas(popup_area);
        draw_author_description(f, app, description_area);
        let content_area = draw_tabs(
          f,
          detail_area,
          "Author Details",
          &app.data.readarr_data.author_info_tabs,
        );
        draw_author_details(f, app, content_area);

        match active_readarr_block {
          _ if DeleteBookUi::accepts(route) => DeleteBookUi::draw(f, app, _area),
          ActiveReadarrBlock::AuthorHistoryDetails => {
            draw_author_history_item_details_popup(f, app);
          }
          ActiveReadarrBlock::AutomaticallySearchAuthorPrompt => {
            let prompt = format!(
              "Do you want to trigger an automatic search of your indexers for all monitored book(s) for the author: {}?",
              app
                .data
                .readarr_data
                .authors
                .current_selection()
                .author_name
            );
            let confirmation_prompt = ConfirmationPrompt::new()
              .title("Automatic Author Search")
              .prompt(&prompt)
              .yes_no_value(app.data.readarr_data.prompt_confirm);

            f.render_widget(
              Popup::new(confirmation_prompt).size(Size::MediumPrompt),
              f.area(),
            );
          }
          ActiveReadarrBlock::UpdateAndScanAuthorPrompt => {
            let prompt = format!(
              "Do you want to trigger an update and disk scan for the author: {}?",
              app
                .data
                .readarr_data
                .authors
                .current_selection()
                .author_name
            );
            let confirmation_prompt = ConfirmationPrompt::new()
              .title("Update and Scan")
              .prompt(&prompt)
              .yes_no_value(app.data.readarr_data.prompt_confirm);

            f.render_widget(
              Popup::new(confirmation_prompt).size(Size::MediumPrompt),
              f.area(),
            );
          }
          ActiveReadarrBlock::ManualAuthorSearchConfirmPrompt => {
            draw_manual_author_search_confirm_prompt(f, app);
          }
          _ => (),
        }
      };

      draw_popup(f, app, draw_author_details_popup, Size::XXLarge);

      if BookDetailsUi::accepts(route) {
        BookDetailsUi::draw(f, app, _area);
      }

      if AuthorOverviewUi::accepts(route) {
        AuthorOverviewUi::draw(f, app, _area);
      }
    }
  }
}

fn draw_author_description(f: &mut Frame<'_>, app: &mut App<'_>, area: Rect) {
  let current_selection = app.data.readarr_data.authors.current_selection();
  let monitored = if current_selection.monitored {
    "Yes"
  } else {
    "No"
  };
  let quality_profile = app
    .data
    .readarr_data
    .quality_profile_map
    .get_by_left(&current_selection.quality_profile_id)
    .cloned()
    .unwrap_or_default();
  let metadata_profile = app
    .data
    .readarr_data
    .metadata_profile_map
    .get_by_left(&current_selection.metadata_profile_id)
    .cloned()
    .unwrap_or_default();
  let overview = collapse_whitespace(&deunicode(
    current_selection
      .overview
      .as_ref()
      .unwrap_or(&String::new()),
  ));

  let mut author_description = vec![
    Line::from(vec![
      "Author: ".primary().bold(),
      current_selection.author_name.text.clone().primary().bold(),
    ]),
    Line::from(vec![
      "Status: ".primary().bold(),
      current_selection.status.to_display_str().default_color(),
    ]),
    Line::from(vec![
      "Genres: ".primary().bold(),
      current_selection.genres.join(", ").default_color(),
    ]),
    Line::from(vec![
      "Rating: ".primary().bold(),
      current_selection
        .ratings
        .as_ref()
        .map_or_else(
          || "N/A".to_owned(),
          |r| format!("{}%", (r.value * 10.0) as i32),
        )
        .default_color(),
    ]),
    Line::from(vec![
      "Path: ".primary().bold(),
      current_selection.path.clone().default_color(),
    ]),
    Line::from(vec![
      "Quality Profile: ".primary().bold(),
      quality_profile.default_color(),
    ]),
    Line::from(vec![
      "Metadata Profile: ".primary().bold(),
      metadata_profile.default_color(),
    ]),
    Line::from(vec![
      "Monitored: ".primary().bold(),
      monitored.default_color(),
    ]),
  ];

  if let Some(stats) = current_selection.statistics.as_ref() {
    let size = convert_to_gb(stats.size_on_disk);
    author_description.extend(vec![
      Line::from(vec![
        "Books: ".primary().bold(),
        stats.book_count.to_string().default_color(),
      ]),
      Line::from(vec![
        "Files: ".primary().bold(),
        format!("{}/{}", stats.book_file_count, stats.total_book_count).default_color(),
      ]),
      Line::from(vec![
        "Size on Disk: ".primary().bold(),
        format!("{size:.2} GB").default_color(),
      ]),
    ]);
  }

  author_description.push(Line::from(vec![
    "Overview: ".primary().bold(),
    overview.default_color(),
  ]));

  let description_paragraph = Paragraph::new(author_description)
    .block(borderless_block())
    .wrap(Wrap { trim: true });
  f.render_widget(description_paragraph, area);
}

fn draw_author_details(f: &mut Frame<'_>, app: &mut App<'_>, area: Rect) {
  if let Route::Readarr(active_readarr_block, _) =
    app.data.readarr_data.author_info_tabs.get_active_route()
  {
    match active_readarr_block {
      ActiveReadarrBlock::AuthorDetails => draw_books_table(f, app, area),
      ActiveReadarrBlock::AuthorHistory => draw_author_history_table(f, app, area),
      ActiveReadarrBlock::ManualAuthorSearch => draw_author_releases(f, app, area),
      _ => (),
    }
  }
}

fn draw_books_table(f: &mut Frame<'_>, app: &mut App<'_>, area: Rect) {
  if let Route::Readarr(active_readarr_block, _) = app.get_current_route() {
    let current_selection = if app.data.readarr_data.books.is_empty() {
      Book::default()
    } else {
      app.data.readarr_data.books.current_selection().clone()
    };
    let content = Some(&mut app.data.readarr_data.books);
    let book_row_mapping = |book: &Book| {
      book.title.scroll_left_or_reset(
        get_width_from_percentage(area, 38),
        *book == current_selection,
        app.should_text_scroll,
      );
      let monitored = if book.monitored { "🏷" } else { "" };
      let release_date = book
        .release_date
        .map_or_else(|| "N/A".to_owned(), |d| d.format("%Y-%m-%d").to_string());
      let book_file_count = book.statistics.as_ref().map_or_else(
        || "N/A".to_owned(),
        |s| format!("{}/{}", s.book_file_count, s.total_book_count),
      );
      let size = book.statistics.as_ref().map_or_else(
        || "N/A".to_owned(),
        |s| {
          let size = convert_to_gb(s.size_on_disk);
          format!("{size:.2} GB")
        },
      );

      decorate_book_row_with_style(
        book,
        Row::new(vec![
          Cell::from(monitored.to_owned()),
          Cell::from(book.title.to_string()),
          Cell::from(book_file_count),
          Cell::from(release_date),
          Cell::from(size),
        ]),
      )
    };

    let is_searching = active_readarr_block == ActiveReadarrBlock::SearchBooks;
    let book_table = ManagarrTable::new(content, book_row_mapping)
      .block(layout_block_top_border())
      .loading(app.is_loading)
      .searching(is_searching)
      .search_produced_empty_results(active_readarr_block == ActiveReadarrBlock::SearchBooksError)
      .headers(["Monitored", "Title", "Files", "Release Date", "Size"])
      .constraints([
        Constraint::Percentage(10),
        Constraint::Percentage(40),
        Constraint::Percentage(15),
        Constraint::Percentage(17),
        Constraint::Percentage(18),
      ]);

    if is_searching {
      book_table.show_cursor(f, area);
    }

    f.render_widget(book_table, area);
  }
}

fn decorate_book_row_with_style<'a>(book: &Book, row: Row<'a>) -> Row<'a> {
  if !book.monitored {
    row.unmonitored()
  } else if let Some(stats) = book.statistics.as_ref() {
    if stats.book_file_count == stats.total_book_count && stats.total_book_count > 0 {
      row.downloaded()
    } else if let Some(release_date) = book.release_date.as_ref() {
      if release_date > &Utc::now() {
        row.unreleased()
      } else {
        row.missing()
      }
    } else {
      row.missing()
    }
  } else {
    row.indeterminate()
  }
}

fn draw_author_history_table(f: &mut Frame<'_>, app: &mut App<'_>, area: Rect) {
  if !app.is_loading {
    let current_selection = if app.data.readarr_data.author_history.is_empty() {
      ReadarrHistoryItem::default()
    } else {
      app
        .data
        .readarr_data
        .author_history
        .current_selection()
        .clone()
    };

    if let Route::Readarr(active_readarr_block, _) = app.get_current_route() {
      let history_row_mapping = |history_item: &ReadarrHistoryItem| {
        let ReadarrHistoryItem {
          source_title,
          quality,
          event_type,
          date,
          ..
        } = history_item;

        source_title.scroll_left_or_reset(
          get_width_from_percentage(area, 40),
          current_selection == *history_item,
          app.should_text_scroll,
        );

        Row::new(vec![
          Cell::from(source_title.to_string()),
          Cell::from(event_type.to_string()),
          Cell::from(quality.quality.name.to_owned()),
          Cell::from(date.to_string()),
        ])
        .primary()
      };
      let history_table = ManagarrTable::new(
        Some(&mut app.data.readarr_data.author_history),
        history_row_mapping,
      )
      .block(layout_block_top_border())
      .loading(app.is_loading)
      .sorting(active_readarr_block == ActiveReadarrBlock::AuthorHistorySortPrompt)
      .searching(active_readarr_block == ActiveReadarrBlock::SearchAuthorHistory)
      .search_produced_empty_results(
        active_readarr_block == ActiveReadarrBlock::SearchAuthorHistoryError,
      )
      .filtering(active_readarr_block == ActiveReadarrBlock::FilterAuthorHistory)
      .filter_produced_empty_results(
        active_readarr_block == ActiveReadarrBlock::FilterAuthorHistoryError,
      )
      .headers(["Source Title", "Event Type", "Quality", "Date"])
      .constraints([
        Constraint::Percentage(40),
        Constraint::Percentage(20),
        Constraint::Percentage(15),
        Constraint::Percentage(25),
      ]);

      if [
        ActiveReadarrBlock::SearchAuthorHistory,
        ActiveReadarrBlock::FilterAuthorHistory,
      ]
      .contains(&active_readarr_block)
      {
        history_table.show_cursor(f, area);
      }

      f.render_widget(history_table, area);
    }
  } else {
    f.render_widget(
      LoadingBlock::new(
        app.is_loading || app.data.readarr_data.books.is_empty(),
        layout_block_top_border(),
      ),
      area,
    );
  }
}

fn draw_author_history_item_details_popup(f: &mut Frame<'_>, app: &mut App<'_>) {
  let current_selection = if app.data.readarr_data.author_history.is_empty() {
    ReadarrHistoryItem::default()
  } else {
    app
      .data
      .readarr_data
      .author_history
      .current_selection()
      .clone()
  };

  let line_vec = create_history_event_details(current_selection);
  let text = Text::from(line_vec);

  let message = Message::new(text)
    .title("Details")
    .style(secondary_style())
    .alignment(Alignment::Left);

  f.render_widget(Popup::new(message).size(Size::NarrowLongMessage), f.area());
}

fn draw_author_releases(f: &mut Frame<'_>, app: &mut App<'_>, area: Rect) {
  let (current_selection, is_empty) = if app.data.readarr_data.author_releases.is_empty() {
    (ReadarrRelease::default(), true)
  } else {
    (
      app
        .data
        .readarr_data
        .author_releases
        .current_selection()
        .clone(),
      app.data.readarr_data.author_releases.is_empty(),
    )
  };

  if let Route::Readarr(active_readarr_block, _) = app.get_current_route() {
    let release_row_mapping = |release: &ReadarrRelease| {
      let ReadarrRelease {
        protocol,
        age,
        title,
        indexer,
        size,
        rejected,
        seeders,
        leechers,
        quality,
        ..
      } = release;

      let age = format!("{age} days");
      title.scroll_left_or_reset(
        get_width_from_percentage(area, 35),
        current_selection == *release
          && active_readarr_block != ActiveReadarrBlock::ManualAuthorSearchConfirmPrompt,
        app.should_text_scroll,
      );
      let size = convert_to_gb(*size);
      let rejected_str = if *rejected { "⛔" } else { "" };
      let peers = if seeders.is_none() || leechers.is_none() {
        Text::from("")
      } else {
        let seeders = seeders
          .clone()
          .unwrap_or(Number::from(0u64))
          .as_u64()
          .unwrap_or_default();
        let leechers = leechers
          .clone()
          .unwrap_or(Number::from(0u64))
          .as_u64()
          .unwrap_or_default();

        decorate_peer_style(
          seeders,
          leechers,
          Text::from(format!("{seeders} / {leechers}")),
        )
      };

      let quality_name = quality.quality.name.clone();

      Row::new(vec![
        Cell::from(protocol.clone()),
        Cell::from(age),
        Cell::from(rejected_str),
        Cell::from(title.to_string()),
        Cell::from(indexer.clone()),
        Cell::from(format!("{size:.1} GB")),
        Cell::from(peers),
        Cell::from(quality_name),
      ])
      .primary()
    };
    let author_release_table = ManagarrTable::new(
      Some(&mut app.data.readarr_data.author_releases),
      release_row_mapping,
    )
    .block(layout_block_top_border())
    .loading(app.is_loading || is_empty)
    .sorting(active_readarr_block == ActiveReadarrBlock::ManualAuthorSearchSortPrompt)
    .headers([
      "Source", "Age", "⛔", "Title", "Indexer", "Size", "Peers", "Quality",
    ])
    .constraints([
      Constraint::Length(9),
      Constraint::Length(10),
      Constraint::Length(5),
      Constraint::Percentage(35),
      Constraint::Percentage(15),
      Constraint::Length(12),
      Constraint::Length(12),
      Constraint::Percentage(10),
    ]);

    f.render_widget(author_release_table, area);
  }
}

fn draw_manual_author_search_confirm_prompt(f: &mut Frame<'_>, app: &mut App<'_>) {
  let current_selection = app.data.readarr_data.author_releases.current_selection();
  let title = if current_selection.rejected {
    "Download Rejected Release"
  } else {
    "Download Release"
  };
  let prompt = if current_selection.rejected {
    format!(
      "Do you really want to download the rejected release: {}?",
      current_selection.title.text
    )
  } else {
    format!(
      "Do you want to download the release: {}?",
      current_selection.title.text
    )
  };

  if current_selection.rejected {
    let mut lines_vec = vec![Line::from("Rejection reasons: ".primary().bold())];
    let mut rejections_spans = current_selection
      .rejections
      .clone()
      .unwrap_or_default()
      .iter()
      .map(|item| Line::from(format!("• {item}").primary().bold()))
      .collect::<Vec<Line<'_>>>();
    lines_vec.append(&mut rejections_spans);

    let content_paragraph = Paragraph::new(lines_vec)
      .block(borderless_block())
      .wrap(Wrap { trim: false })
      .left_aligned();
    let confirmation_prompt = ConfirmationPrompt::new()
      .title(title)
      .prompt(&prompt)
      .content(content_paragraph)
      .yes_no_value(app.data.readarr_data.prompt_confirm);

    f.render_widget(Popup::new(confirmation_prompt).size(Size::Small), f.area());
  } else {
    let confirmation_prompt = ConfirmationPrompt::new()
      .title(title)
      .prompt(&prompt)
      .yes_no_value(app.data.readarr_data.prompt_confirm);

    f.render_widget(
      Popup::new(confirmation_prompt).size(Size::MediumPrompt),
      f.area(),
    );
  }
}
