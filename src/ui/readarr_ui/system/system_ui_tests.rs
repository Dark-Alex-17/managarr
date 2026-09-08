#[cfg(test)]
mod tests {
  use chrono::DateTime;
  use strum::IntoEnumIterator;

  use crate::app::App;
  use crate::models::HorizontallyScrollableText;
  use crate::models::readarr_models::{ReadarrTask, ReadarrTaskName};
  use crate::models::servarr_data::readarr::readarr_data::{
    ActiveReadarrBlock, SYSTEM_DETAILS_BLOCKS,
  };
  use crate::models::servarr_models::QueueEvent;
  use crate::ui::DrawUi;
  use crate::ui::readarr_ui::system::SystemUi;
  use crate::ui::ui_test_utils::test_utils::render_to_string_with_app;

  #[test]
  fn test_system_ui_accepts() {
    let mut system_ui_blocks = Vec::new();
    system_ui_blocks.push(ActiveReadarrBlock::System);
    system_ui_blocks.extend(SYSTEM_DETAILS_BLOCKS);

    ActiveReadarrBlock::iter().for_each(|active_readarr_block| {
      if system_ui_blocks.contains(&active_readarr_block) {
        assert!(SystemUi::accepts(active_readarr_block.into()));
      } else {
        assert!(!SystemUi::accepts(active_readarr_block.into()));
      }
    });
  }

  mod test_extract_log_level {
    use pretty_assertions::assert_str_eq;

    use crate::ui::readarr_ui::system::extract_log_level;

    #[test]
    fn test_extract_log_level_returns_the_field_after_the_timestamp() {
      let level = extract_log_level("2025-12-16 16:40:59 UTC|INFO|ImportListSyncService|Message");

      assert_str_eq!(level, "INFO");
    }

    #[test]
    fn test_extract_log_level_preserves_case() {
      let level = extract_log_level("2025-12-16 16:41:30 UTC|FaTaL|CommandExecutor|Message");

      assert_str_eq!(level, "FaTaL");
    }

    #[test]
    fn test_extract_log_level_ignores_later_fields() {
      let level = extract_log_level("timestamp|WARN|logger|message|with|extra|pipes");

      assert_str_eq!(level, "WARN");
    }

    #[test]
    fn test_extract_log_level_returns_an_empty_string_when_the_level_is_absent() {
      let level = extract_log_level("2025-12-16 16:41:12 UTC||BookScanService|Message");

      assert_str_eq!(level, "");
    }
  }

  mod snapshot_tests {
    use crate::models::stateful_list::StatefulList;
    use crate::ui::ui_test_utils::test_utils::TerminalSize;
    use rstest::rstest;

    use super::*;

    #[test]
    fn test_system_ui_renders_system_tab_loading() {
      let mut app = App::test_default();
      app.is_loading = true;
      app.push_navigation_stack(ActiveReadarrBlock::System.into());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        SystemUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }

    #[test]
    fn test_system_ui_renders_logs_loading() {
      let mut app = App::test_default_fully_populated();
      populate_system_data(&mut app);
      app.is_loading = true;
      app.push_navigation_stack(ActiveReadarrBlock::System.into());
      app.data.readarr_data.logs = StatefulList::default();

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        SystemUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }

    #[test]
    fn test_system_ui_renders_system_tab_task_and_events_loading() {
      let mut app = App::test_default_fully_populated();
      populate_system_data(&mut app);
      app.push_navigation_stack(ActiveReadarrBlock::System.into());
      app.is_loading = true;

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        SystemUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }

    #[test]
    fn test_system_ui_renders_system_tab() {
      let mut app = App::test_default_fully_populated();
      populate_system_data(&mut app);
      app.push_navigation_stack(ActiveReadarrBlock::System.into());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        SystemUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }

