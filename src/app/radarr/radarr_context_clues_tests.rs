#[cfg(test)]
mod tests {
  use crate::app::App;
  use crate::app::context_clues::{
    BARE_POPUP_CONTEXT_CLUES, BLOCKLIST_CONTEXT_CLUES, CONFIRMATION_PROMPT_CONTEXT_CLUES,
    ContextClue, ContextClueProvider, DOWNLOADS_CONTEXT_CLUES, INDEXERS_CONTEXT_CLUES,
    ROOT_FOLDERS_CONTEXT_CLUES, SYSTEM_CONTEXT_CLUES,
  };
  use crate::app::key_binding::DEFAULT_KEYBINDINGS;
  use crate::app::radarr::radarr_context_clues::{
    ADD_MOVIE_SEARCH_RESULTS_CONTEXT_CLUES, COLLECTION_DETAILS_CONTEXT_CLUES,
    COLLECTIONS_CONTEXT_CLUES, LIBRARY_CONTEXT_CLUES, MANUAL_MOVIE_SEARCH_CONTEXT_CLUES,
    MOVIE_DETAILS_CONTEXT_CLUES, RadarrContextClueProvider, SYSTEM_TASKS_CONTEXT_CLUES,
  };
  use crate::models::servarr_data::radarr::radarr_data::{ActiveRadarrBlock, RadarrData};
  use crate::models::servarr_data::sonarr::sonarr_data::ActiveSonarrBlock;
  use pretty_assertions::{assert_eq, assert_str_eq};
  use rstest::rstest;

  #[test]
  fn test_library_context_clues() {
    let mut library_context_clues_iter = LIBRARY_CONTEXT_CLUES.iter();

    let (key_binding, description) = library_context_clues_iter.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.add);
    assert_str_eq!(*description, DEFAULT_KEYBINDINGS.add.desc);

