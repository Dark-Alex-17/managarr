#[cfg(test)]
mod tests {
  use std::cmp::Ordering;

  use pretty_assertions::{assert_eq, assert_str_eq};
  use rstest::rstest;
  use serde_json::Number;
  use strum::IntoEnumIterator;

  use crate::app::App;
  use crate::app::key_binding::DEFAULT_KEYBINDINGS;
  use crate::handlers::KeyEventHandler;
  use crate::handlers::lidarr_handlers::library::{LibraryHandler, artists_sorting_options};
  use crate::models::lidarr_models::{Artist, ArtistStatistics, ArtistStatus};
  use crate::models::servarr_data::lidarr::lidarr_data::{
    ActiveLidarrBlock, DELETE_ARTIST_BLOCKS, EDIT_ARTIST_BLOCKS, EDIT_ARTIST_SELECTION_BLOCKS,
    LIBRARY_BLOCKS,
  };
  use crate::models::servarr_data::lidarr::modals::EditArtistModal;
  use crate::network::lidarr_network::LidarrEvent;
  use crate::{
    assert_modal_absent, assert_modal_present, assert_navigation_popped, assert_navigation_pushed,
  };

  #[test]
  fn test_library_handler_accepts() {
    let mut library_handler_blocks = Vec::new();
    library_handler_blocks.extend(LIBRARY_BLOCKS);
    library_handler_blocks.extend(DELETE_ARTIST_BLOCKS);
    library_handler_blocks.extend(EDIT_ARTIST_BLOCKS);

    ActiveLidarrBlock::iter().for_each(|lidarr_block| {
      if library_handler_blocks.contains(&lidarr_block) {
        assert!(LibraryHandler::accepts(lidarr_block));
      } else {
        assert!(!LibraryHandler::accepts(lidarr_block));
      }
    });
  }

  #[test]
  fn test_artists_sorting_options_name() {
    let expected_cmp_fn: fn(&Artist, &Artist) -> Ordering = |a, b| {
      a.artist_name
        .text
        .to_lowercase()
        .cmp(&b.artist_name.text.to_lowercase())
    };
    let mut expected_artists_vec = artists_vec();
    expected_artists_vec.sort_by(expected_cmp_fn);

    let sort_option = artists_sorting_options()[0].clone();
    let mut sorted_artists_vec = artists_vec();
    sorted_artists_vec.sort_by(sort_option.cmp_fn.unwrap());

    assert_eq!(sorted_artists_vec, expected_artists_vec);
    assert_str_eq!(sort_option.name, "Name");
  }

  #[test]
  fn test_artists_sorting_options_type() {
    let expected_cmp_fn: fn(&Artist, &Artist) -> Ordering = |a, b| {
      a.artist_type
        .as_ref()
        .unwrap_or(&String::new())
        .to_lowercase()
        .cmp(
          &b.artist_type
            .as_ref()
            .unwrap_or(&String::new())
            .to_lowercase(),
        )
    };
    let mut expected_artists_vec = artists_vec();
    expected_artists_vec.sort_by(expected_cmp_fn);

    let sort_option = artists_sorting_options()[1].clone();
    let mut sorted_artists_vec = artists_vec();
    sorted_artists_vec.sort_by(sort_option.cmp_fn.unwrap());

    assert_eq!(sorted_artists_vec, expected_artists_vec);
    assert_str_eq!(sort_option.name, "Type");
  }

  #[test]
  fn test_artists_sorting_options_status() {
    let expected_cmp_fn: fn(&Artist, &Artist) -> Ordering = |a, b| {
      a.status
        .to_string()
        .to_lowercase()
        .cmp(&b.status.to_string().to_lowercase())
    };
    let mut expected_artists_vec = artists_vec();
    expected_artists_vec.sort_by(expected_cmp_fn);

    let sort_option = artists_sorting_options()[2].clone();
    let mut sorted_artists_vec = artists_vec();
    sorted_artists_vec.sort_by(sort_option.cmp_fn.unwrap());

    assert_eq!(sorted_artists_vec, expected_artists_vec);
    assert_str_eq!(sort_option.name, "Status");
  }

