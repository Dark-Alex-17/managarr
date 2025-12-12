#[cfg(test)]
mod tests {
  use strum::IntoEnumIterator;

  use crate::app::App;
  use crate::models::servarr_data::radarr::radarr_data::{
    ActiveRadarrBlock, SYSTEM_DETAILS_BLOCKS,
  };
  use crate::ui::DrawUi;
  use crate::ui::radarr_ui::system::SystemUi;
  use crate::ui::ui_test_utils::test_utils::render_to_string_with_app;

  #[test]
  fn test_system_ui_accepts() {
    let mut system_ui_blocks = Vec::new();
    system_ui_blocks.push(ActiveRadarrBlock::System);
    system_ui_blocks.extend(SYSTEM_DETAILS_BLOCKS);

    ActiveRadarrBlock::iter().for_each(|active_radarr_block| {
      if system_ui_blocks.contains(&active_radarr_block) {
        assert!(SystemUi::accepts(active_radarr_block.into()));
      } else {
        assert!(!SystemUi::accepts(active_radarr_block.into()));
      }
    });
  }

  #[test]
  fn test_system_ui_renders_loading_state() {
    let mut app = App::test_default();
    app.is_loading = true;
    app.push_navigation_stack(ActiveRadarrBlock::System.into());

    let output = render_to_string_with_app(120, 30, &mut app, |f, app| {
      SystemUi::draw(f, app, f.area());
    });

    insta::assert_snapshot!(output);
  }

  #[test]
  fn test_system_ui_renders_system_menu() {
    let mut app = App::test_default();
    app.push_navigation_stack(ActiveRadarrBlock::System.into());

    let output = render_to_string_with_app(120, 30, &mut app, |f, app| {
      SystemUi::draw(f, app, f.area());
    });

    insta::assert_snapshot!(output);
  }
}
