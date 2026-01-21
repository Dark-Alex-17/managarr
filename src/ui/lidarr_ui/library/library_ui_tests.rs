#[cfg(test)]
mod tests {
  use strum::IntoEnumIterator;

  use crate::models::lidarr_models::{Artist, ArtistStatistics, ArtistStatus};
  use crate::models::servarr_data::lidarr::lidarr_data::{
    ADD_ARTIST_BLOCKS, ALBUM_DETAILS_BLOCKS, ARTIST_DETAILS_BLOCKS, ActiveLidarrBlock,
    DELETE_ALBUM_BLOCKS, DELETE_ARTIST_BLOCKS, EDIT_ARTIST_BLOCKS, LIBRARY_BLOCKS,
    TRACK_DETAILS_BLOCKS,
  };
  use crate::ui::DrawUi;
  use crate::ui::lidarr_ui::library::{LibraryUi, decorate_artist_row_with_style};
  use crate::ui::styles::ManagarrStyle;
  use pretty_assertions::assert_eq;
  use ratatui::widgets::{Cell, Row};

  #[test]
  fn test_library_ui_accepts() {
    let mut library_ui_blocks = Vec::new();
    library_ui_blocks.extend(LIBRARY_BLOCKS);
    library_ui_blocks.extend(DELETE_ARTIST_BLOCKS);
    library_ui_blocks.extend(DELETE_ALBUM_BLOCKS);
    library_ui_blocks.extend(EDIT_ARTIST_BLOCKS);
    library_ui_blocks.extend(ADD_ARTIST_BLOCKS);
    library_ui_blocks.extend(ARTIST_DETAILS_BLOCKS);
    library_ui_blocks.extend(ALBUM_DETAILS_BLOCKS);
    library_ui_blocks.extend(TRACK_DETAILS_BLOCKS);

    for active_lidarr_block in ActiveLidarrBlock::iter() {
      if library_ui_blocks.contains(&active_lidarr_block) {
        assert!(
          LibraryUi::accepts(active_lidarr_block.into()),
          "{active_lidarr_block} is not accepted by the LibraryUi"
        );
      } else {
        assert!(
          !LibraryUi::accepts(active_lidarr_block.into()),
          "{active_lidarr_block} should not be accepted by LibraryUi"
        );
      }
    }
  }

  #[test]
  fn test_decorate_row_with_style_unmonitored() {
    let artist = Artist::default();
    let row = Row::new(vec![Cell::from("test".to_owned())]);

    let style = decorate_artist_row_with_style(&artist, row.clone());

    assert_eq!(style, row.unmonitored());
  }

  #[test]
  fn test_decorate_row_with_style_downloaded_when_ended_and_all_tracks_present() {
    let artist = Artist {
      monitored: true,
      status: ArtistStatus::Ended,
      statistics: Some(ArtistStatistics {
        track_file_count: 10,
        total_track_count: 10,
        ..ArtistStatistics::default()
      }),
      ..Artist::default()
    };
    let row = Row::new(vec![Cell::from("test".to_owned())]);

    let style = decorate_artist_row_with_style(&artist, row.clone());

    assert_eq!(style, row.downloaded());
  }

  #[test]
  fn test_decorate_row_with_style_missing_when_ended_and_tracks_are_missing() {
    let artist = Artist {
      monitored: true,
      status: ArtistStatus::Ended,
      statistics: Some(ArtistStatistics {
        track_file_count: 5,
        total_track_count: 10,
        ..ArtistStatistics::default()
      }),
      ..Artist::default()
    };
    let row = Row::new(vec![Cell::from("test".to_owned())]);

    let style = decorate_artist_row_with_style(&artist, row.clone());

    assert_eq!(style, row.missing());
  }

  #[test]
  fn test_decorate_row_with_style_indeterminate_when_ended_and_no_statistics() {
    let artist = Artist {
      monitored: true,
      status: ArtistStatus::Ended,
      statistics: None,
      ..Artist::default()
    };
    let row = Row::new(vec![Cell::from("test".to_owned())]);

    let style = decorate_artist_row_with_style(&artist, row.clone());

    assert_eq!(style, row.indeterminate());
  }

  #[test]
  fn test_decorate_row_with_style_indeterminate_when_ended_and_total_track_count_is_zero() {
    let artist = Artist {
      monitored: true,
      status: ArtistStatus::Ended,
      statistics: Some(ArtistStatistics {
        track_file_count: 0,
        total_track_count: 0,
        ..ArtistStatistics::default()
      }),
      ..Artist::default()
    };
    let row = Row::new(vec![Cell::from("test".to_owned())]);

    let style = decorate_artist_row_with_style(&artist, row.clone());

    assert_eq!(style, row.missing());
  }

  #[test]
  fn test_decorate_row_with_style_unreleased_when_continuing_and_all_tracks_present() {
    let artist = Artist {
      monitored: true,
      status: ArtistStatus::Continuing,
      statistics: Some(ArtistStatistics {
        track_file_count: 10,
        total_track_count: 10,
        ..ArtistStatistics::default()
      }),
      ..Artist::default()
    };
    let row = Row::new(vec![Cell::from("test".to_owned())]);

    let style = decorate_artist_row_with_style(&artist, row.clone());

    assert_eq!(style, row.unreleased());
  }

