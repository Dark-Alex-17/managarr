use ratatui::layout::{Alignment, Rect};
use ratatui::text::{Span, Text};
use ratatui::widgets::{Cell, ListItem, Paragraph, Row};
use ratatui::Frame;

use crate::app::context_clues::{build_context_clue_string, BARE_POPUP_CONTEXT_CLUES};
use crate::app::radarr::radarr_context_clues::SYSTEM_TASKS_CONTEXT_CLUES;
use crate::app::App;
use crate::models::radarr_models::RadarrTask;
use crate::models::servarr_data::radarr::radarr_data::{ActiveRadarrBlock, SYSTEM_DETAILS_BLOCKS};
use crate::models::Route;
use crate::ui::radarr_ui::system::{
  draw_queued_events, extract_task_props, TASK_TABLE_CONSTRAINTS, TASK_TABLE_HEADERS,
};
use crate::ui::styles::ManagarrStyle;
use crate::ui::utils::{borderless_block, style_log_list_item, title_block};
use crate::ui::widgets::confirmation_prompt::ConfirmationPrompt;
use crate::ui::widgets::loading_block::LoadingBlock;
use crate::ui::widgets::managarr_table::ManagarrTable;
use crate::ui::widgets::popup::{Popup, Size};
use crate::ui::widgets::selectable_list::SelectableList;
use crate::ui::{draw_popup, DrawUi};

#[cfg(test)]
#[path = "system_details_ui_tests.rs"]
mod system_details_ui_tests;

pub(super) struct SystemDetailsUi;

impl DrawUi for SystemDetailsUi {
  fn accepts(route: Route) -> bool {
    if let Route::Radarr(active_radarr_block, _) = route {
      return SYSTEM_DETAILS_BLOCKS.contains(&active_radarr_block);
    }

    false
  }

  fn draw(f: &mut Frame<'_>, app: &mut App<'_>, _area: Rect) {
    if let Route::Radarr(active_radarr_block, _) = app.get_current_route() {
      match active_radarr_block {
        ActiveRadarrBlock::SystemLogs => {
          draw_logs_popup(f, app);
        }
        ActiveRadarrBlock::SystemTasks | ActiveRadarrBlock::SystemTaskStartConfirmPrompt => {
          draw_popup(f, app, draw_tasks_popup, Size::Large)
        }
        ActiveRadarrBlock::SystemQueuedEvents => {
          draw_popup(f, app, draw_queued_events, Size::Medium)
        }
        ActiveRadarrBlock::SystemUpdates => {
          draw_updates_popup(f, app);
        }
        _ => (),
      }
    }
  }
}

fn draw_logs_popup(f: &mut Frame<'_>, app: &mut App<'_>) {
  let block = title_block("Log Details");
  let help_footer = format!(
    "<↑↓←→> scroll | {}",
    build_context_clue_string(&BARE_POPUP_CONTEXT_CLUES)
  );

  if app.data.radarr_data.log_details.items.is_empty() {
    let loading = LoadingBlock::new(app.is_loading, borderless_block());
    let popup = Popup::new(loading)
      .size(Size::Large)
      .block(block)
      .footer(&help_footer);

    f.render_widget(popup, f.area());
    return;
  }

  let logs_list = SelectableList::new(&mut app.data.radarr_data.log_details, |log| {
    let log_line = log.to_string();
    let level = log.text.split('|').collect::<Vec<&str>>()[1].to_string();

    style_log_list_item(ListItem::new(Text::from(Span::raw(log_line))), level)
  })
  .block(borderless_block());
  let popup = Popup::new(logs_list)
    .size(Size::Large)
    .block(block)
    .footer(&help_footer);

  f.render_widget(popup, f.area());
}

fn draw_tasks_popup(f: &mut Frame<'_>, app: &mut App<'_>, area: Rect) {
  let help_footer = Some(build_context_clue_string(&SYSTEM_TASKS_CONTEXT_CLUES));
  let tasks_row_mapping = |task: &RadarrTask| {
    let task_props = extract_task_props(task);

    Row::new(vec![
      Cell::from(task_props.name),
      Cell::from(task_props.interval),
      Cell::from(task_props.last_execution),
      Cell::from(task_props.last_duration),
      Cell::from(task_props.next_execution),
    ])
    .primary()
  };
  let tasks_table = ManagarrTable::new(Some(&mut app.data.radarr_data.tasks), tasks_row_mapping)
    .loading(app.is_loading)
    .margin(1)
    .footer(help_footer)
    .footer_alignment(Alignment::Center)
    .headers(TASK_TABLE_HEADERS)
    .constraints(TASK_TABLE_CONSTRAINTS);

  f.render_widget(title_block("Tasks"), area);
  f.render_widget(tasks_table, area);

  if matches!(
    app.get_current_route(),
    Route::Radarr(ActiveRadarrBlock::SystemTaskStartConfirmPrompt, _)
  ) {
    let prompt = format!(
      "Do you want to manually start this task: {}?",
      app.data.radarr_data.tasks.current_selection().name
    );
    let confirmation_prompt = ConfirmationPrompt::new()
      .title("Start Task")
      .prompt(&prompt)
      .yes_no_value(app.data.radarr_data.prompt_confirm);

    f.render_widget(
      Popup::new(confirmation_prompt).size(Size::MediumPrompt),
      f.area(),
    );
  }
}

fn draw_updates_popup(f: &mut Frame<'_>, app: &mut App<'_>) {
  let help_footer = format!(
    "<↑↓> scroll | {}",
    build_context_clue_string(&BARE_POPUP_CONTEXT_CLUES)
  );
  let updates = app.data.radarr_data.updates.get_text();
  let block = title_block("Updates");

  if !updates.is_empty() {
    let updates_paragraph = Paragraph::new(Text::from(updates))
      .block(borderless_block())
      .scroll((app.data.radarr_data.updates.offset, 0));
    let popup = Popup::new(updates_paragraph)
      .size(Size::Large)
      .block(block)
      .footer(&help_footer);

    f.render_widget(popup, f.area());
  } else {
    let loading = LoadingBlock::new(app.is_loading, borderless_block());
    let popup = Popup::new(loading)
      .size(Size::Large)
      .block(block)
      .footer(&help_footer);

    f.render_widget(popup, f.area());
  }
}
