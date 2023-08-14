use std::iter;
use std::rc::Rc;

use tui::backend::Backend;
use tui::layout::{Alignment, Constraint, Rect};
use tui::style::Modifier;
use tui::text::{Line, Span, Text};
use tui::widgets::Paragraph;
use tui::widgets::Row;
use tui::widgets::Table;
use tui::widgets::Tabs;
use tui::widgets::{Block, Wrap};
use tui::widgets::{Clear, List, ListItem};
use tui::Frame;

use crate::app::App;
use crate::models::{HorizontallyScrollableText, Route, StatefulList, StatefulTable, TabState};
use crate::ui::radarr_ui::RadarrUi;
use crate::ui::utils::{
  background_block, borderless_block, centered_rect, horizontal_chunks,
  horizontal_chunks_with_margin, layout_block, layout_block_top_border, layout_button_paragraph,
  layout_button_paragraph_borderless, layout_paragraph_borderless, logo_block, show_cursor,
  style_block_highlight, style_default, style_default_bold, style_failure, style_help,
  style_highlight, style_primary, style_secondary, style_system_function, title_block,
  title_block_centered, vertical_chunks_with_margin,
};

mod radarr_ui;
mod utils;

static HIGHLIGHT_SYMBOL: &str = "=> ";

pub trait DrawUi {
  fn accepts(route: Route) -> bool;
  fn draw<B: Backend>(f: &mut Frame<'_, B>, app: &mut App<'_>, content_rect: Rect);
  fn draw_context_row<B: Backend>(_f: &mut Frame<'_, B>, _app: &App<'_>, _area: Rect) {}
}

pub fn ui<B: Backend>(f: &mut Frame<'_, B>, app: &mut App<'_>) {
  f.render_widget(background_block(), f.size());
  let main_chunks = if !app.error.text.is_empty() {
    let chunks = vertical_chunks_with_margin(
      vec![
        Constraint::Length(3),
        Constraint::Length(3),
        Constraint::Length(10),
        Constraint::Length(0),
      ],
      f.size(),
      1,
    );

    draw_error(f, app, chunks[1]);

    Rc::new([chunks[0], chunks[2], chunks[3]])
  } else {
    vertical_chunks_with_margin(
      vec![
        Constraint::Length(3),
        Constraint::Length(10),
        Constraint::Length(0),
      ],
      f.size(),
      1,
    )
  };

  draw_header_row(f, app, main_chunks[0]);

  if RadarrUi::accepts(*app.get_current_route()) {
    RadarrUi::draw_context_row(f, app, main_chunks[1]);
    RadarrUi::draw(f, app, main_chunks[2]);
  }
}

fn draw_header_row<B: Backend>(f: &mut Frame<'_, B>, app: &mut App<'_>, area: Rect) {
  let chunks =
    horizontal_chunks_with_margin(vec![Constraint::Length(75), Constraint::Min(0)], area, 1);
  let help_text = Text::from(app.server_tabs.get_active_tab_help());

  let titles = app
    .server_tabs
    .tabs
    .iter()
    .map(|tab| Line::from(Span::styled(tab.title, style_default_bold())))
    .collect();
  let tabs = Tabs::new(titles)
    .block(logo_block())
    .highlight_style(style_secondary())
    .select(app.server_tabs.index);
  let help = Paragraph::new(help_text)
    .block(borderless_block())
    .style(style_help())
    .alignment(Alignment::Right);

  f.render_widget(tabs, area);
  f.render_widget(help, chunks[1]);
}

fn draw_error<B: Backend>(f: &mut Frame<'_, B>, app: &mut App<'_>, area: Rect) {
  let block =
    title_block("Error | <esc> to close").style(style_failure().add_modifier(Modifier::BOLD));

  app.error.scroll_left_or_reset(
    area.width as usize,
    true,
    app.tick_count % app.ticks_until_scroll == 0,
  );

  let mut text = Text::from(app.error.to_string());
  text.patch_style(style_failure());

  let paragraph = Paragraph::new(text)
    .block(block)
    .wrap(Wrap { trim: true })
    .style(style_primary());

  f.render_widget(paragraph, area);
}

