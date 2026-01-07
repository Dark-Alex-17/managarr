#[cfg(test)]
mod tests {
  use bimap::BiMap;
  use crate::app::lidarr::lidarr_context_clues::ARTISTS_CONTEXT_CLUES;
  use crate::models::servarr_data::lidarr::lidarr_data::{DELETE_ARTIST_BLOCKS, DELETE_ARTIST_SELECTION_BLOCKS, EDIT_ARTIST_BLOCKS, EDIT_ARTIST_SELECTION_BLOCKS};
  use crate::models::{
    BlockSelectionState, Route,
    servarr_data::lidarr::lidarr_data::{ActiveLidarrBlock, LIBRARY_BLOCKS, LidarrData},
  };
  use chrono::{DateTime, Utc};
  use pretty_assertions::{assert_eq, assert_str_eq};
  use serde_json::Number;

  #[test]
  fn test_from_active_lidarr_block_to_route() {
    assert_eq!(
      Route::from(ActiveLidarrBlock::Artists),
      Route::Lidarr(ActiveLidarrBlock::Artists, None)
    );
  }

  #[test]
  fn test_from_tuple_to_route_with_context() {
    assert_eq!(
      Route::from((ActiveLidarrBlock::Artists, Some(ActiveLidarrBlock::Artists))),
      Route::Lidarr(ActiveLidarrBlock::Artists, Some(ActiveLidarrBlock::Artists),)
    );
  }

  #[test]
  fn test_reset_delete_artist_preferences() {
    let mut lidarr_data = LidarrData {
      delete_artist_files: true,
      add_import_list_exclusion: true,
      ..LidarrData::default()
    };

    lidarr_data.reset_delete_artist_preferences();

    assert!(!lidarr_data.delete_artist_files);
    assert!(!lidarr_data.add_import_list_exclusion);
  }

  #[test]
  fn test_tag_ids_to_display() {
    let mut tags_map = BiMap::new();
    tags_map.insert(3, "test 3".to_owned());
    tags_map.insert(2, "test 2".to_owned());
    tags_map.insert(1, "test 1".to_owned());
    let lidarr_data = LidarrData {
      tags_map,
      ..LidarrData::default()
    };

    assert_str_eq!(lidarr_data.tag_ids_to_display(&[Number::from(1), Number::from(2)]), "test 1, test 2");
  }

  #[test]
  fn test_sorted_quality_profile_names() {
    let mut quality_profile_map = BiMap::new();
    quality_profile_map.insert(3, "test 3".to_owned());
    quality_profile_map.insert(2, "test 2".to_owned());
    quality_profile_map.insert(1, "test 1".to_owned());
    let lidarr_data = LidarrData {
      quality_profile_map,
      ..LidarrData::default()
    };
    let expected_quality_profile_vec = vec!["test 1".to_owned(), "test 2".to_owned(), "test 3".to_owned()];

    assert_iter_eq!(lidarr_data.sorted_quality_profile_names(), expected_quality_profile_vec);
  }

  #[test]
  fn test_sorted_metadata_profile_names() {
    let mut metadata_profile_map = BiMap::new();
    metadata_profile_map.insert(3, "test 3".to_owned());
    metadata_profile_map.insert(2, "test 2".to_owned());
    metadata_profile_map.insert(1, "test 1".to_owned());
    let lidarr_data = LidarrData {
      metadata_profile_map,
      ..LidarrData::default()
    };
    let expected_metadata_profile_vec = vec!["test 1".to_owned(), "test 2".to_owned(), "test 3".to_owned()];

    assert_iter_eq!(lidarr_data.sorted_metadata_profile_names(), expected_metadata_profile_vec);
  }

  #[test]
  fn test_lidarr_data_default() {
    let lidarr_data = LidarrData::default();

    assert!(!lidarr_data.add_import_list_exclusion);
    assert_is_empty!(lidarr_data.artists);
    assert!(!lidarr_data.delete_artist_files);
    assert_is_empty!(lidarr_data.disk_space_vec);
    assert_is_empty!(lidarr_data.downloads);
    assert_none!(lidarr_data.edit_artist_modal);
    assert_is_empty!(lidarr_data.metadata_profile_map);
    assert!(!lidarr_data.prompt_confirm);
    assert_none!(lidarr_data.prompt_confirm_action);
    assert_is_empty!(lidarr_data.quality_profile_map);
    assert_is_empty!(lidarr_data.root_folders);
    assert_eq!(lidarr_data.selected_block, BlockSelectionState::default());
    assert_eq!(lidarr_data.start_time, <DateTime<Utc>>::default());
    assert_is_empty!(lidarr_data.tags_map);
    assert_is_empty!(lidarr_data.version);

    assert_eq!(lidarr_data.main_tabs.tabs.len(), 1);

    assert_str_eq!(lidarr_data.main_tabs.tabs[0].title, "Library");
    assert_eq!(
      lidarr_data.main_tabs.tabs[0].route,
      ActiveLidarrBlock::Artists.into()
    );
    assert_some_eq_x!(
      &lidarr_data.main_tabs.tabs[0].contextual_help,
      &ARTISTS_CONTEXT_CLUES
    );
    assert_none!(lidarr_data.main_tabs.tabs[0].config);
  }

