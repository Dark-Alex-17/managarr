#[cfg(test)]
mod tests {
  use strum::IntoEnumIterator;

  use crate::app::App;
  use crate::models::servarr_data::sonarr::sonarr_data::{
    ActiveSonarrBlock, SYSTEM_DETAILS_BLOCKS,
  };
  use crate::ui::DrawUi;
  use crate::ui::sonarr_ui::system::SystemUi;
  use crate::ui::ui_test_utils::test_utils::render_to_string_with_app;

  #[test]
  fn test_system_ui_accepts() {
    let mut system_ui_blocks = Vec::new();
    system_ui_blocks.push(ActiveSonarrBlock::System);
    system_ui_blocks.extend(SYSTEM_DETAILS_BLOCKS);

    ActiveSonarrBlock::iter().for_each(|active_sonarr_block| {
      if system_ui_blocks.contains(&active_sonarr_block) {
        assert!(SystemUi::accepts(active_sonarr_block.into()));
      } else {
        assert!(!SystemUi::accepts(active_sonarr_block.into()));
      }
    });
  }

  mod snapshot_tests {
    use crate::ui::ui_test_utils::test_utils::TerminalSize;

    use super::*;

    #[test]
    fn test_system_ui_renders_loading_state() {
      let mut app = App::test_default();
      app.is_loading = true;
      app.push_navigation_stack(ActiveSonarrBlock::System.into());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        SystemUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }

    #[test]
    fn test_system_ui_renders_system_menu() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveSonarrBlock::System.into());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        SystemUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }
  }
}