    #[test]
    fn test_system_ui_renders_system_tab_empty() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveReadarrBlock::System.into());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        SystemUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }

    #[rstest]
    fn test_system_details_ui_renders_popups_over_system_ui(
      #[values(
        ActiveReadarrBlock::SystemLogs,
        ActiveReadarrBlock::SystemQueuedEvents,
        ActiveReadarrBlock::SystemTasks,
        ActiveReadarrBlock::SystemTaskStartConfirmPrompt,
        ActiveReadarrBlock::SystemUpdates
      )]
      active_readarr_block: ActiveReadarrBlock,
    ) {
      let mut app = App::test_default_fully_populated();
      populate_system_data(&mut app);
      app.push_navigation_stack(active_readarr_block.into());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        SystemUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(
        format!("popups_over_system_ui_{active_readarr_block}"),
        output
      );
    }
  }

  fn populate_system_data(app: &mut App<'_>) {
    app.data.readarr_data.tasks.set_items(tasks_vec());
    app
      .data
      .readarr_data
      .queued_events
      .set_items(queued_events_vec());
    app.data.readarr_data.logs.set_items(log_lines_vec());
    app.data.readarr_data.log_details.set_items(log_lines_vec());
  }

  fn tasks_vec() -> Vec<ReadarrTask> {
    vec![
      ReadarrTask {
        name: "Backup task".to_owned(),
        task_name: ReadarrTaskName::Backup,
        interval: 120,
        last_execution: DateTime::from(
          DateTime::parse_from_rfc3339("2023-05-20T21:00:00Z").unwrap(),
        ),
        last_duration: "00:00:00.5111547".to_owned(),
        next_execution: DateTime::from(
          DateTime::parse_from_rfc3339("2023-05-20T22:30:00Z").unwrap(),
        ),
      },
      ReadarrTask {
        name: "Rss sync task".to_owned(),
        task_name: ReadarrTaskName::RssSync,
        interval: 15,
        last_execution: DateTime::from(
          DateTime::parse_from_rfc3339("2023-05-20T21:22:00Z").unwrap(),
        ),
        last_duration: "00:00:01.2233445".to_owned(),
        next_execution: DateTime::from(
          DateTime::parse_from_rfc3339("2023-05-20T21:33:00Z").unwrap(),
        ),
      },
      ReadarrTask {
        name: "Housekeeping task".to_owned(),
        task_name: ReadarrTaskName::Housekeeping,
        interval: 1440,
        last_execution: DateTime::from(
          DateTime::parse_from_rfc3339("2023-05-20T17:30:00Z").unwrap(),
        ),
        last_duration: "00:00:09.9887766".to_owned(),
        next_execution: DateTime::from(
          DateTime::parse_from_rfc3339("2023-05-21T15:30:00Z").unwrap(),
        ),
      },
    ]
  }

  fn queued_events_vec() -> Vec<QueueEvent> {
    vec![
      QueueEvent {
        trigger: "manual".to_owned(),
        name: "event name one".to_owned(),
        command_name: "queued event one".to_owned(),
        status: "completed".to_owned(),
        queued: DateTime::from(DateTime::parse_from_rfc3339("2023-05-20T20:30:00Z").unwrap()),
        started: Some(DateTime::from(
          DateTime::parse_from_rfc3339("2023-05-20T21:00:00Z").unwrap(),
        )),
        ended: Some(DateTime::from(
          DateTime::parse_from_rfc3339("2023-05-20T21:29:14Z").unwrap(),
        )),
        duration: Some("00:29:14.1234567".to_owned()),
      },
      QueueEvent {
        trigger: "scheduled".to_owned(),
        name: "event name two".to_owned(),
        command_name: "queued event two".to_owned(),
        status: "started".to_owned(),
        queued: DateTime::from(DateTime::parse_from_rfc3339("2023-05-20T21:15:00Z").unwrap()),
        started: Some(DateTime::from(
          DateTime::parse_from_rfc3339("2023-05-20T21:26:00Z").unwrap(),
        )),
        ended: None,
        duration: Some("01:02:03.7654321".to_owned()),
      },
      QueueEvent {
        trigger: "external".to_owned(),
        name: "event name three".to_owned(),
        command_name: "queued event three".to_owned(),
        status: "queued".to_owned(),
        queued: DateTime::from(DateTime::parse_from_rfc3339("2023-05-20T21:29:00Z").unwrap()),
        started: None,
        ended: None,
        duration: None,
      },
    ]
  }

  fn log_lines_vec() -> Vec<HorizontallyScrollableText> {
    vec![
      "2025-12-16 16:40:59 UTC|INFO|ImportListSyncService|log line alpha".into(),
      "2025-12-16 16:41:12 UTC|WARN|BookScanService|log line beta".into(),
      "2025-12-16 16:41:30 UTC|FATAL|CommandExecutor|log line gamma".into(),
    ]
  }
}
