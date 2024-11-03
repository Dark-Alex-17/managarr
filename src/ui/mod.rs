use std::sync::atomic::Ordering;

use ratatui::layout::{Constraint, Flex, Layout, Rect};
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
  background_block, borderless_block, centered_rect, logo_block, title_block, title_block_centered,
};
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
  f.render_widget(background_block(), f.area());
  let [header_area, context_area, table_area] = if !app.error.text.is_empty() {
    let [header_area, error_area, context_area, table_area] = Layout::vertical([
      Constraint::Length(3),
      Constraint::Length(3),
      Constraint::Length(10),
      Constraint::Fill(0),
    ])
    .areas(f.area());

    draw_error(f, app, error_area);

    [header_area, context_area, table_area]
  } else {
    Layout::vertical([
      Constraint::Length(3),
      Constraint::Length(10),
      Constraint::Fill(0),
    ])
    .areas(f.area())
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
    .right_aligned();

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

  let paragraph = Paragraph::new(Text::from(app.error.to_string().failure()))
    .block(block)
    .wrap(Wrap { trim: true });

  f.render_widget(paragraph, area);
}

pub fn draw_popup(
  f: &mut Frame<'_>,
  app: &mut App<'_>,
  popup_fn: impl Fn(&mut Frame<'_>, &mut App<'_>, Rect),
  size: Size,
) {
  let (percent_x, percent_y) = size.to_percent();
  let popup_area = centered_rect(percent_x, percent_y, f.area());
  f.render_widget(Clear, popup_area);
  f.render_widget(background_block(), popup_area);
  popup_fn(f, app, popup_area);
}

fn draw_popup_ui<T: DrawUi>(f: &mut Frame<'_>, app: &mut App<'_>, size: Size) {
  let (percent_x, percent_y) = size.to_percent();
  let popup_area = centered_rect(percent_x, percent_y, f.area());
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
    .right_aligned();

  f.render_widget(tabs, tabs_area);
  f.render_widget(help, help_area);

  content_area
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
    .offset(box_content.offset.load(Ordering::SeqCst))
    .block(title_block_centered(box_title));

  input_box.show_cursor(f, text_box_area);
  f.render_widget(input_box, text_box_area);

  let help = Paragraph::new("<esc> cancel")
    .help()
    .centered()
    .block(borderless_block());
  f.render_widget(help, help_area);
}
