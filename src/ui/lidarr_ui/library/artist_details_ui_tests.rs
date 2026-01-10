#[cfg(test)]
mod tests {
  use strum::IntoEnumIterator;

  use crate::models::servarr_data::lidarr::lidarr_data::{
    ARTIST_DETAILS_BLOCKS, ActiveLidarrBlock, DELETE_ALBUM_BLOCKS,
  };
  use crate::ui::DrawUi;
  use crate::ui::lidarr_ui::library::artist_details_ui::ArtistDetailsUi;

  #[test]
  fn test_artist_details_ui_accepts() {
    let mut blocks = ARTIST_DETAILS_BLOCKS.clone().to_vec();
    blocks.extend(DELETE_ALBUM_BLOCKS);

    ActiveLidarrBlock::iter().for_each(|active_lidarr_block| {
      if blocks.contains(&active_lidarr_block) {
        assert!(ArtistDetailsUi::accepts(active_lidarr_block.into()));
      } else {
        assert!(!ArtistDetailsUi::accepts(active_lidarr_block.into()));
      }
    });
  }

  mod snapshot_tests {
    use rstest::rstest;

    use crate::app::App;
    use crate::models::BlockSelectionState;
    use crate::models::servarr_data::lidarr::lidarr_data::{
      ActiveLidarrBlock, DELETE_ALBUM_SELECTION_BLOCKS,
    };
    use crate::models::stateful_table::StatefulTable;
    use crate::ui::DrawUi;
    use crate::ui::lidarr_ui::library::artist_details_ui::ArtistDetailsUi;
    use crate::ui::ui_test_utils::test_utils::{TerminalSize, render_to_string_with_app};

    #[rstest]
    #[case(ActiveLidarrBlock::ArtistDetails, 0)]
    #[case(ActiveLidarrBlock::SearchAlbums, 0)]
    #[case(ActiveLidarrBlock::SearchAlbumsError, 0)]
    #[case(ActiveLidarrBlock::UpdateAndScanArtistPrompt, 0)]
    #[case(ActiveLidarrBlock::AutomaticallySearchArtistPrompt, 0)]
    fn test_artist_details_ui_renders(
      #[case] active_lidarr_block: ActiveLidarrBlock,
      #[case] index: usize,
    ) {
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack(active_lidarr_block.into());
      app.data.lidarr_data.artist_info_tabs.set_index(index);

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        ArtistDetailsUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(
        format!("artist_details_{active_lidarr_block}_{index}"),
        output
      );
    }

    #[rstest]
    #[case(ActiveLidarrBlock::ArtistDetails, 0)]
    fn test_artist_details_ui_renders_artist_details_loading(
      #[case] active_lidarr_block: ActiveLidarrBlock,
      #[case] index: usize,
    ) {
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack(active_lidarr_block.into());
      app.data.lidarr_data.artist_info_tabs.set_index(index);
      app.is_loading = true;

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        ArtistDetailsUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(
        format!("loading_artist_details_{active_lidarr_block}"),
        output
      );
    }

    #[rstest]
    #[case(ActiveLidarrBlock::ArtistDetails, 0)]
    fn test_artist_details_ui_renders_artist_details_empty(
      #[case] active_lidarr_block: ActiveLidarrBlock,
      #[case] index: usize,
    ) {
      let mut app = App::test_default_fully_populated();
      app.data.lidarr_data.albums = StatefulTable::default();
      app.push_navigation_stack(active_lidarr_block.into());
      app.data.lidarr_data.artist_info_tabs.set_index(index);

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        ArtistDetailsUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(
        format!("empty_artist_details_{active_lidarr_block}"),
        output
      );
    }

    #[test]
    fn test_artist_details_ui_renders_delete_album_prompt_over_artist_details() {
      let mut app = App::test_default_fully_populated();
      app.data.lidarr_data.selected_block = BlockSelectionState::new(DELETE_ALBUM_SELECTION_BLOCKS);
      app.push_navigation_stack(ActiveLidarrBlock::ArtistDetails.into());
      app.push_navigation_stack(ActiveLidarrBlock::DeleteAlbumPrompt.into());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        ArtistDetailsUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }

    #[test]
    fn test_artist_details_ui_renders_update_and_scan_prompt_over_artist_details() {
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack(ActiveLidarrBlock::ArtistDetails.into());
      app.push_navigation_stack(ActiveLidarrBlock::UpdateAndScanArtistPrompt.into());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        ArtistDetailsUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }

    #[test]
    fn test_artist_details_ui_renders_automatic_search_prompt_over_artist_details() {
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack(ActiveLidarrBlock::ArtistDetails.into());
      app.push_navigation_stack(ActiveLidarrBlock::AutomaticallySearchArtistPrompt.into());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        ArtistDetailsUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }
  }
}
