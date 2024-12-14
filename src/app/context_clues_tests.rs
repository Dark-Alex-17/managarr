#[cfg(test)]
mod test {
  use pretty_assertions::{assert_eq, assert_str_eq};

  use crate::app::context_clues::{
    BARE_POPUP_CONTEXT_CLUES, BLOCKLIST_CONTEXT_CLUES, CONFIRMATION_PROMPT_CONTEXT_CLUES,
    DOWNLOADS_CONTEXT_CLUES, INDEXERS_CONTEXT_CLUES, ROOT_FOLDERS_CONTEXT_CLUES,
    SERVARR_CONTEXT_CLUES, SYSTEM_CONTEXT_CLUES,
  };
  use crate::app::{context_clues::build_context_clue_string, key_binding::DEFAULT_KEYBINDINGS};

  #[test]
  fn test_build_context_clue_string() {
    let test_context_clues_array = [
      (DEFAULT_KEYBINDINGS.add, "add"),
      (DEFAULT_KEYBINDINGS.delete, "delete"),
    ];

    assert_str_eq!(
      build_context_clue_string(&test_context_clues_array),
      "<a> add | <del> delete"
    );
  }

  #[test]
  fn test_servarr_context_clues() {
    let mut servarr_context_clues_iter = SERVARR_CONTEXT_CLUES.iter();

    let (key_binding, description) = servarr_context_clues_iter.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.next_servarr);
    assert_str_eq!(*description, DEFAULT_KEYBINDINGS.next_servarr.desc);

    let (key_binding, description) = servarr_context_clues_iter.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.previous_servarr);
    assert_str_eq!(*description, DEFAULT_KEYBINDINGS.previous_servarr.desc);

    let (key_binding, description) = servarr_context_clues_iter.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.quit);
    assert_str_eq!(*description, DEFAULT_KEYBINDINGS.quit.desc);
    assert_eq!(servarr_context_clues_iter.next(), None);
  }

  #[test]
  fn test_bare_popup_context_clues() {
    let mut bare_popup_context_clues_iter = BARE_POPUP_CONTEXT_CLUES.iter();

    let (key_binding, description) = bare_popup_context_clues_iter.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.esc);
    assert_str_eq!(*description, DEFAULT_KEYBINDINGS.esc.desc);
    assert_eq!(bare_popup_context_clues_iter.next(), None);
  }

  #[test]
  fn test_downloads_context_clues() {
    let mut downloads_context_clues_iter = DOWNLOADS_CONTEXT_CLUES.iter();

    let (key_binding, description) = downloads_context_clues_iter.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.refresh);
    assert_str_eq!(*description, DEFAULT_KEYBINDINGS.refresh.desc);

    let (key_binding, description) = downloads_context_clues_iter.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.delete);
    assert_str_eq!(*description, DEFAULT_KEYBINDINGS.delete.desc);

    let (key_binding, description) = downloads_context_clues_iter.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.update);
    assert_str_eq!(*description, "update downloads");
    assert_eq!(downloads_context_clues_iter.next(), None);
  }

  #[test]
  fn test_blocklist_context_clues() {
    let mut blocklist_context_clues_iter = BLOCKLIST_CONTEXT_CLUES.iter();

    let (key_binding, description) = blocklist_context_clues_iter.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.refresh);
    assert_str_eq!(*description, DEFAULT_KEYBINDINGS.refresh.desc);

    let (key_binding, description) = blocklist_context_clues_iter.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.sort);
    assert_str_eq!(*description, DEFAULT_KEYBINDINGS.sort.desc);

    let (key_binding, description) = blocklist_context_clues_iter.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.submit);
    assert_str_eq!(*description, "details");

    let (key_binding, description) = blocklist_context_clues_iter.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.delete);
    assert_str_eq!(*description, DEFAULT_KEYBINDINGS.delete.desc);

    let (key_binding, description) = blocklist_context_clues_iter.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.clear);
    assert_str_eq!(*description, "clear blocklist");
    assert_eq!(blocklist_context_clues_iter.next(), None);
  }

  #[test]
  fn test_confirmation_prompt_context_clues() {
    let mut confirmation_prompt_context_clues_iter = CONFIRMATION_PROMPT_CONTEXT_CLUES.iter();

    let (key_binding, description) = confirmation_prompt_context_clues_iter.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.confirm);
    assert_str_eq!(*description, "submit");

    let (key_binding, description) = confirmation_prompt_context_clues_iter.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.esc);
    assert_str_eq!(*description, "cancel");
    assert_eq!(confirmation_prompt_context_clues_iter.next(), None);
  }

  #[test]
  fn test_root_folders_context_clues() {
    let mut root_folders_context_clues_iter = ROOT_FOLDERS_CONTEXT_CLUES.iter();

    let (key_binding, description) = root_folders_context_clues_iter.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.add);
    assert_str_eq!(*description, DEFAULT_KEYBINDINGS.add.desc);

    let (key_binding, description) = root_folders_context_clues_iter.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.delete);
    assert_str_eq!(*description, DEFAULT_KEYBINDINGS.delete.desc);

    let (key_binding, description) = root_folders_context_clues_iter.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.refresh);
    assert_str_eq!(*description, DEFAULT_KEYBINDINGS.refresh.desc);
    assert_eq!(root_folders_context_clues_iter.next(), None);
  }

  #[test]
  fn test_indexers_context_clues() {
    let mut indexers_context_clues_iter = INDEXERS_CONTEXT_CLUES.iter();

    let (key_binding, description) = indexers_context_clues_iter.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.submit);
    assert_str_eq!(*description, "edit indexer");

    let (key_binding, description) = indexers_context_clues_iter.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.settings);
    assert_str_eq!(*description, DEFAULT_KEYBINDINGS.settings.desc);

    let (key_binding, description) = indexers_context_clues_iter.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.delete);
    assert_str_eq!(*description, DEFAULT_KEYBINDINGS.delete.desc);

    let (key_binding, description) = indexers_context_clues_iter.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.test);
    assert_str_eq!(*description, "test indexer");

    let (key_binding, description) = indexers_context_clues_iter.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.test_all);
    assert_str_eq!(*description, "test all indexers");

    let (key_binding, description) = indexers_context_clues_iter.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.refresh);
    assert_str_eq!(*description, DEFAULT_KEYBINDINGS.refresh.desc);
    assert_eq!(indexers_context_clues_iter.next(), None);
  }

  #[test]
  fn test_system_context_clues() {
    let mut system_context_clues_iter = SYSTEM_CONTEXT_CLUES.iter();

    let (key_binding, description) = system_context_clues_iter.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.tasks);
    assert_str_eq!(*description, "open tasks");

    let (key_binding, description) = system_context_clues_iter.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.events);
    assert_str_eq!(*description, "open events");

    let (key_binding, description) = system_context_clues_iter.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.logs);
    assert_str_eq!(*description, "open logs");

    let (key_binding, description) = system_context_clues_iter.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.update);
    assert_str_eq!(*description, "open updates");

    let (key_binding, description) = system_context_clues_iter.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.refresh);
    assert_str_eq!(*description, DEFAULT_KEYBINDINGS.refresh.desc);
    assert_eq!(system_context_clues_iter.next(), None);
  }
}
