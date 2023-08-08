use crate::ui::utils::{style_primary, style_secondary};
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
use chrono::DateTime;
use std::ops::Sub;
use tui::style::Modifier;
use tui::text::{Span, Text};
use tui::widgets::{Cell, Row};
use tui::{
  backend::Backend,
  layout::{Constraint, Rect},
  style::{Color, Style},
  widgets::ListItem,
  Frame,
};

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
  let vertical_chunks =
    vertical_chunks(vec![Constraint::Ratio(1, 2), Constraint::Ratio(1, 2)], area);

  let horizontal_chunks = horizontal_chunks(
    vec![Constraint::Ratio(1, 2), Constraint::Ratio(1, 2)],
    vertical_chunks[0],
  );

  draw_tasks(f, app, horizontal_chunks[0]);
  f.render_widget(title_block("Queue"), horizontal_chunks[1]);
  draw_logs(f, app, vertical_chunks[1]);
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
        "Next Duration",
      ],
      constraints: vec![
        Constraint::Percentage(30),
        Constraint::Percentage(12),
        Constraint::Percentage(16),
        Constraint::Percentage(16),
        Constraint::Percentage(16),
      ],
      help: None,
    },
    |task| {
      let interval = format!("{} hours", task.interval.as_u64().as_ref().unwrap() / 60);
      let last_duration = &task.last_duration[..8];
      let next_execution = task.next_execution.sub(DateTime::default()).num_minutes();
      let next_execution_string = if next_execution > 60 {
        format!("{} hours", next_execution / 60)
      } else {
        format!("{} minutes", next_execution)
      };
      let last_execution = task.last_execution.sub(DateTime::default()).num_minutes();
      let last_execution_string = if last_execution > 60 {
        format!("{} hours", last_execution / 60)
      } else {
        format!("{} minutes", last_execution)
      };

      Row::new(vec![
        Cell::from(task.name.clone()),
        Cell::from(interval),
        Cell::from(last_execution_string),
        Cell::from(last_duration.to_owned()),
        Cell::from(next_execution_string),
      ])
      .style(style_primary())
    },
    app.is_loading,
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
