use std::iter;

use ratatui::layout::{Alignment, Constraint, Flex, Layout, Rect};
use ratatui::style::{Style, Stylize};
use ratatui::text::{Line, Text};
use ratatui::widgets::Clear;
use ratatui::widgets::Paragraph;
use ratatui::widgets::Tabs;
use ratatui::widgets::Wrap;
use ratatui::Frame;

use crate::app::App;
use crate::models::{HorizontallyScrollableText, Route, TabState};
use crate::ui::radarr_ui::RadarrUi;
use crate::ui::styles::ManagarrStyle;
use crate::ui::utils::{
  background_block, borderless_block, centered_rect, layout_paragraph_borderless, logo_block,
  title_block, title_block_centered,
};
use crate::ui::widgets::button::Button;
use crate::ui::widgets::checkbox::Checkbox;
use crate::ui::widgets::input_box::InputBox;
use crate::ui::widgets::popup::Size;

mod radarr_ui;
mod styles;
mod utils;
mod widgets;

static HIGHLIGHT_SYMBOL: &str = "=> ";

pub trait DrawUi {
  fn accepts(route: Route) -> bool;
  fn draw(f: &mut Frame<'_>, app: &mut App<'_>, area: Rect);
  fn draw_context_row(_f: &mut Frame<'_>, _app: &App<'_>, _area: Rect) {}
}

pub fn ui(f: &mut Frame<'_>, app: &mut App<'_>) {
  f.render_widget(background_block(), f.size());
  let [header_area, context_area, table_area] = if !app.error.text.is_empty() {
    let [header_area, error_area, context_area, table_area] = Layout::vertical([
      Constraint::Length(3),
      Constraint::Length(3),
      Constraint::Length(10),
      Constraint::Fill(0),
    ])
    .areas(f.size());

    draw_error(f, app, error_area);

    [header_area, context_area, table_area]
  } else {
    Layout::vertical([
      Constraint::Length(3),
      Constraint::Length(10),
      Constraint::Fill(0),
    ])
    .areas(f.size())
  };

  draw_header_row(f, app, header_area);

  if RadarrUi::accepts(*app.get_current_route()) {
    RadarrUi::draw_context_row(f, app, context_area);
    RadarrUi::draw(f, app, table_area);
  }
}

fn draw_header_row(f: &mut Frame<'_>, app: &mut App<'_>, area: Rect) {
  f.render_widget(logo_block(), area);

  let [tabs_area, help_area] = Layout::horizontal([Constraint::Min(25), Constraint::Min(25)])
    .flex(Flex::SpaceBetween)
    .margin(1)
    .areas(area);
  let help_text = Text::from(app.server_tabs.get_active_tab_help().help());

  let titles = app
    .server_tabs
    .tabs
    .iter()
    .map(|tab| Line::from(tab.title.bold()));
  let tabs = Tabs::new(titles)
    .block(borderless_block())
    .highlight_style(Style::new().secondary())
    .select(app.server_tabs.index);
  let help = Paragraph::new(help_text)
    .block(borderless_block())
    .alignment(Alignment::Right);

  f.render_widget(tabs, tabs_area);
  f.render_widget(help, help_area);
}

fn draw_error(f: &mut Frame<'_>, app: &mut App<'_>, area: Rect) {
  let block = title_block("Error | <esc> to close").failure().bold();

  app.error.scroll_left_or_reset(
    area.width as usize,
    true,
    app.tick_count % app.ticks_until_scroll == 0,
  );

  let paragraph = Paragraph::new(Text::from(app.error.to_string()).failure())
    .block(block)
    .wrap(Wrap { trim: true })
    .primary();

  f.render_widget(paragraph, area);
}

pub fn draw_popup(
  f: &mut Frame<'_>,
  app: &mut App<'_>,
  popup_fn: impl Fn(&mut Frame<'_>, &mut App<'_>, Rect),
  size: Size,
) {
  let (percent_x, percent_y) = size.to_percent();
  let popup_area = centered_rect(percent_x, percent_y, f.size());
  f.render_widget(Clear, popup_area);
  f.render_widget(background_block(), popup_area);
  popup_fn(f, app, popup_area);
}

fn draw_popup_ui<T: DrawUi>(f: &mut Frame<'_>, app: &mut App<'_>, size: Size) {
  let (percent_x, percent_y) = size.to_percent();
  let popup_area = centered_rect(percent_x, percent_y, f.size());
  f.render_widget(Clear, popup_area);
  f.render_widget(background_block(), popup_area);
  T::draw(f, app, popup_area);
}

pub fn draw_popup_over(
  f: &mut Frame<'_>,
  app: &mut App<'_>,
  area: Rect,
  background_fn: impl Fn(&mut Frame<'_>, &mut App<'_>, Rect),
  popup_fn: impl Fn(&mut Frame<'_>, &mut App<'_>, Rect),
  size: Size,
) {
  background_fn(f, app, area);

  draw_popup(f, app, popup_fn, size);
}