  #[test]
  fn test_artists_sorting_options_quality_profile() {
    let expected_cmp_fn: fn(&Artist, &Artist) -> Ordering =
      |a, b| a.quality_profile_id.cmp(&b.quality_profile_id);
    let mut expected_artists_vec = artists_vec();
    expected_artists_vec.sort_by(expected_cmp_fn);

    let sort_option = artists_sorting_options()[3].clone();
    let mut sorted_artists_vec = artists_vec();
    sorted_artists_vec.sort_by(sort_option.cmp_fn.unwrap());

    assert_eq!(sorted_artists_vec, expected_artists_vec);
    assert_str_eq!(sort_option.name, "Quality Profile");
  }

  #[test]
  fn test_artists_sorting_options_metadata_profile() {
    let expected_cmp_fn: fn(&Artist, &Artist) -> Ordering =
      |a, b| a.metadata_profile_id.cmp(&b.metadata_profile_id);
    let mut expected_artists_vec = artists_vec();
    expected_artists_vec.sort_by(expected_cmp_fn);

    let sort_option = artists_sorting_options()[4].clone();
    let mut sorted_artists_vec = artists_vec();
    sorted_artists_vec.sort_by(sort_option.cmp_fn.unwrap());

    assert_eq!(sorted_artists_vec, expected_artists_vec);
    assert_str_eq!(sort_option.name, "Metadata Profile");
  }

  #[test]
  fn test_artists_sorting_options_albums() {
    let expected_cmp_fn: fn(&Artist, &Artist) -> Ordering = |a, b| {
      a.statistics
        .as_ref()
        .map_or(0, |stats| stats.album_count)
        .cmp(&b.statistics.as_ref().map_or(0, |stats| stats.album_count))
    };
    let mut expected_artists_vec = artists_vec();
    expected_artists_vec.sort_by(expected_cmp_fn);

    let sort_option = artists_sorting_options()[5].clone();
    let mut sorted_artists_vec = artists_vec();
    sorted_artists_vec.sort_by(sort_option.cmp_fn.unwrap());

    assert_eq!(sorted_artists_vec, expected_artists_vec);
    assert_str_eq!(sort_option.name, "Albums");
  }

  #[test]
  fn test_artists_sorting_options_tracks() {
    let expected_cmp_fn: fn(&Artist, &Artist) -> Ordering = |a, b| {
      a.statistics
        .as_ref()
        .map_or(0, |stats| stats.track_count)
        .cmp(&b.statistics.as_ref().map_or(0, |stats| stats.track_count))
    };
    let mut expected_artists_vec = artists_vec();
    expected_artists_vec.sort_by(expected_cmp_fn);

    let sort_option = artists_sorting_options()[6].clone();
    let mut sorted_artists_vec = artists_vec();
    sorted_artists_vec.sort_by(sort_option.cmp_fn.unwrap());

    assert_eq!(sorted_artists_vec, expected_artists_vec);
    assert_str_eq!(sort_option.name, "Tracks");
  }

  #[test]
  fn test_artists_sorting_options_size() {
    let expected_cmp_fn: fn(&Artist, &Artist) -> Ordering = |a, b| {
      a.statistics
        .as_ref()
        .map_or(0, |stats| stats.size_on_disk)
        .cmp(&b.statistics.as_ref().map_or(0, |stats| stats.size_on_disk))
    };
    let mut expected_artists_vec = artists_vec();
    expected_artists_vec.sort_by(expected_cmp_fn);

    let sort_option = artists_sorting_options()[7].clone();
    let mut sorted_artists_vec = artists_vec();
    sorted_artists_vec.sort_by(sort_option.cmp_fn.unwrap());

    assert_eq!(sorted_artists_vec, expected_artists_vec);
    assert_str_eq!(sort_option.name, "Size");
  }

  #[test]
  fn test_artists_sorting_options_monitored() {
    let expected_cmp_fn: fn(&Artist, &Artist) -> Ordering = |a, b| a.monitored.cmp(&b.monitored);
    let mut expected_artists_vec = artists_vec();
    expected_artists_vec.sort_by(expected_cmp_fn);

    let sort_option = artists_sorting_options()[8].clone();
    let mut sorted_artists_vec = artists_vec();
    sorted_artists_vec.sort_by(sort_option.cmp_fn.unwrap());

    assert_eq!(sorted_artists_vec, expected_artists_vec);
    assert_str_eq!(sort_option.name, "Monitored");
  }