  #[test]
  fn test_library_blocks_contains_expected_blocks() {
    assert_eq!(LIBRARY_BLOCKS.len(), 7);
    assert!(LIBRARY_BLOCKS.contains(&ActiveLidarrBlock::Artists));
    assert!(LIBRARY_BLOCKS.contains(&ActiveLidarrBlock::ArtistsSortPrompt));
    assert!(LIBRARY_BLOCKS.contains(&ActiveLidarrBlock::SearchArtists));
    assert!(LIBRARY_BLOCKS.contains(&ActiveLidarrBlock::SearchArtistsError));
    assert!(LIBRARY_BLOCKS.contains(&ActiveLidarrBlock::FilterArtists));
    assert!(LIBRARY_BLOCKS.contains(&ActiveLidarrBlock::FilterArtistsError));
    assert!(LIBRARY_BLOCKS.contains(&ActiveLidarrBlock::UpdateAllArtistsPrompt));
  }

  #[test]
  fn test_delete_artist_blocks_contents() {
    assert_eq!(DELETE_ARTIST_BLOCKS.len(), 4);
    assert!(DELETE_ARTIST_BLOCKS.contains(&ActiveLidarrBlock::DeleteArtistPrompt));
    assert!(DELETE_ARTIST_BLOCKS.contains(&ActiveLidarrBlock::DeleteArtistConfirmPrompt));
    assert!(DELETE_ARTIST_BLOCKS.contains(&ActiveLidarrBlock::DeleteArtistToggleDeleteFile));
    assert!(DELETE_ARTIST_BLOCKS.contains(&ActiveLidarrBlock::DeleteArtistToggleAddListExclusion));
  }

  #[test]
  fn test_delete_artist_selection_blocks_ordering() {
    let mut delete_artist_block_iter = DELETE_ARTIST_SELECTION_BLOCKS.iter();

    assert_eq!(
      delete_artist_block_iter.next().unwrap(),
      &[ActiveLidarrBlock::DeleteArtistToggleDeleteFile]
    );
    assert_eq!(
      delete_artist_block_iter.next().unwrap(),
      &[ActiveLidarrBlock::DeleteArtistToggleAddListExclusion]
    );
    assert_eq!(
      delete_artist_block_iter.next().unwrap(),
      &[ActiveLidarrBlock::DeleteArtistConfirmPrompt]
    );
    assert_none!(delete_artist_block_iter.next());
  }

  #[test]
  fn test_edit_artist_blocks() {
    assert_eq!(EDIT_ARTIST_BLOCKS.len(), 8);
    assert!(EDIT_ARTIST_BLOCKS.contains(&ActiveLidarrBlock::EditArtistPrompt));
    assert!(EDIT_ARTIST_BLOCKS.contains(&ActiveLidarrBlock::EditArtistConfirmPrompt));
    assert!(EDIT_ARTIST_BLOCKS.contains(&ActiveLidarrBlock::EditArtistPathInput));
    assert!(EDIT_ARTIST_BLOCKS.contains(&ActiveLidarrBlock::EditArtistSelectMetadataProfile));
    assert!(EDIT_ARTIST_BLOCKS.contains(&ActiveLidarrBlock::EditArtistSelectMonitorNewItems));
    assert!(EDIT_ARTIST_BLOCKS.contains(&ActiveLidarrBlock::EditArtistSelectQualityProfile));
    assert!(EDIT_ARTIST_BLOCKS.contains(&ActiveLidarrBlock::EditArtistTagsInput));
    assert!(EDIT_ARTIST_BLOCKS.contains(&ActiveLidarrBlock::EditArtistToggleMonitored));
  }

  #[test]
  fn test_edit_artist_selection_blocks_ordering() {
    let mut edit_artist_block_iter = EDIT_ARTIST_SELECTION_BLOCKS.iter();

    assert_eq!(edit_artist_block_iter.next().unwrap(), &[ActiveLidarrBlock::EditArtistToggleMonitored]);
    assert_eq!(edit_artist_block_iter.next().unwrap(), &[ActiveLidarrBlock::EditArtistSelectMonitorNewItems]);
    assert_eq!(edit_artist_block_iter.next().unwrap(), &[ActiveLidarrBlock::EditArtistSelectQualityProfile]);
    assert_eq!(edit_artist_block_iter.next().unwrap(), &[ActiveLidarrBlock::EditArtistSelectMetadataProfile]);
    assert_eq!(edit_artist_block_iter.next().unwrap(), &[ActiveLidarrBlock::EditArtistPathInput]);
    assert_eq!(edit_artist_block_iter.next().unwrap(), &[ActiveLidarrBlock::EditArtistTagsInput]);
    assert_eq!(edit_artist_block_iter.next().unwrap(), &[ActiveLidarrBlock::EditArtistConfirmPrompt]);
    assert_none!(edit_artist_block_iter.next());
  }
}
