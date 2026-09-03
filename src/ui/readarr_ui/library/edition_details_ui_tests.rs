#[cfg(test)]
mod tests {
  use pretty_assertions::assert_eq;
  use ratatui::text::Line;
  use strum::IntoEnumIterator;

  use crate::app::App;
  use crate::models::readarr_models::Edition;
  use crate::models::servarr_data::readarr::readarr_data::{
    ActiveReadarrBlock, EDITION_DETAILS_BLOCKS,
  };
  use crate::ui::DrawUi;
  use crate::ui::readarr_ui::library::edition_details_ui::{
    EditionDetailsUi, edition_detail_line, style_from_edition,
  };
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

  #[test]
  fn test_edition_detail_line_splits_a_label_from_its_value() {
    let line = edition_detail_line("Title: Test Edition", primary_style());

    assert_eq!(span_contents(&line), vec!["Title:", " Test Edition"]);
  }

  #[test]
  fn test_edition_detail_line_keeps_colons_within_a_value() {
    let line = edition_detail_line("Release Date: 2023-01-05 15:00:00 UTC", primary_style());

    assert_eq!(
      span_contents(&line),
      vec!["Release Date:", " 2023-01-05 15:00:00 UTC"]
    );
  }

  #[test]
  fn test_edition_detail_line_does_not_append_a_colon_to_a_line_without_one() {
    let line = edition_detail_line(
      "My name is Kvothe, pronounced nearly the same as \"quothe.\"",
      primary_style(),
    );

    assert_eq!(
      span_contents(&line),
      vec!["My name is Kvothe, pronounced nearly the same as \"quothe.\""]
    );
  }

  #[test]
  fn test_edition_detail_line_renders_a_carriage_return_only_line_as_blank() {
    let line = edition_detail_line("\r", primary_style());

    assert_eq!(span_contents(&line), vec![""]);
  }

  #[test]
  fn test_edition_detail_line_trims_a_trailing_carriage_return() {
    let line = edition_detail_line(
      "I have stolen princesses back from sleeping barrow kings.\r",
      primary_style(),
    );

    assert_eq!(
      span_contents(&line),
      vec!["I have stolen princesses back from sleeping barrow kings."]
    );
  }

  fn span_contents(line: &Line<'_>) -> Vec<String> {
    line
      .spans
      .iter()
      .map(|span| span.content.to_string())
      .collect()
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
