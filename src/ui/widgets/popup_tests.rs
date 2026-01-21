#[cfg(test)]
mod tests {
  use crate::ui::widgets::popup::{Popup, Size};
  use pretty_assertions::assert_eq;
  use ratatui::widgets::Block;

  #[test]
  fn test_dimensions_to_percent() {
    assert_eq!(Size::SmallPrompt.to_percent(), (20, 20));
    assert_eq!(Size::MediumPrompt.to_percent(), (37, 37));
    assert_eq!(Size::LargePrompt.to_percent(), (45, 45));
    assert_eq!(Size::WideLargePrompt.to_percent(), (70, 50));
    assert_eq!(Size::Message.to_percent(), (25, 8));
    assert_eq!(Size::NarrowMessage.to_percent(), (50, 20));
    assert_eq!(Size::NarrowLongMessage.to_percent(), (50, 35));
    assert_eq!(Size::LargeMessage.to_percent(), (25, 25));
    assert_eq!(Size::InputBox.to_percent(), (30, 13));
    assert_eq!(Size::Dropdown.to_percent(), (20, 30));
    assert_eq!(Size::Small.to_percent(), (40, 40));
    assert_eq!(Size::Medium.to_percent(), (60, 60));
    assert_eq!(Size::Large.to_percent(), (75, 75));
    assert_eq!(Size::XLarge.to_percent(), (83, 83));
    assert_eq!(Size::XXLarge.to_percent(), (90, 90));
    assert_eq!(Size::Long.to_percent(), (65, 75));
    assert_eq!(Size::LongNarrowTable.to_percent(), (55, 85));
  }

  #[test]
  fn test_popup_new() {
    let popup = Popup::new(Block::new());

    assert_eq!(popup.widget, Block::new());
    assert_eq!(popup.percent_x, 0);
    assert_eq!(popup.percent_y, 0);
    assert_eq!(popup.block, None);
    assert_eq!(popup.margin, 0);
  }

  #[test]
  fn test_popup_size() {
    let popup = Popup::new(Block::new()).size(Size::Small);

    assert_eq!(popup.percent_x, 40);
    assert_eq!(popup.percent_y, 40);
    assert_eq!(popup.widget, Block::new());
    assert_eq!(popup.block, None);
    assert_eq!(popup.margin, 0);
  }

  #[test]
  fn test_popup_dimensions() {
    let popup = Popup::new(Block::new()).dimensions(25, 50);

    assert_eq!(popup.percent_x, 25);
    assert_eq!(popup.percent_y, 50);
    assert_eq!(popup.widget, Block::new());
    assert_eq!(popup.block, None);
    assert_eq!(popup.margin, 0);
  }

  #[test]
  fn test_popup_block() {
    let popup = Popup::new(Block::new()).block(Block::new());

    assert_eq!(popup.block, Some(Block::new()));
    assert_eq!(popup.widget, Block::new());
    assert_eq!(popup.percent_x, 0);
    assert_eq!(popup.percent_y, 0);
    assert_eq!(popup.margin, 0);
  }

  #[test]
  fn test_popup_margin() {
    let popup = Popup::new(Block::new()).margin(5);

    assert_eq!(popup.margin, 5);
    assert_eq!(popup.widget, Block::new());
    assert_eq!(popup.percent_x, 0);
    assert_eq!(popup.percent_y, 0);
    assert_eq!(popup.block, None);
  }
}
