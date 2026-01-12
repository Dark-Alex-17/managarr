#[cfg(test)]
mod test {
  use crate::app::context_clues::{
    BARE_POPUP_CONTEXT_CLUES, BLOCKLIST_CONTEXT_CLUES, CONFIRMATION_PROMPT_CONTEXT_CLUES,
    ContextClueProvider, DOWNLOADS_CONTEXT_CLUES, HISTORY_CONTEXT_CLUES, INDEXERS_CONTEXT_CLUES,
    ROOT_FOLDERS_CONTEXT_CLUES, SERVARR_CONTEXT_CLUES, SYSTEM_CONTEXT_CLUES,
    ServarrContextClueProvider,
  };
  use crate::app::{App, key_binding::DEFAULT_KEYBINDINGS};
  use crate::models::servarr_data::ActiveKeybindingBlock;
  use crate::models::servarr_data::radarr::radarr_data::ActiveRadarrBlock;
  use crate::models::servarr_data::sonarr::sonarr_data::ActiveSonarrBlock;

  #[test]
  fn test_servarr_context_clues() {
    let mut servarr_context_clues_iter = SERVARR_CONTEXT_CLUES.iter();

    assert_some_eq_x!(
      servarr_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.up, "scroll up")
    );
    assert_some_eq_x!(
      servarr_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.down, "scroll down")
    );
    assert_some_eq_x!(
      servarr_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.left, "previous tab")
    );
    assert_some_eq_x!(
      servarr_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.right, "next tab")
    );
    assert_some_eq_x!(
      servarr_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.pg_up, DEFAULT_KEYBINDINGS.pg_up.desc)
    );
    assert_some_eq_x!(
      servarr_context_clues_iter.next(),
      &(
        DEFAULT_KEYBINDINGS.pg_down,
        DEFAULT_KEYBINDINGS.pg_down.desc
      )
    );
    assert_some_eq_x!(
      servarr_context_clues_iter.next(),
      &(
        DEFAULT_KEYBINDINGS.next_servarr,
        DEFAULT_KEYBINDINGS.next_servarr.desc
      )
    );
    assert_some_eq_x!(
      servarr_context_clues_iter.next(),
      &(
        DEFAULT_KEYBINDINGS.previous_servarr,
        DEFAULT_KEYBINDINGS.previous_servarr.desc
      )
    );
    assert_some_eq_x!(
      servarr_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.quit, DEFAULT_KEYBINDINGS.quit.desc)
    );
    assert_some_eq_x!(
      servarr_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.help, DEFAULT_KEYBINDINGS.help.desc)
    );
    assert_none!(servarr_context_clues_iter.next());
  }

  #[test]
  fn test_bare_popup_context_clues() {
    let mut bare_popup_context_clues_iter = BARE_POPUP_CONTEXT_CLUES.iter();

    assert_some_eq_x!(
      bare_popup_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.esc, DEFAULT_KEYBINDINGS.esc.desc)
    );
    assert_none!(bare_popup_context_clues_iter.next());
  }

  #[test]
  fn test_downloads_context_clues() {
    let mut downloads_context_clues_iter = DOWNLOADS_CONTEXT_CLUES.iter();

    assert_some_eq_x!(
      downloads_context_clues_iter.next(),
      &(
        DEFAULT_KEYBINDINGS.refresh,
        DEFAULT_KEYBINDINGS.refresh.desc
      )
    );
    assert_some_eq_x!(
      downloads_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.delete, DEFAULT_KEYBINDINGS.delete.desc)
    );
    assert_some_eq_x!(
      downloads_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.update, "update downloads")
    );
    assert_none!(downloads_context_clues_iter.next());
  }

  #[test]
  fn test_blocklist_context_clues() {
    let mut blocklist_context_clues_iter = BLOCKLIST_CONTEXT_CLUES.iter();

    assert_some_eq_x!(
      blocklist_context_clues_iter.next(),
      &(
        DEFAULT_KEYBINDINGS.refresh,
        DEFAULT_KEYBINDINGS.refresh.desc
      )
    );
    assert_some_eq_x!(
      blocklist_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.sort, DEFAULT_KEYBINDINGS.sort.desc)
    );
    assert_some_eq_x!(
      blocklist_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.submit, "details")
    );
    assert_some_eq_x!(
      blocklist_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.delete, DEFAULT_KEYBINDINGS.delete.desc)
    );
    assert_some_eq_x!(
      blocklist_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.clear, "clear blocklist")
    );
    assert_none!(blocklist_context_clues_iter.next());
  }

  #[test]
  fn test_confirmation_prompt_context_clues() {
    let mut confirmation_prompt_context_clues_iter = CONFIRMATION_PROMPT_CONTEXT_CLUES.iter();

    assert_some_eq_x!(
      confirmation_prompt_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.confirm, "submit")
    );
    assert_some_eq_x!(
      confirmation_prompt_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.esc, "cancel")
    );
    assert_none!(confirmation_prompt_context_clues_iter.next());
  }

  #[test]
  fn test_root_folders_context_clues() {
    let mut root_folders_context_clues_iter = ROOT_FOLDERS_CONTEXT_CLUES.iter();

    assert_some_eq_x!(
      root_folders_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.add, DEFAULT_KEYBINDINGS.add.desc)
    );
    assert_some_eq_x!(
      root_folders_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.delete, DEFAULT_KEYBINDINGS.delete.desc)
    );
    assert_some_eq_x!(
      root_folders_context_clues_iter.next(),
      &(
        DEFAULT_KEYBINDINGS.refresh,
        DEFAULT_KEYBINDINGS.refresh.desc
      )
    );
    assert_none!(root_folders_context_clues_iter.next());
  }

  #[test]
  fn test_indexers_context_clues() {
    let mut indexers_context_clues_iter = INDEXERS_CONTEXT_CLUES.iter();

    assert_some_eq_x!(
      indexers_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.submit, "edit indexer")
    );
    assert_some_eq_x!(
      indexers_context_clues_iter.next(),
      &(
        DEFAULT_KEYBINDINGS.settings,
        DEFAULT_KEYBINDINGS.settings.desc
      )
    );
    assert_some_eq_x!(
      indexers_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.delete, DEFAULT_KEYBINDINGS.delete.desc)
    );
    assert_some_eq_x!(
      indexers_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.test, "test indexer")
    );
    assert_some_eq_x!(
      indexers_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.test_all, "test all indexers")
    );
    assert_some_eq_x!(
      indexers_context_clues_iter.next(),
      &(
        DEFAULT_KEYBINDINGS.refresh,
        DEFAULT_KEYBINDINGS.refresh.desc
      )
    );
    assert_none!(indexers_context_clues_iter.next());
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
  fn test_system_context_clues() {
    let mut system_context_clues_iter = SYSTEM_CONTEXT_CLUES.iter();

    assert_some_eq_x!(
      system_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.tasks, "open tasks")
    );
    assert_some_eq_x!(
      system_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.events, "open events")
    );
    assert_some_eq_x!(
      system_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.logs, "open logs")
    );
    assert_some_eq_x!(
      system_context_clues_iter.next(),
      &(DEFAULT_KEYBINDINGS.update, "open updates")
    );
    assert_some_eq_x!(
      system_context_clues_iter.next(),
      &(
        DEFAULT_KEYBINDINGS.refresh,
        DEFAULT_KEYBINDINGS.refresh.desc
      )
    );
    assert_none!(system_context_clues_iter.next());
  }

  #[test]
  fn test_servarr_context_clue_provider_delegates_to_radarr_provider() {
    let mut app = App::test_default();
    app.push_navigation_stack(ActiveRadarrBlock::SystemTasks.into());

    let context_clues = ServarrContextClueProvider::get_context_clues(&mut app);

    assert_some_eq_x!(
      context_clues,
      &crate::app::radarr::radarr_context_clues::SYSTEM_TASKS_CONTEXT_CLUES,
    );
  }

  #[test]
  fn test_servarr_context_clue_provider_delegates_to_sonarr_provider() {
    let mut app = App::test_default();
    app.push_navigation_stack(ActiveSonarrBlock::SystemTasks.into());

    let context_clues = ServarrContextClueProvider::get_context_clues(&mut app);

    assert_some_eq_x!(
      context_clues,
      &crate::app::sonarr::sonarr_context_clues::SYSTEM_TASKS_CONTEXT_CLUES,
    );
  }

  #[test]
  fn test_servarr_context_clue_provider_unsupported_route_returns_none() {
    let mut app = App::test_default();
    app.push_navigation_stack(ActiveKeybindingBlock::Help.into());

    let context_clues = ServarrContextClueProvider::get_context_clues(&mut app);

    assert_none!(context_clues);
  }
}
