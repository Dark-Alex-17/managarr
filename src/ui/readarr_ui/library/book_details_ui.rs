use crate::app::App;
use crate::models::Route;
use crate::models::readarr_models::{BookFile, Edition, ReadarrHistoryItem, ReadarrRelease};
use crate::models::servarr_data::readarr::readarr_data::{ActiveReadarrBlock, BOOK_DETAILS_BLOCKS};
use crate::ui::readarr_ui::library::edition_details_ui::EditionDetailsUi;
use crate::ui::readarr_ui::readarr_ui_utils::create_history_event_details;
use crate::ui::styles::{ManagarrStyle, secondary_style};
use crate::ui::utils::{
  borderless_block, decorate_peer_style, get_width_from_percentage, layout_block_top_border,
};
use crate::ui::widgets::confirmation_prompt::ConfirmationPrompt;
use crate::ui::widgets::loading_block::LoadingBlock;
use crate::ui::widgets::managarr_table::ManagarrTable;
use crate::ui::widgets::message::Message;
use crate::ui::widgets::popup::{Popup, Size};
use crate::ui::{DrawUi, draw_popup, draw_tabs};
use crate::utils::format_size;
use ratatui::Frame;
use ratatui::layout::{Alignment, Constraint, Rect};
use ratatui::prelude::{Line, Stylize, Text};
use ratatui::widgets::{Cell, Paragraph, Row, Wrap};
use serde_json::Number;

#[cfg(test)]
#[path = "book_details_ui_tests.rs"]
mod book_details_ui_tests;

pub(super) struct BookDetailsUi;

impl DrawUi for BookDetailsUi {
  fn accepts(route: Route) -> bool {
    let Route::Readarr(active_readarr_block, _) = route else {
      return false;
    };
    EditionDetailsUi::accepts(route) || BOOK_DETAILS_BLOCKS.contains(&active_readarr_block)
  }

  fn draw(f: &mut Frame<'_>, app: &mut App<'_>, _area: Rect) {
    let route = app.get_current_route();
    if app.data.readarr_data.book_details_modal.is_some()
      && let Route::Readarr(active_readarr_block, _) = app.get_current_route()
    {
      let draw_book_details_popup = |f: &mut Frame<'_>, app: &mut App<'_>, popup_area: Rect| {
        let content_area = draw_tabs(
          f,
          popup_area,
          &format!(
            "{} Details",
            app.data.readarr_data.books.current_selection().title.text
          ),
          &app
            .data
            .readarr_data
            .book_details_modal
            .as_ref()
            .expect("Book details modal is undefined")
            .book_details_tabs,
        );
        draw_book_details(f, app, content_area);

        match active_readarr_block {
          ActiveReadarrBlock::AutomaticallySearchBookPrompt => {
            let prompt = format!(
              "Do you want to trigger an automatic search of your indexers for the book: {}?",
              app.data.readarr_data.books.current_selection().title.text
            );
            let confirmation_prompt = ConfirmationPrompt::new()
              .title("Automatic Book Search")
              .prompt(&prompt)
              .yes_no_value(app.data.readarr_data.prompt_confirm);

            f.render_widget(
              Popup::new(confirmation_prompt).size(Size::MediumPrompt),
              f.area(),
            );
          }
          ActiveReadarrBlock::DeleteBookFilePrompt => {
            let prompt = format!(
              "Do you really want to delete this book file: \n{}?",
              app.data.readarr_data.books.current_selection().title.text
            );
            let confirmation_prompt = ConfirmationPrompt::new()
              .title("Delete Book File")
              .prompt(&prompt)
              .yes_no_value(app.data.readarr_data.prompt_confirm);

            f.render_widget(
              Popup::new(confirmation_prompt).size(Size::MediumPrompt),
              f.area(),
            );
          }
          ActiveReadarrBlock::ManualBookSearchConfirmPrompt => {
            draw_manual_book_search_confirm_prompt(f, app);
          }
          ActiveReadarrBlock::BookHistoryDetails => {
            draw_history_item_details_popup(f, app);
          }
          _ => (),
        }
      };

      draw_popup(f, app, draw_book_details_popup, Size::XLarge);

      if EditionDetailsUi::accepts(route) {
        EditionDetailsUi::draw(f, app, _area);
      }
    }
  }
}

fn draw_book_details(f: &mut Frame<'_>, app: &mut App<'_>, area: Rect) {
  if let Some(book_details_modal) = app.data.readarr_data.book_details_modal.as_ref()
    && let Route::Readarr(active_readarr_block, _) =
      book_details_modal.book_details_tabs.get_active_route()
  {
    match active_readarr_block {
      ActiveReadarrBlock::BookDetails => draw_editions_table(f, app, area),
      ActiveReadarrBlock::BookHistory => draw_book_history_table(f, app, area),
      ActiveReadarrBlock::BookFileInfo => draw_book_file_info(f, app, area),
      ActiveReadarrBlock::ManualBookSearch => draw_book_releases(f, app, area),
      _ => (),
    }
  }
}

