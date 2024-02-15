#[cfg(test)]
mod tests {
  use pretty_assertions::assert_str_eq;
  use strum::IntoEnumIterator;

  use crate::app::key_binding::DEFAULT_KEYBINDINGS;
  use crate::app::App;
  use crate::event::Key;
  use crate::handlers::radarr_handlers::system::system_details_handler::SystemDetailsHandler;
  use crate::handlers::KeyEventHandler;
  use crate::models::radarr_models::{QueueEvent, Task};
  use crate::models::servarr_data::radarr::radarr_data::{
    ActiveRadarrBlock, SYSTEM_DETAILS_BLOCKS,
  };

  mod test_handle_scroll_up_and_down {
    use rstest::rstest;

    use crate::models::{HorizontallyScrollableText, ScrollableText};
    use crate::{simple_stateful_iterable_vec, test_iterable_scroll};

    use super::*;

    test_iterable_scroll!(
      test_log_details_scroll,
      SystemDetailsHandler,
      log_details,
      simple_stateful_iterable_vec!(HorizontallyScrollableText, String, text),
      ActiveRadarrBlock::SystemLogs,
      None,
      text
    );

    test_iterable_scroll!(
      test_tasks_scroll,
      SystemDetailsHandler,
      tasks,
      simple_stateful_iterable_vec!(Task, String, name),
      ActiveRadarrBlock::SystemTasks,
      None,
      name
    );

    test_iterable_scroll!(
      test_queued_events_scroll,
      SystemDetailsHandler,
      queued_events,
      simple_stateful_iterable_vec!(QueueEvent, String, name),
      ActiveRadarrBlock::SystemQueuedEvents,
      None,
      name
    );

    #[test]
    fn test_system_updates_scroll() {
      let mut app = App::default();
      app.data.radarr_data.updates = ScrollableText::with_string("Test 1\nTest 2".to_owned());

      SystemDetailsHandler::with(
        &DEFAULT_KEYBINDINGS.up.key,
        &mut app,
        &ActiveRadarrBlock::SystemUpdates,
        &None,
      )
      .handle();

      assert_eq!(app.data.radarr_data.updates.offset, 0);

      SystemDetailsHandler::with(
        &DEFAULT_KEYBINDINGS.down.key,
        &mut app,
        &ActiveRadarrBlock::SystemUpdates,
        &None,
      )
      .handle();

      assert_eq!(app.data.radarr_data.updates.offset, 1);
    }
  }

  mod test_handle_home_end {
    use crate::models::{HorizontallyScrollableText, ScrollableText};
    use crate::{extended_stateful_iterable_vec, test_iterable_home_and_end};

    use super::*;

    test_iterable_home_and_end!(
      test_log_details_home_end,
      SystemDetailsHandler,
      log_details,
      extended_stateful_iterable_vec!(HorizontallyScrollableText, String, text),
      ActiveRadarrBlock::SystemLogs,
      None,
      text
    );

    test_iterable_home_and_end!(
      test_tasks_home_end,
      SystemDetailsHandler,
      tasks,
      extended_stateful_iterable_vec!(Task, String, name),
      ActiveRadarrBlock::SystemTasks,
      None,
      name
    );

    test_iterable_home_and_end!(
      test_queued_events_home_end,
      SystemDetailsHandler,
      queued_events,
      extended_stateful_iterable_vec!(QueueEvent, String, name),
      ActiveRadarrBlock::SystemQueuedEvents,
      None,
      name
    );

    #[test]
    fn test_system_updates_home_end() {
      let mut app = App::default();
      app.data.radarr_data.updates = ScrollableText::with_string("Test 1\nTest 2".to_owned());

      SystemDetailsHandler::with(
        &DEFAULT_KEYBINDINGS.end.key,
        &mut app,
        &ActiveRadarrBlock::SystemUpdates,
        &None,
      )
      .handle();

      assert_eq!(app.data.radarr_data.updates.offset, 1);

      SystemDetailsHandler::with(
        &DEFAULT_KEYBINDINGS.home.key,
        &mut app,
        &ActiveRadarrBlock::SystemUpdates,
        &None,
      )
      .handle();

      assert_eq!(app.data.radarr_data.updates.offset, 0);
    }
  }

  mod test_handle_left_right_action {
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    use super::*;

