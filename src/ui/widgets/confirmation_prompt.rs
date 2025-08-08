use crate::app::context_clues::{build_context_clue_string, CONFIRMATION_PROMPT_CONTEXT_CLUES};
use crate::ui::styles::ManagarrStyle;
use crate::ui::utils::{layout_paragraph_borderless, title_block_centered};
use crate::ui::widgets::button::Button;
use crate::ui::widgets::checkbox::Checkbox;
use derive_setters::Setters;
use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Flex, Layout, Rect};
use ratatui::text::Text;
use ratatui::widgets::{Paragraph, Widget};
use std::iter;

#[cfg(test)]
#[path = "confirmation_prompt_tests.rs"]
mod confirmation_prompt_tests;

#[derive(Setters)]
pub struct ConfirmationPrompt<'a> {
  title: &'a str,
  prompt: &'a str,
  #[setters(strip_option)]
  content: Option<Paragraph<'a>>,
  #[setters(strip_option)]
  checkboxes: Option<Vec<Checkbox<'a>>>,
  yes_no_value: bool,
  yes_no_highlighted: bool,
}

impl ConfirmationPrompt<'_> {
  pub fn new() -> Self {
    Self {
      title: "",
      prompt: "",
      content: None,
      checkboxes: None,
      yes_no_value: false,
      yes_no_highlighted: true,
    }
  }

  fn render_confirmation_prompt_with_checkboxes(self, area: Rect, buf: &mut Buffer) {
    title_block_centered(self.title).render(area, buf);
    let help_text =
      Text::from(build_context_clue_string(&CONFIRMATION_PROMPT_CONTEXT_CLUES).help());
    let help_paragraph = Paragraph::new(help_text).centered();

    if let Some(checkboxes) = self.checkboxes {
      let mut constraints = vec![
        Constraint::Length(4),
        Constraint::Fill(1),
        Constraint::Length(3),
        Constraint::Length(1),
      ];
      constraints.splice(
        1..1,
        iter::repeat_n(Constraint::Length(3), checkboxes.len()),
      );
      let chunks = Layout::vertical(constraints).margin(1).split(area);
      let [yes_area, no_area] =
        Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
          .areas(chunks[checkboxes.len() + 2]);

      layout_paragraph_borderless(self.prompt).render(chunks[0], buf);
      help_paragraph.render(chunks[checkboxes.len() + 3], buf);

      checkboxes
        .into_iter()
        .enumerate()
        .for_each(|(i, checkbox)| {
          checkbox.render(chunks[i + 1], buf);
        });

      Button::new()
        .title("Yes")
        .selected(self.yes_no_value && self.yes_no_highlighted)
        .render(yes_area, buf);
      Button::new()
        .title("No")
        .selected(!self.yes_no_value && self.yes_no_highlighted)
        .render(no_area, buf);
    }
  }

  fn render_confirmation_prompt(self, area: Rect, buf: &mut Buffer) {
    title_block_centered(self.title).render(area, buf);
    let help_text =
      Text::from(build_context_clue_string(&CONFIRMATION_PROMPT_CONTEXT_CLUES).help());
    let help_paragraph = Paragraph::new(help_text).centered();

    let [prompt_area, buttons_area] = if let Some(content_paragraph) = self.content {
      let [prompt_area, content_area, _, buttons_area, help_area] = Layout::vertical([
        Constraint::Length(4),
        Constraint::Length(7),
        Constraint::Fill(1),
        Constraint::Length(3),
        Constraint::Length(1),
      ])
      .margin(1)
      .areas(area);

      content_paragraph.render(content_area, buf);
      help_paragraph.render(help_area, buf);

      [prompt_area, buttons_area]
    } else {
      let [prompt_area, buttons_area, _, help_area] = Layout::vertical([
        Constraint::Percentage(72),
        Constraint::Length(3),
        Constraint::Fill(0),
        Constraint::Min(1),
      ])
      .margin(1)
      .flex(Flex::SpaceBetween)
      .areas(area);

      help_paragraph.render(help_area, buf);

      [prompt_area, buttons_area]
    };

    layout_paragraph_borderless(self.prompt).render(prompt_area, buf);

    let [yes_area, no_area] =
      Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
        .areas(buttons_area);

    Button::new()
      .title("Yes")
      .selected(self.yes_no_value)
      .render(yes_area, buf);
    Button::new()
      .title("No")
      .selected(!self.yes_no_value)
      .render(no_area, buf);
  }
}

impl Widget for ConfirmationPrompt<'_> {
  fn render(self, area: Rect, buf: &mut Buffer) {
    if self.checkboxes.is_some() {
      self.render_confirmation_prompt_with_checkboxes(area, buf);
    } else {
      self.render_confirmation_prompt(area, buf);
    }
  }
}
