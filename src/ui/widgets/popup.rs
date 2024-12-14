use crate::ui::styles::ManagarrStyle;
use crate::ui::utils::{background_block, centered_rect, layout_block_top_border};
use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::prelude::Text;
use ratatui::widgets::{Block, Clear, Paragraph, Widget};

#[cfg(test)]
#[path = "popup_tests.rs"]
mod popup_tests;

pub enum Size {
  SmallPrompt,
  MediumPrompt,
  LargePrompt,
  WideLargePrompt,
  Message,
  NarrowMessage,
  LargeMessage,
  InputBox,
  Dropdown,
  Small,
  Medium,
  Large,
  XLarge,
  XXLarge,
  Long,
}

impl Size {
  pub fn to_percent(&self) -> (u16, u16) {
    match self {
      Size::SmallPrompt => (20, 20),
      Size::MediumPrompt => (37, 37),
      Size::LargePrompt => (45, 45),
      Size::WideLargePrompt => (70, 50),
      Size::Message => (25, 8),
      Size::NarrowMessage => (50, 20),
      Size::LargeMessage => (25, 25),
      Size::InputBox => (30, 13),
      Size::Dropdown => (20, 30),
      Size::Small => (40, 40),
      Size::Medium => (60, 60),
      Size::Large => (75, 75),
      Size::XLarge => (83, 83),
      Size::XXLarge => (90, 90),
      Size::Long => (65, 75),
    }
  }
}

pub struct Popup<'a, T: Widget> {
  widget: T,
  percent_x: u16,
  percent_y: u16,
  block: Option<Block<'a>>,
  footer: Option<&'a str>,
}

impl<'a, T: Widget> Popup<'a, T> {
  pub fn new(widget: T) -> Self {
    Self {
      widget,
      percent_x: 0,
      percent_y: 0,
      block: None,
      footer: None,
    }
  }

  pub fn size(mut self, size: Size) -> Self {
    let (percent_x, percent_y) = size.to_percent();
    self.percent_x = percent_x;
    self.percent_y = percent_y;
    self
  }

  pub fn dimensions(mut self, percent_x: u16, percent_y: u16) -> Self {
    self.percent_x = percent_x;
    self.percent_y = percent_y;
    self
  }

  pub fn block(mut self, block: Block<'a>) -> Self {
    self.block = Some(block);
    self
  }

  pub fn footer(mut self, footer: &'a str) -> Self {
    self.footer = Some(footer);
    self
  }

  fn render_popup(self, area: Rect, buf: &mut Buffer) {
    let mut popup_area = centered_rect(self.percent_x, self.percent_y, area);
    let height = if popup_area.height < 3 {
      3
    } else {
      popup_area.height
    };
    popup_area = Rect {
      height,
      ..popup_area
    };
    Clear.render(popup_area, buf);
    background_block().render(popup_area, buf);

    if let Some(block) = self.block {
      block.render(popup_area, buf);
    }

    let content_area = if let Some(footer) = self.footer {
      let [content_area, help_footer_area] =
        Layout::vertical([Constraint::Fill(0), Constraint::Length(2)])
          .margin(1)
          .areas(popup_area);

      Paragraph::new(Text::from(format!(" {footer}").help()))
        .block(layout_block_top_border())
        .left_aligned()
        .render(help_footer_area, buf);

      content_area
    } else {
      popup_area
    };

    self.widget.render(content_area, buf);
  }
}

impl<'a, T: Widget> Widget for Popup<'a, T> {
  fn render(self, area: Rect, buf: &mut Buffer) {
    self.render_popup(area, buf);
  }
}
