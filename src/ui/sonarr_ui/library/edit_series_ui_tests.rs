#[cfg(test)]
mod tests {
  use strum::IntoEnumIterator;

  use crate::app::App;
  use crate::models::BlockSelectionState;
  use crate::models::servarr_data::sonarr::sonarr_data::{
    ActiveSonarrBlock, EDIT_SERIES_BLOCKS, EDIT_SERIES_SELECTION_BLOCKS,
  };
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
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(ActiveSonarrBlock::EditSeriesPrompt, None)]
    #[case(ActiveSonarrBlock::EditSeriesConfirmPrompt, None)]
    #[case(ActiveSonarrBlock::EditSeriesSelectSeriesType, None)]
    #[case(ActiveSonarrBlock::EditSeriesSelectQualityProfile, None)]
    #[case(ActiveSonarrBlock::EditSeriesSelectLanguageProfile, None)]
    #[case(
      ActiveSonarrBlock::EditSeriesPrompt,
      Some(ActiveSonarrBlock::SeriesDetails)
    )]
    #[case(
      ActiveSonarrBlock::EditSeriesConfirmPrompt,
      Some(ActiveSonarrBlock::SeriesDetails)
    )]
    #[case(
      ActiveSonarrBlock::EditSeriesSelectSeriesType,
      Some(ActiveSonarrBlock::SeriesDetails)
    )]
    #[case(
      ActiveSonarrBlock::EditSeriesSelectQualityProfile,
      Some(ActiveSonarrBlock::SeriesDetails)
    )]
    #[case(
      ActiveSonarrBlock::EditSeriesSelectLanguageProfile,
      Some(ActiveSonarrBlock::SeriesDetails)
    )]
    fn test_edit_series_ui_renders(
      #[case] active_sonarr_block: ActiveSonarrBlock,
      #[case] context: Option<ActiveSonarrBlock>,
    ) {
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack((active_sonarr_block, context).into());
      app.data.sonarr_data.selected_block = BlockSelectionState::new(EDIT_SERIES_SELECTION_BLOCKS);

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        EditSeriesUi::draw(f, app, f.area());
      });

      if let Some(context) = context {
        insta::assert_snapshot!(
          format!("edit_series_{active_sonarr_block}_{context}"),
          output
        );
      } else {
        insta::assert_snapshot!(format!("edit_series_{active_sonarr_block}"), output);
      }
    }
  }
}
