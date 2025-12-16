#[cfg(test)]
mod tests {
  use bimap::BiMap;
  use strum::IntoEnumIterator;

  use crate::app::App;
  use crate::models::BlockSelectionState;
  use crate::models::servarr_data::sonarr::modals::EditSeriesModal;
  use crate::models::servarr_data::sonarr::sonarr_data::{
    ActiveSonarrBlock, EDIT_SERIES_BLOCKS, EDIT_SERIES_SELECTION_BLOCKS,
  };
  use crate::models::sonarr_models::Series;
  use crate::models::stateful_table::StatefulTable;
  use crate::ui::DrawUi;
  use crate::ui::sonarr_ui::library::edit_series_ui::EditSeriesUi;
  use crate::ui::ui_test_utils::test_utils::render_to_string_with_app;

  #[test]
  fn test_edit_series_ui_accepts() {
    ActiveSonarrBlock::iter().for_each(|active_sonarr_block| {
      if EDIT_SERIES_BLOCKS.contains(&active_sonarr_block) {
        assert!(EditSeriesUi::accepts(active_sonarr_block.into()));
      } else {
        assert!(!EditSeriesUi::accepts(active_sonarr_block.into()));
      }
    });
  }

  mod snapshot_tests {
    use crate::ui::ui_test_utils::test_utils::TerminalSize;

    use super::*;

    #[test]
    fn test_edit_series_ui_renders_edit_series_modal() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveSonarrBlock::EditSeriesPathInput.into());
      app.data.sonarr_data.quality_profile_map = BiMap::from_iter(vec![(1, "HD-1080p".to_owned())]);
      app.data.sonarr_data.language_profiles_map =
        BiMap::from_iter(vec![(1, "English".to_owned())]);
      app.data.sonarr_data.series = StatefulTable::default();
      app.data.sonarr_data.series.set_items(vec![Series {
        id: 1,
        title: "Test Series".into(),
        path: "/tv/test".to_owned(),
        quality_profile_id: 1,
        language_profile_id: 1,
        ..Series::default()
      }]);
      app.data.sonarr_data.selected_block = BlockSelectionState::new(EDIT_SERIES_SELECTION_BLOCKS);
      app.data.sonarr_data.edit_series_modal = Some(EditSeriesModal::from(&app.data.sonarr_data));

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        EditSeriesUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }
  }
}
