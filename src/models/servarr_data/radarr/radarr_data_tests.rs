#[cfg(test)]
mod tests {
  mod radarr_data_tests {
    use chrono::{DateTime, Utc};
    use pretty_assertions::{assert_eq, assert_str_eq};

    use crate::app::context_clues::build_context_clue_string;
    use crate::app::radarr::radarr_context_clues::{
      BLOCKLIST_CONTEXT_CLUES, COLLECTIONS_CONTEXT_CLUES, DOWNLOADS_CONTEXT_CLUES,
      INDEXERS_CONTEXT_CLUES, LIBRARY_CONTEXT_CLUES, MANUAL_MOVIE_SEARCH_CONTEXTUAL_CONTEXT_CLUES,
      MANUAL_MOVIE_SEARCH_CONTEXT_CLUES, MOVIE_DETAILS_CONTEXT_CLUES, ROOT_FOLDERS_CONTEXT_CLUES,
      SYSTEM_CONTEXT_CLUES,
    };

    use crate::models::servarr_data::radarr::radarr_data::radarr_test_utils::utils;
    use crate::models::servarr_data::radarr::radarr_data::{ActiveRadarrBlock, RadarrData};
    use crate::models::Route;

    use crate::assert_movie_info_tabs_reset;
    use crate::models::BlockSelectionState;

    #[test]
    fn test_from_tuple_to_route_with_context() {
      assert_eq!(
        Route::from((
          ActiveRadarrBlock::AddMoviePrompt,
          Some(ActiveRadarrBlock::AddMovieSearchResults)
        )),
        Route::Radarr(
          ActiveRadarrBlock::AddMoviePrompt,
          Some(ActiveRadarrBlock::AddMovieSearchResults),
        )
      );
    }

    #[test]
    fn test_reset_delete_movie_preferences() {
      let mut radarr_data = utils::create_test_radarr_data();

      radarr_data.reset_delete_movie_preferences();

      assert!(!radarr_data.delete_movie_files);
      assert!(!radarr_data.add_list_exclusion);
    }

    #[test]
    fn test_reset_movie_info_tabs() {
      let mut radarr_data = utils::create_test_radarr_data();

      radarr_data.reset_movie_info_tabs();

      assert_movie_info_tabs_reset!(radarr_data);
    }