    #[test]
    fn test_handle_log_details_left_right() {
      let active_radarr_block = ActiveRadarrBlock::SystemLogs;
      let mut app = App::default();
      app
        .data
        .radarr_data
        .log_details
        .set_items(vec!["t1".into(), "t22".into()]);

      SystemDetailsHandler::with(
        &DEFAULT_KEYBINDINGS.left.key,
        &mut app,
        &active_radarr_block,
        &None,
      )
      .handle();

      assert_eq!(app.data.radarr_data.log_details.items[0].to_string(), "t1");
      assert_eq!(app.data.radarr_data.log_details.items[1].to_string(), "t22");

      SystemDetailsHandler::with(
        &DEFAULT_KEYBINDINGS.right.key,
        &mut app,
        &active_radarr_block,
        &None,
      )
      .handle();

      assert_eq!(app.data.radarr_data.log_details.items[0].to_string(), "1");
      assert_eq!(app.data.radarr_data.log_details.items[1].to_string(), "22");

      SystemDetailsHandler::with(
        &DEFAULT_KEYBINDINGS.right.key,
        &mut app,
        &active_radarr_block,
        &None,
      )
      .handle();

      assert_eq!(app.data.radarr_data.log_details.items[0].to_string(), "");
      assert_eq!(app.data.radarr_data.log_details.items[1].to_string(), "2");

      SystemDetailsHandler::with(
        &DEFAULT_KEYBINDINGS.right.key,
        &mut app,
        &active_radarr_block,
        &None,
      )
      .handle();

      assert_eq!(app.data.radarr_data.log_details.items[0].to_string(), "");
      assert_eq!(app.data.radarr_data.log_details.items[1].to_string(), "");

      SystemDetailsHandler::with(
        &DEFAULT_KEYBINDINGS.right.key,
        &mut app,
        &active_radarr_block,
        &None,
      )
      .handle();

      assert_eq!(app.data.radarr_data.log_details.items[0].to_string(), "");
      assert_eq!(app.data.radarr_data.log_details.items[1].to_string(), "");

      SystemDetailsHandler::with(
        &DEFAULT_KEYBINDINGS.left.key,
        &mut app,
        &active_radarr_block,
        &None,
      )
      .handle();

      assert_eq!(app.data.radarr_data.log_details.items[0].to_string(), "1");
      assert_eq!(app.data.radarr_data.log_details.items[1].to_string(), "2");

      SystemDetailsHandler::with(
        &DEFAULT_KEYBINDINGS.left.key,
        &mut app,
        &active_radarr_block,
        &None,
      )
      .handle();

      assert_eq!(app.data.radarr_data.log_details.items[0].to_string(), "t1");
      assert_eq!(app.data.radarr_data.log_details.items[1].to_string(), "22");

      SystemDetailsHandler::with(
        &DEFAULT_KEYBINDINGS.left.key,
        &mut app,
        &active_radarr_block,
        &None,
      )
      .handle();

      assert_eq!(app.data.radarr_data.log_details.items[0].to_string(), "t1");
      assert_eq!(app.data.radarr_data.log_details.items[1].to_string(), "t22");
    }

    #[rstest]
    fn test_left_right_prompt_toggle(
      #[values(DEFAULT_KEYBINDINGS.left.key, DEFAULT_KEYBINDINGS.right.key)] key: Key,
    ) {
      let mut app = App::default();

      SystemDetailsHandler::with(
        &key,
        &mut app,
        &ActiveRadarrBlock::SystemTaskStartConfirmPrompt,
        &None,
      )
      .handle();

      assert!(app.data.radarr_data.prompt_confirm);

      SystemDetailsHandler::with(
        &key,
        &mut app,
        &ActiveRadarrBlock::SystemTaskStartConfirmPrompt,
        &None,
      )
      .handle();

      assert!(!app.data.radarr_data.prompt_confirm);
    }
  }

  mod test_handle_submit {
    use pretty_assertions::assert_eq;

    use crate::network::radarr_network::RadarrEvent;

    use super::*;

    const SUBMIT_KEY: Key = DEFAULT_KEYBINDINGS.submit.key;

    #[test]
    fn test_system_tasks_submit() {
      let mut app = App::default();

      SystemDetailsHandler::with(
        &SUBMIT_KEY,
        &mut app,
        &ActiveRadarrBlock::SystemTasks,
        &None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        &ActiveRadarrBlock::SystemTaskStartConfirmPrompt.into()
      );
    }

    #[test]
    fn test_system_tasks_start_task_prompt_confirm_submit() {
      let mut app = App::default();
      app.data.radarr_data.prompt_confirm = true;
      app.push_navigation_stack(ActiveRadarrBlock::SystemTasks.into());
      app.push_navigation_stack(ActiveRadarrBlock::SystemTaskStartConfirmPrompt.into());

      SystemDetailsHandler::with(
        &SUBMIT_KEY,
        &mut app,
        &ActiveRadarrBlock::SystemTaskStartConfirmPrompt,
        &None,
      )
      .handle();

      assert!(app.data.radarr_data.prompt_confirm);
      assert_eq!(
        app.data.radarr_data.prompt_confirm_action,
        Some(RadarrEvent::StartTask)
      );
      assert_eq!(
        app.get_current_route(),
        &ActiveRadarrBlock::SystemTasks.into()
      );
    }

    #[test]
    fn test_system_tasks_start_task_prompt_decline_submit() {
      let mut app = App::default();
      app.push_navigation_stack(ActiveRadarrBlock::SystemTasks.into());
      app.push_navigation_stack(ActiveRadarrBlock::SystemTaskStartConfirmPrompt.into());

      SystemDetailsHandler::with(
        &SUBMIT_KEY,
        &mut app,
        &ActiveRadarrBlock::SystemTaskStartConfirmPrompt,
        &None,
      )
      .handle();

      assert!(!app.data.radarr_data.prompt_confirm);
      assert_eq!(app.data.radarr_data.prompt_confirm_action, None);
      assert_eq!(
        app.get_current_route(),
        &ActiveRadarrBlock::SystemTasks.into()
      );
    }
  }

