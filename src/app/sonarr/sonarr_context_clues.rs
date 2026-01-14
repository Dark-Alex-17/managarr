use crate::app::context_clues::{
  BARE_POPUP_CONTEXT_CLUES, CONFIRMATION_PROMPT_CONTEXT_CLUES, ContextClueProvider,
  SYSTEM_TASKS_CONTEXT_CLUES,
};
use crate::app::{App, context_clues::ContextClue, key_binding::DEFAULT_KEYBINDINGS};
use crate::models::Route;
use crate::models::servarr_data::sonarr::sonarr_data::{
  ADD_SERIES_BLOCKS, ActiveSonarrBlock, EDIT_INDEXER_BLOCKS, EDIT_SERIES_BLOCKS,
  EPISODE_DETAILS_BLOCKS, INDEXER_SETTINGS_BLOCKS, SEASON_DETAILS_BLOCKS, SERIES_DETAILS_BLOCKS,
};

#[cfg(test)]
#[path = "sonarr_context_clues_tests.rs"]
mod sonarr_context_clues_tests;

pub static ADD_SERIES_SEARCH_RESULTS_CONTEXT_CLUES: [ContextClue; 2] = [
  (DEFAULT_KEYBINDINGS.submit, "details"),
  (DEFAULT_KEYBINDINGS.esc, "edit search"),
];

pub static SERIES_CONTEXT_CLUES: [ContextClue; 11] = [
  (DEFAULT_KEYBINDINGS.add, DEFAULT_KEYBINDINGS.add.desc),
  (DEFAULT_KEYBINDINGS.edit, DEFAULT_KEYBINDINGS.edit.desc),
  (
    DEFAULT_KEYBINDINGS.toggle_monitoring,
    DEFAULT_KEYBINDINGS.toggle_monitoring.desc,
  ),
  (DEFAULT_KEYBINDINGS.sort, DEFAULT_KEYBINDINGS.sort.desc),
  (DEFAULT_KEYBINDINGS.delete, DEFAULT_KEYBINDINGS.delete.desc),
  (DEFAULT_KEYBINDINGS.search, DEFAULT_KEYBINDINGS.search.desc),
  (DEFAULT_KEYBINDINGS.filter, DEFAULT_KEYBINDINGS.filter.desc),
  (
    DEFAULT_KEYBINDINGS.refresh,
    DEFAULT_KEYBINDINGS.refresh.desc,
  ),
  (DEFAULT_KEYBINDINGS.update, "update all"),
  (DEFAULT_KEYBINDINGS.submit, "details"),
  (DEFAULT_KEYBINDINGS.esc, "cancel filter"),
];

pub static SERIES_DETAILS_CONTEXT_CLUES: [ContextClue; 8] = [
  (
    DEFAULT_KEYBINDINGS.refresh,
    DEFAULT_KEYBINDINGS.refresh.desc,
  ),
  (DEFAULT_KEYBINDINGS.edit, DEFAULT_KEYBINDINGS.edit.desc),
  (
    DEFAULT_KEYBINDINGS.toggle_monitoring,
    DEFAULT_KEYBINDINGS.toggle_monitoring.desc,
  ),
  (DEFAULT_KEYBINDINGS.submit, "season details"),
  (DEFAULT_KEYBINDINGS.search, DEFAULT_KEYBINDINGS.search.desc),
  (DEFAULT_KEYBINDINGS.update, DEFAULT_KEYBINDINGS.update.desc),
  (
    DEFAULT_KEYBINDINGS.auto_search,
    DEFAULT_KEYBINDINGS.auto_search.desc,
  ),
  (DEFAULT_KEYBINDINGS.esc, DEFAULT_KEYBINDINGS.esc.desc),
];

