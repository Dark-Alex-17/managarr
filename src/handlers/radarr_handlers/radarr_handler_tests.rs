#[cfg(test)]
mod tests {
  use pretty_assertions::assert_eq;
  use rstest::rstest;
  use strum::IntoEnumIterator;

  use crate::app::key_binding::DEFAULT_KEYBINDINGS;
  use crate::app::App;
  use crate::handlers::radarr_handlers::{handle_change_tab_left_right_keys, RadarrHandler};
  use crate::handlers::KeyEventHandler;
  use crate::models::servarr_data::radarr::radarr_data::ActiveRadarrBlock;
  use crate::test_handler_delegation;

  #[rstest]
  #[case(0, ActiveRadarrBlock::System, ActiveRadarrBlock::Collections)]
  #[case(1, ActiveRadarrBlock::Movies, ActiveRadarrBlock::Downloads)]
  #[case(2, ActiveRadarrBlock::Collections, ActiveRadarrBlock::Blocklist)]
  #[case(3, ActiveRadarrBlock::Downloads, ActiveRadarrBlock::RootFolders)]
  #[case(4, ActiveRadarrBlock::Blocklist, ActiveRadarrBlock::Indexers)]
  #[case(5, ActiveRadarrBlock::RootFolders, ActiveRadarrBlock::System)]
  #[case(6, ActiveRadarrBlock::Indexers, ActiveRadarrBlock::Movies)]
  fn test_radarr_handler_change_tab_left_right_keys(
    #[case] index: usize,
    #[case] left_block: ActiveRadarrBlock,
    #[case] right_block: ActiveRadarrBlock,
  ) {
    let mut app = App::default();
    app.data.radarr_data.main_tabs.set_index(index);

    handle_change_tab_left_right_keys(&mut app, DEFAULT_KEYBINDINGS.left.key);

    assert_eq!(
      app.data.radarr_data.main_tabs.get_active_route(),
      left_block.into()
    );
    assert_eq!(app.get_current_route(), left_block.into());

    app.data.radarr_data.main_tabs.set_index(index);

    handle_change_tab_left_right_keys(&mut app, DEFAULT_KEYBINDINGS.right.key);

    assert_eq!(
      app.data.radarr_data.main_tabs.get_active_route(),
      right_block.into()
    );
    assert_eq!(app.get_current_route(), right_block.into());
  }

