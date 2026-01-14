#[cfg(test)]
mod tests {
  use strum::IntoEnumIterator;

  use crate::app::App;
  use crate::models::servarr_data::lidarr::lidarr_data::{
    ActiveLidarrBlock, SYSTEM_DETAILS_BLOCKS,
  };
  use crate::ui::DrawUi;
  use crate::ui::lidarr_ui::system::SystemUi;
  use crate::ui::ui_test_utils::test_utils::render_to_string_with_app;

  #[test]
  fn test_system_ui_accepts() {
    let mut system_ui_blocks = Vec::new();
    system_ui_blocks.push(ActiveLidarrBlock::System);
    system_ui_blocks.extend(SYSTEM_DETAILS_BLOCKS);

    ActiveLidarrBlock::iter().for_each(|active_lidarr_block| {
      if system_ui_blocks.contains(&active_lidarr_block) {
        assert!(SystemUi::accepts(active_lidarr_block.into()));
      } else {
        assert!(!SystemUi::accepts(active_lidarr_block.into()));
      }
    });
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
      app.push_navigation_stack(ActiveLidarrBlock::System.into());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        SystemUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }

    #[test]
    fn test_system_ui_renders_logs_loading() {
      let mut app = App::test_default_fully_populated();
      app.is_loading = true;
      app.push_navigation_stack(ActiveLidarrBlock::System.into());
      app.data.lidarr_data.logs = StatefulList::default();

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        SystemUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }

    #[test]
    fn test_system_ui_renders_system_tab_task_and_events_loading() {
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack(ActiveLidarrBlock::System.into());
      app.is_loading = true;

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        SystemUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }

    #[test]
    fn test_system_ui_renders_system_tab() {
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack(ActiveLidarrBlock::System.into());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        SystemUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }

    #[test]
    fn test_system_ui_renders_system_tab_empty() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveLidarrBlock::System.into());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        SystemUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }

    #[rstest]
    fn test_system_details_ui_renders_popups_over_system_ui(
      #[values(
        ActiveLidarrBlock::SystemLogs,
        ActiveLidarrBlock::SystemQueuedEvents,
        ActiveLidarrBlock::SystemTasks,
        ActiveLidarrBlock::SystemTaskStartConfirmPrompt,
        ActiveLidarrBlock::SystemUpdates
      )]
      active_lidarr_block: ActiveLidarrBlock,
    ) {
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack(active_lidarr_block.into());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        SystemUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(
        format!("popups_over_system_ui_{active_lidarr_block}"),
        output
      );
    }
  }
}
