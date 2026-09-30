#[cfg(test)]
mod tests {
  use rstest::rstest;
  use strum::IntoEnumIterator;

  use crate::app::App;
  use crate::app::key_binding::DEFAULT_KEYBINDINGS;
  use crate::assert_navigation_pushed;
  use crate::event::Key;
  use crate::handlers::KeyEventHandler;
  use crate::handlers::readarr_handlers::system::SystemHandler;
  use crate::models::HorizontallyScrollableText;
  use crate::models::readarr_models::{ReadarrTask, ReadarrTaskName};
  use crate::models::servarr_data::readarr::readarr_data::{
    ActiveReadarrBlock, SYSTEM_DETAILS_BLOCKS,
  };
  use crate::models::servarr_models::QueueEvent;
  use crate::test_handler_delegation;

  mod test_handle_left_right_action {
    use pretty_assertions::assert_eq;

    use super::*;
    use crate::assert_navigation_pushed;

    #[rstest]
    fn test_system_tab_left(#[values(true, false)] is_loading: bool) {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveReadarrBlock::System.into());
      app.is_loading = is_loading;
      app.data.readarr_data.main_tabs.set_index(6);

      SystemHandler::new(
        DEFAULT_KEYBINDINGS.left.key,
        &mut app,
        ActiveReadarrBlock::System,
        None,
      )
      .handle();

      assert_eq!(
        app.data.readarr_data.main_tabs.get_active_route(),
        ActiveReadarrBlock::Indexers.into()
      );
      assert_navigation_pushed!(app, ActiveReadarrBlock::Indexers.into());
    }

    #[rstest]
    fn test_system_tab_right(#[values(true, false)] is_loading: bool) {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveReadarrBlock::System.into());
      app.is_loading = is_loading;
      app.data.readarr_data.main_tabs.set_index(6);

      SystemHandler::new(
        DEFAULT_KEYBINDINGS.right.key,
        &mut app,
        ActiveReadarrBlock::System,
        None,
      )
      .handle();

      assert_eq!(
        app.data.readarr_data.main_tabs.get_active_route(),
        ActiveReadarrBlock::Authors.into()
      );
      assert_navigation_pushed!(app, ActiveReadarrBlock::Authors.into());
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
      app.push_navigation_stack(ActiveReadarrBlock::System.into());
      app.push_navigation_stack(ActiveReadarrBlock::System.into());

      SystemHandler::new(ESC_KEY, &mut app, ActiveReadarrBlock::System, None).handle();

      assert_navigation_popped!(app, ActiveReadarrBlock::System.into());
      assert_is_empty!(app.error.text);
    }
  }

  mod test_handle_key_char {
    use pretty_assertions::{assert_eq, assert_str_eq};

    use super::*;

    #[test]
    fn test_update_system_key() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveReadarrBlock::System.into());
      seed_system_data(&mut app);

      SystemHandler::new(
        DEFAULT_KEYBINDINGS.update.key,
        &mut app,
        ActiveReadarrBlock::System,
        None,
      )
      .handle();

