#[cfg(test)]
mod tests {
  use bimap::BiMap;
  use pretty_assertions::assert_eq;
  use ratatui::style::Style;
  use rstest::rstest;
  use strum::IntoEnumIterator;

  use crate::app::App;
  use crate::models::radarr_models::{Movie, RadarrRelease};
  use crate::models::servarr_data::radarr::modals::MovieDetailsModal;
  use crate::models::servarr_data::radarr::radarr_data::{ActiveRadarrBlock, MOVIE_DETAILS_BLOCKS};
  use crate::models::stateful_table::StatefulTable;
  use crate::ui::DrawUi;
  use crate::ui::radarr_ui::library::movie_details_ui::{
    MovieDetailsUi, style_from_download_status,
  };
  use crate::ui::styles::ManagarrStyle;
  use crate::ui::ui_test_utils::test_utils::render_to_string_with_app;

  #[test]
  fn test_movie_details_ui_accepts() {
    ActiveRadarrBlock::iter().for_each(|active_radarr_block| {
      if MOVIE_DETAILS_BLOCKS.contains(&active_radarr_block) {
        assert!(MovieDetailsUi::accepts(active_radarr_block.into()));
      } else {
        assert!(!MovieDetailsUi::accepts(active_radarr_block.into()));
      }
    });
  }

  #[rstest]
  #[case("Downloading", true, "", Style::new().downloading())]
  #[case("Downloaded", true, "", Style::new().downloaded())]
  #[case("Awaiting Import", true, "", Style::new().awaiting_import())]
  #[case("Missing", false, "", Style::new().unmonitored_missing())]
  #[case("Missing", false, "", Style::new().unmonitored_missing())]
  #[case("Missing", true, "released", Style::new().missing())]
  #[case("", true, "", Style::new().downloaded())]
  fn test_style_from_download_status(
    #[case] download_status: &str,
    #[case] is_monitored: bool,
    #[case] movie_status: &str,
    #[case] expected_style: Style,
  ) {
    assert_eq!(
      style_from_download_status(download_status, is_monitored, movie_status.to_owned()),
      expected_style
    );
  }

  #[test]
  fn test_movie_details_ui_renders_loading_state() {
    let mut app = App::test_default();
    app.is_loading = true;
    app.push_navigation_stack(ActiveRadarrBlock::MovieDetails.into());
    app.data.radarr_data.movies = StatefulTable::default();

    let output = render_to_string_with_app(120, 30, &mut app, |f, app| {
      MovieDetailsUi::draw(f, app, f.area());
    });

    insta::assert_snapshot!(output);
  }

  #[test]
  fn test_movie_details_ui_renders_movie_details_tab() {
    let mut app = App::test_default();
    app.push_navigation_stack(ActiveRadarrBlock::MovieDetails.into());
    app.data.radarr_data.quality_profile_map =
      BiMap::from_iter(vec![(2222, "HD - 1080p".to_owned())]);
    app.data.radarr_data.movies = StatefulTable::default();
    app.data.radarr_data.movies.set_items(vec![Movie {
      id: 1,
      title: "Test Movie".into(),
      ..Movie::default()
    }]);

    let output = render_to_string_with_app(120, 30, &mut app, |f, app| {
      MovieDetailsUi::draw(f, app, f.area());
    });

    insta::assert_snapshot!(output);
  }

  #[test]
  fn test_movie_details_ui_renders_movie_history_tab() {
    let mut app = App::test_default();
    app.push_navigation_stack(ActiveRadarrBlock::MovieDetails.into());
    app.data.radarr_data.quality_profile_map =
      BiMap::from_iter(vec![(2222, "HD - 1080p".to_owned())]);
    app.data.radarr_data.movies = StatefulTable::default();
    app.data.radarr_data.movies.set_items(vec![Movie {
      id: 1,
      title: "Test Movie".into(),
      ..Movie::default()
    }]);
    app.data.radarr_data.movie_info_tabs.set_index(1);

    let output = render_to_string_with_app(120, 30, &mut app, |f, app| {
      MovieDetailsUi::draw(f, app, f.area());
    });

    insta::assert_snapshot!(output);
  }

  #[test]
  fn test_movie_details_ui_renders_manual_search_tab() {
    let mut app = App::test_default();
    app.push_navigation_stack(ActiveRadarrBlock::MovieDetails.into());
    app.data.radarr_data.quality_profile_map =
      BiMap::from_iter(vec![(2222, "HD - 1080p".to_owned())]);
    app.data.radarr_data.movies = StatefulTable::default();
    app.data.radarr_data.movies.set_items(vec![Movie {
      id: 1,
      title: "Test Movie".into(),
      ..Movie::default()
    }]);
    app.data.radarr_data.movie_details_modal = Some(MovieDetailsModal::default());
    app
      .data
      .radarr_data
      .movie_details_modal
      .as_mut()
      .unwrap()
      .movie_releases = StatefulTable::default();
    app
      .data
      .radarr_data
      .movie_details_modal
      .as_mut()
      .unwrap()
      .movie_releases
      .set_items(vec![RadarrRelease {
        title: "Test Release".into(),
        ..RadarrRelease::default()
      }]);
    app.data.radarr_data.movie_info_tabs.set_index(2);

    let output = render_to_string_with_app(120, 30, &mut app, |f, app| {
      MovieDetailsUi::draw(f, app, f.area());
    });

    insta::assert_snapshot!(output);
  }
}
