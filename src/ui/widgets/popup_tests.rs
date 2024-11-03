#[cfg(test)]
mod tests {
  use crate::ui::widgets::popup::{Popup, Size};
  use pretty_assertions::assert_eq;
  use ratatui::widgets::Block;

  #[test]
  fn test_dimensions_to_percent() {
    assert_eq!(Size::SmallPrompt.to_percent(), (20, 20));
    assert_eq!(Size::Prompt.to_percent(), (37, 37));
    assert_eq!(Size::LargePrompt.to_percent(), (70, 45));
    assert_eq!(Size::Message.to_percent(), (25, 8));
    assert_eq!(Size::NarrowMessage.to_percent(), (50, 20));
    assert_eq!(Size::LargeMessage.to_percent(), (25, 25));
    assert_eq!(Size::InputBox.to_percent(), (30, 13));
    assert_eq!(Size::Dropdown.to_percent(), (20, 30));
    assert_eq!(Size::Small.to_percent(), (40, 40));
    assert_eq!(Size::Medium.to_percent(), (60, 60));
    assert_eq!(Size::Large.to_percent(), (75, 75));
  }

  #[test]
  fn test_popup_new() {
    let popup = Popup::new(Block::new());

    assert_eq!(popup.widget, Block::new());
    assert_eq!(popup.percent_x, 0);
    assert_eq!(popup.percent_y, 0);
    assert_eq!(popup.block, None);
    assert_eq!(popup.footer, None);
  }

  #[test]
  fn test_popup_size() {
    let popup = Popup::new(Block::new()).size(Size::Small);

    assert_eq!(popup.percent_x, 40);
    assert_eq!(popup.percent_y, 40);
    assert_eq!(popup.widget, Block::new());
    assert_eq!(popup.block, None);
    assert_eq!(popup.footer, None);
  }

  #[test]
  fn test_popup_dimensions() {
    let popup = Popup::new(Block::new()).dimensions(25, 50);

    assert_eq!(popup.percent_x, 25);
    assert_eq!(popup.percent_y, 50);
    assert_eq!(popup.widget, Block::new());
    assert_eq!(popup.block, None);
    assert_eq!(popup.footer, None);
  }

  #[test]
  fn test_popup_block() {
    let popup = Popup::new(Block::new()).block(Block::new());

    assert_eq!(popup.block, Some(Block::new()));
    assert_eq!(popup.widget, Block::new());
    assert_eq!(popup.percent_x, 0);
    assert_eq!(popup.percent_y, 0);
    assert_eq!(popup.footer, None);
  }

  #[test]
  fn test_popup_footer() {
    let popup = Popup::new(Block::new()).footer("footer");

    assert_eq!(popup.footer, Some("footer"));
    assert_eq!(popup.widget, Block::new());
    assert_eq!(popup.percent_x, 0);
    assert_eq!(popup.percent_y, 0);
    assert_eq!(popup.block, None);
  }
}
