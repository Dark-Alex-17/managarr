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
  use crate::ui::readarr_ui::system::system_details_ui::SystemDetailsUi;
  use crate::ui::ui_test_utils::test_utils::render_to_string_with_app;

  #[test]
  fn test_system_details_ui_accepts() {
    ActiveReadarrBlock::iter().for_each(|active_readarr_block| {
      if SYSTEM_DETAILS_BLOCKS.contains(&active_readarr_block) {
        assert!(SystemDetailsUi::accepts(active_readarr_block.into()));
      } else {
        assert!(!SystemDetailsUi::accepts(active_readarr_block.into()));
      }
    });
  }

  mod snapshot_tests {
    use super::*;
    use crate::models::ScrollableText;
    use crate::ui::ui_test_utils::test_utils::TerminalSize;
    use rstest::rstest;

    #[rstest]
    fn test_system_details_ui_popups(
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
        SystemDetailsUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(format!("popup_{active_readarr_block}"), output);
    }

    #[test]
    fn test_system_details_ui_task_start_confirm_prompt_names_the_selected_task() {
      let mut app = App::test_default_fully_populated();
      populate_system_data(&mut app);
      app.data.readarr_data.tasks.select_index(Some(2));
      app.push_navigation_stack(ActiveReadarrBlock::SystemTaskStartConfirmPrompt.into());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        SystemDetailsUi::draw(f, app, f.area());
      });

      assert!(output.contains("Do you want to manually start this task: Housekeeping task?"));
      insta::assert_snapshot!(output);
    }

    #[test]
    fn test_system_details_ui_updates_popup_is_loading_when_updates_are_present() {
      let mut app = App::test_default_fully_populated();
      app.is_loading = true;
      app.push_navigation_stack(ActiveReadarrBlock::SystemUpdates.into());

      assert!(!app.data.readarr_data.updates.get_text().is_empty());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        SystemDetailsUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }

    #[rstest]
    fn test_system_details_ui_loading(
      #[values(
        ActiveReadarrBlock::SystemLogs,
        ActiveReadarrBlock::SystemQueuedEvents,
        ActiveReadarrBlock::SystemTasks,
        ActiveReadarrBlock::SystemUpdates
      )]
      active_readarr_block: ActiveReadarrBlock,
    ) {
      let mut app = App::test_default();
      app.is_loading = true;
      app.push_navigation_stack(active_readarr_block.into());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        SystemDetailsUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(format!("loading_{active_readarr_block}"), output);
    }

    #[test]
    fn test_system_details_ui_updates_popup_loading_when_empty() {
      let mut app = App::test_default_fully_populated();
      app.is_loading = true;
      app.push_navigation_stack(ActiveReadarrBlock::SystemUpdates.into());
      app.data.readarr_data.updates = ScrollableText::with_string("".to_string());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        SystemDetailsUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }

    #[rstest]
    fn test_system_details_ui_popups_empty(
      #[values(
        ActiveReadarrBlock::SystemLogs,
        ActiveReadarrBlock::SystemQueuedEvents,
        ActiveReadarrBlock::SystemTasks
      )]
      active_readarr_block: ActiveReadarrBlock,
    ) {
      let mut app = App::test_default();
      app.push_navigation_stack(active_readarr_block.into());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        SystemDetailsUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(format!("empty_{active_readarr_block}"), output);
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