pub fn draw_popup<B: Backend>(
  f: &mut Frame<'_, B>,
  app: &mut App<'_>,
  popup_fn: impl Fn(&mut Frame<'_, B>, &mut App<'_>, Rect),
  percent_x: u16,
  percent_y: u16,
) {
  let popup_area = centered_rect(percent_x, percent_y, f.size());
  f.render_widget(Clear, popup_area);
  f.render_widget(background_block(), popup_area);
  popup_fn(f, app, popup_area);
}

pub fn draw_popup_ui<B: Backend, T: DrawUi>(
  f: &mut Frame<'_, B>,
  app: &mut App<'_>,
  percent_x: u16,
  percent_y: u16,
) {
  let popup_area = centered_rect(percent_x, percent_y, f.size());
  f.render_widget(Clear, popup_area);
  f.render_widget(background_block(), popup_area);
  T::draw(f, app, popup_area);
}

pub fn draw_popup_over<B: Backend>(
  f: &mut Frame<'_, B>,
  app: &mut App<'_>,
  area: Rect,
  background_fn: impl Fn(&mut Frame<'_, B>, &mut App<'_>, Rect),
  popup_fn: impl Fn(&mut Frame<'_, B>, &mut App<'_>, Rect),
  percent_x: u16,
  percent_y: u16,
) {
  background_fn(f, app, area);

  draw_popup(f, app, popup_fn, percent_x, percent_y);
}

pub fn draw_popup_over_ui<B: Backend, T: DrawUi>(
  f: &mut Frame<'_, B>,
  app: &mut App<'_>,
  area: Rect,
  background_fn: impl Fn(&mut Frame<'_, B>, &mut App<'_>, Rect),
  percent_x: u16,
  percent_y: u16,
) {
  background_fn(f, app, area);

  draw_popup_ui::<B, T>(f, app, percent_x, percent_y);
}

pub fn draw_prompt_popup_over<B: Backend>(
  f: &mut Frame<'_, B>,
  app: &mut App<'_>,
  area: Rect,
  background_fn: impl Fn(&mut Frame<'_, B>, &mut App<'_>, Rect),
  popup_fn: impl Fn(&mut Frame<'_, B>, &mut App<'_>, Rect),
) {
  draw_popup_over(f, app, area, background_fn, popup_fn, 35, 35);
}

pub fn draw_small_popup_over<B: Backend>(
  f: &mut Frame<'_, B>,
  app: &mut App<'_>,
  area: Rect,
  background_fn: impl Fn(&mut Frame<'_, B>, &mut App<'_>, Rect),
  popup_fn: impl Fn(&mut Frame<'_, B>, &mut App<'_>, Rect),
) {
  draw_popup_over(f, app, area, background_fn, popup_fn, 40, 40);
}

pub fn draw_medium_popup_over<B: Backend>(
  f: &mut Frame<'_, B>,
  app: &mut App<'_>,
  area: Rect,
  background_fn: impl Fn(&mut Frame<'_, B>, &mut App<'_>, Rect),
  popup_fn: impl Fn(&mut Frame<'_, B>, &mut App<'_>, Rect),
) {
  draw_popup_over(f, app, area, background_fn, popup_fn, 60, 60);
}

pub fn draw_large_popup_over<B: Backend>(
  f: &mut Frame<'_, B>,
  app: &mut App<'_>,
  area: Rect,
  background_fn: impl Fn(&mut Frame<'_, B>, &mut App<'_>, Rect),
  popup_fn: impl Fn(&mut Frame<'_, B>, &mut App<'_>, Rect),
) {
  draw_popup_over(f, app, area, background_fn, popup_fn, 75, 75);
}

pub fn draw_large_popup_over_background_fn_with_ui<B: Backend, T: DrawUi>(
  f: &mut Frame<'_, B>,
  app: &mut App<'_>,
  area: Rect,
  background_fn: impl Fn(&mut Frame<'_, B>, &mut App<'_>, Rect),
) {
  draw_popup_over_ui::<B, T>(f, app, area, background_fn, 75, 75);
}

