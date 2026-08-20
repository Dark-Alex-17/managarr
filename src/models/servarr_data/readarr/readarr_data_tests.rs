#[cfg(test)]
mod tests {
  use crate::app::context_clues::{
    BLOCKLIST_CONTEXT_CLUES, DOWNLOADS_CONTEXT_CLUES, HISTORY_CONTEXT_CLUES,
    INDEXERS_CONTEXT_CLUES, ROOT_FOLDERS_CONTEXT_CLUES, SYSTEM_CONTEXT_CLUES,
  };
  use crate::app::readarr::readarr_context_clues::{
    AUTHOR_DETAILS_CONTEXT_CLUES, AUTHOR_HISTORY_CONTEXT_CLUES, AUTHORS_CONTEXT_CLUES,
    MANUAL_AUTHOR_SEARCH_CONTEXT_CLUES,
  };
  use crate::models::readarr_models::{Book, ReadarrHistoryItem, ReadarrRelease};
  use crate::models::servarr_data::readarr::readarr_data::{
    ADD_AUTHOR_BLOCKS, ADD_AUTHOR_SELECTION_BLOCKS, ADD_ROOT_FOLDER_BLOCKS,
    ADD_ROOT_FOLDER_SELECTION_BLOCKS, AUTHOR_DETAILS_BLOCKS, ActiveReadarrBlock, BLOCKLIST_BLOCKS,
    BOOK_DETAILS_BLOCKS, DELETE_AUTHOR_BLOCKS, DELETE_AUTHOR_SELECTION_BLOCKS, DELETE_BOOK_BLOCKS,
    DELETE_BOOK_SELECTION_BLOCKS, DOWNLOADS_BLOCKS, EDIT_AUTHOR_BLOCKS,
    EDIT_AUTHOR_SELECTION_BLOCKS, EDIT_INDEXER_BLOCKS, EDIT_INDEXER_NZB_SELECTION_BLOCKS,
    EDIT_INDEXER_TORRENT_SELECTION_BLOCKS, EDITION_DETAILS_BLOCKS, HISTORY_BLOCKS,
    INDEXER_SETTINGS_BLOCKS, INDEXER_SETTINGS_SELECTION_BLOCKS, INDEXERS_BLOCKS, LIBRARY_BLOCKS,
    ROOT_FOLDERS_BLOCKS, ReadarrData, SYSTEM_DETAILS_BLOCKS,
  };
  use crate::models::{BlockSelectionState, Route};
  use bimap::BiMap;
  use chrono::{DateTime, Utc};
  use pretty_assertions::{assert_eq, assert_str_eq};
  use serde_json::Number;

  #[test]
  fn test_from_active_readarr_block_to_route() {
    assert_eq!(
      Route::from(ActiveReadarrBlock::Authors),
      Route::Readarr(ActiveReadarrBlock::Authors, None)
    );
  }

  #[test]
  fn test_from_tuple_to_route_with_context() {
    assert_eq!(
      Route::from((
        ActiveReadarrBlock::Authors,
        Some(ActiveReadarrBlock::Authors)
      )),
      Route::Readarr(
        ActiveReadarrBlock::Authors,
        Some(ActiveReadarrBlock::Authors),
      )
    );
  }

  #[test]
  fn test_reset_delete_preferences() {
    let mut readarr_data = ReadarrData {
      delete_files: true,
      add_import_list_exclusion: true,
      ..ReadarrData::default()
    };

    readarr_data.reset_delete_preferences();

    assert!(!readarr_data.delete_files);
    assert!(!readarr_data.add_import_list_exclusion);
  }

  #[test]
  fn test_reset_author_info_tabs() {
    let mut readarr_data = ReadarrData::default();
    readarr_data.books.set_items(vec![Book::default()]);
    readarr_data
      .author_releases
      .set_items(vec![ReadarrRelease::default()]);
    readarr_data
      .author_history
      .set_items(vec![ReadarrHistoryItem::default()]);
    readarr_data.author_info_tabs.index = 1;

    readarr_data.reset_author_info_tabs();

    assert_is_empty!(readarr_data.books);
    assert_is_empty!(readarr_data.author_releases);
    assert_is_empty!(readarr_data.author_history);
    assert_eq!(readarr_data.author_info_tabs.index, 0);
  }

  #[test]
  fn test_tag_ids_to_display() {
    let mut tags_map = BiMap::new();
    tags_map.insert(3, "test 3".to_owned());
    tags_map.insert(2, "test 2".to_owned());
    tags_map.insert(1, "test 1".to_owned());
    let readarr_data = ReadarrData {
      tags_map,
      ..ReadarrData::default()
    };

    assert_str_eq!(
      readarr_data.tag_ids_to_display(&[Number::from(1), Number::from(2)]),
      "test 1, test 2"
    );
  }

  #[test]
  fn test_sorted_quality_profile_names() {
    let mut quality_profile_map = BiMap::new();
    quality_profile_map.insert(3, "test 1".to_owned());
    quality_profile_map.insert(2, "test 2".to_owned());
    quality_profile_map.insert(1, "test 3".to_owned());
    let readarr_data = ReadarrData {
      quality_profile_map,
      ..ReadarrData::default()
    };
    let expected_quality_profile_vec = vec![
      "test 3".to_owned(),
      "test 2".to_owned(),
      "test 1".to_owned(),
    ];

    assert_iter_eq!(
      readarr_data.sorted_quality_profile_names(),
      expected_quality_profile_vec
    );
  }

