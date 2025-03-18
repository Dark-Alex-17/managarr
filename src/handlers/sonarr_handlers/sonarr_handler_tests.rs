#[cfg(test)]
mod tests {
  use crate::app::key_binding::DEFAULT_KEYBINDINGS;
  use crate::app::App;
  use crate::handlers::sonarr_handlers::handle_change_tab_left_right_keys;
  use crate::handlers::sonarr_handlers::SonarrHandler;
  use crate::handlers::KeyEventHandler;
  use crate::models::servarr_data::sonarr::sonarr_data::ActiveSonarrBlock;
  use crate::test_handler_delegation;
  use pretty_assertions::assert_eq;
  use rstest::rstest;
  use strum::IntoEnumIterator;

  #[rstest]
  #[case(0, ActiveSonarrBlock::System, ActiveSonarrBlock::Downloads)]
  #[case(1, ActiveSonarrBlock::Series, ActiveSonarrBlock::Blocklist)]
  #[case(2, ActiveSonarrBlock::Downloads, ActiveSonarrBlock::History)]
  #[case(3, ActiveSonarrBlock::Blocklist, ActiveSonarrBlock::RootFolders)]
  #[case(4, ActiveSonarrBlock::History, ActiveSonarrBlock::Indexers)]
  #[case(5, ActiveSonarrBlock::RootFolders, ActiveSonarrBlock::System)]
  #[case(6, ActiveSonarrBlock::Indexers, ActiveSonarrBlock::Series)]
  fn test_sonarr_handler_change_tab_left_right_keys(
    #[case] index: usize,
    #[case] left_block: ActiveSonarrBlock,
    #[case] right_block: ActiveSonarrBlock,
  ) {
    let mut app = App::test_default();
    app.data.sonarr_data.main_tabs.set_index(index);

    handle_change_tab_left_right_keys(&mut app, DEFAULT_KEYBINDINGS.left.key);

    assert_eq!(
      app.data.sonarr_data.main_tabs.get_active_route(),
      left_block.into()
    );
    assert_eq!(app.get_current_route(), left_block.into());

    app.data.sonarr_data.main_tabs.set_index(index);

    handle_change_tab_left_right_keys(&mut app, DEFAULT_KEYBINDINGS.right.key);

    assert_eq!(
      app.data.sonarr_data.main_tabs.get_active_route(),
      right_block.into()
    );
    assert_eq!(app.get_current_route(), right_block.into());
  }

  #[rstest]
  #[case(0, ActiveSonarrBlock::System, ActiveSonarrBlock::Downloads)]
  #[case(1, ActiveSonarrBlock::Series, ActiveSonarrBlock::Blocklist)]
  #[case(2, ActiveSonarrBlock::Downloads, ActiveSonarrBlock::History)]
  #[case(3, ActiveSonarrBlock::Blocklist, ActiveSonarrBlock::RootFolders)]
  #[case(4, ActiveSonarrBlock::History, ActiveSonarrBlock::Indexers)]
  #[case(5, ActiveSonarrBlock::RootFolders, ActiveSonarrBlock::System)]
  #[case(6, ActiveSonarrBlock::Indexers, ActiveSonarrBlock::Series)]
  fn test_sonarr_handler_change_tab_left_right_keys_alt_navigation(
    #[case] index: usize,
    #[case] left_block: ActiveSonarrBlock,
    #[case] right_block: ActiveSonarrBlock,
  ) {
    let mut app = App::test_default();
    app.data.sonarr_data.main_tabs.set_index(index);

    handle_change_tab_left_right_keys(&mut app, DEFAULT_KEYBINDINGS.left.alt.unwrap());

    assert_eq!(
      app.data.sonarr_data.main_tabs.get_active_route(),
      left_block.into()
    );
    assert_eq!(app.get_current_route(), left_block.into());

    app.data.sonarr_data.main_tabs.set_index(index);

    handle_change_tab_left_right_keys(&mut app, DEFAULT_KEYBINDINGS.right.alt.unwrap());

    assert_eq!(
      app.data.sonarr_data.main_tabs.get_active_route(),
      right_block.into()
    );
    assert_eq!(app.get_current_route(), right_block.into());
  }

