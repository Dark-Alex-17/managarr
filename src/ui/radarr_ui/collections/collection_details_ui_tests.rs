#[cfg(test)]
mod tests {
  use pretty_assertions::assert_str_eq;
  use strum::IntoEnumIterator;

  use crate::app::App;
  use crate::models::servarr_data::radarr::radarr_data::{
    ActiveRadarrBlock, COLLECTION_DETAILS_BLOCKS,
  };
  use crate::ui::DrawUi;
  use crate::ui::radarr_ui::collections::collection_details_ui::{
    CollectionDetailsUi, overview_line,
  };
  use crate::ui::ui_test_utils::test_utils::{TerminalSize, render_to_string_with_app};

  #[test]
  fn test_overview_line_trims_a_trailing_carriage_return() {
    let line = overview_line("The sequel took seven years to finish.\r");

    assert_str_eq!(
      line.spans[0].content,
      "The sequel took seven years to finish."
    );
  }

  #[test]
  fn test_overview_line_renders_a_carriage_return_only_line_as_blank() {
    let line = overview_line("\r");

    assert_str_eq!(line.spans[0].content, "");
  }

  #[test]
  fn test_overview_line_does_not_split_a_label_from_a_colon_in_the_prose() {
    let line = overview_line("It was shot in Madison, Wisconsin: a city the cast never left.");

    assert_eq!(line.spans.len(), 1);
    assert_str_eq!(
      line.spans[0].content,
      "It was shot in Madison, Wisconsin: a city the cast never left."
    );
  }

  #[test]
  fn test_collection_details_ui_accepts() {
    ActiveRadarrBlock::iter().for_each(|active_radarr_block| {
      if COLLECTION_DETAILS_BLOCKS.contains(&active_radarr_block) {
        assert!(CollectionDetailsUi::accepts(active_radarr_block.into()));
      } else {
        assert!(!CollectionDetailsUi::accepts(active_radarr_block.into()));
      }
    });

    assert!(CollectionDetailsUi::accepts(
      (
        ActiveRadarrBlock::CollectionDetails,
        Some(ActiveRadarrBlock::CollectionDetails)
      )
        .into()
    ));
    assert!(CollectionDetailsUi::accepts(
      (
        ActiveRadarrBlock::AddMoviePrompt,
        Some(ActiveRadarrBlock::CollectionDetails)
      )
        .into()
    ));
  }

  #[test]
  fn test_scrolling_the_movie_overview_does_not_scroll_the_collection_details_behind_it() {
    let mut app = App::test_default_fully_populated();
    app.push_navigation_stack(ActiveRadarrBlock::CollectionDetails.into());

    let before = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
      CollectionDetailsUi::draw(f, app, f.area());
    });

    app
      .data
      .radarr_data
      .movie_overview_modal
      .as_mut()
      .unwrap()
      .overview
      .offset = 3;

    let after = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
      CollectionDetailsUi::draw(f, app, f.area());
    });

    assert_contains!(before, "Overview: Collection blah blah blah");
    assert_str_eq!(before, after);
  }

  mod snapshot_tests {
    use super::*;
    use crate::models::stateful_table::StatefulTable;
    use rstest::rstest;

    #[rstest]
    fn test_collection_details_ui_renders_collection_details(
      #[values(
        ActiveRadarrBlock::CollectionDetails,
        ActiveRadarrBlock::ViewMovieOverview
      )]
      active_radarr_block: ActiveRadarrBlock,
    ) {
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack(active_radarr_block.into());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        CollectionDetailsUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(active_radarr_block.to_string(), output);
    }

    #[test]
    fn test_collection_details_ui_renders_scrolled_movie_overview() {
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack(ActiveRadarrBlock::ViewMovieOverview.into());
      app
        .data
        .radarr_data
        .movie_overview_modal
        .as_mut()
        .unwrap()
        .overview
        .offset = 2;

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        CollectionDetailsUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }

    #[test]
    fn test_collection_details_ui_renders_movie_overview_loading() {
      let mut app = App::test_default_fully_populated();
      app.is_loading = true;
      app.push_navigation_stack(ActiveRadarrBlock::ViewMovieOverview.into());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        CollectionDetailsUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }

    #[test]
    fn test_collection_details_ui_renders_movie_overview_when_the_modal_is_absent() {
      let mut app = App::test_default_fully_populated();
      app.data.radarr_data.movie_overview_modal = None;
      app.push_navigation_stack(ActiveRadarrBlock::ViewMovieOverview.into());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        CollectionDetailsUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }

    #[test]
    fn test_collection_details_ui_renders_collection_details_empty() {
      let mut app = App::test_default_fully_populated();
      app.data.radarr_data.collection_movies = StatefulTable::default();
      app.push_navigation_stack(ActiveRadarrBlock::CollectionDetails.into());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        CollectionDetailsUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }
  }
}
