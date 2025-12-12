#[cfg(test)]
mod tests {
  use bimap::BiMap;
  use strum::IntoEnumIterator;

  use crate::app::App;
  use crate::models::BlockSelectionState;
  use crate::models::radarr_models::Collection;
  use crate::models::servarr_data::radarr::modals::EditCollectionModal;
  use crate::models::servarr_data::radarr::radarr_data::{
    ActiveRadarrBlock, EDIT_COLLECTION_BLOCKS, EDIT_COLLECTION_SELECTION_BLOCKS,
  };
  use crate::models::stateful_table::StatefulTable;
  use crate::ui::DrawUi;
  use crate::ui::radarr_ui::collections::edit_collection_ui::EditCollectionUi;
  use crate::ui::ui_test_utils::test_utils::render_to_string_with_app;

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

  #[test]
  fn test_edit_collection_ui_renders_edit_collection_modal() {
    let mut app = App::test_default();
    app.push_navigation_stack(ActiveRadarrBlock::EditCollectionRootFolderPathInput.into());
    app.data.radarr_data.quality_profile_map = BiMap::from_iter(vec![(1, "HD - 1080p".to_owned())]);
    app.data.radarr_data.collections = StatefulTable::default();
    app.data.radarr_data.collections.set_items(vec![Collection {
      id: 1,
      title: "Test Collection".into(),
      quality_profile_id: 1,
      root_folder_path: Some("/movies".to_owned()),
      ..Collection::default()
    }]);
    app.data.radarr_data.selected_block =
      BlockSelectionState::new(EDIT_COLLECTION_SELECTION_BLOCKS);
    app.data.radarr_data.edit_collection_modal =
      Some(EditCollectionModal::from(&app.data.radarr_data));

    let output = render_to_string_with_app(120, 30, &mut app, |f, app| {
      EditCollectionUi::draw(f, app, f.area());
    });

    insta::assert_snapshot!(output);
  }
}
