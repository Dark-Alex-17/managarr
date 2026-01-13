#[cfg(test)]
mod tests {
  use rstest::rstest;
  use strum::IntoEnumIterator;

  use crate::app::App;
  use crate::app::key_binding::DEFAULT_KEYBINDINGS;
  use crate::assert_navigation_pushed;
  use crate::event::Key;
  use crate::handlers::KeyEventHandler;
  use crate::handlers::radarr_handlers::system::SystemHandler;
  use crate::models::radarr_models::RadarrTask;
  use crate::models::servarr_data::radarr::radarr_data::{
    ActiveRadarrBlock, SYSTEM_DETAILS_BLOCKS,
  };
  use crate::models::servarr_models::QueueEvent;
  use crate::test_handler_delegation;

  mod test_handle_left_right_action {
    use pretty_assertions::assert_eq;

    use super::*;
    use crate::assert_navigation_pushed;

    #[rstest]
    fn test_system_tab_left(#[values(true, false)] is_ready: bool) {
      let mut app = App::test_default();
      app.is_loading = is_ready;
      app.data.radarr_data.main_tabs.set_index(7);

      SystemHandler::new(
        DEFAULT_KEYBINDINGS.left.key,
        &mut app,
        ActiveRadarrBlock::System,
        None,
      )
      .handle();

      assert_eq!(
        app.data.radarr_data.main_tabs.get_active_route(),
        ActiveRadarrBlock::Indexers.into()
      );
      assert_navigation_pushed!(app, ActiveRadarrBlock::Indexers.into());
    }

    #[rstest]
    fn test_system_tab_right(#[values(true, false)] is_ready: bool) {
      let mut app = App::test_default();
      app.is_loading = is_ready;
      app.data.radarr_data.main_tabs.set_index(7);

      SystemHandler::new(
        DEFAULT_KEYBINDINGS.right.key,
        &mut app,
        ActiveRadarrBlock::System,
        None,
      )
      .handle();

      assert_eq!(
        app.data.radarr_data.main_tabs.get_active_route(),
        ActiveRadarrBlock::Movies.into()
      );
      assert_navigation_pushed!(app, ActiveRadarrBlock::Movies.into());
    }
  }

  mod test_handle_esc {

    use super::*;
    use crate::assert_navigation_popped;

    const ESC_KEY: Key = DEFAULT_KEYBINDINGS.esc.key;

    #[rstest]
    fn test_default_esc(#[values(true, false)] is_loading: bool) {
      let mut app = App::test_default();
      app.is_loading = is_loading;
      app.error = "test error".to_owned().into();
      app.push_navigation_stack(ActiveRadarrBlock::System.into());
      app.push_navigation_stack(ActiveRadarrBlock::System.into());

      SystemHandler::new(ESC_KEY, &mut app, ActiveRadarrBlock::System, None).handle();

      assert_navigation_popped!(app, ActiveRadarrBlock::System.into());
      assert_is_empty!(app.error.text);
    }
  }

  mod test_handle_key_char {
    use pretty_assertions::{assert_eq, assert_str_eq};

    use crate::models::HorizontallyScrollableText;

    use super::*;

    #[test]
    fn test_update_system_key() {
      let mut app = App::test_default();
      app.data.radarr_data.logs.set_items(vec![
        HorizontallyScrollableText::from("test 1"),
        HorizontallyScrollableText::from("test 2"),
      ]);
      app
        .data
        .radarr_data
        .queued_events
        .set_items(vec![QueueEvent::default()]);
      app
        .data
        .radarr_data
        .tasks
        .set_items(vec![RadarrTask::default()]);

      SystemHandler::new(
        DEFAULT_KEYBINDINGS.update.key,
        &mut app,
        ActiveRadarrBlock::System,
        None,
      )
      .handle();

      assert_navigation_pushed!(app, ActiveRadarrBlock::SystemUpdates.into());
    }

