#[cfg(test)]
mod tests {
  use crate::ui::utils::layout_block;
  use crate::ui::widgets::input_box::InputBox;
  use crate::ui::widgets::input_box_popup::InputBoxPopup;
  use pretty_assertions::assert_eq;

  #[test]
  fn test_input_box_popup_new() {
    let expected_input_box = InputBox::new("test");

    let input_box_popup = InputBoxPopup::new("test");

    assert_eq!(input_box_popup.input_box, expected_input_box);
  }

  #[test]
  fn test_input_box_popup_block() {
    let expected_input_box = InputBox::new("test").block(layout_block().title("title"));

    let input_box_popup = InputBoxPopup::new("test").block(layout_block().title("title"));

    assert_eq!(input_box_popup.input_box, expected_input_box);
  }

  #[test]
  fn test_input_box_popup_offset() {
    let expected_input_box = InputBox::new("test").offset(5);

    let input_box_popup = InputBoxPopup::new("test").offset(5);

    assert_eq!(input_box_popup.input_box, expected_input_box);
  }
}
