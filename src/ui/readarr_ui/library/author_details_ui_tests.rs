#[cfg(test)]
mod tests {
  use chrono::{Duration, Utc};
  use pretty_assertions::assert_eq;
  use ratatui::widgets::{Cell, Row};
  use strum::IntoEnumIterator;

  use crate::models::readarr_models::{Book, BookStatistics};
  use crate::models::servarr_data::readarr::readarr_data::{
    AUTHOR_DETAILS_BLOCKS, ActiveReadarrBlock,
  };
  use crate::ui::DrawUi;
  use crate::ui::readarr_ui::library::author_details_ui::{
    AuthorDetailsUi, decorate_book_row_with_style,
  };
  use crate::ui::styles::ManagarrStyle;

  #[test]
  fn test_author_details_ui_accepts() {
    ActiveReadarrBlock::iter().for_each(|active_readarr_block| {
      if AUTHOR_DETAILS_BLOCKS.contains(&active_readarr_block) {
        assert!(AuthorDetailsUi::accepts(active_readarr_block.into()));
      } else {
        assert!(!AuthorDetailsUi::accepts(active_readarr_block.into()));
      }
    });
  }

  #[test]
  fn test_decorate_book_row_with_style_unmonitored() {
    let book = Book::default();
    let row = Row::new(vec![Cell::from("test".to_owned())]);

    let style = decorate_book_row_with_style(&book, row.clone());

    assert_eq!(style, row.unmonitored());
  }

  #[test]
  fn test_decorate_book_row_with_style_downloaded_when_all_files_present() {
    let book = Book {
      monitored: true,
      statistics: Some(BookStatistics {
        book_file_count: 3,
        total_book_count: 3,
        ..BookStatistics::default()
      }),
      ..Book::default()
    };
    let row = Row::new(vec![Cell::from("test".to_owned())]);

    let style = decorate_book_row_with_style(&book, row.clone());

    assert_eq!(style, row.downloaded());
  }

  #[test]
  fn test_decorate_book_row_with_style_unreleased_when_files_are_missing_and_release_date_is_future()
   {
    let book = Book {
      monitored: true,
      release_date: Some(Utc::now() + Duration::days(1)),
      statistics: Some(BookStatistics {
        book_file_count: 0,
        total_book_count: 3,
        ..BookStatistics::default()
      }),
      ..Book::default()
    };
    let row = Row::new(vec![Cell::from("test".to_owned())]);

    let style = decorate_book_row_with_style(&book, row.clone());

    assert_eq!(style, row.unreleased());
  }

  #[test]
  fn test_decorate_book_row_with_style_missing_when_files_are_missing_and_release_date_has_passed()
  {
    let book = Book {
      monitored: true,
      release_date: Some(Utc::now() - Duration::days(1)),
      statistics: Some(BookStatistics {
        book_file_count: 1,
        total_book_count: 3,
        ..BookStatistics::default()
      }),
      ..Book::default()
    };
    let row = Row::new(vec![Cell::from("test".to_owned())]);

    let style = decorate_book_row_with_style(&book, row.clone());

    assert_eq!(style, row.missing());
  }

  #[test]
  fn test_decorate_book_row_with_style_missing_when_files_are_missing_and_no_release_date() {
    let book = Book {
      monitored: true,
      release_date: None,
      statistics: Some(BookStatistics {
        book_file_count: 1,
        total_book_count: 3,
        ..BookStatistics::default()
      }),
      ..Book::default()
    };
    let row = Row::new(vec![Cell::from("test".to_owned())]);

    let style = decorate_book_row_with_style(&book, row.clone());

    assert_eq!(style, row.missing());
  }

  #[test]
  fn test_decorate_book_row_with_style_missing_when_total_book_count_is_zero() {
    let book = Book {
      monitored: true,
      release_date: Some(Utc::now() - Duration::days(1)),
      statistics: Some(BookStatistics::default()),
      ..Book::default()
    };
    let row = Row::new(vec![Cell::from("test".to_owned())]);

    let style = decorate_book_row_with_style(&book, row.clone());

    assert_eq!(style, row.missing());
  }

  #[test]
  fn test_decorate_book_row_with_style_indeterminate_when_no_statistics() {
    let book = Book {
      monitored: true,
      statistics: None,
      ..Book::default()
    };
    let row = Row::new(vec![Cell::from("test".to_owned())]);

    let style = decorate_book_row_with_style(&book, row.clone());

    assert_eq!(style, row.indeterminate());
  }

  mod snapshot_tests {
    use rstest::rstest;

    use crate::app::App;
    use crate::models::readarr_models::{AuthorStatistics, BookStatistics};
    use crate::models::servarr_data::readarr::readarr_data::ActiveReadarrBlock;
    use crate::models::stateful_table::StatefulTable;
    use crate::ui::DrawUi;
    use crate::ui::readarr_ui::library::author_details_ui::AuthorDetailsUi;
    use crate::ui::ui_test_utils::test_utils::{TerminalSize, render_to_string_with_app};