  #[test]
  fn test_sorted_metadata_profile_names() {
    let mut metadata_profile_map = BiMap::new();
    metadata_profile_map.insert(3, "test 1".to_owned());
    metadata_profile_map.insert(2, "test 2".to_owned());
    metadata_profile_map.insert(1, "test 3".to_owned());
    let readarr_data = ReadarrData {
      metadata_profile_map,
      ..ReadarrData::default()
    };
    let expected_metadata_profile_vec = vec![
      "test 3".to_owned(),
      "test 2".to_owned(),
      "test 1".to_owned(),
    ];

    assert_iter_eq!(
      readarr_data.sorted_metadata_profile_names(),
      expected_metadata_profile_vec
    );
  }

  #[test]
  fn test_readarr_data_default() {
    let readarr_data = ReadarrData::default();

    assert_none!(readarr_data.add_author_modal);
    assert_none!(readarr_data.add_author_search);
    assert!(!readarr_data.add_import_list_exclusion);
    assert_none!(readarr_data.add_root_folder_modal);
    assert_none!(readarr_data.add_searched_authors);
    assert_is_empty!(readarr_data.author_history);
    assert_is_empty!(readarr_data.author_releases);
    assert_is_empty!(readarr_data.authors);
    assert_is_empty!(readarr_data.blocklist);
    assert_none!(readarr_data.book_details_modal);
    assert_is_empty!(readarr_data.books);
    assert!(!readarr_data.delete_files);
    assert_is_empty!(readarr_data.disk_space_vec);
    assert_is_empty!(readarr_data.downloads);
    assert_none!(readarr_data.edit_author_modal);
    assert_none!(readarr_data.edit_indexer_modal);
    assert_is_empty!(readarr_data.history);
    assert_none!(readarr_data.indexer_settings);
    assert_none!(readarr_data.indexer_test_all_results);
    assert_none!(readarr_data.indexer_test_errors);
    assert_is_empty!(readarr_data.indexers);
    assert_is_empty!(readarr_data.log_details);
    assert_is_empty!(readarr_data.logs);
    assert_is_empty!(readarr_data.metadata_profile_map);
    assert!(!readarr_data.prompt_confirm);
    assert_none!(readarr_data.prompt_confirm_action);
    assert_is_empty!(readarr_data.quality_profile_map);
    assert_is_empty!(readarr_data.queued_events);
    assert_is_empty!(readarr_data.root_folders);
    assert_eq!(readarr_data.selected_block, BlockSelectionState::default());
    assert_eq!(readarr_data.start_time, <DateTime<Utc>>::default());
    assert_is_empty!(readarr_data.tags_map);
    assert_is_empty!(readarr_data.tasks);
    assert_is_empty!(readarr_data.updates);
    assert_is_empty!(readarr_data.version);

    assert_eq!(readarr_data.main_tabs.tabs.len(), 7);

    assert_str_eq!(readarr_data.main_tabs.tabs[0].title, "Library");
    assert_eq!(
      readarr_data.main_tabs.tabs[0].route,
      ActiveReadarrBlock::Authors.into()
    );
    assert_some_eq_x!(
      &readarr_data.main_tabs.tabs[0].contextual_help,
      &AUTHORS_CONTEXT_CLUES
    );
    assert_none!(readarr_data.main_tabs.tabs[0].config);

    assert_str_eq!(readarr_data.main_tabs.tabs[1].title, "Downloads");
    assert_eq!(
      readarr_data.main_tabs.tabs[1].route,
      ActiveReadarrBlock::Downloads.into()
    );
    assert_some_eq_x!(
      &readarr_data.main_tabs.tabs[1].contextual_help,
      &DOWNLOADS_CONTEXT_CLUES
    );
    assert_none!(readarr_data.main_tabs.tabs[1].config);

    assert_str_eq!(readarr_data.main_tabs.tabs[2].title, "Blocklist");
    assert_eq!(
      readarr_data.main_tabs.tabs[2].route,
      ActiveReadarrBlock::Blocklist.into()
    );
    assert_some_eq_x!(
      &readarr_data.main_tabs.tabs[2].contextual_help,
      &BLOCKLIST_CONTEXT_CLUES
    );
    assert_none!(readarr_data.main_tabs.tabs[2].config);

    assert_str_eq!(readarr_data.main_tabs.tabs[3].title, "History");
    assert_eq!(
      readarr_data.main_tabs.tabs[3].route,
      ActiveReadarrBlock::History.into()
    );
    assert_some_eq_x!(
      &readarr_data.main_tabs.tabs[3].contextual_help,
      &HISTORY_CONTEXT_CLUES
    );
    assert_none!(readarr_data.main_tabs.tabs[3].config);

    assert_str_eq!(readarr_data.main_tabs.tabs[4].title, "Root Folders");
    assert_eq!(
      readarr_data.main_tabs.tabs[4].route,
      ActiveReadarrBlock::RootFolders.into()
    );
    assert_some_eq_x!(
      &readarr_data.main_tabs.tabs[4].contextual_help,
      &ROOT_FOLDERS_CONTEXT_CLUES
    );
    assert_none!(readarr_data.main_tabs.tabs[4].config);

    assert_str_eq!(readarr_data.main_tabs.tabs[5].title, "Indexers");
    assert_eq!(
      readarr_data.main_tabs.tabs[5].route,
      ActiveReadarrBlock::Indexers.into()
    );
    assert_some_eq_x!(
      &readarr_data.main_tabs.tabs[5].contextual_help,
      &INDEXERS_CONTEXT_CLUES
    );
    assert_none!(readarr_data.main_tabs.tabs[5].config);

    assert_str_eq!(readarr_data.main_tabs.tabs[6].title, "System");
    assert_eq!(
      readarr_data.main_tabs.tabs[6].route,
      ActiveReadarrBlock::System.into()
    );
    assert_some_eq_x!(
      &readarr_data.main_tabs.tabs[6].contextual_help,
      &SYSTEM_CONTEXT_CLUES
    );
    assert_none!(readarr_data.main_tabs.tabs[6].config);

    assert_eq!(readarr_data.author_info_tabs.tabs.len(), 3);

    assert_str_eq!(readarr_data.author_info_tabs.tabs[0].title, "Books");
    assert_eq!(
      readarr_data.author_info_tabs.tabs[0].route,
      ActiveReadarrBlock::AuthorDetails.into()
    );
    assert_some_eq_x!(
      &readarr_data.author_info_tabs.tabs[0].contextual_help,
      &AUTHOR_DETAILS_CONTEXT_CLUES
    );
    assert_none!(readarr_data.author_info_tabs.tabs[0].config);

    assert_str_eq!(readarr_data.author_info_tabs.tabs[1].title, "History");
    assert_eq!(
      readarr_data.author_info_tabs.tabs[1].route,
      ActiveReadarrBlock::AuthorHistory.into()
    );
    assert_some_eq_x!(
      &readarr_data.author_info_tabs.tabs[1].contextual_help,
      &AUTHOR_HISTORY_CONTEXT_CLUES
    );
    assert_none!(readarr_data.author_info_tabs.tabs[1].config);

    assert_str_eq!(readarr_data.author_info_tabs.tabs[2].title, "Author Search");
    assert_eq!(
      readarr_data.author_info_tabs.tabs[2].route,
      ActiveReadarrBlock::ManualAuthorSearch.into()
    );
    assert_some_eq_x!(
      &readarr_data.author_info_tabs.tabs[2].contextual_help,
      &MANUAL_AUTHOR_SEARCH_CONTEXT_CLUES
    );
    assert_none!(readarr_data.author_info_tabs.tabs[2].config);
  }

