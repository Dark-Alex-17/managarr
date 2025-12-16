#[cfg(test)]
mod tests {
  use strum::IntoEnumIterator;

  use crate::app::App;
  use crate::models::servarr_data::radarr::radarr_data::ActiveRadarrBlock;
  use crate::ui::DrawUi;
  use crate::ui::radarr_ui::indexers::test_all_indexers_ui::TestAllIndexersUi;
  use crate::ui::ui_test_utils::test_utils::render_to_string_with_app;

  #[test]
  fn test_test_all_indexers_ui_accepts() {
    ActiveRadarrBlock::iter().for_each(|active_radarr_block| {
      if active_radarr_block == ActiveRadarrBlock::TestAllIndexers {
        assert!(TestAllIndexersUi::accepts(active_radarr_block.into()));
      } else {
        assert!(!TestAllIndexersUi::accepts(active_radarr_block.into()));
      }
    });
  }

  mod snapshot_tests {
    use crate::ui::ui_test_utils::test_utils::TerminalSize;

    use super::*;

    #[test]
    fn test_test_all_indexers_ui_renders_loading_state() {
      let mut app = App::test_default();
      app.is_loading = true;
      app.push_navigation_stack(ActiveRadarrBlock::TestAllIndexers.into());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        TestAllIndexersUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }
  }
}