fn draw_editions_table(f: &mut Frame<'_>, app: &mut App<'_>, area: Rect) {
  if let Route::Readarr(active_readarr_block, _) = app.get_current_route() {
    let content = Some(
      &mut app
        .data
        .readarr_data
        .book_details_modal
        .as_mut()
        .expect("Book details modal is undefined")
        .editions,
    );

    let edition_row_mapping = |edition: &Edition| {
      let Edition {
        format,
        isbn13,
        asin,
        publisher,
        page_count,
        release_date,
        monitored,
        ..
      } = edition;

      let page_count = page_count
        .map(|count| count.to_string())
        .unwrap_or_default();
      let release_date = release_date
        .map(|date| date.format("%Y-%m-%d").to_string())
        .unwrap_or_default();
      let monitored = if *monitored { "🏷" } else { "" };

      decorate_edition_row_with_style(
        edition,
        Row::new(vec![
          Cell::from(format.clone().unwrap_or_default()),
          Cell::from(isbn13.clone().unwrap_or_default()),
          Cell::from(asin.clone().unwrap_or_default()),
          Cell::from(publisher.clone().unwrap_or_default()),
          Cell::from(page_count),
          Cell::from(release_date),
          Cell::from(monitored.to_owned()),
        ]),
      )
    };

    let is_searching = active_readarr_block == ActiveReadarrBlock::SearchEditions;
    let editions_table = ManagarrTable::new(content, edition_row_mapping)
      .block(layout_block_top_border())
      .loading(app.is_loading)
      .searching(is_searching)
      .search_produced_empty_results(
        active_readarr_block == ActiveReadarrBlock::SearchEditionsError,
      )
      .headers([
        "Format",
        "ISBN13",
        "ASIN",
        "Publisher",
        "Page Count",
        "Release Date",
        "Monitored",
      ])
      .constraints([
        Constraint::Percentage(14),
        Constraint::Percentage(16),
        Constraint::Percentage(14),
        Constraint::Percentage(21),
        Constraint::Percentage(11),
        Constraint::Percentage(14),
        Constraint::Percentage(10),
      ]);

    if is_searching {
      editions_table.show_cursor(f, area);
    }

    f.render_widget(editions_table, area);
  }
}

fn decorate_edition_row_with_style<'a>(edition: &Edition, row: Row<'a>) -> Row<'a> {
  if edition.monitored {
    row.primary()
  } else {
    row.unmonitored()
  }
}

fn draw_book_file_info(f: &mut Frame<'_>, app: &mut App<'_>, area: Rect) {
  match app.data.readarr_data.book_details_modal.as_ref() {
    Some(book_details_modal) if !app.is_loading && !book_details_modal.book_files.is_empty() => {
      let content = Some(
        &mut app
          .data
          .readarr_data
          .book_details_modal
          .as_mut()
          .expect("Book details modal is undefined")
          .book_files,
      );

      let book_file_row_mapping = |book_file: &BookFile| {
        let BookFile {
          path,
          size,
          quality,
          media_info,
          ..
        } = book_file;

        let size = format_size(*size, 2);
        let audio_info = media_info
          .as_ref()
          .map(|media_info| {
            let codec = media_info.audio_codec.as_deref().unwrap_or("");
            let channels = format!("{}.0", media_info.audio_channels);
            let bitrate = media_info.audio_bit_rate.as_deref().unwrap_or("");
            let sample_rate = media_info.audio_sample_rate.as_deref().unwrap_or("");
            let bits = media_info.audio_bits.as_deref().unwrap_or("");
            format!("{codec} - {channels} - {bitrate} - {sample_rate} - {bits}")
          })
          .unwrap_or_default();

        Row::new(vec![
          Cell::from(path.clone()),
          Cell::from(size),
          Cell::from(quality.quality.name.clone()),
          Cell::from(audio_info),
        ])
        .primary()
      };

      let book_files_table = ManagarrTable::new(content, book_file_row_mapping)
        .block(layout_block_top_border())
        .loading(app.is_loading)
        .headers(["Path", "Size", "Quality", "Media Info"])
        .constraints([
          Constraint::Percentage(45),
          Constraint::Percentage(11),
          Constraint::Percentage(14),
          Constraint::Percentage(30),
        ]);

      f.render_widget(book_files_table, area);
    }
    _ => f.render_widget(
      LoadingBlock::new(
        app.is_loading || app.data.readarr_data.book_details_modal.is_none(),
        layout_block_top_border(),
      ),
      area,
    ),
  }
}