    #[test]
    fn test_radarr_data_defaults() {
      let radarr_data = RadarrData::default();

      assert!(radarr_data.root_folders.items.is_empty());
      assert_eq!(radarr_data.disk_space_vec, Vec::new());
      assert!(radarr_data.version.is_empty());
      assert_eq!(radarr_data.start_time, <DateTime<Utc>>::default());
      assert!(radarr_data.movies.items.is_empty());
      assert_eq!(radarr_data.selected_block, BlockSelectionState::default());
      assert!(radarr_data.downloads.items.is_empty());
      assert!(radarr_data.indexers.items.is_empty());
      assert!(radarr_data.blocklist.items.is_empty());
      assert!(radarr_data.quality_profile_map.is_empty());
      assert!(radarr_data.tags_map.is_empty());
      assert!(radarr_data.collections.items.is_empty());
      assert!(radarr_data.collection_movies.items.is_empty());
      assert!(radarr_data.logs.items.is_empty());
      assert!(radarr_data.log_details.items.is_empty());
      assert!(radarr_data.tasks.items.is_empty());
      assert!(radarr_data.queued_events.items.is_empty());
      assert!(radarr_data.updates.get_text().is_empty());
      assert!(radarr_data.add_movie_search.is_none());
      assert!(radarr_data.add_movie_modal.is_none());
      assert!(radarr_data.add_searched_movies.is_none());
      assert!(radarr_data.edit_movie_modal.is_none());
      assert!(radarr_data.edit_collection_modal.is_none());
      assert!(radarr_data.edit_root_folder.is_none());
      assert!(radarr_data.edit_indexer_modal.is_none());
      assert!(radarr_data.indexer_settings.is_none());
      assert!(radarr_data.indexer_test_error.is_none());
      assert!(radarr_data.indexer_test_all_results.is_none());
      assert!(radarr_data.movie_details_modal.is_none());
      assert!(radarr_data.prompt_confirm_action.is_none());
      assert!(!radarr_data.prompt_confirm);
      assert!(!radarr_data.delete_movie_files);
      assert!(!radarr_data.add_list_exclusion);

      assert_eq!(radarr_data.main_tabs.tabs.len(), 7);

      assert_str_eq!(radarr_data.main_tabs.tabs[0].title, "Library");
      assert_eq!(
        radarr_data.main_tabs.tabs[0].route,
        ActiveRadarrBlock::Movies.into()
      );
      assert!(radarr_data.main_tabs.tabs[0].help.is_empty());
      assert_eq!(
        radarr_data.main_tabs.tabs[0].contextual_help,
        Some(build_context_clue_string(&LIBRARY_CONTEXT_CLUES))
      );

      assert_str_eq!(radarr_data.main_tabs.tabs[1].title, "Collections");
      assert_eq!(
        radarr_data.main_tabs.tabs[1].route,
        ActiveRadarrBlock::Collections.into()
      );
      assert!(radarr_data.main_tabs.tabs[1].help.is_empty());
      assert_eq!(
        radarr_data.main_tabs.tabs[1].contextual_help,
        Some(build_context_clue_string(&COLLECTIONS_CONTEXT_CLUES))
      );

      assert_str_eq!(radarr_data.main_tabs.tabs[2].title, "Downloads");
      assert_eq!(
        radarr_data.main_tabs.tabs[2].route,
        ActiveRadarrBlock::Downloads.into()
      );
      assert!(radarr_data.main_tabs.tabs[2].help.is_empty());
      assert_eq!(
        radarr_data.main_tabs.tabs[2].contextual_help,
        Some(build_context_clue_string(&DOWNLOADS_CONTEXT_CLUES))
      );

      assert_str_eq!(radarr_data.main_tabs.tabs[3].title, "Blocklist");
      assert_eq!(
        radarr_data.main_tabs.tabs[3].route,
        ActiveRadarrBlock::Blocklist.into()
      );
      assert!(radarr_data.main_tabs.tabs[3].help.is_empty());
      assert_eq!(
        radarr_data.main_tabs.tabs[3].contextual_help,
        Some(build_context_clue_string(&BLOCKLIST_CONTEXT_CLUES))
      );

      assert_str_eq!(radarr_data.main_tabs.tabs[4].title, "Root Folders");
      assert_eq!(
        radarr_data.main_tabs.tabs[4].route,
        ActiveRadarrBlock::RootFolders.into()
      );
      assert!(radarr_data.main_tabs.tabs[4].help.is_empty());
      assert_eq!(
        radarr_data.main_tabs.tabs[4].contextual_help,
        Some(build_context_clue_string(&ROOT_FOLDERS_CONTEXT_CLUES))
      );

      assert_str_eq!(radarr_data.main_tabs.tabs[5].title, "Indexers");
      assert_eq!(
        radarr_data.main_tabs.tabs[5].route,
        ActiveRadarrBlock::Indexers.into()
      );
      assert!(radarr_data.main_tabs.tabs[5].help.is_empty());
      assert_eq!(
        radarr_data.main_tabs.tabs[5].contextual_help,
        Some(build_context_clue_string(&INDEXERS_CONTEXT_CLUES))
      );

      assert_str_eq!(radarr_data.main_tabs.tabs[6].title, "System");
      assert_eq!(
        radarr_data.main_tabs.tabs[6].route,
        ActiveRadarrBlock::System.into()
      );
      assert!(radarr_data.main_tabs.tabs[6].help.is_empty());
      assert_eq!(
        radarr_data.main_tabs.tabs[6].contextual_help,
        Some(build_context_clue_string(&SYSTEM_CONTEXT_CLUES))
      );

      assert_eq!(radarr_data.movie_info_tabs.tabs.len(), 6);

      assert_str_eq!(radarr_data.movie_info_tabs.tabs[0].title, "Details");
      assert_eq!(
        radarr_data.movie_info_tabs.tabs[0].route,
        ActiveRadarrBlock::MovieDetails.into()
      );
      assert_str_eq!(
        radarr_data.movie_info_tabs.tabs[0].help,
        build_context_clue_string(&MOVIE_DETAILS_CONTEXT_CLUES)
      );
      assert!(radarr_data.movie_info_tabs.tabs[0]
        .contextual_help
        .is_none());

      assert_str_eq!(radarr_data.movie_info_tabs.tabs[1].title, "History");
      assert_eq!(
        radarr_data.movie_info_tabs.tabs[1].route,
        ActiveRadarrBlock::MovieHistory.into()
      );
      assert_str_eq!(
        radarr_data.movie_info_tabs.tabs[1].help,
        build_context_clue_string(&MOVIE_DETAILS_CONTEXT_CLUES)
      );
      assert!(radarr_data.movie_info_tabs.tabs[1]
        .contextual_help
        .is_none());

      assert_str_eq!(radarr_data.movie_info_tabs.tabs[2].title, "File");
      assert_eq!(
        radarr_data.movie_info_tabs.tabs[2].route,
        ActiveRadarrBlock::FileInfo.into()
      );
      assert_str_eq!(
        radarr_data.movie_info_tabs.tabs[2].help,
        build_context_clue_string(&MOVIE_DETAILS_CONTEXT_CLUES)
      );
      assert!(radarr_data.movie_info_tabs.tabs[2]
        .contextual_help
        .is_none());

      assert_str_eq!(radarr_data.movie_info_tabs.tabs[3].title, "Cast");
      assert_eq!(
        radarr_data.movie_info_tabs.tabs[3].route,
        ActiveRadarrBlock::Cast.into()
      );
      assert_str_eq!(
        radarr_data.movie_info_tabs.tabs[3].help,
        build_context_clue_string(&MOVIE_DETAILS_CONTEXT_CLUES)
      );
      assert!(radarr_data.movie_info_tabs.tabs[3]
        .contextual_help
        .is_none());

      assert_str_eq!(radarr_data.movie_info_tabs.tabs[4].title, "Crew");
      assert_eq!(
        radarr_data.movie_info_tabs.tabs[4].route,
        ActiveRadarrBlock::Crew.into()
      );
      assert_str_eq!(
        radarr_data.movie_info_tabs.tabs[4].help,
        build_context_clue_string(&MOVIE_DETAILS_CONTEXT_CLUES)
      );
      assert!(radarr_data.movie_info_tabs.tabs[4]
        .contextual_help
        .is_none());

      assert_str_eq!(radarr_data.movie_info_tabs.tabs[5].title, "Manual Search");
      assert_eq!(
        radarr_data.movie_info_tabs.tabs[5].route,
        ActiveRadarrBlock::ManualSearch.into()
      );
      assert_str_eq!(
        radarr_data.movie_info_tabs.tabs[5].help,
        build_context_clue_string(&MANUAL_MOVIE_SEARCH_CONTEXT_CLUES)
      );
      assert_eq!(
        radarr_data.movie_info_tabs.tabs[5].contextual_help,
        Some(build_context_clue_string(
          &MANUAL_MOVIE_SEARCH_CONTEXTUAL_CONTEXT_CLUES
        ))
      );
    }
  }

