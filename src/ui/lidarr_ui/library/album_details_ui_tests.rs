#[cfg(test)]
mod tests {
  use strum::IntoEnumIterator;

  use crate::app::App;
  use crate::models::servarr_data::lidarr::lidarr_data::{
    ALBUM_DETAILS_BLOCKS, ActiveLidarrBlock, TRACK_DETAILS_BLOCKS,
  };
  use crate::models::stateful_table::StatefulTable;
  use crate::ui::DrawUi;
  use crate::ui::lidarr_ui::library::album_details_ui::AlbumDetailsUi;
  use crate::ui::ui_test_utils::test_utils::render_to_string_with_app;

  #[test]
  fn test_album_details_ui_accepts() {
    let mut album_details_blocks = ALBUM_DETAILS_BLOCKS.to_vec();
    album_details_blocks.extend(TRACK_DETAILS_BLOCKS);

    ActiveLidarrBlock::iter().for_each(|active_lidarr_block| {
      if album_details_blocks.contains(&active_lidarr_block) {
        assert!(AlbumDetailsUi::accepts(active_lidarr_block.into()));
      } else {
        assert!(!AlbumDetailsUi::accepts(active_lidarr_block.into()));
      }
    });
  }

  mod snapshot_tests {
    use crate::ui::ui_test_utils::test_utils::TerminalSize;
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(ActiveLidarrBlock::AlbumDetails, 0)]
    #[case(ActiveLidarrBlock::AlbumHistory, 1)]
    #[case(ActiveLidarrBlock::SearchTracks, 0)]
    #[case(ActiveLidarrBlock::SearchTracksError, 0)]
    #[case(ActiveLidarrBlock::AutomaticallySearchAlbumPrompt, 0)]
    #[case(ActiveLidarrBlock::AutomaticallySearchAlbumPrompt, 1)]
    #[case(ActiveLidarrBlock::AutomaticallySearchAlbumPrompt, 2)]
    #[case(ActiveLidarrBlock::SearchAlbumHistory, 1)]
    #[case(ActiveLidarrBlock::SearchAlbumHistoryError, 1)]
    #[case(ActiveLidarrBlock::FilterAlbumHistory, 1)]
    #[case(ActiveLidarrBlock::FilterAlbumHistoryError, 1)]
    #[case(ActiveLidarrBlock::AlbumHistorySortPrompt, 1)]
    #[case(ActiveLidarrBlock::AlbumHistoryDetails, 1)]
    #[case(ActiveLidarrBlock::ManualAlbumSearch, 2)]
    #[case(ActiveLidarrBlock::ManualAlbumSearchConfirmPrompt, 2)]
    #[case(ActiveLidarrBlock::ManualAlbumSearchSortPrompt, 2)]
    #[case(ActiveLidarrBlock::DeleteTrackFilePrompt, 0)]
    fn test_album_details_ui_renders(
      #[case] active_lidarr_block: ActiveLidarrBlock,
      #[case] index: usize,
    ) {
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack(active_lidarr_block.into());
      app
        .data
        .lidarr_data
        .album_details_modal
        .as_mut()
        .unwrap()
        .album_details_tabs
        .set_index(index);

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        AlbumDetailsUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(
        format!("album_details_renders_{active_lidarr_block}_{index}"),
        output
      );
    }

    #[rstest]
    #[case(ActiveLidarrBlock::AlbumDetails, 0)]
    #[case(ActiveLidarrBlock::AlbumHistory, 1)]
    #[case(ActiveLidarrBlock::AlbumHistoryDetails, 1)]
    #[case(ActiveLidarrBlock::ManualAlbumSearch, 2)]
    fn test_album_details_ui_renders_loading(
      #[case] active_lidarr_block: ActiveLidarrBlock,
      #[case] index: usize,
    ) {
      let mut app = App::test_default_fully_populated();
      app.is_loading = true;
      app.push_navigation_stack(active_lidarr_block.into());
      {
        let album_details_modal = app.data.lidarr_data.album_details_modal.as_mut().unwrap();
        album_details_modal.album_releases = StatefulTable::default();
        album_details_modal.album_history = StatefulTable::default();
        album_details_modal.tracks = StatefulTable::default();
        album_details_modal.album_details_tabs.set_index(index);
      }

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        AlbumDetailsUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(
        format!("loading_album_details_{active_lidarr_block}_{index}"),
        output
      );
    }

    #[rstest]
    #[case(ActiveLidarrBlock::AlbumDetails, 0)]
    #[case(ActiveLidarrBlock::AlbumHistory, 1)]
    #[case(ActiveLidarrBlock::AlbumHistoryDetails, 1)]
    #[case(ActiveLidarrBlock::ManualAlbumSearch, 2)]
    fn test_album_details_ui_renders_empty(
      #[case] active_lidarr_block: ActiveLidarrBlock,
      #[case] index: usize,
    ) {
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack(active_lidarr_block.into());
      {
        let album_details_modal = app.data.lidarr_data.album_details_modal.as_mut().unwrap();
        album_details_modal.album_releases = StatefulTable::default();
        album_details_modal.album_history = StatefulTable::default();
        album_details_modal.tracks = StatefulTable::default();
        album_details_modal.album_details_tabs.set_index(index);
      }

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        AlbumDetailsUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(
        format!("empty_album_details_{active_lidarr_block}_{index}"),
        output
      );
    }

    #[test]
    fn test_album_details_ui_renders_track_details_over_album_details() {
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack(ActiveLidarrBlock::TrackDetails.into());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        AlbumDetailsUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }
  }
}
