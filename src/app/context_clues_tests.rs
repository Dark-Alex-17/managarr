#[cfg(test)]
mod test {
  use pretty_assertions::{assert_eq, assert_str_eq};

  use crate::app::context_clues::{BARE_POPUP_CONTEXT_CLUES, SERVARR_CONTEXT_CLUES};
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

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.tab);
    assert_str_eq!(*description, "change servarr");

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
}
