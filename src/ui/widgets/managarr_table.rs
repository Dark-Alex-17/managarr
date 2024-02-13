use crate::models::stateful_table::StatefulTable;
use crate::ui::styles::ManagarrStyle;
use crate::ui::utils::layout_block_top_border;
use crate::ui::widgets::loading_block::LoadingBlock;
use crate::ui::widgets::popup::Popup;
use crate::ui::widgets::selectable_list::SelectableList;
use crate::ui::HIGHLIGHT_SYMBOL;
use ratatui::buffer::Buffer;
use ratatui::layout::{Alignment, Constraint, Layout, Rect};
use ratatui::prelude::{Style, Stylize, Text};
use ratatui::widgets::{Block, ListItem, Paragraph, Row, StatefulWidget, Table, Widget};
use std::fmt::Debug;

#[cfg(test)]
#[path = "managarr_table_tests.rs"]
mod managarr_table_tests;

pub struct ManagarrTable<'a, T, F>
where
  F: Fn(&T) -> Row<'a>,
  T: Clone + PartialEq + Eq + Debug,
{
  content: Option<&'a mut StatefulTable<T>>,
  table_headers: Vec<String>,
  constraints: Vec<Constraint>,
  row_mapper: F,
  footer: Option<String>,
  footer_alignment: Alignment,
  block: Block<'a>,
  margin: u16,
  is_loading: bool,
  highlight_rows: bool,
  is_sorting: bool,
}

impl<'a, T, F> ManagarrTable<'a, T, F>
where
  F: Fn(&T) -> Row<'a>,
  T: Clone + PartialEq + Eq + Debug,
{
  pub fn new(content: Option<&'a mut StatefulTable<T>>, row_mapper: F) -> Self {
    Self {
      content,
      table_headers: Vec::new(),
      constraints: Vec::new(),
      row_mapper,
      footer: None,
      footer_alignment: Alignment::Left,
      block: Block::new(),
      margin: 0,
      is_loading: false,
      highlight_rows: true,
      is_sorting: false,
    }
  }

  pub fn headers<I>(mut self, headers: I) -> Self
  where
    I: IntoIterator,
    I::Item: Into<String>,
  {
    self.table_headers = headers.into_iter().map(Into::into).collect();
    self
  }

  pub fn constraints<I>(mut self, constraints: I) -> Self
  where
    I: IntoIterator,
    I::Item: Into<Constraint>,
  {
    self.constraints = constraints.into_iter().map(Into::into).collect();
    self
  }

  pub fn footer(mut self, footer: Option<String>) -> Self {
    self.footer = footer;
    self
  }

  pub fn footer_alignment(mut self, alignment: Alignment) -> Self {
    self.footer_alignment = alignment;
    self
  }

  pub fn block(mut self, block: Block<'a>) -> Self {
    self.block = block;
    self
  }

  pub fn margin(mut self, margin: u16) -> Self {
    self.margin = margin;
    self
  }

  pub fn loading(mut self, is_loading: bool) -> Self {
    self.is_loading = is_loading;
    self
  }

  pub fn highlight_rows(mut self, highlight_rows: bool) -> Self {
    self.highlight_rows = highlight_rows;
    self
  }

  pub fn sorting(mut self, is_sorting: bool) -> Self {
    self.is_sorting = is_sorting;
    self
  }

  fn render_table(self, area: Rect, buf: &mut Buffer) {
    let table_headers = self.parse_headers();
    let table_area = if let Some(ref footer) = self.footer {
      let [content_area, footer_area] =
        Layout::vertical([Constraint::Fill(0), Constraint::Length(2)])
          .margin(self.margin)
          .areas(area);

      Paragraph::new(Text::from(format!(" {footer}").help()))
        .block(layout_block_top_border())
        .alignment(self.footer_alignment)
        .render(footer_area, buf);

      content_area
    } else {
      area
    };
    let loading_block = LoadingBlock::new(self.is_loading, self.block.clone());

    if let Some(content) = self.content {
      let (table_contents, table_state) = if content.filtered_items.is_some() {
        (
          content.filtered_items.as_ref().unwrap(),
          content.filtered_state.as_mut().unwrap(),
        )
      } else {
        (&content.items, &mut content.state)
      };
      if !table_contents.is_empty() {
        let rows = table_contents.iter().map(&self.row_mapper);

        let headers = Row::new(table_headers).default().bold().bottom_margin(0);

        let mut table = Table::new(rows, &self.constraints)
          .header(headers)
          .block(self.block);

        if self.highlight_rows {
          table = table
            .highlight_style(Style::new().highlight())
            .highlight_symbol(HIGHLIGHT_SYMBOL);
        }

        StatefulWidget::render(table, table_area, buf, table_state);

        if content.sort.is_some() && self.is_sorting {
          let selectable_list = SelectableList::new(content.sort.as_mut().unwrap(), |item| {
            ListItem::new(Text::from(item.name))
          });
          Popup::new(selectable_list, 20, 50).render(table_area, buf);
        }
      } else {
        loading_block.render(table_area, buf);
      }
    } else {
      loading_block.render(table_area, buf);
    }
  }

  fn parse_headers(&self) -> Vec<Text<'a>> {
    if let Some(ref content) = self.content {
      if let Some(ref sort_list) = content.sort {
        if !self.is_sorting {
          let mut new_headers = self.table_headers.clone();
          let idx = sort_list.state.selected().unwrap_or(0);
          let direction = if content.sort_asc { " ▲" } else { " ▼" };
          new_headers[idx].push_str(direction);

          return new_headers.into_iter().map(Text::from).collect();
        }
      }
    }

    self
      .table_headers
      .clone()
      .into_iter()
      .map(Text::from)
      .collect()
  }
}

impl<'a, T, F> Widget for ManagarrTable<'a, T, F>
where
  F: Fn(&T) -> Row<'a>,
  T: Clone + PartialEq + Eq + Debug,
{
  fn render(self, area: Rect, buf: &mut Buffer) {
    self.render_table(area, buf);
  }
}
