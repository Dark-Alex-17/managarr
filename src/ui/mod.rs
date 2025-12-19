use std::cell::Cell;
use std::sync::atomic::Ordering;

use ratatui::Frame;
use ratatui::layout::{Constraint, Flex, Layout, Rect};
use ratatui::style::{Style, Stylize};
use ratatui::text::{Line, Text};
use ratatui::widgets::Paragraph;
use ratatui::widgets::Tabs;
use ratatui::widgets::Wrap;
use ratatui::widgets::{Clear, Row};
use sonarr_ui::SonarrUi;
use utils::layout_block;

use crate::app::App;
use crate::models::servarr_models::KeybindingItem;
use crate::models::{HorizontallyScrollableText, Route, TabState};
use crate::ui::radarr_ui::RadarrUi;
use crate::ui::styles::ManagarrStyle;
use crate::ui::theme::Theme;
use crate::ui::utils::{
  background_block, borderless_block, centered_rect, logo_block, title_block, title_block_centered,
  unstyled_title_block,
};
use crate::ui::widgets::input_box::InputBox;
use crate::ui::widgets::managarr_table::ManagarrTable;
use crate::ui::widgets::popup::Size;

mod builtin_themes;
mod radarr_ui;
mod sonarr_ui;
mod styles;
pub mod theme;
#[cfg(test)]
mod ui_property_tests;
#[cfg(test)]
pub mod ui_test_utils;
#[cfg(test)]
mod ui_tests;
mod utils;
mod widgets;

static HIGHLIGHT_SYMBOL: &str = "=> ";
thread_local! {
  pub static THEME: Cell<Theme> = Cell::new(Theme::default());
}

pub trait DrawUi {
  fn accepts(route: Route) -> bool;
  fn draw(f: &mut Frame<'_>, app: &mut App<'_>, area: Rect);
  fn draw_context_row(_f: &mut Frame<'_>, _app: &App<'_>, _area: Rect) {}
}

pub fn ui(f: &mut Frame<'_>, app: &mut App<'_>) {
  app.on_ui_scroll_tick();
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

  match app.get_current_route() {
    route if RadarrUi::accepts(route) => {
      RadarrUi::draw_context_row(f, app, context_area);
      RadarrUi::draw(f, app, table_area);
    }
    route if SonarrUi::accepts(route) => {
      SonarrUi::draw_context_row(f, app, context_area);
      SonarrUi::draw(f, app, table_area);
    }
    _ => (),
  }

  if app.keymapping_table.is_some() {
    draw_help_popup(f, app);
  }
}

fn draw_header_row(f: &mut Frame<'_>, app: &mut App<'_>, area: Rect) {
  f.render_widget(logo_block(), area);

  let [tabs_area, help_area] = Layout::horizontal([Constraint::Min(25), Constraint::Min(25)])
    .flex(Flex::SpaceBetween)
    .margin(1)
    .areas(area);
  let help_text = Text::from("<?> to open help".help());

  let titles = app
    .server_tabs
    .tabs
    .iter()
    .map(|tab| Line::from(tab.title.clone().bold()));
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
  let block = unstyled_title_block("Error | <esc> to close")
    .failure()
    .bold();

  app
    .error
    .scroll_left_or_reset(area.width as usize, true, app.ui_scroll_tick_count == 0);

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

pub fn draw_help_popup(f: &mut Frame<'_>, app: &mut App<'_>) {
  let (percent_x, percent_y) = Size::LongNarrowTable.to_percent();
  let table_area = centered_rect(percent_x, percent_y, f.area());
  let keymap_row_mapping = |keymap: &KeybindingItem| {
    Row::new(vec![
      ratatui::widgets::Cell::from(keymap.key.clone()),
      ratatui::widgets::Cell::from(keymap.alt_key.clone()),
      ratatui::widgets::Cell::from(keymap.desc.clone()),
    ])
    .primary()
  };
  let keymapping_table = ManagarrTable::new(
    Some(app.keymapping_table.as_mut().unwrap()),
    keymap_row_mapping,
  )
  .block(title_block("Keybindings"))
  .loading(app.is_loading)
  .headers(["Key", "Alt Key", "Description"])
  .constraints([
    Constraint::Ratio(1, 3),
    Constraint::Ratio(1, 3),
    Constraint::Ratio(1, 3),
  ]);
  f.render_widget(Clear, table_area);
  f.render_widget(background_block(), table_area);
  f.render_widget(keymapping_table, table_area);
}

fn draw_tabs(f: &mut Frame<'_>, area: Rect, title: &str, tab_state: &TabState) -> Rect {
  if title.is_empty() {
    f.render_widget(layout_block().default(), area);
  } else {
    f.render_widget(title_block(title), area);
  }

  let [header_area, content_area] = Layout::vertical([Constraint::Length(1), Constraint::Fill(0)])
    .margin(1)
    .areas(area);

  let titles = tab_state
    .tabs
    .iter()
    .map(|tab_route| Line::from(tab_route.title.clone().bold()));
  let tabs = Tabs::new(titles)
    .block(borderless_block())
    .highlight_style(Style::new().secondary())
    .select(tab_state.index);

  f.render_widget(tabs, header_area);

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
