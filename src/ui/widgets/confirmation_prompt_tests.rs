#[cfg(test)]
mod tests {
  use crate::ui::widgets::confirmation_prompt::ConfirmationPrompt;
  use pretty_assertions::{assert_eq, assert_str_eq};

  #[test]
  fn test_confirmation_prompt_new() {
    let confirmation_prompt = ConfirmationPrompt::new();

    assert_str_eq!(confirmation_prompt.title, "");
    assert_str_eq!(confirmation_prompt.prompt, "");
    assert_eq!(confirmation_prompt.content, None);
    assert_eq!(confirmation_prompt.checkboxes, None);
    assert!(!confirmation_prompt.yes_no_value);
    assert!(confirmation_prompt.yes_no_highlighted);
  }
}
