use crate::ui::styles::ManagarrStyle;
use crate::ui::utils::title_block_centered;
use ratatui::buffer::Buffer;
use ratatui::layout::{Alignment, Rect};
use ratatui::style::Stylize;
use ratatui::text::Text;
use ratatui::widgets::{Paragraph, Widget};

pub struct ErrorMessage<'a> {
  text: Text<'a>,
}

impl<'a> ErrorMessage<'a> {
  pub fn new<T>(message: T) -> Self
  where
    T: Into<Text<'a>>,
  {
    ErrorMessage {
      text: message.into(),
    }
  }

  fn render_error_message(self, area: Rect, buf: &mut Buffer) {
    Paragraph::new(self.text)
      .failure()
      .alignment(Alignment::Center)
      .block(title_block_centered("Error").failure().bold())
      .render(area, buf);
  }
}

impl<'a> Widget for ErrorMessage<'a> {
  fn render(self, area: Rect, buf: &mut Buffer) {
    self.render_error_message(area, buf);
  }
}