  #[test]
  fn test_library_blocks_contains_expected_blocks() {
    assert_eq!(LIBRARY_BLOCKS.len(), 7);
    assert!(LIBRARY_BLOCKS.contains(&ActiveReadarrBlock::Authors));
    assert!(LIBRARY_BLOCKS.contains(&ActiveReadarrBlock::AuthorsSortPrompt));
    assert!(LIBRARY_BLOCKS.contains(&ActiveReadarrBlock::FilterAuthors));
    assert!(LIBRARY_BLOCKS.contains(&ActiveReadarrBlock::FilterAuthorsError));
    assert!(LIBRARY_BLOCKS.contains(&ActiveReadarrBlock::SearchAuthors));
    assert!(LIBRARY_BLOCKS.contains(&ActiveReadarrBlock::SearchAuthorsError));
    assert!(LIBRARY_BLOCKS.contains(&ActiveReadarrBlock::UpdateAllAuthorsPrompt));
  }

  #[test]
  fn test_author_details_blocks_contains_expected_blocks() {
    assert_eq!(AUTHOR_DETAILS_BLOCKS.len(), 15);
    assert!(AUTHOR_DETAILS_BLOCKS.contains(&ActiveReadarrBlock::AuthorDetails));
    assert!(AUTHOR_DETAILS_BLOCKS.contains(&ActiveReadarrBlock::AuthorHistory));
    assert!(AUTHOR_DETAILS_BLOCKS.contains(&ActiveReadarrBlock::AuthorHistoryDetails));
    assert!(AUTHOR_DETAILS_BLOCKS.contains(&ActiveReadarrBlock::AuthorHistorySortPrompt));
    assert!(AUTHOR_DETAILS_BLOCKS.contains(&ActiveReadarrBlock::AutomaticallySearchAuthorPrompt));
    assert!(AUTHOR_DETAILS_BLOCKS.contains(&ActiveReadarrBlock::FilterAuthorHistory));
    assert!(AUTHOR_DETAILS_BLOCKS.contains(&ActiveReadarrBlock::FilterAuthorHistoryError));
    assert!(AUTHOR_DETAILS_BLOCKS.contains(&ActiveReadarrBlock::ManualAuthorSearch));
    assert!(AUTHOR_DETAILS_BLOCKS.contains(&ActiveReadarrBlock::ManualAuthorSearchConfirmPrompt));
    assert!(AUTHOR_DETAILS_BLOCKS.contains(&ActiveReadarrBlock::ManualAuthorSearchSortPrompt));
    assert!(AUTHOR_DETAILS_BLOCKS.contains(&ActiveReadarrBlock::SearchBooks));
    assert!(AUTHOR_DETAILS_BLOCKS.contains(&ActiveReadarrBlock::SearchBooksError));
    assert!(AUTHOR_DETAILS_BLOCKS.contains(&ActiveReadarrBlock::SearchAuthorHistory));
    assert!(AUTHOR_DETAILS_BLOCKS.contains(&ActiveReadarrBlock::SearchAuthorHistoryError));
    assert!(AUTHOR_DETAILS_BLOCKS.contains(&ActiveReadarrBlock::UpdateAndScanAuthorPrompt));
  }