  #[test]
  fn test_artists_sorting_options_tags() {
    let expected_cmp_fn: fn(&Artist, &Artist) -> Ordering = |a, b| {
      let a_str = a
        .tags
        .iter()
        .map(|tag| tag.as_i64().unwrap().to_string())
        .collect::<Vec<String>>()
        .join(",");
      let b_str = b
        .tags
        .iter()
        .map(|tag| tag.as_i64().unwrap().to_string())
        .collect::<Vec<String>>()
        .join(",");
      a_str.cmp(&b_str)
    };
    let mut expected_artists_vec = artists_vec();
    expected_artists_vec.sort_by(expected_cmp_fn);

    let sort_option = artists_sorting_options()[9].clone();
    let mut sorted_artists_vec = artists_vec();
    sorted_artists_vec.sort_by(sort_option.cmp_fn.unwrap());

    assert_eq!(sorted_artists_vec, expected_artists_vec);
    assert_str_eq!(sort_option.name, "Tags");
  }

  #[test]
  fn test_toggle_monitoring_key() {
    let mut app = App::test_default();
    app
      .data
      .lidarr_data
      .artists
      .set_items(vec![Artist::default()]);
    app.push_navigation_stack(ActiveLidarrBlock::Artists.into());
    app.is_routing = false;

    LibraryHandler::new(
      DEFAULT_KEYBINDINGS.toggle_monitoring.key,
      &mut app,
      ActiveLidarrBlock::Artists,
      None,
    )
    .handle();

    assert_eq!(app.get_current_route(), ActiveLidarrBlock::Artists.into());
    assert!(app.data.lidarr_data.prompt_confirm);
    assert!(app.is_routing);
    assert_some_eq_x!(
      &app.data.lidarr_data.prompt_confirm_action,
      &LidarrEvent::ToggleArtistMonitoring(0)
    );
  }

  #[test]
  fn test_toggle_monitoring_key_no_op_when_not_ready() {
    let mut app = App::test_default();
    app.is_loading = true;
    app.push_navigation_stack(ActiveLidarrBlock::Artists.into());
    app.is_routing = false;

    LibraryHandler::new(
      DEFAULT_KEYBINDINGS.toggle_monitoring.key,
      &mut app,
      ActiveLidarrBlock::Artists,
      None,
    )
    .handle();

    assert_eq!(app.get_current_route(), ActiveLidarrBlock::Artists.into());
    assert!(!app.data.lidarr_data.prompt_confirm);
    assert_modal_absent!(app.data.lidarr_data.prompt_confirm_action);
    assert!(!app.is_routing);
  }

  #[test]
  fn test_update_all_artists_key() {
    let mut app = App::test_default();
    app
      .data
      .lidarr_data
      .artists
      .set_items(vec![Artist::default()]);

    LibraryHandler::new(
      DEFAULT_KEYBINDINGS.update.key,
      &mut app,
      ActiveLidarrBlock::Artists,
      None,
    )
    .handle();

    assert_navigation_pushed!(app, ActiveLidarrBlock::UpdateAllArtistsPrompt.into());
  }

  #[test]
  fn test_update_all_artists_key_no_op_when_not_ready() {
    let mut app = App::test_default();
    app.is_loading = true;
    app.push_navigation_stack(ActiveLidarrBlock::Artists.into());
    app
      .data
      .lidarr_data
      .artists
      .set_items(vec![Artist::default()]);

    LibraryHandler::new(
      DEFAULT_KEYBINDINGS.update.key,
      &mut app,
      ActiveLidarrBlock::Artists,
      None,
    )
    .handle();

    assert_eq!(app.get_current_route(), ActiveLidarrBlock::Artists.into());
  }

  #[test]
  fn test_update_all_artists_prompt_confirm_submit() {
    let mut app = App::test_default();
    app
      .data
      .lidarr_data
      .artists
      .set_items(vec![Artist::default()]);
    app.data.lidarr_data.prompt_confirm = true;
    app.push_navigation_stack(ActiveLidarrBlock::Artists.into());
    app.push_navigation_stack(ActiveLidarrBlock::UpdateAllArtistsPrompt.into());

    LibraryHandler::new(
      DEFAULT_KEYBINDINGS.submit.key,
      &mut app,
      ActiveLidarrBlock::UpdateAllArtistsPrompt,
      None,
    )
    .handle();

    assert!(app.data.lidarr_data.prompt_confirm);
    assert_some_eq_x!(
      &app.data.lidarr_data.prompt_confirm_action,
      &LidarrEvent::UpdateAllArtists
    );
    assert_navigation_popped!(app, ActiveLidarrBlock::Artists.into());
  }