  #[rstest]
  #[case(0, ActiveSonarrBlock::Series)]
  #[case(1, ActiveSonarrBlock::Downloads)]
  #[case(2, ActiveSonarrBlock::Blocklist)]
  #[case(3, ActiveSonarrBlock::History)]
  #[case(4, ActiveSonarrBlock::RootFolders)]
  #[case(5, ActiveSonarrBlock::Indexers)]
  #[case(6, ActiveSonarrBlock::System)]
  fn test_sonarr_handler_change_tab_left_right_keys_alt_navigation_no_op_when_ignoring_quit_key(
    #[case] index: usize,
    #[case] block: ActiveSonarrBlock,
  ) {
    let mut app = App::test_default();
    app.push_navigation_stack(block.into());
    app.should_ignore_quit_key = true;
    app.data.sonarr_data.main_tabs.set_index(index);

    handle_change_tab_left_right_keys(&mut app, DEFAULT_KEYBINDINGS.left.alt.unwrap());

    assert_eq!(
      app.data.sonarr_data.main_tabs.get_active_route(),
      block.into()
    );
    assert_eq!(app.get_current_route(), block.into());

    app.data.sonarr_data.main_tabs.set_index(index);

    handle_change_tab_left_right_keys(&mut app, DEFAULT_KEYBINDINGS.right.alt.unwrap());

    assert_eq!(
      app.data.sonarr_data.main_tabs.get_active_route(),
      block.into()
    );
    assert_eq!(app.get_current_route(), block.into());
  }

