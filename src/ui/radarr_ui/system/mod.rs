use crate::ui::styles::default_style;
use std::ops::Sub;

#[cfg(test)]
use crate::ui::ui_test_utils::test_utils::Utc;
#[cfg(not(test))]
use chrono::Utc;
use ratatui::layout::Layout;
use ratatui::text::{Span, Text};
use ratatui::widgets::{Cell, Row};
use ratatui::{
  Frame,
  layout::{Constraint, Rect},
  widgets::ListItem,
};

use crate::app::App;
use crate::models::radarr_models::RadarrTask;
use crate::models::servarr_data::radarr::radarr_data::ActiveRadarrBlock;
use crate::models::servarr_models::QueueEvent;
use crate::ui::radarr_ui::system::system_details_ui::SystemDetailsUi;
use crate::ui::styles::ManagarrStyle;
use crate::ui::utils::{convert_to_minutes_hours_days, style_log_list_item};
use crate::ui::widgets::loading_block::LoadingBlock;
use crate::ui::widgets::managarr_table::ManagarrTable;
use crate::ui::widgets::selectable_list::SelectableList;
use crate::{
  models::Route,
  ui::{DrawUi, utils::title_block},
};

mod system_details_ui;

#[cfg(test)]
#[path = "system_ui_tests.rs"]
mod system_ui_tests;

pub(super) const TASK_TABLE_HEADERS: [&str; 5] = [
  "Name",
  "Interval",
  "Last Execution",
  "Last Duration",
  "Next Execution",
];

pub(super) const TASK_TABLE_CONSTRAINTS: [Constraint; 5] = [
  Constraint::Percentage(30),
  Constraint::Percentage(12),
  Constraint::Percentage(18),
  Constraint::Percentage(18),
  Constraint::Percentage(22),
];

pub(super) struct SystemUi;

impl DrawUi for SystemUi {
  fn accepts(route: Route) -> bool {
    let Route::Radarr(active_radarr_block, _) = route else {
      return false;
    };
    SystemDetailsUi::accepts(route) || active_radarr_block == ActiveRadarrBlock::System
  }

  fn draw(f: &mut Frame<'_>, app: &mut App<'_>, area: Rect) {
    let route = app.get_current_route();
    draw_system_ui_layout(f, app, area);

    if SystemDetailsUi::accepts(route) {
      SystemDetailsUi::draw(f, app, area);
    }
  }
}

fn draw_system_ui_layout(f: &mut Frame<'_>, app: &mut App<'_>, area: Rect) {
  let [activities_area, logs_area] =
    Layout::vertical([Constraint::Ratio(1, 2), Constraint::Ratio(1, 2)]).areas(area);

  let [tasks_area, events_area] =
    Layout::horizontal([Constraint::Ratio(1, 2), Constraint::Ratio(1, 2)]).areas(activities_area);

  draw_tasks(f, app, tasks_area);
  draw_queued_events(f, app, events_area);
  draw_logs(f, app, logs_area);
}

fn draw_tasks(f: &mut Frame<'_>, app: &mut App<'_>, area: Rect) {
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
    .block(title_block("Tasks"))
    .loading(app.is_loading)
    .highlight_rows(false)
    .headers(TASK_TABLE_HEADERS)
    .constraints(TASK_TABLE_CONSTRAINTS);

  f.render_widget(tasks_table, area);
}

pub(super) fn draw_queued_events(f: &mut Frame<'_>, app: &mut App<'_>, area: Rect) {
  let events_row_mapping = |event: &QueueEvent| {
    let queued = convert_to_minutes_hours_days(Utc::now().sub(event.queued).num_minutes());
    let queued_string = if queued != "now" {
      format!("{queued} ago")
    } else {
      queued
    };
    let started_string = if event.started.is_some() {
      let started =
        convert_to_minutes_hours_days(Utc::now().sub(event.started.unwrap()).num_minutes());

      if started != "now" {
        format!("{started} ago")
      } else {
        started
      }
    } else {
      String::new()
    };

    let duration = if event.duration.is_some() {
      event.duration.as_ref().map_or("", |it| &it[..8])
    } else {
      ""
    };

    Row::new(vec![
      Cell::from(event.trigger.clone()),
      Cell::from(event.status.clone()),
      Cell::from(event.command_name.clone()),
      Cell::from(queued_string),
      Cell::from(started_string),
      Cell::from(duration.to_owned()),
    ])
    .primary()
  };
  let events_table = ManagarrTable::new(
    Some(&mut app.data.radarr_data.queued_events),
    events_row_mapping,
  )
  .block(title_block("Queued Events"))
  .loading(app.is_loading)
  .highlight_rows(false)
  .headers(["Trigger", "Status", "Name", "Queued", "Started", "Duration"])
  .constraints([
    Constraint::Percentage(13),
    Constraint::Percentage(13),
    Constraint::Percentage(30),
    Constraint::Percentage(16),
    Constraint::Percentage(14),
    Constraint::Percentage(14),
  ]);

  f.render_widget(events_table, area);
}

fn draw_logs(f: &mut Frame<'_>, app: &mut App<'_>, area: Rect) {
  let block = title_block("Logs");

  if app.data.radarr_data.logs.items.is_empty() {
    f.render_widget(LoadingBlock::new(app.is_loading, block), area);
    return;
  }

  let logs_box = SelectableList::new(&mut app.data.radarr_data.logs, |log| {
    let log_line = log.to_string();
    let level = log_line.split('|').collect::<Vec<&str>>()[1].to_string();

    style_log_list_item(ListItem::new(Text::from(Span::raw(log_line))), level)
  })
  .block(block)
  .highlight_style(default_style());

  f.render_widget(logs_box, area);
}

pub(super) struct TaskProps {
  pub(super) name: String,
  pub(super) interval: String,
  pub(super) last_execution: String,
  pub(super) last_duration: String,
  pub(super) next_execution: String,
}

pub(super) fn extract_task_props(task: &RadarrTask) -> TaskProps {
  let interval = convert_to_minutes_hours_days(task.interval);
  let last_duration = &task.last_duration[..8];
  let next_execution =
    convert_to_minutes_hours_days((task.next_execution - Utc::now()).num_minutes());
  let last_execution =
    convert_to_minutes_hours_days((Utc::now() - task.last_execution).num_minutes());
  let last_execution_string = if last_execution != "now" {
    format!("{last_execution} ago")
  } else {
    last_execution
  };

  TaskProps {
    name: task.name.clone(),
    interval,
    last_execution: last_execution_string,
    last_duration: last_duration.to_owned(),
    next_execution,
  }
}
