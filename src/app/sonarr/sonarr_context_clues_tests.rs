#[cfg(test)]
mod tests {
  use crate::app::context_clues::{
    BARE_POPUP_CONTEXT_CLUES, BLOCKLIST_CONTEXT_CLUES, CONFIRMATION_PROMPT_CONTEXT_CLUES,
    ContextClue, ContextClueProvider, DOWNLOADS_CONTEXT_CLUES, INDEXERS_CONTEXT_CLUES,
    ROOT_FOLDERS_CONTEXT_CLUES, SYSTEM_CONTEXT_CLUES,
  };
  use crate::app::sonarr::sonarr_context_clues::{
    SELECTABLE_EPISODE_DETAILS_CONTEXT_CLUES, SonarrContextClueProvider,
  };
  use crate::app::{
    App,
    key_binding::DEFAULT_KEYBINDINGS,
    sonarr::sonarr_context_clues::{
      ADD_SERIES_SEARCH_RESULTS_CONTEXT_CLUES, EPISODE_DETAILS_CONTEXT_CLUES,
      HISTORY_CONTEXT_CLUES, MANUAL_EPISODE_SEARCH_CONTEXT_CLUES,
      MANUAL_SEASON_SEARCH_CONTEXT_CLUES, SEASON_DETAILS_CONTEXT_CLUES,
      SEASON_HISTORY_CONTEXT_CLUES, SERIES_CONTEXT_CLUES, SERIES_DETAILS_CONTEXT_CLUES,
      SERIES_HISTORY_CONTEXT_CLUES, SYSTEM_TASKS_CONTEXT_CLUES,
    },
  };
  use crate::models::servarr_data::radarr::radarr_data::ActiveRadarrBlock;
  use crate::models::servarr_data::sonarr::modals::{EpisodeDetailsModal, SeasonDetailsModal};
  use crate::models::servarr_data::sonarr::sonarr_data::{ActiveSonarrBlock, SonarrData};
  use rstest::rstest;

