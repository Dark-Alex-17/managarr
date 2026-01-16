#[cfg(test)]
mod tests {
  use crate::app::context_clues::{
    DOWNLOADS_CONTEXT_CLUES, HISTORY_CONTEXT_CLUES, INDEXERS_CONTEXT_CLUES,
    ROOT_FOLDERS_CONTEXT_CLUES, SYSTEM_CONTEXT_CLUES,
  };
  use crate::app::lidarr::lidarr_context_clues::{
    ARTIST_DETAILS_CONTEXT_CLUES, ARTIST_HISTORY_CONTEXT_CLUES, ARTISTS_CONTEXT_CLUES,
    MANUAL_ARTIST_SEARCH_CONTEXT_CLUES,
  };
  use crate::models::lidarr_models::{Album, LidarrHistoryItem, LidarrRelease};
  use crate::models::servarr_data::lidarr::lidarr_data::{
    ADD_ARTIST_BLOCKS, ADD_ARTIST_SELECTION_BLOCKS, ADD_ROOT_FOLDER_BLOCKS, ALBUM_DETAILS_BLOCKS,
    ARTIST_DETAILS_BLOCKS, DELETE_ALBUM_BLOCKS, DELETE_ALBUM_SELECTION_BLOCKS,
    DELETE_ARTIST_BLOCKS, DELETE_ARTIST_SELECTION_BLOCKS, DOWNLOADS_BLOCKS, EDIT_ARTIST_BLOCKS,
    EDIT_ARTIST_SELECTION_BLOCKS, EDIT_INDEXER_BLOCKS, EDIT_INDEXER_NZB_SELECTION_BLOCKS,
    EDIT_INDEXER_TORRENT_SELECTION_BLOCKS, HISTORY_BLOCKS, INDEXER_SETTINGS_BLOCKS,
    INDEXER_SETTINGS_SELECTION_BLOCKS, INDEXERS_BLOCKS, ROOT_FOLDERS_BLOCKS, SYSTEM_DETAILS_BLOCKS,
  };
  use crate::models::{
    BlockSelectionState, Route,
    servarr_data::lidarr::lidarr_data::{ActiveLidarrBlock, LIBRARY_BLOCKS, LidarrData},
  };
  use bimap::BiMap;
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
  fn test_reset_delete_preferences() {
    let mut lidarr_data = LidarrData {
      delete_files: true,
      add_import_list_exclusion: true,
      ..LidarrData::default()
    };

    lidarr_data.reset_delete_preferences();

    assert!(!lidarr_data.delete_files);
    assert!(!lidarr_data.add_import_list_exclusion);
  }

  #[test]
  fn test_reset_artist_info_tabs() {
    let mut lidarr_data = LidarrData::default();
    lidarr_data.albums.set_items(vec![Album::default()]);
    lidarr_data
      .discography_releases
      .set_items(vec![LidarrRelease::default()]);
    lidarr_data
      .artist_history
      .set_items(vec![LidarrHistoryItem::default()]);
    lidarr_data.artist_info_tabs.index = 1;

    lidarr_data.reset_artist_info_tabs();

    assert_is_empty!(lidarr_data.albums);
    assert_is_empty!(lidarr_data.discography_releases);
    assert_is_empty!(lidarr_data.artist_history);
    assert_eq!(lidarr_data.artist_info_tabs.index, 0);
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

    assert_str_eq!(
      lidarr_data.tag_ids_to_display(&[Number::from(1), Number::from(2)]),
      "test 1, test 2"
    );
  }

  #[test]
  fn test_sorted_quality_profile_names() {
    let mut quality_profile_map = BiMap::new();
    quality_profile_map.insert(3, "test 1".to_owned());
    quality_profile_map.insert(2, "test 2".to_owned());
    quality_profile_map.insert(1, "test 3".to_owned());
    let lidarr_data = LidarrData {
      quality_profile_map,
      ..LidarrData::default()
    };
    let expected_quality_profile_vec = vec![
      "test 3".to_owned(),
      "test 2".to_owned(),
      "test 1".to_owned(),
    ];

    assert_iter_eq!(
      lidarr_data.sorted_quality_profile_names(),
      expected_quality_profile_vec
    );
  }

