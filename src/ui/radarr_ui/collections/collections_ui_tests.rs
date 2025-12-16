#[cfg(test)]
mod tests {
  use strum::IntoEnumIterator;

  use crate::app::App;

  use crate::models::servarr_data::radarr::radarr_data::{
    ActiveRadarrBlock, COLLECTION_DETAILS_BLOCKS, COLLECTIONS_BLOCKS, EDIT_COLLECTION_BLOCKS,
  };
  use crate::models::stateful_table::StatefulTable;
  use crate::ui::DrawUi;
  use crate::ui::radarr_ui::collections::CollectionsUi;
  use crate::ui::ui_test_utils::test_utils::{TerminalSize, render_to_string_with_app};

  #[test]
  fn test_collections_ui_accepts() {
    let mut collections_ui_blocks = Vec::new();
    collections_ui_blocks.extend(COLLECTIONS_BLOCKS);
    collections_ui_blocks.extend(COLLECTION_DETAILS_BLOCKS);
    collections_ui_blocks.extend(EDIT_COLLECTION_BLOCKS);

    ActiveRadarrBlock::iter().for_each(|active_radarr_block| {
      if collections_ui_blocks.contains(&active_radarr_block) {
        assert!(CollectionsUi::accepts(active_radarr_block.into()));
      } else {
        assert!(!CollectionsUi::accepts(active_radarr_block.into()));
      }
    });
  }

  mod snapshot_tests {
    use super::*;
    use crate::models::BlockSelectionState;
    use crate::models::servarr_data::radarr::radarr_data::EDIT_COLLECTION_SELECTION_BLOCKS;
    use rstest::rstest;

    #[rstest]
    #[case(true, false, false)]
    #[case(false, true, false)]
    #[case(false, false, true)]
    fn test_radarr_ui_renders_collections_tab_loading(
      #[case] is_loading: bool,
      #[case] empty_movies: bool,
      #[case] empty_profile_map: bool,
    ) {
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack(ActiveRadarrBlock::Collections.into());
      app.is_loading = is_loading;
      if empty_movies {
        app.data.radarr_data.movies = StatefulTable::default();
      }

      if empty_profile_map {
        app.data.radarr_data.quality_profile_map = Default::default();
      }

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        CollectionsUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }

    #[rstest]
    fn test_radarr_ui_renders_collections_tab(
      #[values(
        ActiveRadarrBlock::Collections,
        ActiveRadarrBlock::CollectionsSortPrompt,
        ActiveRadarrBlock::FilterCollections,
        ActiveRadarrBlock::FilterCollectionsError,
        ActiveRadarrBlock::SearchCollection,
        ActiveRadarrBlock::SearchCollectionError,
        ActiveRadarrBlock::UpdateAllCollectionsPrompt
      )]
      active_radarr_block: ActiveRadarrBlock,
    ) {
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack(active_radarr_block.into());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        CollectionsUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(active_radarr_block.to_string(), output);
    }

    #[test]
    fn test_radarr_ui_renders_collections_tab_empty() {
      let mut app = App::test_default_fully_populated();
      app.data.radarr_data.collections = StatefulTable::default();
      app.push_navigation_stack(ActiveRadarrBlock::Collections.into());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        CollectionsUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }

    #[rstest]
    #[case(
      ActiveRadarrBlock::CollectionDetails,
      ActiveRadarrBlock::EditCollectionPrompt
    )]
    #[case(
      ActiveRadarrBlock::CollectionDetails,
      ActiveRadarrBlock::EditCollectionConfirmPrompt
    )]
    #[case(
      ActiveRadarrBlock::CollectionDetails,
      ActiveRadarrBlock::EditCollectionRootFolderPathInput
    )]
    #[case(
      ActiveRadarrBlock::CollectionDetails,
      ActiveRadarrBlock::EditCollectionSelectMinimumAvailability
    )]
    #[case(
      ActiveRadarrBlock::CollectionDetails,
      ActiveRadarrBlock::EditCollectionSelectQualityProfile
    )]
    #[case(
      ActiveRadarrBlock::CollectionDetails,
      ActiveRadarrBlock::EditCollectionToggleSearchOnAdd
    )]
    #[case(
      ActiveRadarrBlock::CollectionDetails,
      ActiveRadarrBlock::EditCollectionToggleMonitored
    )]
    fn test_edit_collection_ui_renders_edit_collection_modal(
      #[case] context_block: ActiveRadarrBlock,
      #[case] active_radarr_block: ActiveRadarrBlock,
    ) {
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack((active_radarr_block, Some(context_block)).into());
      app.data.radarr_data.selected_block =
        BlockSelectionState::new(EDIT_COLLECTION_SELECTION_BLOCKS);

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        CollectionsUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(
        format!(
          "{}_{}",
          active_radarr_block.to_string(),
          context_block.to_string()
        ),
        output
      );
    }
  }
}