    let (key_binding, description) = library_context_clues_iter.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.edit);
    assert_str_eq!(*description, DEFAULT_KEYBINDINGS.edit.desc);

    let (key_binding, description) = library_context_clues_iter.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.toggle_monitoring);
    assert_str_eq!(*description, DEFAULT_KEYBINDINGS.toggle_monitoring.desc);

    let (key_binding, description) = library_context_clues_iter.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.sort);
    assert_str_eq!(*description, DEFAULT_KEYBINDINGS.sort.desc);

    let (key_binding, description) = library_context_clues_iter.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.delete);
    assert_str_eq!(*description, DEFAULT_KEYBINDINGS.delete.desc);

    let (key_binding, description) = library_context_clues_iter.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.search);
    assert_str_eq!(*description, DEFAULT_KEYBINDINGS.search.desc);

    let (key_binding, description) = library_context_clues_iter.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.filter);
    assert_str_eq!(*description, DEFAULT_KEYBINDINGS.filter.desc);

    let (key_binding, description) = library_context_clues_iter.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.refresh);
    assert_str_eq!(*description, DEFAULT_KEYBINDINGS.refresh.desc);

    let (key_binding, description) = library_context_clues_iter.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.update);
    assert_str_eq!(*description, "update all");

    let (key_binding, description) = library_context_clues_iter.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.submit);
    assert_str_eq!(*description, "details");

    let (key_binding, description) = library_context_clues_iter.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.esc);
    assert_str_eq!(*description, "cancel filter");
    assert_eq!(library_context_clues_iter.next(), None);
  }

  #[test]
  fn test_collections_context_clues() {
    let mut collections_context_clues = COLLECTIONS_CONTEXT_CLUES.iter();

    let (key_binding, description) = collections_context_clues.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.search);
    assert_str_eq!(*description, DEFAULT_KEYBINDINGS.search.desc);

    let (key_binding, description) = collections_context_clues.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.edit);
    assert_str_eq!(*description, DEFAULT_KEYBINDINGS.edit.desc);

    let (key_binding, description) = collections_context_clues.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.sort);
    assert_str_eq!(*description, DEFAULT_KEYBINDINGS.sort.desc);

    let (key_binding, description) = collections_context_clues.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.filter);
    assert_str_eq!(*description, DEFAULT_KEYBINDINGS.filter.desc);

    let (key_binding, description) = collections_context_clues.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.refresh);
    assert_str_eq!(*description, DEFAULT_KEYBINDINGS.refresh.desc);

    let (key_binding, description) = collections_context_clues.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.update);
    assert_str_eq!(*description, "update all");

    let (key_binding, description) = collections_context_clues.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.submit);
    assert_str_eq!(*description, "details");

    let (key_binding, description) = collections_context_clues.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.esc);
    assert_str_eq!(*description, "cancel filter");
    assert_eq!(collections_context_clues.next(), None);
  }

  #[test]
  fn test_movie_details_context_clues() {
    let mut movie_details_context_clues_iter = MOVIE_DETAILS_CONTEXT_CLUES.iter();

    let (key_binding, description) = movie_details_context_clues_iter.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.refresh);
    assert_str_eq!(*description, DEFAULT_KEYBINDINGS.refresh.desc);

    let (key_binding, description) = movie_details_context_clues_iter.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.update);
    assert_str_eq!(*description, DEFAULT_KEYBINDINGS.update.desc);

    let (key_binding, description) = movie_details_context_clues_iter.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.edit);
    assert_str_eq!(*description, DEFAULT_KEYBINDINGS.edit.desc);

    let (key_binding, description) = movie_details_context_clues_iter.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.auto_search);
    assert_str_eq!(*description, DEFAULT_KEYBINDINGS.auto_search.desc);

    let (key_binding, description) = movie_details_context_clues_iter.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.esc);
    assert_str_eq!(*description, DEFAULT_KEYBINDINGS.esc.desc);
    assert_eq!(movie_details_context_clues_iter.next(), None);
  }

  #[test]
  fn test_manual_movie_search_context_clues() {
    let mut manual_movie_search_context_clues_iter = MANUAL_MOVIE_SEARCH_CONTEXT_CLUES.iter();

    let (key_binding, description) = manual_movie_search_context_clues_iter.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.refresh);
    assert_str_eq!(*description, DEFAULT_KEYBINDINGS.refresh.desc);

    let (key_binding, description) = manual_movie_search_context_clues_iter.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.update);
    assert_str_eq!(*description, DEFAULT_KEYBINDINGS.update.desc);

    let (key_binding, description) = manual_movie_search_context_clues_iter.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.edit);
    assert_str_eq!(*description, DEFAULT_KEYBINDINGS.edit.desc);

    let (key_binding, description) = manual_movie_search_context_clues_iter.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.sort);
    assert_str_eq!(*description, DEFAULT_KEYBINDINGS.sort.desc);

    let (key_binding, description) = manual_movie_search_context_clues_iter.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.auto_search);
    assert_str_eq!(*description, DEFAULT_KEYBINDINGS.auto_search.desc);

    let (key_binding, description) = manual_movie_search_context_clues_iter.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.submit);
    assert_str_eq!(*description, "details");

    let (key_binding, description) = manual_movie_search_context_clues_iter.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.esc);
    assert_str_eq!(*description, DEFAULT_KEYBINDINGS.esc.desc);

    assert_eq!(manual_movie_search_context_clues_iter.next(), None);
  }

  #[test]
  fn test_add_movie_search_results_context_clues() {
    let mut add_movie_search_results_context_clues_iter =
      ADD_MOVIE_SEARCH_RESULTS_CONTEXT_CLUES.iter();

    let (key_binding, description) = add_movie_search_results_context_clues_iter.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.submit);
    assert_str_eq!(*description, "details");

    let (key_binding, description) = add_movie_search_results_context_clues_iter.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.esc);
    assert_str_eq!(*description, "edit search");
    assert_eq!(add_movie_search_results_context_clues_iter.next(), None);
  }

  #[test]
  fn test_system_tasks_context_clues() {
    let mut system_tasks_context_clues_iter = SYSTEM_TASKS_CONTEXT_CLUES.iter();

    let (key_binding, description) = system_tasks_context_clues_iter.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.submit);
    assert_str_eq!(*description, "start task");

    let (key_binding, description) = system_tasks_context_clues_iter.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.esc);
    assert_str_eq!(*description, DEFAULT_KEYBINDINGS.esc.desc);
    assert_eq!(system_tasks_context_clues_iter.next(), None);
  }

  #[test]
  fn test_collection_details_context_clues() {
    let mut collection_details_context_clues_iter = COLLECTION_DETAILS_CONTEXT_CLUES.iter();

    let (key_binding, description) = collection_details_context_clues_iter.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.submit);
    assert_str_eq!(*description, "show overview/add movie");

    let (key_binding, description) = collection_details_context_clues_iter.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.edit);
    assert_str_eq!(*description, "edit collection");

    let (key_binding, description) = collection_details_context_clues_iter.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.esc);
    assert_str_eq!(*description, DEFAULT_KEYBINDINGS.esc.desc);
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

    assert!(context_clues.is_some());
    assert_eq!(&BARE_POPUP_CONTEXT_CLUES, context_clues.unwrap());
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

    assert!(context_clues.is_some());
    assert_eq!(expected_context_clues, context_clues.unwrap());
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

    assert!(context_clues.is_some());
    assert_eq!(&CONFIRMATION_PROMPT_CONTEXT_CLUES, context_clues.unwrap());
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

    assert!(context_clues.is_some());
    assert_eq!(&CONFIRMATION_PROMPT_CONTEXT_CLUES, context_clues.unwrap());
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

    assert!(context_clues.is_some());
    assert_eq!(&CONFIRMATION_PROMPT_CONTEXT_CLUES, context_clues.unwrap());
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

    assert!(context_clues.is_some());
    assert_eq!(&CONFIRMATION_PROMPT_CONTEXT_CLUES, context_clues.unwrap());
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

    assert!(context_clues.is_some());
    assert_eq!(&CONFIRMATION_PROMPT_CONTEXT_CLUES, context_clues.unwrap());
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

    assert!(context_clues.is_some());
    assert_eq!(
      &ADD_MOVIE_SEARCH_RESULTS_CONTEXT_CLUES,
      context_clues.unwrap()
    );
  }

  #[test]
  fn test_radarr_context_clue_provider_collection_details_context_clues() {
    let mut app = App::test_default();
    app.data.radarr_data = RadarrData::default();
    app.push_navigation_stack(ActiveRadarrBlock::CollectionDetails.into());

    let context_clues = RadarrContextClueProvider::get_context_clues(&mut app);

    assert!(context_clues.is_some());
    assert_eq!(&COLLECTION_DETAILS_CONTEXT_CLUES, context_clues.unwrap());
  }

  #[test]
  fn test_radarr_context_clue_provider_system_tasks_context_clues() {
    let mut app = App::test_default();
    app.data.radarr_data = RadarrData::default();

    app.push_navigation_stack(ActiveRadarrBlock::SystemTasks.into());

    let context_clues = RadarrContextClueProvider::get_context_clues(&mut app);

    assert!(context_clues.is_some());
    assert_eq!(&SYSTEM_TASKS_CONTEXT_CLUES, context_clues.unwrap());
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

    assert!(context_clues.is_some());
    assert_eq!(expected_context_clues, context_clues.unwrap());
  }
}
