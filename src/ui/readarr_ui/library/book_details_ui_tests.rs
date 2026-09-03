#[cfg(test)]
mod tests {
  use pretty_assertions::assert_eq;
  use ratatui::widgets::{Cell, Row};
  use strum::IntoEnumIterator;

  use crate::app::App;
  use crate::models::readarr_models::Edition;
  use crate::models::servarr_data::readarr::readarr_data::{
    ActiveReadarrBlock, BOOK_DETAILS_BLOCKS,
  };
  use crate::ui::DrawUi;
  use crate::ui::readarr_ui::library::book_details_ui::{
    BookDetailsUi, decorate_edition_row_with_style,
  };
  use crate::ui::styles::ManagarrStyle;
  use crate::ui::ui_test_utils::test_utils::render_to_string_with_app;

  #[test]
  fn test_book_details_ui_accepts() {
    ActiveReadarrBlock::iter().for_each(|active_readarr_block| {
      if BOOK_DETAILS_BLOCKS.contains(&active_readarr_block) {
        assert!(BookDetailsUi::accepts(active_readarr_block.into()));
      } else {
        assert!(!BookDetailsUi::accepts(active_readarr_block.into()));
      }
    });
  }

  #[test]
  fn test_decorate_edition_row_with_style_monitored() {
    let edition = Edition {
      monitored: true,
      ..Edition::default()
    };
    let row = Row::new(vec![Cell::from("test".to_owned())]);

    let style = decorate_edition_row_with_style(&edition, row.clone());

    assert_eq!(style, row.primary());
  }

  #[test]
  fn test_decorate_edition_row_with_style_unmonitored() {
    let edition = Edition {
      monitored: false,
      ..Edition::default()
    };
    let row = Row::new(vec![Cell::from("test".to_owned())]);

    let style = decorate_edition_row_with_style(&edition, row.clone());

    assert_eq!(style, row.unmonitored());
  }

  mod snapshot_tests {
    use crate::models::stateful_table::StatefulTable;
    use crate::ui::ui_test_utils::test_utils::TerminalSize;
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(ActiveReadarrBlock::BookDetails, 0)]
    #[case(ActiveReadarrBlock::SearchEditions, 0)]
    #[case(ActiveReadarrBlock::SearchEditionsError, 0)]
    #[case(ActiveReadarrBlock::AutomaticallySearchBookPrompt, 0)]
    #[case(ActiveReadarrBlock::AutomaticallySearchBookPrompt, 1)]
    #[case(ActiveReadarrBlock::AutomaticallySearchBookPrompt, 2)]
    #[case(ActiveReadarrBlock::AutomaticallySearchBookPrompt, 3)]
    #[case(ActiveReadarrBlock::DeleteBookFilePrompt, 0)]
    #[case(ActiveReadarrBlock::DeleteBookFilePrompt, 2)]
    #[case(ActiveReadarrBlock::BookHistory, 1)]
    #[case(ActiveReadarrBlock::SearchBookHistory, 1)]
    #[case(ActiveReadarrBlock::SearchBookHistoryError, 1)]
    #[case(ActiveReadarrBlock::FilterBookHistory, 1)]
    #[case(ActiveReadarrBlock::FilterBookHistoryError, 1)]
    #[case(ActiveReadarrBlock::BookHistorySortPrompt, 1)]
    #[case(ActiveReadarrBlock::BookHistoryDetails, 1)]
    #[case(ActiveReadarrBlock::BookFileInfo, 2)]
    #[case(ActiveReadarrBlock::ManualBookSearch, 3)]
    #[case(ActiveReadarrBlock::ManualBookSearchConfirmPrompt, 3)]
    #[case(ActiveReadarrBlock::ManualBookSearchSortPrompt, 3)]
    fn test_book_details_ui_renders(
      #[case] active_readarr_block: ActiveReadarrBlock,
      #[case] index: usize,
    ) {
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack(active_readarr_block.into());
      app
        .data
        .readarr_data
        .book_details_modal
        .as_mut()
        .unwrap()
        .book_details_tabs
        .set_index(index);

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        BookDetailsUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(
        format!("book_details_renders_{active_readarr_block}_{index}"),
        output
      );
    }

    #[rstest]
    #[case(ActiveReadarrBlock::BookDetails, 0)]
    #[case(ActiveReadarrBlock::BookHistory, 1)]
    #[case(ActiveReadarrBlock::BookFileInfo, 2)]
    #[case(ActiveReadarrBlock::ManualBookSearch, 3)]
    fn test_book_details_ui_renders_loading(
      #[case] active_readarr_block: ActiveReadarrBlock,
      #[case] index: usize,
    ) {
      let mut app = App::test_default_fully_populated();
      app.is_loading = true;
      app.push_navigation_stack(active_readarr_block.into());
      {
        let book_details_modal = app.data.readarr_data.book_details_modal.as_mut().unwrap();
        book_details_modal.editions = StatefulTable::default();
        book_details_modal.book_files = StatefulTable::default();
        book_details_modal.book_history = StatefulTable::default();
        book_details_modal.book_releases = StatefulTable::default();
        book_details_modal.book_details_tabs.set_index(index);
      }

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        BookDetailsUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(
        format!("loading_book_details_{active_readarr_block}_{index}"),
        output
      );
    }

    #[rstest]
    #[case(ActiveReadarrBlock::BookDetails, 0)]
    #[case(ActiveReadarrBlock::BookHistory, 1)]
    #[case(ActiveReadarrBlock::BookFileInfo, 2)]
    #[case(ActiveReadarrBlock::ManualBookSearch, 3)]
    fn test_book_details_ui_renders_empty(
      #[case] active_readarr_block: ActiveReadarrBlock,
      #[case] index: usize,
    ) {
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack(active_readarr_block.into());
      {
        let book_details_modal = app.data.readarr_data.book_details_modal.as_mut().unwrap();
        book_details_modal.editions = StatefulTable::default();
        book_details_modal.book_files = StatefulTable::default();
        book_details_modal.book_history = StatefulTable::default();
        book_details_modal.book_releases = StatefulTable::default();
        book_details_modal.book_details_tabs.set_index(index);
      }

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        BookDetailsUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(
        format!("empty_book_details_{active_readarr_block}_{index}"),
        output
      );
    }

    #[test]
    fn test_book_details_ui_renders_editions_table_with_zero_editions() {
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack(ActiveReadarrBlock::BookDetails.into());
      app
        .data
        .readarr_data
        .book_details_modal
        .as_mut()
        .unwrap()
        .editions = StatefulTable::default();

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        BookDetailsUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }

    #[test]
    fn test_book_details_ui_renders_editions_table_with_a_single_edition() {
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack(ActiveReadarrBlock::BookDetails.into());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        BookDetailsUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }

    #[test]
    fn test_book_details_ui_renders_editions_table_with_an_unmonitored_edition() {
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack(ActiveReadarrBlock::BookDetails.into());
      app
        .data
        .readarr_data
        .book_details_modal
        .as_mut()
        .unwrap()
        .editions
        .set_items(vec![Edition {
          monitored: false,
          format: Some("Paperback".to_owned()),
          isbn13: Some("9780756404741".to_owned()),
          asin: Some("B0043RSJ9S".to_owned()),
          publisher: Some("DAW Books".to_owned()),
          page_count: Some(662),
          ..Edition::default()
        }]);

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        BookDetailsUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }

    #[test]
    fn test_book_details_ui_renders_nothing_when_the_book_details_modal_is_undefined() {
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack(ActiveReadarrBlock::BookDetails.into());
      app.data.readarr_data.book_details_modal = None;

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        BookDetailsUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }
  }
}