  #[test]
  fn test_update_all_artists_prompt_decline_submit() {
    let mut app = App::test_default();
    app
      .data
      .lidarr_data
      .artists
      .set_items(vec![Artist::default()]);
    app.push_navigation_stack(ActiveLidarrBlock::Artists.into());
    app.push_navigation_stack(ActiveLidarrBlock::UpdateAllArtistsPrompt.into());

    LibraryHandler::new(
      DEFAULT_KEYBINDINGS.submit.key,
      &mut app,
      ActiveLidarrBlock::UpdateAllArtistsPrompt,
      None,
    )
    .handle();

    assert!(!app.data.lidarr_data.prompt_confirm);
    assert_none!(app.data.lidarr_data.prompt_confirm_action);
    assert_navigation_popped!(app, ActiveLidarrBlock::Artists.into());
  }

  #[test]
  fn test_update_all_artists_prompt_esc() {
    let mut app = App::test_default();
    app.push_navigation_stack(ActiveLidarrBlock::Artists.into());
    app.push_navigation_stack(ActiveLidarrBlock::UpdateAllArtistsPrompt.into());
    app.data.lidarr_data.prompt_confirm = true;

    LibraryHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveLidarrBlock::UpdateAllArtistsPrompt,
      None,
    )
    .handle();