fn draw_book_history_table(f: &mut Frame<'_>, app: &mut App<'_>, area: Rect) {
  match app.data.readarr_data.book_details_modal.as_ref() {
    Some(book_details_modal) if !app.is_loading => {
      let current_selection = if book_details_modal.book_history.is_empty() {
        ReadarrHistoryItem::default()
      } else {
        book_details_modal.book_history.current_selection().clone()
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
        let mut book_history_table = &mut app
          .data
          .readarr_data
          .book_details_modal
          .as_mut()
          .expect("Book details modal is undefined")
          .book_history;
        let history_table = ManagarrTable::new(Some(&mut book_history_table), history_row_mapping)
          .block(layout_block_top_border())
          .loading(app.is_loading)
          .sorting(active_readarr_block == ActiveReadarrBlock::BookHistorySortPrompt)
          .searching(active_readarr_block == ActiveReadarrBlock::SearchBookHistory)
          .search_produced_empty_results(
            active_readarr_block == ActiveReadarrBlock::SearchBookHistoryError,
          )
          .filtering(active_readarr_block == ActiveReadarrBlock::FilterBookHistory)
          .filter_produced_empty_results(
            active_readarr_block == ActiveReadarrBlock::FilterBookHistoryError,
          )
          .headers(["Source Title", "Event Type", "Quality", "Date"])
          .constraints([
            Constraint::Percentage(40),
            Constraint::Percentage(20),
            Constraint::Percentage(15),
            Constraint::Percentage(25),
          ]);

        if [
          ActiveReadarrBlock::SearchBookHistory,
          ActiveReadarrBlock::FilterBookHistory,
        ]
        .contains(&active_readarr_block)
        {
          history_table.show_cursor(f, area);
        }

        f.render_widget(history_table, area);
      }
    }
    _ => f.render_widget(
      LoadingBlock::new(
        app.is_loading || app.data.readarr_data.book_details_modal.is_none(),
        layout_block_top_border(),
      ),
      area,
    ),
  }
}

fn draw_book_releases(f: &mut Frame<'_>, app: &mut App<'_>, area: Rect) {
  match app.data.readarr_data.book_details_modal.as_ref() {
    Some(book_details_modal) if !app.is_loading => {
      let (current_selection, is_empty) = if book_details_modal.book_releases.is_empty() {
        (ReadarrRelease::default(), true)
      } else {
        (
          book_details_modal.book_releases.current_selection().clone(),
          book_details_modal.book_releases.is_empty(),
        )
      };

      if let Route::Readarr(active_readarr_block, _) = app.get_current_route() {
        let book_release_row_mapping = |release: &ReadarrRelease| {
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
              && active_readarr_block != ActiveReadarrBlock::ManualBookSearchConfirmPrompt,
            app.should_text_scroll,
          );
          let size = format_size(*size, 1);
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
            Cell::from(size),
            Cell::from(peers),
            Cell::from(quality_name),
          ])
          .primary()
        };
        let mut book_release_table = &mut app
          .data
          .readarr_data
          .book_details_modal
          .as_mut()
          .expect("Book details modal is undefined")
          .book_releases;
        let release_table =
          ManagarrTable::new(Some(&mut book_release_table), book_release_row_mapping)
            .block(layout_block_top_border())
            .loading(app.is_loading || is_empty)
            .sorting(active_readarr_block == ActiveReadarrBlock::ManualBookSearchSortPrompt)
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

        f.render_widget(release_table, area);
      }
    }
    _ => f.render_widget(
      LoadingBlock::new(
        app.is_loading || app.data.readarr_data.book_details_modal.is_none(),
        layout_block_top_border(),
      ),
      area,
    ),
  }
}

fn draw_manual_book_search_confirm_prompt(f: &mut Frame<'_>, app: &mut App<'_>) {
  let current_selection = app
    .data
    .readarr_data
    .book_details_modal
    .as_ref()
    .expect("Book details modal is undefined")
    .book_releases
    .current_selection();
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

fn draw_history_item_details_popup(f: &mut Frame<'_>, app: &mut App<'_>) {
  let current_selection =
    if let Some(book_details_modal) = app.data.readarr_data.book_details_modal.as_ref() {
      if book_details_modal.book_history.is_empty() {
        ReadarrHistoryItem::default()
      } else {
        book_details_modal.book_history.current_selection().clone()
      }
    } else {
      ReadarrHistoryItem::default()
    };

  let line_vec = create_history_event_details(current_selection);
  let text = Text::from(line_vec);

  let message = Message::new(text)
    .title("Details")
    .style(secondary_style())
    .alignment(Alignment::Left);

  f.render_widget(Popup::new(message).size(Size::NarrowLongMessage), f.area());
}
