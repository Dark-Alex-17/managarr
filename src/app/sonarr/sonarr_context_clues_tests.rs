#[cfg(test)]
mod tests {
  use pretty_assertions::{assert_eq, assert_str_eq};

  use crate::app::{
    key_binding::DEFAULT_KEYBINDINGS,
    sonarr::sonarr_context_clues::{
      ADD_SERIES_SEARCH_RESULTS_CONTEXT_CLUES, EPISODE_DETAILS_CONTEXT_CLUES,
      HISTORY_CONTEXT_CLUES, MANUAL_EPISODE_SEARCH_CONTEXTUAL_CONTEXT_CLUES,
      MANUAL_EPISODE_SEARCH_CONTEXT_CLUES, MANUAL_SEASON_SEARCH_CONTEXT_CLUES,
      SEASON_DETAILS_CONTEXT_CLUES, SERIES_CONTEXT_CLUES, SERIES_DETAILS_CONTEXT_CLUES,
      SYSTEM_TASKS_CONTEXT_CLUES,
    },
  };

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
  fn test_history_context_clues() {
    let mut history_context_clues_iter = HISTORY_CONTEXT_CLUES.iter();

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

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.update);
    assert_str_eq!(*description, DEFAULT_KEYBINDINGS.update.desc);

    let (key_binding, description) = series_details_context_clues_iter.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.submit);
    assert_str_eq!(*description, "details");

    let (key_binding, description) = series_details_context_clues_iter.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.search);
    assert_str_eq!(*description, "auto search");

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

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.submit);
    assert_str_eq!(*description, "details");

    let (key_binding, description) = season_details_context_clues_iter.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.search);
    assert_str_eq!(*description, "auto search");

    let (key_binding, description) = season_details_context_clues_iter.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.delete);
    assert_str_eq!(*description, "delete episode");

    let (key_binding, description) = season_details_context_clues_iter.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.esc);
    assert_str_eq!(*description, DEFAULT_KEYBINDINGS.esc.desc);
    assert_eq!(season_details_context_clues_iter.next(), None);
  }

  #[test]
  fn test_manual_season_search_context_clues() {
    let mut manual_season_search_context_clues_iter = MANUAL_SEASON_SEARCH_CONTEXT_CLUES.iter();

    let (key_binding, description) = manual_season_search_context_clues_iter.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.refresh);
    assert_str_eq!(*description, DEFAULT_KEYBINDINGS.refresh.desc);

    let (key_binding, description) = manual_season_search_context_clues_iter.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.search);
    assert_str_eq!(*description, "auto search");

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

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.search);
    assert_str_eq!(*description, "auto search");

    let (key_binding, description) = manual_episode_search_context_clues_iter.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.sort);
    assert_str_eq!(*description, DEFAULT_KEYBINDINGS.sort.desc);

    let (key_binding, description) = manual_episode_search_context_clues_iter.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.esc);
    assert_str_eq!(*description, DEFAULT_KEYBINDINGS.esc.desc);
    assert_eq!(manual_episode_search_context_clues_iter.next(), None);
  }

  #[test]
  fn test_manual_episode_search_contextual_context_clues() {
    let mut manual_search_contextual_context_clues_iter =
      MANUAL_EPISODE_SEARCH_CONTEXTUAL_CONTEXT_CLUES.iter();
    let (key_binding, description) = manual_search_contextual_context_clues_iter.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.submit);
    assert_str_eq!(*description, "details");
    assert_eq!(manual_search_contextual_context_clues_iter.next(), None);
  }

  #[test]
  fn test_episode_details_context_clues() {
    let mut episode_details_context_clues_iter = EPISODE_DETAILS_CONTEXT_CLUES.iter();

    let (key_binding, description) = episode_details_context_clues_iter.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.refresh);
    assert_str_eq!(*description, DEFAULT_KEYBINDINGS.refresh.desc);

    let (key_binding, description) = episode_details_context_clues_iter.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.search);
    assert_str_eq!(*description, "auto search");

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
}
