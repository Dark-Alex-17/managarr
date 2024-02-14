#[cfg(test)]
mod tests {
  use crate::ui::widgets::checkbox::Checkbox;
  use crate::ui::widgets::confirmation_prompt::ConfirmationPrompt;
  use pretty_assertions::{assert_eq, assert_str_eq};
  use ratatui::widgets::Paragraph;

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

  #[test]
  fn test_confirmation_prompt_title() {
    let confirmation_prompt = ConfirmationPrompt::new().title("title");

    assert_str_eq!(confirmation_prompt.title, "title");
    assert_str_eq!(confirmation_prompt.prompt, "");
    assert_eq!(confirmation_prompt.content, None);
    assert_eq!(confirmation_prompt.checkboxes, None);
    assert!(!confirmation_prompt.yes_no_value);
    assert!(confirmation_prompt.yes_no_highlighted);
  }

  #[test]
  fn test_confirmation_prompt_prompt() {
    let confirmation_prompt = ConfirmationPrompt::new().prompt("prompt");

    assert_str_eq!(confirmation_prompt.prompt, "prompt");
    assert_str_eq!(confirmation_prompt.title, "");
    assert_eq!(confirmation_prompt.content, None);
    assert_eq!(confirmation_prompt.checkboxes, None);
    assert!(!confirmation_prompt.yes_no_value);
    assert!(confirmation_prompt.yes_no_highlighted);
  }

  #[test]
  fn test_confirmation_prompt_content() {
    let content = Paragraph::new("content");
    let confirmation_prompt = ConfirmationPrompt::new().content(content.clone());

    assert_eq!(confirmation_prompt.content, Some(content));
    assert_str_eq!(confirmation_prompt.title, "");
    assert_str_eq!(confirmation_prompt.prompt, "");
    assert_eq!(confirmation_prompt.checkboxes, None);
    assert!(!confirmation_prompt.yes_no_value);
    assert!(confirmation_prompt.yes_no_highlighted);
  }

  #[test]
  fn test_confirmation_prompt_checkboxes() {
    let checkboxes = vec![Checkbox::new("test").highlighted(true).checked(false)];
    let confirmation_prompt = ConfirmationPrompt::new().checkboxes(checkboxes.clone());

    assert_eq!(confirmation_prompt.checkboxes, Some(checkboxes));
    assert_str_eq!(confirmation_prompt.title, "");
    assert_str_eq!(confirmation_prompt.prompt, "");
    assert_eq!(confirmation_prompt.content, None);
    assert!(!confirmation_prompt.yes_no_value);
    assert!(confirmation_prompt.yes_no_highlighted);
  }

  #[test]
  fn test_confirmation_prompt_yes_no_value() {
    let confirmation_prompt = ConfirmationPrompt::new().yes_no_value(true);

    assert!(confirmation_prompt.yes_no_value);
    assert_str_eq!(confirmation_prompt.title, "");
    assert_str_eq!(confirmation_prompt.prompt, "");
    assert_eq!(confirmation_prompt.content, None);
    assert_eq!(confirmation_prompt.checkboxes, None);
    assert!(confirmation_prompt.yes_no_highlighted);
  }

  #[test]
  fn test_confirmation_prompt_yes_no_highlighted() {
    let confirmation_prompt = ConfirmationPrompt::new().yes_no_highlighted(false);

    assert!(!confirmation_prompt.yes_no_highlighted);
    assert_str_eq!(confirmation_prompt.title, "");
    assert_str_eq!(confirmation_prompt.prompt, "");
    assert_eq!(confirmation_prompt.content, None);
    assert_eq!(confirmation_prompt.checkboxes, None);
    assert!(!confirmation_prompt.yes_no_value);
  }
}
