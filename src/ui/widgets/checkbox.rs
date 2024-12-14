use crate::ui::styles::ManagarrStyle;
use crate::ui::utils::{borderless_block, layout_block, style_block_highlight};
use derive_setters::Setters;
use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::prelude::Text;
use ratatui::style::Stylize;
use ratatui::widgets::{Paragraph, Widget};

#[derive(PartialEq, Debug, Copy, Clone, Setters)]
pub struct Checkbox<'a> {
  #[setters(skip)]
  label: &'a str,
  #[setters(rename = "checked")]
  is_checked: bool,
  #[setters(rename = "highlighted")]
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
      .right_aligned()
      .primary()
      .render(label_area, buf);

    Paragraph::new(Text::from(check))
      .block(layout_block())
      .centered()
      .style(style_block_highlight(self.is_highlighted).bold())
      .render(checkbox_box_area, buf);
  }
}

impl<'a> Widget for Checkbox<'a> {
  fn render(self, area: Rect, buf: &mut Buffer) {
    self.render_checkbox(area, buf);
  }
}