    #[test]
    fn test_update_system_key_no_op_if_not_ready() {
      let mut app = App::test_default();
      app.is_loading = true;
      app.push_navigation_stack(ActiveRadarrBlock::System.into());
      app.data.radarr_data.logs.set_items(vec![
        HorizontallyScrollableText::from("test 1"),
        HorizontallyScrollableText::from("test 2"),
      ]);
      app
        .data
        .radarr_data
        .queued_events
        .set_items(vec![QueueEvent::default()]);
      app
        .data
        .radarr_data
        .tasks
        .set_items(vec![RadarrTask::default()]);

      SystemHandler::new(
        DEFAULT_KEYBINDINGS.update.key,
        &mut app,
        ActiveRadarrBlock::System,
        None,
      )
      .handle();

      assert_eq!(app.get_current_route(), ActiveRadarrBlock::System.into());
    }

    #[test]
    fn test_queued_events_key() {
      let mut app = App::test_default();
      app.data.radarr_data.logs.set_items(vec![
        HorizontallyScrollableText::from("test 1"),
        HorizontallyScrollableText::from("test 2"),
      ]);
      app
        .data
        .radarr_data
        .queued_events
        .set_items(vec![QueueEvent::default()]);
      app
        .data
        .radarr_data
        .tasks
        .set_items(vec![RadarrTask::default()]);

      SystemHandler::new(
        DEFAULT_KEYBINDINGS.events.key,
        &mut app,
        ActiveRadarrBlock::System,
        None,
      )
      .handle();

      assert_navigation_pushed!(app, ActiveRadarrBlock::SystemQueuedEvents.into());
    }

    #[test]
    fn test_queued_events_key_no_op_if_not_ready() {
      let mut app = App::test_default();
      app.is_loading = true;
      app.push_navigation_stack(ActiveRadarrBlock::System.into());
      app.data.radarr_data.logs.set_items(vec![
        HorizontallyScrollableText::from("test 1"),
        HorizontallyScrollableText::from("test 2"),
      ]);
      app
        .data
        .radarr_data
        .queued_events
        .set_items(vec![QueueEvent::default()]);
      app
        .data
        .radarr_data
        .tasks
        .set_items(vec![RadarrTask::default()]);

      SystemHandler::new(
        DEFAULT_KEYBINDINGS.events.key,
        &mut app,
        ActiveRadarrBlock::System,
        None,
      )
      .handle();

      assert_eq!(app.get_current_route(), ActiveRadarrBlock::System.into());
    }

    #[test]
    fn test_refresh_system_key() {
      let mut app = App::test_default();
      app.data.radarr_data.logs.set_items(vec![
        HorizontallyScrollableText::from("test 1"),
        HorizontallyScrollableText::from("test 2"),
      ]);
      app
        .data
        .radarr_data
        .queued_events
        .set_items(vec![QueueEvent::default()]);
      app
        .data
        .radarr_data
        .tasks
        .set_items(vec![RadarrTask::default()]);
      app.push_navigation_stack(ActiveRadarrBlock::System.into());

      SystemHandler::new(
        DEFAULT_KEYBINDINGS.refresh.key,
        &mut app,
        ActiveRadarrBlock::System,
        None,
      )
      .handle();

      assert_navigation_pushed!(app, ActiveRadarrBlock::System.into());
      assert!(app.should_refresh);
    }

    #[test]
    fn test_refresh_system_key_no_op_if_not_ready() {
      let mut app = App::test_default();
      app.is_loading = true;
      app.push_navigation_stack(ActiveRadarrBlock::System.into());
      app.data.radarr_data.logs.set_items(vec![
        HorizontallyScrollableText::from("test 1"),
        HorizontallyScrollableText::from("test 2"),
      ]);
      app
        .data
        .radarr_data
        .queued_events
        .set_items(vec![QueueEvent::default()]);
      app
        .data
        .radarr_data
        .tasks
        .set_items(vec![RadarrTask::default()]);
      app.push_navigation_stack(ActiveRadarrBlock::System.into());

      SystemHandler::new(
        DEFAULT_KEYBINDINGS.refresh.key,
        &mut app,
        ActiveRadarrBlock::System,
        None,
      )
      .handle();

      assert_eq!(app.get_current_route(), ActiveRadarrBlock::System.into());
      assert!(!app.should_refresh);
    }

