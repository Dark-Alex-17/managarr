#[cfg(test)]
mod tests {
  use pretty_assertions::{assert_eq, assert_str_eq};
  use strum::IntoEnumIterator;

  use crate::app::key_binding::DEFAULT_KEYBINDINGS;
  use crate::app::App;
  use crate::event::Key;
  use crate::handlers::sonarr_handlers::system::system_details_handler::SystemDetailsHandler;
  use crate::handlers::KeyEventHandler;
  use crate::models::servarr_data::sonarr::sonarr_data::{
    ActiveSonarrBlock, SYSTEM_DETAILS_BLOCKS,
  };
  use crate::models::servarr_models::QueueEvent;
  use crate::models::sonarr_models::{SonarrTask, SonarrTaskName};
  use crate::models::{HorizontallyScrollableText, ScrollableText};

  mod test_handle_scroll_up_and_down {
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    use crate::models::{HorizontallyScrollableText, ScrollableText};
    use crate::{simple_stateful_iterable_vec, test_iterable_scroll};

    use super::*;

    test_iterable_scroll!(
      test_log_details_scroll,
      SystemDetailsHandler,
      sonarr_data,
      log_details,
      simple_stateful_iterable_vec!(HorizontallyScrollableText, String, text),
      ActiveSonarrBlock::SystemLogs,
      None,
      text
    );

    #[rstest]
    fn test_log_details_scroll_no_op_when_not_ready(
      #[values(DEFAULT_KEYBINDINGS.up.key, DEFAULT_KEYBINDINGS.down.key)] key: Key,
    ) {
      let mut app = App::default();
      app.push_navigation_stack(ActiveSonarrBlock::System.into());
      app.is_loading = true;
      app
        .data
        .sonarr_data
        .log_details
        .set_items(simple_stateful_iterable_vec!(
          HorizontallyScrollableText,
          String,
          text
        ));

      SystemDetailsHandler::new(key, &mut app, ActiveSonarrBlock::SystemLogs, None).handle();

      assert_str_eq!(
        app.data.sonarr_data.log_details.current_selection().text,
        "Test 1"
      );

      SystemDetailsHandler::new(key, &mut app, ActiveSonarrBlock::SystemLogs, None).handle();

      assert_str_eq!(
        app.data.sonarr_data.log_details.current_selection().text,
        "Test 1"
      );
    }

    #[rstest]
    fn test_tasks_scroll(
      #[values(DEFAULT_KEYBINDINGS.up.key, DEFAULT_KEYBINDINGS.down.key)] key: Key,
    ) {
      let mut app = App::default();
      app.push_navigation_stack(ActiveSonarrBlock::System.into());
      app.data.sonarr_data.updates = ScrollableText::with_string("Test 1\nTest 2".to_owned());
      app
        .data
        .sonarr_data
        .tasks
        .set_items(simple_stateful_iterable_vec!(SonarrTask, String, name));

      SystemDetailsHandler::new(key, &mut app, ActiveSonarrBlock::SystemTasks, None).handle();

      assert_str_eq!(
        app.data.sonarr_data.tasks.current_selection().name,
        "Test 2"
      );

      SystemDetailsHandler::new(key, &mut app, ActiveSonarrBlock::SystemTasks, None).handle();

      assert_str_eq!(
        app.data.sonarr_data.tasks.current_selection().name,
        "Test 1"
      );
    }

    #[rstest]
    fn test_tasks_scroll_no_op_when_not_ready(
      #[values(DEFAULT_KEYBINDINGS.up.key, DEFAULT_KEYBINDINGS.down.key)] key: Key,
    ) {
      let mut app = App::default();
      app.push_navigation_stack(ActiveSonarrBlock::System.into());
      app.is_loading = true;
      app.data.sonarr_data.updates = ScrollableText::with_string("Test 1\nTest 2".to_owned());
      app
        .data
        .sonarr_data
        .tasks
        .set_items(simple_stateful_iterable_vec!(SonarrTask, String, name));

      SystemDetailsHandler::new(key, &mut app, ActiveSonarrBlock::SystemTasks, None).handle();

      assert_str_eq!(
        app.data.sonarr_data.tasks.current_selection().name,
        "Test 1"
      );

      SystemDetailsHandler::new(key, &mut app, ActiveSonarrBlock::SystemTasks, None).handle();

      assert_str_eq!(
        app.data.sonarr_data.tasks.current_selection().name,
        "Test 1"
      );
    }