  #[test]
  fn test_book_details_blocks_contents() {
    assert_eq!(BOOK_DETAILS_BLOCKS.len(), 16);
    assert!(BOOK_DETAILS_BLOCKS.contains(&ActiveReadarrBlock::BookDetails));
    assert!(BOOK_DETAILS_BLOCKS.contains(&ActiveReadarrBlock::BookHistory));
    assert!(BOOK_DETAILS_BLOCKS.contains(&ActiveReadarrBlock::BookHistoryDetails));
    assert!(BOOK_DETAILS_BLOCKS.contains(&ActiveReadarrBlock::BookHistorySortPrompt));
    assert!(BOOK_DETAILS_BLOCKS.contains(&ActiveReadarrBlock::AutomaticallySearchBookPrompt));
    assert!(BOOK_DETAILS_BLOCKS.contains(&ActiveReadarrBlock::DeleteBookFilePrompt));
    assert!(BOOK_DETAILS_BLOCKS.contains(&ActiveReadarrBlock::FilterBookHistory));
    assert!(BOOK_DETAILS_BLOCKS.contains(&ActiveReadarrBlock::FilterBookHistoryError));
    assert!(BOOK_DETAILS_BLOCKS.contains(&ActiveReadarrBlock::BookFileInfo));
    assert!(BOOK_DETAILS_BLOCKS.contains(&ActiveReadarrBlock::ManualBookSearch));
    assert!(BOOK_DETAILS_BLOCKS.contains(&ActiveReadarrBlock::ManualBookSearchConfirmPrompt));
    assert!(BOOK_DETAILS_BLOCKS.contains(&ActiveReadarrBlock::ManualBookSearchSortPrompt));
    assert!(BOOK_DETAILS_BLOCKS.contains(&ActiveReadarrBlock::SearchBookHistory));
    assert!(BOOK_DETAILS_BLOCKS.contains(&ActiveReadarrBlock::SearchBookHistoryError));
    assert!(BOOK_DETAILS_BLOCKS.contains(&ActiveReadarrBlock::SearchEditions));
    assert!(BOOK_DETAILS_BLOCKS.contains(&ActiveReadarrBlock::SearchEditionsError));
  }

  #[test]
  fn test_book_details_blocks_excludes_book_edition_details() {
    assert!(!BOOK_DETAILS_BLOCKS.contains(&ActiveReadarrBlock::BookEditionDetails));
  }

  #[test]
  fn test_edition_details_blocks_contents() {
    assert_eq!(EDITION_DETAILS_BLOCKS.len(), 1);
    assert!(EDITION_DETAILS_BLOCKS.contains(&ActiveReadarrBlock::BookEditionDetails));
  }

  #[test]
  fn test_blocklist_blocks_contents() {
    assert_eq!(BLOCKLIST_BLOCKS.len(), 5);
    assert!(BLOCKLIST_BLOCKS.contains(&ActiveReadarrBlock::Blocklist));
    assert!(BLOCKLIST_BLOCKS.contains(&ActiveReadarrBlock::BlocklistItemDetails));
    assert!(BLOCKLIST_BLOCKS.contains(&ActiveReadarrBlock::DeleteBlocklistItemPrompt));
    assert!(BLOCKLIST_BLOCKS.contains(&ActiveReadarrBlock::BlocklistClearAllItemsPrompt));
    assert!(BLOCKLIST_BLOCKS.contains(&ActiveReadarrBlock::BlocklistSortPrompt));
  }

  #[test]
  fn test_downloads_blocks_contains_expected_blocks() {
    assert_eq!(DOWNLOADS_BLOCKS.len(), 3);
    assert!(DOWNLOADS_BLOCKS.contains(&ActiveReadarrBlock::Downloads));
    assert!(DOWNLOADS_BLOCKS.contains(&ActiveReadarrBlock::DeleteDownloadPrompt));
    assert!(DOWNLOADS_BLOCKS.contains(&ActiveReadarrBlock::UpdateDownloadsPrompt));
  }

  #[test]
  fn test_history_blocks_contains_expected_blocks() {
    assert_eq!(HISTORY_BLOCKS.len(), 7);
    assert!(HISTORY_BLOCKS.contains(&ActiveReadarrBlock::History));
    assert!(HISTORY_BLOCKS.contains(&ActiveReadarrBlock::HistoryItemDetails));
    assert!(HISTORY_BLOCKS.contains(&ActiveReadarrBlock::HistorySortPrompt));
    assert!(HISTORY_BLOCKS.contains(&ActiveReadarrBlock::SearchHistory));
    assert!(HISTORY_BLOCKS.contains(&ActiveReadarrBlock::SearchHistoryError));
    assert!(HISTORY_BLOCKS.contains(&ActiveReadarrBlock::FilterHistory));
    assert!(HISTORY_BLOCKS.contains(&ActiveReadarrBlock::FilterHistoryError));
  }

