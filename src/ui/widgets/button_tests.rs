#[cfg(test)]
mod tests {
  use crate::ui::widgets::button::Button;
  use pretty_assertions::{assert_eq, assert_str_eq};
  use ratatui::style::{Style, Stylize};

  #[test]
  fn test_title() {
    let button = Button::default().title("Title");

    assert_str_eq!(button.title, "Title");
  }

  #[test]
  fn test_label() {
    let button = Button::default().label("Label");

    assert_eq!(button.label, Some("Label"));
  }

  #[test]
  fn test_icon() {
    let button = Button::default().icon("Icon");

    assert_eq!(button.icon, Some("Icon"));
  }

  #[test]
  fn test_style() {
    let button = Button::default().style(Style::new().bold());

    assert_eq!(button.style, Style::new().bold());
  }

  #[test]
  fn test_selected() {
    let button = Button::default().selected(true);

    assert!(button.is_selected);
  }
}
