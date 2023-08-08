use crate::ui::utils::{layout_block_top_border, style_help, style_primary, style_secondary};
use crate::ui::{draw_table, TableProps};
use crate::{
  app::{radarr::ActiveRadarrBlock, App},
  models::Route,
  ui::{
    draw_list_box,
    utils::{horizontal_chunks, style_default, style_failure, title_block, vertical_chunks},
    DrawUi,
  },
};
use chrono::Utc;
use std::ops::Sub;
use tui::layout::Alignment;
use tui::style::Modifier;
use tui::text::{Span, Text};
use tui::widgets::{Cell, Paragraph, Row};
use tui::{
  backend::Backend,
  layout::{Constraint, Rect},
  style::{Color, Style},
  widgets::ListItem,
  Frame,
};

#[cfg(test)]
#[path = "system_ui_tests.rs"]
mod system_ui_tests;

pub(super) struct SystemUi {}

impl DrawUi for SystemUi {
  fn draw<B: Backend>(f: &mut Frame<'_, B>, app: &mut App<'_>, content_rect: Rect) {
    if matches!(
      *app.get_current_route(),
      Route::Radarr(ActiveRadarrBlock::System, _)
    ) {
      draw_system_ui_layout(f, app, content_rect)
    }
  }
}

fn draw_system_ui_layout<B: Backend>(f: &mut Frame<'_, B>, app: &mut App<'_>, area: Rect) {
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
  draw_events(f, app, horizontal_chunks[1]);
  draw_logs(f, app, vertical_chunks[1]);
  draw_help(f, app, vertical_chunks[2]);
}

fn draw_tasks<B: Backend>(f: &mut Frame<'_, B>, app: &mut App<'_>, area: Rect) {
  let block = title_block("Tasks");
  draw_table(
    f,
    area,
    block,
    TableProps {
      content: &mut app.data.radarr_data.tasks,
      table_headers: vec![
        "Name",
        "Interval",
        "Last Execution",
        "Last Duration",
        "Next Execution",
      ],
      constraints: vec![
        Constraint::Percentage(30),
        Constraint::Percentage(12),
        Constraint::Percentage(18),
        Constraint::Percentage(18),
        Constraint::Percentage(22),
      ],
      help: None,
    },
    |task| {
      let interval = convert_to_minutes_hours_days(*task.interval.as_i64().as_ref().unwrap());
      let last_duration = &task.last_duration[..8];
      let next_execution =
        convert_to_minutes_hours_days(task.next_execution.sub(Utc::now()).num_minutes());
      let last_execution =
        convert_to_minutes_hours_days(Utc::now().sub(task.last_execution).num_minutes());
      let last_execution_string = if last_execution != "now" {
        format!("{} ago", last_execution)
      } else {
        last_execution
      };

      Row::new(vec![
        Cell::from(task.name.clone()),
        Cell::from(interval),
        Cell::from(last_execution_string),
        Cell::from(last_duration.to_owned()),
        Cell::from(next_execution),
      ])
      .style(style_primary())
    },
    app.is_loading,
    false,
  );
}

fn draw_events<B: Backend>(f: &mut Frame<'_, B>, app: &mut App<'_>, area: Rect) {
  let block = title_block("Events");
  draw_table(
    f,
    area,
    block,
    TableProps {
      content: &mut app.data.radarr_data.events,
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

      let duration = &event.duration[..8];

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
    &mut app.data.radarr_data.logs,
    "Logs",
    |log| {
      let log_line = if log.exception.is_some() {
        Text::from(Span::raw(format!(
          "{}|{}|{}|{}|{}",
          log.time,
          log.level.as_ref().unwrap().to_uppercase(),
          log.logger.as_ref().unwrap(),
          log.exception_type.as_ref().unwrap(),
          log.exception.as_ref().unwrap()
        )))
      } else {
        Text::from(Span::raw(format!(
          "{}|{}|{}|{}",
          log.time,
          log.level.as_ref().unwrap().to_uppercase(),
          log.logger.as_ref().unwrap(),
          log.message.as_ref().unwrap()
        )))
      };

      ListItem::new(log_line).style(determine_log_style_by_level(log.level.as_ref().unwrap()))
    },
    app.is_loading,
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

fn determine_log_style_by_level(level: &str) -> Style {
  match level.to_lowercase().as_str() {
    "trace" => Style::default().fg(Color::Gray),
    "debug" => Style::default().fg(Color::Blue),
    "info" => style_default(),
    "warn" => style_secondary(),
    "error" => style_failure(),
    "fatal" => style_failure().add_modifier(Modifier::BOLD),
    _ => style_default(),
  }
}

fn convert_to_minutes_hours_days(time: i64) -> String {
  if time < 60 {
    if time == 0 {
      "now".to_owned()
    } else if time == 1 {
      format!("{} minute", time)
    } else {
      format!("{} minutes", time)
    }
  } else if time / 60 < 24 {
    let hours = time / 60;
    if hours == 1 {
      format!("{} hour", hours)
    } else {
      format!("{} hours", hours)
    }
  } else {
    let days = time / (60 * 24);
    if days == 1 {
      format!("{} day", days)
    } else {
      format!("{} days", days)
    }
  }
}
