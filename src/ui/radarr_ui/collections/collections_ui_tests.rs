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
  use crate::ui::ui_test_utils::test_utils::render_to_string_with_app;

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

  #[test]
  fn test_collections_ui_renders_loading_state() {
    let mut app = App::test_default();
    app.is_loading = true;
    app.push_navigation_stack(ActiveRadarrBlock::Collections.into());

    let output = render_to_string_with_app(120, 30, &mut app, |f, app| {
      CollectionsUi::draw(f, app, f.area());
    });

    insta::assert_snapshot!(output);
  }

  #[test]
  fn test_collections_ui_renders_empty_collections() {
    let mut app = App::test_default();
    app.push_navigation_stack(ActiveRadarrBlock::Collections.into());
    app.data.radarr_data.collections = StatefulTable::default();

    let output = render_to_string_with_app(120, 30, &mut app, |f, app| {
      CollectionsUi::draw(f, app, f.area());
    });

    insta::assert_snapshot!(output);
  }
}
