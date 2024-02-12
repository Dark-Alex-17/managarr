use crate::ui::styles::ManagarrStyle;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::prelude::Text;
use ratatui::widgets::{Block, Paragraph, Widget};

pub struct LoadingBlock<'a> {
  is_loading: bool,
  block: Block<'a>,
}

impl<'a> LoadingBlock<'a> {
  pub fn new(is_loading: bool, block: Block<'a>) -> Self {
    Self { is_loading, block }
  }

  fn render_loading_block(self, area: Rect, buf: &mut Buffer) {
    if self.is_loading {
      Paragraph::new(Text::from("\n\n Loading ...\n\n"))
        .system_function()
        .block(self.block)
        .render(area, buf);
    } else {
      self.block.render(area, buf);
    }
  }
}

impl<'a> Widget for LoadingBlock<'a> {
  fn render(self, area: Rect, buf: &mut Buffer) {
    self.render_loading_block(area, buf);
  }
}