  #[test]
  fn test_sorted_metadata_profile_names() {
    let mut metadata_profile_map = BiMap::new();
    metadata_profile_map.insert(3, "test 1".to_owned());
    metadata_profile_map.insert(2, "test 2".to_owned());
    metadata_profile_map.insert(1, "test 3".to_owned());
    let lidarr_data = LidarrData {
      metadata_profile_map,
      ..LidarrData::default()
    };
    let expected_metadata_profile_vec = vec![
      "test 3".to_owned(),
      "test 2".to_owned(),
      "test 1".to_owned(),
    ];

    assert_iter_eq!(
      lidarr_data.sorted_metadata_profile_names(),
      expected_metadata_profile_vec
    );
  }

  #[test]
  fn test_lidarr_data_default() {
    let lidarr_data = LidarrData::default();

    assert_none!(lidarr_data.add_artist_search);
    assert!(!lidarr_data.add_import_list_exclusion);
    assert_none!(lidarr_data.add_searched_artists);
    assert_is_empty!(lidarr_data.albums);
    assert_none!(lidarr_data.album_details_modal);
    assert_is_empty!(lidarr_data.artists);
    assert_is_empty!(lidarr_data.artist_history);
    assert!(!lidarr_data.delete_files);
    assert_is_empty!(lidarr_data.disk_space_vec);
    assert_is_empty!(lidarr_data.downloads);
    assert_none!(lidarr_data.edit_artist_modal);
    assert_none!(lidarr_data.add_root_folder_modal);
    assert_is_empty!(lidarr_data.discography_releases);
    assert_is_empty!(lidarr_data.history);
    assert_is_empty!(lidarr_data.logs);
    assert_is_empty!(lidarr_data.log_details);
    assert_is_empty!(lidarr_data.metadata_profile_map);
    assert!(!lidarr_data.prompt_confirm);
    assert_none!(lidarr_data.prompt_confirm_action);
    assert_is_empty!(lidarr_data.quality_profile_map);
    assert_is_empty!(lidarr_data.queued_events);
    assert_is_empty!(lidarr_data.root_folders);
    assert_eq!(lidarr_data.selected_block, BlockSelectionState::default());
    assert_eq!(lidarr_data.start_time, <DateTime<Utc>>::default());
    assert_is_empty!(lidarr_data.tags_map);
    assert_is_empty!(lidarr_data.tasks);
    assert_is_empty!(lidarr_data.updates);
    assert_is_empty!(lidarr_data.version);

    assert_eq!(lidarr_data.main_tabs.tabs.len(), 6);

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

    assert_str_eq!(lidarr_data.main_tabs.tabs[1].title, "Downloads");
    assert_eq!(
      lidarr_data.main_tabs.tabs[1].route,
      ActiveLidarrBlock::Downloads.into()
    );
    assert_some_eq_x!(
      &lidarr_data.main_tabs.tabs[1].contextual_help,
      &DOWNLOADS_CONTEXT_CLUES
    );
    assert_none!(lidarr_data.main_tabs.tabs[1].config);

    assert_str_eq!(lidarr_data.main_tabs.tabs[2].title, "History");
    assert_eq!(
      lidarr_data.main_tabs.tabs[2].route,
      ActiveLidarrBlock::History.into()
    );
    assert_some_eq_x!(
      &lidarr_data.main_tabs.tabs[2].contextual_help,
      &HISTORY_CONTEXT_CLUES
    );
    assert_none!(lidarr_data.main_tabs.tabs[2].config);

    assert_str_eq!(lidarr_data.main_tabs.tabs[3].title, "Root Folders");
    assert_eq!(
      lidarr_data.main_tabs.tabs[3].route,
      ActiveLidarrBlock::RootFolders.into()
    );
    assert_some_eq_x!(
      &lidarr_data.main_tabs.tabs[3].contextual_help,
      &ROOT_FOLDERS_CONTEXT_CLUES
    );
    assert_none!(lidarr_data.main_tabs.tabs[3].config);

    assert_str_eq!(lidarr_data.main_tabs.tabs[4].title, "Indexers");
    assert_eq!(
      lidarr_data.main_tabs.tabs[4].route,
      ActiveLidarrBlock::Indexers.into()
    );
    assert_some_eq_x!(
      &lidarr_data.main_tabs.tabs[4].contextual_help,
      &INDEXERS_CONTEXT_CLUES
    );
    assert_none!(lidarr_data.main_tabs.tabs[4].config);

    assert_str_eq!(lidarr_data.main_tabs.tabs[5].title, "System");
    assert_eq!(
      lidarr_data.main_tabs.tabs[5].route,
      ActiveLidarrBlock::System.into()
    );
    assert_some_eq_x!(
      &lidarr_data.main_tabs.tabs[5].contextual_help,
      &SYSTEM_CONTEXT_CLUES
    );
    assert_none!(lidarr_data.main_tabs.tabs[5].config);

    assert_eq!(lidarr_data.artist_info_tabs.tabs.len(), 3);
    assert_str_eq!(lidarr_data.artist_info_tabs.tabs[0].title, "Albums");
    assert_eq!(
      lidarr_data.artist_info_tabs.tabs[0].route,
      ActiveLidarrBlock::ArtistDetails.into()
    );
    assert_some_eq_x!(
      &lidarr_data.artist_info_tabs.tabs[0].contextual_help,
      &ARTIST_DETAILS_CONTEXT_CLUES
    );
    assert_none!(lidarr_data.artist_info_tabs.tabs[0].config);

    assert_str_eq!(lidarr_data.artist_info_tabs.tabs[1].title, "History");
    assert_eq!(
      lidarr_data.artist_info_tabs.tabs[1].route,
      ActiveLidarrBlock::ArtistHistory.into()
    );
    assert_some_eq_x!(
      &lidarr_data.artist_info_tabs.tabs[1].contextual_help,
      &ARTIST_HISTORY_CONTEXT_CLUES
    );
    assert_none!(lidarr_data.artist_info_tabs.tabs[1].config);

    assert_str_eq!(lidarr_data.artist_info_tabs.tabs[2].title, "Manual Search");
    assert_eq!(
      lidarr_data.artist_info_tabs.tabs[2].route,
      ActiveLidarrBlock::ManualArtistSearch.into()
    );
    assert_some_eq_x!(
      &lidarr_data.artist_info_tabs.tabs[2].contextual_help,
      &MANUAL_ARTIST_SEARCH_CONTEXT_CLUES
    );
    assert_none!(lidarr_data.artist_info_tabs.tabs[2].config);
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
  fn test_artist_details_blocks_contains_expected_blocks() {
    assert_eq!(ARTIST_DETAILS_BLOCKS.len(), 15);
    assert!(ARTIST_DETAILS_BLOCKS.contains(&ActiveLidarrBlock::ArtistDetails));
    assert!(ARTIST_DETAILS_BLOCKS.contains(&ActiveLidarrBlock::ArtistHistory));
    assert!(ARTIST_DETAILS_BLOCKS.contains(&ActiveLidarrBlock::ArtistHistoryDetails));
    assert!(ARTIST_DETAILS_BLOCKS.contains(&ActiveLidarrBlock::ArtistHistorySortPrompt));
    assert!(ARTIST_DETAILS_BLOCKS.contains(&ActiveLidarrBlock::AutomaticallySearchArtistPrompt));
    assert!(ARTIST_DETAILS_BLOCKS.contains(&ActiveLidarrBlock::FilterArtistHistory));
    assert!(ARTIST_DETAILS_BLOCKS.contains(&ActiveLidarrBlock::FilterArtistHistoryError));
    assert!(ARTIST_DETAILS_BLOCKS.contains(&ActiveLidarrBlock::ManualArtistSearch));
    assert!(ARTIST_DETAILS_BLOCKS.contains(&ActiveLidarrBlock::ManualArtistSearchConfirmPrompt));
    assert!(ARTIST_DETAILS_BLOCKS.contains(&ActiveLidarrBlock::ManualArtistSearchSortPrompt));
    assert!(ARTIST_DETAILS_BLOCKS.contains(&ActiveLidarrBlock::SearchAlbums));
    assert!(ARTIST_DETAILS_BLOCKS.contains(&ActiveLidarrBlock::SearchAlbumsError));
    assert!(ARTIST_DETAILS_BLOCKS.contains(&ActiveLidarrBlock::SearchArtistHistory));
    assert!(ARTIST_DETAILS_BLOCKS.contains(&ActiveLidarrBlock::SearchArtistHistoryError));
    assert!(ARTIST_DETAILS_BLOCKS.contains(&ActiveLidarrBlock::UpdateAndScanArtistPrompt));
  }

  #[test]
  fn test_album_details_blocks_contents() {
    assert_eq!(ALBUM_DETAILS_BLOCKS.len(), 15);
    assert!(ALBUM_DETAILS_BLOCKS.contains(&ActiveLidarrBlock::AlbumDetails));
    assert!(ALBUM_DETAILS_BLOCKS.contains(&ActiveLidarrBlock::AlbumHistory));
    assert!(ALBUM_DETAILS_BLOCKS.contains(&ActiveLidarrBlock::SearchTracks));
    assert!(ALBUM_DETAILS_BLOCKS.contains(&ActiveLidarrBlock::SearchTracksError));
    assert!(ALBUM_DETAILS_BLOCKS.contains(&ActiveLidarrBlock::AutomaticallySearchAlbumPrompt));
    assert!(ALBUM_DETAILS_BLOCKS.contains(&ActiveLidarrBlock::SearchAlbumHistory));
    assert!(ALBUM_DETAILS_BLOCKS.contains(&ActiveLidarrBlock::SearchAlbumHistoryError));
    assert!(ALBUM_DETAILS_BLOCKS.contains(&ActiveLidarrBlock::FilterAlbumHistory));
    assert!(ALBUM_DETAILS_BLOCKS.contains(&ActiveLidarrBlock::FilterAlbumHistoryError));
    assert!(ALBUM_DETAILS_BLOCKS.contains(&ActiveLidarrBlock::AlbumHistorySortPrompt));
    assert!(ALBUM_DETAILS_BLOCKS.contains(&ActiveLidarrBlock::AlbumHistoryDetails));
    assert!(ALBUM_DETAILS_BLOCKS.contains(&ActiveLidarrBlock::ManualAlbumSearch));
    assert!(ALBUM_DETAILS_BLOCKS.contains(&ActiveLidarrBlock::ManualAlbumSearchConfirmPrompt));
    assert!(ALBUM_DETAILS_BLOCKS.contains(&ActiveLidarrBlock::ManualAlbumSearchSortPrompt));
    assert!(ALBUM_DETAILS_BLOCKS.contains(&ActiveLidarrBlock::DeleteTrackFilePrompt));
  }

  #[test]
  fn test_downloads_blocks_contains_expected_blocks() {
    assert_eq!(DOWNLOADS_BLOCKS.len(), 3);
    assert!(DOWNLOADS_BLOCKS.contains(&ActiveLidarrBlock::Downloads));
    assert!(DOWNLOADS_BLOCKS.contains(&ActiveLidarrBlock::DeleteDownloadPrompt));
    assert!(DOWNLOADS_BLOCKS.contains(&ActiveLidarrBlock::UpdateDownloadsPrompt));
  }

  #[test]
  fn test_history_blocks_contains_expected_blocks() {
    assert_eq!(HISTORY_BLOCKS.len(), 7);
    assert!(HISTORY_BLOCKS.contains(&ActiveLidarrBlock::History));
    assert!(HISTORY_BLOCKS.contains(&ActiveLidarrBlock::HistoryItemDetails));
    assert!(HISTORY_BLOCKS.contains(&ActiveLidarrBlock::HistorySortPrompt));
    assert!(HISTORY_BLOCKS.contains(&ActiveLidarrBlock::SearchHistory));
    assert!(HISTORY_BLOCKS.contains(&ActiveLidarrBlock::SearchHistoryError));
    assert!(HISTORY_BLOCKS.contains(&ActiveLidarrBlock::FilterHistory));
    assert!(HISTORY_BLOCKS.contains(&ActiveLidarrBlock::FilterHistoryError));
  }

  #[test]
  fn test_add_artist_blocks_contents() {
    assert_eq!(ADD_ARTIST_BLOCKS.len(), 12);
    assert!(ADD_ARTIST_BLOCKS.contains(&ActiveLidarrBlock::AddArtistAlreadyInLibrary));
    assert!(ADD_ARTIST_BLOCKS.contains(&ActiveLidarrBlock::AddArtistConfirmPrompt));
    assert!(ADD_ARTIST_BLOCKS.contains(&ActiveLidarrBlock::AddArtistEmptySearchResults));
    assert!(ADD_ARTIST_BLOCKS.contains(&ActiveLidarrBlock::AddArtistPrompt));
    assert!(ADD_ARTIST_BLOCKS.contains(&ActiveLidarrBlock::AddArtistSearchInput));
    assert!(ADD_ARTIST_BLOCKS.contains(&ActiveLidarrBlock::AddArtistSearchResults));
    assert!(ADD_ARTIST_BLOCKS.contains(&ActiveLidarrBlock::AddArtistSelectMetadataProfile));
    assert!(ADD_ARTIST_BLOCKS.contains(&ActiveLidarrBlock::AddArtistSelectMonitor));
    assert!(ADD_ARTIST_BLOCKS.contains(&ActiveLidarrBlock::AddArtistSelectMonitorNewItems));
    assert!(ADD_ARTIST_BLOCKS.contains(&ActiveLidarrBlock::AddArtistSelectQualityProfile));
    assert!(ADD_ARTIST_BLOCKS.contains(&ActiveLidarrBlock::AddArtistSelectRootFolder));
    assert!(ADD_ARTIST_BLOCKS.contains(&ActiveLidarrBlock::AddArtistTagsInput));
  }

  #[test]
  fn test_add_artist_selection_blocks_ordering() {
    let mut add_artist_block_iter = ADD_ARTIST_SELECTION_BLOCKS.iter();

    assert_eq!(
      add_artist_block_iter.next().unwrap(),
      &[ActiveLidarrBlock::AddArtistSelectRootFolder]
    );
    assert_eq!(
      add_artist_block_iter.next().unwrap(),
      &[ActiveLidarrBlock::AddArtistSelectMonitor]
    );
    assert_eq!(
      add_artist_block_iter.next().unwrap(),
      &[ActiveLidarrBlock::AddArtistSelectMonitorNewItems]
    );
    assert_eq!(
      add_artist_block_iter.next().unwrap(),
      &[ActiveLidarrBlock::AddArtistSelectQualityProfile]
    );
    assert_eq!(
      add_artist_block_iter.next().unwrap(),
      &[ActiveLidarrBlock::AddArtistSelectMetadataProfile]
    );
    assert_eq!(
      add_artist_block_iter.next().unwrap(),
      &[ActiveLidarrBlock::AddArtistTagsInput]
    );
    assert_eq!(
      add_artist_block_iter.next().unwrap(),
      &[ActiveLidarrBlock::AddArtistConfirmPrompt]
    );
    assert_none!(add_artist_block_iter.next());
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
  fn test_delete_album_blocks_contents() {
    assert_eq!(DELETE_ALBUM_BLOCKS.len(), 4);
    assert!(DELETE_ALBUM_BLOCKS.contains(&ActiveLidarrBlock::DeleteAlbumPrompt));
    assert!(DELETE_ALBUM_BLOCKS.contains(&ActiveLidarrBlock::DeleteAlbumConfirmPrompt));
    assert!(DELETE_ALBUM_BLOCKS.contains(&ActiveLidarrBlock::DeleteAlbumToggleDeleteFile));
    assert!(DELETE_ALBUM_BLOCKS.contains(&ActiveLidarrBlock::DeleteAlbumToggleAddListExclusion));
  }

  #[test]
  fn test_delete_album_selection_blocks_ordering() {
    let mut delete_album_block_iter = DELETE_ALBUM_SELECTION_BLOCKS.iter();

    assert_eq!(
      delete_album_block_iter.next().unwrap(),
      &[ActiveLidarrBlock::DeleteAlbumToggleDeleteFile]
    );
    assert_eq!(
      delete_album_block_iter.next().unwrap(),
      &[ActiveLidarrBlock::DeleteAlbumToggleAddListExclusion]
    );
    assert_eq!(
      delete_album_block_iter.next().unwrap(),
      &[ActiveLidarrBlock::DeleteAlbumConfirmPrompt]
    );
    assert_none!(delete_album_block_iter.next());
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

    assert_eq!(
      edit_artist_block_iter.next().unwrap(),
      &[ActiveLidarrBlock::EditArtistToggleMonitored]
    );
    assert_eq!(
      edit_artist_block_iter.next().unwrap(),
      &[ActiveLidarrBlock::EditArtistSelectMonitorNewItems]
    );
    assert_eq!(
      edit_artist_block_iter.next().unwrap(),
      &[ActiveLidarrBlock::EditArtistSelectQualityProfile]
    );
    assert_eq!(
      edit_artist_block_iter.next().unwrap(),
      &[ActiveLidarrBlock::EditArtistSelectMetadataProfile]
    );
    assert_eq!(
      edit_artist_block_iter.next().unwrap(),
      &[ActiveLidarrBlock::EditArtistPathInput]
    );
    assert_eq!(
      edit_artist_block_iter.next().unwrap(),
      &[ActiveLidarrBlock::EditArtistTagsInput]
    );
    assert_eq!(
      edit_artist_block_iter.next().unwrap(),
      &[ActiveLidarrBlock::EditArtistConfirmPrompt]
    );
    assert_none!(edit_artist_block_iter.next());
  }

  #[test]
  fn test_root_folders_blocks_contents() {
    assert_eq!(ROOT_FOLDERS_BLOCKS.len(), 2);
    assert!(ROOT_FOLDERS_BLOCKS.contains(&ActiveLidarrBlock::RootFolders));
    assert!(ROOT_FOLDERS_BLOCKS.contains(&ActiveLidarrBlock::DeleteRootFolderPrompt));
  }

  #[test]
  fn test_edit_indexer_blocks_contents() {
    assert_eq!(EDIT_INDEXER_BLOCKS.len(), 11);
    assert!(EDIT_INDEXER_BLOCKS.contains(&ActiveLidarrBlock::EditIndexerPrompt));
    assert!(EDIT_INDEXER_BLOCKS.contains(&ActiveLidarrBlock::EditIndexerConfirmPrompt));
    assert!(EDIT_INDEXER_BLOCKS.contains(&ActiveLidarrBlock::EditIndexerApiKeyInput));
    assert!(EDIT_INDEXER_BLOCKS.contains(&ActiveLidarrBlock::EditIndexerNameInput));
    assert!(EDIT_INDEXER_BLOCKS.contains(&ActiveLidarrBlock::EditIndexerSeedRatioInput));
    assert!(EDIT_INDEXER_BLOCKS.contains(&ActiveLidarrBlock::EditIndexerToggleEnableRss));
    assert!(
      EDIT_INDEXER_BLOCKS.contains(&ActiveLidarrBlock::EditIndexerToggleEnableAutomaticSearch)
    );
    assert!(
      EDIT_INDEXER_BLOCKS.contains(&ActiveLidarrBlock::EditIndexerToggleEnableInteractiveSearch)
    );
    assert!(EDIT_INDEXER_BLOCKS.contains(&ActiveLidarrBlock::EditIndexerUrlInput));
    assert!(EDIT_INDEXER_BLOCKS.contains(&ActiveLidarrBlock::EditIndexerTagsInput));
    assert!(EDIT_INDEXER_BLOCKS.contains(&ActiveLidarrBlock::EditIndexerPriorityInput));
  }

  #[test]
  fn test_edit_indexer_nzb_selection_blocks_ordering() {
    let mut edit_indexer_nzb_selection_block_iter = EDIT_INDEXER_NZB_SELECTION_BLOCKS.iter();

    assert_eq!(
      edit_indexer_nzb_selection_block_iter.next().unwrap(),
      &[
        ActiveLidarrBlock::EditIndexerNameInput,
        ActiveLidarrBlock::EditIndexerUrlInput,
      ]
    );
    assert_eq!(
      edit_indexer_nzb_selection_block_iter.next().unwrap(),
      &[
        ActiveLidarrBlock::EditIndexerToggleEnableRss,
        ActiveLidarrBlock::EditIndexerApiKeyInput,
      ]
    );
    assert_eq!(
      edit_indexer_nzb_selection_block_iter.next().unwrap(),
      &[
        ActiveLidarrBlock::EditIndexerToggleEnableAutomaticSearch,
        ActiveLidarrBlock::EditIndexerTagsInput,
      ]
    );
    assert_eq!(
      edit_indexer_nzb_selection_block_iter.next().unwrap(),
      &[
        ActiveLidarrBlock::EditIndexerToggleEnableInteractiveSearch,
        ActiveLidarrBlock::EditIndexerPriorityInput,
      ]
    );
    assert_eq!(
      edit_indexer_nzb_selection_block_iter.next().unwrap(),
      &[
        ActiveLidarrBlock::EditIndexerConfirmPrompt,
        ActiveLidarrBlock::EditIndexerConfirmPrompt,
      ]
    );
    assert_eq!(edit_indexer_nzb_selection_block_iter.next(), None);
  }

  #[test]
  fn test_edit_indexer_torrent_selection_blocks_ordering() {
    let mut edit_indexer_torrent_selection_block_iter =
      EDIT_INDEXER_TORRENT_SELECTION_BLOCKS.iter();

    assert_eq!(
      edit_indexer_torrent_selection_block_iter.next().unwrap(),
      &[
        ActiveLidarrBlock::EditIndexerNameInput,
        ActiveLidarrBlock::EditIndexerUrlInput,
      ]
    );
    assert_eq!(
      edit_indexer_torrent_selection_block_iter.next().unwrap(),
      &[
        ActiveLidarrBlock::EditIndexerToggleEnableRss,
        ActiveLidarrBlock::EditIndexerApiKeyInput,
      ]
    );
    assert_eq!(
      edit_indexer_torrent_selection_block_iter.next().unwrap(),
      &[
        ActiveLidarrBlock::EditIndexerToggleEnableAutomaticSearch,
        ActiveLidarrBlock::EditIndexerSeedRatioInput,
      ]
    );
    assert_eq!(
      edit_indexer_torrent_selection_block_iter.next().unwrap(),
      &[
        ActiveLidarrBlock::EditIndexerToggleEnableInteractiveSearch,
        ActiveLidarrBlock::EditIndexerTagsInput,
      ]
    );
    assert_eq!(
      edit_indexer_torrent_selection_block_iter.next().unwrap(),
      &[
        ActiveLidarrBlock::EditIndexerPriorityInput,
        ActiveLidarrBlock::EditIndexerConfirmPrompt,
      ]
    );
    assert_eq!(
      edit_indexer_torrent_selection_block_iter.next().unwrap(),
      &[
        ActiveLidarrBlock::EditIndexerConfirmPrompt,
        ActiveLidarrBlock::EditIndexerConfirmPrompt,
      ]
    );
    assert_eq!(edit_indexer_torrent_selection_block_iter.next(), None);
  }

  #[test]
  fn test_indexer_settings_blocks_contents() {
    assert_eq!(INDEXER_SETTINGS_BLOCKS.len(), 6);
    assert!(INDEXER_SETTINGS_BLOCKS.contains(&ActiveLidarrBlock::AllIndexerSettingsPrompt));
    assert!(INDEXER_SETTINGS_BLOCKS.contains(&ActiveLidarrBlock::IndexerSettingsConfirmPrompt));
    assert!(INDEXER_SETTINGS_BLOCKS.contains(&ActiveLidarrBlock::IndexerSettingsMaximumSizeInput));
    assert!(INDEXER_SETTINGS_BLOCKS.contains(&ActiveLidarrBlock::IndexerSettingsMinimumAgeInput));
    assert!(INDEXER_SETTINGS_BLOCKS.contains(&ActiveLidarrBlock::IndexerSettingsRetentionInput));
    assert!(
      INDEXER_SETTINGS_BLOCKS.contains(&ActiveLidarrBlock::IndexerSettingsRssSyncIntervalInput)
    );
  }

  #[test]
  fn test_indexer_settings_selection_blocks_ordering() {
    let mut indexer_settings_block_iter = INDEXER_SETTINGS_SELECTION_BLOCKS.iter();

    assert_eq!(
      indexer_settings_block_iter.next().unwrap(),
      &[ActiveLidarrBlock::IndexerSettingsMinimumAgeInput,]
    );
    assert_eq!(
      indexer_settings_block_iter.next().unwrap(),
      &[ActiveLidarrBlock::IndexerSettingsRetentionInput,]
    );
    assert_eq!(
      indexer_settings_block_iter.next().unwrap(),
      &[ActiveLidarrBlock::IndexerSettingsMaximumSizeInput,]
    );
    assert_eq!(
      indexer_settings_block_iter.next().unwrap(),
      &[ActiveLidarrBlock::IndexerSettingsRssSyncIntervalInput,]
    );
    assert_eq!(
      indexer_settings_block_iter.next().unwrap(),
      &[ActiveLidarrBlock::IndexerSettingsConfirmPrompt,]
    );
    assert_eq!(indexer_settings_block_iter.next(), None);
  }

  #[test]
  fn test_indexers_blocks_contents() {
    assert_eq!(INDEXERS_BLOCKS.len(), 3);
    assert!(INDEXERS_BLOCKS.contains(&ActiveLidarrBlock::Indexers));
    assert!(INDEXERS_BLOCKS.contains(&ActiveLidarrBlock::DeleteIndexerPrompt));
    assert!(INDEXERS_BLOCKS.contains(&ActiveLidarrBlock::TestIndexer));
  }

  #[test]
  fn test_add_root_folder_blocks_contents() {
    assert_eq!(ADD_ROOT_FOLDER_BLOCKS.len(), 9);
    assert!(ADD_ROOT_FOLDER_BLOCKS.contains(&ActiveLidarrBlock::AddRootFolderPrompt));
    assert!(ADD_ROOT_FOLDER_BLOCKS.contains(&ActiveLidarrBlock::AddRootFolderConfirmPrompt));
    assert!(ADD_ROOT_FOLDER_BLOCKS.contains(&ActiveLidarrBlock::AddRootFolderNameInput));
    assert!(ADD_ROOT_FOLDER_BLOCKS.contains(&ActiveLidarrBlock::AddRootFolderPathInput));
    assert!(ADD_ROOT_FOLDER_BLOCKS.contains(&ActiveLidarrBlock::AddRootFolderSelectMonitor));
    assert!(
      ADD_ROOT_FOLDER_BLOCKS.contains(&ActiveLidarrBlock::AddRootFolderSelectMonitorNewItems)
    );
    assert!(ADD_ROOT_FOLDER_BLOCKS.contains(&ActiveLidarrBlock::AddRootFolderSelectQualityProfile));
    assert!(
      ADD_ROOT_FOLDER_BLOCKS.contains(&ActiveLidarrBlock::AddRootFolderSelectMetadataProfile)
    );
    assert!(ADD_ROOT_FOLDER_BLOCKS.contains(&ActiveLidarrBlock::AddRootFolderTagsInput));
  }

  #[test]
  fn test_system_details_blocks_contents() {
    assert_eq!(SYSTEM_DETAILS_BLOCKS.len(), 5);
    assert!(SYSTEM_DETAILS_BLOCKS.contains(&ActiveLidarrBlock::SystemLogs));
    assert!(SYSTEM_DETAILS_BLOCKS.contains(&ActiveLidarrBlock::SystemQueuedEvents));
    assert!(SYSTEM_DETAILS_BLOCKS.contains(&ActiveLidarrBlock::SystemTasks));
    assert!(SYSTEM_DETAILS_BLOCKS.contains(&ActiveLidarrBlock::SystemTaskStartConfirmPrompt));
    assert!(SYSTEM_DETAILS_BLOCKS.contains(&ActiveLidarrBlock::SystemUpdates));
  }
}