    #[rstest]
    fn test_queued_events_scroll(
      #[values(DEFAULT_KEYBINDINGS.up.key, DEFAULT_KEYBINDINGS.down.key)] key: Key,
    ) {
      let mut app = App::default();
      app.push_navigation_stack(ActiveSonarrBlock::System.into());
      app.data.sonarr_data.updates = ScrollableText::with_string("Test 1\nTest 2".to_owned());
      app
        .data
        .sonarr_data
        .queued_events
        .set_items(simple_stateful_iterable_vec!(QueueEvent, String, name));

      SystemDetailsHandler::new(key, &mut app, ActiveSonarrBlock::SystemQueuedEvents, None)
        .handle();

      assert_str_eq!(
        app.data.sonarr_data.queued_events.current_selection().name,
        "Test 2"
      );

      SystemDetailsHandler::new(key, &mut app, ActiveSonarrBlock::SystemQueuedEvents, None)
        .handle();

      assert_str_eq!(
        app.data.sonarr_data.queued_events.current_selection().name,
        "Test 1"
      );
    }

    #[rstest]
    fn test_queued_events_scroll_no_op_when_not_ready(
      #[values(DEFAULT_KEYBINDINGS.up.key, DEFAULT_KEYBINDINGS.down.key)] key: Key,
    ) {
      let mut app = App::default();
      app.push_navigation_stack(ActiveSonarrBlock::System.into());
      app.is_loading = true;
      app.data.sonarr_data.updates = ScrollableText::with_string("Test 1\nTest 2".to_owned());
      app
        .data
        .sonarr_data
        .queued_events
        .set_items(simple_stateful_iterable_vec!(QueueEvent, String, name));

      SystemDetailsHandler::new(key, &mut app, ActiveSonarrBlock::SystemQueuedEvents, None)
        .handle();

      assert_str_eq!(
        app.data.sonarr_data.queued_events.current_selection().name,
        "Test 1"
      );

      SystemDetailsHandler::new(key, &mut app, ActiveSonarrBlock::SystemQueuedEvents, None)
        .handle();

      assert_str_eq!(
        app.data.sonarr_data.queued_events.current_selection().name,
        "Test 1"
      );
    }

    #[test]
    fn test_system_updates_scroll() {
      let mut app = App::default();
      app.push_navigation_stack(ActiveSonarrBlock::System.into());
      app.data.sonarr_data.updates = ScrollableText::with_string("Test 1\nTest 2".to_owned());

      SystemDetailsHandler::new(
        DEFAULT_KEYBINDINGS.up.key,
        &mut app,
        ActiveSonarrBlock::SystemUpdates,
        None,
      )
      .handle();

      assert_eq!(app.data.sonarr_data.updates.offset, 0);

      SystemDetailsHandler::new(
        DEFAULT_KEYBINDINGS.down.key,
        &mut app,
        ActiveSonarrBlock::SystemUpdates,
        None,
      )
      .handle();

      assert_eq!(app.data.sonarr_data.updates.offset, 1);
    }

    #[test]
    fn test_system_updates_scroll_no_op_when_not_ready() {
      let mut app = App::default();
      app.push_navigation_stack(ActiveSonarrBlock::System.into());
      app.is_loading = true;
      app.data.sonarr_data.updates = ScrollableText::with_string("Test 1\nTest 2".to_owned());

      SystemDetailsHandler::new(
        DEFAULT_KEYBINDINGS.up.key,
        &mut app,
        ActiveSonarrBlock::SystemUpdates,
        None,
      )
      .handle();

      assert_eq!(app.data.sonarr_data.updates.offset, 0);

      SystemDetailsHandler::new(
        DEFAULT_KEYBINDINGS.down.key,
        &mut app,
        ActiveSonarrBlock::SystemUpdates,
        None,
      )
      .handle();

      assert_eq!(app.data.sonarr_data.updates.offset, 0);
    }
  }