  mod active_radarr_block_tests {
    use pretty_assertions::assert_eq;

    use crate::models::servarr_data::radarr::radarr_data::{
      ActiveRadarrBlock, ADD_MOVIE_BLOCKS, ADD_MOVIE_SELECTION_BLOCKS, BLOCKLIST_BLOCKS,
      COLLECTIONS_BLOCKS, COLLECTION_DETAILS_BLOCKS, DELETE_MOVIE_BLOCKS,
      DELETE_MOVIE_SELECTION_BLOCKS, DOWNLOADS_BLOCKS, EDIT_COLLECTION_BLOCKS,
      EDIT_COLLECTION_SELECTION_BLOCKS, EDIT_INDEXER_BLOCKS, EDIT_INDEXER_NZB_SELECTION_BLOCKS,
      EDIT_INDEXER_TORRENT_SELECTION_BLOCKS, EDIT_MOVIE_BLOCKS, EDIT_MOVIE_SELECTION_BLOCKS,
      INDEXERS_BLOCKS, INDEXER_SETTINGS_BLOCKS, INDEXER_SETTINGS_SELECTION_BLOCKS, LIBRARY_BLOCKS,
      MOVIE_DETAILS_BLOCKS, ROOT_FOLDERS_BLOCKS, SYSTEM_DETAILS_BLOCKS,
    };

    #[test]
    fn test_library_blocks_contents() {
      assert_eq!(LIBRARY_BLOCKS.len(), 7);
      assert!(LIBRARY_BLOCKS.contains(&ActiveRadarrBlock::Movies));
      assert!(LIBRARY_BLOCKS.contains(&ActiveRadarrBlock::MoviesSortPrompt));
      assert!(LIBRARY_BLOCKS.contains(&ActiveRadarrBlock::SearchMovie));
      assert!(LIBRARY_BLOCKS.contains(&ActiveRadarrBlock::SearchMovieError));
      assert!(LIBRARY_BLOCKS.contains(&ActiveRadarrBlock::FilterMovies));
      assert!(LIBRARY_BLOCKS.contains(&ActiveRadarrBlock::FilterMoviesError));
      assert!(LIBRARY_BLOCKS.contains(&ActiveRadarrBlock::UpdateAllMoviesPrompt));
    }

    #[test]
    fn test_collections_blocks_contents() {
      assert_eq!(COLLECTIONS_BLOCKS.len(), 7);
      assert!(COLLECTIONS_BLOCKS.contains(&ActiveRadarrBlock::Collections));
      assert!(COLLECTIONS_BLOCKS.contains(&ActiveRadarrBlock::CollectionsSortPrompt));
      assert!(COLLECTIONS_BLOCKS.contains(&ActiveRadarrBlock::SearchCollection));
      assert!(COLLECTIONS_BLOCKS.contains(&ActiveRadarrBlock::SearchCollectionError));
      assert!(COLLECTIONS_BLOCKS.contains(&ActiveRadarrBlock::FilterCollections));
      assert!(COLLECTIONS_BLOCKS.contains(&ActiveRadarrBlock::FilterCollectionsError));
      assert!(COLLECTIONS_BLOCKS.contains(&ActiveRadarrBlock::UpdateAllCollectionsPrompt));
    }

