#[cfg(test)]
mod tests {
  use strum::IntoEnumIterator;

  use crate::app::App;
  use crate::models::servarr_data::radarr::radarr_data::{
    ActiveRadarrBlock, SYSTEM_DETAILS_BLOCKS,
  };

  use crate::ui::DrawUi;
  use crate::ui::radarr_ui::system::system_details_ui::SystemDetailsUi;
  use crate::ui::ui_test_utils::test_utils::{TerminalSize, render_to_string_with_app};

  #[test]
  fn test_system_details_ui_accepts() {
    ActiveRadarrBlock::iter().for_each(|active_radarr_block| {
      if SYSTEM_DETAILS_BLOCKS.contains(&active_radarr_block) {
        assert!(SystemDetailsUi::accepts(active_radarr_block.into()));
      } else {
        assert!(!SystemDetailsUi::accepts(active_radarr_block.into()));
      }
    });
  }

  mod snapshot_tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    fn test_system_details_ui_renders_tasks(
      #[values(
        ActiveRadarrBlock::SystemLogs,
        ActiveRadarrBlock::SystemQueuedEvents,
        ActiveRadarrBlock::SystemTasks,
        ActiveRadarrBlock::SystemTaskStartConfirmPrompt,
        ActiveRadarrBlock::SystemUpdates
      )]
      active_radarr_block: ActiveRadarrBlock,
    ) {
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack(active_radarr_block.into());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        SystemDetailsUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(active_radarr_block.to_string(), output);
    }

    #[test]
    fn test_system_details_ui_renders_tasks_loading() {
      let mut app = App::test_default_fully_populated();
      app.is_loading = true;
      app.push_navigation_stack(ActiveRadarrBlock::SystemTasks.into());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        SystemDetailsUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }

    #[test]
    fn test_system_details_ui_renders_queued_events_loading() {
      let mut app = App::test_default_fully_populated();
      app.is_loading = true;
      app.push_navigation_stack(ActiveRadarrBlock::SystemQueuedEvents.into());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        SystemDetailsUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }

    #[test]
    fn test_system_details_ui_renders_logs_loading() {
      let mut app = App::test_default_fully_populated();
      app.is_loading = true;
      app.push_navigation_stack(ActiveRadarrBlock::SystemLogs.into());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        SystemDetailsUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }
  }
}
