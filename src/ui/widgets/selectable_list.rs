use crate::models::stateful_list::StatefulList;
use crate::ui::styles::ManagarrStyle;
use crate::ui::utils::layout_block;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::prelude::Widget;
use ratatui::style::Style;
use ratatui::widgets::{List, ListItem, StatefulWidget};

pub struct SelectableList<'a, T, F>
where
  F: Fn(&T) -> ListItem<'a>,
{
  content: &'a mut StatefulList<T>,
  row_mapper: F,
}

impl<'a, T, F> SelectableList<'a, T, F>
where
  F: Fn(&T) -> ListItem<'a>,
{
  pub fn new(content: &'a mut StatefulList<T>, row_mapper: F) -> Self {
    Self {
      content,
      row_mapper,
    }
  }

  fn render_list(self, area: Rect, buf: &mut Buffer) {
    let items: Vec<ListItem<'_>> = self.content.items.iter().map(&self.row_mapper).collect();

    let selectable_list = List::new(items)
      .block(layout_block())
      .highlight_style(Style::new().highlight());
    
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