  #[test]
  fn test_add_author_blocks_contents() {
    assert_eq!(ADD_AUTHOR_BLOCKS.len(), 12);
    assert!(ADD_AUTHOR_BLOCKS.contains(&ActiveReadarrBlock::AddAuthorAlreadyInLibrary));
    assert!(ADD_AUTHOR_BLOCKS.contains(&ActiveReadarrBlock::AddAuthorConfirmPrompt));
    assert!(ADD_AUTHOR_BLOCKS.contains(&ActiveReadarrBlock::AddAuthorEmptySearchResults));
    assert!(ADD_AUTHOR_BLOCKS.contains(&ActiveReadarrBlock::AddAuthorPrompt));
    assert!(ADD_AUTHOR_BLOCKS.contains(&ActiveReadarrBlock::AddAuthorSearchInput));
    assert!(ADD_AUTHOR_BLOCKS.contains(&ActiveReadarrBlock::AddAuthorSearchResults));
    assert!(ADD_AUTHOR_BLOCKS.contains(&ActiveReadarrBlock::AddAuthorSelectMetadataProfile));
    assert!(ADD_AUTHOR_BLOCKS.contains(&ActiveReadarrBlock::AddAuthorSelectMonitor));
    assert!(ADD_AUTHOR_BLOCKS.contains(&ActiveReadarrBlock::AddAuthorSelectMonitorNewItems));
    assert!(ADD_AUTHOR_BLOCKS.contains(&ActiveReadarrBlock::AddAuthorSelectQualityProfile));
    assert!(ADD_AUTHOR_BLOCKS.contains(&ActiveReadarrBlock::AddAuthorSelectRootFolder));
    assert!(ADD_AUTHOR_BLOCKS.contains(&ActiveReadarrBlock::AddAuthorTagsInput));
  }

  #[test]
  fn test_add_author_selection_blocks_ordering() {
    let mut add_author_block_iter = ADD_AUTHOR_SELECTION_BLOCKS.iter();

    assert_eq!(
      add_author_block_iter.next().unwrap(),
      &[ActiveReadarrBlock::AddAuthorSelectRootFolder]
    );
    assert_eq!(
      add_author_block_iter.next().unwrap(),
      &[ActiveReadarrBlock::AddAuthorSelectMonitor]
    );
    assert_eq!(
      add_author_block_iter.next().unwrap(),
      &[ActiveReadarrBlock::AddAuthorSelectMonitorNewItems]
    );
    assert_eq!(
      add_author_block_iter.next().unwrap(),
      &[ActiveReadarrBlock::AddAuthorSelectQualityProfile]
    );
    assert_eq!(
      add_author_block_iter.next().unwrap(),
      &[ActiveReadarrBlock::AddAuthorSelectMetadataProfile]
    );
    assert_eq!(
      add_author_block_iter.next().unwrap(),
      &[ActiveReadarrBlock::AddAuthorTagsInput]
    );
    assert_eq!(
      add_author_block_iter.next().unwrap(),
      &[ActiveReadarrBlock::AddAuthorConfirmPrompt]
    );
    assert_none!(add_author_block_iter.next());
  }

  #[test]
  fn test_delete_author_blocks_contents() {
    assert_eq!(DELETE_AUTHOR_BLOCKS.len(), 4);
    assert!(DELETE_AUTHOR_BLOCKS.contains(&ActiveReadarrBlock::DeleteAuthorPrompt));
    assert!(DELETE_AUTHOR_BLOCKS.contains(&ActiveReadarrBlock::DeleteAuthorConfirmPrompt));
    assert!(DELETE_AUTHOR_BLOCKS.contains(&ActiveReadarrBlock::DeleteAuthorToggleDeleteFile));
    assert!(DELETE_AUTHOR_BLOCKS.contains(&ActiveReadarrBlock::DeleteAuthorToggleAddListExclusion));
  }

  #[test]
  fn test_delete_author_selection_blocks_ordering() {
    let mut delete_author_block_iter = DELETE_AUTHOR_SELECTION_BLOCKS.iter();

    assert_eq!(
      delete_author_block_iter.next().unwrap(),
      &[ActiveReadarrBlock::DeleteAuthorToggleDeleteFile]
    );
    assert_eq!(
      delete_author_block_iter.next().unwrap(),
      &[ActiveReadarrBlock::DeleteAuthorToggleAddListExclusion]
    );
    assert_eq!(
      delete_author_block_iter.next().unwrap(),
      &[ActiveReadarrBlock::DeleteAuthorConfirmPrompt]
    );
    assert_none!(delete_author_block_iter.next());
  }

  #[test]
  fn test_delete_book_blocks_contents() {
    assert_eq!(DELETE_BOOK_BLOCKS.len(), 4);
    assert!(DELETE_BOOK_BLOCKS.contains(&ActiveReadarrBlock::DeleteBookPrompt));
    assert!(DELETE_BOOK_BLOCKS.contains(&ActiveReadarrBlock::DeleteBookConfirmPrompt));
    assert!(DELETE_BOOK_BLOCKS.contains(&ActiveReadarrBlock::DeleteBookToggleDeleteFile));
    assert!(DELETE_BOOK_BLOCKS.contains(&ActiveReadarrBlock::DeleteBookToggleAddListExclusion));
  }