pub fn draw_drop_down_popup<B: Backend>(
  f: &mut Frame<'_, B>,
  app: &mut App<'_>,
  area: Rect,
  background_fn: impl Fn(&mut Frame<'_, B>, &mut App<'_>, Rect),
  drop_down_fn: impl Fn(&mut Frame<'_, B>, &mut App<'_>, Rect),
) {
  draw_popup_over(f, app, area, background_fn, drop_down_fn, 20, 30);
}

pub fn draw_error_popup_over<B: Backend>(
  f: &mut Frame<'_, B>,
  app: &mut App<'_>,
  area: Rect,
  message: &str,
  background_fn: fn(&mut Frame<'_, B>, &mut App<'_>, Rect),
) {
  background_fn(f, app, area);
  draw_error_popup(f, message);
}

pub fn draw_error_popup<B: Backend>(f: &mut Frame<'_, B>, message: &str) {
  let prompt_area = centered_rect(25, 8, f.size());
  f.render_widget(Clear, prompt_area);
  f.render_widget(background_block(), prompt_area);

  let error_message = Paragraph::new(Text::from(message))
    .block(title_block_centered("Error").style(style_failure()))
    .style(style_failure().add_modifier(Modifier::BOLD))
    .wrap(Wrap { trim: false })
    .alignment(Alignment::Center);

  f.render_widget(error_message, prompt_area);
}

fn draw_tabs<'a, B: Backend>(
  f: &mut Frame<'_, B>,
  area: Rect,
  title: &str,
  tab_state: &TabState,
) -> (Rect, Block<'a>) {
  let chunks =
    vertical_chunks_with_margin(vec![Constraint::Length(2), Constraint::Min(0)], area, 1);
  let horizontal_chunks = horizontal_chunks_with_margin(
    vec![Constraint::Percentage(10), Constraint::Min(0)],
    area,
    1,
  );
  let block = title_block(title);
  let mut help_text = Text::from(tab_state.get_active_tab_help());
  help_text.patch_style(style_help());

  let titles = tab_state
    .tabs
    .iter()
    .map(|tab_route| Line::from(Span::styled(tab_route.title, style_default_bold())))
    .collect();
  let tabs = Tabs::new(titles)
    .block(block)
    .highlight_style(style_secondary())
    .select(tab_state.index);
  let help = Paragraph::new(help_text)
    .block(borderless_block())
    .alignment(Alignment::Right);

  f.render_widget(tabs, area);
  f.render_widget(help, horizontal_chunks[1]);

  (chunks[1], layout_block_top_border())
}

pub struct TableProps<'a, T> {
  pub content: Option<&'a mut StatefulTable<T>>,
  pub wrapped_content: Option<Option<&'a mut StatefulTable<T>>>,
  pub table_headers: Vec<&'a str>,
  pub constraints: Vec<Constraint>,
  pub help: Option<String>,
}

pub struct ListProps<'a, T> {
  pub content: &'a mut StatefulList<T>,
  pub title: &'static str,
  pub is_loading: bool,
  pub is_popup: bool,
  pub help: Option<String>,
}

fn draw_table<'a, B, T, F>(
  f: &mut Frame<'_, B>,
  content_area: Rect,
  block: Block<'_>,
  table_props: TableProps<'a, T>,
  row_mapper: F,
  is_loading: bool,
  highlight: bool,
) where
  B: Backend,
  F: Fn(&T) -> Row<'a>,
{
  let TableProps {
    content,
    wrapped_content,
    table_headers,
    constraints,
    help,
  } = table_props;

  let content_area = draw_help_and_get_content_rect(f, content_area, help);

  if wrapped_content.is_some() && wrapped_content.as_ref().unwrap().is_some() {
    draw_table_contents(
      f,
      block,
      row_mapper,
      highlight,
      wrapped_content.unwrap().as_mut().unwrap(),
      table_headers,
      &constraints,
      content_area,
    );
  } else if content.is_some() && !content.as_ref().unwrap().items.is_empty() {
    draw_table_contents(
      f,
      block,
      row_mapper,
      highlight,
      content.unwrap(),
      table_headers,
      &constraints,
      content_area,
    );
  } else {
    loading(f, block, content_area, is_loading);
  }
}

