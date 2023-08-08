use std::ops::Sub;

use chrono::Utc;
use tui::layout::Alignment;
use tui::text::{Span, Text};
use tui::widgets::{Cell, Paragraph, Row};
use tui::{
  backend::Backend,
  layout::{Constraint, Rect},
  widgets::ListItem,
  Frame,
};

use crate::app::App;
use crate::models::radarr_models::Task;
use crate::models::servarr_data::radarr::radarr_data::ActiveRadarrBlock;
use crate::ui::radarr_ui::radarr_ui_utils::{
  convert_to_minutes_hours_days, determine_log_style_by_level,
};
use crate::ui::radarr_ui::system::system_details_ui::SystemDetailsUi;
use crate::ui::utils::{layout_block_top_border, style_help, style_primary};
use crate::ui::{draw_table, ListProps, TableProps};
use crate::{
  models::Route,
  ui::{
    draw_list_box,
    utils::{horizontal_chunks, title_block, vertical_chunks},
    DrawUi,
  },
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

pub(super) struct SystemUi {}

impl DrawUi for SystemUi {
  fn accepts(route: Route) -> bool {
    if let Route::Radarr(active_radarr_block, _) = route {
      return SystemDetailsUi::accepts(route) || active_radarr_block == ActiveRadarrBlock::System;
    }

    false
  }

  fn draw<B: Backend>(f: &mut Frame<'_, B>, app: &mut App<'_>, content_rect: Rect) {
    let route = *app.get_current_route();

    match route {
      _ if SystemDetailsUi::accepts(route) => SystemDetailsUi::draw(f, app, content_rect),
      _ if matches!(route, Route::Radarr(ActiveRadarrBlock::System, _)) => {
        draw_system_ui_layout(f, app, content_rect)
      }
      _ => (),
    }
  }
}

pub(super) fn draw_system_ui_layout<B: Backend>(
  f: &mut Frame<'_, B>,
  app: &mut App<'_>,
  area: Rect,
) {
  let vertical_chunks = vertical_chunks(
    vec![
      Constraint::Ratio(1, 2),
      Constraint::Ratio(1, 2),
      Constraint::Min(2),
    ],
    area,
  );

  let horizontal_chunks = horizontal_chunks(
    vec![Constraint::Ratio(1, 2), Constraint::Ratio(1, 2)],
    vertical_chunks[0],
  );

  draw_tasks(f, app, horizontal_chunks[0]);
  draw_queued_events(f, app, horizontal_chunks[1]);
  draw_logs(f, app, vertical_chunks[1]);
  draw_help(f, app, vertical_chunks[2]);
}

fn draw_tasks<B: Backend>(f: &mut Frame<'_, B>, app: &mut App<'_>, area: Rect) {
  draw_table(
    f,
    area,
    title_block("Tasks"),
    TableProps {
      content: &mut app.data.radarr_data.tasks,
      table_headers: TASK_TABLE_HEADERS.to_vec(),
      constraints: TASK_TABLE_CONSTRAINTS.to_vec(),
      help: None,
    },
    |task| {
      let task_props = extract_task_props(task);

      Row::new(vec![
        Cell::from(task_props.name),
        Cell::from(task_props.interval),
        Cell::from(task_props.last_execution),
        Cell::from(task_props.last_duration),
        Cell::from(task_props.next_execution),
      ])
      .style(style_primary())
    },
    app.is_loading,
    false,
  );
}

pub(super) fn draw_queued_events<B: Backend>(f: &mut Frame<'_, B>, app: &mut App<'_>, area: Rect) {
  draw_table(
    f,
    area,
    title_block("Queued Events"),
    TableProps {
      content: &mut app.data.radarr_data.queued_events,
      table_headers: vec!["Trigger", "Status", "Name", "Queued", "Started", "Duration"],
      constraints: vec![
        Constraint::Percentage(13),
        Constraint::Percentage(13),
        Constraint::Percentage(30),
        Constraint::Percentage(16),
        Constraint::Percentage(14),
        Constraint::Percentage(14),
      ],
      help: None,
    },
    |event| {
      let queued = convert_to_minutes_hours_days(Utc::now().sub(event.queued).num_minutes());
      let queued_string = if queued != "now" {
        format!("{} ago", queued)
      } else {
        queued
      };
      let started_string = if event.started.is_some() {
        let started =
          convert_to_minutes_hours_days(Utc::now().sub(event.started.unwrap()).num_minutes());

        if started != "now" {
          format!("{} ago", started)
        } else {
          started
        }
      } else {
        String::new()
      };

      let duration = if event.duration.is_some() {
        &event.duration.as_ref().unwrap()[..8]
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
      .style(style_primary())
    },
    app.is_loading,
    false,
  );
}

fn draw_logs<B: Backend>(f: &mut Frame<'_, B>, app: &mut App<'_>, area: Rect) {
  draw_list_box(
    f,
    area,
    |log| {
      let log_line = log.to_string();
      let level = log_line.split('|').collect::<Vec<&str>>()[1];
      let style = determine_log_style_by_level(level);

      ListItem::new(Text::from(Span::raw(log_line))).style(style)
    },
    ListProps {
      content: &mut app.data.radarr_data.logs,
      title: "Logs",
      is_loading: app.is_loading,
      is_popup: false,
      help: None,
    },
  );
}

fn draw_help<B: Backend>(f: &mut Frame<'_, B>, app: &mut App<'_>, area: Rect) {
  let mut help_text = Text::from(format!(
    " {}",
    app
      .data
      .radarr_data
      .main_tabs
      .get_active_tab_contextual_help()
      .unwrap()
  ));
  help_text.patch_style(style_help());
  let help_paragraph = Paragraph::new(help_text)
    .block(layout_block_top_border())
    .alignment(Alignment::Left);

  f.render_widget(help_paragraph, area);
}

pub(super) struct TaskProps {
  pub(super) name: String,
  pub(super) interval: String,
  pub(super) last_execution: String,
  pub(super) last_duration: String,
  pub(super) next_execution: String,
}

pub(super) fn extract_task_props(task: &Task) -> TaskProps {
  let interval = convert_to_minutes_hours_days(*task.interval.as_i64().as_ref().unwrap());
  let last_duration = &task.last_duration[..8];
  let next_execution =
    convert_to_minutes_hours_days((task.next_execution - Utc::now()).num_minutes());
  let last_execution =
    convert_to_minutes_hours_days((Utc::now() - task.last_execution).num_minutes());
  let last_execution_string = if last_execution != "now" {
    format!("{} ago", last_execution)
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