  #[test]
  fn test_add_series_search_results_context_clues() {
    let mut add_series_search_results_context_clues_iter =
      ADD_SERIES_SEARCH_RESULTS_CONTEXT_CLUES.iter();

    assert_some_eq_x!(
      add_series_search_results_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.submit, "details")
    );
    assert_some_eq_x!(
      add_series_search_results_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.esc, "edit search")
    );
    assert_none!(add_series_search_results_context_clues_iter.next());
  }

  #[test]
  fn test_series_context_clues() {
    let mut series_context_clues_iter = SERIES_CONTEXT_CLUES.iter();

    assert_some_eq_x!(
      series_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.add, DEFAULT_KEYBINDINGS.add.desc)
    );
    assert_some_eq_x!(
      series_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.edit, DEFAULT_KEYBINDINGS.edit.desc)
    );
    assert_some_eq_x!(
      series_context_clues_iter.next(),
      &(
        DEFAULT_KEYBINDINGS.toggle_monitoring,
        DEFAULT_KEYBINDINGS.toggle_monitoring.desc
      )
    );
    assert_some_eq_x!(
      series_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.sort, DEFAULT_KEYBINDINGS.sort.desc)
    );
    assert_some_eq_x!(
      series_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.delete, DEFAULT_KEYBINDINGS.delete.desc)
    );
    assert_some_eq_x!(
      series_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.search, DEFAULT_KEYBINDINGS.search.desc)
    );
    assert_some_eq_x!(
      series_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.filter, DEFAULT_KEYBINDINGS.filter.desc)
    );
    assert_some_eq_x!(
      series_context_clues_iter.next(),
      &(
        DEFAULT_KEYBINDINGS.refresh,
        DEFAULT_KEYBINDINGS.refresh.desc
      )
    );
    assert_some_eq_x!(
      series_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.update, "update all")
    );
    assert_some_eq_x!(
      series_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.submit, "details")
    );
    assert_some_eq_x!(
      series_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.esc, "cancel filter")
    );
    assert_none!(series_context_clues_iter.next());
  }

  #[test]
  fn test_series_history_context_clues() {
    let mut series_history_context_clues_iter = SERIES_HISTORY_CONTEXT_CLUES.iter();

    assert_some_eq_x!(
      series_history_context_clues_iter.next(),
      &(
        DEFAULT_KEYBINDINGS.refresh,
        DEFAULT_KEYBINDINGS.refresh.desc
      )
    );
    assert_some_eq_x!(
      series_history_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.edit, DEFAULT_KEYBINDINGS.edit.desc)
    );
    assert_some_eq_x!(
      series_history_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.submit, "details")
    );
    assert_some_eq_x!(
      series_history_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.sort, DEFAULT_KEYBINDINGS.sort.desc)
    );
    assert_some_eq_x!(
      series_history_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.search, DEFAULT_KEYBINDINGS.search.desc)
    );
    assert_some_eq_x!(
      series_history_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.filter, DEFAULT_KEYBINDINGS.filter.desc)
    );
    assert_some_eq_x!(
      series_history_context_clues_iter.next(),
      &(
        DEFAULT_KEYBINDINGS.auto_search,
        DEFAULT_KEYBINDINGS.auto_search.desc
      )
    );
    assert_some_eq_x!(
      series_history_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.update, DEFAULT_KEYBINDINGS.update.desc)
    );
    assert_some_eq_x!(
      series_history_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.esc, "cancel filter/close")
    );
    assert_none!(series_history_context_clues_iter.next());
  }

  #[test]
  fn test_history_context_clues() {
    let mut history_context_clues_iter = HISTORY_CONTEXT_CLUES.iter();

    assert_some_eq_x!(
      history_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.submit, "details")
    );
    assert_some_eq_x!(
      history_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.sort, DEFAULT_KEYBINDINGS.sort.desc)
    );
    assert_some_eq_x!(
      history_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.search, DEFAULT_KEYBINDINGS.search.desc)
    );
    assert_some_eq_x!(
      history_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.filter, DEFAULT_KEYBINDINGS.filter.desc)
    );
    assert_some_eq_x!(
      history_context_clues_iter.next(),
      &(
        DEFAULT_KEYBINDINGS.refresh,
        DEFAULT_KEYBINDINGS.refresh.desc
      )
    );
    assert_some_eq_x!(
      history_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.esc, "cancel filter")
    );
    assert_none!(history_context_clues_iter.next());
  }

  #[test]
  fn test_series_details_context_clues() {
    let mut series_details_context_clues_iter = SERIES_DETAILS_CONTEXT_CLUES.iter();

    assert_some_eq_x!(
      series_details_context_clues_iter.next(),
      &(
        DEFAULT_KEYBINDINGS.refresh,
        DEFAULT_KEYBINDINGS.refresh.desc
      )
    );
    assert_some_eq_x!(
      series_details_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.edit, DEFAULT_KEYBINDINGS.edit.desc)
    );
    assert_some_eq_x!(
      series_details_context_clues_iter.next(),
      &(
        DEFAULT_KEYBINDINGS.toggle_monitoring,
        DEFAULT_KEYBINDINGS.toggle_monitoring.desc
      )
    );
    assert_some_eq_x!(
      series_details_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.submit, "season details")
    );
    assert_some_eq_x!(
      series_details_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.search, DEFAULT_KEYBINDINGS.search.desc)
    );
    assert_some_eq_x!(
      series_details_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.update, DEFAULT_KEYBINDINGS.update.desc)
    );
    assert_some_eq_x!(
      series_details_context_clues_iter.next(),
      &(
        DEFAULT_KEYBINDINGS.auto_search,
        DEFAULT_KEYBINDINGS.auto_search.desc
      )
    );
    assert_some_eq_x!(
      series_details_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.esc, DEFAULT_KEYBINDINGS.esc.desc)
    );
    assert_none!(series_details_context_clues_iter.next());
  }

  #[test]
  fn test_season_details_context_clues() {
    let mut season_details_context_clues_iter = SEASON_DETAILS_CONTEXT_CLUES.iter();

    assert_some_eq_x!(
      season_details_context_clues_iter.next(),
      &(
        DEFAULT_KEYBINDINGS.refresh,
        DEFAULT_KEYBINDINGS.refresh.desc
      )
    );
    assert_some_eq_x!(
      season_details_context_clues_iter.next(),
      &(
        DEFAULT_KEYBINDINGS.toggle_monitoring,
        DEFAULT_KEYBINDINGS.toggle_monitoring.desc
      )
    );
    assert_some_eq_x!(
      season_details_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.search, DEFAULT_KEYBINDINGS.search.desc)
    );
    assert_some_eq_x!(
      season_details_context_clues_iter.next(),
      &(
        DEFAULT_KEYBINDINGS.auto_search,
        DEFAULT_KEYBINDINGS.auto_search.desc
      )
    );
    assert_some_eq_x!(
      season_details_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.esc, DEFAULT_KEYBINDINGS.esc.desc)
    );
    assert_some_eq_x!(
      season_details_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.submit, "episode details")
    );
    assert_some_eq_x!(
      season_details_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.delete, "delete episode")
    );
    assert_none!(season_details_context_clues_iter.next());
  }

  #[test]
  fn test_season_history_context_clues() {
    let mut season_history_context_clues_iter = SEASON_HISTORY_CONTEXT_CLUES.iter();
    assert_some_eq_x!(
      season_history_context_clues_iter.next(),
      &(
        DEFAULT_KEYBINDINGS.refresh,
        DEFAULT_KEYBINDINGS.refresh.desc
      )
    );
    assert_some_eq_x!(
      season_history_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.sort, DEFAULT_KEYBINDINGS.sort.desc)
    );
    assert_some_eq_x!(
      season_history_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.search, DEFAULT_KEYBINDINGS.search.desc)
    );
    assert_some_eq_x!(
      season_history_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.filter, DEFAULT_KEYBINDINGS.filter.desc)
    );
    assert_some_eq_x!(
      season_history_context_clues_iter.next(),
      &(
        DEFAULT_KEYBINDINGS.auto_search,
        DEFAULT_KEYBINDINGS.auto_search.desc
      )
    );
    assert_some_eq_x!(
      season_history_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.submit, "details")
    );
    assert_some_eq_x!(
      season_history_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.esc, "cancel filter/close")
    );
    assert_none!(season_history_context_clues_iter.next());
  }

  #[test]
  fn test_manual_season_search_context_clues() {
    let mut manual_season_search_context_clues_iter = MANUAL_SEASON_SEARCH_CONTEXT_CLUES.iter();

    assert_some_eq_x!(
      manual_season_search_context_clues_iter.next(),
      &(
        DEFAULT_KEYBINDINGS.refresh,
        DEFAULT_KEYBINDINGS.refresh.desc
      )
    );
    assert_some_eq_x!(
      manual_season_search_context_clues_iter.next(),
      &(
        DEFAULT_KEYBINDINGS.auto_search,
        DEFAULT_KEYBINDINGS.auto_search.desc
      )
    );
    assert_some_eq_x!(
      manual_season_search_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.sort, DEFAULT_KEYBINDINGS.sort.desc)
    );
    assert_some_eq_x!(
      manual_season_search_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.submit, "details")
    );
    assert_some_eq_x!(
      manual_season_search_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.esc, DEFAULT_KEYBINDINGS.esc.desc)
    );
    assert_none!(manual_season_search_context_clues_iter.next());
  }

  #[test]
  fn test_manual_episode_search_context_clues() {
    let mut manual_episode_search_context_clues_iter = MANUAL_EPISODE_SEARCH_CONTEXT_CLUES.iter();

    assert_some_eq_x!(
      manual_episode_search_context_clues_iter.next(),
      &(
        DEFAULT_KEYBINDINGS.refresh,
        DEFAULT_KEYBINDINGS.refresh.desc
      )
    );
    assert_some_eq_x!(
      manual_episode_search_context_clues_iter.next(),
      &(
        DEFAULT_KEYBINDINGS.auto_search,
        DEFAULT_KEYBINDINGS.auto_search.desc
      )
    );
    assert_some_eq_x!(
      manual_episode_search_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.sort, DEFAULT_KEYBINDINGS.sort.desc)
    );
    assert_some_eq_x!(
      manual_episode_search_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.submit, "details")
    );
    assert_some_eq_x!(
      manual_episode_search_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.esc, DEFAULT_KEYBINDINGS.esc.desc)
    );
    assert_none!(manual_episode_search_context_clues_iter.next());
  }

  #[test]
  fn test_episode_details_context_clues() {
    let mut episode_details_context_clues_iter = EPISODE_DETAILS_CONTEXT_CLUES.iter();

    assert_some_eq_x!(
      episode_details_context_clues_iter.next(),
      &(
        DEFAULT_KEYBINDINGS.refresh,
        DEFAULT_KEYBINDINGS.refresh.desc
      )
    );
    assert_some_eq_x!(
      episode_details_context_clues_iter.next(),
      &(
        DEFAULT_KEYBINDINGS.auto_search,
        DEFAULT_KEYBINDINGS.auto_search.desc
      )
    );
    assert_some_eq_x!(
      episode_details_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.esc, DEFAULT_KEYBINDINGS.esc.desc)
    );
    assert_none!(episode_details_context_clues_iter.next());
  }

  #[test]
  fn test_selectable_episode_details_context_clues() {
    let mut episode_details_context_clues_iter = SELECTABLE_EPISODE_DETAILS_CONTEXT_CLUES.iter();

    assert_some_eq_x!(
      episode_details_context_clues_iter.next(),
      &(
        DEFAULT_KEYBINDINGS.refresh,
        DEFAULT_KEYBINDINGS.refresh.desc
      )
    );
    assert_some_eq_x!(
      episode_details_context_clues_iter.next(),
      &(
        DEFAULT_KEYBINDINGS.auto_search,
        DEFAULT_KEYBINDINGS.auto_search.desc
      )
    );
    assert_some_eq_x!(
      episode_details_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.submit, "details")
    );
    assert_some_eq_x!(
      episode_details_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.esc, DEFAULT_KEYBINDINGS.esc.desc)
    );
    assert_none!(episode_details_context_clues_iter.next());
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
    assert_none!(system_tasks_context_clues_iter.next());
  }

  #[test]
  #[should_panic(
    expected = "SonarrContextClueProvider::get_context_clues called with non-Sonarr route"
  )]
  fn test_sonarr_context_clue_provider_get_context_clues_non_sonarr_route() {
    let mut app = App::test_default();
    app.push_navigation_stack(ActiveRadarrBlock::default().into());

    SonarrContextClueProvider::get_context_clues(&mut app);
  }

  #[rstest]
  #[case(0, ActiveSonarrBlock::SeriesDetails, &SERIES_DETAILS_CONTEXT_CLUES)]
  #[case(1, ActiveSonarrBlock::SeriesHistory, &SERIES_HISTORY_CONTEXT_CLUES)]
  fn test_sonarr_context_clue_provider_series_info_tabs(
    #[case] index: usize,
    #[case] active_sonarr_block: ActiveSonarrBlock,
    #[case] expected_context_clues: &[ContextClue],
  ) {
    let mut app = App::test_default();
    app.data.sonarr_data = SonarrData::default();
    app.data.sonarr_data.series_info_tabs.set_index(index);
    app.push_navigation_stack(active_sonarr_block.into());

    let context_clues = SonarrContextClueProvider::get_context_clues(&mut app);

    assert_some_eq_x!(context_clues, expected_context_clues);
  }

  #[rstest]
  #[case(0, ActiveSonarrBlock::SeasonDetails, &SEASON_DETAILS_CONTEXT_CLUES)]
  #[case(1, ActiveSonarrBlock::SeasonHistory, &SEASON_HISTORY_CONTEXT_CLUES)]
  #[case(2, ActiveSonarrBlock::ManualSeasonSearch, &MANUAL_SEASON_SEARCH_CONTEXT_CLUES)]
  fn test_sonarr_context_clue_provider_season_details_tabs(
    #[case] index: usize,
    #[case] active_sonarr_block: ActiveSonarrBlock,
    #[case] expected_context_clues: &[ContextClue],
  ) {
    let mut app = App::test_default();
    let mut season_details_modal = SeasonDetailsModal::default();
    season_details_modal.season_details_tabs.set_index(index);
    let sonarr_data = SonarrData {
      season_details_modal: Some(season_details_modal),
      ..SonarrData::default()
    };
    app.data.sonarr_data = sonarr_data;
    app.push_navigation_stack(active_sonarr_block.into());

    let context_clues = SonarrContextClueProvider::get_context_clues(&mut app);

    assert_some_eq_x!(context_clues, expected_context_clues);
  }

  #[rstest]
  #[case(0, ActiveSonarrBlock::EpisodeDetails, &EPISODE_DETAILS_CONTEXT_CLUES)]
  #[case(1, ActiveSonarrBlock::EpisodeHistory, &SELECTABLE_EPISODE_DETAILS_CONTEXT_CLUES)]
  #[case(2, ActiveSonarrBlock::EpisodeFile, &EPISODE_DETAILS_CONTEXT_CLUES)]
  #[case(3, ActiveSonarrBlock::ManualEpisodeSearch, &MANUAL_EPISODE_SEARCH_CONTEXT_CLUES)]
  fn test_sonarr_context_clue_provider_episode_details_tabs(
    #[case] index: usize,
    #[case] active_sonarr_block: ActiveSonarrBlock,
    #[case] expected_context_clues: &[ContextClue],
  ) {
    let mut app = App::test_default();
    let mut episode_details_modal = EpisodeDetailsModal::default();
    episode_details_modal.episode_details_tabs.set_index(index);
    let sonarr_data = SonarrData {
      season_details_modal: Some(SeasonDetailsModal {
        episode_details_modal: Some(episode_details_modal),
        ..SeasonDetailsModal::default()
      }),
      ..SonarrData::default()
    };
    app.data.sonarr_data = sonarr_data;
    app.push_navigation_stack(active_sonarr_block.into());

    let context_clues = SonarrContextClueProvider::get_context_clues(&mut app);

    assert_some_eq_x!(context_clues, expected_context_clues);
  }

  #[rstest]
  fn test_sonarr_context_clue_provider_bare_popup_context_clues(
    #[values(
      ActiveSonarrBlock::TestAllIndexers,
      ActiveSonarrBlock::AddSeriesSearchInput,
      ActiveSonarrBlock::AddSeriesEmptySearchResults,
      ActiveSonarrBlock::SystemLogs,
      ActiveSonarrBlock::SystemUpdates
    )]
    active_sonarr_block: ActiveSonarrBlock,
  ) {
    let mut app = App::test_default();
    app.push_navigation_stack(active_sonarr_block.into());

    let context_clues = SonarrContextClueProvider::get_context_clues(&mut app);

    assert_some_eq_x!(context_clues, &BARE_POPUP_CONTEXT_CLUES);
  }

  #[rstest]
  fn test_sonarr_context_clue_provider_confirmation_prompt_context_clues(
    #[values(
      ActiveSonarrBlock::AddSeriesPrompt,
      ActiveSonarrBlock::AddSeriesSelectMonitor,
      ActiveSonarrBlock::AddSeriesSelectSeriesType,
      ActiveSonarrBlock::AddSeriesSelectQualityProfile,
      ActiveSonarrBlock::AddSeriesSelectLanguageProfile,
      ActiveSonarrBlock::AddSeriesSelectRootFolder,
      ActiveSonarrBlock::AddSeriesTagsInput,
      ActiveSonarrBlock::SystemTaskStartConfirmPrompt
    )]
    active_sonarr_block: ActiveSonarrBlock,
  ) {
    let mut app = App::test_default();
    app.push_navigation_stack(active_sonarr_block.into());

    let context_clues = SonarrContextClueProvider::get_context_clues(&mut app);

    assert_some_eq_x!(context_clues, &CONFIRMATION_PROMPT_CONTEXT_CLUES);
  }

  #[rstest]
  fn test_sonarr_context_clue_provider_confirmation_prompt_popup_clues_edit_indexer_blocks(
    #[values(
      ActiveSonarrBlock::EditIndexerPrompt,
      ActiveSonarrBlock::EditIndexerConfirmPrompt,
      ActiveSonarrBlock::EditIndexerApiKeyInput,
      ActiveSonarrBlock::EditIndexerNameInput,
      ActiveSonarrBlock::EditIndexerSeedRatioInput,
      ActiveSonarrBlock::EditIndexerToggleEnableRss,
      ActiveSonarrBlock::EditIndexerToggleEnableAutomaticSearch,
      ActiveSonarrBlock::EditIndexerToggleEnableInteractiveSearch,
      ActiveSonarrBlock::EditIndexerPriorityInput,
      ActiveSonarrBlock::EditIndexerUrlInput,
      ActiveSonarrBlock::EditIndexerTagsInput
    )]
    active_sonarr_block: ActiveSonarrBlock,
  ) {
    let mut app = App::test_default();
    app.push_navigation_stack(active_sonarr_block.into());

    let context_clues = SonarrContextClueProvider::get_context_clues(&mut app);

    assert_some_eq_x!(context_clues, &CONFIRMATION_PROMPT_CONTEXT_CLUES);
  }

  #[rstest]
  fn test_sonarr_context_clue_provider_confirmation_prompt_popup_clues_indexer_settings_blocks(
    #[values(
      ActiveSonarrBlock::AllIndexerSettingsPrompt,
      ActiveSonarrBlock::IndexerSettingsConfirmPrompt,
      ActiveSonarrBlock::IndexerSettingsMaximumSizeInput,
      ActiveSonarrBlock::IndexerSettingsMinimumAgeInput,
      ActiveSonarrBlock::IndexerSettingsRetentionInput,
      ActiveSonarrBlock::IndexerSettingsRssSyncIntervalInput
    )]
    active_sonarr_block: ActiveSonarrBlock,
  ) {
    let mut app = App::test_default();
    app.push_navigation_stack(active_sonarr_block.into());

    let context_clues = SonarrContextClueProvider::get_context_clues(&mut app);

    assert_some_eq_x!(context_clues, &CONFIRMATION_PROMPT_CONTEXT_CLUES);
  }

  #[rstest]
  fn test_sonarr_context_clue_provider_confirmation_prompt_popup_clues_edit_series_blocks(
    #[values(
      ActiveSonarrBlock::EditSeriesPrompt,
      ActiveSonarrBlock::EditSeriesConfirmPrompt,
      ActiveSonarrBlock::EditSeriesPathInput,
      ActiveSonarrBlock::EditSeriesSelectSeriesType,
      ActiveSonarrBlock::EditSeriesSelectQualityProfile,
      ActiveSonarrBlock::EditSeriesSelectLanguageProfile,
      ActiveSonarrBlock::EditSeriesTagsInput,
      ActiveSonarrBlock::EditSeriesToggleMonitored,
      ActiveSonarrBlock::EditSeriesToggleSeasonFolder
    )]
    active_sonarr_block: ActiveSonarrBlock,
  ) {
    let mut app = App::test_default();
    app.push_navigation_stack(active_sonarr_block.into());

    let context_clues = SonarrContextClueProvider::get_context_clues(&mut app);

    assert_some_eq_x!(context_clues, &CONFIRMATION_PROMPT_CONTEXT_CLUES);
  }

  #[rstest]
  fn test_sonarr_context_clue_provider_add_series_search_results_clues(
    #[values(
      ActiveSonarrBlock::AddSeriesAlreadyInLibrary,
      ActiveSonarrBlock::AddSeriesSearchResults
    )]
    active_sonarr_block: ActiveSonarrBlock,
  ) {
    let mut app = App::test_default();
    app.push_navigation_stack(active_sonarr_block.into());

    let context_clues = SonarrContextClueProvider::get_context_clues(&mut app);

    assert_some_eq_x!(context_clues, &ADD_SERIES_SEARCH_RESULTS_CONTEXT_CLUES);
  }

  #[test]
  fn test_sonarr_context_clue_provider_system_tasks_clues() {
    let mut app = App::test_default();

    app.push_navigation_stack(ActiveSonarrBlock::SystemTasks.into());
    let context_clues = SonarrContextClueProvider::get_context_clues(&mut app);

    assert_some_eq_x!(context_clues, &SYSTEM_TASKS_CONTEXT_CLUES);
  }

  #[rstest]
  #[case(0, ActiveSonarrBlock::Series, &SERIES_CONTEXT_CLUES)]
  #[case(1, ActiveSonarrBlock::Downloads, &DOWNLOADS_CONTEXT_CLUES)]
  #[case(2, ActiveSonarrBlock::Blocklist, &BLOCKLIST_CONTEXT_CLUES)]
  #[case(3, ActiveSonarrBlock::History, &HISTORY_CONTEXT_CLUES)]
  #[case(4, ActiveSonarrBlock::RootFolders, &ROOT_FOLDERS_CONTEXT_CLUES)]
  #[case(5, ActiveSonarrBlock::Indexers, &INDEXERS_CONTEXT_CLUES)]
  #[case(6, ActiveSonarrBlock::System, &SYSTEM_CONTEXT_CLUES)]
  fn test_sonarr_context_clue_provider_sonarr_tabs(
    #[case] index: usize,
    #[case] active_sonarr_block: ActiveSonarrBlock,
    #[case] expected_context_clues: &[ContextClue],
  ) {
    let mut app = App::test_default();
    app.data.sonarr_data = SonarrData::default();
    app.data.sonarr_data.main_tabs.set_index(index);
    app.push_navigation_stack(active_sonarr_block.into());

    let context_clues = SonarrContextClueProvider::get_context_clues(&mut app);

    assert_some_eq_x!(context_clues, expected_context_clues);
  }
}
