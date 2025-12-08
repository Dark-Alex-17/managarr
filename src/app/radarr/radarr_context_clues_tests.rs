#[cfg(test)]
mod tests {
	use crate::app::context_clues::{
		ContextClue, ContextClueProvider, BARE_POPUP_CONTEXT_CLUES, BLOCKLIST_CONTEXT_CLUES,
		CONFIRMATION_PROMPT_CONTEXT_CLUES, DOWNLOADS_CONTEXT_CLUES, INDEXERS_CONTEXT_CLUES,
		ROOT_FOLDERS_CONTEXT_CLUES, SYSTEM_CONTEXT_CLUES,
	};
	use crate::app::key_binding::DEFAULT_KEYBINDINGS;
	use crate::app::radarr::radarr_context_clues::{
		RadarrContextClueProvider, ADD_MOVIE_SEARCH_RESULTS_CONTEXT_CLUES, COLLECTIONS_CONTEXT_CLUES,
		COLLECTION_DETAILS_CONTEXT_CLUES, LIBRARY_CONTEXT_CLUES, MANUAL_MOVIE_SEARCH_CONTEXT_CLUES,
		MOVIE_DETAILS_CONTEXT_CLUES, SYSTEM_TASKS_CONTEXT_CLUES,
	};
	use crate::app::App;
	use crate::models::servarr_data::radarr::radarr_data::{ActiveRadarrBlock, RadarrData};
	use crate::models::servarr_data::sonarr::sonarr_data::ActiveSonarrBlock;
	use pretty_assertions::assert_eq;
	use rstest::rstest;