    #[rstest]
    #[case(ActiveReadarrBlock::AuthorDetails, 0)]
    #[case(ActiveReadarrBlock::AuthorHistory, 1)]
    #[case(ActiveReadarrBlock::ManualAuthorSearch, 2)]
    #[case(ActiveReadarrBlock::SearchBooks, 0)]
    #[case(ActiveReadarrBlock::SearchBooksError, 0)]
    #[case(ActiveReadarrBlock::UpdateAndScanAuthorPrompt, 0)]
    #[case(ActiveReadarrBlock::UpdateAndScanAuthorPrompt, 1)]
    #[case(ActiveReadarrBlock::UpdateAndScanAuthorPrompt, 2)]
    #[case(ActiveReadarrBlock::AutomaticallySearchAuthorPrompt, 0)]
    #[case(ActiveReadarrBlock::AutomaticallySearchAuthorPrompt, 1)]
    #[case(ActiveReadarrBlock::AutomaticallySearchAuthorPrompt, 2)]
    #[case(ActiveReadarrBlock::SearchAuthorHistory, 1)]
    #[case(ActiveReadarrBlock::SearchAuthorHistoryError, 1)]
    #[case(ActiveReadarrBlock::FilterAuthorHistory, 1)]
    #[case(ActiveReadarrBlock::FilterAuthorHistoryError, 1)]
    #[case(ActiveReadarrBlock::AuthorHistorySortPrompt, 1)]
    #[case(ActiveReadarrBlock::AuthorHistoryDetails, 1)]
    #[case(ActiveReadarrBlock::ManualAuthorSearchConfirmPrompt, 2)]
    #[case(ActiveReadarrBlock::ManualAuthorSearchSortPrompt, 2)]
    fn test_author_details_ui_renders(
      #[case] active_readarr_block: ActiveReadarrBlock,
      #[case] index: usize,
    ) {
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack(active_readarr_block.into());
      app.data.readarr_data.author_info_tabs.set_index(index);

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        AuthorDetailsUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(
        format!("author_details_{active_readarr_block}_{index}"),
        output
      );
    }

    #[rstest]
    #[case(ActiveReadarrBlock::AuthorDetails, 0)]
    #[case(ActiveReadarrBlock::AuthorHistory, 1)]
    #[case(ActiveReadarrBlock::ManualAuthorSearch, 2)]
    fn test_author_details_ui_renders_author_details_loading(
      #[case] active_readarr_block: ActiveReadarrBlock,
      #[case] index: usize,
    ) {
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack(active_readarr_block.into());
      app.data.readarr_data.author_info_tabs.set_index(index);
      app.is_loading = true;

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        AuthorDetailsUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(
        format!("loading_author_details_{active_readarr_block}"),
        output
      );
    }

    #[rstest]
    #[case(ActiveReadarrBlock::AuthorDetails, 0)]
    #[case(ActiveReadarrBlock::AuthorHistory, 1)]
    #[case(ActiveReadarrBlock::AuthorHistoryDetails, 1)]
    #[case(ActiveReadarrBlock::ManualAuthorSearch, 2)]
    fn test_author_details_ui_renders_author_details_empty(
      #[case] active_readarr_block: ActiveReadarrBlock,
      #[case] index: usize,
    ) {
      let mut app = App::test_default_fully_populated();
      app.data.readarr_data.books = StatefulTable::default();
      app.data.readarr_data.author_releases = StatefulTable::default();
      app.data.readarr_data.author_history = StatefulTable::default();
      app.push_navigation_stack(active_readarr_block.into());
      app.data.readarr_data.author_info_tabs.set_index(index);

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        AuthorDetailsUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(
        format!("empty_author_details_{active_readarr_block}"),
        output
      );
    }

    #[test]
    fn test_author_details_ui_renders_update_and_scan_prompt_over_author_details() {
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack(ActiveReadarrBlock::AuthorDetails.into());
      app.push_navigation_stack(ActiveReadarrBlock::UpdateAndScanAuthorPrompt.into());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        AuthorDetailsUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }

    #[test]
    fn test_author_details_ui_renders_automatic_search_prompt_over_author_details() {
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack(ActiveReadarrBlock::AuthorDetails.into());
      app.push_navigation_stack(ActiveReadarrBlock::AutomaticallySearchAuthorPrompt.into());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        AuthorDetailsUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }

    #[test]
    fn test_author_details_ui_renders_statistics() {
      let mut app = App::test_default_fully_populated();
      let mut author = app.data.readarr_data.authors.current_selection().clone();
      author.statistics = Some(AuthorStatistics {
        book_count: 3,
        book_file_count: 2,
        total_book_count: 3,
        available_book_count: 2,
        size_on_disk: 3543348019,
        percent_of_books: 66.6,
      });
      app.data.readarr_data.authors.set_items(vec![author]);
      let mut book = app.data.readarr_data.books.current_selection().clone();
      book.statistics = Some(BookStatistics {
        book_file_count: 1,
        total_book_count: 1,
        size_on_disk: 1771674009,
        percent_of_books: 100.0,
      });
      app.data.readarr_data.books.set_items(vec![book]);
      app.push_navigation_stack(ActiveReadarrBlock::AuthorDetails.into());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        AuthorDetailsUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }
  }
}