    #[test]
    fn test_logs_key() {
      let mut app = App::test_default();
      app.data.radarr_data.logs.set_items(vec![
        HorizontallyScrollableText::from("test 1"),
        HorizontallyScrollableText::from("test 2"),
      ]);
      app
        .data
        .radarr_data
        .queued_events
        .set_items(vec![QueueEvent::default()]);
      app
        .data
        .radarr_data
        .tasks
        .set_items(vec![RadarrTask::default()]);

      SystemHandler::new(
        DEFAULT_KEYBINDINGS.logs.key,
        &mut app,
        ActiveRadarrBlock::System,
        None,
      )
      .handle();

      assert_navigation_pushed!(app, ActiveRadarrBlock::SystemLogs.into());
      assert_eq!(
        app.data.radarr_data.log_details.items,
        app.data.radarr_data.logs.items
      );
      assert_str_eq!(
        app.data.radarr_data.log_details.current_selection().text,
        "test 2"
      );
    }

    #[test]
    fn test_logs_key_no_op_when_not_ready() {
      let mut app = App::test_default();
      app.is_loading = true;
      app.push_navigation_stack(ActiveRadarrBlock::System.into());
      app.data.radarr_data.logs.set_items(vec![
        HorizontallyScrollableText::from("test 1"),
        HorizontallyScrollableText::from("test 2"),
      ]);
      app
        .data
        .radarr_data
        .queued_events
        .set_items(vec![QueueEvent::default()]);
      app
        .data
        .radarr_data
        .tasks
        .set_items(vec![RadarrTask::default()]);

      SystemHandler::new(
        DEFAULT_KEYBINDINGS.logs.key,
        &mut app,
        ActiveRadarrBlock::System,
        None,
      )
      .handle();

      assert_eq!(app.get_current_route(), ActiveRadarrBlock::System.into());
      assert_is_empty!(app.data.radarr_data.log_details);
    }

    #[test]
    fn test_tasks_key() {
      let mut app = App::test_default();
      app.data.radarr_data.logs.set_items(vec![
        HorizontallyScrollableText::from("test 1"),
        HorizontallyScrollableText::from("test 2"),
      ]);
      app
        .data
        .radarr_data
        .queued_events
        .set_items(vec![QueueEvent::default()]);
      app
        .data
        .radarr_data
        .tasks
        .set_items(vec![RadarrTask::default()]);

      SystemHandler::new(
        DEFAULT_KEYBINDINGS.tasks.key,
        &mut app,
        ActiveRadarrBlock::System,
        None,
      )
      .handle();

      assert_navigation_pushed!(app, ActiveRadarrBlock::SystemTasks.into());
    }

    #[test]
    fn test_tasks_key_no_op_when_not_ready() {
      let mut app = App::test_default();
      app.is_loading = true;
      app.push_navigation_stack(ActiveRadarrBlock::System.into());
      app.data.radarr_data.logs.set_items(vec![
        HorizontallyScrollableText::from("test 1"),
        HorizontallyScrollableText::from("test 2"),
      ]);
      app
        .data
        .radarr_data
        .queued_events
        .set_items(vec![QueueEvent::default()]);
      app
        .data
        .radarr_data
        .tasks
        .set_items(vec![RadarrTask::default()]);

      SystemHandler::new(
        DEFAULT_KEYBINDINGS.tasks.key,
        &mut app,
        ActiveRadarrBlock::System,
        None,
      )
      .handle();

      assert_eq!(app.get_current_route(), ActiveRadarrBlock::System.into());
    }
  }

