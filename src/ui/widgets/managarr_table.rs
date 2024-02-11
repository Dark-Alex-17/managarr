use crate::models::StatefulTable;
use crate::ui::styles::ManagarrStyle;
use crate::ui::utils::layout_block_top_border;
use crate::ui::widgets::loading_block::LoadingBlock;
use crate::ui::HIGHLIGHT_SYMBOL;
use ratatui::buffer::Buffer;
use ratatui::layout::{Alignment, Constraint, Layout, Rect};
use ratatui::prelude::{Style, Stylize, Text};
use ratatui::widgets::{Block, Paragraph, Row, StatefulWidget, Table, Widget};

pub struct ManagarrTable<'a, T, F>
where
  F: Fn(&T) -> Row<'a>,
{
  content: Option<&'a mut StatefulTable<T>>,
  table_headers: Vec<Text<'a>>,
  constraints: Vec<Constraint>,
  row_mapper: F,
  footer: Option<String>,
  footer_alignment: Alignment,
  block: Block<'a>,
  margin: u16,
  is_loading: bool,
  highlight_rows: bool,
}

impl<'a, T, F> ManagarrTable<'a, T, F>
where
  F: Fn(&T) -> Row<'a>,
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
    }
  }

  pub fn headers<I>(mut self, headers: I) -> Self
  where
    I: IntoIterator,
    I::Item: Into<Text<'a>>,
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

  pub fn highlight_rows(mut self, hightlight_rows: bool) -> Self {
    self.highlight_rows = hightlight_rows;
    self
  }

  fn render_table(&mut self, area: Rect, buf: &mut Buffer) {
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

    if let Some(ref mut content) = self.content {
      if !content.items.is_empty() {
        let rows = content.items.iter().map(&self.row_mapper);

        let headers = Row::new(self.table_headers.clone())
          .default()
          .bold()
          .bottom_margin(0);

        let mut table = Table::new(rows, &self.constraints)
          .header(headers)
          .block(self.block.clone());

        if self.highlight_rows {
          table = table
            .highlight_style(Style::new().highlight())
            .highlight_symbol(HIGHLIGHT_SYMBOL);
        }

        StatefulWidget::render(table, table_area, buf, &mut content.state);
      } else {
        loading_block.render(table_area, buf);
      }
    } else {
      loading_block.render(table_area, buf);
    }
  }
}

impl<'a, T, F> Widget for ManagarrTable<'a, T, F>
where
  F: Fn(&T) -> Row<'a>,
{
  fn render(mut self, area: Rect, buf: &mut Buffer) {
    self.render_table(area, buf);
  }
}
