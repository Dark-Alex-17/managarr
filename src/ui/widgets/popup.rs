use crate::ui::utils::{background_block, centered_rect};
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::widgets::{Clear, Widget};

pub struct Popup<T: Widget> {
  widget: T,
  percent_x: u16,
  percent_y: u16,
}

impl<T: Widget> Popup<T> {
  pub fn new(widget: T, percent_x: u16, percent_y: u16) -> Self {
    Self {
      widget,
      percent_x,
      percent_y,
    }
  }

  fn render_popup(self, area: Rect, buf: &mut Buffer) {
    let popup_area = centered_rect(self.percent_x, self.percent_y, area);

    Clear.render(popup_area, buf);
    background_block().render(popup_area, buf);
    self.widget.render(popup_area, buf);
  }
}

impl<T: Widget> Widget for Popup<T> {
  fn render(self, area: Rect, buf: &mut Buffer) {
    self.render_popup(area, buf);
  }
}