  #[test]
  fn test_delete_book_selection_blocks_ordering() {
    let mut delete_book_block_iter = DELETE_BOOK_SELECTION_BLOCKS.iter();

    assert_eq!(
      delete_book_block_iter.next().unwrap(),
      &[ActiveReadarrBlock::DeleteBookToggleDeleteFile]
    );
    assert_eq!(
      delete_book_block_iter.next().unwrap(),
      &[ActiveReadarrBlock::DeleteBookToggleAddListExclusion]
    );
    assert_eq!(
      delete_book_block_iter.next().unwrap(),
      &[ActiveReadarrBlock::DeleteBookConfirmPrompt]
    );
    assert_none!(delete_book_block_iter.next());
  }

  #[test]
  fn test_edit_author_blocks() {
    assert_eq!(EDIT_AUTHOR_BLOCKS.len(), 8);
    assert!(EDIT_AUTHOR_BLOCKS.contains(&ActiveReadarrBlock::EditAuthorPrompt));
    assert!(EDIT_AUTHOR_BLOCKS.contains(&ActiveReadarrBlock::EditAuthorConfirmPrompt));
    assert!(EDIT_AUTHOR_BLOCKS.contains(&ActiveReadarrBlock::EditAuthorPathInput));
    assert!(EDIT_AUTHOR_BLOCKS.contains(&ActiveReadarrBlock::EditAuthorSelectMetadataProfile));
    assert!(EDIT_AUTHOR_BLOCKS.contains(&ActiveReadarrBlock::EditAuthorSelectMonitorNewItems));
    assert!(EDIT_AUTHOR_BLOCKS.contains(&ActiveReadarrBlock::EditAuthorSelectQualityProfile));
    assert!(EDIT_AUTHOR_BLOCKS.contains(&ActiveReadarrBlock::EditAuthorTagsInput));
    assert!(EDIT_AUTHOR_BLOCKS.contains(&ActiveReadarrBlock::EditAuthorToggleMonitored));
  }

  #[test]
  fn test_edit_author_selection_blocks_ordering() {
    let mut edit_author_block_iter = EDIT_AUTHOR_SELECTION_BLOCKS.iter();

    assert_eq!(
      edit_author_block_iter.next().unwrap(),
      &[ActiveReadarrBlock::EditAuthorToggleMonitored]
    );
    assert_eq!(
      edit_author_block_iter.next().unwrap(),
      &[ActiveReadarrBlock::EditAuthorSelectMonitorNewItems]
    );
    assert_eq!(
      edit_author_block_iter.next().unwrap(),
      &[ActiveReadarrBlock::EditAuthorSelectQualityProfile]
    );
    assert_eq!(
      edit_author_block_iter.next().unwrap(),
      &[ActiveReadarrBlock::EditAuthorSelectMetadataProfile]
    );
    assert_eq!(
      edit_author_block_iter.next().unwrap(),
      &[ActiveReadarrBlock::EditAuthorPathInput]
    );
    assert_eq!(
      edit_author_block_iter.next().unwrap(),
      &[ActiveReadarrBlock::EditAuthorTagsInput]
    );
    assert_eq!(
      edit_author_block_iter.next().unwrap(),
      &[ActiveReadarrBlock::EditAuthorConfirmPrompt]
    );
    assert_none!(edit_author_block_iter.next());
  }

  #[test]
  fn test_root_folders_blocks_contents() {
    assert_eq!(ROOT_FOLDERS_BLOCKS.len(), 2);
    assert!(ROOT_FOLDERS_BLOCKS.contains(&ActiveReadarrBlock::RootFolders));
    assert!(ROOT_FOLDERS_BLOCKS.contains(&ActiveReadarrBlock::DeleteRootFolderPrompt));
  }

  #[test]
  fn test_add_root_folder_blocks_contents() {
    assert_eq!(ADD_ROOT_FOLDER_BLOCKS.len(), 9);
    assert!(ADD_ROOT_FOLDER_BLOCKS.contains(&ActiveReadarrBlock::AddRootFolderPrompt));
    assert!(ADD_ROOT_FOLDER_BLOCKS.contains(&ActiveReadarrBlock::AddRootFolderConfirmPrompt));
    assert!(ADD_ROOT_FOLDER_BLOCKS.contains(&ActiveReadarrBlock::AddRootFolderNameInput));
    assert!(ADD_ROOT_FOLDER_BLOCKS.contains(&ActiveReadarrBlock::AddRootFolderPathInput));
    assert!(ADD_ROOT_FOLDER_BLOCKS.contains(&ActiveReadarrBlock::AddRootFolderSelectMonitor));
    assert!(
      ADD_ROOT_FOLDER_BLOCKS.contains(&ActiveReadarrBlock::AddRootFolderSelectMonitorNewItems)
    );
    assert!(
      ADD_ROOT_FOLDER_BLOCKS.contains(&ActiveReadarrBlock::AddRootFolderSelectQualityProfile)
    );
    assert!(
      ADD_ROOT_FOLDER_BLOCKS.contains(&ActiveReadarrBlock::AddRootFolderSelectMetadataProfile)
    );
    assert!(ADD_ROOT_FOLDER_BLOCKS.contains(&ActiveReadarrBlock::AddRootFolderTagsInput));
  }

