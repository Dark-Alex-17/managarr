#[cfg(test)]
mod tests {
  use strum::IntoEnumIterator;

  use crate::app::App;
  use crate::models::HorizontallyScrollableText;
  use crate::models::servarr_data::sonarr::sonarr_data::{ADD_SERIES_BLOCKS, ActiveSonarrBlock};
  use crate::ui::DrawUi;
  use crate::ui::sonarr_ui::library::add_series_ui::AddSeriesUi;
  use crate::ui::ui_test_utils::test_utils::render_to_string_with_app;

  #[test]
  fn test_add_series_ui_accepts() {
    ActiveSonarrBlock::iter().for_each(|active_sonarr_block| {
      if ADD_SERIES_BLOCKS.contains(&active_sonarr_block) {
        assert!(AddSeriesUi::accepts(active_sonarr_block.into()));
      } else {
        assert!(!AddSeriesUi::accepts(active_sonarr_block.into()));
      }
    });
  }

  mod snapshot_tests {
    use crate::models::BlockSelectionState;
    use crate::models::servarr_data::sonarr::sonarr_data::ADD_SERIES_SELECTION_BLOCKS;
    use crate::ui::ui_test_utils::test_utils::TerminalSize;
    use rstest::rstest;

    use super::*;

    #[test]
    fn test_add_series_ui_renders_loading_search() {
      let mut app = App::test_default_fully_populated();
      app.is_loading = true;
      app.push_navigation_stack(ActiveSonarrBlock::AddSeriesSearchResults.into());
      app.data.sonarr_data.add_series_search = Some(HorizontallyScrollableText::default());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        AddSeriesUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }

    #[rstest]
    fn test_add_series_ui_renders(
      #[values(
        ActiveSonarrBlock::AddSeriesAlreadyInLibrary,
        ActiveSonarrBlock::AddSeriesEmptySearchResults,
        ActiveSonarrBlock::AddSeriesPrompt,
        ActiveSonarrBlock::AddSeriesSearchResults,
        ActiveSonarrBlock::AddSeriesSelectLanguageProfile,
        ActiveSonarrBlock::AddSeriesSelectMonitor,
        ActiveSonarrBlock::AddSeriesSelectQualityProfile,
        ActiveSonarrBlock::AddSeriesSelectRootFolder,
        ActiveSonarrBlock::AddSeriesSelectSeriesType
      )]
      active_sonarr_block: ActiveSonarrBlock,
    ) {
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack(active_sonarr_block.into());
      app.data.sonarr_data.selected_block = BlockSelectionState::new(ADD_SERIES_SELECTION_BLOCKS);

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        AddSeriesUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(format!("add_series_ui_{active_sonarr_block}"), output);
    }
  }
}
