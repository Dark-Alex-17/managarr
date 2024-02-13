#[cfg(test)]
mod tests {
  use crate::ui::widgets::error_message::ErrorMessage;
  use pretty_assertions::assert_eq;
  use ratatui::text::Text;

  #[test]
  fn test_error_message_new() {
    let message = "This is an error message";
    let error_message = ErrorMessage::new(message);

    assert_eq!(error_message.text, Text::from(message));
  }
}
