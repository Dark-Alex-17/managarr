#[cfg(test)]
mod tests {
  use rstest::rstest;
  use strum::IntoEnumIterator;

  use crate::app::key_binding::DEFAULT_KEYBINDINGS;
  use crate::app::App;
  use crate::event::Key;
  use crate::handlers::sonarr_handlers::system::SystemHandler;
  use crate::handlers::KeyEventHandler;
  use crate::models::servarr_data::sonarr::sonarr_data::{
    ActiveSonarrBlock, SYSTEM_DETAILS_BLOCKS,
  };
  use crate::models::servarr_models::QueueEvent;
  use crate::models::sonarr_models::SonarrTask;
  use crate::test_handler_delegation;

  mod test_handle_left_right_action {
    use pretty_assertions::assert_eq;

    use super::*;

    #[rstest]
    fn test_system_tab_left(#[values(true, false)] is_ready: bool) {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveSonarrBlock::System.into());
      app.is_loading = is_ready;
      app.data.sonarr_data.main_tabs.set_index(6);

      SystemHandler::new(
        DEFAULT_KEYBINDINGS.left.key,
        &mut app,
        ActiveSonarrBlock::System,
        None,
      )
      .handle();

      assert_eq!(
        app.data.sonarr_data.main_tabs.get_active_route(),
        ActiveSonarrBlock::Indexers.into()
      );
      assert_eq!(app.get_current_route(), ActiveSonarrBlock::Indexers.into());
    }

    #[rstest]
    fn test_system_tab_right(#[values(true, false)] is_ready: bool) {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveSonarrBlock::System.into());
      app.is_loading = is_ready;
      app.data.sonarr_data.main_tabs.set_index(6);

      SystemHandler::new(
        DEFAULT_KEYBINDINGS.right.key,
        &mut app,
        ActiveSonarrBlock::System,
        None,
      )
      .handle();

      assert_eq!(
        app.data.sonarr_data.main_tabs.get_active_route(),
        ActiveSonarrBlock::Series.into()
      );
      assert_eq!(app.get_current_route(), ActiveSonarrBlock::Series.into());
    }
  }

  mod test_handle_esc {
    use pretty_assertions::assert_eq;

    use super::*;

    const ESC_KEY: Key = DEFAULT_KEYBINDINGS.esc.key;

    #[rstest]
    fn test_default_esc(#[values(true, false)] is_loading: bool) {
      let mut app = App::test_default();
      app.is_loading = is_loading;
      app.error = "test error".to_owned().into();
      app.push_navigation_stack(ActiveSonarrBlock::System.into());
      app.push_navigation_stack(ActiveSonarrBlock::System.into());

      SystemHandler::new(ESC_KEY, &mut app, ActiveSonarrBlock::System, None).handle();

      assert_eq!(app.get_current_route(), ActiveSonarrBlock::System.into());
      assert!(app.error.text.is_empty());
    }
  }

  mod test_handle_key_char {
    use pretty_assertions::{assert_eq, assert_str_eq};

    use crate::models::HorizontallyScrollableText;

    use super::*;

    #[test]
    fn test_update_system_key() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveSonarrBlock::System.into());
      app.data.sonarr_data.logs.set_items(vec![
        HorizontallyScrollableText::from("test 1"),
        HorizontallyScrollableText::from("test 2"),
      ]);
      app
        .data
        .sonarr_data
        .queued_events
        .set_items(vec![QueueEvent::default()]);
      app
        .data
        .sonarr_data
        .tasks
        .set_items(vec![SonarrTask::default()]);