  #[rstest]
  fn test_delegates_system_blocks_to_system_handler(
    #[values(
      ActiveRadarrBlock::System,
      ActiveRadarrBlock::SystemLogs,
      ActiveRadarrBlock::SystemQueuedEvents,
      ActiveRadarrBlock::SystemTasks,
      ActiveRadarrBlock::SystemTaskStartConfirmPrompt,
      ActiveRadarrBlock::SystemUpdates
    )]
    active_radarr_block: ActiveRadarrBlock,
  ) {
    test_handler_delegation!(
      RadarrHandler,
      ActiveRadarrBlock::System,
      active_radarr_block
    );
  }

  #[rstest]
  fn test_delegates_library_blocks_to_library_handler(
    #[values(
      ActiveRadarrBlock::Movies,
      ActiveRadarrBlock::MoviesSortPrompt,
      ActiveRadarrBlock::SearchMovie,
      ActiveRadarrBlock::SearchMovieError,
      ActiveRadarrBlock::FilterMovies,
      ActiveRadarrBlock::FilterMoviesError,
      ActiveRadarrBlock::UpdateAllMoviesPrompt,
      ActiveRadarrBlock::AddMovieSearchInput,
      ActiveRadarrBlock::AddMovieSearchResults,
      ActiveRadarrBlock::AddMoviePrompt,
      ActiveRadarrBlock::AddMovieSelectMonitor,
      ActiveRadarrBlock::AddMovieSelectMinimumAvailability,
      ActiveRadarrBlock::AddMovieSelectQualityProfile,
      ActiveRadarrBlock::AddMovieSelectRootFolder,
      ActiveRadarrBlock::AddMovieAlreadyInLibrary,
      ActiveRadarrBlock::AddMovieTagsInput,
      ActiveRadarrBlock::MovieDetails,
      ActiveRadarrBlock::MovieHistory,
      ActiveRadarrBlock::FileInfo,
      ActiveRadarrBlock::Cast,
      ActiveRadarrBlock::Crew,
      ActiveRadarrBlock::AutomaticallySearchMoviePrompt,
      ActiveRadarrBlock::UpdateAndScanPrompt,
      ActiveRadarrBlock::ManualSearch,
      ActiveRadarrBlock::ManualSearchConfirmPrompt,
      ActiveRadarrBlock::EditMoviePrompt,
      ActiveRadarrBlock::EditMoviePathInput,
      ActiveRadarrBlock::EditMovieSelectMinimumAvailability,
      ActiveRadarrBlock::EditMovieSelectQualityProfile,
      ActiveRadarrBlock::EditMovieTagsInput,
      ActiveRadarrBlock::DeleteMoviePrompt
    )]
    active_radarr_block: ActiveRadarrBlock,
  ) {
    test_handler_delegation!(
      RadarrHandler,
      ActiveRadarrBlock::Movies,
      active_radarr_block
    );
  }

  #[rstest]
  fn test_delegates_collections_blocks_to_collections_handler(
    #[values(
      ActiveRadarrBlock::Collections,
      ActiveRadarrBlock::SearchCollection,
      ActiveRadarrBlock::CollectionsSortPrompt,
      ActiveRadarrBlock::SearchCollectionError,
      ActiveRadarrBlock::FilterCollections,
      ActiveRadarrBlock::FilterCollectionsError,
      ActiveRadarrBlock::UpdateAllCollectionsPrompt,
      ActiveRadarrBlock::CollectionDetails,
      ActiveRadarrBlock::ViewMovieOverview,
      ActiveRadarrBlock::EditCollectionPrompt,
      ActiveRadarrBlock::EditCollectionRootFolderPathInput,
      ActiveRadarrBlock::EditCollectionSelectMinimumAvailability,
      ActiveRadarrBlock::EditCollectionSelectQualityProfile
    )]
    active_radarr_block: ActiveRadarrBlock,
  ) {
    test_handler_delegation!(
      RadarrHandler,
      ActiveRadarrBlock::Collections,
      active_radarr_block
    );
  }

  #[rstest]
  fn test_delegates_indexers_blocks_to_indexers_handler(
    #[values(
      ActiveRadarrBlock::DeleteIndexerPrompt,
      ActiveRadarrBlock::Indexers,
      ActiveRadarrBlock::AllIndexerSettingsPrompt,
      ActiveRadarrBlock::IndexerSettingsAvailabilityDelayInput,
      ActiveRadarrBlock::IndexerSettingsConfirmPrompt,
      ActiveRadarrBlock::IndexerSettingsMaximumSizeInput,
      ActiveRadarrBlock::IndexerSettingsMinimumAgeInput,
      ActiveRadarrBlock::IndexerSettingsRetentionInput,
      ActiveRadarrBlock::IndexerSettingsRssSyncIntervalInput,
      ActiveRadarrBlock::IndexerSettingsToggleAllowHardcodedSubs,
      ActiveRadarrBlock::IndexerSettingsTogglePreferIndexerFlags,
      ActiveRadarrBlock::IndexerSettingsWhitelistedSubtitleTagsInput
    )]
    active_radarr_block: ActiveRadarrBlock,
  ) {
    test_handler_delegation!(
      RadarrHandler,
      ActiveRadarrBlock::Indexers,
      active_radarr_block
    );
  }

  #[rstest]
  fn test_delegates_downloads_blocks_to_downloads_handler(
    #[values(
      ActiveRadarrBlock::Downloads,
      ActiveRadarrBlock::DeleteDownloadPrompt,
      ActiveRadarrBlock::UpdateDownloadsPrompt
    )]
    active_radarr_block: ActiveRadarrBlock,
  ) {
    test_handler_delegation!(
      RadarrHandler,
      ActiveRadarrBlock::Downloads,
      active_radarr_block
    );
  }

  #[rstest]
  fn test_delegates_root_folders_blocks_to_root_folders_handler(
    #[values(
      ActiveRadarrBlock::RootFolders,
      ActiveRadarrBlock::AddRootFolderPrompt,
      ActiveRadarrBlock::DeleteRootFolderPrompt
    )]
    active_radarr_block: ActiveRadarrBlock,
  ) {
    test_handler_delegation!(
      RadarrHandler,
      ActiveRadarrBlock::RootFolders,
      active_radarr_block
    );
  }

  #[rstest]
  fn test_delegates_blocklist_blocks_to_blocklist_handler(
    #[values(
      ActiveRadarrBlock::Blocklist,
      ActiveRadarrBlock::BlocklistItemDetails,
      ActiveRadarrBlock::DeleteBlocklistItemPrompt,
      ActiveRadarrBlock::BlocklistClearAllItemsPrompt,
      ActiveRadarrBlock::BlocklistSortPrompt
    )]
    active_radarr_block: ActiveRadarrBlock,
  ) {
    test_handler_delegation!(
      RadarrHandler,
      ActiveRadarrBlock::Blocklist,
      active_radarr_block
    );
  }

  #[test]
  fn test_radarr_handler_accepts() {
    ActiveRadarrBlock::iter().for_each(|active_radarr_block| {
      assert!(RadarrHandler::accepts(active_radarr_block));
    })
  }

  #[test]
  fn test_radarr_handler_is_ready() {
    let mut app = App::default();
    app.is_loading = true;

    let handler = RadarrHandler::with(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveRadarrBlock::System,
      None,
    );

    assert!(handler.is_ready());
  }
}
