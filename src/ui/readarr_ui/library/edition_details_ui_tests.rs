#[cfg(test)]
mod tests {
  use pretty_assertions::assert_eq;
  use strum::IntoEnumIterator;

  use crate::app::App;
  use crate::models::readarr_models::Edition;
  use crate::models::servarr_data::readarr::readarr_data::{
    ActiveReadarrBlock, EDITION_DETAILS_BLOCKS,
  };
  use crate::ui::DrawUi;
  use crate::ui::readarr_ui::library::edition_details_ui::{EditionDetailsUi, style_from_edition};
  use crate::ui::styles::{primary_style, unmonitored_style};
  use crate::ui::ui_test_utils::test_utils::render_to_string_with_app;

  #[test]
  fn test_edition_details_ui_accepts() {
    ActiveReadarrBlock::iter().for_each(|active_readarr_block| {
      if EDITION_DETAILS_BLOCKS.contains(&active_readarr_block) {
        assert!(EditionDetailsUi::accepts(active_readarr_block.into()));
      } else {
        assert!(!EditionDetailsUi::accepts(active_readarr_block.into()));
      }
    });
  }

  #[test]
  fn test_style_from_edition_monitored() {
    let edition = Edition {
      monitored: true,
      ..Edition::default()
    };

    let style = style_from_edition(&edition);

    assert_eq!(style, primary_style());
  }

  #[test]
  fn test_style_from_edition_unmonitored() {
    let edition = Edition {
      monitored: false,
      ..Edition::default()
    };

    let style = style_from_edition(&edition);

    assert_eq!(style, unmonitored_style());
  }

  mod snapshot_tests {
    use crate::models::servarr_data::readarr::modals::EditionDetailsModal;
    use crate::models::stateful_table::StatefulTable;
    use crate::ui::ui_test_utils::test_utils::TerminalSize;

    use super::*;

    #[test]
    fn test_edition_details_ui_renders() {
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack(ActiveReadarrBlock::BookEditionDetails.into());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        EditionDetailsUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }

    #[test]
    fn test_edition_details_ui_renders_loading() {
      let mut app = App::test_default_fully_populated();
      app.is_loading = true;
      app.push_navigation_stack(ActiveReadarrBlock::BookEditionDetails.into());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        EditionDetailsUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }

    #[test]
    fn test_edition_details_ui_renders_empty() {
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack(ActiveReadarrBlock::BookEditionDetails.into());
      app
        .data
        .readarr_data
        .book_details_modal
        .as_mut()
        .unwrap()
        .edition_details_modal = Some(EditionDetailsModal::default());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        EditionDetailsUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }

    #[test]
    fn test_edition_details_ui_renders_with_zero_editions() {
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack(ActiveReadarrBlock::BookEditionDetails.into());
      app
        .data
        .readarr_data
        .book_details_modal
        .as_mut()
        .unwrap()
        .editions = StatefulTable::default();

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        EditionDetailsUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }

    #[test]
    fn test_edition_details_ui_renders_scrolled() {
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack(ActiveReadarrBlock::BookEditionDetails.into());
      app
        .data
        .readarr_data
        .book_details_modal
        .as_mut()
        .unwrap()
        .edition_details_modal
        .as_mut()
        .unwrap()
        .edition_details
        .offset = 2;

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        EditionDetailsUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }

    #[test]
    fn test_edition_details_ui_renders_nothing_when_the_edition_details_modal_is_undefined() {
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack(ActiveReadarrBlock::BookEditionDetails.into());
      app
        .data
        .readarr_data
        .book_details_modal
        .as_mut()
        .unwrap()
        .edition_details_modal = None;

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        EditionDetailsUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }

    #[test]
    fn test_edition_details_ui_renders_nothing_when_the_book_details_modal_is_undefined() {
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack(ActiveReadarrBlock::BookEditionDetails.into());
      app.data.readarr_data.book_details_modal = None;

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        EditionDetailsUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }
  }
}
