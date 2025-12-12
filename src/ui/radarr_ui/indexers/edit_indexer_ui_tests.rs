#[cfg(test)]
mod tests {
  use strum::IntoEnumIterator;

  use crate::app::App;
  use crate::models::BlockSelectionState;
  use crate::models::servarr_data::modals::EditIndexerModal;
  use crate::models::servarr_data::radarr::radarr_data::{
    ActiveRadarrBlock, EDIT_INDEXER_BLOCKS, EDIT_INDEXER_TORRENT_SELECTION_BLOCKS,
  };
  use crate::models::servarr_models::{Indexer, IndexerField};
  use crate::models::stateful_table::StatefulTable;
  use crate::ui::DrawUi;
  use crate::ui::radarr_ui::indexers::edit_indexer_ui::EditIndexerUi;
  use crate::ui::ui_test_utils::test_utils::render_to_string_with_app;
  use serde_json::json;

  #[test]
  fn test_edit_indexer_ui_accepts() {
    ActiveRadarrBlock::iter().for_each(|active_radarr_block| {
      if EDIT_INDEXER_BLOCKS.contains(&active_radarr_block) {
        assert!(EditIndexerUi::accepts(active_radarr_block.into()));
      } else {
        assert!(!EditIndexerUi::accepts(active_radarr_block.into()));
      }
    });
  }

  #[test]
  fn test_edit_indexer_ui_renders_edit_indexer_modal() {
    let mut app = App::test_default();
    app.push_navigation_stack(ActiveRadarrBlock::EditIndexerNameInput.into());
    app.data.radarr_data.indexers = StatefulTable::default();
    app.data.radarr_data.indexers.set_items(vec![Indexer {
      id: 1,
      name: Some("Test Indexer".to_owned()),
      enable_rss: true,
      priority: 25,
      fields: Some(vec![
        IndexerField {
          name: Some("baseUrl".to_owned()),
          value: Some(json!("https://test.indexer.com")),
        },
        IndexerField {
          name: Some("apiKey".to_owned()),
          value: Some(json!("test-api-key")),
        },
        IndexerField {
          name: Some("seedCriteria.seedRatio".to_owned()),
          value: Some(json!(1.0)),
        },
      ]),
      ..Indexer::default()
    }]);
    app.data.radarr_data.selected_block =
      BlockSelectionState::new(EDIT_INDEXER_TORRENT_SELECTION_BLOCKS);
    app.data.radarr_data.edit_indexer_modal = Some(EditIndexerModal::from(&app.data.radarr_data));

    let output = render_to_string_with_app(120, 30, &mut app, |f, app| {
      EditIndexerUi::draw(f, app, f.area());
    });

    insta::assert_snapshot!(output);
  }
}