  #[rstest]
  fn test_delegates_system_details_blocks_to_system_details_handler(
    #[values(
      ActiveRadarrBlock::SystemLogs,
      ActiveRadarrBlock::SystemQueuedEvents,
      ActiveRadarrBlock::SystemTasks,
      ActiveRadarrBlock::SystemTaskStartConfirmPrompt,
      ActiveRadarrBlock::SystemUpdates
    )]
    active_radarr_block: ActiveRadarrBlock,
  ) {
    test_handler_delegation!(
      SystemHandler,
      ActiveRadarrBlock::System,
      active_radarr_block
    );
  }

  #[test]
  fn test_system_handler_accepts() {
    let mut system_blocks = vec![ActiveRadarrBlock::System];
    system_blocks.extend(SYSTEM_DETAILS_BLOCKS);

    ActiveRadarrBlock::iter().for_each(|active_radarr_block| {
      if system_blocks.contains(&active_radarr_block) {
        assert!(SystemHandler::accepts(active_radarr_block));
      } else {
        assert!(!SystemHandler::accepts(active_radarr_block));
      }
    })
  }

  #[rstest]
  fn test_system_handler_ignore_special_keys(
    #[values(true, false)] ignore_special_keys_for_textbox_input: bool,
  ) {
    let mut app = App::test_default();
    app.ignore_special_keys_for_textbox_input = ignore_special_keys_for_textbox_input;
    let handler = SystemHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveRadarrBlock::default(),
      None,
    );

    assert_eq!(
      handler.ignore_special_keys(),
      ignore_special_keys_for_textbox_input
    );
  }

  #[test]
  fn test_system_handler_is_not_ready_when_loading() {
    let mut app = App::test_default();
    app.is_loading = true;

    let system_handler = SystemHandler::new(
      DEFAULT_KEYBINDINGS.update.key,
      &mut app,
      ActiveRadarrBlock::System,
      None,
    );

    assert!(!system_handler.is_ready());
  }

  #[test]
  fn test_system_handler_is_not_ready_when_logs_is_empty() {
    let mut app = App::test_default();
    app.is_loading = false;
    app
      .data
      .radarr_data
      .tasks
      .set_items(vec![RadarrTask::default()]);
    app
      .data
      .radarr_data
      .queued_events
      .set_items(vec![QueueEvent::default()]);

    let system_handler = SystemHandler::new(
      DEFAULT_KEYBINDINGS.update.key,
      &mut app,
      ActiveRadarrBlock::System,
      None,
    );

    assert!(!system_handler.is_ready());
  }

  #[test]
  fn test_system_handler_is_not_ready_when_tasks_is_empty() {
    let mut app = App::test_default();
    app.is_loading = false;
    app.data.radarr_data.logs.set_items(vec!["test".into()]);
    app
      .data
      .radarr_data
      .queued_events
      .set_items(vec![QueueEvent::default()]);

    let system_handler = SystemHandler::new(
      DEFAULT_KEYBINDINGS.update.key,
      &mut app,
      ActiveRadarrBlock::System,
      None,
    );

    assert!(!system_handler.is_ready());
  }

  #[test]
  fn test_system_handler_is_not_ready_when_queued_events_is_empty() {
    let mut app = App::test_default();
    app.is_loading = false;
    app.data.radarr_data.logs.set_items(vec!["test".into()]);
    app
      .data
      .radarr_data
      .tasks
      .set_items(vec![RadarrTask::default()]);

    let system_handler = SystemHandler::new(
      DEFAULT_KEYBINDINGS.update.key,
      &mut app,
      ActiveRadarrBlock::System,
      None,
    );

    assert!(!system_handler.is_ready());
  }

  #[test]
  fn test_system_handler_is_ready_when_all_required_tables_are_not_empty() {
    let mut app = App::test_default();
    app.is_loading = false;
    app.data.radarr_data.logs.set_items(vec!["test".into()]);
    app
      .data
      .radarr_data
      .tasks
      .set_items(vec![RadarrTask::default()]);
    app
      .data
      .radarr_data
      .queued_events
      .set_items(vec![QueueEvent::default()]);

    let system_handler = SystemHandler::new(
      DEFAULT_KEYBINDINGS.update.key,
      &mut app,
      ActiveRadarrBlock::System,
      None,
    );

    assert!(system_handler.is_ready());
  }
}