      SystemHandler::new(
        DEFAULT_KEYBINDINGS.update.key,
        &mut app,
        ActiveSonarrBlock::System,
        None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveSonarrBlock::SystemUpdates.into()
      );
    }

    #[test]
    fn test_update_system_key_no_op_if_not_ready() {
      let mut app = App::test_default();
      app.is_loading = true;
      app.push_navigation_stack(ActiveSonarrBlock::System.into());
      app.data.sonarr_data.logs.set_items(vec![
        HorizontallyScrollableText::from("test 1"),
        HorizontallyScrollableText::from("test 2"),
      ]);
      app
        .data
        .sonarr_data
        .queued_events
        .set_items(vec![QueueEvent::default()]);
      app
        .data
        .sonarr_data
        .tasks
        .set_items(vec![SonarrTask::default()]);

      SystemHandler::new(
        DEFAULT_KEYBINDINGS.update.key,
        &mut app,
        ActiveSonarrBlock::System,
        None,
      )
      .handle();

      assert_eq!(app.get_current_route(), ActiveSonarrBlock::System.into());
    }

    #[test]
    fn test_queued_events_key() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveSonarrBlock::System.into());
      app.data.sonarr_data.logs.set_items(vec![
        HorizontallyScrollableText::from("test 1"),
        HorizontallyScrollableText::from("test 2"),
      ]);
      app
        .data
        .sonarr_data
        .queued_events
        .set_items(vec![QueueEvent::default()]);
      app
        .data
        .sonarr_data
        .tasks
        .set_items(vec![SonarrTask::default()]);

      SystemHandler::new(
        DEFAULT_KEYBINDINGS.events.key,
        &mut app,
        ActiveSonarrBlock::System,
        None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveSonarrBlock::SystemQueuedEvents.into()
      );
    }

    #[test]
    fn test_queued_events_key_no_op_if_not_ready() {
      let mut app = App::test_default();
      app.is_loading = true;
      app.push_navigation_stack(ActiveSonarrBlock::System.into());
      app.data.sonarr_data.logs.set_items(vec![
        HorizontallyScrollableText::from("test 1"),
        HorizontallyScrollableText::from("test 2"),
      ]);
      app
        .data
        .sonarr_data
        .queued_events
        .set_items(vec![QueueEvent::default()]);
      app
        .data
        .sonarr_data
        .tasks
        .set_items(vec![SonarrTask::default()]);

      SystemHandler::new(
        DEFAULT_KEYBINDINGS.events.key,
        &mut app,
        ActiveSonarrBlock::System,
        None,
      )
      .handle();

      assert_eq!(app.get_current_route(), ActiveSonarrBlock::System.into());
    }

    #[test]
    fn test_refresh_system_key() {
      let mut app = App::test_default();
      app.data.sonarr_data.logs.set_items(vec![
        HorizontallyScrollableText::from("test 1"),
        HorizontallyScrollableText::from("test 2"),
      ]);
      app
        .data
        .sonarr_data
        .queued_events
        .set_items(vec![QueueEvent::default()]);
      app
        .data
        .sonarr_data
        .tasks
        .set_items(vec![SonarrTask::default()]);
      app.push_navigation_stack(ActiveSonarrBlock::System.into());

      SystemHandler::new(
        DEFAULT_KEYBINDINGS.refresh.key,
        &mut app,
        ActiveSonarrBlock::System,
        None,
      )
      .handle();

      assert_eq!(app.get_current_route(), ActiveSonarrBlock::System.into());
      assert!(app.should_refresh);
    }

    #[test]
    fn test_refresh_system_key_no_op_if_not_ready() {
      let mut app = App::test_default();
      app.is_loading = true;
      app.push_navigation_stack(ActiveSonarrBlock::System.into());
      app.data.sonarr_data.logs.set_items(vec![
        HorizontallyScrollableText::from("test 1"),
        HorizontallyScrollableText::from("test 2"),
      ]);
      app
        .data
        .sonarr_data
        .queued_events
        .set_items(vec![QueueEvent::default()]);
      app
        .data
        .sonarr_data
        .tasks
        .set_items(vec![SonarrTask::default()]);
      app.push_navigation_stack(ActiveSonarrBlock::System.into());

      SystemHandler::new(
        DEFAULT_KEYBINDINGS.refresh.key,
        &mut app,
        ActiveSonarrBlock::System,
        None,
      )
      .handle();

      assert_eq!(app.get_current_route(), ActiveSonarrBlock::System.into());
      assert!(!app.should_refresh);
    }

    #[test]
    fn test_logs_key() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveSonarrBlock::System.into());
      app.data.sonarr_data.logs.set_items(vec![
        HorizontallyScrollableText::from("test 1"),
        HorizontallyScrollableText::from("test 2"),
      ]);
      app
        .data
        .sonarr_data
        .queued_events
        .set_items(vec![QueueEvent::default()]);
      app
        .data
        .sonarr_data
        .tasks
        .set_items(vec![SonarrTask::default()]);

      SystemHandler::new(
        DEFAULT_KEYBINDINGS.logs.key,
        &mut app,
        ActiveSonarrBlock::System,
        None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveSonarrBlock::SystemLogs.into()
      );
      assert_eq!(
        app.data.sonarr_data.log_details.items,
        app.data.sonarr_data.logs.items
      );
      assert_str_eq!(
        app.data.sonarr_data.log_details.current_selection().text,
        "test 2"
      );
    }

    #[test]
    fn test_logs_key_no_op_when_not_ready() {
      let mut app = App::test_default();
      app.is_loading = true;
      app.push_navigation_stack(ActiveSonarrBlock::System.into());
      app.data.sonarr_data.logs.set_items(vec![
        HorizontallyScrollableText::from("test 1"),
        HorizontallyScrollableText::from("test 2"),
      ]);
      app
        .data
        .sonarr_data
        .queued_events
        .set_items(vec![QueueEvent::default()]);
      app
        .data
        .sonarr_data
        .tasks
        .set_items(vec![SonarrTask::default()]);

      SystemHandler::new(
        DEFAULT_KEYBINDINGS.logs.key,
        &mut app,
        ActiveSonarrBlock::System,
        None,
      )
      .handle();

      assert_eq!(app.get_current_route(), ActiveSonarrBlock::System.into());
      assert!(app.data.sonarr_data.log_details.is_empty());
    }

    #[test]
    fn test_tasks_key() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveSonarrBlock::System.into());
      app.data.sonarr_data.logs.set_items(vec![
        HorizontallyScrollableText::from("test 1"),
        HorizontallyScrollableText::from("test 2"),
      ]);
      app
        .data
        .sonarr_data
        .queued_events
        .set_items(vec![QueueEvent::default()]);
      app
        .data
        .sonarr_data
        .tasks
        .set_items(vec![SonarrTask::default()]);

      SystemHandler::new(
        DEFAULT_KEYBINDINGS.tasks.key,
        &mut app,
        ActiveSonarrBlock::System,
        None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveSonarrBlock::SystemTasks.into()
      );
    }

    #[test]
    fn test_tasks_key_no_op_when_not_ready() {
      let mut app = App::test_default();
      app.is_loading = true;
      app.push_navigation_stack(ActiveSonarrBlock::System.into());
      app.data.sonarr_data.logs.set_items(vec![
        HorizontallyScrollableText::from("test 1"),
        HorizontallyScrollableText::from("test 2"),
      ]);
      app
        .data
        .sonarr_data
        .queued_events
        .set_items(vec![QueueEvent::default()]);
      app
        .data
        .sonarr_data
        .tasks
        .set_items(vec![SonarrTask::default()]);

      SystemHandler::new(
        DEFAULT_KEYBINDINGS.tasks.key,
        &mut app,
        ActiveSonarrBlock::System,
        None,
      )
      .handle();

      assert_eq!(app.get_current_route(), ActiveSonarrBlock::System.into());
    }
  }

  #[rstest]
  fn test_delegates_system_details_blocks_to_system_details_handler(
    #[values(
      ActiveSonarrBlock::SystemLogs,
      ActiveSonarrBlock::SystemQueuedEvents,
      ActiveSonarrBlock::SystemTasks,
      ActiveSonarrBlock::SystemTaskStartConfirmPrompt,
      ActiveSonarrBlock::SystemUpdates
    )]
    active_sonarr_block: ActiveSonarrBlock,
  ) {
    test_handler_delegation!(
      SystemHandler,
      ActiveSonarrBlock::System,
      active_sonarr_block
    );
  }

  #[test]
  fn test_system_handler_accepts() {
    let mut system_blocks = vec![ActiveSonarrBlock::System];
    system_blocks.extend(SYSTEM_DETAILS_BLOCKS);

    ActiveSonarrBlock::iter().for_each(|active_sonarr_block| {
      if system_blocks.contains(&active_sonarr_block) {
        assert!(SystemHandler::accepts(active_sonarr_block));
      } else {
        assert!(!SystemHandler::accepts(active_sonarr_block));
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
      ActiveSonarrBlock::default(),
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
    app.push_navigation_stack(ActiveSonarrBlock::System.into());
    app.is_loading = true;

    let system_handler = SystemHandler::new(
      DEFAULT_KEYBINDINGS.update.key,
      &mut app,
      ActiveSonarrBlock::System,
      None,
    );

    assert!(!system_handler.is_ready());
  }

  #[test]
  fn test_system_handler_is_not_ready_when_logs_is_empty() {
    let mut app = App::test_default();
    app.push_navigation_stack(ActiveSonarrBlock::System.into());
    app.is_loading = false;
    app
      .data
      .sonarr_data
      .tasks
      .set_items(vec![SonarrTask::default()]);
    app
      .data
      .sonarr_data
      .queued_events
      .set_items(vec![QueueEvent::default()]);

    let system_handler = SystemHandler::new(
      DEFAULT_KEYBINDINGS.update.key,
      &mut app,
      ActiveSonarrBlock::System,
      None,
    );

    assert!(!system_handler.is_ready());
  }

  #[test]
  fn test_system_handler_is_not_ready_when_tasks_is_empty() {
    let mut app = App::test_default();
    app.push_navigation_stack(ActiveSonarrBlock::System.into());
    app.is_loading = false;
    app.data.sonarr_data.logs.set_items(vec!["test".into()]);
    app
      .data
      .sonarr_data
      .queued_events
      .set_items(vec![QueueEvent::default()]);

    let system_handler = SystemHandler::new(
      DEFAULT_KEYBINDINGS.update.key,
      &mut app,
      ActiveSonarrBlock::System,
      None,
    );

    assert!(!system_handler.is_ready());
  }

  #[test]
  fn test_system_handler_is_not_ready_when_queued_events_is_empty() {
    let mut app = App::test_default();
    app.push_navigation_stack(ActiveSonarrBlock::System.into());
    app.is_loading = false;
    app.data.sonarr_data.logs.set_items(vec!["test".into()]);
    app
      .data
      .sonarr_data
      .tasks
      .set_items(vec![SonarrTask::default()]);

    let system_handler = SystemHandler::new(
      DEFAULT_KEYBINDINGS.update.key,
      &mut app,
      ActiveSonarrBlock::System,
      None,
    );

    assert!(!system_handler.is_ready());
  }

  #[test]
  fn test_system_handler_is_ready_when_all_required_tables_are_not_empty() {
    let mut app = App::test_default();
    app.push_navigation_stack(ActiveSonarrBlock::System.into());
    app.is_loading = false;
    app.data.sonarr_data.logs.set_items(vec!["test".into()]);
    app
      .data
      .sonarr_data
      .tasks
      .set_items(vec![SonarrTask::default()]);
    app
      .data
      .sonarr_data
      .queued_events
      .set_items(vec![QueueEvent::default()]);

    let system_handler = SystemHandler::new(
      DEFAULT_KEYBINDINGS.update.key,
      &mut app,
      ActiveSonarrBlock::System,
      None,
    );

    assert!(system_handler.is_ready());
  }
}