  #[rstest]
  fn test_delegates_library_blocks_to_library_handler(
    #[values(
      ActiveSonarrBlock::AddSeriesAlreadyInLibrary,
      ActiveSonarrBlock::AddSeriesEmptySearchResults,
      ActiveSonarrBlock::AddSeriesPrompt,
      ActiveSonarrBlock::AddSeriesSearchInput,
      ActiveSonarrBlock::AddSeriesSearchResults,
      ActiveSonarrBlock::AddSeriesSelectLanguageProfile,
      ActiveSonarrBlock::AddSeriesSelectMonitor,
      ActiveSonarrBlock::AddSeriesSelectQualityProfile,
      ActiveSonarrBlock::AddSeriesSelectRootFolder,
      ActiveSonarrBlock::AddSeriesSelectSeriesType,
      ActiveSonarrBlock::AddSeriesTagsInput,
      ActiveSonarrBlock::AutomaticallySearchEpisodePrompt,
      ActiveSonarrBlock::AutomaticallySearchSeasonPrompt,
      ActiveSonarrBlock::AutomaticallySearchSeriesPrompt,
      ActiveSonarrBlock::DeleteEpisodeFilePrompt,
      ActiveSonarrBlock::DeleteSeriesPrompt,
      ActiveSonarrBlock::EditSeriesPrompt,
      ActiveSonarrBlock::EditSeriesPathInput,
      ActiveSonarrBlock::EditSeriesSelectSeriesType,
      ActiveSonarrBlock::EditSeriesSelectQualityProfile,
      ActiveSonarrBlock::EditSeriesSelectLanguageProfile,
      ActiveSonarrBlock::EditSeriesTagsInput,
      ActiveSonarrBlock::EpisodeDetails,
      ActiveSonarrBlock::EpisodeFile,
      ActiveSonarrBlock::EpisodeHistory,
      ActiveSonarrBlock::FilterSeries,
      ActiveSonarrBlock::FilterSeriesError,
      ActiveSonarrBlock::FilterSeriesHistory,
      ActiveSonarrBlock::FilterSeriesHistoryError,
      ActiveSonarrBlock::ManualEpisodeSearch,
      ActiveSonarrBlock::ManualEpisodeSearchConfirmPrompt,
      ActiveSonarrBlock::ManualEpisodeSearchSortPrompt,
      ActiveSonarrBlock::ManualSeasonSearch,
      ActiveSonarrBlock::ManualSeasonSearchConfirmPrompt,
      ActiveSonarrBlock::ManualSeasonSearchSortPrompt,
      ActiveSonarrBlock::SearchEpisodes,
      ActiveSonarrBlock::SearchEpisodesError,
      ActiveSonarrBlock::SearchSeason,
      ActiveSonarrBlock::SearchSeasonError,
      ActiveSonarrBlock::SearchSeries,
      ActiveSonarrBlock::SearchSeriesError,
      ActiveSonarrBlock::SearchSeriesHistory,
      ActiveSonarrBlock::SearchSeriesHistoryError,
      ActiveSonarrBlock::SeasonDetails,
      ActiveSonarrBlock::Series,
      ActiveSonarrBlock::SeriesDetails,
      ActiveSonarrBlock::SeriesHistory,
      ActiveSonarrBlock::SeriesHistorySortPrompt,
      ActiveSonarrBlock::SeriesSortPrompt,
      ActiveSonarrBlock::UpdateAllSeriesPrompt,
      ActiveSonarrBlock::UpdateAndScanSeriesPrompt,
      ActiveSonarrBlock::SeriesHistoryDetails
    )]
    active_sonarr_block: ActiveSonarrBlock,
  ) {
    test_handler_delegation!(
      SonarrHandler,
      ActiveSonarrBlock::Series,
      active_sonarr_block
    );
  }

  #[rstest]
  fn test_delegates_downloads_blocks_to_downloads_handler(
    #[values(
      ActiveSonarrBlock::Downloads,
      ActiveSonarrBlock::DeleteDownloadPrompt,
      ActiveSonarrBlock::UpdateDownloadsPrompt
    )]
    active_sonarr_block: ActiveSonarrBlock,
  ) {
    test_handler_delegation!(
      SonarrHandler,
      ActiveSonarrBlock::Downloads,
      active_sonarr_block
    );
  }

  #[rstest]
  fn test_delegates_blocklist_blocks_to_blocklist_handler(
    #[values(
      ActiveSonarrBlock::Blocklist,
      ActiveSonarrBlock::BlocklistItemDetails,
      ActiveSonarrBlock::DeleteBlocklistItemPrompt,
      ActiveSonarrBlock::BlocklistClearAllItemsPrompt,
      ActiveSonarrBlock::BlocklistSortPrompt
    )]
    active_sonarr_block: ActiveSonarrBlock,
  ) {
    test_handler_delegation!(
      SonarrHandler,
      ActiveSonarrBlock::Blocklist,
      active_sonarr_block
    );
  }

  #[rstest]
  fn test_delegates_history_blocks_to_history_handler(
    #[values(
      ActiveSonarrBlock::History,
      ActiveSonarrBlock::HistoryItemDetails,
      ActiveSonarrBlock::HistorySortPrompt,
      ActiveSonarrBlock::FilterHistory,
      ActiveSonarrBlock::FilterHistoryError,
      ActiveSonarrBlock::SearchHistory,
      ActiveSonarrBlock::SearchHistoryError
    )]
    active_sonarr_block: ActiveSonarrBlock,
  ) {
    test_handler_delegation!(
      SonarrHandler,
      ActiveSonarrBlock::History,
      active_sonarr_block
    );
  }

  #[rstest]
  fn test_delegates_root_folders_blocks_to_root_folders_handler(
    #[values(
      ActiveSonarrBlock::RootFolders,
      ActiveSonarrBlock::AddRootFolderPrompt,
      ActiveSonarrBlock::DeleteRootFolderPrompt
    )]
    active_sonarr_block: ActiveSonarrBlock,
  ) {
    test_handler_delegation!(
      SonarrHandler,
      ActiveSonarrBlock::RootFolders,
      active_sonarr_block
    );
  }

  #[rstest]
  fn test_delegates_indexers_blocks_to_indexers_handler(
    #[values(
      ActiveSonarrBlock::DeleteIndexerPrompt,
      ActiveSonarrBlock::Indexers,
      ActiveSonarrBlock::AllIndexerSettingsPrompt,
      ActiveSonarrBlock::IndexerSettingsConfirmPrompt,
      ActiveSonarrBlock::IndexerSettingsMaximumSizeInput,
      ActiveSonarrBlock::IndexerSettingsMinimumAgeInput,
      ActiveSonarrBlock::IndexerSettingsRetentionInput,
      ActiveSonarrBlock::IndexerSettingsRssSyncIntervalInput
    )]
    active_sonarr_block: ActiveSonarrBlock,
  ) {
    test_handler_delegation!(
      SonarrHandler,
      ActiveSonarrBlock::Indexers,
      active_sonarr_block
    );
  }

  #[rstest]
  fn test_delegates_system_blocks_to_system_handler(
    #[values(
      ActiveSonarrBlock::System,
      ActiveSonarrBlock::SystemLogs,
      ActiveSonarrBlock::SystemQueuedEvents,
      ActiveSonarrBlock::SystemTasks,
      ActiveSonarrBlock::SystemTaskStartConfirmPrompt,
      ActiveSonarrBlock::SystemUpdates
    )]
    active_sonarr_block: ActiveSonarrBlock,
  ) {
    test_handler_delegation!(
      SonarrHandler,
      ActiveSonarrBlock::System,
      active_sonarr_block
    );
  }

  #[test]
  fn test_sonarr_handler_accepts() {
    ActiveSonarrBlock::iter().for_each(|active_sonarr_block| {
      assert!(SonarrHandler::accepts(active_sonarr_block));
    })
  }

  #[rstest]
  fn test_sonarr_handler_ignore_alt_navigation(
    #[values(true, false)] should_ignore_quit_key: bool,
  ) {
    let mut app = App::test_default();
    app.should_ignore_quit_key = should_ignore_quit_key;
    let handler = SonarrHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveSonarrBlock::default(),
      None,
    );

    assert_eq!(handler.ignore_alt_navigation(), should_ignore_quit_key);
  }

  #[test]
  fn test_sonarr_handler_is_ready() {
    let mut app = App::test_default();
    app.is_loading = true;

    let handler = SonarrHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveSonarrBlock::default(),
      None,
    );

    assert!(handler.is_ready());
  }
}
