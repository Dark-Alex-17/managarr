#[cfg(test)]
mod tests {
  use crate::ui::styles::ManagarrStyle;
  use crate::ui::widgets::message::Message;
  use pretty_assertions::{assert_eq, assert_str_eq};
  use ratatui::layout::Alignment;
  use ratatui::style::{Style, Stylize};
  use ratatui::text::Text;

  #[test]
  fn test_error_message_new() {
    let test_message = "This is a message";

    let message = Message::new(test_message);

    assert_eq!(message.text, Text::from(test_message));
    assert_str_eq!(message.title, "Error");
    assert_eq!(message.style, Style::new().failure().bold());
    assert_eq!(message.alignment, Alignment::Center);
  }
}
