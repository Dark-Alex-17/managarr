use crate::ui::styles::ManagarrStyle;
use crate::ui::utils::{background_block, borderless_block, centered_rect};
use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::widgets::{Block, Clear, Paragraph, Widget, WidgetRef};

use super::input_box::InputBox;

#[cfg(test)]
#[path = "input_box_popup_tests.rs"]
mod input_box_popup_tests;

pub struct InputBoxPopup<'a> {
  input_box: InputBox<'a>,
}

impl<'a> InputBoxPopup<'a> {
  pub fn new(content: &'a str) -> Self {
    Self {
      input_box: InputBox::new(content),
    }
  }

  pub fn block(mut self, block: Block<'a>) -> InputBoxPopup<'a> {
    self.input_box = self.input_box.block(block);
    self
  }

  pub fn offset(mut self, offset: usize) -> InputBoxPopup<'a> {
    self.input_box = self.input_box.offset(offset);
    self
  }

  fn render_popup(&self, area: Rect, buf: &mut Buffer) {
    let popup_area = Rect {
      height: 6,
      ..centered_rect(30, 20, area)
    };
    Clear.render(popup_area, buf);
    background_block().render(popup_area, buf);

    let [text_box_area, help_area] =
      Layout::vertical([Constraint::Length(3), Constraint::Length(1)])
        .margin(1)
        .areas(popup_area);
    self.input_box.render_ref(text_box_area, buf);

    let help = Paragraph::new("<esc> cancel")
      .help()
      .centered()
      .block(borderless_block());
    help.render(help_area, buf);
  }
}

impl WidgetRef for InputBoxPopup<'_> {
  fn render_ref(&self, area: Rect, buf: &mut Buffer) {
    self.render_popup(area, buf);
  }
}