  #[test]
  fn test_decorate_row_with_style_missing_when_continuing_and_tracks_are_missing() {
    let artist = Artist {
      monitored: true,
      status: ArtistStatus::Continuing,
      statistics: Some(ArtistStatistics {
        track_file_count: 5,
        total_track_count: 10,
        ..ArtistStatistics::default()
      }),
      ..Artist::default()
    };
    let row = Row::new(vec![Cell::from("test".to_owned())]);

    let style = decorate_artist_row_with_style(&artist, row.clone());

    assert_eq!(style, row.missing());
  }

  #[test]
  fn test_decorate_row_with_style_indeterminate_when_continuing_and_no_statistics() {
    let artist = Artist {
      monitored: true,
      status: ArtistStatus::Continuing,
      statistics: None,
      ..Artist::default()
    };
    let row = Row::new(vec![Cell::from("test".to_owned())]);

    let style = decorate_artist_row_with_style(&artist, row.clone());

    assert_eq!(style, row.indeterminate());
  }

  #[test]
  fn test_decorate_row_with_style_defaults_to_indeterminate_for_deleted_status() {
    let artist = Artist {
      monitored: true,
      status: ArtistStatus::Deleted,
      ..Artist::default()
    };
    let row = Row::new(vec![Cell::from("test".to_owned())]);

    let style = decorate_artist_row_with_style(&artist, row.clone());

    assert_eq!(style, row.indeterminate());
  }

  mod snapshot_tests {
    use crate::app::App;
    use crate::models::BlockSelectionState;
    use crate::models::servarr_data::lidarr::lidarr_data::{
      ActiveLidarrBlock, DELETE_ARTIST_SELECTION_BLOCKS, EDIT_ARTIST_SELECTION_BLOCKS,
    };
    use rstest::rstest;

    use crate::ui::DrawUi;
    use crate::ui::lidarr_ui::library::LibraryUi;
    use crate::ui::ui_test_utils::test_utils::{TerminalSize, render_to_string_with_app};

    #[rstest]
    fn test_library_ui_renders(
      #[values(
        ActiveLidarrBlock::Artists,
        ActiveLidarrBlock::ArtistsSortPrompt,
        ActiveLidarrBlock::SearchArtists,
        ActiveLidarrBlock::SearchArtistsError,
        ActiveLidarrBlock::FilterArtists,
        ActiveLidarrBlock::FilterArtistsError
      )]
      active_lidarr_block: ActiveLidarrBlock,
    ) {
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack(active_lidarr_block.into());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        LibraryUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(format!("lidarr_library_{active_lidarr_block}"), output);
    }

    #[test]
    fn test_library_ui_renders_loading() {
      let mut app = App::test_default_fully_populated();
      app.is_loading = true;
      app.push_navigation_stack(ActiveLidarrBlock::Artists.into());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        LibraryUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }

    #[test]
    fn test_library_ui_renders_empty() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveLidarrBlock::Artists.into());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        LibraryUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }

    #[test]
    fn test_library_ui_renders_delete_artist_over_library() {
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack(ActiveLidarrBlock::DeleteArtistPrompt.into());
      app.data.lidarr_data.selected_block =
        BlockSelectionState::new(DELETE_ARTIST_SELECTION_BLOCKS);

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        LibraryUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }

    #[test]
    fn test_library_ui_renders_update_all_artists_prompt() {
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack(ActiveLidarrBlock::Artists.into());
      app.push_navigation_stack(ActiveLidarrBlock::UpdateAllArtistsPrompt.into());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        LibraryUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }

    #[test]
    fn test_library_ui_renders_artist_details_over_library() {
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack(ActiveLidarrBlock::Artists.into());
      app.push_navigation_stack(ActiveLidarrBlock::ArtistDetails.into());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        LibraryUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }

    #[rstest]
    #[case(ActiveLidarrBlock::ArtistDetails, 0)]
    #[case(ActiveLidarrBlock::ArtistHistory, 1)]
    #[case(ActiveLidarrBlock::ManualArtistSearch, 2)]
    fn test_library_ui_renders_edit_artist_over_artist_details(
      #[case] active_lidarr_block: ActiveLidarrBlock,
      #[case] index: usize,
    ) {
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack(ActiveLidarrBlock::Artists.into());
      app.push_navigation_stack(
        (
          ActiveLidarrBlock::EditArtistPrompt,
          Some(active_lidarr_block),
        )
          .into(),
      );
      app.data.lidarr_data.artist_info_tabs.set_index(index);
      app.data.lidarr_data.selected_block = BlockSelectionState::new(EDIT_ARTIST_SELECTION_BLOCKS);

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        LibraryUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(
        format!("edit_artist_renders_over_{active_lidarr_block}"),
        output
      );
    }

    #[test]
    fn test_library_ui_renders_dropdown_over_edit_artist_over_artist_details() {
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack(ActiveLidarrBlock::Artists.into());
      app.push_navigation_stack(ActiveLidarrBlock::ArtistDetails.into());
      app.push_navigation_stack(ActiveLidarrBlock::EditArtistPrompt.into());
      app.push_navigation_stack(ActiveLidarrBlock::EditArtistSelectMetadataProfile.into());
      app.data.lidarr_data.selected_block = BlockSelectionState::new(EDIT_ARTIST_SELECTION_BLOCKS);

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        LibraryUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }
  }
}