    assert_navigation_popped!(app, ActiveLidarrBlock::Artists.into());
    assert!(!app.data.lidarr_data.prompt_confirm);
  }

  #[test]
  fn test_update_all_artists_prompt_left_right() {
    let mut app = App::test_default();
    app.push_navigation_stack(ActiveLidarrBlock::Artists.into());
    app.push_navigation_stack(ActiveLidarrBlock::UpdateAllArtistsPrompt.into());

    LibraryHandler::new(
      DEFAULT_KEYBINDINGS.left.key,
      &mut app,
      ActiveLidarrBlock::UpdateAllArtistsPrompt,
      None,
    )
    .handle();

    assert!(app.data.lidarr_data.prompt_confirm);

    LibraryHandler::new(
      DEFAULT_KEYBINDINGS.right.key,
      &mut app,
      ActiveLidarrBlock::UpdateAllArtistsPrompt,
      None,
    )
    .handle();

    assert!(!app.data.lidarr_data.prompt_confirm);
  }

  #[test]
  fn test_update_all_artists_prompt_confirm_key() {
    let mut app = App::test_default();
    app
      .data
      .lidarr_data
      .artists
      .set_items(vec![Artist::default()]);
    app.push_navigation_stack(ActiveLidarrBlock::Artists.into());
    app.push_navigation_stack(ActiveLidarrBlock::UpdateAllArtistsPrompt.into());

    LibraryHandler::new(
      DEFAULT_KEYBINDINGS.confirm.key,
      &mut app,
      ActiveLidarrBlock::UpdateAllArtistsPrompt,
      None,
    )
    .handle();

    assert!(app.data.lidarr_data.prompt_confirm);
    assert_some_eq_x!(
      &app.data.lidarr_data.prompt_confirm_action,
      &LidarrEvent::UpdateAllArtists
    );
    assert_navigation_popped!(app, ActiveLidarrBlock::Artists.into());
  }

  fn artists_vec() -> Vec<Artist> {
    vec![
      Artist {
        id: 3,
        artist_name: "Test Artist 1".into(),
        artist_type: Some("Group".to_owned()),
        status: ArtistStatus::Ended,
        quality_profile_id: 1,
        metadata_profile_id: 1,
        monitored: false,
        tags: vec![Number::from(1), Number::from(2)],
        statistics: Some(ArtistStatistics {
          album_count: 5,
          track_count: 50,
          size_on_disk: 789,
          ..ArtistStatistics::default()
        }),
        ..Artist::default()
      },
      Artist {
        id: 2,
        artist_name: "Test Artist 2".into(),
        artist_type: Some("Solo".to_owned()),
        status: ArtistStatus::Continuing,
        quality_profile_id: 2,
        metadata_profile_id: 2,
        monitored: false,
        tags: vec![Number::from(1), Number::from(3)],
        statistics: Some(ArtistStatistics {
          album_count: 10,
          track_count: 100,
          size_on_disk: 456,
          ..ArtistStatistics::default()
        }),
        ..Artist::default()
      },
      Artist {
        id: 1,
        artist_name: "Test Artist 3".into(),
        artist_type: None,
        status: ArtistStatus::Deleted,
        quality_profile_id: 3,
        metadata_profile_id: 3,
        monitored: true,
        tags: vec![Number::from(2), Number::from(3)],
        statistics: Some(ArtistStatistics {
          album_count: 3,
          track_count: 30,
          size_on_disk: 123,
          ..ArtistStatistics::default()
        }),
        ..Artist::default()
      },
    ]
  }

  #[test]
  fn test_delegates_delete_artist_blocks_to_delete_artist_handler() {
    let mut app = App::test_default();
    app
      .data
      .lidarr_data
      .artists
      .set_items(vec![Artist::default()]);
    app.push_navigation_stack(ActiveLidarrBlock::Artists.into());
    app.push_navigation_stack(ActiveLidarrBlock::DeleteArtistPrompt.into());

    LibraryHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveLidarrBlock::DeleteArtistPrompt,
      None,
    )
    .handle();

    assert_eq!(app.get_current_route(), ActiveLidarrBlock::Artists.into());
  }

  #[rstest]
  fn test_delegates_edit_artist_blocks_to_edit_artist_handler(
    #[values(
      ActiveLidarrBlock::EditArtistPrompt,
      ActiveLidarrBlock::EditArtistSelectMetadataProfile,
      ActiveLidarrBlock::EditArtistSelectMonitorNewItems,
      ActiveLidarrBlock::EditArtistSelectQualityProfile,
      ActiveLidarrBlock::EditArtistTagsInput,
      ActiveLidarrBlock::EditArtistPathInput,
    )]
    active_lidarr_block: ActiveLidarrBlock,
  ) {
    let mut app = App::test_default();
    app
      .data
      .lidarr_data
      .artists
      .set_items(vec![Artist::default()]);
    app.data.lidarr_data.edit_artist_modal = Some(EditArtistModal::default());
    app.push_navigation_stack(ActiveLidarrBlock::Artists.into());
    app.push_navigation_stack(active_lidarr_block.into());

    LibraryHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      active_lidarr_block,
      None,
    )
    .handle();

    assert_eq!(app.get_current_route(), ActiveLidarrBlock::Artists.into());
  }

  #[test]
  fn test_edit_key() {
    let mut app = App::test_default();
    app
      .data
      .lidarr_data
      .artists
      .set_items(vec![Artist::default()]);
    app.data.lidarr_data.quality_profile_map =
      bimap::BiMap::from_iter([(0i64, "Default Quality".to_owned())]);
    app.data.lidarr_data.metadata_profile_map =
      bimap::BiMap::from_iter([(0i64, "Default Metadata".to_owned())]);
    app.push_navigation_stack(ActiveLidarrBlock::Artists.into());

    LibraryHandler::new(
      DEFAULT_KEYBINDINGS.edit.key,
      &mut app,
      ActiveLidarrBlock::Artists,
      None,
    )
    .handle();

    assert_navigation_pushed!(app, ActiveLidarrBlock::EditArtistPrompt.into());
    assert_modal_present!(app.data.lidarr_data.edit_artist_modal);
    assert_eq!(
      app.data.lidarr_data.selected_block.blocks,
      EDIT_ARTIST_SELECTION_BLOCKS
    );
  }

  #[test]
  fn test_edit_key_no_op_when_not_ready() {
    let mut app = App::test_default();
    app.is_loading = true;
    app.push_navigation_stack(ActiveLidarrBlock::Artists.into());
    app
      .data
      .lidarr_data
      .artists
      .set_items(vec![Artist::default()]);

    LibraryHandler::new(
      DEFAULT_KEYBINDINGS.edit.key,
      &mut app,
      ActiveLidarrBlock::Artists,
      None,
    )
    .handle();

    assert_eq!(app.get_current_route(), ActiveLidarrBlock::Artists.into());
    assert_modal_absent!(app.data.lidarr_data.edit_artist_modal);
  }

  #[test]
  fn test_refresh_key() {
    let mut app = App::test_default();
    app
      .data
      .lidarr_data
      .artists
      .set_items(vec![Artist::default()]);
    app.push_navigation_stack(ActiveLidarrBlock::Artists.into());

    LibraryHandler::new(
      DEFAULT_KEYBINDINGS.refresh.key,
      &mut app,
      ActiveLidarrBlock::Artists,
      None,
    )
    .handle();

    assert_eq!(app.get_current_route(), ActiveLidarrBlock::Artists.into());
    assert!(app.should_refresh);
  }
}
