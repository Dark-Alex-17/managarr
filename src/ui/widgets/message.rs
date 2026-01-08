use crate::ui::styles::failure_style;
use crate::ui::utils::title_block_centered;
use derive_setters::Setters;
use ratatui::buffer::Buffer;
use ratatui::layout::{Alignment, Rect};
use ratatui::style::Style;
use ratatui::text::Text;
use ratatui::widgets::{Paragraph, Widget, Wrap};

#[cfg(test)]
#[path = "message_tests.rs"]
mod message_tests;

#[derive(Setters)]
pub struct Message<'a> {
  text: Text<'a>,
  title: &'a str,
  style: Style,
  alignment: Alignment,
}

impl<'a> Message<'a> {
  pub fn new<T>(message: T) -> Self
  where
    T: Into<Text<'a>>,
  {
    Message {
      text: message.into(),
      title: "Error",
      style: failure_style().bold(),
      alignment: Alignment::Center,
    }
  }

  fn render_message(self, area: Rect, buf: &mut Buffer) {
    Paragraph::new(self.text)
      .style(self.style)
      .alignment(self.alignment)
      .block(title_block_centered(self.title).style(self.style))
      .wrap(Wrap { trim: true })
      .render(area, buf);
  }
}

impl Widget for Message<'_> {
  fn render(self, area: Rect, buf: &mut Buffer) {
    self.render_message(area, buf);
  }
}
