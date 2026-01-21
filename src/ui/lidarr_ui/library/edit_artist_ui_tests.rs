#[cfg(test)]
mod tests {
  use strum::IntoEnumIterator;

  use crate::app::App;
  use crate::models::BlockSelectionState;
  use crate::models::servarr_data::lidarr::lidarr_data::{
    ActiveLidarrBlock, EDIT_ARTIST_BLOCKS, EDIT_ARTIST_SELECTION_BLOCKS,
  };
  use crate::ui::DrawUi;
  use crate::ui::lidarr_ui::library::edit_artist_ui::EditArtistUi;
  use crate::ui::ui_test_utils::test_utils::render_to_string_with_app;

  #[test]
  fn test_edit_artist_ui_accepts() {
    ActiveLidarrBlock::iter().for_each(|active_lidarr_block| {
      if EDIT_ARTIST_BLOCKS.contains(&active_lidarr_block) {
        assert!(EditArtistUi::accepts(active_lidarr_block.into()));
      } else {
        assert!(!EditArtistUi::accepts(active_lidarr_block.into()));
      }
    });
  }

  mod snapshot_tests {
    use crate::ui::ui_test_utils::test_utils::TerminalSize;
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(ActiveLidarrBlock::EditArtistPrompt)]
    #[case(ActiveLidarrBlock::EditArtistConfirmPrompt)]
    #[case(ActiveLidarrBlock::EditArtistSelectMetadataProfile)]
    #[case(ActiveLidarrBlock::EditArtistSelectMonitorNewItems)]
    #[case(ActiveLidarrBlock::EditArtistSelectQualityProfile)]
    fn test_edit_artist_ui_renders(#[case] active_lidarr_block: ActiveLidarrBlock) {
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack(active_lidarr_block.into());
      app.data.lidarr_data.selected_block = BlockSelectionState::new(EDIT_ARTIST_SELECTION_BLOCKS);

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        EditArtistUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(format!("edit_artist_{active_lidarr_block}"), output);
    }
  }
}