pub static SERIES_HISTORY_CONTEXT_CLUES: [ContextClue; 9] = [
  (
    DEFAULT_KEYBINDINGS.refresh,
    DEFAULT_KEYBINDINGS.refresh.desc,
  ),
  (DEFAULT_KEYBINDINGS.edit, DEFAULT_KEYBINDINGS.edit.desc),
  (DEFAULT_KEYBINDINGS.submit, "details"),
  (DEFAULT_KEYBINDINGS.sort, DEFAULT_KEYBINDINGS.sort.desc),
  (DEFAULT_KEYBINDINGS.search, DEFAULT_KEYBINDINGS.search.desc),
  (DEFAULT_KEYBINDINGS.filter, DEFAULT_KEYBINDINGS.filter.desc),
  (
    DEFAULT_KEYBINDINGS.auto_search,
    DEFAULT_KEYBINDINGS.auto_search.desc,
  ),
  (DEFAULT_KEYBINDINGS.update, DEFAULT_KEYBINDINGS.update.desc),
  (DEFAULT_KEYBINDINGS.esc, "cancel filter/close"),
];

pub static SEASON_DETAILS_CONTEXT_CLUES: [ContextClue; 7] = [
  (
    DEFAULT_KEYBINDINGS.refresh,
    DEFAULT_KEYBINDINGS.refresh.desc,
  ),
  (
    DEFAULT_KEYBINDINGS.toggle_monitoring,
    DEFAULT_KEYBINDINGS.toggle_monitoring.desc,
  ),
  (DEFAULT_KEYBINDINGS.search, DEFAULT_KEYBINDINGS.search.desc),
  (
    DEFAULT_KEYBINDINGS.auto_search,
    DEFAULT_KEYBINDINGS.auto_search.desc,
  ),
  (DEFAULT_KEYBINDINGS.esc, DEFAULT_KEYBINDINGS.esc.desc),
  (DEFAULT_KEYBINDINGS.submit, "episode details"),
  (DEFAULT_KEYBINDINGS.delete, "delete episode"),
];

pub static SEASON_HISTORY_CONTEXT_CLUES: [ContextClue; 7] = [
  (
    DEFAULT_KEYBINDINGS.refresh,
    DEFAULT_KEYBINDINGS.refresh.desc,
  ),
  (DEFAULT_KEYBINDINGS.sort, DEFAULT_KEYBINDINGS.sort.desc),
  (DEFAULT_KEYBINDINGS.search, DEFAULT_KEYBINDINGS.search.desc),
  (DEFAULT_KEYBINDINGS.filter, DEFAULT_KEYBINDINGS.filter.desc),
  (
    DEFAULT_KEYBINDINGS.auto_search,
    DEFAULT_KEYBINDINGS.auto_search.desc,
  ),
  (DEFAULT_KEYBINDINGS.submit, "details"),
  (DEFAULT_KEYBINDINGS.esc, "cancel filter/close"),
];

pub static MANUAL_SEASON_SEARCH_CONTEXT_CLUES: [ContextClue; 5] = [
  (
    DEFAULT_KEYBINDINGS.refresh,
    DEFAULT_KEYBINDINGS.refresh.desc,
  ),
  (
    DEFAULT_KEYBINDINGS.auto_search,
    DEFAULT_KEYBINDINGS.auto_search.desc,
  ),
  (DEFAULT_KEYBINDINGS.sort, DEFAULT_KEYBINDINGS.sort.desc),
  (DEFAULT_KEYBINDINGS.submit, "details"),
  (DEFAULT_KEYBINDINGS.esc, DEFAULT_KEYBINDINGS.esc.desc),
];

pub static MANUAL_EPISODE_SEARCH_CONTEXT_CLUES: [ContextClue; 5] = [
  (
    DEFAULT_KEYBINDINGS.refresh,
    DEFAULT_KEYBINDINGS.refresh.desc,
  ),
  (
    DEFAULT_KEYBINDINGS.auto_search,
    DEFAULT_KEYBINDINGS.auto_search.desc,
  ),
  (DEFAULT_KEYBINDINGS.sort, DEFAULT_KEYBINDINGS.sort.desc),
  (DEFAULT_KEYBINDINGS.submit, "details"),
  (DEFAULT_KEYBINDINGS.esc, DEFAULT_KEYBINDINGS.esc.desc),
];