      assert_navigation_pushed!(app, ActiveReadarrBlock::SystemUpdates.into());
    }

    #[test]
    fn test_update_system_key_no_op_if_not_ready() {
      let mut app = App::test_default();
      app.is_loading = true;
      app.push_navigation_stack(ActiveReadarrBlock::System.into());
      seed_system_data(&mut app);

      SystemHandler::new(
        DEFAULT_KEYBINDINGS.update.key,
        &mut app,
        ActiveReadarrBlock::System,
        None,
      )
      .handle();

      assert_eq!(app.get_current_route(), ActiveReadarrBlock::System.into());
    }

    #[test]
    fn test_queued_events_key() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveReadarrBlock::System.into());
      seed_system_data(&mut app);

      SystemHandler::new(
        DEFAULT_KEYBINDINGS.events.key,
        &mut app,
        ActiveReadarrBlock::System,
        None,
      )
      .handle();

      assert_navigation_pushed!(app, ActiveReadarrBlock::SystemQueuedEvents.into());
    }

    #[test]
    fn test_queued_events_key_no_op_if_not_ready() {
      let mut app = App::test_default();
      app.is_loading = true;
      app.push_navigation_stack(ActiveReadarrBlock::System.into());
      seed_system_data(&mut app);

      SystemHandler::new(
        DEFAULT_KEYBINDINGS.events.key,
        &mut app,
        ActiveReadarrBlock::System,
        None,
      )
      .handle();

      assert_eq!(app.get_current_route(), ActiveReadarrBlock::System.into());
    }

    #[test]
    fn test_refresh_system_key() {
      let mut app = App::test_default();
      seed_system_data(&mut app);
      app.push_navigation_stack(ActiveReadarrBlock::System.into());

      SystemHandler::new(
        DEFAULT_KEYBINDINGS.refresh.key,
        &mut app,
        ActiveReadarrBlock::System,
        None,
      )
      .handle();

      assert_navigation_pushed!(app, ActiveReadarrBlock::System.into());
      assert!(app.should_refresh);
    }

    #[test]
    fn test_refresh_system_key_no_op_if_not_ready() {
      let mut app = App::test_default();
      app.is_loading = true;
      seed_system_data(&mut app);
      app.push_navigation_stack(ActiveReadarrBlock::System.into());

      SystemHandler::new(
        DEFAULT_KEYBINDINGS.refresh.key,
        &mut app,
        ActiveReadarrBlock::System,
        None,
      )
      .handle();

      assert_eq!(app.get_current_route(), ActiveReadarrBlock::System.into());
      assert!(!app.should_refresh);
    }

    #[test]
    fn test_logs_key() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveReadarrBlock::System.into());
      seed_system_data(&mut app);

      SystemHandler::new(
        DEFAULT_KEYBINDINGS.logs.key,
        &mut app,
        ActiveReadarrBlock::System,
        None,
      )
      .handle();

      assert_navigation_pushed!(app, ActiveReadarrBlock::SystemLogs.into());
      assert_eq!(
        app.data.readarr_data.log_details.items,
        app.data.readarr_data.logs.items
      );
      assert_str_eq!(
        app.data.readarr_data.log_details.current_selection().text,
        "log line gamma"
      );
    }

    #[test]
    fn test_logs_key_no_op_when_not_ready() {
      let mut app = App::test_default();
      app.is_loading = true;
      app.push_navigation_stack(ActiveReadarrBlock::System.into());
      seed_system_data(&mut app);

      SystemHandler::new(
        DEFAULT_KEYBINDINGS.logs.key,
        &mut app,
        ActiveReadarrBlock::System,
        None,
      )
      .handle();

      assert_eq!(app.get_current_route(), ActiveReadarrBlock::System.into());
      assert_is_empty!(app.data.readarr_data.log_details);
    }

    #[test]
    fn test_tasks_key() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveReadarrBlock::System.into());
      seed_system_data(&mut app);

      SystemHandler::new(
        DEFAULT_KEYBINDINGS.tasks.key,
        &mut app,
        ActiveReadarrBlock::System,
        None,
      )
      .handle();

      assert_navigation_pushed!(app, ActiveReadarrBlock::SystemTasks.into());
    }

    #[test]
    fn test_tasks_key_no_op_when_not_ready() {
      let mut app = App::test_default();
      app.is_loading = true;
      app.push_navigation_stack(ActiveReadarrBlock::System.into());
      seed_system_data(&mut app);

      SystemHandler::new(
        DEFAULT_KEYBINDINGS.tasks.key,
        &mut app,
        ActiveReadarrBlock::System,
        None,
      )
      .handle();

      assert_eq!(app.get_current_route(), ActiveReadarrBlock::System.into());
    }

    #[test]
    fn test_unmapped_key_is_a_no_op() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveReadarrBlock::System.into());
      seed_system_data(&mut app);

      SystemHandler::new(
        DEFAULT_KEYBINDINGS.add.key,
        &mut app,
        ActiveReadarrBlock::System,
        None,
      )
      .handle();

      assert_eq!(app.get_current_route(), ActiveReadarrBlock::System.into());
      assert!(!app.should_refresh);
      assert_is_empty!(app.data.readarr_data.log_details);
    }
  }

  #[rstest]
  fn test_delegates_system_details_blocks_to_system_details_handler(
    #[values(
      ActiveReadarrBlock::SystemLogs,
      ActiveReadarrBlock::SystemQueuedEvents,
      ActiveReadarrBlock::SystemTasks,
      ActiveReadarrBlock::SystemTaskStartConfirmPrompt,
      ActiveReadarrBlock::SystemUpdates
    )]
    active_readarr_block: ActiveReadarrBlock,
  ) {
    test_handler_delegation!(
      SystemHandler,
      ActiveReadarrBlock::System,
      active_readarr_block
    );
  }

  #[test]
  fn test_system_handler_accepts() {
    let mut system_blocks = vec![ActiveReadarrBlock::System];
    system_blocks.extend(SYSTEM_DETAILS_BLOCKS);

    ActiveReadarrBlock::iter().for_each(|active_readarr_block| {
      if system_blocks.contains(&active_readarr_block) {
        assert!(SystemHandler::accepts(active_readarr_block));
      } else {
        assert!(!SystemHandler::accepts(active_readarr_block));
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
      ActiveReadarrBlock::default(),
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
    app.push_navigation_stack(ActiveReadarrBlock::System.into());
    app.is_loading = true;
    seed_system_data(&mut app);

    let system_handler = SystemHandler::new(
      DEFAULT_KEYBINDINGS.update.key,
      &mut app,
      ActiveReadarrBlock::System,
      None,
    );

    assert!(!system_handler.is_ready());
  }

  #[test]
  fn test_system_handler_is_not_ready_when_logs_is_empty() {
    let mut app = App::test_default();
    app.push_navigation_stack(ActiveReadarrBlock::System.into());
    app.is_loading = false;
    app.data.readarr_data.tasks.set_items(tasks_vec());
    app
      .data
      .readarr_data
      .queued_events
      .set_items(queued_events_vec());

    let system_handler = SystemHandler::new(
      DEFAULT_KEYBINDINGS.update.key,
      &mut app,
      ActiveReadarrBlock::System,
      None,
    );

    assert!(!system_handler.is_ready());
  }

  #[test]
  fn test_system_handler_is_not_ready_when_tasks_is_empty() {
    let mut app = App::test_default();
    app.push_navigation_stack(ActiveReadarrBlock::System.into());
    app.is_loading = false;
    app.data.readarr_data.logs.set_items(logs_vec());
    app
      .data
      .readarr_data
      .queued_events
      .set_items(queued_events_vec());

    let system_handler = SystemHandler::new(
      DEFAULT_KEYBINDINGS.update.key,
      &mut app,
      ActiveReadarrBlock::System,
      None,
    );

    assert!(!system_handler.is_ready());
  }

  #[test]
  fn test_system_handler_is_not_ready_when_queued_events_is_empty() {
    let mut app = App::test_default();
    app.push_navigation_stack(ActiveReadarrBlock::System.into());
    app.is_loading = false;
    app.data.readarr_data.logs.set_items(logs_vec());
    app.data.readarr_data.tasks.set_items(tasks_vec());

    let system_handler = SystemHandler::new(
      DEFAULT_KEYBINDINGS.update.key,
      &mut app,
      ActiveReadarrBlock::System,
      None,
    );

    assert!(!system_handler.is_ready());
  }

  #[test]
  fn test_system_handler_is_ready_when_all_required_tables_are_not_empty() {
    let mut app = App::test_default();
    app.push_navigation_stack(ActiveReadarrBlock::System.into());
    app.is_loading = false;
    seed_system_data(&mut app);

    let system_handler = SystemHandler::new(
      DEFAULT_KEYBINDINGS.update.key,
      &mut app,
      ActiveReadarrBlock::System,
      None,
    );

    assert!(system_handler.is_ready());
  }

  fn seed_system_data(app: &mut App<'_>) {
    app.data.readarr_data.logs.set_items(logs_vec());
    app
      .data
      .readarr_data
      .queued_events
      .set_items(queued_events_vec());
    app.data.readarr_data.tasks.set_items(tasks_vec());
  }

  fn logs_vec() -> Vec<HorizontallyScrollableText> {
    vec![
      HorizontallyScrollableText::from("log line alpha"),
      HorizontallyScrollableText::from("log line beta"),
      HorizontallyScrollableText::from("log line gamma"),
    ]
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
}
