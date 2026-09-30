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

  mod test_log_level_styling {
    use std::sync::atomic::Ordering;

    use pretty_assertions::assert_eq;
    use ratatui::style::{Modifier, Style};

    use crate::models::HorizontallyScrollableText;
    use crate::ui::styles::failure_style;
    use crate::ui::ui_test_utils::test_utils::create_test_terminal;

    use super::*;

    #[test]
    fn test_system_ui_styles_a_scrolled_log_using_its_unscrolled_level() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveRadarrBlock::System.into());
      app.data.radarr_data.logs.set_items(log_lines_vec());
      app.data.radarr_data.logs.items[1]
        .offset
        .store(30, Ordering::SeqCst);

      let style = rendered_row_style(&mut app, "CommandExecutor|log line gamma");

      assert_eq!(style.fg, failure_style().fg);
      assert!(style.add_modifier.contains(Modifier::BOLD));
    }

    #[test]
    fn test_system_ui_styles_a_fully_scrolled_log_using_its_unscrolled_level() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveRadarrBlock::System.into());
      app.data.radarr_data.logs.set_items(log_lines_vec());
      app.data.radarr_data.logs.items[1]
        .offset
        .store(46, Ordering::SeqCst);

      let style = rendered_row_style(&mut app, "log line gamma");

      assert_eq!(style.fg, failure_style().fg);
      assert!(style.add_modifier.contains(Modifier::BOLD));
    }

    fn rendered_row_style(app: &mut App<'_>, needle: &str) -> Style {
      let (width, height) = TerminalSize::Large.to_cartesian();
      let mut terminal = create_test_terminal(width, height);

      terminal
        .draw(|f| {
          SystemUi::draw(f, app, f.area());
        })
        .unwrap();

      let buffer = terminal.backend().buffer();

      for y in 0..height {
        let row = (0..width)
          .map(|x| buffer.cell((x, y)).expect("a rendered cell").symbol())
          .collect::<String>();

        if let Some(byte_index) = row.find(needle) {
          let column = row[..byte_index].chars().count() as u16;

          return buffer.cell((column, y)).expect("a rendered cell").style();
        }
      }

      panic!("no rendered row contained {needle}");
    }

    fn log_lines_vec() -> Vec<HorizontallyScrollableText> {
      vec![
        "2025-12-15 16:14:45 UTC|INFO|DownloadDecisionMaker|log line alpha".into(),
        "2025-12-16 16:41:30 UTC|FATAL|CommandExecutor|log line gamma".into(),
      ]
    }
  }

  mod snapshot_tests {
    use super::*;
    use crate::models::stateful_list::StatefulList;
    use crate::models::stateful_table::StatefulTable;
    use rstest::rstest;

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

    #[rstest]
    fn test_system_details_ui_renders_popups_over_system_tab(
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
        SystemUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(
        format!("system_details_over_system_tab_{active_radarr_block}"),
        output
      );
    }
  }
}
