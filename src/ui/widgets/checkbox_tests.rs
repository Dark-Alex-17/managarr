#[cfg(test)]
mod tests {
  use crate::ui::widgets::checkbox::Checkbox;
  use pretty_assertions::assert_str_eq;

  #[test]
  fn test_checkbox_new() {
    let checkbox = Checkbox::new("test");

    assert_str_eq!(checkbox.label, "test");
    assert!(!checkbox.is_checked);
    assert!(!checkbox.is_highlighted);
  }

  #[test]
  fn test_checkbox_checked() {
    let checkbox = Checkbox::new("test").checked(true);

    assert_str_eq!(checkbox.label, "test");
    assert!(checkbox.is_checked);
    assert!(!checkbox.is_highlighted);
  }

  #[test]
  fn test_checkbox_highlighted() {
    let checkbox = Checkbox::new("test").highlighted(true);

    assert_str_eq!(checkbox.label, "test");
    assert!(!checkbox.is_checked);
    assert!(checkbox.is_highlighted);
  }
}
