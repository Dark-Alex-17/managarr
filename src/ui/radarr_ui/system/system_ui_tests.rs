#[cfg(test)]
mod tests {
  use strum::IntoEnumIterator;

  use crate::app::App;
  use crate::models::servarr_data::radarr::radarr_data::{
    ActiveRadarrBlock, SYSTEM_DETAILS_BLOCKS,
  };
  use crate::ui::DrawUi;
  use crate::ui::radarr_ui::system::SystemUi;
  use crate::ui::ui_test_utils::test_utils::{TerminalSize, render_to_string_with_app};

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

  mod snapshot_tests {
    use super::*;
    use crate::models::stateful_list::StatefulList;
    use crate::models::stateful_table::StatefulTable;

    #[test]
    fn test_radarr_ui_renders_system_tab_loading() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveRadarrBlock::System.into());
      app.is_loading = true;

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        SystemUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }

    #[test]
    fn test_radarr_ui_renders_system_tab_loading_logs() {
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack(ActiveRadarrBlock::System.into());
      app.data.radarr_data.logs = StatefulList::default();

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        SystemUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }

    #[test]
    fn test_radarr_ui_renders_system_tab_loading_events_and_tasks() {
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack(ActiveRadarrBlock::System.into());
      app.is_loading = true;

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        SystemUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }

    #[test]
    fn test_radarr_ui_renders_system_tab() {
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack(ActiveRadarrBlock::System.into());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        SystemUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }

    #[test]
    fn test_radarr_ui_renders_system_tab_empty() {
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack(ActiveRadarrBlock::System.into());
      {
        let radarr_data = &mut app.data.radarr_data;
        radarr_data.logs = StatefulList::default();
        radarr_data.tasks = StatefulTable::default();
        radarr_data.queued_events = StatefulTable::default();
      }

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        SystemUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }
  }
}
