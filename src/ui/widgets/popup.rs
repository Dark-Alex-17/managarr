use crate::ui::styles::ManagarrStyle;
use crate::ui::utils::{background_block, centered_rect, layout_block_top_border};
use ratatui::buffer::Buffer;
use ratatui::layout::{Alignment, Constraint, Layout, Rect};
use ratatui::prelude::Text;
use ratatui::widgets::{Block, Clear, Paragraph, Widget};

#[cfg(test)]
#[path = "popup_tests.rs"]
mod popup_tests;

pub struct Popup<'a, T: Widget> {
  widget: T,
  percent_x: u16,
  percent_y: u16,
  block: Option<Block<'a>>,
  footer: Option<&'a str>,
  footer_alignment: Alignment,
}

impl<'a, T: Widget> Popup<'a, T> {
  pub fn new(widget: T, percent_x: u16, percent_y: u16) -> Self {
    Self {
      widget,
      percent_x,
      percent_y,
      block: None,
      footer: None,
      footer_alignment: Alignment::Left,
    }
  }

  pub fn block(mut self, block: Block<'a>) -> Self {
    self.block = Some(block);
    self
  }

  pub fn footer(mut self, footer: &'a str) -> Self {
    self.footer = Some(footer);
    self
  }

  pub fn footer_alignment(mut self, alignment: Alignment) -> Self {
    self.footer_alignment = alignment;
    self
  }

  fn render_popup(self, area: Rect, buf: &mut Buffer) {
    let popup_area = centered_rect(self.percent_x, self.percent_y, area);
    Clear.render(popup_area, buf);
    background_block().render(popup_area, buf);

    if let Some(block) = self.block {
      block.render(popup_area, buf);
    }

    let content_area = if let Some(footer) = self.footer {
      let [content_area, help_footer_area] =
        Layout::vertical([Constraint::Fill(0), Constraint::Length(2)])
          .margin(1)
          .areas(popup_area);

      Paragraph::new(Text::from(format!(" {footer}").help()))
        .block(layout_block_top_border())
        .alignment(self.footer_alignment)
        .render(help_footer_area, buf);

      content_area
    } else {
      popup_area
    };

    self.widget.render(content_area, buf);
  }
}

impl<'a, T: Widget> Widget for Popup<'a, T> {
  fn render(self, area: Rect, buf: &mut Buffer) {
    self.render_popup(area, buf);
  }
}
