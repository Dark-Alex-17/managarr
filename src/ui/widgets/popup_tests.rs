#[cfg(test)]
mod tests {
  use crate::ui::widgets::popup::Popup;
  use pretty_assertions::assert_eq;
  use ratatui::layout::Alignment;
  use ratatui::widgets::Block;

  #[test]
  fn test_popup_new() {
    let popup = Popup::new(Block::new(), 50, 50);

    assert_eq!(popup.widget, Block::new());
    assert_eq!(popup.percent_x, 50);
    assert_eq!(popup.percent_y, 50);
    assert_eq!(popup.block, None);
    assert_eq!(popup.footer, None);
    assert_eq!(popup.footer_alignment, Alignment::Left);
  }

  #[test]
  fn test_popup_block() {
    let popup = Popup::new(Block::new(), 50, 50).block(Block::new());

    assert_eq!(popup.block, Some(Block::new()));
    assert_eq!(popup.widget, Block::new());
    assert_eq!(popup.percent_x, 50);
    assert_eq!(popup.percent_y, 50);
    assert_eq!(popup.footer, None);
    assert_eq!(popup.footer_alignment, Alignment::Left);
  }

  #[test]
  fn test_popup_footer() {
    let popup = Popup::new(Block::new(), 50, 50).footer("footer");

    assert_eq!(popup.footer, Some("footer"));
    assert_eq!(popup.widget, Block::new());
    assert_eq!(popup.percent_x, 50);
    assert_eq!(popup.percent_y, 50);
    assert_eq!(popup.block, None);
    assert_eq!(popup.footer_alignment, Alignment::Left);
  }

  #[test]
  fn test_popup_footer_alignment() {
    let popup = Popup::new(Block::new(), 50, 50).footer_alignment(Alignment::Center);

    assert_eq!(popup.footer_alignment, Alignment::Center);
    assert_eq!(popup.widget, Block::new());
    assert_eq!(popup.percent_x, 50);
    assert_eq!(popup.percent_y, 50);
    assert_eq!(popup.block, None);
    assert_eq!(popup.footer, None);
  }
}
