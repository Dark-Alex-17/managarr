#[cfg(test)]
mod tests {
  use strum::IntoEnumIterator;

  use crate::models::servarr_data::lidarr::lidarr_data::{ADD_ARTIST_BLOCKS, ActiveLidarrBlock};
  use crate::ui::DrawUi;
  use crate::ui::lidarr_ui::library::add_artist_ui::AddArtistUi;

  #[test]
  fn test_add_artist_ui_accepts() {
    ActiveLidarrBlock::iter().for_each(|active_lidarr_block| {
      if ADD_ARTIST_BLOCKS.contains(&active_lidarr_block) {
        assert!(AddArtistUi::accepts(active_lidarr_block.into()));
      } else {
        assert!(!AddArtistUi::accepts(active_lidarr_block.into()));
      }
    });
  }

  mod snapshot_tests {
    use super::*;
    use crate::app::App;
    use crate::models::HorizontallyScrollableText;
    use crate::ui::ui_test_utils::test_utils::{TerminalSize, render_to_string_with_app};
    use rstest::rstest;

    #[test]
    fn test_add_artist_ui_renders_loading_for_search() {
      let mut app = App::test_default_fully_populated();
      app.data.lidarr_data.add_artist_search = Some(HorizontallyScrollableText::default());
      app.data.lidarr_data.add_searched_artists = None;
      app.push_navigation_stack(ActiveLidarrBlock::AddArtistSearchResults.into());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        AddArtistUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }

    #[rstest]
    fn test_add_artist_ui_renders(
      #[values(
        ActiveLidarrBlock::AddArtistSearchInput,
        ActiveLidarrBlock::AddArtistSearchResults,
        ActiveLidarrBlock::AddArtistEmptySearchResults
      )]
      active_lidarr_block: ActiveLidarrBlock,
    ) {
      let mut app = App::test_default_fully_populated();
      app.data.lidarr_data.add_artist_search = Some("Test Artist".into());
      app.push_navigation_stack(active_lidarr_block.into());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        AddArtistUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(format!("add_artist_ui_{active_lidarr_block}"), output);
    }
  }
}
