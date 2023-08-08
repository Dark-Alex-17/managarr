#[cfg(test)]
mod test {
  use crate::app::key_binding::{build_keymapping_string, DEFAULT_KEYBINDINGS};
  use pretty_assertions::assert_str_eq;

  #[test]
  fn test_build_keymapping_string() {
    let test_keys_array = [
      (DEFAULT_KEYBINDINGS.add, "add"),
      (DEFAULT_KEYBINDINGS.delete, "delete"),
    ];

    assert_str_eq!(
      build_keymapping_string(&test_keys_array),
      "<a> add | <del> delete"
    );
  }
}