pub fn draw_popup_over_ui<T: DrawUi>(
  f: &mut Frame<'_>,
  app: &mut App<'_>,
  area: Rect,
  background_fn: impl Fn(&mut Frame<'_>, &mut App<'_>, Rect),
  size: Size,
) {
  background_fn(f, app, area);

  draw_popup_ui::<T>(f, app, size);
}

fn draw_tabs(f: &mut Frame<'_>, area: Rect, title: &str, tab_state: &TabState) -> Rect {
  f.render_widget(title_block(title), area);

  let [header_area, content_area] = Layout::vertical([Constraint::Length(1), Constraint::Fill(0)])
    .margin(1)
    .areas(area);
  let [tabs_area, help_area] =
    Layout::horizontal([Constraint::Percentage(45), Constraint::Fill(0)]).areas(header_area);

  let titles = tab_state
    .tabs
    .iter()
    .map(|tab_route| Line::from(tab_route.title.bold()));
  let tabs = Tabs::new(titles)
    .block(borderless_block())
    .highlight_style(Style::new().secondary())
    .select(tab_state.index);
  let help = Paragraph::new(Text::from(tab_state.get_active_tab_help().help()))
    .block(borderless_block())
    .alignment(Alignment::Right);

  f.render_widget(tabs, tabs_area);
  f.render_widget(help, help_area);

  content_area
}

pub fn draw_prompt_box(
  f: &mut Frame<'_>,
  area: Rect,
  title: &str,
  prompt: &str,
  yes_no_value: bool,
) {
  draw_prompt_box_with_content(f, area, title, prompt, None, yes_no_value);
}

pub fn draw_prompt_box_with_content(
  f: &mut Frame<'_>,
  area: Rect,
  title: &str,
  prompt: &str,
  content: Option<Paragraph<'_>>,
  yes_no_value: bool,
) {
  f.render_widget(title_block_centered(title), area);

  let [prompt_area, buttons_area] = if let Some(content_paragraph) = content {
    let [prompt_area, content_area, _, buttons_area] = Layout::vertical([
      Constraint::Length(4),
      Constraint::Length(7),
      Constraint::Fill(0),
      Constraint::Length(3),
    ])
    .margin(1)
    .areas(area);

    f.render_widget(content_paragraph, content_area);

    [prompt_area, buttons_area]
  } else {
    let [prompt_area, _, buttons_area] = Layout::vertical([
      Constraint::Percentage(72),
      Constraint::Fill(0),
      Constraint::Length(3),
    ])
    .margin(1)
    .areas(area);

    [prompt_area, buttons_area]
  };

  let prompt_paragraph = layout_paragraph_borderless(prompt);
  f.render_widget(prompt_paragraph, prompt_area);

  let [yes_area, no_area] =
    Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
      .areas(buttons_area);

  let yes_button = Button::new().title("Yes").selected(yes_no_value);
  let no_button = Button::new().title("No").selected(!yes_no_value);

  f.render_widget(yes_button, yes_area);
  f.render_widget(no_button, no_area);
}

pub fn draw_prompt_box_with_checkboxes(
  f: &mut Frame<'_>,
  area: Rect,
  title: &str,
  prompt: &str,
  checkboxes: Vec<(&str, bool, bool)>,
  highlight_yes_no: bool,
  yes_no_value: bool,
) {
  let mut constraints = vec![
    Constraint::Length(4),
    Constraint::Fill(0),
    Constraint::Length(3),
  ];
  constraints.splice(
    1..1,
    iter::repeat(Constraint::Length(3)).take(checkboxes.len()),
  );
  let chunks = Layout::vertical(constraints).margin(1).split(area);
  let prompt_paragraph = layout_paragraph_borderless(prompt);

  for i in 0..checkboxes.len() {
    let (label, is_checked, is_highlighted) = checkboxes[i];
    let checkbox = Checkbox::new(label)
      .checked(is_checked)
      .highlighted(is_highlighted);
    f.render_widget(checkbox, chunks[i + 1]);
  }

  let [yes_area, no_area] =
    Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
      .areas(chunks[checkboxes.len() + 2]);

  let yes_button = Button::new()
    .title("Yes")
    .selected(yes_no_value && highlight_yes_no);
  let no_button = Button::new()
    .title("No")
    .selected(!yes_no_value && highlight_yes_no);

  f.render_widget(title_block_centered(title), area);
  f.render_widget(prompt_paragraph, chunks[0]);
  f.render_widget(yes_button, yes_area);
  f.render_widget(no_button, no_area);
}

pub fn draw_input_box_popup(
  f: &mut Frame<'_>,
  area: Rect,
  box_title: &str,
  box_content: &HorizontallyScrollableText,
) {
  let [text_box_area, help_area] = Layout::vertical([Constraint::Length(3), Constraint::Length(1)])
    .margin(1)
    .areas(area);

  let input_box = InputBox::new(&box_content.text)
    .offset(*box_content.offset.borrow())
    .block(title_block_centered(box_title));

  input_box.show_cursor(f, text_box_area);
  f.render_widget(input_box, text_box_area);

  let help = Paragraph::new("<esc> cancel")
    .help()
    .alignment(Alignment::Center)
    .block(borderless_block());
  f.render_widget(help, help_area);
}