  mod test_handle_home_end {
    use crate::models::{HorizontallyScrollableText, ScrollableText};
    use crate::{extended_stateful_iterable_vec, test_iterable_home_and_end};

    use super::*;
    use pretty_assertions::assert_eq;

    test_iterable_home_and_end!(
      test_log_details_home_end,
      SystemDetailsHandler,
      sonarr_data,
      log_details,
      extended_stateful_iterable_vec!(HorizontallyScrollableText, String, text),
      ActiveSonarrBlock::SystemLogs,
      None,
      text
    );

    #[test]
    fn test_log_details_home_end_no_op_when_not_ready() {
      let mut app = App::default();
      app.push_navigation_stack(ActiveSonarrBlock::System.into());
      app.is_loading = true;
      app
        .data
        .sonarr_data
        .log_details
        .set_items(extended_stateful_iterable_vec!(
          HorizontallyScrollableText,
          String,
          text
        ));

      SystemDetailsHandler::new(
        DEFAULT_KEYBINDINGS.end.key,
        &mut app,
        ActiveSonarrBlock::SystemLogs,
        None,
      )
      .handle();

      assert_str_eq!(
        app.data.sonarr_data.log_details.current_selection().text,
        "Test 1"
      );

      SystemDetailsHandler::new(
        DEFAULT_KEYBINDINGS.home.key,
        &mut app,
        ActiveSonarrBlock::SystemLogs,
        None,
      )
      .handle();

      assert_str_eq!(
        app.data.sonarr_data.log_details.current_selection().text,
        "Test 1"
      );
    }

    #[test]
    fn test_tasks_home_end() {
      let mut app = App::default();
      app.push_navigation_stack(ActiveSonarrBlock::System.into());
      app.data.sonarr_data.updates =
        ScrollableText::with_string("Test 1\nTest 2\nTest 3".to_owned());
      app
        .data
        .sonarr_data
        .tasks
        .set_items(extended_stateful_iterable_vec!(SonarrTask, String, name));

      SystemDetailsHandler::new(
        DEFAULT_KEYBINDINGS.end.key,
        &mut app,
        ActiveSonarrBlock::SystemTasks,
        None,
      )
      .handle();

      assert_str_eq!(
        app.data.sonarr_data.tasks.current_selection().name,
        "Test 3"
      );

      SystemDetailsHandler::new(
        DEFAULT_KEYBINDINGS.home.key,
        &mut app,
        ActiveSonarrBlock::SystemTasks,
        None,
      )
      .handle();

      assert_str_eq!(
        app.data.sonarr_data.tasks.current_selection().name,
        "Test 1"
      );
    }

    #[test]
    fn test_tasks_home_end_no_op_when_not_ready() {
      let mut app = App::default();
      app.push_navigation_stack(ActiveSonarrBlock::System.into());
      app.is_loading = true;
      app.data.sonarr_data.updates =
        ScrollableText::with_string("Test 1\nTest 2\nTest 3".to_owned());
      app
        .data
        .sonarr_data
        .tasks
        .set_items(extended_stateful_iterable_vec!(SonarrTask, String, name));

      SystemDetailsHandler::new(
        DEFAULT_KEYBINDINGS.end.key,
        &mut app,
        ActiveSonarrBlock::SystemTasks,
        None,
      )
      .handle();

      assert_str_eq!(
        app.data.sonarr_data.tasks.current_selection().name,
        "Test 1"
      );

      SystemDetailsHandler::new(
        DEFAULT_KEYBINDINGS.home.key,
        &mut app,
        ActiveSonarrBlock::SystemTasks,
        None,
      )
      .handle();

      assert_str_eq!(
        app.data.sonarr_data.tasks.current_selection().name,
        "Test 1"
      );
    }