fn draw_table_contents<'a, B, T, F>(
  f: &mut Frame<'_, B>,
  block: Block<'_>,
  row_mapper: F,
  highlight: bool,
  content: &mut StatefulTable<T>,
  table_headers: Vec<&str>,
  constraints: &Vec<Constraint>,
  content_area: Rect,
) where
  B: Backend,
  F: Fn(&T) -> Row<'a>,
{
  let rows = content.items.iter().map(row_mapper);

  let headers = Row::new(table_headers)
    .style(style_default_bold())
    .bottom_margin(0);

  let mut table = Table::new(rows).header(headers).block(block);

  if highlight {
    table = table
      .highlight_style(style_highlight())
      .highlight_symbol(HIGHLIGHT_SYMBOL);
  }

  table = table.widths(&constraints);

  f.render_stateful_widget(table, content_area, &mut content.state);
}

pub fn loading<B: Backend>(f: &mut Frame<'_, B>, block: Block<'_>, area: Rect, is_loading: bool) {
  if is_loading {
    let text = "\n\n Loading ...\n\n".to_owned();
    let mut text = Text::from(text);
    text.patch_style(style_system_function());

    let paragraph = Paragraph::new(text)
      .style(style_system_function())
      .block(block);
    f.render_widget(paragraph, area);
  } else {
    f.render_widget(block, area)
  }
}

pub fn draw_prompt_box<B: Backend>(
  f: &mut Frame<'_, B>,
  prompt_area: Rect,
  title: &str,
  prompt: &str,
  yes_no_value: bool,
) {
  draw_prompt_box_with_content(f, prompt_area, title, prompt, None, yes_no_value);
}

pub fn draw_prompt_box_with_content<B: Backend>(
  f: &mut Frame<'_, B>,
  prompt_area: Rect,
  title: &str,
  prompt: &str,
  content: Option<Paragraph<'_>>,
  yes_no_value: bool,
) {
  f.render_widget(title_block_centered(title), prompt_area);

  let chunks = if let Some(content_paragraph) = content {
    let vertical_chunks = vertical_chunks_with_margin(
      vec![
        Constraint::Length(4),
        Constraint::Length(7),
        Constraint::Min(0),
        Constraint::Length(3),
      ],
      prompt_area,
      1,
    );

    f.render_widget(content_paragraph, vertical_chunks[1]);

    Rc::new([vertical_chunks[0], vertical_chunks[2], vertical_chunks[3]])
  } else {
    vertical_chunks_with_margin(
      vec![
        Constraint::Percentage(72),
        Constraint::Min(0),
        Constraint::Length(3),
      ],
      prompt_area,
      1,
    )
  };

  let prompt_paragraph = layout_paragraph_borderless(prompt);
  f.render_widget(prompt_paragraph, chunks[0]);

  let horizontal_chunks = horizontal_chunks(
    vec![Constraint::Percentage(50), Constraint::Percentage(50)],
    chunks[2],
  );

  draw_button(f, horizontal_chunks[0], "Yes", yes_no_value);
  draw_button(f, horizontal_chunks[1], "No", !yes_no_value);
}

pub fn draw_prompt_box_with_checkboxes<B: Backend>(
  f: &mut Frame<'_, B>,
  prompt_area: Rect,
  title: &str,
  prompt: &str,
  checkboxes: Vec<(&str, bool, bool)>,
  highlight_yes_no: bool,
  yes_no_value: bool,
) {
  f.render_widget(title_block_centered(title), prompt_area);
  let mut constraints = vec![
    Constraint::Length(4),
    Constraint::Min(0),
    Constraint::Length(3),
  ];

  constraints.splice(
    1..1,
    iter::repeat(Constraint::Length(3)).take(checkboxes.len()),
  );

  let chunks = vertical_chunks_with_margin(constraints, prompt_area, 1);

  let prompt_paragraph = layout_paragraph_borderless(prompt);
  f.render_widget(prompt_paragraph, chunks[0]);

  for i in 0..checkboxes.len() {
    let (label, is_checked, is_selected) = checkboxes[i];
    draw_checkbox_with_label(f, chunks[i + 1], label, is_checked, is_selected);
  }

  let horizontal_chunks = horizontal_chunks(
    vec![Constraint::Percentage(50), Constraint::Percentage(50)],
    chunks[checkboxes.len() + 2],
  );

  draw_button(
    f,
    horizontal_chunks[0],
    "Yes",
    highlight_yes_no && yes_no_value,
  );
  draw_button(
    f,
    horizontal_chunks[1],
    "No",
    highlight_yes_no && !yes_no_value,
  );
}

