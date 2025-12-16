#[cfg(test)]
mod tests {
  use strum::IntoEnumIterator;

  use crate::app::App;
  use crate::models::servarr_data::radarr::radarr_data::{
    ActiveRadarrBlock, COLLECTION_DETAILS_BLOCKS,
  };
  use crate::ui::DrawUi;
  use crate::ui::radarr_ui::collections::collection_details_ui::CollectionDetailsUi;
  use crate::ui::ui_test_utils::test_utils::{TerminalSize, render_to_string_with_app};

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

  mod snapshot_tests {
    use rstest::rstest;
    use crate::models::stateful_table::StatefulTable;
    use super::*;

    #[rstest]
    fn test_collection_details_ui_renders_collection_details(
      #[values(
      ActiveRadarrBlock::CollectionDetails,
      ActiveRadarrBlock::ViewMovieOverview
      )] active_radarr_block: ActiveRadarrBlock
    ) {
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack(active_radarr_block.into());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        CollectionDetailsUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(active_radarr_block.to_string(), output);
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