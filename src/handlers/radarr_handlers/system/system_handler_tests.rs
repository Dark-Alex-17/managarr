#[cfg(test)]
mod tests {
  use pretty_assertions::assert_eq;
  use rstest::rstest;
  use strum::IntoEnumIterator;

  use crate::app::key_binding::DEFAULT_KEYBINDINGS;
  use crate::app::App;
  use crate::event::Key;
  use crate::handlers::radarr_handlers::system::SystemHandler;
  use crate::handlers::KeyEventHandler;
  use crate::models::servarr_data::radarr::radarr_data::{
    ActiveRadarrBlock, SYSTEM_DETAILS_BLOCKS,
  };
  use crate::test_handler_delegation;

  mod test_handle_left_right_action {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_system_tab_left() {
      let mut app = App::default();
      app.data.radarr_data.main_tabs.set_index(6);

      SystemHandler::with(
        &DEFAULT_KEYBINDINGS.left.key,
        &mut app,
        &ActiveRadarrBlock::System,
        &None,
      )
      .handle();

      assert_eq!(
        app.data.radarr_data.main_tabs.get_active_route(),
        &ActiveRadarrBlock::Indexers.into()
      );
      assert_eq!(app.get_current_route(), &ActiveRadarrBlock::Indexers.into());
    }

    #[test]
    fn test_system_tab_right() {
      let mut app = App::default();
      app.data.radarr_data.main_tabs.set_index(6);

      SystemHandler::with(
        &DEFAULT_KEYBINDINGS.right.key,
        &mut app,
        &ActiveRadarrBlock::System,
        &None,
      )
      .handle();

      assert_eq!(
        app.data.radarr_data.main_tabs.get_active_route(),
        &ActiveRadarrBlock::Movies.into()
      );
      assert_eq!(app.get_current_route(), &ActiveRadarrBlock::Movies.into());
    }
  }

  mod test_handle_esc {
    use pretty_assertions::assert_eq;

    use super::*;

    const ESC_KEY: Key = DEFAULT_KEYBINDINGS.esc.key;

    #[test]
    fn test_default_esc() {
      let mut app = App::default();
      app.error = "test error".to_owned().into();
      app.push_navigation_stack(ActiveRadarrBlock::System.into());
      app.push_navigation_stack(ActiveRadarrBlock::System.into());

      SystemHandler::with(&ESC_KEY, &mut app, &ActiveRadarrBlock::System, &None).handle();

      assert_eq!(app.get_current_route(), &ActiveRadarrBlock::System.into());
      assert!(app.error.text.is_empty());
    }
  }

  mod test_handle_key_char {
    use pretty_assertions::{assert_eq, assert_str_eq};

    use crate::assert_refresh_key;
    use crate::models::HorizontallyScrollableText;

    use super::*;

    #[test]
    fn test_update_system_key() {
      let mut app = App::default();

      SystemHandler::with(
        &DEFAULT_KEYBINDINGS.update.key,
        &mut app,
        &ActiveRadarrBlock::System,
        &None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        &ActiveRadarrBlock::SystemUpdates.into()
      );
    }

    #[test]
    fn test_queued_events_key() {
      let mut app = App::default();

      SystemHandler::with(
        &DEFAULT_KEYBINDINGS.events.key,
        &mut app,
        &ActiveRadarrBlock::System,
        &None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        &ActiveRadarrBlock::SystemQueuedEvents.into()
      );
    }

    #[test]
    fn test_refresh_system_key() {
      assert_refresh_key!(SystemHandler, ActiveRadarrBlock::System);
    }

    #[test]
    fn test_logs_key() {
      let mut app = App::default();
      app.data.radarr_data.logs.set_items(vec![
        HorizontallyScrollableText::from("test 1"),
        HorizontallyScrollableText::from("test 2"),
      ]);

      SystemHandler::with(
        &DEFAULT_KEYBINDINGS.logs.key,
        &mut app,
        &ActiveRadarrBlock::System,
        &None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        &ActiveRadarrBlock::SystemLogs.into()
      );
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
    fn test_tasks_key() {
      let mut app = App::default();

      SystemHandler::with(
        &DEFAULT_KEYBINDINGS.tasks.key,
        &mut app,
        &ActiveRadarrBlock::System,
        &None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        &ActiveRadarrBlock::SystemTasks.into()
      );
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
        assert!(SystemHandler::accepts(&active_radarr_block));
      } else {
        assert!(!SystemHandler::accepts(&active_radarr_block));
      }
    })
  }
}
