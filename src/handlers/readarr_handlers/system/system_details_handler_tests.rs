#[cfg(test)]
mod tests {
  use pretty_assertions::{assert_eq, assert_str_eq};
  use rstest::rstest;
  use strum::IntoEnumIterator;

  use crate::app::App;
  use crate::app::key_binding::DEFAULT_KEYBINDINGS;
  use crate::assert_navigation_pushed;
  use crate::event::Key;
  use crate::handlers::KeyEventHandler;
  use crate::handlers::readarr_handlers::system::system_details_handler::SystemDetailsHandler;
  use crate::models::readarr_models::{ReadarrTask, ReadarrTaskName};
  use crate::models::servarr_data::readarr::readarr_data::{
    ActiveReadarrBlock, SYSTEM_DETAILS_BLOCKS,
  };
  use crate::models::servarr_models::QueueEvent;
  use crate::models::{HorizontallyScrollableText, ScrollableText};

  mod test_handle_scroll_up_and_down {
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    use crate::models::HorizontallyScrollableText;
    use crate::{simple_stateful_iterable_vec, test_iterable_scroll};

    use super::*;

    test_iterable_scroll!(
      test_log_details_scroll,
      SystemDetailsHandler,
      readarr_data,
      log_details,
      simple_stateful_iterable_vec!(HorizontallyScrollableText, String, text),
      ActiveReadarrBlock::SystemLogs,
      None,
      text
    );

    #[rstest]
    fn test_log_details_scroll_no_op_when_not_ready(
      #[values(DEFAULT_KEYBINDINGS.up.key, DEFAULT_KEYBINDINGS.down.key)] key: Key,
    ) {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveReadarrBlock::System.into());
      app.is_loading = true;
      app
        .data
        .readarr_data
        .log_details
        .set_items(simple_stateful_iterable_vec!(
          HorizontallyScrollableText,
          String,
          text
        ));

      SystemDetailsHandler::new(key, &mut app, ActiveReadarrBlock::SystemLogs, None).handle();

      assert_str_eq!(
        app.data.readarr_data.log_details.current_selection().text,
        "Test 1"
      );

      SystemDetailsHandler::new(key, &mut app, ActiveReadarrBlock::SystemLogs, None).handle();

      assert_str_eq!(
        app.data.readarr_data.log_details.current_selection().text,
        "Test 1"
      );
    }

    #[rstest]
    fn test_tasks_scroll(
      #[values(DEFAULT_KEYBINDINGS.up.key, DEFAULT_KEYBINDINGS.down.key)] key: Key,
    ) {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveReadarrBlock::System.into());
      app.data.readarr_data.tasks.set_items(tasks_vec());
      app
        .data
        .readarr_data
        .queued_events
        .set_items(queued_events_vec());

      SystemDetailsHandler::new(key, &mut app, ActiveReadarrBlock::SystemTasks, None).handle();

      assert_str_eq!(
        app.data.readarr_data.tasks.current_selection().name,
        "Rss sync task"
      );
      assert_str_eq!(
        app.data.readarr_data.queued_events.current_selection().name,
        "queued event one"
      );

      SystemDetailsHandler::new(key, &mut app, ActiveReadarrBlock::SystemTasks, None).handle();