    #[test]
    fn test_indexers_blocks_contents() {
      assert_eq!(INDEXERS_BLOCKS.len(), 4);
      assert!(INDEXERS_BLOCKS.contains(&ActiveRadarrBlock::AddIndexer));
      assert!(INDEXERS_BLOCKS.contains(&ActiveRadarrBlock::DeleteIndexerPrompt));
      assert!(INDEXERS_BLOCKS.contains(&ActiveRadarrBlock::Indexers));
      assert!(INDEXERS_BLOCKS.contains(&ActiveRadarrBlock::TestIndexer));
    }

    #[test]
    fn test_root_folders_blocks_contents() {
      assert_eq!(ROOT_FOLDERS_BLOCKS.len(), 3);
      assert!(ROOT_FOLDERS_BLOCKS.contains(&ActiveRadarrBlock::RootFolders));
      assert!(ROOT_FOLDERS_BLOCKS.contains(&ActiveRadarrBlock::AddRootFolderPrompt));
      assert!(ROOT_FOLDERS_BLOCKS.contains(&ActiveRadarrBlock::DeleteRootFolderPrompt));
    }

    #[test]
    fn test_blocklist_blocks_contents() {
      assert_eq!(BLOCKLIST_BLOCKS.len(), 5);
      assert!(BLOCKLIST_BLOCKS.contains(&ActiveRadarrBlock::Blocklist));
      assert!(BLOCKLIST_BLOCKS.contains(&ActiveRadarrBlock::BlocklistItemDetails));
      assert!(BLOCKLIST_BLOCKS.contains(&ActiveRadarrBlock::DeleteBlocklistItemPrompt));
      assert!(BLOCKLIST_BLOCKS.contains(&ActiveRadarrBlock::BlocklistClearAllItemsPrompt));
      assert!(BLOCKLIST_BLOCKS.contains(&ActiveRadarrBlock::BlocklistSortPrompt));
    }

    #[test]
    fn test_add_movie_blocks_contents() {
      assert_eq!(ADD_MOVIE_BLOCKS.len(), 10);
      assert!(ADD_MOVIE_BLOCKS.contains(&ActiveRadarrBlock::AddMovieSearchInput));
      assert!(ADD_MOVIE_BLOCKS.contains(&ActiveRadarrBlock::AddMovieSearchResults));
      assert!(ADD_MOVIE_BLOCKS.contains(&ActiveRadarrBlock::AddMovieEmptySearchResults));
      assert!(ADD_MOVIE_BLOCKS.contains(&ActiveRadarrBlock::AddMoviePrompt));
      assert!(ADD_MOVIE_BLOCKS.contains(&ActiveRadarrBlock::AddMovieSelectMinimumAvailability));
      assert!(ADD_MOVIE_BLOCKS.contains(&ActiveRadarrBlock::AddMovieSelectMonitor));
      assert!(ADD_MOVIE_BLOCKS.contains(&ActiveRadarrBlock::AddMovieSelectQualityProfile));
      assert!(ADD_MOVIE_BLOCKS.contains(&ActiveRadarrBlock::AddMovieSelectRootFolder));
      assert!(ADD_MOVIE_BLOCKS.contains(&ActiveRadarrBlock::AddMovieAlreadyInLibrary));
      assert!(ADD_MOVIE_BLOCKS.contains(&ActiveRadarrBlock::AddMovieTagsInput));
    }

    #[test]
    fn test_edit_collection_blocks_contents() {
      assert_eq!(EDIT_COLLECTION_BLOCKS.len(), 7);
      assert!(EDIT_COLLECTION_BLOCKS.contains(&ActiveRadarrBlock::EditCollectionPrompt));
      assert!(EDIT_COLLECTION_BLOCKS.contains(&ActiveRadarrBlock::EditCollectionConfirmPrompt));
      assert!(
        EDIT_COLLECTION_BLOCKS.contains(&ActiveRadarrBlock::EditCollectionRootFolderPathInput)
      );
      assert!(EDIT_COLLECTION_BLOCKS
        .contains(&ActiveRadarrBlock::EditCollectionSelectMinimumAvailability));
      assert!(
        EDIT_COLLECTION_BLOCKS.contains(&ActiveRadarrBlock::EditCollectionSelectQualityProfile)
      );
      assert!(EDIT_COLLECTION_BLOCKS.contains(&ActiveRadarrBlock::EditCollectionToggleSearchOnAdd));
      assert!(EDIT_COLLECTION_BLOCKS.contains(&ActiveRadarrBlock::EditCollectionToggleMonitored));
    }

