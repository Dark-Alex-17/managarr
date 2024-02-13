use crate::models::stateful_list::StatefulList;
use crate::ui::styles::ManagarrStyle;
use crate::ui::utils::layout_block;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::prelude::Widget;
use ratatui::style::Style;
use ratatui::widgets::{Block, List, ListItem, StatefulWidget};

#[cfg(test)]
#[path = "selectable_list_tests.rs"]
mod selectable_list_tests;

pub struct SelectableList<'a, T, F>
where
  F: Fn(&T) -> ListItem<'a>,
{
  content: &'a mut StatefulList<T>,
  row_mapper: F,
  highlight_style: Style,
  block: Block<'a>,
}

impl<'a, T, F> SelectableList<'a, T, F>
where
  F: Fn(&T) -> ListItem<'a>,
{
  pub fn new(content: &'a mut StatefulList<T>, row_mapper: F) -> Self {
    Self {
      content,
      row_mapper,
      highlight_style: Style::new().highlight(),
      block: layout_block(),
    }
  }

  pub fn highlight_style(mut self, style: Style) -> Self {
    self.highlight_style = style;
    self
  }

  pub fn block(mut self, block: Block<'a>) -> Self {
    self.block = block;
    self
  }

  fn render_list(self, area: Rect, buf: &mut Buffer) {
    let items: Vec<ListItem<'_>> = self.content.items.iter().map(&self.row_mapper).collect();

    let selectable_list = List::new(items)
      .block(self.block)
      .highlight_style(self.highlight_style);

    StatefulWidget::render(selectable_list, area, buf, &mut self.content.state);
  }
}

impl<'a, T, F> Widget for SelectableList<'a, T, F>
where
  F: Fn(&T) -> ListItem<'a>,
{
  fn render(self, area: Rect, buf: &mut Buffer) {
    self.render_list(area, buf);
  }
}