pub static EPISODE_DETAILS_CONTEXT_CLUES: [ContextClue; 3] = [
  (
    DEFAULT_KEYBINDINGS.refresh,
    DEFAULT_KEYBINDINGS.refresh.desc,
  ),
  (
    DEFAULT_KEYBINDINGS.auto_search,
    DEFAULT_KEYBINDINGS.auto_search.desc,
  ),
  (DEFAULT_KEYBINDINGS.esc, DEFAULT_KEYBINDINGS.esc.desc),
];

pub static SELECTABLE_EPISODE_DETAILS_CONTEXT_CLUES: [ContextClue; 4] = [
  (
    DEFAULT_KEYBINDINGS.refresh,
    DEFAULT_KEYBINDINGS.refresh.desc,
  ),
  (
    DEFAULT_KEYBINDINGS.auto_search,
    DEFAULT_KEYBINDINGS.auto_search.desc,
  ),
  (DEFAULT_KEYBINDINGS.submit, "details"),
  (DEFAULT_KEYBINDINGS.esc, DEFAULT_KEYBINDINGS.esc.desc),
];

pub(in crate::app) struct SonarrContextClueProvider;

impl ContextClueProvider for SonarrContextClueProvider {
  fn get_context_clues(app: &mut App<'_>) -> Option<&'static [ContextClue]> {
    let Route::Sonarr(active_sonarr_block, _) = app.get_current_route() else {
      panic!("SonarrContextClueProvider::get_context_clues called with non-Sonarr route");
    };
    match active_sonarr_block {
      _ if SERIES_DETAILS_BLOCKS.contains(&active_sonarr_block) => app
        .data
        .sonarr_data
        .series_info_tabs
        .get_active_route_contextual_help(),
      _ if SEASON_DETAILS_BLOCKS.contains(&active_sonarr_block) => app
        .data
        .sonarr_data
        .season_details_modal
        .as_ref()
        .unwrap()
        .season_details_tabs
        .get_active_route_contextual_help(),
      _ if EPISODE_DETAILS_BLOCKS.contains(&active_sonarr_block) => app
        .data
        .sonarr_data
        .season_details_modal
        .as_ref()
        .unwrap()
        .episode_details_modal
        .as_ref()
        .unwrap()
        .episode_details_tabs
        .get_active_route_contextual_help(),
      ActiveSonarrBlock::TestAllIndexers
      | ActiveSonarrBlock::AddSeriesSearchInput
      | ActiveSonarrBlock::AddSeriesEmptySearchResults
      | ActiveSonarrBlock::SystemLogs
      | ActiveSonarrBlock::SystemUpdates => Some(&BARE_POPUP_CONTEXT_CLUES),
      _ if EDIT_INDEXER_BLOCKS.contains(&active_sonarr_block)
        || INDEXER_SETTINGS_BLOCKS.contains(&active_sonarr_block)
        || EDIT_SERIES_BLOCKS.contains(&active_sonarr_block) =>
      {
        Some(&CONFIRMATION_PROMPT_CONTEXT_CLUES)
      }
      ActiveSonarrBlock::AddSeriesPrompt
      | ActiveSonarrBlock::AddSeriesSelectMonitor
      | ActiveSonarrBlock::AddSeriesSelectSeriesType
      | ActiveSonarrBlock::AddSeriesSelectQualityProfile
      | ActiveSonarrBlock::AddSeriesSelectLanguageProfile
      | ActiveSonarrBlock::AddSeriesSelectRootFolder
      | ActiveSonarrBlock::AddSeriesTagsInput
      | ActiveSonarrBlock::SystemTaskStartConfirmPrompt => Some(&CONFIRMATION_PROMPT_CONTEXT_CLUES),
      _ if ADD_SERIES_BLOCKS.contains(&active_sonarr_block) => {
        Some(&ADD_SERIES_SEARCH_RESULTS_CONTEXT_CLUES)
      }
      ActiveSonarrBlock::SystemTasks => Some(&SYSTEM_TASKS_CONTEXT_CLUES),
      _ => app
        .data
        .sonarr_data
        .main_tabs
        .get_active_route_contextual_help(),
    }
  }
}
