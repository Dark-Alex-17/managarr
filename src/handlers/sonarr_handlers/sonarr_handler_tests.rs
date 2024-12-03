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
    let mut app = App::default();
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
      // ActiveSonarrBlock::AutomaticallySearchEpisodePrompt,
      // ActiveSonarrBlock::AutomaticallySearchSeasonPrompt,
      // ActiveSonarrBlock::AutomaticallySearchSeriesPrompt,
      // ActiveSonarrBlock::DeleteEpisodeFilePrompt,
      ActiveSonarrBlock::DeleteSeriesPrompt,
      ActiveSonarrBlock::EditSeriesPrompt,
      ActiveSonarrBlock::EditSeriesPathInput,
      ActiveSonarrBlock::EditSeriesSelectSeriesType,
      ActiveSonarrBlock::EditSeriesSelectQualityProfile,
      ActiveSonarrBlock::EditSeriesSelectLanguageProfile,
      ActiveSonarrBlock::EditSeriesTagsInput,
      // ActiveSonarrBlock::EpisodeDetails,
      // ActiveSonarrBlock::EpisodeFile,
      // ActiveSonarrBlock::EpisodeHistory,
      // ActiveSonarrBlock::EpisodesSortPrompt,
      // ActiveSonarrBlock::FilterEpisodes,
      // ActiveSonarrBlock::FilterEpisodesError,
      ActiveSonarrBlock::FilterSeries,
      ActiveSonarrBlock::FilterSeriesError,
      // ActiveSonarrBlock::FilterSeriesHistory,
      // ActiveSonarrBlock::FilterSeriesHistoryError,
      // ActiveSonarrBlock::ManualEpisodeSearch,
      // ActiveSonarrBlock::ManualEpisodeSearchConfirmPrompt,
      // ActiveSonarrBlock::ManualEpisodeSearchSortPrompt,
      // ActiveSonarrBlock::ManualSeasonSearch,
      // ActiveSonarrBlock::ManualSeasonSearchConfirmPrompt,
      // ActiveSonarrBlock::ManualSeasonSearchSortPrompt,
      // ActiveSonarrBlock::SearchEpisodes,
      // ActiveSonarrBlock::SearchEpisodesError,
      // ActiveSonarrBlock::SearchSeason,
      // ActiveSonarrBlock::SearchSeasonError,
      ActiveSonarrBlock::SearchSeries,
      ActiveSonarrBlock::SearchSeriesError,
      // ActiveSonarrBlock::SearchSeriesHistory,
      // ActiveSonarrBlock::SearchSeriesHistoryError,
      // ActiveSonarrBlock::SeasonDetails,
      ActiveSonarrBlock::Series,
      // ActiveSonarrBlock::SeriesDetails,
      // ActiveSonarrBlock::SeriesHistory,
      // ActiveSonarrBlock::SeriesHistorySortPrompt,
      ActiveSonarrBlock::SeriesSortPrompt,
      ActiveSonarrBlock::UpdateAllSeriesPrompt,
      // ActiveSonarrBlock::UpdateAndScanSeriesPrompt
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
}