    #[test]
    fn test_edit_movie_blocks_contents() {
      assert_eq!(EDIT_MOVIE_BLOCKS.len(), 7);
      assert!(EDIT_MOVIE_BLOCKS.contains(&ActiveRadarrBlock::EditMoviePrompt));
      assert!(EDIT_MOVIE_BLOCKS.contains(&ActiveRadarrBlock::EditMovieConfirmPrompt));
      assert!(EDIT_MOVIE_BLOCKS.contains(&ActiveRadarrBlock::EditMoviePathInput));
      assert!(EDIT_MOVIE_BLOCKS.contains(&ActiveRadarrBlock::EditMovieSelectMinimumAvailability));
      assert!(EDIT_MOVIE_BLOCKS.contains(&ActiveRadarrBlock::EditMovieSelectQualityProfile));
      assert!(EDIT_MOVIE_BLOCKS.contains(&ActiveRadarrBlock::EditMovieTagsInput));
      assert!(EDIT_MOVIE_BLOCKS.contains(&ActiveRadarrBlock::EditMovieToggleMonitored));
    }

    #[test]
    fn test_downloads_blocks_contents() {
      assert_eq!(DOWNLOADS_BLOCKS.len(), 3);
      assert!(DOWNLOADS_BLOCKS.contains(&ActiveRadarrBlock::Downloads));
      assert!(DOWNLOADS_BLOCKS.contains(&ActiveRadarrBlock::DeleteDownloadPrompt));
      assert!(DOWNLOADS_BLOCKS.contains(&ActiveRadarrBlock::UpdateDownloadsPrompt));
    }

    #[test]
    fn test_movie_details_blocks_contents() {
      assert_eq!(MOVIE_DETAILS_BLOCKS.len(), 10);
      assert!(MOVIE_DETAILS_BLOCKS.contains(&ActiveRadarrBlock::MovieDetails));
      assert!(MOVIE_DETAILS_BLOCKS.contains(&ActiveRadarrBlock::MovieHistory));
      assert!(MOVIE_DETAILS_BLOCKS.contains(&ActiveRadarrBlock::FileInfo));
      assert!(MOVIE_DETAILS_BLOCKS.contains(&ActiveRadarrBlock::Cast));
      assert!(MOVIE_DETAILS_BLOCKS.contains(&ActiveRadarrBlock::Crew));
      assert!(MOVIE_DETAILS_BLOCKS.contains(&ActiveRadarrBlock::AutomaticallySearchMoviePrompt));
      assert!(MOVIE_DETAILS_BLOCKS.contains(&ActiveRadarrBlock::UpdateAndScanPrompt));
      assert!(MOVIE_DETAILS_BLOCKS.contains(&ActiveRadarrBlock::ManualSearch));
      assert!(MOVIE_DETAILS_BLOCKS.contains(&ActiveRadarrBlock::ManualSearchSortPrompt));
      assert!(MOVIE_DETAILS_BLOCKS.contains(&ActiveRadarrBlock::ManualSearchConfirmPrompt));
    }

    #[test]
    fn test_collection_details_blocks_contents() {
      assert_eq!(COLLECTION_DETAILS_BLOCKS.len(), 2);
      assert!(COLLECTION_DETAILS_BLOCKS.contains(&ActiveRadarrBlock::CollectionDetails));
      assert!(COLLECTION_DETAILS_BLOCKS.contains(&ActiveRadarrBlock::ViewMovieOverview));
    }

    #[test]
    fn test_delete_movie_blocks_contents() {
      assert_eq!(DELETE_MOVIE_BLOCKS.len(), 4);
      assert!(DELETE_MOVIE_BLOCKS.contains(&ActiveRadarrBlock::DeleteMoviePrompt));
      assert!(DELETE_MOVIE_BLOCKS.contains(&ActiveRadarrBlock::DeleteMovieConfirmPrompt));
      assert!(DELETE_MOVIE_BLOCKS.contains(&ActiveRadarrBlock::DeleteMovieToggleDeleteFile));
      assert!(DELETE_MOVIE_BLOCKS.contains(&ActiveRadarrBlock::DeleteMovieToggleAddListExclusion));
    }

