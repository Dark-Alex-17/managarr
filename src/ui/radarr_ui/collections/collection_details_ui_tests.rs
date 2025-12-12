#[cfg(test)]
mod tests {
  use bimap::BiMap;
  use strum::IntoEnumIterator;

  use crate::app::App;
  use crate::models::radarr_models::{Collection, CollectionMovie};
  use crate::models::servarr_data::radarr::radarr_data::{
    ActiveRadarrBlock, COLLECTION_DETAILS_BLOCKS,
  };
  use crate::models::stateful_table::StatefulTable;
  use crate::ui::DrawUi;
  use crate::ui::radarr_ui::collections::collection_details_ui::CollectionDetailsUi;
  use crate::ui::ui_test_utils::test_utils::render_to_string_with_app;

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
  fn test_collection_details_ui_renders_collection_details() {
    let mut app = App::test_default();
    app.push_navigation_stack(ActiveRadarrBlock::CollectionDetails.into());
    app.data.radarr_data.quality_profile_map = BiMap::from_iter(vec![(1, "HD - 1080p".to_owned())]);
    app.data.radarr_data.collections = StatefulTable::default();
    app.data.radarr_data.collections.set_items(vec![Collection {
      id: 1,
      title: "Test Collection".into(),
      quality_profile_id: 1,
      movies: Some(vec![CollectionMovie {
        title: "Movie 1".into(),
        ..CollectionMovie::default()
      }]),
      ..Collection::default()
    }]);

    let output = render_to_string_with_app(120, 30, &mut app, |f, app| {
      CollectionDetailsUi::draw(f, app, f.area());
    });

    insta::assert_snapshot!(output);
  }
}
