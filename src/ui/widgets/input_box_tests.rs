#[cfg(test)]
mod tests {
  use crate::ui::styles::ManagarrStyle;
  use crate::ui::utils::layout_block;
  use crate::ui::widgets::input_box::InputBox;
  use pretty_assertions::{assert_eq, assert_str_eq};
  use ratatui::style::Style;

  #[test]
  fn test_input_box_new() {
    let input_box = InputBox::new("test");

    assert_str_eq!(input_box.content, "test");
    assert_eq!(input_box.offset, 0);
    assert_eq!(input_box.style, Style::new().default());
    assert_eq!(input_box.block, layout_block());
    assert_eq!(input_box.label, None);
    assert!(input_box.cursor_after_string);
    assert_eq!(input_box.is_highlighted, None);
    assert_eq!(input_box.is_selected, None);
  }

  #[test]
  fn test_input_box_is_selected() {
    let input_box = InputBox::new("test").selected(true);

    assert!(input_box.is_selected());
  }
}