      assert_str_eq!(
        app.data.readarr_data.tasks.current_selection().name,
        "Backup task"
      );
      assert_str_eq!(
        app.data.readarr_data.queued_events.current_selection().name,
        "queued event one"
      );
    }

    #[rstest]
    fn test_tasks_scroll_no_op_when_not_ready(
      #[values(DEFAULT_KEYBINDINGS.up.key, DEFAULT_KEYBINDINGS.down.key)] key: Key,
    ) {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveReadarrBlock::System.into());
      app.is_loading = true;
      app.data.readarr_data.tasks.set_items(tasks_vec());

      SystemDetailsHandler::new(key, &mut app, ActiveReadarrBlock::SystemTasks, None).handle();

      assert_str_eq!(
        app.data.readarr_data.tasks.current_selection().name,
        "Backup task"
      );

      SystemDetailsHandler::new(key, &mut app, ActiveReadarrBlock::SystemTasks, None).handle();

      assert_str_eq!(
        app.data.readarr_data.tasks.current_selection().name,
        "Backup task"
      );
    }

    #[rstest]
    fn test_queued_events_scroll(
      #[values(DEFAULT_KEYBINDINGS.up.key, DEFAULT_KEYBINDINGS.down.key)] key: Key,
    ) {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveReadarrBlock::System.into());
      app.data.readarr_data.tasks.set_items(tasks_vec());
      app
        .data
        .readarr_data
        .queued_events
        .set_items(queued_events_vec());

      SystemDetailsHandler::new(key, &mut app, ActiveReadarrBlock::SystemQueuedEvents, None)
        .handle();

      assert_str_eq!(
        app.data.readarr_data.queued_events.current_selection().name,
        "queued event two"
      );
      assert_str_eq!(
        app.data.readarr_data.tasks.current_selection().name,
        "Backup task"
      );

      SystemDetailsHandler::new(key, &mut app, ActiveReadarrBlock::SystemQueuedEvents, None)
        .handle();

      assert_str_eq!(
        app.data.readarr_data.queued_events.current_selection().name,
        "queued event one"
      );
      assert_str_eq!(
        app.data.readarr_data.tasks.current_selection().name,
        "Backup task"
      );
    }

    #[rstest]
    fn test_queued_events_scroll_no_op_when_not_ready(
      #[values(DEFAULT_KEYBINDINGS.up.key, DEFAULT_KEYBINDINGS.down.key)] key: Key,
    ) {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveReadarrBlock::System.into());
      app.is_loading = true;
      app.data.readarr_data.tasks.set_items(tasks_vec());
      app
        .data
        .readarr_data
        .queued_events
        .set_items(queued_events_vec());

      SystemDetailsHandler::new(key, &mut app, ActiveReadarrBlock::SystemQueuedEvents, None)
        .handle();

      assert_str_eq!(
        app.data.readarr_data.queued_events.current_selection().name,
        "queued event one"
      );

      SystemDetailsHandler::new(key, &mut app, ActiveReadarrBlock::SystemQueuedEvents, None)
        .handle();

      assert_str_eq!(
        app.data.readarr_data.queued_events.current_selection().name,
        "queued event one"
      );
    }

    #[test]
    fn test_system_updates_scroll() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveReadarrBlock::System.into());
      app.data.readarr_data.updates = updates_text();

      SystemDetailsHandler::new(
        DEFAULT_KEYBINDINGS.up.key,
        &mut app,
        ActiveReadarrBlock::SystemUpdates,
        None,
      )
      .handle();

      assert_eq!(app.data.readarr_data.updates.offset, 0);

      SystemDetailsHandler::new(
        DEFAULT_KEYBINDINGS.down.key,
        &mut app,
        ActiveReadarrBlock::SystemUpdates,
        None,
      )
      .handle();

      assert_eq!(app.data.readarr_data.updates.offset, 1);
    }

    #[test]
    fn test_system_updates_scroll_no_op_when_not_ready() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveReadarrBlock::System.into());
      app.is_loading = true;
      app.data.readarr_data.updates = updates_text();

      SystemDetailsHandler::new(
        DEFAULT_KEYBINDINGS.up.key,
        &mut app,
        ActiveReadarrBlock::SystemUpdates,
        None,
      )
      .handle();

      assert_eq!(app.data.readarr_data.updates.offset, 0);

      SystemDetailsHandler::new(
        DEFAULT_KEYBINDINGS.down.key,
        &mut app,
        ActiveReadarrBlock::SystemUpdates,
        None,
      )
      .handle();

      assert_eq!(app.data.readarr_data.updates.offset, 0);
    }
  }

  mod test_handle_home_end {
    use crate::models::HorizontallyScrollableText;
    use crate::{extended_stateful_iterable_vec, test_iterable_home_and_end};

    use super::*;
    use pretty_assertions::assert_eq;

    test_iterable_home_and_end!(
      test_log_details_home_end,
      SystemDetailsHandler,
      readarr_data,
      log_details,
      extended_stateful_iterable_vec!(HorizontallyScrollableText, String, text),
      ActiveReadarrBlock::SystemLogs,
      None,
      text
    );

    #[test]
    fn test_log_details_home_end_no_op_when_not_ready() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveReadarrBlock::System.into());
      app.is_loading = true;
      app
        .data
        .readarr_data
        .log_details
        .set_items(extended_stateful_iterable_vec!(
          HorizontallyScrollableText,
          String,
          text
        ));

      SystemDetailsHandler::new(
        DEFAULT_KEYBINDINGS.end.key,
        &mut app,
        ActiveReadarrBlock::SystemLogs,
        None,
      )
      .handle();

      assert_str_eq!(
        app.data.readarr_data.log_details.current_selection().text,
        "Test 1"
      );

      SystemDetailsHandler::new(
        DEFAULT_KEYBINDINGS.home.key,
        &mut app,
        ActiveReadarrBlock::SystemLogs,
        None,
      )
      .handle();

      assert_str_eq!(
        app.data.readarr_data.log_details.current_selection().text,
        "Test 1"
      );
    }

    #[test]
    fn test_tasks_home_end() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveReadarrBlock::System.into());
      app.data.readarr_data.tasks.set_items(extended_tasks_vec());
      app
        .data
        .readarr_data
        .queued_events
        .set_items(extended_queued_events_vec());

      SystemDetailsHandler::new(
        DEFAULT_KEYBINDINGS.end.key,
        &mut app,
        ActiveReadarrBlock::SystemTasks,
        None,
      )
      .handle();

      assert_str_eq!(
        app.data.readarr_data.tasks.current_selection().name,
        "Housekeeping task"
      );
      assert_str_eq!(
        app.data.readarr_data.queued_events.current_selection().name,
        "queued event one"
      );

      SystemDetailsHandler::new(
        DEFAULT_KEYBINDINGS.home.key,
        &mut app,
        ActiveReadarrBlock::SystemTasks,
        None,
      )
      .handle();

      assert_str_eq!(
        app.data.readarr_data.tasks.current_selection().name,
        "Backup task"
      );
    }

    #[test]
    fn test_tasks_home_end_no_op_when_not_ready() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveReadarrBlock::System.into());
      app.is_loading = true;
      app.data.readarr_data.tasks.set_items(extended_tasks_vec());

      SystemDetailsHandler::new(
        DEFAULT_KEYBINDINGS.end.key,
        &mut app,
        ActiveReadarrBlock::SystemTasks,
        None,
      )
      .handle();

      assert_str_eq!(
        app.data.readarr_data.tasks.current_selection().name,
        "Backup task"
      );

      SystemDetailsHandler::new(
        DEFAULT_KEYBINDINGS.home.key,
        &mut app,
        ActiveReadarrBlock::SystemTasks,
        None,
      )
      .handle();

      assert_str_eq!(
        app.data.readarr_data.tasks.current_selection().name,
        "Backup task"
      );
    }

    #[test]
    fn test_queued_events_home_end() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveReadarrBlock::System.into());
      app.data.readarr_data.tasks.set_items(extended_tasks_vec());
      app
        .data
        .readarr_data
        .queued_events
        .set_items(extended_queued_events_vec());

      SystemDetailsHandler::new(
        DEFAULT_KEYBINDINGS.end.key,
        &mut app,
        ActiveReadarrBlock::SystemQueuedEvents,
        None,
      )
      .handle();

      assert_str_eq!(
        app.data.readarr_data.queued_events.current_selection().name,
        "queued event three"
      );
      assert_str_eq!(
        app.data.readarr_data.tasks.current_selection().name,
        "Backup task"
      );

      SystemDetailsHandler::new(
        DEFAULT_KEYBINDINGS.home.key,
        &mut app,
        ActiveReadarrBlock::SystemQueuedEvents,
        None,
      )
      .handle();

      assert_str_eq!(
        app.data.readarr_data.queued_events.current_selection().name,
        "queued event one"
      );
    }

    #[test]
    fn test_queued_events_home_end_no_op_when_not_ready() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveReadarrBlock::System.into());
      app.is_loading = true;
      app.data.readarr_data.tasks.set_items(extended_tasks_vec());
      app
        .data
        .readarr_data
        .queued_events
        .set_items(extended_queued_events_vec());

      SystemDetailsHandler::new(
        DEFAULT_KEYBINDINGS.end.key,
        &mut app,
        ActiveReadarrBlock::SystemQueuedEvents,
        None,
      )
      .handle();

      assert_str_eq!(
        app.data.readarr_data.queued_events.current_selection().name,
        "queued event one"
      );

      SystemDetailsHandler::new(
        DEFAULT_KEYBINDINGS.home.key,
        &mut app,
        ActiveReadarrBlock::SystemQueuedEvents,
        None,
      )
      .handle();

      assert_str_eq!(
        app.data.readarr_data.queued_events.current_selection().name,
        "queued event one"
      );
    }

    #[test]
    fn test_system_updates_home_end() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveReadarrBlock::System.into());
      app.data.readarr_data.updates = updates_text();

      SystemDetailsHandler::new(
        DEFAULT_KEYBINDINGS.end.key,
        &mut app,
        ActiveReadarrBlock::SystemUpdates,
        None,
      )
      .handle();

      assert_eq!(app.data.readarr_data.updates.offset, 1);

      SystemDetailsHandler::new(
        DEFAULT_KEYBINDINGS.home.key,
        &mut app,
        ActiveReadarrBlock::SystemUpdates,
        None,
      )
      .handle();

      assert_eq!(app.data.readarr_data.updates.offset, 0);
    }

    #[test]
    fn test_system_updates_home_end_no_op_when_not_ready() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveReadarrBlock::System.into());
      app.is_loading = true;
      app.data.readarr_data.updates = updates_text();

      SystemDetailsHandler::new(
        DEFAULT_KEYBINDINGS.end.key,
        &mut app,
        ActiveReadarrBlock::SystemUpdates,
        None,
      )
      .handle();

      assert_eq!(app.data.readarr_data.updates.offset, 0);

      SystemDetailsHandler::new(
        DEFAULT_KEYBINDINGS.home.key,
        &mut app,
        ActiveReadarrBlock::SystemUpdates,
        None,
      )
      .handle();

      assert_eq!(app.data.readarr_data.updates.offset, 0);
    }
  }

  mod test_handle_left_right_action {
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    use super::*;

    #[test]
    fn test_handle_log_details_left_right() {
      let active_readarr_block = ActiveReadarrBlock::SystemLogs;
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveReadarrBlock::System.into());
      app
        .data
        .readarr_data
        .log_details
        .set_items(vec!["t1".into(), "t22".into()]);

      SystemDetailsHandler::new(
        DEFAULT_KEYBINDINGS.left.key,
        &mut app,
        active_readarr_block,
        None,
      )
      .handle();

      assert_eq!(app.data.readarr_data.log_details.items[0].to_string(), "t1");
      assert_eq!(
        app.data.readarr_data.log_details.items[1].to_string(),
        "t22"
      );

      SystemDetailsHandler::new(
        DEFAULT_KEYBINDINGS.right.key,
        &mut app,
        active_readarr_block,
        None,
      )
      .handle();

      assert_eq!(app.data.readarr_data.log_details.items[0].to_string(), "1");
      assert_eq!(app.data.readarr_data.log_details.items[1].to_string(), "22");

      SystemDetailsHandler::new(
        DEFAULT_KEYBINDINGS.right.key,
        &mut app,
        active_readarr_block,
        None,
      )
      .handle();

      assert_eq!(app.data.readarr_data.log_details.items[0].to_string(), "");
      assert_eq!(app.data.readarr_data.log_details.items[1].to_string(), "2");

      SystemDetailsHandler::new(
        DEFAULT_KEYBINDINGS.right.key,
        &mut app,
        active_readarr_block,
        None,
      )
      .handle();

      assert_eq!(app.data.readarr_data.log_details.items[0].to_string(), "");
      assert_eq!(app.data.readarr_data.log_details.items[1].to_string(), "");

      SystemDetailsHandler::new(
        DEFAULT_KEYBINDINGS.right.key,
        &mut app,
        active_readarr_block,
        None,
      )
      .handle();

      assert_eq!(app.data.readarr_data.log_details.items[0].to_string(), "");
      assert_eq!(app.data.readarr_data.log_details.items[1].to_string(), "");

      SystemDetailsHandler::new(
        DEFAULT_KEYBINDINGS.left.key,
        &mut app,
        active_readarr_block,
        None,
      )
      .handle();

      assert_eq!(app.data.readarr_data.log_details.items[0].to_string(), "1");
      assert_eq!(app.data.readarr_data.log_details.items[1].to_string(), "2");

      SystemDetailsHandler::new(
        DEFAULT_KEYBINDINGS.left.key,
        &mut app,
        active_readarr_block,
        None,
      )
      .handle();

      assert_eq!(app.data.readarr_data.log_details.items[0].to_string(), "t1");
      assert_eq!(app.data.readarr_data.log_details.items[1].to_string(), "22");

      SystemDetailsHandler::new(
        DEFAULT_KEYBINDINGS.left.key,
        &mut app,
        active_readarr_block,
        None,
      )
      .handle();

      assert_eq!(app.data.readarr_data.log_details.items[0].to_string(), "t1");
      assert_eq!(
        app.data.readarr_data.log_details.items[1].to_string(),
        "t22"
      );
    }

    #[rstest]
    fn test_left_right_prompt_toggle(
      #[values(DEFAULT_KEYBINDINGS.left.key, DEFAULT_KEYBINDINGS.right.key)] key: Key,
    ) {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveReadarrBlock::System.into());

      SystemDetailsHandler::new(
        key,
        &mut app,
        ActiveReadarrBlock::SystemTaskStartConfirmPrompt,
        None,
      )
      .handle();

      assert!(app.data.readarr_data.prompt_confirm);

      SystemDetailsHandler::new(
        key,
        &mut app,
        ActiveReadarrBlock::SystemTaskStartConfirmPrompt,
        None,
      )
      .handle();

      assert!(!app.data.readarr_data.prompt_confirm);
    }
  }

  mod test_handle_submit {
    use crate::assert_navigation_popped;
    use crate::network::readarr_network::ReadarrEvent;
    use pretty_assertions::assert_eq;

    use super::*;

    const SUBMIT_KEY: Key = DEFAULT_KEYBINDINGS.submit.key;

    #[test]
    fn test_system_tasks_submit() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveReadarrBlock::System.into());
      app.data.readarr_data.updates = updates_text();

      SystemDetailsHandler::new(SUBMIT_KEY, &mut app, ActiveReadarrBlock::SystemTasks, None)
        .handle();

      assert_navigation_pushed!(app, ActiveReadarrBlock::SystemTaskStartConfirmPrompt.into());
    }

    #[test]
    fn test_system_tasks_submit_no_op_when_not_ready() {
      let mut app = App::test_default();
      app.is_loading = true;
      app.push_navigation_stack(ActiveReadarrBlock::SystemTasks.into());
      app.data.readarr_data.updates = updates_text();

      SystemDetailsHandler::new(SUBMIT_KEY, &mut app, ActiveReadarrBlock::SystemTasks, None)
        .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveReadarrBlock::SystemTasks.into()
      );
    }

    #[test]
    fn test_system_tasks_start_task_prompt_confirm_submit() {
      let mut app = App::test_default();
      app.data.readarr_data.updates = updates_text();
      app.data.readarr_data.prompt_confirm = true;
      app.data.readarr_data.tasks.set_items(tasks_vec());
      app.data.readarr_data.tasks.select_index(Some(1));
      app.push_navigation_stack(ActiveReadarrBlock::SystemTasks.into());
      app.push_navigation_stack(ActiveReadarrBlock::SystemTaskStartConfirmPrompt.into());

      SystemDetailsHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveReadarrBlock::SystemTaskStartConfirmPrompt,
        None,
      )
      .handle();

      assert!(app.data.readarr_data.prompt_confirm);
      assert_some_eq_x!(
        &app.data.readarr_data.prompt_confirm_action,
        &ReadarrEvent::StartTask(ReadarrTaskName::RssSync)
      );
      assert_navigation_popped!(app, ActiveReadarrBlock::SystemTasks.into());
    }

    #[test]
    fn test_system_tasks_start_task_prompt_decline_submit() {
      let mut app = App::test_default();
      app.data.readarr_data.updates = updates_text();
      app.data.readarr_data.tasks.set_items(tasks_vec());
      app.push_navigation_stack(ActiveReadarrBlock::SystemTasks.into());
      app.push_navigation_stack(ActiveReadarrBlock::SystemTaskStartConfirmPrompt.into());

      SystemDetailsHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveReadarrBlock::SystemTaskStartConfirmPrompt,
        None,
      )
      .handle();

      assert!(!app.data.readarr_data.prompt_confirm);
      assert_none!(app.data.readarr_data.prompt_confirm_action);
      assert_navigation_popped!(app, ActiveReadarrBlock::SystemTasks.into());
    }
  }

  mod test_handle_esc {
    use crate::models::HorizontallyScrollableText;
    use rstest::rstest;

    use super::*;
    use crate::assert_navigation_popped;

    const ESC_KEY: Key = DEFAULT_KEYBINDINGS.esc.key;

    #[rstest]
    fn test_esc_system_logs(#[values(true, false)] is_loading: bool) {
      let mut app = App::test_default();
      app.is_loading = is_loading;
      app
        .data
        .readarr_data
        .log_details
        .set_items(vec![HorizontallyScrollableText::from("test")]);
      app.push_navigation_stack(ActiveReadarrBlock::System.into());
      app.push_navigation_stack(ActiveReadarrBlock::SystemLogs.into());
      app
        .data
        .readarr_data
        .log_details
        .set_items(vec![HorizontallyScrollableText::default()]);

      SystemDetailsHandler::new(ESC_KEY, &mut app, ActiveReadarrBlock::SystemLogs, None).handle();

      assert_navigation_popped!(app, ActiveReadarrBlock::System.into());
      assert_is_empty!(app.data.readarr_data.log_details.items);
    }

    #[rstest]
    fn test_esc_system_tasks(#[values(true, false)] is_loading: bool) {
      let mut app = App::test_default();
      app.is_loading = is_loading;
      app.push_navigation_stack(ActiveReadarrBlock::System.into());
      app.push_navigation_stack(ActiveReadarrBlock::SystemTasks.into());
      app.data.readarr_data.tasks.set_items(tasks_vec());

      SystemDetailsHandler::new(ESC_KEY, &mut app, ActiveReadarrBlock::SystemTasks, None).handle();

      assert_navigation_popped!(app, ActiveReadarrBlock::System.into());
    }

    #[rstest]
    fn test_esc_system_queued_events(#[values(true, false)] is_loading: bool) {
      let mut app = App::test_default();
      app.is_loading = is_loading;
      app.push_navigation_stack(ActiveReadarrBlock::System.into());
      app.push_navigation_stack(ActiveReadarrBlock::SystemQueuedEvents.into());
      app
        .data
        .readarr_data
        .queued_events
        .set_items(queued_events_vec());

      SystemDetailsHandler::new(
        ESC_KEY,
        &mut app,
        ActiveReadarrBlock::SystemQueuedEvents,
        None,
      )
      .handle();

      assert_navigation_popped!(app, ActiveReadarrBlock::System.into());
    }

    #[rstest]
    fn test_esc_system_updates(#[values(true, false)] is_loading: bool) {
      let mut app = App::test_default();
      app.is_loading = is_loading;
      app.push_navigation_stack(ActiveReadarrBlock::System.into());
      app.push_navigation_stack(ActiveReadarrBlock::SystemUpdates.into());

      SystemDetailsHandler::new(ESC_KEY, &mut app, ActiveReadarrBlock::SystemUpdates, None)
        .handle();

      assert_navigation_popped!(app, ActiveReadarrBlock::System.into());
    }

    #[test]
    fn test_system_tasks_start_task_prompt_esc() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveReadarrBlock::SystemTasks.into());
      app.push_navigation_stack(ActiveReadarrBlock::SystemTaskStartConfirmPrompt.into());
      app.data.readarr_data.prompt_confirm = true;

      SystemDetailsHandler::new(
        ESC_KEY,
        &mut app,
        ActiveReadarrBlock::SystemTaskStartConfirmPrompt,
        None,
      )
      .handle();

      assert_navigation_popped!(app, ActiveReadarrBlock::SystemTasks.into());
      assert!(!app.data.readarr_data.prompt_confirm);
    }
  }

  mod test_handle_key_char {
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    use crate::network::readarr_network::ReadarrEvent;

    use super::*;
    use crate::{assert_navigation_popped, assert_navigation_pushed};

    #[rstest]
    fn test_refresh_key(
      #[values(
        ActiveReadarrBlock::SystemLogs,
        ActiveReadarrBlock::SystemTasks,
        ActiveReadarrBlock::SystemQueuedEvents,
        ActiveReadarrBlock::SystemUpdates
      )]
      active_readarr_block: ActiveReadarrBlock,
    ) {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveReadarrBlock::System.into());
      app.data.readarr_data.updates = updates_text();
      app.push_navigation_stack(active_readarr_block.into());

      SystemDetailsHandler::new(
        DEFAULT_KEYBINDINGS.refresh.key,
        &mut app,
        active_readarr_block,
        None,
      )
      .handle();

      assert_navigation_pushed!(app, active_readarr_block.into());
      assert!(app.should_refresh);
    }

    #[rstest]
    fn test_refresh_key_no_op_when_not_ready(
      #[values(
        ActiveReadarrBlock::SystemLogs,
        ActiveReadarrBlock::SystemTasks,
        ActiveReadarrBlock::SystemQueuedEvents,
        ActiveReadarrBlock::SystemUpdates
      )]
      active_readarr_block: ActiveReadarrBlock,
    ) {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveReadarrBlock::System.into());
      app.is_loading = true;
      app.data.readarr_data.updates = updates_text();
      app.push_navigation_stack(active_readarr_block.into());

      SystemDetailsHandler::new(
        DEFAULT_KEYBINDINGS.refresh.key,
        &mut app,
        active_readarr_block,
        None,
      )
      .handle();

      assert_eq!(app.get_current_route(), active_readarr_block.into());
      assert!(!app.should_refresh);
    }

    #[test]
    fn test_system_tasks_start_task_prompt_confirm() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveReadarrBlock::System.into());
      app.data.readarr_data.updates = updates_text();
      app.data.readarr_data.tasks.set_items(tasks_vec());
      app.data.readarr_data.tasks.select_index(Some(1));
      app.push_navigation_stack(ActiveReadarrBlock::SystemTasks.into());
      app.push_navigation_stack(ActiveReadarrBlock::SystemTaskStartConfirmPrompt.into());

      SystemDetailsHandler::new(
        DEFAULT_KEYBINDINGS.confirm.key,
        &mut app,
        ActiveReadarrBlock::SystemTaskStartConfirmPrompt,
        None,
      )
      .handle();

      assert!(app.data.readarr_data.prompt_confirm);
      assert_eq!(
        app.data.readarr_data.prompt_confirm_action,
        Some(ReadarrEvent::StartTask(ReadarrTaskName::RssSync))
      );
      assert_navigation_popped!(app, ActiveReadarrBlock::SystemTasks.into());
    }
  }

  #[test]
  fn test_system_details_handler_accepts() {
    ActiveReadarrBlock::iter().for_each(|active_readarr_block| {
      if SYSTEM_DETAILS_BLOCKS.contains(&active_readarr_block) {
        assert!(SystemDetailsHandler::accepts(active_readarr_block));
      } else {
        assert!(!SystemDetailsHandler::accepts(active_readarr_block));
      }
    })
  }

  #[rstest]
  fn test_system_details_handler_ignore_special_keys(
    #[values(true, false)] ignore_special_keys_for_textbox_input: bool,
  ) {
    let mut app = App::test_default();
    app.ignore_special_keys_for_textbox_input = ignore_special_keys_for_textbox_input;
    let handler = SystemDetailsHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveReadarrBlock::default(),
      None,
    );

    assert_eq!(
      handler.ignore_special_keys(),
      ignore_special_keys_for_textbox_input
    );
  }

  #[test]
  fn test_extract_task_name_reads_the_selected_task() {
    let mut app = App::test_default();
    app.data.readarr_data.tasks.set_items(tasks_vec());
    app.data.readarr_data.tasks.select_index(Some(1));

    let task_name = SystemDetailsHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveReadarrBlock::SystemTasks,
      None,
    )
    .extract_task_name();

    assert_eq!(task_name, ReadarrTaskName::RssSync);
  }

  #[test]
  fn test_extract_task_name_reads_the_first_task_when_nothing_is_selected() {
    let mut app = App::test_default();
    app.data.readarr_data.tasks.set_items(tasks_vec());

    let task_name = SystemDetailsHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveReadarrBlock::SystemTasks,
      None,
    )
    .extract_task_name();

    assert_eq!(task_name, ReadarrTaskName::Backup);
  }

  #[test]
  fn test_system_details_handler_not_ready_when_loading() {
    let mut app = App::test_default();
    app.push_navigation_stack(ActiveReadarrBlock::System.into());
    app.is_loading = true;
    app.data.readarr_data.updates = updates_text();

    let handler = SystemDetailsHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveReadarrBlock::SystemUpdates,
      None,
    );

    assert!(!handler.is_ready());
  }

  #[test]
  fn test_system_details_handler_not_ready_when_log_details_and_updates_and_tasks_are_empty() {
    let mut app = App::test_default();
    app.push_navigation_stack(ActiveReadarrBlock::System.into());
    app.is_loading = false;

    let handler = SystemDetailsHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveReadarrBlock::SystemUpdates,
      None,
    );

    assert!(!handler.is_ready());
  }

  #[test]
  fn test_system_details_handler_ready_when_not_loading_and_log_details_is_not_empty() {
    let mut app = App::test_default();
    app.push_navigation_stack(ActiveReadarrBlock::System.into());
    app.is_loading = false;
    app
      .data
      .readarr_data
      .log_details
      .set_items(vec![HorizontallyScrollableText::default()]);

    let handler = SystemDetailsHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveReadarrBlock::SystemUpdates,
      None,
    );

    assert!(handler.is_ready());
  }

  #[test]
  fn test_system_details_handler_ready_when_not_loading_and_tasks_is_not_empty() {
    let mut app = App::test_default();
    app.push_navigation_stack(ActiveReadarrBlock::System.into());
    app.is_loading = false;
    app.data.readarr_data.tasks.set_items(tasks_vec());

    let handler = SystemDetailsHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveReadarrBlock::SystemTasks,
      None,
    );

    assert!(handler.is_ready());
  }

  #[test]
  fn test_system_details_handler_ready_when_not_loading_and_updates_is_not_empty() {
    let mut app = App::test_default();
    app.push_navigation_stack(ActiveReadarrBlock::System.into());
    app.is_loading = false;
    app.data.readarr_data.updates = updates_text();

    let handler = SystemDetailsHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveReadarrBlock::SystemUpdates,
      None,
    );

    assert!(handler.is_ready());
  }

  fn updates_text() -> ScrollableText {
    ScrollableText::with_string("update note one\nupdate note two".to_owned())
  }

  fn tasks_vec() -> Vec<ReadarrTask> {
    vec![
      ReadarrTask {
        name: "Backup task".to_owned(),
        task_name: ReadarrTaskName::Backup,
        ..ReadarrTask::default()
      },
      ReadarrTask {
        name: "Rss sync task".to_owned(),
        task_name: ReadarrTaskName::RssSync,
        ..ReadarrTask::default()
      },
    ]
  }

  fn extended_tasks_vec() -> Vec<ReadarrTask> {
    let mut tasks = tasks_vec();
    tasks.push(ReadarrTask {
      name: "Housekeeping task".to_owned(),
      task_name: ReadarrTaskName::Housekeeping,
      ..ReadarrTask::default()
    });

    tasks
  }

  fn queued_events_vec() -> Vec<QueueEvent> {
    vec![
      QueueEvent {
        name: "queued event one".to_owned(),
        ..QueueEvent::default()
      },
      QueueEvent {
        name: "queued event two".to_owned(),
        ..QueueEvent::default()
      },
    ]
  }

  fn extended_queued_events_vec() -> Vec<QueueEvent> {
    let mut queued_events = queued_events_vec();
    queued_events.push(QueueEvent {
      name: "queued event three".to_owned(),
      ..QueueEvent::default()
    });

    queued_events
  }
}
