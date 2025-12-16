#[cfg(test)]
mod tests {
  use strum::IntoEnumIterator;

  use crate::app::App;
  use crate::models::servarr_data::sonarr::sonarr_data::{
    ActiveSonarrBlock, SYSTEM_DETAILS_BLOCKS,
  };

  use crate::ui::DrawUi;
  use crate::ui::sonarr_ui::system::system_details_ui::SystemDetailsUi;
  use crate::ui::ui_test_utils::test_utils::render_to_string_with_app;

  #[test]
  fn test_system_details_ui_accepts() {
    ActiveSonarrBlock::iter().for_each(|active_sonarr_block| {
      if SYSTEM_DETAILS_BLOCKS.contains(&active_sonarr_block) {
        assert!(SystemDetailsUi::accepts(active_sonarr_block.into()));
      } else {
        assert!(!SystemDetailsUi::accepts(active_sonarr_block.into()));
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
        ActiveSonarrBlock::SystemLogs,
        ActiveSonarrBlock::SystemQueuedEvents,
        ActiveSonarrBlock::SystemTasks,
        ActiveSonarrBlock::SystemTaskStartConfirmPrompt,
        ActiveSonarrBlock::SystemUpdates
      )]
      active_sonarr_block: ActiveSonarrBlock,
    ) {
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack(active_sonarr_block.into());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        SystemDetailsUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(format!("popup_{active_sonarr_block}"), output);
    }

    #[rstest]
    fn test_system_details_ui_loading(
      #[values(
        ActiveSonarrBlock::SystemLogs,
        ActiveSonarrBlock::SystemQueuedEvents,
        ActiveSonarrBlock::SystemTasks,
        ActiveSonarrBlock::SystemUpdates
      )]
      active_sonarr_block: ActiveSonarrBlock,
    ) {
      let mut app = App::test_default();
      app.is_loading = true;
      app.push_navigation_stack(active_sonarr_block.into());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        SystemDetailsUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(format!("loading_{active_sonarr_block}"), output);
    }

    #[test]
    fn test_system_details_ui_updates_popup_loading_when_empty() {
      let mut app = App::test_default_fully_populated();
      app.is_loading = true;
      app.push_navigation_stack(ActiveSonarrBlock::SystemUpdates.into());
      app.data.sonarr_data.updates = ScrollableText::with_string("".to_string());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        SystemDetailsUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }

    #[rstest]
    fn test_system_details_ui_popups_empty(
      #[values(
        ActiveSonarrBlock::SystemLogs,
        ActiveSonarrBlock::SystemQueuedEvents,
        ActiveSonarrBlock::SystemTasks
      )]
      active_sonarr_block: ActiveSonarrBlock,
    ) {
      let mut app = App::test_default();
      app.push_navigation_stack(active_sonarr_block.into());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        SystemDetailsUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(format!("empty_{active_sonarr_block}"), output);
    }
  }
}
