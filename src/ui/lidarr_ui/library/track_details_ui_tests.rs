#[cfg(test)]
mod tests {
  use pretty_assertions::assert_eq;
  use strum::IntoEnumIterator;

  use crate::app::App;
  use crate::models::lidarr_models::Track;
  use crate::models::servarr_data::lidarr::lidarr_data::{ActiveLidarrBlock, TRACK_DETAILS_BLOCKS};
  use crate::ui::DrawUi;
  use crate::ui::lidarr_ui::library::track_details_ui::{TrackDetailsUi, style_from_status};
  use crate::ui::styles::{downloaded_style, missing_style};
  use crate::ui::ui_test_utils::test_utils::render_to_string_with_app;

  #[test]
  fn test_track_details_ui_accepts() {
    ActiveLidarrBlock::iter().for_each(|active_lidarr_block| {
      if TRACK_DETAILS_BLOCKS.contains(&active_lidarr_block) {
        assert!(TrackDetailsUi::accepts(active_lidarr_block.into()));
      } else {
        assert!(!TrackDetailsUi::accepts(active_lidarr_block.into()));
      }
    });
  }

  #[test]
  fn test_style_from_status_downloaded_when_track_has_file() {
    let track = Track {
      has_file: true,
      ..Track::default()
    };

    let style = style_from_status(&track);

    assert_eq!(style, downloaded_style());
  }

  #[test]
  fn test_style_from_status_missing_when_track_has_no_file() {
    let track = Track {
      has_file: false,
      ..Track::default()
    };

    let style = style_from_status(&track);

    assert_eq!(style, missing_style());
  }

  mod snapshot_tests {
    use crate::ui::ui_test_utils::test_utils::TerminalSize;
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(ActiveLidarrBlock::TrackDetails, 0)]
    #[case(ActiveLidarrBlock::TrackHistory, 1)]
    #[case(ActiveLidarrBlock::TrackHistoryDetails, 1)]
    #[case(ActiveLidarrBlock::SearchTrackHistory, 1)]
    #[case(ActiveLidarrBlock::SearchTrackHistoryError, 1)]
    #[case(ActiveLidarrBlock::FilterTrackHistory, 1)]
    #[case(ActiveLidarrBlock::FilterTrackHistoryError, 1)]
    #[case(ActiveLidarrBlock::TrackHistorySortPrompt, 1)]
    #[case(ActiveLidarrBlock::TrackHistoryDetails, 1)]
    fn test_track_details_ui_renders(
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
        .track_details_modal
        .as_mut()
        .unwrap()
        .track_details_tabs
        .set_index(index);

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        TrackDetailsUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(
        format!("track_details_{active_lidarr_block}_{index}"),
        output
      );
    }

    #[rstest]
    #[case(ActiveLidarrBlock::TrackDetails, 0)]
    #[case(ActiveLidarrBlock::TrackHistory, 1)]
    fn test_track_details_ui_renders_loading(
      #[case] active_lidarr_block: ActiveLidarrBlock,
      #[case] index: usize,
    ) {
      let mut app = App::test_default_fully_populated();
      app.is_loading = true;
      app.push_navigation_stack(active_lidarr_block.into());
      app
        .data
        .lidarr_data
        .album_details_modal
        .as_mut()
        .unwrap()
        .track_details_modal
        .as_mut()
        .unwrap()
        .track_details_tabs
        .set_index(index);

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        TrackDetailsUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(
        format!("loading_track_details_{active_lidarr_block}_{index}"),
        output
      );
    }

    #[rstest]
    #[case(ActiveLidarrBlock::TrackDetails, 0)]
    #[case(ActiveLidarrBlock::TrackHistory, 1)]
    fn test_track_details_ui_renders_empty(
      #[case] active_lidarr_block: ActiveLidarrBlock,
      #[case] index: usize,
    ) {
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack(active_lidarr_block.into());
      {
        let track_details_modal = app
          .data
          .lidarr_data
          .album_details_modal
          .as_mut()
          .unwrap()
          .track_details_modal
          .as_mut()
          .unwrap();
        track_details_modal.track_details_tabs.set_index(index);
        track_details_modal.track_details = Default::default();
        track_details_modal.track_history = Default::default();
      }

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        TrackDetailsUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(
        format!("empty_track_details_{active_lidarr_block}_{index}"),
        output
      );
    }
  }
}
