#[cfg(test)]
mod tests {
  use pretty_assertions::{assert_eq, assert_str_eq};

  use crate::app::{
    key_binding::DEFAULT_KEYBINDINGS,
    sonarr::sonarr_context_clues::{HISTORY_CONTEXT_CLUES, SERIES_CONTEXT_CLUES},
  };

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

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.delete);
    assert_str_eq!(*description, "mark as failed");

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
}
