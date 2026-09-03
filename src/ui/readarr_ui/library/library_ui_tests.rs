#[cfg(test)]
mod tests {
  use strum::IntoEnumIterator;

  use crate::models::readarr_models::{Author, AuthorStatistics, AuthorStatus};
  use crate::models::servarr_data::readarr::readarr_data::{
    AUTHOR_DETAILS_BLOCKS, ActiveReadarrBlock, LIBRARY_BLOCKS,
  };
  use crate::ui::DrawUi;
  use crate::ui::readarr_ui::library::{LibraryUi, decorate_author_row_with_style};
  use crate::ui::styles::ManagarrStyle;
  use pretty_assertions::assert_eq;
  use ratatui::widgets::{Cell, Row};

  #[test]
  fn test_library_ui_accepts() {
    let mut blocks = LIBRARY_BLOCKS.to_vec();
    blocks.extend(AUTHOR_DETAILS_BLOCKS);

    for active_readarr_block in ActiveReadarrBlock::iter() {
      if blocks.contains(&active_readarr_block) {
        assert!(
          LibraryUi::accepts(active_readarr_block.into()),
          "{active_readarr_block} is not accepted by the LibraryUi"
        );
      } else {
        assert!(
          !LibraryUi::accepts(active_readarr_block.into()),
          "{active_readarr_block} should not be accepted by LibraryUi"
        );
      }
    }
  }

  #[test]
  fn test_decorate_row_with_style_unmonitored() {
    let author = Author::default();
    let row = Row::new(vec![Cell::from("test".to_owned())]);

    let style = decorate_author_row_with_style(&author, row.clone());

    assert_eq!(style, row.unmonitored());
  }

  #[test]
  fn test_decorate_row_with_style_downloaded_when_ended_and_all_books_present() {
    let author = Author {
      monitored: true,
      status: AuthorStatus::Ended,
      statistics: Some(AuthorStatistics {
        book_file_count: 10,
        total_book_count: 10,
        ..AuthorStatistics::default()
      }),
      ..Author::default()
    };
    let row = Row::new(vec![Cell::from("test".to_owned())]);

    let style = decorate_author_row_with_style(&author, row.clone());

    assert_eq!(style, row.downloaded());
  }

  #[test]
  fn test_decorate_row_with_style_missing_when_ended_and_books_are_missing() {
    let author = Author {
      monitored: true,
      status: AuthorStatus::Ended,
      statistics: Some(AuthorStatistics {
        book_file_count: 5,
        total_book_count: 10,
        ..AuthorStatistics::default()
      }),
      ..Author::default()
    };
    let row = Row::new(vec![Cell::from("test".to_owned())]);

    let style = decorate_author_row_with_style(&author, row.clone());

    assert_eq!(style, row.missing());
  }

  #[test]
  fn test_decorate_row_with_style_indeterminate_when_ended_and_no_statistics() {
    let author = Author {
      monitored: true,
      status: AuthorStatus::Ended,
      statistics: None,
      ..Author::default()
    };
    let row = Row::new(vec![Cell::from("test".to_owned())]);

    let style = decorate_author_row_with_style(&author, row.clone());

    assert_eq!(style, row.indeterminate());
  }

  #[test]
  fn test_decorate_row_with_style_indeterminate_when_ended_and_total_book_count_is_zero() {
    let author = Author {
      monitored: true,
      status: AuthorStatus::Ended,
      statistics: Some(AuthorStatistics {
        book_file_count: 0,
        total_book_count: 0,
        ..AuthorStatistics::default()
      }),
      ..Author::default()
    };
    let row = Row::new(vec![Cell::from("test".to_owned())]);

    let style = decorate_author_row_with_style(&author, row.clone());

    assert_eq!(style, row.missing());
  }

  #[test]
  fn test_decorate_row_with_style_unreleased_when_continuing_and_all_books_present() {
    let author = Author {
      monitored: true,
      status: AuthorStatus::Continuing,
      statistics: Some(AuthorStatistics {
        book_file_count: 10,
        total_book_count: 10,
        ..AuthorStatistics::default()
      }),
      ..Author::default()
    };
    let row = Row::new(vec![Cell::from("test".to_owned())]);

    let style = decorate_author_row_with_style(&author, row.clone());

    assert_eq!(style, row.unreleased());
  }

  #[test]
  fn test_decorate_row_with_style_missing_when_continuing_and_books_are_missing() {
    let author = Author {
      monitored: true,
      status: AuthorStatus::Continuing,
      statistics: Some(AuthorStatistics {
        book_file_count: 5,
        total_book_count: 10,
        ..AuthorStatistics::default()
      }),
      ..Author::default()
    };
    let row = Row::new(vec![Cell::from("test".to_owned())]);

    let style = decorate_author_row_with_style(&author, row.clone());

    assert_eq!(style, row.missing());
  }

  #[test]
  fn test_decorate_row_with_style_indeterminate_when_continuing_and_no_statistics() {
    let author = Author {
      monitored: true,
      status: AuthorStatus::Continuing,
      statistics: None,
      ..Author::default()
    };
    let row = Row::new(vec![Cell::from("test".to_owned())]);

    let style = decorate_author_row_with_style(&author, row.clone());

    assert_eq!(style, row.indeterminate());
  }

  #[test]
  fn test_decorate_row_with_style_defaults_to_indeterminate_for_deleted_status() {
    let author = Author {
      monitored: true,
      status: AuthorStatus::Deleted,
      ..Author::default()
    };
    let row = Row::new(vec![Cell::from("test".to_owned())]);

    let style = decorate_author_row_with_style(&author, row.clone());

    assert_eq!(style, row.indeterminate());
  }

  mod snapshot_tests {
    use crate::app::App;
    use crate::models::servarr_data::readarr::readarr_data::ActiveReadarrBlock;
    use rstest::rstest;

    use crate::ui::DrawUi;
    use crate::ui::readarr_ui::library::LibraryUi;
    use crate::ui::ui_test_utils::test_utils::{TerminalSize, render_to_string_with_app};

    #[rstest]
    fn test_library_ui_renders(
      #[values(
        ActiveReadarrBlock::Authors,
        ActiveReadarrBlock::AuthorsSortPrompt,
        ActiveReadarrBlock::SearchAuthors,
        ActiveReadarrBlock::SearchAuthorsError,
        ActiveReadarrBlock::FilterAuthors,
        ActiveReadarrBlock::FilterAuthorsError
      )]
      active_readarr_block: ActiveReadarrBlock,
    ) {
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack(active_readarr_block.into());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        LibraryUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(format!("readarr_library_{active_readarr_block}"), output);
    }

    #[test]
    fn test_library_ui_renders_loading() {
      let mut app = App::test_default_fully_populated();
      app.is_loading = true;
      app.push_navigation_stack(ActiveReadarrBlock::Authors.into());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        LibraryUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }

    #[test]
    fn test_library_ui_renders_empty() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveReadarrBlock::Authors.into());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        LibraryUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }

    #[test]
    fn test_library_ui_renders_author_details_over_library() {
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack(ActiveReadarrBlock::Authors.into());
      app.push_navigation_stack(ActiveReadarrBlock::AuthorDetails.into());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        LibraryUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }
  }
}