    #[test]
    fn test_edit_indexer_blocks_contents() {
      assert_eq!(EDIT_INDEXER_BLOCKS.len(), 10);
      assert!(EDIT_INDEXER_BLOCKS.contains(&ActiveRadarrBlock::EditIndexerPrompt));
      assert!(EDIT_INDEXER_BLOCKS.contains(&ActiveRadarrBlock::EditIndexerConfirmPrompt));
      assert!(EDIT_INDEXER_BLOCKS.contains(&ActiveRadarrBlock::EditIndexerApiKeyInput));
      assert!(EDIT_INDEXER_BLOCKS.contains(&ActiveRadarrBlock::EditIndexerNameInput));
      assert!(EDIT_INDEXER_BLOCKS.contains(&ActiveRadarrBlock::EditIndexerSeedRatioInput));
      assert!(EDIT_INDEXER_BLOCKS.contains(&ActiveRadarrBlock::EditIndexerToggleEnableRss));
      assert!(
        EDIT_INDEXER_BLOCKS.contains(&ActiveRadarrBlock::EditIndexerToggleEnableAutomaticSearch)
      );
      assert!(
        EDIT_INDEXER_BLOCKS.contains(&ActiveRadarrBlock::EditIndexerToggleEnableInteractiveSearch)
      );
      assert!(EDIT_INDEXER_BLOCKS.contains(&ActiveRadarrBlock::EditIndexerUrlInput));
      assert!(EDIT_INDEXER_BLOCKS.contains(&ActiveRadarrBlock::EditIndexerTagsInput));
    }

    #[test]
    fn test_indexer_settings_blocks_contents() {
      assert_eq!(INDEXER_SETTINGS_BLOCKS.len(), 10);
      assert!(INDEXER_SETTINGS_BLOCKS.contains(&ActiveRadarrBlock::AllIndexerSettingsPrompt));
      assert!(
        INDEXER_SETTINGS_BLOCKS.contains(&ActiveRadarrBlock::IndexerSettingsAvailabilityDelayInput)
      );
      assert!(INDEXER_SETTINGS_BLOCKS.contains(&ActiveRadarrBlock::IndexerSettingsConfirmPrompt));
      assert!(INDEXER_SETTINGS_BLOCKS.contains(&ActiveRadarrBlock::IndexerSettingsMaximumSizeInput));
      assert!(INDEXER_SETTINGS_BLOCKS.contains(&ActiveRadarrBlock::IndexerSettingsMinimumAgeInput));
      assert!(INDEXER_SETTINGS_BLOCKS.contains(&ActiveRadarrBlock::IndexerSettingsRetentionInput));
      assert!(
        INDEXER_SETTINGS_BLOCKS.contains(&ActiveRadarrBlock::IndexerSettingsRssSyncIntervalInput)
      );
      assert!(INDEXER_SETTINGS_BLOCKS
        .contains(&ActiveRadarrBlock::IndexerSettingsToggleAllowHardcodedSubs));
      assert!(INDEXER_SETTINGS_BLOCKS
        .contains(&ActiveRadarrBlock::IndexerSettingsTogglePreferIndexerFlags));
      assert!(INDEXER_SETTINGS_BLOCKS
        .contains(&ActiveRadarrBlock::IndexerSettingsWhitelistedSubtitleTagsInput));
    }

    #[test]
    fn test_system_details_blocks_contents() {
      assert_eq!(SYSTEM_DETAILS_BLOCKS.len(), 5);
      assert!(SYSTEM_DETAILS_BLOCKS.contains(&ActiveRadarrBlock::SystemLogs));
      assert!(SYSTEM_DETAILS_BLOCKS.contains(&ActiveRadarrBlock::SystemQueuedEvents));
      assert!(SYSTEM_DETAILS_BLOCKS.contains(&ActiveRadarrBlock::SystemTasks));
      assert!(SYSTEM_DETAILS_BLOCKS.contains(&ActiveRadarrBlock::SystemTaskStartConfirmPrompt));
      assert!(SYSTEM_DETAILS_BLOCKS.contains(&ActiveRadarrBlock::SystemUpdates));
    }

    #[test]
    fn test_add_movie_selection_blocks_ordering() {
      let mut add_movie_block_iter = ADD_MOVIE_SELECTION_BLOCKS.iter();

      assert_eq!(
        add_movie_block_iter.next().unwrap(),
        &ActiveRadarrBlock::AddMovieSelectRootFolder
      );
      assert_eq!(
        add_movie_block_iter.next().unwrap(),
        &ActiveRadarrBlock::AddMovieSelectMonitor
      );
      assert_eq!(
        add_movie_block_iter.next().unwrap(),
        &ActiveRadarrBlock::AddMovieSelectMinimumAvailability
      );
      assert_eq!(
        add_movie_block_iter.next().unwrap(),
        &ActiveRadarrBlock::AddMovieSelectQualityProfile
      );
      assert_eq!(
        add_movie_block_iter.next().unwrap(),
        &ActiveRadarrBlock::AddMovieTagsInput
      );
      assert_eq!(
        add_movie_block_iter.next().unwrap(),
        &ActiveRadarrBlock::AddMovieConfirmPrompt
      );
      assert_eq!(add_movie_block_iter.next(), None);
    }