  mod test_handle_esc {
    use pretty_assertions::assert_eq;

    use crate::models::HorizontallyScrollableText;

    use super::*;

    const ESC_KEY: Key = DEFAULT_KEYBINDINGS.esc.key;

    #[test]
    fn test_esc_system_logs() {
      let mut app = App::default();
      app
        .data
        .radarr_data
        .log_details
        .set_items(vec![HorizontallyScrollableText::from("test")]);
      app.push_navigation_stack(ActiveRadarrBlock::System.into());
      app.push_navigation_stack(ActiveRadarrBlock::SystemLogs.into());
      app
        .data
        .radarr_data
        .log_details
        .set_items(vec![HorizontallyScrollableText::default()]);

      SystemDetailsHandler::with(&ESC_KEY, &mut app, &ActiveRadarrBlock::SystemLogs, &None)
        .handle();

      assert_eq!(app.get_current_route(), &ActiveRadarrBlock::System.into());
      assert!(app.data.radarr_data.log_details.items.is_empty());
    }

    #[test]
    fn test_esc_system_tasks() {
      let mut app = App::default();
      app.push_navigation_stack(ActiveRadarrBlock::System.into());
      app.push_navigation_stack(ActiveRadarrBlock::SystemTasks.into());
      app.data.radarr_data.tasks.set_items(vec![Task::default()]);

      SystemDetailsHandler::with(&ESC_KEY, &mut app, &ActiveRadarrBlock::SystemTasks, &None)
        .handle();

      assert_eq!(app.get_current_route(), &ActiveRadarrBlock::System.into());
    }

    #[test]
    fn test_esc_system_queued_events() {
      let mut app = App::default();
      app.push_navigation_stack(ActiveRadarrBlock::System.into());
      app.push_navigation_stack(ActiveRadarrBlock::SystemQueuedEvents.into());
      app
        .data
        .radarr_data
        .queued_events
        .set_items(vec![QueueEvent::default()]);

      SystemDetailsHandler::with(
        &ESC_KEY,
        &mut app,
        &ActiveRadarrBlock::SystemQueuedEvents,
        &None,
      )
      .handle();

      assert_eq!(app.get_current_route(), &ActiveRadarrBlock::System.into());
    }

    #[test]
    fn test_esc_system_updates() {
      let mut app = App::default();
      app.push_navigation_stack(ActiveRadarrBlock::System.into());
      app.push_navigation_stack(ActiveRadarrBlock::SystemUpdates.into());

      SystemDetailsHandler::with(&ESC_KEY, &mut app, &ActiveRadarrBlock::SystemUpdates, &None)
        .handle();

      assert_eq!(app.get_current_route(), &ActiveRadarrBlock::System.into());
    }

    #[test]
    fn test_system_tasks_start_task_prompt_esc() {
      let mut app = App::default();
      app.push_navigation_stack(ActiveRadarrBlock::SystemTasks.into());
      app.push_navigation_stack(ActiveRadarrBlock::SystemTaskStartConfirmPrompt.into());
      app.data.radarr_data.prompt_confirm = true;

      SystemDetailsHandler::with(
        &ESC_KEY,
        &mut app,
        &ActiveRadarrBlock::SystemTaskStartConfirmPrompt,
        &None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        &ActiveRadarrBlock::SystemTasks.into()
      );
      assert!(!app.data.radarr_data.prompt_confirm);
    }
  }

  mod test_handle_key_char {
    use rstest::rstest;

    use super::*;

    #[rstest]
    fn test_refresh_key(
      #[values(
        ActiveRadarrBlock::SystemLogs,
        ActiveRadarrBlock::SystemTasks,
        ActiveRadarrBlock::SystemQueuedEvents,
        ActiveRadarrBlock::SystemUpdates
      )]
      active_radarr_block: ActiveRadarrBlock,
    ) {
      let mut app = App::default();
      app.push_navigation_stack(active_radarr_block.into());

      SystemDetailsHandler::with(
        &DEFAULT_KEYBINDINGS.refresh.key,
        &mut app,
        &active_radarr_block,
        &None,
      )
      .handle();

      assert_eq!(app.get_current_route(), &active_radarr_block.into());
      assert!(app.should_refresh);
    }
  }

  #[test]
  fn test_system_details_handler_accepts() {
    ActiveRadarrBlock::iter().for_each(|active_radarr_block| {
      if SYSTEM_DETAILS_BLOCKS.contains(&active_radarr_block) {
        assert!(SystemDetailsHandler::accepts(&active_radarr_block));
      } else {
        assert!(!SystemDetailsHandler::accepts(&active_radarr_block));
      }
    })
  }
}
