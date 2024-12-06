use crate::models::stateful_table::StatefulTable;
use crate::ui::styles::ManagarrStyle;
use crate::ui::utils::{centered_rect, layout_block_top_border, title_block_centered};
use crate::ui::widgets::loading_block::LoadingBlock;
use crate::ui::widgets::popup::Popup;
use crate::ui::widgets::selectable_list::SelectableList;
use crate::ui::HIGHLIGHT_SYMBOL;
use ratatui::buffer::Buffer;
use ratatui::layout::{Alignment, Constraint, Layout, Position, Rect};
use ratatui::prelude::{Style, Stylize, Text};
use ratatui::widgets::{Block, ListItem, Paragraph, Row, StatefulWidget, Table, Widget, WidgetRef};
use ratatui::Frame;
use std::fmt::Debug;
use std::sync::atomic::Ordering;

use super::input_box_popup::InputBoxPopup;
use super::message::Message;
use super::popup::Size;

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
  is_searching: bool,
  search_produced_empty_results: bool,
  is_filtering: bool,
  filter_produced_empty_results: bool,
  search_box_content_length: usize,
  search_box_offset: usize,
  filter_box_content_length: usize,
  filter_box_offset: usize,
}

impl<'a, T, F> ManagarrTable<'a, T, F>
where
  F: Fn(&T) -> Row<'a>,
  T: Clone + PartialEq + Eq + Debug,
{
  pub fn new(content: Option<&'a mut StatefulTable<T>>, row_mapper: F) -> Self {
    let mut managarr_table = Self {
      content: None,
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
      is_searching: false,
      search_produced_empty_results: false,
      is_filtering: false,
      filter_produced_empty_results: false,
      search_box_content_length: 0,
      search_box_offset: 0,
      filter_box_content_length: 0,
      filter_box_offset: 0,
    };

    if let Some(content) = content.as_ref() {
      if let Some(search) = content.search.as_ref() {
        managarr_table.search_box_content_length = search.text.len();
        managarr_table.search_box_offset = search.offset.load(Ordering::SeqCst);
      } else if let Some(filter) = content.filter.as_ref() {
        managarr_table.filter_box_content_length = filter.text.len();
        managarr_table.filter_box_offset = filter.offset.load(Ordering::SeqCst);
      }
    }

    managarr_table.content = content;
    managarr_table
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

  pub fn searching(mut self, is_searching: bool) -> Self {
    self.is_searching = is_searching;
    self
  }

  pub fn search_produced_empty_results(mut self, no_search_results: bool) -> Self {
    self.search_produced_empty_results = no_search_results;
    self
  }

  pub fn filtering(mut self, is_filtering: bool) -> Self {
    self.is_filtering = is_filtering;
    self
  }

  pub fn filter_produced_empty_results(mut self, no_filter_results: bool) -> Self {
    self.filter_produced_empty_results = no_filter_results;
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
            .row_highlight_style(Style::new().highlight())
            .highlight_symbol(HIGHLIGHT_SYMBOL);
        }

        StatefulWidget::render(table, table_area, buf, table_state);

        if content.sort.is_some() && self.is_sorting {
          let selectable_list = SelectableList::new(content.sort.as_mut().unwrap(), |item| {
            ListItem::new(Text::from(item.name))
          });
          Popup::new(selectable_list)
            .dimensions(20, 50)
            .render(table_area, buf);
        }

        if self.is_searching {
          let box_content = &content.search.as_ref().unwrap();
          InputBoxPopup::new(&box_content.text)
            .offset(box_content.offset.load(Ordering::SeqCst))
            .block(title_block_centered("Search"))
            .render_ref(table_area, buf);
        }

        if self.is_filtering {
          let box_content = &content.filter.as_ref().unwrap();
          InputBoxPopup::new(&box_content.text)
            .offset(box_content.offset.load(Ordering::SeqCst))
            .block(title_block_centered("Filter"))
            .render_ref(table_area, buf);
        }

        if self.search_produced_empty_results {
          Popup::new(Message::new("No items found matching search"))
            .size(Size::Message)
            .render(table_area, buf);
        }

        if self.filter_produced_empty_results {
          Popup::new(Message::new("The given filter produced empty results"))
            .size(Size::Message)
            .render(table_area, buf);
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

  pub fn show_cursor(&self, f: &mut Frame<'_>, area: Rect) {
    let mut draw_cursor = |length: usize, offset: usize| {
      let table_area = if self.footer.is_some() {
        let [content_area, _] = Layout::vertical([Constraint::Fill(0), Constraint::Length(2)])
          .margin(self.margin)
          .areas(area);
        content_area
      } else {
        area
      };
      let popup_area = Rect {
        height: 7,
        ..centered_rect(30, 20, table_area)
      };
      let [text_box_area, _] = Layout::vertical([Constraint::Length(3), Constraint::Length(1)])
        .margin(1)
        .areas(popup_area);
      f.set_cursor_position(Position {
        x: text_box_area.x + (length - offset) as u16 + 1,
        y: text_box_area.y + 1,
      });
    };

    if self.is_searching {
      draw_cursor(self.search_box_content_length, self.search_box_offset);
    } else if self.is_filtering {
      draw_cursor(self.filter_box_content_length, self.filter_box_offset);
    }
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