pub fn draw_checkbox<B: Backend>(
  f: &mut Frame<'_, B>,
  area: Rect,
  is_checked: bool,
  is_selected: bool,
) {
  let check = if is_checked { "✔" } else { "" };
  let label_paragraph = Paragraph::new(Text::from(check))
    .block(layout_block())
    .alignment(Alignment::Center)
    .style(style_block_highlight(is_selected).add_modifier(Modifier::BOLD));
  let checkbox_area = Rect { width: 5, ..area };

  f.render_widget(label_paragraph, checkbox_area);
}

pub fn draw_checkbox_with_label<B: Backend>(
  f: &mut Frame<'_, B>,
  area: Rect,
  label: &str,
  is_checked: bool,
  is_selected: bool,
) {
  let horizontal_chunks = horizontal_chunks(
    vec![
      Constraint::Percentage(48),
      Constraint::Percentage(48),
      Constraint::Percentage(4),
    ],
    area,
  );

  let label_paragraph = Paragraph::new(Text::from(format!("\n{}: ", label)))
    .block(borderless_block())
    .alignment(Alignment::Right)
    .style(style_primary());

  f.render_widget(label_paragraph, horizontal_chunks[0]);

  draw_checkbox(f, horizontal_chunks[1], is_checked, is_selected);
}

pub fn draw_button<B: Backend>(f: &mut Frame<'_, B>, area: Rect, label: &str, is_selected: bool) {
  let label_paragraph = layout_button_paragraph(is_selected, label, Alignment::Center);

  f.render_widget(label_paragraph, area);
}

pub fn draw_button_with_icon<B: Backend>(
  f: &mut Frame<'_, B>,
  area: Rect,
  label: &str,
  icon: &str,
  is_selected: bool,
) {
  let label_paragraph = layout_button_paragraph_borderless(is_selected, label, Alignment::Left);
  let icon_paragraph = layout_button_paragraph_borderless(is_selected, icon, Alignment::Right);

  let horizontal_chunks = horizontal_chunks_with_margin(
    vec![
      Constraint::Percentage(50),
      Constraint::Percentage(49),
      Constraint::Percentage(1),
    ],
    area,
    1,
  );

  f.render_widget(
    layout_block().style(style_block_highlight(is_selected)),
    area,
  );
  f.render_widget(label_paragraph, horizontal_chunks[0]);
  f.render_widget(icon_paragraph, horizontal_chunks[1]);
}

pub fn draw_drop_down_menu_button<B: Backend>(
  f: &mut Frame<'_, B>,
  area: Rect,
  description: &str,
  selection: &str,
  is_selected: bool,
) {
  let horizontal_chunks = horizontal_chunks(
    vec![
      Constraint::Percentage(48),
      Constraint::Percentage(48),
      Constraint::Percentage(4),
    ],
    area,
  );

  let description_paragraph = Paragraph::new(Text::from(format!("\n{}: ", description)))
    .block(borderless_block())
    .alignment(Alignment::Right)
    .style(style_primary());

  f.render_widget(description_paragraph, horizontal_chunks[0]);

  draw_button_with_icon(f, horizontal_chunks[1], selection, "▼", is_selected);
}

pub fn draw_selectable_list<'a, B: Backend, T>(
  f: &mut Frame<'_, B>,
  area: Rect,
  content: &'a mut StatefulList<T>,
  item_mapper: impl Fn(&T) -> ListItem<'a>,
) {
  let items: Vec<ListItem<'_>> = content.items.iter().map(item_mapper).collect();
  let list = List::new(items)
    .block(layout_block())
    .highlight_style(style_highlight());

  f.render_stateful_widget(list, area, &mut content.state);
}

