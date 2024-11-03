use crate::ui::styles::ManagarrStyle;
use crate::ui::utils::{layout_block, style_block_highlight};
use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Flex, Layout, Rect};
use ratatui::prelude::{Style, Text, Widget};
use ratatui::style::Styled;
use ratatui::widgets::Paragraph;

#[cfg(test)]
#[path = "button_tests.rs"]
mod button_tests;

#[derive(Default)]
pub struct Button<'a> {
  title: &'a str,
  label: Option<&'a str>,
  icon: Option<&'a str>,
  style: Style,
  is_selected: bool,
}

impl<'a> Button<'a> {
  pub fn title(mut self, title: &'a str) -> Button<'a> {
    self.title = title;
    self
  }

  pub fn label(mut self, label: &'a str) -> Button<'a> {
    self.label = Some(label);
    self
  }

  pub fn icon(mut self, icon: &'a str) -> Button<'a> {
    self.icon = Some(icon);
    self
  }

  pub fn style<S: Into<Style>>(mut self, style: S) -> Button<'a> {
    self.style = style.into();
    self
  }

  pub fn selected(mut self, is_selected: bool) -> Button<'a> {
    self.is_selected = is_selected;
    self
  }

  fn render_button_with_icon(self, area: Rect, buf: &mut Buffer) {
    let [title_area, icon_area] = Layout::horizontal([
      Constraint::Length(self.title.len() as u16),
      Constraint::Percentage(25),
    ])
    .flex(Flex::SpaceBetween)
    .margin(1)
    .areas(area);
    let style = style_block_highlight(self.is_selected);

    if let Some(icon) = self.icon {
      layout_block().style(style).render(area, buf);
      Paragraph::new(Text::from(self.title))
        .left_aligned()
        .style(style)
        .render(title_area, buf);
      Paragraph::new(Text::from(format!("{icon} ")))
        .right_aligned()
        .style(style)
        .render(icon_area, buf);
    }
  }

  fn render_labeled_button(self, area: Rect, buf: &mut Buffer) {
    let [label_area, button_area] =
      Layout::horizontal([Constraint::Percentage(48), Constraint::Percentage(48)]).areas(area);
    let label_paragraph = Paragraph::new(Text::from(format!("\n{}: ", self.label.unwrap())))
      .right_aligned()
      .primary();

    if self.icon.is_some() {
      self.render_button_with_icon(button_area, buf);
      label_paragraph.render(label_area, buf);
    } else {
      self.render_button(button_area, buf);
      label_paragraph.render(label_area, buf);
    }
  }

  fn render_button(self, area: Rect, buf: &mut Buffer) {
    Paragraph::new(Text::from(self.title))
      .block(layout_block())
      .centered()
      .style(style_block_highlight(self.is_selected))
      .render(area, buf);
  }
}

impl<'a> Widget for Button<'a> {
  fn render(self, area: Rect, buf: &mut Buffer)
  where
    Self: Sized,
  {
    if self.label.is_some() {
      self.render_labeled_button(area, buf);
    } else if self.icon.is_some() {
      self.render_button_with_icon(area, buf);
    } else {
      self.render_button(area, buf);
    }
  }
}

impl<'a> Styled for Button<'a> {
  type Item = Button<'a>;

  fn style(&self) -> Style {
    self.style
  }

  fn set_style<S: Into<Style>>(self, style: S) -> Self::Item {
    self.style(style)
  }
}
