#[cfg(test)]
mod tests {
  use strum::IntoEnumIterator;

  use crate::app::App;
  use crate::models::servarr_data::lidarr::lidarr_data::{
    ActiveLidarrBlock, SYSTEM_DETAILS_BLOCKS,
  };

  use crate::ui::DrawUi;
  use crate::ui::lidarr_ui::system::system_details_ui::SystemDetailsUi;
  use crate::ui::ui_test_utils::test_utils::render_to_string_with_app;

  #[test]
  fn test_system_details_ui_accepts() {
    ActiveLidarrBlock::iter().for_each(|active_lidarr_block| {
      if SYSTEM_DETAILS_BLOCKS.contains(&active_lidarr_block) {
        assert!(SystemDetailsUi::accepts(active_lidarr_block.into()));
      } else {
        assert!(!SystemDetailsUi::accepts(active_lidarr_block.into()));
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
        SystemDetailsUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(format!("popup_{active_lidarr_block}"), output);
    }

    #[rstest]
    fn test_system_details_ui_loading(
      #[values(
        ActiveLidarrBlock::SystemLogs,
        ActiveLidarrBlock::SystemQueuedEvents,
        ActiveLidarrBlock::SystemTasks,
        ActiveLidarrBlock::SystemUpdates
      )]
      active_lidarr_block: ActiveLidarrBlock,
    ) {
      let mut app = App::test_default();
      app.is_loading = true;
      app.push_navigation_stack(active_lidarr_block.into());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        SystemDetailsUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(format!("loading_{active_lidarr_block}"), output);
    }

    #[test]
    fn test_system_details_ui_updates_popup_loading_when_empty() {
      let mut app = App::test_default_fully_populated();
      app.is_loading = true;
      app.push_navigation_stack(ActiveLidarrBlock::SystemUpdates.into());
      app.data.lidarr_data.updates = ScrollableText::with_string("".to_string());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        SystemDetailsUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }

    #[rstest]
    fn test_system_details_ui_popups_empty(
      #[values(
        ActiveLidarrBlock::SystemLogs,
        ActiveLidarrBlock::SystemQueuedEvents,
        ActiveLidarrBlock::SystemTasks
      )]
      active_lidarr_block: ActiveLidarrBlock,
    ) {
      let mut app = App::test_default();
      app.push_navigation_stack(active_lidarr_block.into());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        SystemDetailsUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(format!("empty_{active_lidarr_block}"), output);
    }
  }
}