  #[test]
  fn test_add_root_folder_selection_blocks_ordering() {
    let mut add_root_folder_block_iter = ADD_ROOT_FOLDER_SELECTION_BLOCKS.iter();

    assert_eq!(
      add_root_folder_block_iter.next().unwrap(),
      &[ActiveReadarrBlock::AddRootFolderNameInput]
    );
    assert_eq!(
      add_root_folder_block_iter.next().unwrap(),
      &[ActiveReadarrBlock::AddRootFolderPathInput]
    );
    assert_eq!(
      add_root_folder_block_iter.next().unwrap(),
      &[ActiveReadarrBlock::AddRootFolderSelectMonitor]
    );
    assert_eq!(
      add_root_folder_block_iter.next().unwrap(),
      &[ActiveReadarrBlock::AddRootFolderSelectMonitorNewItems]
    );
    assert_eq!(
      add_root_folder_block_iter.next().unwrap(),
      &[ActiveReadarrBlock::AddRootFolderSelectQualityProfile]
    );
    assert_eq!(
      add_root_folder_block_iter.next().unwrap(),
      &[ActiveReadarrBlock::AddRootFolderSelectMetadataProfile]
    );
    assert_eq!(
      add_root_folder_block_iter.next().unwrap(),
      &[ActiveReadarrBlock::AddRootFolderTagsInput]
    );
    assert_eq!(
      add_root_folder_block_iter.next().unwrap(),
      &[ActiveReadarrBlock::AddRootFolderConfirmPrompt]
    );
    assert_none!(add_root_folder_block_iter.next());
  }

  #[test]
  fn test_edit_indexer_blocks_contents() {
    assert_eq!(EDIT_INDEXER_BLOCKS.len(), 11);
    assert!(EDIT_INDEXER_BLOCKS.contains(&ActiveReadarrBlock::EditIndexerPrompt));
    assert!(EDIT_INDEXER_BLOCKS.contains(&ActiveReadarrBlock::EditIndexerConfirmPrompt));
    assert!(EDIT_INDEXER_BLOCKS.contains(&ActiveReadarrBlock::EditIndexerApiKeyInput));
    assert!(EDIT_INDEXER_BLOCKS.contains(&ActiveReadarrBlock::EditIndexerNameInput));
    assert!(EDIT_INDEXER_BLOCKS.contains(&ActiveReadarrBlock::EditIndexerSeedRatioInput));
    assert!(EDIT_INDEXER_BLOCKS.contains(&ActiveReadarrBlock::EditIndexerToggleEnableRss));
    assert!(
      EDIT_INDEXER_BLOCKS.contains(&ActiveReadarrBlock::EditIndexerToggleEnableAutomaticSearch)
    );
    assert!(
      EDIT_INDEXER_BLOCKS.contains(&ActiveReadarrBlock::EditIndexerToggleEnableInteractiveSearch)
    );
    assert!(EDIT_INDEXER_BLOCKS.contains(&ActiveReadarrBlock::EditIndexerPriorityInput));
    assert!(EDIT_INDEXER_BLOCKS.contains(&ActiveReadarrBlock::EditIndexerUrlInput));
    assert!(EDIT_INDEXER_BLOCKS.contains(&ActiveReadarrBlock::EditIndexerTagsInput));
  }

  #[test]
  fn test_edit_indexer_torrent_selection_blocks_ordering() {
    let mut edit_indexer_block_iter = EDIT_INDEXER_TORRENT_SELECTION_BLOCKS.iter();

    assert_eq!(
      edit_indexer_block_iter.next().unwrap(),
      &[
        ActiveReadarrBlock::EditIndexerNameInput,
        ActiveReadarrBlock::EditIndexerUrlInput
      ]
    );
    assert_eq!(
      edit_indexer_block_iter.next().unwrap(),
      &[
        ActiveReadarrBlock::EditIndexerToggleEnableRss,
        ActiveReadarrBlock::EditIndexerApiKeyInput
      ]
    );
    assert_eq!(
      edit_indexer_block_iter.next().unwrap(),
      &[
        ActiveReadarrBlock::EditIndexerToggleEnableAutomaticSearch,
        ActiveReadarrBlock::EditIndexerSeedRatioInput
      ]
    );
    assert_eq!(
      edit_indexer_block_iter.next().unwrap(),
      &[
        ActiveReadarrBlock::EditIndexerToggleEnableInteractiveSearch,
        ActiveReadarrBlock::EditIndexerTagsInput
      ]
    );
    assert_eq!(
      edit_indexer_block_iter.next().unwrap(),
      &[
        ActiveReadarrBlock::EditIndexerPriorityInput,
        ActiveReadarrBlock::EditIndexerConfirmPrompt
      ]
    );
    assert_eq!(
      edit_indexer_block_iter.next().unwrap(),
      &[
        ActiveReadarrBlock::EditIndexerConfirmPrompt,
        ActiveReadarrBlock::EditIndexerConfirmPrompt
      ]
    );
    assert_none!(edit_indexer_block_iter.next());
  }

