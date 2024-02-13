use crate::ui::styles::ManagarrStyle;
use crate::ui::utils::{borderless_block, layout_block, style_block_highlight};
use ratatui::buffer::Buffer;
use ratatui::layout::{Alignment, Constraint, Layout, Rect};
use ratatui::prelude::Text;
use ratatui::style::Stylize;
use ratatui::widgets::{Paragraph, Widget};

#[cfg(test)]
#[path = "checkbox_tests.rs"]
mod checkbox_tests;

pub struct Checkbox<'a> {
  label: &'a str,
  is_checked: bool,
  is_highlighted: bool,
}

impl<'a> Checkbox<'a> {
  pub fn new(label: &'a str) -> Checkbox<'a> {
    Checkbox {
      label,
      is_checked: false,
      is_highlighted: false,
    }
  }

  pub fn checked(mut self, is_checked: bool) -> Checkbox<'a> {
    self.is_checked = is_checked;
    self
  }

  pub fn highlighted(mut self, is_selected: bool) -> Checkbox<'a> {
    self.is_highlighted = is_selected;
    self
  }

  fn render_checkbox(self, area: Rect, buf: &mut Buffer) {
    let check = if self.is_checked { "✔" } else { "" };
    let [label_area, checkbox_area] =
      Layout::horizontal([Constraint::Percentage(48), Constraint::Percentage(48)]).areas(area);
    let checkbox_box_area = Rect {
      width: 5,
      ..checkbox_area
    };

    Paragraph::new(Text::from(format!("\n{}: ", self.label)))
      .block(borderless_block())
      .alignment(Alignment::Right)
      .primary()
      .render(label_area, buf);

    Paragraph::new(Text::from(check))
      .block(layout_block())
      .alignment(Alignment::Center)
      .style(style_block_highlight(self.is_highlighted).bold())
      .render(checkbox_box_area, buf);
  }
}

impl<'a> Widget for Checkbox<'a> {
  fn render(self, area: Rect, buf: &mut Buffer) {
    self.render_checkbox(area, buf);
  }
}
