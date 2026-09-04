#[cfg(test)]
mod tests {
  use chrono::{Duration, Utc};
  use pretty_assertions::assert_eq;
  use ratatui::widgets::{Cell, Row};
  use strum::IntoEnumIterator;

  use crate::models::readarr_models::{Book, BookStatistics};
  use crate::models::servarr_data::readarr::readarr_data::{
    AUTHOR_DETAILS_BLOCKS, ActiveReadarrBlock, BOOK_DETAILS_BLOCKS, DELETE_BOOK_BLOCKS,
    EDITION_DETAILS_BLOCKS,
  };
  use crate::ui::DrawUi;
  use crate::ui::readarr_ui::library::author_details_ui::{
    AuthorDetailsUi, decorate_book_row_with_style,
  };
  use crate::ui::styles::ManagarrStyle;

  #[test]
  fn test_author_details_ui_accepts() {
    let mut author_details_blocks = AUTHOR_DETAILS_BLOCKS.to_vec();
    author_details_blocks.extend(BOOK_DETAILS_BLOCKS);
    author_details_blocks.extend(EDITION_DETAILS_BLOCKS);
    author_details_blocks.extend(DELETE_BOOK_BLOCKS);

    ActiveReadarrBlock::iter().for_each(|active_readarr_block| {
      if author_details_blocks.contains(&active_readarr_block) {
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
    use crate::models::BlockSelectionState;
    use crate::models::readarr_models::{AuthorStatistics, BookStatistics};
    use crate::models::servarr_data::readarr::readarr_data::{
      ActiveReadarrBlock, DELETE_BOOK_SELECTION_BLOCKS,
    };
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

    #[test]
    fn test_author_details_ui_renders_every_field_despite_a_long_overview() {
      let mut app = App::test_default_fully_populated();
      let mut author = app.data.readarr_data.authors.current_selection().clone();
      author.overview = Some("Carl Edward Sagan was an American astronomer, planetary scientist, cosmologist, astrophysicist, astrobiologist, author, and science communicator. His best known scientific contribution is his research on the possibility of extraterrestrial life, including experimental demonstration of the production of amino acids from basic chemicals by radiation. He assembled the first physical messages sent into space, the Pioneer plaque and the Voyager Golden Record, universal messages that could potentially be understood by any extraterrestrial intelligence that might find them. He argued in favor of the hypothesis, which has become accepted, that the high surface temperatures of Venus are the result of the greenhouse effect.".to_owned());
      app.data.readarr_data.authors.set_items(vec![author]);
      app.push_navigation_stack(ActiveReadarrBlock::AuthorDetails.into());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        AuthorDetailsUi::draw(f, app, f.area());
      });

      assert_contains!(output, "Status:");
      assert_contains!(output, "Genres:");
      assert_contains!(output, "Rating:");
      assert_contains!(output, "Path:");
      assert_contains!(output, "Monitored:");
      assert_contains!(output, "Overview:");
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

    #[test]
    fn test_author_details_ui_renders_book_details_over_author_details() {
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack(ActiveReadarrBlock::BookDetails.into());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        AuthorDetailsUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }

    #[test]
    fn test_author_details_ui_renders_delete_book_over_author_details() {
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack(ActiveReadarrBlock::AuthorDetails.into());
      app.data.readarr_data.delete_files = true;
      app.data.readarr_data.add_import_list_exclusion = false;
      app.data.readarr_data.selected_block = BlockSelectionState::new(DELETE_BOOK_SELECTION_BLOCKS);
      app.push_navigation_stack(ActiveReadarrBlock::DeleteBookPrompt.into());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        AuthorDetailsUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }
  }
}
