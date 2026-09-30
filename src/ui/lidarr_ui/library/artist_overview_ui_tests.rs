#[cfg(test)]
mod tests {
  use pretty_assertions::assert_eq;
  use ratatui::style::Modifier;
  use ratatui::text::Line;
  use strum::IntoEnumIterator;

  use crate::app::App;
  use crate::models::lidarr_models::Artist;
  use crate::models::servarr_data::lidarr::lidarr_data::{
    ARTIST_OVERVIEW_BLOCKS, ActiveLidarrBlock,
  };
  use crate::ui::DrawUi;
  use crate::ui::lidarr_ui::library::artist_overview_ui::{
    ArtistOverviewUi, overview_line, style_from_artist,
  };
  use crate::ui::styles::{primary_style, unmonitored_style};
  use crate::ui::ui_test_utils::test_utils::render_to_string_with_app;

  #[test]
  fn test_artist_overview_ui_accepts() {
    ActiveLidarrBlock::iter().for_each(|active_lidarr_block| {
      if ARTIST_OVERVIEW_BLOCKS.contains(&active_lidarr_block) {
        assert!(ArtistOverviewUi::accepts(active_lidarr_block.into()));
      } else {
        assert!(!ArtistOverviewUi::accepts(active_lidarr_block.into()));
      }
    });
  }

  #[test]
  fn test_style_from_artist_monitored() {
    let artist = Artist {
      monitored: true,
      ..Artist::default()
    };

    let style = style_from_artist(&artist);

    assert_eq!(style, primary_style());
  }

  #[test]
  fn test_style_from_artist_unmonitored() {
    let artist = Artist {
      monitored: false,
      ..Artist::default()
    };

    let style = style_from_artist(&artist);

    assert_eq!(style, unmonitored_style());
  }

  #[test]
  fn test_overview_line_does_not_split_a_label_from_a_colon_in_the_prose() {
    let line = overview_line(
      "She was born in Madison, Wisconsin: a city she has never really left.",
      primary_style(),
    );

    assert_eq!(
      span_contents(&line),
      vec!["She was born in Madison, Wisconsin: a city she has never really left."]
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
    let line = overview_line("an unmonitored artist", unmonitored_style());

    assert_eq!(line.spans[0].style.fg, unmonitored_style().fg);
  }

  #[test]
  fn test_overview_line_trims_a_trailing_carriage_return() {
    let line = overview_line(
      "Her first album took her seven years to finish.\r",
      primary_style(),
    );

    assert_eq!(
      span_contents(&line),
      vec!["Her first album took her seven years to finish."]
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
    use crate::models::servarr_data::lidarr::modals::ArtistOverviewModal;
    use crate::models::stateful_table::StatefulTable;
    use crate::ui::ui_test_utils::test_utils::TerminalSize;

    use super::*;

    #[test]
    fn test_artist_overview_ui_renders() {
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack(ActiveLidarrBlock::ArtistOverview.into());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        ArtistOverviewUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }

    #[test]
    fn test_artist_overview_ui_renders_loading() {
      let mut app = App::test_default_fully_populated();
      app.is_loading = true;
      app.push_navigation_stack(ActiveLidarrBlock::ArtistOverview.into());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        ArtistOverviewUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }

    #[test]
    fn test_artist_overview_ui_renders_scrolled() {
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack(ActiveLidarrBlock::ArtistOverview.into());
      app
        .data
        .lidarr_data
        .artist_overview_modal
        .as_mut()
        .unwrap()
        .overview
        .offset = 2;

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        ArtistOverviewUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }

    #[test]
    fn test_artist_overview_ui_renders_an_artist_with_no_overview() {
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack(ActiveLidarrBlock::ArtistOverview.into());
      app.data.lidarr_data.artist_overview_modal = Some(ArtistOverviewModal {
        overview: ScrollableText::with_string(String::new()),
      });

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        ArtistOverviewUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }

    #[test]
    fn test_artist_overview_ui_renders_nothing_when_the_artist_overview_modal_is_undefined() {
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack(ActiveLidarrBlock::ArtistOverview.into());
      app.data.lidarr_data.artist_overview_modal = None;

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        ArtistOverviewUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }

    #[test]
    fn test_artist_overview_ui_renders_with_zero_artists() {
      let mut app = App::test_default_fully_populated();
      app.data.lidarr_data.artists = StatefulTable::default();
      app.push_navigation_stack(ActiveLidarrBlock::ArtistOverview.into());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        ArtistOverviewUi::draw(f, app, f.area());
      });

      assert_contains!(output, "some interesting description of the artist");
    }
  }
}
