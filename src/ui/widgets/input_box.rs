use ratatui::buffer::Buffer;
use ratatui::layout::{Alignment, Constraint, Layout, Rect};
use ratatui::prelude::Text;
use ratatui::style::{Style, Styled, Stylize};
use ratatui::widgets::{Block, Paragraph, Widget};
use ratatui::Frame;

use crate::ui::styles::ManagarrStyle;
use crate::ui::utils::{borderless_block, layout_block};

#[cfg(test)]
#[path = "input_box_tests.rs"]
mod input_box_tests;

pub struct InputBox<'a> {
  content: &'a str,
  offset: usize,
  style: Style,
  block: Block<'a>,
  label: Option<&'a str>,
  cursor_after_string: bool,
  is_highlighted: Option<bool>,
  is_selected: Option<bool>,
}

impl<'a> InputBox<'a> {
  pub fn new(content: &'a str) -> InputBox<'_> {
    InputBox {
      content,
      offset: 0,
      style: Style::new().default(),
      block: layout_block(),
      label: None,
      cursor_after_string: true,
      is_highlighted: None,
      is_selected: None,
    }
  }

  pub fn style<S: Into<Style>>(mut self, style: S) -> InputBox<'a> {
    self.style = style.into();
    self
  }

  pub fn block(mut self, block: Block<'a>) -> InputBox<'a> {
    self.block = block;
    self
  }

  pub fn label(mut self, label: &'a str) -> InputBox<'a> {
    self.label = Some(label);
    self
  }

  pub fn offset(mut self, offset: usize) -> InputBox<'a> {
    self.offset = offset;
    self
  }

  pub fn cursor_after_string(mut self, cursor_after_string: bool) -> InputBox<'a> {
    self.cursor_after_string = cursor_after_string;
    self
  }

  pub fn highlighted(mut self, is_highlighted: bool) -> InputBox<'a> {
    self.is_highlighted = Some(is_highlighted);
    self
  }

  pub fn selected(mut self, is_selected: bool) -> InputBox<'a> {
    self.is_selected = Some(is_selected);
    self
  }

  pub fn is_selected(&self) -> bool {
    self.is_selected.unwrap_or_default()
  }

  pub fn show_cursor(&self, f: &mut Frame<'_>, area: Rect) {
    let area = if self.label.is_some() {
      Layout::horizontal([Constraint::Percentage(48), Constraint::Percentage(48)]).split(area)[1]
    } else {
      area
    };

    if self.cursor_after_string {
      f.set_cursor(
        area.x + (self.content.len() - self.offset) as u16 + 1,
        area.y + 1,
      );
    } else {
      f.set_cursor(area.x + 1u16, area.y + 1);
    }
  }

  fn render_input_box(self, area: Rect, buf: &mut Buffer) {
    let style =
      if matches!(self.is_highlighted, Some(true)) && matches!(self.is_selected, Some(false)) {
        Style::new().system_function().bold()
      } else {
        self.style
      };

    let input_box_paragraph = Paragraph::new(Text::from(self.content))
      .style(style)
      .block(self.block);

    if let Some(label) = self.label {
      let [label_area, text_box_area] =
        Layout::horizontal([Constraint::Percentage(48), Constraint::Percentage(48)]).areas(area);

      Paragraph::new(Text::from(format!("\n{label}: ")))
        .block(borderless_block())
        .alignment(Alignment::Right)
        .primary()
        .render(label_area, buf);
      input_box_paragraph.render(text_box_area, buf);
    } else {
      input_box_paragraph.render(area, buf);
    }
  }
}

impl<'a> Widget for InputBox<'a> {
  fn render(self, area: Rect, buf: &mut Buffer)
  where
    Self: Sized,
  {
    self.render_input_box(area, buf);
  }
}

impl<'a> Styled for InputBox<'a> {
  type Item = InputBox<'a>;

  fn style(&self) -> Style {
    self.style
  }

  fn set_style<S: Into<Style>>(self, style: S) -> Self::Item {
    self.style(style)
  }
}

#[macro_export]
macro_rules! render_selectable_input_box {
  ($input_box:ident, $frame:ident, $area:ident) => {
    if $input_box.is_selected() {
      $input_box.show_cursor($frame, $area);
    }

    $frame.render_widget($input_box, $area);
  };
}