    #[test]
    fn test_edit_movie_selection_blocks_ordering() {
      let mut edit_movie_block_iter = EDIT_MOVIE_SELECTION_BLOCKS.iter();

      assert_eq!(
        edit_movie_block_iter.next().unwrap(),
        &ActiveRadarrBlock::EditMovieToggleMonitored
      );
      assert_eq!(
        edit_movie_block_iter.next().unwrap(),
        &ActiveRadarrBlock::EditMovieSelectMinimumAvailability
      );
      assert_eq!(
        edit_movie_block_iter.next().unwrap(),
        &ActiveRadarrBlock::EditMovieSelectQualityProfile
      );
      assert_eq!(
        edit_movie_block_iter.next().unwrap(),
        &ActiveRadarrBlock::EditMoviePathInput
      );
      assert_eq!(
        edit_movie_block_iter.next().unwrap(),
        &ActiveRadarrBlock::EditMovieTagsInput
      );
      assert_eq!(
        edit_movie_block_iter.next().unwrap(),
        &ActiveRadarrBlock::EditMovieConfirmPrompt
      );
      assert_eq!(edit_movie_block_iter.next(), None);
    }

    #[test]
    fn test_edit_collection_selection_blocks_ordering() {
      let mut edit_collection_block_iter = EDIT_COLLECTION_SELECTION_BLOCKS.iter();

      assert_eq!(
        edit_collection_block_iter.next().unwrap(),
        &ActiveRadarrBlock::EditCollectionToggleMonitored
      );
      assert_eq!(
        edit_collection_block_iter.next().unwrap(),
        &ActiveRadarrBlock::EditCollectionSelectMinimumAvailability
      );
      assert_eq!(
        edit_collection_block_iter.next().unwrap(),
        &ActiveRadarrBlock::EditCollectionSelectQualityProfile
      );
      assert_eq!(
        edit_collection_block_iter.next().unwrap(),
        &ActiveRadarrBlock::EditCollectionRootFolderPathInput
      );
      assert_eq!(
        edit_collection_block_iter.next().unwrap(),
        &ActiveRadarrBlock::EditCollectionToggleSearchOnAdd
      );
      assert_eq!(
        edit_collection_block_iter.next().unwrap(),
        &ActiveRadarrBlock::EditCollectionConfirmPrompt
      );
      assert_eq!(edit_collection_block_iter.next(), None);
    }

    #[test]
    fn test_delete_movie_selection_blocks_ordering() {
      let mut delete_movie_block_iter = DELETE_MOVIE_SELECTION_BLOCKS.iter();

      assert_eq!(
        delete_movie_block_iter.next().unwrap(),
        &ActiveRadarrBlock::DeleteMovieToggleDeleteFile
      );
      assert_eq!(
        delete_movie_block_iter.next().unwrap(),
        &ActiveRadarrBlock::DeleteMovieToggleAddListExclusion
      );
      assert_eq!(
        delete_movie_block_iter.next().unwrap(),
        &ActiveRadarrBlock::DeleteMovieConfirmPrompt
      );
      assert_eq!(delete_movie_block_iter.next(), None);
    }

    #[test]
    fn test_edit_indexer_torrent_selection_blocks_ordering() {
      let mut edit_indexer_torrent_selection_block_iter =
        EDIT_INDEXER_TORRENT_SELECTION_BLOCKS.iter();

      assert_eq!(
        edit_indexer_torrent_selection_block_iter.next().unwrap(),
        &ActiveRadarrBlock::EditIndexerNameInput
      );
      assert_eq!(
        edit_indexer_torrent_selection_block_iter.next().unwrap(),
        &ActiveRadarrBlock::EditIndexerToggleEnableRss
      );
      assert_eq!(
        edit_indexer_torrent_selection_block_iter.next().unwrap(),
        &ActiveRadarrBlock::EditIndexerToggleEnableAutomaticSearch
      );
      assert_eq!(
        edit_indexer_torrent_selection_block_iter.next().unwrap(),
        &ActiveRadarrBlock::EditIndexerToggleEnableInteractiveSearch
      );
      assert_eq!(
        edit_indexer_torrent_selection_block_iter.next().unwrap(),
        &ActiveRadarrBlock::EditIndexerConfirmPrompt
      );
      assert_eq!(
        edit_indexer_torrent_selection_block_iter.next().unwrap(),
        &ActiveRadarrBlock::EditIndexerUrlInput
      );
      assert_eq!(
        edit_indexer_torrent_selection_block_iter.next().unwrap(),
        &ActiveRadarrBlock::EditIndexerApiKeyInput
      );
      assert_eq!(
        edit_indexer_torrent_selection_block_iter.next().unwrap(),
        &ActiveRadarrBlock::EditIndexerSeedRatioInput
      );
      assert_eq!(
        edit_indexer_torrent_selection_block_iter.next().unwrap(),
        &ActiveRadarrBlock::EditIndexerTagsInput
      );
      assert_eq!(
        edit_indexer_torrent_selection_block_iter.next().unwrap(),
        &ActiveRadarrBlock::EditIndexerConfirmPrompt
      );
      assert_eq!(edit_indexer_torrent_selection_block_iter.next(), None);
    }

