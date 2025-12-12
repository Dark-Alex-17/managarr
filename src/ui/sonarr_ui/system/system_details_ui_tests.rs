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

    #[test]
    fn test_system_details_ui_renders_loading_tasks() {
      let mut app = App::test_default();
      app.is_loading = true;
      app.push_navigation_stack(ActiveSonarrBlock::SystemTasks.into());

      let output = render_to_string_with_app(120, 30, &mut app, |f, app| {
        SystemDetailsUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }

    #[test]
    fn test_system_details_ui_renders_logs() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveSonarrBlock::SystemLogs.into());
      app.data.sonarr_data.logs.set_items(vec![
        "2023-01-01T12:00:00Z | Info | Test log message 1"
          .to_owned()
          .into(),
        "2023-01-01T12:01:00Z | Warn | Test warning message"
          .to_owned()
          .into(),
      ]);

      let output = render_to_string_with_app(120, 30, &mut app, |f, app| {
        SystemDetailsUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }

    #[test]
    fn test_system_details_ui_renders_tasks() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveSonarrBlock::SystemTasks.into());

      let output = render_to_string_with_app(120, 30, &mut app, |f, app| {
        SystemDetailsUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }
  }
}
