#[cfg(test)]
mod tests {
  use pretty_assertions::assert_eq;
  use ratatui::style::Modifier;
  use ratatui::text::Line;
  use strum::IntoEnumIterator;

  use crate::app::App;
  use crate::models::servarr_data::sonarr::sonarr_data::{
    ActiveSonarrBlock, SERIES_OVERVIEW_BLOCKS,
  };
  use crate::models::sonarr_models::Series;
  use crate::ui::DrawUi;
  use crate::ui::sonarr_ui::library::series_overview_ui::{
    SeriesOverviewUi, overview_line, style_from_series,
  };
  use crate::ui::styles::{primary_style, unmonitored_style};
  use crate::ui::ui_test_utils::test_utils::render_to_string_with_app;

  #[test]
  fn test_series_overview_ui_accepts() {
    ActiveSonarrBlock::iter().for_each(|active_sonarr_block| {
      if SERIES_OVERVIEW_BLOCKS.contains(&active_sonarr_block) {
        assert!(SeriesOverviewUi::accepts(active_sonarr_block.into()));
      } else {
        assert!(!SeriesOverviewUi::accepts(active_sonarr_block.into()));
      }
    });
  }

  #[test]
  fn test_style_from_series_monitored() {
    let series = Series {
      monitored: true,
      ..Series::default()
    };

    let style = style_from_series(&series);

    assert_eq!(style, primary_style());
  }

  #[test]
  fn test_style_from_series_unmonitored() {
    let series = Series {
      monitored: false,
      ..Series::default()
    };

    let style = style_from_series(&series);

    assert_eq!(style, unmonitored_style());
  }

  #[test]
  fn test_overview_line_does_not_split_a_label_from_a_colon_in_the_prose() {
    let line = overview_line(
      "It was filmed in Madison, Wisconsin: a city the crew never really left.",
      primary_style(),
    );

    assert_eq!(
      span_contents(&line),
      vec!["It was filmed in Madison, Wisconsin: a city the crew never really left."]
    );
  }

  #[test]
  fn test_overview_line_renders_the_prose_without_bolding_it() {
    let line = overview_line("Overview: a prose line", primary_style());

    assert_eq!(line.spans.len(), 1);
    assert!(!line.spans[0].style.add_modifier.contains(Modifier::BOLD));
  }

  #[test]
  fn test_overview_line_applies_the_given_style() {
    let line = overview_line("an unmonitored series", unmonitored_style());

    assert_eq!(line.spans[0].style.fg, unmonitored_style().fg);
  }

  #[test]
  fn test_overview_line_trims_a_trailing_carriage_return() {
    let line = overview_line(
      "The first season took seven years to finish.\r",
      primary_style(),
    );

    assert_eq!(
      span_contents(&line),
      vec!["The first season took seven years to finish."]
    );
  }

  #[test]
  fn test_overview_line_renders_a_carriage_return_only_line_as_blank() {
    let line = overview_line("\r", primary_style());

    assert_eq!(span_contents(&line), vec![""]);
  }

  fn span_contents(line: &Line<'_>) -> Vec<String> {
    line
      .spans
      .iter()
      .map(|span| span.content.to_string())
      .collect()
  }

  mod snapshot_tests {
    use crate::models::ScrollableText;
    use crate::models::servarr_data::sonarr::modals::SeriesOverviewModal;
    use crate::models::stateful_table::StatefulTable;
    use crate::ui::ui_test_utils::test_utils::TerminalSize;

    use super::*;

    #[test]
    fn test_series_overview_ui_renders() {
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack(ActiveSonarrBlock::SeriesOverview.into());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        SeriesOverviewUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }

    #[test]
    fn test_series_overview_ui_renders_loading() {
      let mut app = App::test_default_fully_populated();
      app.is_loading = true;
      app.push_navigation_stack(ActiveSonarrBlock::SeriesOverview.into());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        SeriesOverviewUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }

    #[test]
    fn test_series_overview_ui_renders_scrolled() {
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack(ActiveSonarrBlock::SeriesOverview.into());
      app
        .data
        .sonarr_data
        .series_overview_modal
        .as_mut()
        .unwrap()
        .overview
        .offset = 2;

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        SeriesOverviewUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }

    #[test]
    fn test_series_overview_ui_renders_a_series_with_no_overview() {
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack(ActiveSonarrBlock::SeriesOverview.into());
      app.data.sonarr_data.series_overview_modal = Some(SeriesOverviewModal {
        overview: ScrollableText::with_string(String::new()),
      });

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        SeriesOverviewUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }

    #[test]
    fn test_series_overview_ui_renders_nothing_when_the_series_overview_modal_is_undefined() {
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack(ActiveSonarrBlock::SeriesOverview.into());
      app.data.sonarr_data.series_overview_modal = None;

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        SeriesOverviewUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }

    #[test]
    fn test_series_overview_ui_renders_with_zero_series() {
      let mut app = App::test_default_fully_populated();
      app.data.sonarr_data.series = StatefulTable::default();
      app.push_navigation_stack(ActiveSonarrBlock::SeriesOverview.into());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        SeriesOverviewUi::draw(f, app, f.area());
      });

      assert_contains!(output, "Blah blah blah");
    }
  }
}
