#[cfg(test)]
mod tests {
  use strum::IntoEnumIterator;

  use crate::app::App;
  use crate::models::BlockSelectionState;
  use crate::models::servarr_data::radarr::radarr_data::{
    ActiveRadarrBlock, EDIT_COLLECTION_BLOCKS, EDIT_COLLECTION_SELECTION_BLOCKS,
  };
  use crate::ui::DrawUi;
  use crate::ui::radarr_ui::collections::edit_collection_ui::EditCollectionUi;
  use crate::ui::ui_test_utils::test_utils::{TerminalSize, render_to_string_with_app};

  #[test]
  fn test_edit_collection_ui_accepts() {
    ActiveRadarrBlock::iter().for_each(|active_radarr_block| {
      if EDIT_COLLECTION_BLOCKS.contains(&active_radarr_block) {
        assert!(EditCollectionUi::accepts(active_radarr_block.into()));
      } else {
        assert!(!EditCollectionUi::accepts(active_radarr_block.into()));
      }
    });

    assert!(EditCollectionUi::accepts(
      (
        ActiveRadarrBlock::EditCollectionPrompt,
        Some(ActiveRadarrBlock::CollectionDetails)
      )
        .into()
    ));
  }

  mod snapshot_tests {
    use rstest::rstest;
    use super::*;

    #[rstest]
    #[case(ActiveRadarrBlock::Collections, ActiveRadarrBlock::EditCollectionPrompt)]
    #[case(ActiveRadarrBlock::Collections, ActiveRadarrBlock::EditCollectionConfirmPrompt)]
    #[case(ActiveRadarrBlock::Collections, ActiveRadarrBlock::EditCollectionRootFolderPathInput)]
    #[case(ActiveRadarrBlock::Collections, ActiveRadarrBlock::EditCollectionSelectMinimumAvailability)]
    #[case(ActiveRadarrBlock::Collections, ActiveRadarrBlock::EditCollectionSelectQualityProfile)]
    #[case(ActiveRadarrBlock::Collections, ActiveRadarrBlock::EditCollectionToggleSearchOnAdd)]
    #[case(ActiveRadarrBlock::Collections, ActiveRadarrBlock::EditCollectionToggleMonitored)]
    #[case(ActiveRadarrBlock::CollectionDetails, ActiveRadarrBlock::EditCollectionPrompt)]
    #[case(ActiveRadarrBlock::CollectionDetails, ActiveRadarrBlock::EditCollectionConfirmPrompt)]
    #[case(ActiveRadarrBlock::CollectionDetails, ActiveRadarrBlock::EditCollectionRootFolderPathInput)]
    #[case(ActiveRadarrBlock::CollectionDetails, ActiveRadarrBlock::EditCollectionSelectMinimumAvailability)]
    #[case(ActiveRadarrBlock::CollectionDetails, ActiveRadarrBlock::EditCollectionSelectQualityProfile)]
    #[case(ActiveRadarrBlock::CollectionDetails, ActiveRadarrBlock::EditCollectionToggleSearchOnAdd)]
    #[case(ActiveRadarrBlock::CollectionDetails, ActiveRadarrBlock::EditCollectionToggleMonitored)]
    fn test_edit_collection_ui_renders_edit_collection_modal(
      #[case] active_radarr_block: ActiveRadarrBlock,
      #[case] context_block: ActiveRadarrBlock,
    ) {
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack((active_radarr_block, Some(context_block)).into());
      app.data.radarr_data.selected_block =
        BlockSelectionState::new(EDIT_COLLECTION_SELECTION_BLOCKS);

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        EditCollectionUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(format!("{}_{}", active_radarr_block.to_string(), context_block.to_string()), output);
    }
  }
}