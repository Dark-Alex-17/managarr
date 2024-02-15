use crate::ui::styles::ManagarrStyle;
use crate::ui::utils::title_block_centered;
use ratatui::buffer::Buffer;
use ratatui::layout::{Alignment, Rect};
use ratatui::style::{Style, Stylize};
use ratatui::text::Text;
use ratatui::widgets::{Paragraph, Widget, Wrap};

#[cfg(test)]
#[path = "message_tests.rs"]
mod message_tests;

pub struct Message<'a> {
  text: Text<'a>,
  title: &'a str,
  style: Style,
}

impl<'a> Message<'a> {
  pub fn new<T>(message: T) -> Self
  where
    T: Into<Text<'a>>,
  {
    Message {
      text: message.into(),
      title: "Error",
      style: Style::new().failure().bold(),
    }
  }

  pub fn title(mut self, title: &'a str) -> Self {
    self.title = title;
    self
  }

  pub fn style(mut self, style: Style) -> Self {
    self.style = style;
    self
  }

  fn render_message(self, area: Rect, buf: &mut Buffer) {
    Paragraph::new(self.text)
      .style(self.style)
      .alignment(Alignment::Center)
      .block(title_block_centered(self.title).style(self.style))
      .wrap(Wrap { trim: true })
      .render(area, buf);
  }
}

impl<'a> Widget for Message<'a> {
  fn render(self, area: Rect, buf: &mut Buffer) {
    self.render_message(area, buf);
  }
}