    #[test]
    fn test_queued_events_home_end() {
      let mut app = App::default();
      app.push_navigation_stack(ActiveSonarrBlock::System.into());
      app.data.sonarr_data.updates =
        ScrollableText::with_string("Test 1\nTest 2\nTest 3".to_owned());
      app
        .data
        .sonarr_data
        .queued_events
        .set_items(extended_stateful_iterable_vec!(QueueEvent, String, name));

      SystemDetailsHandler::new(
        DEFAULT_KEYBINDINGS.end.key,
        &mut app,
        ActiveSonarrBlock::SystemQueuedEvents,
        None,
      )
      .handle();

      assert_str_eq!(
        app.data.sonarr_data.queued_events.current_selection().name,
        "Test 3"
      );

      SystemDetailsHandler::new(
        DEFAULT_KEYBINDINGS.home.key,
        &mut app,
        ActiveSonarrBlock::SystemQueuedEvents,
        None,
      )
      .handle();

      assert_str_eq!(
        app.data.sonarr_data.queued_events.current_selection().name,
        "Test 1"
      );
    }

    #[test]
    fn test_queued_events_home_end_no_op_when_not_ready() {
      let mut app = App::default();
      app.push_navigation_stack(ActiveSonarrBlock::System.into());
      app.is_loading = true;
      app.data.sonarr_data.updates =
        ScrollableText::with_string("Test 1\nTest 2\nTest 3".to_owned());
      app
        .data
        .sonarr_data
        .queued_events
        .set_items(extended_stateful_iterable_vec!(QueueEvent, String, name));

      SystemDetailsHandler::new(
        DEFAULT_KEYBINDINGS.end.key,
        &mut app,
        ActiveSonarrBlock::SystemQueuedEvents,
        None,
      )
      .handle();

      assert_str_eq!(
        app.data.sonarr_data.queued_events.current_selection().name,
        "Test 1"
      );

      SystemDetailsHandler::new(
        DEFAULT_KEYBINDINGS.home.key,
        &mut app,
        ActiveSonarrBlock::SystemQueuedEvents,
        None,
      )
      .handle();

      assert_str_eq!(
        app.data.sonarr_data.queued_events.current_selection().name,
        "Test 1"
      );
    }

    #[test]
    fn test_system_updates_home_end() {
      let mut app = App::default();
      app.push_navigation_stack(ActiveSonarrBlock::System.into());
      app.data.sonarr_data.updates = ScrollableText::with_string("Test 1\nTest 2".to_owned());

      SystemDetailsHandler::new(
        DEFAULT_KEYBINDINGS.end.key,
        &mut app,
        ActiveSonarrBlock::SystemUpdates,
        None,
      )
      .handle();

      assert_eq!(app.data.sonarr_data.updates.offset, 1);

      SystemDetailsHandler::new(
        DEFAULT_KEYBINDINGS.home.key,
        &mut app,
        ActiveSonarrBlock::SystemUpdates,
        None,
      )
      .handle();

      assert_eq!(app.data.sonarr_data.updates.offset, 0);
    }

    #[test]
    fn test_system_updates_home_end_no_op_when_not_ready() {
      let mut app = App::default();
      app.push_navigation_stack(ActiveSonarrBlock::System.into());
      app.is_loading = true;
      app.data.sonarr_data.updates = ScrollableText::with_string("Test 1\nTest 2".to_owned());

      SystemDetailsHandler::new(
        DEFAULT_KEYBINDINGS.end.key,
        &mut app,
        ActiveSonarrBlock::SystemUpdates,
        None,
      )
      .handle();

      assert_eq!(app.data.sonarr_data.updates.offset, 0);

      SystemDetailsHandler::new(
        DEFAULT_KEYBINDINGS.home.key,
        &mut app,
        ActiveSonarrBlock::SystemUpdates,
        None,
      )
      .handle();

      assert_eq!(app.data.sonarr_data.updates.offset, 0);
    }
  }

  mod test_handle_left_right_action {
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    use super::*;