pub fn draw_list_box<'a, B: Backend, T>(
  f: &mut Frame<'_, B>,
  area: Rect,
  item_mapper: impl Fn(&T) -> ListItem<'a>,
  list_props: ListProps<'a, T>,
) {
  let ListProps {
    content,
    title,
    is_loading,
    is_popup,
    help,
  } = list_props;

  let (content_area, block) = if is_popup {
    f.render_widget(title_block(title), area);
    (
      draw_help_and_get_content_rect(f, area, help),
      borderless_block(),
    )
  } else {
    (area, title_block(title))
  };

  if !content.items.is_empty() {
    let items: Vec<ListItem<'_>> = content.items.iter().map(item_mapper).collect();
    let mut list = List::new(items).block(block);

    if is_popup {
      list = list.highlight_style(style_highlight());
    }

    f.render_stateful_widget(list, content_area, &mut content.state);
  } else {
    loading(f, block, content_area, is_loading);
  }
}

fn draw_help_and_get_content_rect<B: Backend>(
  f: &mut Frame<'_, B>,
  area: Rect,
  help: Option<String>,
) -> Rect {
  if let Some(help_string) = help {
    let chunks =
      vertical_chunks_with_margin(vec![Constraint::Min(0), Constraint::Length(2)], area, 1);

    let mut help_test = Text::from(format!(" {}", help_string));
    help_test.patch_style(style_help());
    let help_paragraph = Paragraph::new(help_test)
      .block(layout_block_top_border())
      .alignment(Alignment::Left);

    f.render_widget(help_paragraph, chunks[1]);

    chunks[0]
  } else {
    area
  }
}

pub fn draw_text_box<B: Backend>(
  f: &mut Frame<'_, B>,
  text_box_area: Rect,
  block_title: Option<&str>,
  block_content: &str,
  offset: usize,
  should_show_cursor: bool,
  is_selected: bool,
) {
  let (block, style) = if let Some(title) = block_title {
    (title_block_centered(title), style_default())
  } else {
    (
      layout_block(),
      if should_show_cursor {
        style_default()
      } else {
        style_block_highlight(is_selected)
      },
    )
  };
  let paragraph = Paragraph::new(Text::from(block_content))
    .style(style)
    .block(block);
  f.render_widget(paragraph, text_box_area);

  if should_show_cursor {
    show_cursor(f, text_box_area, offset, block_content);
  }
}

pub fn draw_text_box_with_label<B: Backend>(
  f: &mut Frame<'_, B>,
  area: Rect,
  label: &str,
  text: &str,
  offset: usize,
  is_selected: bool,
  should_show_cursor: bool,
) {
  let horizontal_chunks = horizontal_chunks(
    vec![
      Constraint::Percentage(48),
      Constraint::Percentage(48),
      Constraint::Percentage(4),
    ],
    area,
  );

  let label_paragraph = Paragraph::new(Text::from(format!("\n{}: ", label)))
    .block(borderless_block())
    .alignment(Alignment::Right)
    .style(style_primary());

  f.render_widget(label_paragraph, horizontal_chunks[0]);

  draw_text_box(
    f,
    horizontal_chunks[1],
    None,
    text,
    offset,
    should_show_cursor,
    is_selected,
  );
}

pub fn draw_input_box_popup<B: Backend>(
  f: &mut Frame<'_, B>,
  input_box_area: Rect,
  box_title: &str,
  box_content: &HorizontallyScrollableText,
) {
  let chunks = vertical_chunks_with_margin(
    vec![
      Constraint::Length(3),
      Constraint::Length(1),
      Constraint::Min(0),
    ],
    input_box_area,
    1,
  );

  draw_text_box(
    f,
    chunks[0],
    Some(box_title),
    &box_content.text,
    *box_content.offset.borrow(),
    true,
    false,
  );

  let help = Paragraph::new("<esc> cancel")
    .style(style_help())
    .alignment(Alignment::Center)
    .block(borderless_block());
  f.render_widget(help, chunks[1]);
}

pub fn draw_error_message_popup<B: Backend>(
  f: &mut Frame<'_, B>,
  error_message_area: Rect,
  error_msg: &str,
) {
  let input = Paragraph::new(error_msg)
    .style(style_failure())
    .alignment(Alignment::Center)
    .block(layout_block());

  f.render_widget(input, error_message_area);
}
