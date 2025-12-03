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
  use pretty_assertions::{assert_eq, assert_str_eq};
  use rstest::rstest;

  #[test]
  fn test_add_series_search_results_context_clues() {
    let mut add_series_search_results_context_clues_iter =
      ADD_SERIES_SEARCH_RESULTS_CONTEXT_CLUES.iter();

    let (key_binding, description) = add_series_search_results_context_clues_iter.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.submit);
    assert_str_eq!(*description, "details");

    let (key_binding, description) = add_series_search_results_context_clues_iter.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.esc);
    assert_str_eq!(*description, "edit search");
    assert_eq!(add_series_search_results_context_clues_iter.next(), None);
  }

  #[test]
  fn test_series_context_clues() {
    let mut series_context_clues_iter = SERIES_CONTEXT_CLUES.iter();

    let (key_binding, description) = series_context_clues_iter.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.add);
    assert_str_eq!(*description, DEFAULT_KEYBINDINGS.add.desc);

    let (key_binding, description) = series_context_clues_iter.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.edit);
    assert_str_eq!(*description, DEFAULT_KEYBINDINGS.edit.desc);

    let (key_binding, description) = series_context_clues_iter.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.toggle_monitoring);
    assert_str_eq!(*description, DEFAULT_KEYBINDINGS.toggle_monitoring.desc);

    let (key_binding, description) = series_context_clues_iter.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.sort);
    assert_str_eq!(*description, DEFAULT_KEYBINDINGS.sort.desc);

    let (key_binding, description) = series_context_clues_iter.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.delete);
    assert_str_eq!(*description, DEFAULT_KEYBINDINGS.delete.desc);

    let (key_binding, description) = series_context_clues_iter.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.search);
    assert_str_eq!(*description, DEFAULT_KEYBINDINGS.search.desc);

    let (key_binding, description) = series_context_clues_iter.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.filter);
    assert_str_eq!(*description, DEFAULT_KEYBINDINGS.filter.desc);

    let (key_binding, description) = series_context_clues_iter.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.refresh);
    assert_str_eq!(*description, DEFAULT_KEYBINDINGS.refresh.desc);

    let (key_binding, description) = series_context_clues_iter.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.update);
    assert_str_eq!(*description, "update all");

    let (key_binding, description) = series_context_clues_iter.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.submit);
    assert_str_eq!(*description, "details");

    let (key_binding, description) = series_context_clues_iter.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.esc);
    assert_str_eq!(*description, "cancel filter");
    assert_eq!(series_context_clues_iter.next(), None);
  }

  #[test]
  fn test_series_history_context_clues() {
    let mut series_history_context_clues_iter = SERIES_HISTORY_CONTEXT_CLUES.iter();

    let (key_binding, description) = series_history_context_clues_iter.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.refresh);
    assert_str_eq!(*description, DEFAULT_KEYBINDINGS.refresh.desc);

    let (key_binding, description) = series_history_context_clues_iter.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.edit);
    assert_str_eq!(*description, DEFAULT_KEYBINDINGS.edit.desc);

    let (key_binding, description) = series_history_context_clues_iter.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.submit);
    assert_str_eq!(*description, "details");

    let (key_binding, description) = series_history_context_clues_iter.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.sort);
    assert_str_eq!(*description, DEFAULT_KEYBINDINGS.sort.desc);

    let (key_binding, description) = series_history_context_clues_iter.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.search);
    assert_str_eq!(*description, DEFAULT_KEYBINDINGS.search.desc);

    let (key_binding, description) = series_history_context_clues_iter.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.filter);
    assert_str_eq!(*description, DEFAULT_KEYBINDINGS.filter.desc);

    let (key_binding, description) = series_history_context_clues_iter.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.auto_search);
    assert_str_eq!(*description, DEFAULT_KEYBINDINGS.auto_search.desc);

    let (key_binding, description) = series_history_context_clues_iter.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.update);
    assert_str_eq!(*description, DEFAULT_KEYBINDINGS.update.desc);

    let (key_binding, description) = series_history_context_clues_iter.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.esc);
    assert_str_eq!(*description, "cancel filter/close");
    assert_eq!(series_history_context_clues_iter.next(), None);
  }

  #[test]
  fn test_history_context_clues() {
    let mut history_context_clues_iter = HISTORY_CONTEXT_CLUES.iter();

    let (key_binding, description) = history_context_clues_iter.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.submit);
    assert_str_eq!(*description, "details");

    let (key_binding, description) = history_context_clues_iter.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.sort);
    assert_str_eq!(*description, DEFAULT_KEYBINDINGS.sort.desc);

    let (key_binding, description) = history_context_clues_iter.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.search);
    assert_str_eq!(*description, DEFAULT_KEYBINDINGS.search.desc);

    let (key_binding, description) = history_context_clues_iter.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.filter);
    assert_str_eq!(*description, DEFAULT_KEYBINDINGS.filter.desc);

    let (key_binding, description) = history_context_clues_iter.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.refresh);
    assert_str_eq!(*description, DEFAULT_KEYBINDINGS.refresh.desc);

    let (key_binding, description) = history_context_clues_iter.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.esc);
    assert_str_eq!(*description, "cancel filter");
    assert_eq!(history_context_clues_iter.next(), None);
  }

  #[test]
  fn test_series_details_context_clues() {
    let mut series_details_context_clues_iter = SERIES_DETAILS_CONTEXT_CLUES.iter();

    let (key_binding, description) = series_details_context_clues_iter.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.refresh);
    assert_str_eq!(*description, DEFAULT_KEYBINDINGS.refresh.desc);

    let (key_binding, description) = series_details_context_clues_iter.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.edit);
    assert_str_eq!(*description, DEFAULT_KEYBINDINGS.edit.desc);

    let (key_binding, description) = series_details_context_clues_iter.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.toggle_monitoring);
    assert_str_eq!(*description, DEFAULT_KEYBINDINGS.toggle_monitoring.desc);

    let (key_binding, description) = series_details_context_clues_iter.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.submit);
    assert_str_eq!(*description, "season details");

    let (key_binding, description) = series_details_context_clues_iter.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.search);
    assert_str_eq!(*description, DEFAULT_KEYBINDINGS.search.desc);

    let (key_binding, description) = series_details_context_clues_iter.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.update);
    assert_str_eq!(*description, DEFAULT_KEYBINDINGS.update.desc);

    let (key_binding, description) = series_details_context_clues_iter.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.auto_search);
    assert_str_eq!(*description, DEFAULT_KEYBINDINGS.auto_search.desc);

    let (key_binding, description) = series_details_context_clues_iter.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.esc);
    assert_str_eq!(*description, DEFAULT_KEYBINDINGS.esc.desc);
    assert_eq!(series_details_context_clues_iter.next(), None);
  }

  #[test]
  fn test_season_details_context_clues() {
    let mut season_details_context_clues_iter = SEASON_DETAILS_CONTEXT_CLUES.iter();

    let (key_binding, description) = season_details_context_clues_iter.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.refresh);
    assert_str_eq!(*description, DEFAULT_KEYBINDINGS.refresh.desc);

    let (key_binding, description) = season_details_context_clues_iter.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.toggle_monitoring);
    assert_str_eq!(*description, DEFAULT_KEYBINDINGS.toggle_monitoring.desc);

    let (key_binding, description) = season_details_context_clues_iter.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.search);
    assert_str_eq!(*description, DEFAULT_KEYBINDINGS.search.desc);

    let (key_binding, description) = season_details_context_clues_iter.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.auto_search);
    assert_str_eq!(*description, DEFAULT_KEYBINDINGS.auto_search.desc);

    let (key_binding, description) = season_details_context_clues_iter.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.esc);
    assert_str_eq!(*description, DEFAULT_KEYBINDINGS.esc.desc);

    let (key_binding, description) = season_details_context_clues_iter.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.submit);
    assert_str_eq!(*description, "episode details");

    let (key_binding, description) = season_details_context_clues_iter.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.delete);
    assert_str_eq!(*description, "delete episode");

    assert_eq!(season_details_context_clues_iter.next(), None);
  }

  #[test]
  fn test_season_history_context_clues() {
    let mut season_history_context_clues_iter = SEASON_HISTORY_CONTEXT_CLUES.iter();
    let (key_binding, description) = season_history_context_clues_iter.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.refresh);
    assert_str_eq!(*description, DEFAULT_KEYBINDINGS.refresh.desc);

    let (key_binding, description) = season_history_context_clues_iter.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.sort);
    assert_str_eq!(*description, DEFAULT_KEYBINDINGS.sort.desc);

    let (key_binding, description) = season_history_context_clues_iter.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.search);
    assert_str_eq!(*description, DEFAULT_KEYBINDINGS.search.desc);

    let (key_binding, description) = season_history_context_clues_iter.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.filter);
    assert_str_eq!(*description, DEFAULT_KEYBINDINGS.filter.desc);

    let (key_binding, description) = season_history_context_clues_iter.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.auto_search);
    assert_str_eq!(*description, DEFAULT_KEYBINDINGS.auto_search.desc);

    let (key_binding, description) = season_history_context_clues_iter.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.submit);
    assert_str_eq!(*description, "details");

    let (key_binding, description) = season_history_context_clues_iter.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.esc);
    assert_str_eq!(*description, "cancel filter/close");
    assert_eq!(season_history_context_clues_iter.next(), None);
  }

  #[test]
  fn test_manual_season_search_context_clues() {
    let mut manual_season_search_context_clues_iter = MANUAL_SEASON_SEARCH_CONTEXT_CLUES.iter();

    let (key_binding, description) = manual_season_search_context_clues_iter.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.refresh);
    assert_str_eq!(*description, DEFAULT_KEYBINDINGS.refresh.desc);

    let (key_binding, description) = manual_season_search_context_clues_iter.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.auto_search);
    assert_str_eq!(*description, DEFAULT_KEYBINDINGS.auto_search.desc);

    let (key_binding, description) = manual_season_search_context_clues_iter.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.sort);
    assert_str_eq!(*description, DEFAULT_KEYBINDINGS.sort.desc);

    let (key_binding, description) = manual_season_search_context_clues_iter.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.submit);
    assert_str_eq!(*description, "details");

    let (key_binding, description) = manual_season_search_context_clues_iter.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.esc);
    assert_str_eq!(*description, DEFAULT_KEYBINDINGS.esc.desc);
    assert_eq!(manual_season_search_context_clues_iter.next(), None);
  }

  #[test]
  fn test_manual_episode_search_context_clues() {
    let mut manual_episode_search_context_clues_iter = MANUAL_EPISODE_SEARCH_CONTEXT_CLUES.iter();

    let (key_binding, description) = manual_episode_search_context_clues_iter.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.refresh);
    assert_str_eq!(*description, DEFAULT_KEYBINDINGS.refresh.desc);

    let (key_binding, description) = manual_episode_search_context_clues_iter.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.auto_search);
    assert_str_eq!(*description, DEFAULT_KEYBINDINGS.auto_search.desc);

    let (key_binding, description) = manual_episode_search_context_clues_iter.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.sort);
    assert_str_eq!(*description, DEFAULT_KEYBINDINGS.sort.desc);

    let (key_binding, description) = manual_episode_search_context_clues_iter.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.submit);
    assert_str_eq!(*description, "details");

    let (key_binding, description) = manual_episode_search_context_clues_iter.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.esc);
    assert_str_eq!(*description, DEFAULT_KEYBINDINGS.esc.desc);
    assert_eq!(manual_episode_search_context_clues_iter.next(), None);
  }

  #[test]
  fn test_episode_details_context_clues() {
    let mut episode_details_context_clues_iter = EPISODE_DETAILS_CONTEXT_CLUES.iter();

    let (key_binding, description) = episode_details_context_clues_iter.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.refresh);
    assert_str_eq!(*description, DEFAULT_KEYBINDINGS.refresh.desc);

    let (key_binding, description) = episode_details_context_clues_iter.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.auto_search);
    assert_str_eq!(*description, DEFAULT_KEYBINDINGS.auto_search.desc);

    let (key_binding, description) = episode_details_context_clues_iter.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.esc);
    assert_str_eq!(*description, DEFAULT_KEYBINDINGS.esc.desc);
    assert_eq!(episode_details_context_clues_iter.next(), None);
  }

  #[test]
  fn test_selectable_episode_details_context_clues() {
    let mut episode_details_context_clues_iter = SELECTABLE_EPISODE_DETAILS_CONTEXT_CLUES.iter();

    let (key_binding, description) = episode_details_context_clues_iter.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.refresh);
    assert_str_eq!(*description, DEFAULT_KEYBINDINGS.refresh.desc);

    let (key_binding, description) = episode_details_context_clues_iter.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.auto_search);
    assert_str_eq!(*description, DEFAULT_KEYBINDINGS.auto_search.desc);

    let (key_binding, description) = episode_details_context_clues_iter.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.submit);
    assert_str_eq!(*description, "details");

    let (key_binding, description) = episode_details_context_clues_iter.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.esc);
    assert_str_eq!(*description, DEFAULT_KEYBINDINGS.esc.desc);
    assert_eq!(episode_details_context_clues_iter.next(), None);
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
  #[should_panic(
    expected = "SonarrContextClueProvider::get_context_clues called with non-Sonarr route"
  )]
  fn test_sonarr_context_clue_provider_get_context_clues_non_sonarr_route() {
    let mut app = App::test_default();
    app.push_navigation_stack(ActiveRadarrBlock::default().into());

    // This should panic because the route is not a Sonarr route
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

    assert!(context_clues.is_some());
    assert_eq!(expected_context_clues, context_clues.unwrap());
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

    assert!(context_clues.is_some());
    assert_eq!(expected_context_clues, context_clues.unwrap());
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

    assert!(context_clues.is_some());
    assert_eq!(expected_context_clues, context_clues.unwrap());
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

    assert!(context_clues.is_some());
    assert_eq!(context_clues.unwrap(), &BARE_POPUP_CONTEXT_CLUES);
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

    assert!(context_clues.is_some());
    assert_eq!(context_clues.unwrap(), &CONFIRMATION_PROMPT_CONTEXT_CLUES);
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

    assert!(context_clues.is_some());
    assert_eq!(context_clues.unwrap(), &CONFIRMATION_PROMPT_CONTEXT_CLUES);
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

    assert!(context_clues.is_some());
    assert_eq!(context_clues.unwrap(), &CONFIRMATION_PROMPT_CONTEXT_CLUES);
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

    assert!(context_clues.is_some());
    assert_eq!(context_clues.unwrap(), &CONFIRMATION_PROMPT_CONTEXT_CLUES);
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

    assert!(context_clues.is_some());
    assert_eq!(
      context_clues.unwrap(),
      &ADD_SERIES_SEARCH_RESULTS_CONTEXT_CLUES
    );
  }

  #[test]
  fn test_sonarr_context_clue_provider_system_tasks_clues() {
    let mut app = App::test_default();

    app.push_navigation_stack(ActiveSonarrBlock::SystemTasks.into());
    let context_clues = SonarrContextClueProvider::get_context_clues(&mut app);

    assert!(context_clues.is_some());
    assert_eq!(context_clues.unwrap(), &SYSTEM_TASKS_CONTEXT_CLUES);
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

    assert!(context_clues.is_some());
    assert_eq!(expected_context_clues, context_clues.unwrap());
  }
}
