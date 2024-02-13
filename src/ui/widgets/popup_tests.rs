#[cfg(test)]
mod tests {
  use crate::ui::widgets::popup::Popup;
  use pretty_assertions::assert_eq;
  use ratatui::widgets::Block;

  #[test]
  fn test_popup_new() {
    let popup = Popup::new(Block::new(), 50, 50);

    assert_eq!(popup.widget, Block::new());
    assert_eq!(popup.percent_x, 50);
    assert_eq!(popup.percent_y, 50);
  }
}