    #[test]
    fn test_handle_log_details_left_right() {
      let active_sonarr_block = ActiveSonarrBlock::SystemLogs;
      let mut app = App::default();
      app.push_navigation_stack(ActiveSonarrBlock::System.into());
      app
        .data
        .sonarr_data
        .log_details
        .set_items(vec!["t1".into(), "t22".into()]);

      SystemDetailsHandler::new(
        DEFAULT_KEYBINDINGS.left.key,
        &mut app,
        active_sonarr_block,
        None,
      )
      .handle();

      assert_eq!(app.data.sonarr_data.log_details.items[0].to_string(), "t1");
      assert_eq!(app.data.sonarr_data.log_details.items[1].to_string(), "t22");

      SystemDetailsHandler::new(
        DEFAULT_KEYBINDINGS.right.key,
        &mut app,
        active_sonarr_block,
        None,
      )
      .handle();

      assert_eq!(app.data.sonarr_data.log_details.items[0].to_string(), "1");
      assert_eq!(app.data.sonarr_data.log_details.items[1].to_string(), "22");

      SystemDetailsHandler::new(
        DEFAULT_KEYBINDINGS.right.key,
        &mut app,
        active_sonarr_block,
        None,
      )
      .handle();

      assert_eq!(app.data.sonarr_data.log_details.items[0].to_string(), "");
      assert_eq!(app.data.sonarr_data.log_details.items[1].to_string(), "2");

      SystemDetailsHandler::new(
        DEFAULT_KEYBINDINGS.right.key,
        &mut app,
        active_sonarr_block,
        None,
      )
      .handle();

      assert_eq!(app.data.sonarr_data.log_details.items[0].to_string(), "");
      assert_eq!(app.data.sonarr_data.log_details.items[1].to_string(), "");

      SystemDetailsHandler::new(
        DEFAULT_KEYBINDINGS.right.key,
        &mut app,
        active_sonarr_block,
        None,
      )
      .handle();

      assert_eq!(app.data.sonarr_data.log_details.items[0].to_string(), "");
      assert_eq!(app.data.sonarr_data.log_details.items[1].to_string(), "");

      SystemDetailsHandler::new(
        DEFAULT_KEYBINDINGS.left.key,
        &mut app,
        active_sonarr_block,
        None,
      )
      .handle();

      assert_eq!(app.data.sonarr_data.log_details.items[0].to_string(), "1");
      assert_eq!(app.data.sonarr_data.log_details.items[1].to_string(), "2");

      SystemDetailsHandler::new(
        DEFAULT_KEYBINDINGS.left.key,
        &mut app,
        active_sonarr_block,
        None,
      )
      .handle();

      assert_eq!(app.data.sonarr_data.log_details.items[0].to_string(), "t1");
      assert_eq!(app.data.sonarr_data.log_details.items[1].to_string(), "22");

      SystemDetailsHandler::new(
        DEFAULT_KEYBINDINGS.left.key,
        &mut app,
        active_sonarr_block,
        None,
      )
      .handle();

      assert_eq!(app.data.sonarr_data.log_details.items[0].to_string(), "t1");
      assert_eq!(app.data.sonarr_data.log_details.items[1].to_string(), "t22");
    }

    #[rstest]
    fn test_left_right_prompt_toggle(
      #[values(DEFAULT_KEYBINDINGS.left.key, DEFAULT_KEYBINDINGS.right.key)] key: Key,
    ) {
      let mut app = App::default();
      app.push_navigation_stack(ActiveSonarrBlock::System.into());

      SystemDetailsHandler::new(
        key,
        &mut app,
        ActiveSonarrBlock::SystemTaskStartConfirmPrompt,
        None,
      )
      .handle();

      assert!(app.data.sonarr_data.prompt_confirm);

      SystemDetailsHandler::new(
        key,
        &mut app,
        ActiveSonarrBlock::SystemTaskStartConfirmPrompt,
        None,
      )
      .handle();

      assert!(!app.data.sonarr_data.prompt_confirm);
    }
  }

  mod test_handle_submit {
    use pretty_assertions::assert_eq;

    use crate::network::sonarr_network::SonarrEvent;

    use super::*;

    const SUBMIT_KEY: Key = DEFAULT_KEYBINDINGS.submit.key;

    #[test]
    fn test_system_tasks_submit() {
      let mut app = App::default();
      app.push_navigation_stack(ActiveSonarrBlock::System.into());
      app.data.sonarr_data.updates = ScrollableText::with_string("Test".to_owned());

      SystemDetailsHandler::new(SUBMIT_KEY, &mut app, ActiveSonarrBlock::SystemTasks, None)
        .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveSonarrBlock::SystemTaskStartConfirmPrompt.into()
      );
    }