    #[test]
    fn test_edit_indexer_nzb_selection_blocks_ordering() {
      let mut edit_indexer_nzb_selection_block_iter = EDIT_INDEXER_NZB_SELECTION_BLOCKS.iter();

      assert_eq!(
        edit_indexer_nzb_selection_block_iter.next().unwrap(),
        &ActiveRadarrBlock::EditIndexerNameInput
      );
      assert_eq!(
        edit_indexer_nzb_selection_block_iter.next().unwrap(),
        &ActiveRadarrBlock::EditIndexerToggleEnableRss
      );
      assert_eq!(
        edit_indexer_nzb_selection_block_iter.next().unwrap(),
        &ActiveRadarrBlock::EditIndexerToggleEnableAutomaticSearch
      );
      assert_eq!(
        edit_indexer_nzb_selection_block_iter.next().unwrap(),
        &ActiveRadarrBlock::EditIndexerToggleEnableInteractiveSearch
      );
      assert_eq!(
        edit_indexer_nzb_selection_block_iter.next().unwrap(),
        &ActiveRadarrBlock::EditIndexerConfirmPrompt
      );
      assert_eq!(
        edit_indexer_nzb_selection_block_iter.next().unwrap(),
        &ActiveRadarrBlock::EditIndexerUrlInput
      );
      assert_eq!(
        edit_indexer_nzb_selection_block_iter.next().unwrap(),
        &ActiveRadarrBlock::EditIndexerApiKeyInput
      );
      assert_eq!(
        edit_indexer_nzb_selection_block_iter.next().unwrap(),
        &ActiveRadarrBlock::EditIndexerTagsInput
      );
      assert_eq!(
        edit_indexer_nzb_selection_block_iter.next().unwrap(),
        &ActiveRadarrBlock::EditIndexerConfirmPrompt
      );
      assert_eq!(
        edit_indexer_nzb_selection_block_iter.next().unwrap(),
        &ActiveRadarrBlock::EditIndexerConfirmPrompt
      );
      assert_eq!(edit_indexer_nzb_selection_block_iter.next(), None);
    }

    #[test]
    fn test_indexer_settings_selection_blocks_ordering() {
      let mut indexer_settings_block_iter = INDEXER_SETTINGS_SELECTION_BLOCKS.iter();

      assert_eq!(
        indexer_settings_block_iter.next().unwrap(),
        &ActiveRadarrBlock::IndexerSettingsMinimumAgeInput
      );
      assert_eq!(
        indexer_settings_block_iter.next().unwrap(),
        &ActiveRadarrBlock::IndexerSettingsRetentionInput
      );
      assert_eq!(
        indexer_settings_block_iter.next().unwrap(),
        &ActiveRadarrBlock::IndexerSettingsMaximumSizeInput
      );
      assert_eq!(
        indexer_settings_block_iter.next().unwrap(),
        &ActiveRadarrBlock::IndexerSettingsTogglePreferIndexerFlags
      );
      assert_eq!(
        indexer_settings_block_iter.next().unwrap(),
        &ActiveRadarrBlock::IndexerSettingsConfirmPrompt
      );
      assert_eq!(
        indexer_settings_block_iter.next().unwrap(),
        &ActiveRadarrBlock::IndexerSettingsAvailabilityDelayInput
      );
      assert_eq!(
        indexer_settings_block_iter.next().unwrap(),
        &ActiveRadarrBlock::IndexerSettingsRssSyncIntervalInput
      );
      assert_eq!(
        indexer_settings_block_iter.next().unwrap(),
        &ActiveRadarrBlock::IndexerSettingsWhitelistedSubtitleTagsInput
      );
      assert_eq!(
        indexer_settings_block_iter.next().unwrap(),
        &ActiveRadarrBlock::IndexerSettingsToggleAllowHardcodedSubs
      );
      assert_eq!(
        indexer_settings_block_iter.next().unwrap(),
        &ActiveRadarrBlock::IndexerSettingsConfirmPrompt
      );
      assert_eq!(indexer_settings_block_iter.next(), None);
    }
  }
}
