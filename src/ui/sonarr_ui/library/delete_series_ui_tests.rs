#[cfg(test)]
mod tests {
  use strum::IntoEnumIterator;

  use crate::app::App;
  use crate::models::BlockSelectionState;
  use crate::models::servarr_data::sonarr::sonarr_data::{
    ActiveSonarrBlock, DELETE_SERIES_BLOCKS, DELETE_SERIES_SELECTION_BLOCKS,
  };
  use crate::models::sonarr_models::Series;
  use crate::models::stateful_table::StatefulTable;
  use crate::ui::DrawUi;
  use crate::ui::sonarr_ui::library::delete_series_ui::DeleteSeriesUi;
  use crate::ui::ui_test_utils::test_utils::render_to_string_with_app;

  #[test]
  fn test_delete_series_ui_accepts() {
    ActiveSonarrBlock::iter().for_each(|active_sonarr_block| {
      if DELETE_SERIES_BLOCKS.contains(&active_sonarr_block) {
        assert!(DeleteSeriesUi::accepts(active_sonarr_block.into()));
      } else {
        assert!(!DeleteSeriesUi::accepts(active_sonarr_block.into()));
      }
    });
  }

  mod snapshot_tests {
    use super::*;

    #[test]
    fn test_delete_series_ui_renders_delete_series_toggle() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveSonarrBlock::DeleteSeriesPrompt.into());
      app.data.sonarr_data.series = StatefulTable::default();
      app.data.sonarr_data.series.set_items(vec![Series {
        id: 1,
        title: "Test Series".into(),
        ..Series::default()
      }]);
      app.data.sonarr_data.selected_block =
        BlockSelectionState::new(DELETE_SERIES_SELECTION_BLOCKS);

      let output = render_to_string_with_app(120, 30, &mut app, |f, app| {
        DeleteSeriesUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }
  }
}