    #[test]
    fn test_system_tasks_submit_no_op_when_not_ready() {
      let mut app = App::default();
      app.is_loading = true;
      app.push_navigation_stack(ActiveSonarrBlock::SystemTasks.into());
      app.data.sonarr_data.updates = ScrollableText::with_string("Test".to_owned());

      SystemDetailsHandler::new(SUBMIT_KEY, &mut app, ActiveSonarrBlock::SystemTasks, None)
        .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveSonarrBlock::SystemTasks.into()
      );
    }

    #[test]
    fn test_system_tasks_start_task_prompt_confirm_submit() {
      let mut app = App::default();
      app.data.sonarr_data.updates = ScrollableText::with_string("Test".to_owned());
      app.data.sonarr_data.prompt_confirm = true;
      app
        .data
        .sonarr_data
        .tasks
        .set_items(vec![SonarrTask::default()]);
      app.push_navigation_stack(ActiveSonarrBlock::SystemTasks.into());
      app.push_navigation_stack(ActiveSonarrBlock::SystemTaskStartConfirmPrompt.into());

      SystemDetailsHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveSonarrBlock::SystemTaskStartConfirmPrompt,
        None,
      )
      .handle();

      assert!(app.data.sonarr_data.prompt_confirm);
      assert_eq!(
        app.data.sonarr_data.prompt_confirm_action,
        Some(SonarrEvent::StartTask(SonarrTaskName::default()))
      );
      assert_eq!(
        app.get_current_route(),
        ActiveSonarrBlock::SystemTasks.into()
      );
    }

    #[test]
    fn test_system_tasks_start_task_prompt_decline_submit() {
      let mut app = App::default();
      app.data.sonarr_data.updates = ScrollableText::with_string("Test".to_owned());
      app.push_navigation_stack(ActiveSonarrBlock::SystemTasks.into());
      app.push_navigation_stack(ActiveSonarrBlock::SystemTaskStartConfirmPrompt.into());

      SystemDetailsHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveSonarrBlock::SystemTaskStartConfirmPrompt,
        None,
      )
      .handle();

      assert!(!app.data.sonarr_data.prompt_confirm);
      assert_eq!(app.data.sonarr_data.prompt_confirm_action, None);
      assert_eq!(
        app.get_current_route(),
        ActiveSonarrBlock::SystemTasks.into()
      );
    }
  }

  mod test_handle_esc {
    use crate::models::HorizontallyScrollableText;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    use super::*;

    const ESC_KEY: Key = DEFAULT_KEYBINDINGS.esc.key;

    #[rstest]
    fn test_esc_system_logs(#[values(true, false)] is_ready: bool) {
      let mut app = App::default();
      app.is_loading = is_ready;
      app
        .data
        .sonarr_data
        .log_details
        .set_items(vec![HorizontallyScrollableText::from("test")]);
      app.push_navigation_stack(ActiveSonarrBlock::System.into());
      app.push_navigation_stack(ActiveSonarrBlock::SystemLogs.into());
      app
        .data
        .sonarr_data
        .log_details
        .set_items(vec![HorizontallyScrollableText::default()]);

      SystemDetailsHandler::new(ESC_KEY, &mut app, ActiveSonarrBlock::SystemLogs, None).handle();

      assert_eq!(app.get_current_route(), ActiveSonarrBlock::System.into());
      assert!(app.data.sonarr_data.log_details.items.is_empty());
    }

    #[rstest]
    fn test_esc_system_tasks(#[values(true, false)] is_ready: bool) {
      let mut app = App::default();
      app.is_loading = is_ready;
      app.push_navigation_stack(ActiveSonarrBlock::System.into());
      app.push_navigation_stack(ActiveSonarrBlock::SystemTasks.into());
      app
        .data
        .sonarr_data
        .tasks
        .set_items(vec![SonarrTask::default()]);

      SystemDetailsHandler::new(ESC_KEY, &mut app, ActiveSonarrBlock::SystemTasks, None).handle();

      assert_eq!(app.get_current_route(), ActiveSonarrBlock::System.into());
    }

    #[rstest]
    fn test_esc_system_queued_events(#[values(true, false)] is_ready: bool) {
      let mut app = App::default();
      app.is_loading = is_ready;
      app.push_navigation_stack(ActiveSonarrBlock::System.into());
      app.push_navigation_stack(ActiveSonarrBlock::SystemQueuedEvents.into());
      app
        .data
        .sonarr_data
        .queued_events
        .set_items(vec![QueueEvent::default()]);

      SystemDetailsHandler::new(
        ESC_KEY,
        &mut app,
        ActiveSonarrBlock::SystemQueuedEvents,
        None,
      )
      .handle();

      assert_eq!(app.get_current_route(), ActiveSonarrBlock::System.into());
    }

    #[rstest]
    fn test_esc_system_updates(#[values(true, false)] is_ready: bool) {
      let mut app = App::default();
      app.is_loading = is_ready;
      app.push_navigation_stack(ActiveSonarrBlock::System.into());
      app.push_navigation_stack(ActiveSonarrBlock::SystemUpdates.into());

      SystemDetailsHandler::new(ESC_KEY, &mut app, ActiveSonarrBlock::SystemUpdates, None).handle();

      assert_eq!(app.get_current_route(), ActiveSonarrBlock::System.into());
    }

    #[test]
    fn test_system_tasks_start_task_prompt_esc() {
      let mut app = App::default();
      app.push_navigation_stack(ActiveSonarrBlock::SystemTasks.into());
      app.push_navigation_stack(ActiveSonarrBlock::SystemTaskStartConfirmPrompt.into());
      app.data.sonarr_data.prompt_confirm = true;

      SystemDetailsHandler::new(
        ESC_KEY,
        &mut app,
        ActiveSonarrBlock::SystemTaskStartConfirmPrompt,
        None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveSonarrBlock::SystemTasks.into()
      );
      assert!(!app.data.sonarr_data.prompt_confirm);
    }
  }

  mod test_handle_key_char {
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    use crate::network::sonarr_network::SonarrEvent;

    use super::*;

    #[rstest]
    fn test_refresh_key(
      #[values(
        ActiveSonarrBlock::SystemLogs,
        ActiveSonarrBlock::SystemTasks,
        ActiveSonarrBlock::SystemQueuedEvents,
        ActiveSonarrBlock::SystemUpdates
      )]
      active_sonarr_block: ActiveSonarrBlock,
    ) {
      let mut app = App::default();
      app.push_navigation_stack(ActiveSonarrBlock::System.into());
      app.data.sonarr_data.updates = ScrollableText::with_string("Test".to_owned());
      app.push_navigation_stack(active_sonarr_block.into());

      SystemDetailsHandler::new(
        DEFAULT_KEYBINDINGS.refresh.key,
        &mut app,
        active_sonarr_block,
        None,
      )
      .handle();

      assert_eq!(app.get_current_route(), active_sonarr_block.into());
      assert!(app.should_refresh);
    }

    #[rstest]
    fn test_refresh_key_no_op_when_not_ready(
      #[values(
        ActiveSonarrBlock::SystemLogs,
        ActiveSonarrBlock::SystemTasks,
        ActiveSonarrBlock::SystemQueuedEvents,
        ActiveSonarrBlock::SystemUpdates
      )]
      active_sonarr_block: ActiveSonarrBlock,
    ) {
      let mut app = App::default();
      app.push_navigation_stack(ActiveSonarrBlock::System.into());
      app.is_loading = true;
      app.data.sonarr_data.updates = ScrollableText::with_string("Test".to_owned());
      app.push_navigation_stack(active_sonarr_block.into());

      SystemDetailsHandler::new(
        DEFAULT_KEYBINDINGS.refresh.key,
        &mut app,
        active_sonarr_block,
        None,
      )
      .handle();

      assert_eq!(app.get_current_route(), active_sonarr_block.into());
      assert!(!app.should_refresh);
    }

    #[test]
    fn test_system_tasks_start_task_prompt_confirm() {
      let mut app = App::default();
      app.push_navigation_stack(ActiveSonarrBlock::System.into());
      app.data.sonarr_data.updates = ScrollableText::with_string("Test".to_owned());
      app
        .data
        .sonarr_data
        .tasks
        .set_items(vec![SonarrTask::default()]);
      app.push_navigation_stack(ActiveSonarrBlock::SystemTasks.into());
      app.push_navigation_stack(ActiveSonarrBlock::SystemTaskStartConfirmPrompt.into());

      SystemDetailsHandler::new(
        DEFAULT_KEYBINDINGS.confirm.key,
        &mut app,
        ActiveSonarrBlock::SystemTaskStartConfirmPrompt,
        None,
      )
      .handle();

      assert!(app.data.sonarr_data.prompt_confirm);
      assert_eq!(
        app.data.sonarr_data.prompt_confirm_action,
        Some(SonarrEvent::StartTask(SonarrTaskName::default()))
      );
      assert_eq!(
        app.get_current_route(),
        ActiveSonarrBlock::SystemTasks.into()
      );
    }
  }

  #[test]
  fn test_system_details_handler_accepts() {
    ActiveSonarrBlock::iter().for_each(|active_sonarr_block| {
      if SYSTEM_DETAILS_BLOCKS.contains(&active_sonarr_block) {
        assert!(SystemDetailsHandler::accepts(active_sonarr_block));
      } else {
        assert!(!SystemDetailsHandler::accepts(active_sonarr_block));
      }
    })
  }

  #[test]
  fn test_extract_task_name() {
    let mut app = App::default();
    app
      .data
      .sonarr_data
      .tasks
      .set_items(vec![SonarrTask::default()]);

    let task_name = SystemDetailsHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveSonarrBlock::SystemTasks,
      None,
    )
    .extract_task_name();

    assert_eq!(task_name, SonarrTaskName::default());
  }

  #[test]
  fn test_system_details_handler_not_ready_when_loading() {
    let mut app = App::default();
    app.push_navigation_stack(ActiveSonarrBlock::System.into());
    app.is_loading = true;

    let handler = SystemDetailsHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveSonarrBlock::SystemUpdates,
      None,
    );

    assert!(!handler.is_ready());
  }

  #[test]
  fn test_system_details_handler_not_ready_when_log_details_and_updates_and_tasks_are_empty() {
    let mut app = App::default();
    app.push_navigation_stack(ActiveSonarrBlock::System.into());
    app.is_loading = false;

    let handler = SystemDetailsHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveSonarrBlock::SystemUpdates,
      None,
    );

    assert!(!handler.is_ready());
  }

  #[test]
  fn test_system_details_handler_ready_when_not_loading_and_log_details_is_not_empty() {
    let mut app = App::default();
    app.push_navigation_stack(ActiveSonarrBlock::System.into());
    app.is_loading = false;
    app
      .data
      .sonarr_data
      .log_details
      .set_items(vec![HorizontallyScrollableText::default()]);

    let handler = SystemDetailsHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveSonarrBlock::SystemUpdates,
      None,
    );

    assert!(handler.is_ready());
  }

  #[test]
  fn test_system_details_handler_ready_when_not_loading_and_tasks_is_not_empty() {
    let mut app = App::default();
    app.push_navigation_stack(ActiveSonarrBlock::System.into());
    app.is_loading = false;
    app
      .data
      .sonarr_data
      .tasks
      .set_items(vec![SonarrTask::default()]);

    let handler = SystemDetailsHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveSonarrBlock::SystemTasks,
      None,
    );

    assert!(handler.is_ready());
  }

  #[test]
  fn test_system_details_handler_ready_when_not_loading_and_updates_is_not_empty() {
    let mut app = App::default();
    app.push_navigation_stack(ActiveSonarrBlock::System.into());
    app.is_loading = false;
    app.data.sonarr_data.updates = ScrollableText::with_string("Test".to_owned());

    let handler = SystemDetailsHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveSonarrBlock::SystemUpdates,
      None,
    );

    assert!(handler.is_ready());
  }
}