	#[test]
  fn test_library_context_clues() {
    let mut library_context_clues_iter = LIBRARY_CONTEXT_CLUES.iter();

    assert_some_eq_x!(
      library_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.add, DEFAULT_KEYBINDINGS.add.desc)
    );
    assert_some_eq_x!(
      library_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.edit, DEFAULT_KEYBINDINGS.edit.desc)
    );
    assert_some_eq_x!(
      library_context_clues_iter.next(),
      &(
        DEFAULT_KEYBINDINGS.toggle_monitoring,
        DEFAULT_KEYBINDINGS.toggle_monitoring.desc
      )
    );
    assert_some_eq_x!(
      library_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.sort, DEFAULT_KEYBINDINGS.sort.desc)
    );
    assert_some_eq_x!(
      library_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.delete, DEFAULT_KEYBINDINGS.delete.desc)
    );
    assert_some_eq_x!(
      library_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.search, DEFAULT_KEYBINDINGS.search.desc)
    );
    assert_some_eq_x!(
      library_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.filter, DEFAULT_KEYBINDINGS.filter.desc)
    );
    assert_some_eq_x!(
      library_context_clues_iter.next(),
      &(
        DEFAULT_KEYBINDINGS.refresh,
        DEFAULT_KEYBINDINGS.refresh.desc
      )
    );
    assert_some_eq_x!(
      library_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.update, "update all")
    );
    assert_some_eq_x!(
      library_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.submit, "details")
    );
    assert_some_eq_x!(
      library_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.esc, "cancel filter")
    );
    assert_none!(library_context_clues_iter.next());
  }

  #[test]
  fn test_collections_context_clues() {
    let mut collections_context_clues = COLLECTIONS_CONTEXT_CLUES.iter();

    assert_some_eq_x!(
      collections_context_clues.next(),
      &(DEFAULT_KEYBINDINGS.search, DEFAULT_KEYBINDINGS.search.desc)
    );
    assert_some_eq_x!(
      collections_context_clues.next(),
      &(DEFAULT_KEYBINDINGS.edit, DEFAULT_KEYBINDINGS.edit.desc)
    );
    assert_some_eq_x!(
      collections_context_clues.next(),
      &(DEFAULT_KEYBINDINGS.sort, DEFAULT_KEYBINDINGS.sort.desc)
    );
    assert_some_eq_x!(
      collections_context_clues.next(),
      &(DEFAULT_KEYBINDINGS.filter, DEFAULT_KEYBINDINGS.filter.desc)
    );
    assert_some_eq_x!(
      collections_context_clues.next(),
      &(
        DEFAULT_KEYBINDINGS.refresh,
        DEFAULT_KEYBINDINGS.refresh.desc
      )
    );
    assert_some_eq_x!(
      collections_context_clues.next(),
      &(DEFAULT_KEYBINDINGS.update, "update all")
    );
    assert_some_eq_x!(
      collections_context_clues.next(),
      &(DEFAULT_KEYBINDINGS.submit, "details")
    );
    assert_some_eq_x!(
      collections_context_clues.next(),
      &(DEFAULT_KEYBINDINGS.esc, "cancel filter")
    );
    assert_eq!(collections_context_clues.next(), None);
  }

  #[test]
  fn test_movie_details_context_clues() {
    let mut movie_details_context_clues_iter = MOVIE_DETAILS_CONTEXT_CLUES.iter();

    assert_some_eq_x!(
      movie_details_context_clues_iter.next(),
      &(
        DEFAULT_KEYBINDINGS.refresh,
        DEFAULT_KEYBINDINGS.refresh.desc
      )
    );
    assert_some_eq_x!(
      movie_details_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.update, DEFAULT_KEYBINDINGS.update.desc)
    );
    assert_some_eq_x!(
      movie_details_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.edit, DEFAULT_KEYBINDINGS.edit.desc)
    );
    assert_some_eq_x!(
      movie_details_context_clues_iter.next(),
      &(
        DEFAULT_KEYBINDINGS.auto_search,
        DEFAULT_KEYBINDINGS.auto_search.desc
      )
    );
    assert_some_eq_x!(
      movie_details_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.esc, DEFAULT_KEYBINDINGS.esc.desc)
    );
    assert_eq!(movie_details_context_clues_iter.next(), None);
  }

  #[test]
  fn test_manual_movie_search_context_clues() {
    let mut manual_movie_search_context_clues_iter = MANUAL_MOVIE_SEARCH_CONTEXT_CLUES.iter();

    assert_some_eq_x!(
      manual_movie_search_context_clues_iter.next(),
      &(
        DEFAULT_KEYBINDINGS.refresh,
        DEFAULT_KEYBINDINGS.refresh.desc
      )
    );
    assert_some_eq_x!(
      manual_movie_search_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.update, DEFAULT_KEYBINDINGS.update.desc)
    );
    assert_some_eq_x!(
      manual_movie_search_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.edit, DEFAULT_KEYBINDINGS.edit.desc)
    );
    assert_some_eq_x!(
      manual_movie_search_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.sort, DEFAULT_KEYBINDINGS.sort.desc)
    );
    assert_some_eq_x!(
      manual_movie_search_context_clues_iter.next(),
      &(
        DEFAULT_KEYBINDINGS.auto_search,
        DEFAULT_KEYBINDINGS.auto_search.desc
      )
    );
    assert_some_eq_x!(
      manual_movie_search_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.submit, "details")
    );
    assert_some_eq_x!(
      manual_movie_search_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.esc, DEFAULT_KEYBINDINGS.esc.desc)
    );
    assert_eq!(manual_movie_search_context_clues_iter.next(), None);
  }

  #[test]
  fn test_add_movie_search_results_context_clues() {
    let mut add_movie_search_results_context_clues_iter =
      ADD_MOVIE_SEARCH_RESULTS_CONTEXT_CLUES.iter();

    assert_some_eq_x!(
      add_movie_search_results_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.submit, "details")
    );
    assert_some_eq_x!(
      add_movie_search_results_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.esc, "edit search")
    );
    assert_eq!(add_movie_search_results_context_clues_iter.next(), None);
  }

  #[test]
  fn test_system_tasks_context_clues() {
    let mut system_tasks_context_clues_iter = SYSTEM_TASKS_CONTEXT_CLUES.iter();

    assert_some_eq_x!(
      system_tasks_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.submit, "start task")
    );
    assert_some_eq_x!(
      system_tasks_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.esc, DEFAULT_KEYBINDINGS.esc.desc)
    );
    assert_eq!(system_tasks_context_clues_iter.next(), None);
  }

  #[test]
  fn test_collection_details_context_clues() {
    let mut collection_details_context_clues_iter = COLLECTION_DETAILS_CONTEXT_CLUES.iter();

    assert_some_eq_x!(
      collection_details_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.submit, "show overview/add movie")
    );
    assert_some_eq_x!(
      collection_details_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.edit, "edit collection")
    );
    assert_some_eq_x!(
      collection_details_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.esc, DEFAULT_KEYBINDINGS.esc.desc)
    );
    assert_eq!(collection_details_context_clues_iter.next(), None);
  }

  #[test]
  #[should_panic(
    expected = "RadarrContextClueProvider::get_context_clues called with non-Radarr route"
  )]
  fn test_radarr_context_clue_provider_get_context_clues_non_radarr_route() {
    let mut app = App::test_default();
    app.push_navigation_stack(ActiveSonarrBlock::default().into());

    // This should panic because the route is not a Radarr route
    RadarrContextClueProvider::get_context_clues(&mut app);
  }

  #[rstest]
  #[case(ActiveRadarrBlock::TestAllIndexers, None)]
  #[case(ActiveRadarrBlock::AddMovieSearchInput, None)]
  #[case(ActiveRadarrBlock::AddMovieEmptySearchResults, None)]
  #[case(ActiveRadarrBlock::SystemLogs, None)]
  #[case(ActiveRadarrBlock::SystemUpdates, None)]
  #[case(ActiveRadarrBlock::ViewMovieOverview, None)]
  #[case(
    ActiveRadarrBlock::CollectionDetails,
    Some(ActiveRadarrBlock::ViewMovieOverview)
  )]
  fn test_radarr_context_clue_provider_bare_popup_context_clues(
    #[case] active_radarr_block: ActiveRadarrBlock,
    #[case] context_option: Option<ActiveRadarrBlock>,
  ) {
    let mut app = App::test_default();
    app.push_navigation_stack((active_radarr_block, context_option).into());

    let context_clues = RadarrContextClueProvider::get_context_clues(&mut app);

    assert_some_eq_x!(context_clues, &BARE_POPUP_CONTEXT_CLUES);
  }

  #[rstest]
  #[case(0, ActiveRadarrBlock::MovieDetails, &MOVIE_DETAILS_CONTEXT_CLUES)]
  #[case(1, ActiveRadarrBlock::MovieHistory, &MOVIE_DETAILS_CONTEXT_CLUES)]
  #[case(2, ActiveRadarrBlock::FileInfo, &MOVIE_DETAILS_CONTEXT_CLUES)]
  #[case(3, ActiveRadarrBlock::Cast, &MOVIE_DETAILS_CONTEXT_CLUES)]
  #[case(4, ActiveRadarrBlock::Crew, &MOVIE_DETAILS_CONTEXT_CLUES)]
  #[case(5, ActiveRadarrBlock::ManualSearch, &MANUAL_MOVIE_SEARCH_CONTEXT_CLUES)]
  fn test_radarr_context_clue_provider_movie_details_block_context_clues(
    #[case] index: usize,
    #[case] active_radarr_block: ActiveRadarrBlock,
    #[case] expected_context_clues: &[ContextClue],
  ) {
    let mut app = App::test_default();
    app.data.radarr_data = RadarrData::default();
    app.data.radarr_data.movie_info_tabs.set_index(index);
    app.push_navigation_stack(active_radarr_block.into());

    let context_clues = RadarrContextClueProvider::get_context_clues(&mut app);

    assert_some_eq_x!(context_clues, expected_context_clues);
  }

  #[rstest]
  fn test_radarr_context_clue_provider_confirmation_prompt_context_clues(
    #[values(
      ActiveRadarrBlock::AddMoviePrompt,
      ActiveRadarrBlock::AddMovieSelectMonitor,
      ActiveRadarrBlock::AddMovieSelectMinimumAvailability,
      ActiveRadarrBlock::AddMovieSelectQualityProfile,
      ActiveRadarrBlock::AddMovieSelectRootFolder,
      ActiveRadarrBlock::AddMovieTagsInput,
      ActiveRadarrBlock::SystemTaskStartConfirmPrompt
    )]
    active_radarr_block: ActiveRadarrBlock,
  ) {
    let mut app = App::test_default();
    app.data.radarr_data = RadarrData::default();
    app.push_navigation_stack(active_radarr_block.into());

    let context_clues = RadarrContextClueProvider::get_context_clues(&mut app);

    assert_some_eq_x!(context_clues, &CONFIRMATION_PROMPT_CONTEXT_CLUES);
  }

  #[rstest]
  fn test_radarr_context_clue_provider_confirmation_prompt_context_clues_edit_collection_blocks(
    #[values(
      ActiveRadarrBlock::EditCollectionPrompt,
      ActiveRadarrBlock::EditCollectionConfirmPrompt,
      ActiveRadarrBlock::EditCollectionRootFolderPathInput,
      ActiveRadarrBlock::EditCollectionSelectMinimumAvailability,
      ActiveRadarrBlock::EditCollectionSelectQualityProfile,
      ActiveRadarrBlock::EditCollectionToggleSearchOnAdd,
      ActiveRadarrBlock::EditCollectionToggleMonitored
    )]
    active_radarr_block: ActiveRadarrBlock,
  ) {
    let mut app = App::test_default();
    app.data.radarr_data = RadarrData::default();
    app.push_navigation_stack(active_radarr_block.into());

    let context_clues = RadarrContextClueProvider::get_context_clues(&mut app);

    assert_some_eq_x!(context_clues, &CONFIRMATION_PROMPT_CONTEXT_CLUES);
  }

  #[rstest]
  fn test_radarr_context_clue_provider_confirmation_prompt_context_clues_edit_indexer_blocks(
    #[values(
      ActiveRadarrBlock::EditIndexerPrompt,
      ActiveRadarrBlock::EditIndexerConfirmPrompt,
      ActiveRadarrBlock::EditIndexerApiKeyInput,
      ActiveRadarrBlock::EditIndexerNameInput,
      ActiveRadarrBlock::EditIndexerSeedRatioInput,
      ActiveRadarrBlock::EditIndexerToggleEnableRss,
      ActiveRadarrBlock::EditIndexerToggleEnableAutomaticSearch,
      ActiveRadarrBlock::EditIndexerToggleEnableInteractiveSearch,
      ActiveRadarrBlock::EditIndexerPriorityInput,
      ActiveRadarrBlock::EditIndexerUrlInput,
      ActiveRadarrBlock::EditIndexerTagsInput
    )]
    active_radarr_block: ActiveRadarrBlock,
  ) {
    let mut app = App::test_default();
    app.data.radarr_data = RadarrData::default();
    app.push_navigation_stack(active_radarr_block.into());

    let context_clues = RadarrContextClueProvider::get_context_clues(&mut app);

    assert_some_eq_x!(context_clues, &CONFIRMATION_PROMPT_CONTEXT_CLUES);
  }

  #[rstest]
  fn test_radarr_context_clue_provider_confirmation_prompt_context_clues_indexer_settings_blocks(
    #[values(
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
    let mut app = App::test_default();
    app.data.radarr_data = RadarrData::default();
    app.push_navigation_stack(active_radarr_block.into());

    let context_clues = RadarrContextClueProvider::get_context_clues(&mut app);

    assert_some_eq_x!(context_clues, &CONFIRMATION_PROMPT_CONTEXT_CLUES);
  }

  #[rstest]
  fn test_radarr_context_clue_provider_confirmation_prompt_context_clues_edit_movie_blocks(
    #[values(
      ActiveRadarrBlock::EditMoviePrompt,
      ActiveRadarrBlock::EditMovieConfirmPrompt,
      ActiveRadarrBlock::EditMoviePathInput,
      ActiveRadarrBlock::EditMovieSelectMinimumAvailability,
      ActiveRadarrBlock::EditMovieSelectQualityProfile,
      ActiveRadarrBlock::EditMovieTagsInput,
      ActiveRadarrBlock::EditMovieToggleMonitored
    )]
    active_radarr_block: ActiveRadarrBlock,
  ) {
    let mut app = App::test_default();
    app.data.radarr_data = RadarrData::default();
    app.push_navigation_stack(active_radarr_block.into());

    let context_clues = RadarrContextClueProvider::get_context_clues(&mut app);

    assert_some_eq_x!(context_clues, &CONFIRMATION_PROMPT_CONTEXT_CLUES);
  }

  #[rstest]
  fn test_radarr_context_clue_provider_add_movie_search_results_context_clues(
    #[values(
      ActiveRadarrBlock::AddMovieSearchResults,
      ActiveRadarrBlock::AddMovieAlreadyInLibrary
    )]
    active_radarr_block: ActiveRadarrBlock,
  ) {
    let mut app = App::test_default();
    app.data.radarr_data = RadarrData::default();
    app.push_navigation_stack(active_radarr_block.into());

    let context_clues = RadarrContextClueProvider::get_context_clues(&mut app);

    assert_some_eq_x!(context_clues, &ADD_MOVIE_SEARCH_RESULTS_CONTEXT_CLUES);
  }

  #[test]
  fn test_radarr_context_clue_provider_collection_details_context_clues() {
    let mut app = App::test_default();
    app.data.radarr_data = RadarrData::default();
    app.push_navigation_stack(ActiveRadarrBlock::CollectionDetails.into());

    let context_clues = RadarrContextClueProvider::get_context_clues(&mut app);

    assert_some_eq_x!(context_clues, &COLLECTION_DETAILS_CONTEXT_CLUES);
  }

  #[test]
  fn test_radarr_context_clue_provider_system_tasks_context_clues() {
    let mut app = App::test_default();
    app.data.radarr_data = RadarrData::default();

    app.push_navigation_stack(ActiveRadarrBlock::SystemTasks.into());

    let context_clues = RadarrContextClueProvider::get_context_clues(&mut app);

    assert_some_eq_x!(context_clues, &SYSTEM_TASKS_CONTEXT_CLUES);
  }

  #[rstest]
  #[case(0, ActiveRadarrBlock::Movies, &LIBRARY_CONTEXT_CLUES)]
  #[case(1, ActiveRadarrBlock::Collections, &COLLECTIONS_CONTEXT_CLUES)]
  #[case(2, ActiveRadarrBlock::Downloads, &DOWNLOADS_CONTEXT_CLUES)]
  #[case(3, ActiveRadarrBlock::Blocklist, &BLOCKLIST_CONTEXT_CLUES)]
  #[case(4, ActiveRadarrBlock::RootFolders, &ROOT_FOLDERS_CONTEXT_CLUES)]
  #[case(5, ActiveRadarrBlock::Indexers, &INDEXERS_CONTEXT_CLUES)]
  #[case(6, ActiveRadarrBlock::System, &SYSTEM_CONTEXT_CLUES)]
  fn test_radarr_context_clue_provider_radarr_blocks_context_clues(
    #[case] index: usize,
    #[case] active_radarr_block: ActiveRadarrBlock,
    #[case] expected_context_clues: &[ContextClue],
  ) {
    let mut app = App::test_default();
    app.data.radarr_data = RadarrData::default();
    app.data.radarr_data.main_tabs.set_index(index);
    app.push_navigation_stack(active_radarr_block.into());

    let context_clues = RadarrContextClueProvider::get_context_clues(&mut app);

    assert_some_eq_x!(context_clues, expected_context_clues);
  }
}