  #[test]
  fn test_edit_indexer_nzb_selection_blocks_ordering() {
    let mut edit_indexer_block_iter = EDIT_INDEXER_NZB_SELECTION_BLOCKS.iter();

    assert_eq!(
      edit_indexer_block_iter.next().unwrap(),
      &[
        ActiveReadarrBlock::EditIndexerNameInput,
        ActiveReadarrBlock::EditIndexerUrlInput
      ]
    );
    assert_eq!(
      edit_indexer_block_iter.next().unwrap(),
      &[
        ActiveReadarrBlock::EditIndexerToggleEnableRss,
        ActiveReadarrBlock::EditIndexerApiKeyInput
      ]
    );
    assert_eq!(
      edit_indexer_block_iter.next().unwrap(),
      &[
        ActiveReadarrBlock::EditIndexerToggleEnableAutomaticSearch,
        ActiveReadarrBlock::EditIndexerTagsInput
      ]
    );
    assert_eq!(
      edit_indexer_block_iter.next().unwrap(),
      &[
        ActiveReadarrBlock::EditIndexerToggleEnableInteractiveSearch,
        ActiveReadarrBlock::EditIndexerPriorityInput
      ]
    );
    assert_eq!(
      edit_indexer_block_iter.next().unwrap(),
      &[
        ActiveReadarrBlock::EditIndexerConfirmPrompt,
        ActiveReadarrBlock::EditIndexerConfirmPrompt
      ]
    );
    assert_none!(edit_indexer_block_iter.next());
  }

  #[test]
  fn test_indexer_settings_blocks_contents() {
    assert_eq!(INDEXER_SETTINGS_BLOCKS.len(), 6);
    assert!(INDEXER_SETTINGS_BLOCKS.contains(&ActiveReadarrBlock::AllIndexerSettingsPrompt));
    assert!(INDEXER_SETTINGS_BLOCKS.contains(&ActiveReadarrBlock::IndexerSettingsConfirmPrompt));
    assert!(INDEXER_SETTINGS_BLOCKS.contains(&ActiveReadarrBlock::IndexerSettingsMaximumSizeInput));
    assert!(INDEXER_SETTINGS_BLOCKS.contains(&ActiveReadarrBlock::IndexerSettingsMinimumAgeInput));
    assert!(INDEXER_SETTINGS_BLOCKS.contains(&ActiveReadarrBlock::IndexerSettingsRetentionInput));
    assert!(
      INDEXER_SETTINGS_BLOCKS.contains(&ActiveReadarrBlock::IndexerSettingsRssSyncIntervalInput)
    );
  }

  #[test]
  fn test_indexer_settings_selection_blocks_ordering() {
    let mut indexer_settings_block_iter = INDEXER_SETTINGS_SELECTION_BLOCKS.iter();

    assert_eq!(
      indexer_settings_block_iter.next().unwrap(),
      &[ActiveReadarrBlock::IndexerSettingsMinimumAgeInput]
    );
    assert_eq!(
      indexer_settings_block_iter.next().unwrap(),
      &[ActiveReadarrBlock::IndexerSettingsRetentionInput]
    );
    assert_eq!(
      indexer_settings_block_iter.next().unwrap(),
      &[ActiveReadarrBlock::IndexerSettingsMaximumSizeInput]
    );
    assert_eq!(
      indexer_settings_block_iter.next().unwrap(),
      &[ActiveReadarrBlock::IndexerSettingsRssSyncIntervalInput]
    );
    assert_eq!(
      indexer_settings_block_iter.next().unwrap(),
      &[ActiveReadarrBlock::IndexerSettingsConfirmPrompt]
    );
    assert_none!(indexer_settings_block_iter.next());
  }

  #[test]
  fn test_indexers_blocks_contents() {
    assert_eq!(INDEXERS_BLOCKS.len(), 3);
    assert!(INDEXERS_BLOCKS.contains(&ActiveReadarrBlock::Indexers));
    assert!(INDEXERS_BLOCKS.contains(&ActiveReadarrBlock::DeleteIndexerPrompt));
    assert!(INDEXERS_BLOCKS.contains(&ActiveReadarrBlock::TestIndexer));
  }

  #[test]
  fn test_indexers_blocks_excludes_test_all_indexers() {
    assert!(!INDEXERS_BLOCKS.contains(&ActiveReadarrBlock::TestAllIndexers));
  }

  #[test]
  fn test_system_details_blocks_contents() {
    assert_eq!(SYSTEM_DETAILS_BLOCKS.len(), 5);
    assert!(SYSTEM_DETAILS_BLOCKS.contains(&ActiveReadarrBlock::SystemLogs));
    assert!(SYSTEM_DETAILS_BLOCKS.contains(&ActiveReadarrBlock::SystemQueuedEvents));
    assert!(SYSTEM_DETAILS_BLOCKS.contains(&ActiveReadarrBlock::SystemTasks));
    assert!(SYSTEM_DETAILS_BLOCKS.contains(&ActiveReadarrBlock::SystemTaskStartConfirmPrompt));
    assert!(SYSTEM_DETAILS_BLOCKS.contains(&ActiveReadarrBlock::SystemUpdates));
  }
}
