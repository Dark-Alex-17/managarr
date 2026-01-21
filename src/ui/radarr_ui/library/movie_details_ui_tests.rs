#[cfg(test)]
mod tests {
  use pretty_assertions::assert_eq;
  use ratatui::style::Style;
  use rstest::rstest;
  use strum::IntoEnumIterator;

  use crate::app::App;
  use crate::models::servarr_data::radarr::radarr_data::{ActiveRadarrBlock, MOVIE_DETAILS_BLOCKS};
  use crate::ui::DrawUi;
  use crate::ui::radarr_ui::library::movie_details_ui::{
    MovieDetailsUi, style_from_download_status,
  };
  use crate::ui::styles::{
    awaiting_import_style, downloaded_style, downloading_style, missing_style,
    unmonitored_missing_style,
  };
  use crate::ui::ui_test_utils::test_utils::{TerminalSize, render_to_string_with_app};

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
  #[case("Downloading", true, "", downloading_style())]
  #[case("Downloaded", true, "", downloaded_style())]
  #[case("Awaiting Import", true, "", awaiting_import_style())]
  #[case("Missing", false, "", unmonitored_missing_style())]
  #[case("Missing", false, "", unmonitored_missing_style())]
  #[case("Missing", true, "released", missing_style())]
  #[case("", true, "", downloaded_style())]
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

  mod snapshot_tests {
    use super::*;

    #[rstest]
    #[case(ActiveRadarrBlock::MovieDetails, None, 0)]
    #[case(
      ActiveRadarrBlock::MovieDetails,
      Some(ActiveRadarrBlock::AutomaticallySearchMoviePrompt),
      0
    )]
    #[case(
      ActiveRadarrBlock::MovieDetails,
      Some(ActiveRadarrBlock::UpdateAndScanPrompt),
      0
    )]
    #[case(ActiveRadarrBlock::MovieHistory, None, 1)]
    #[case(
      ActiveRadarrBlock::MovieHistory,
      Some(ActiveRadarrBlock::AutomaticallySearchMoviePrompt),
      1
    )]
    #[case(
      ActiveRadarrBlock::MovieHistory,
      Some(ActiveRadarrBlock::UpdateAndScanPrompt),
      1
    )]
    #[case(ActiveRadarrBlock::FileInfo, None, 2)]
    #[case(
      ActiveRadarrBlock::FileInfo,
      Some(ActiveRadarrBlock::AutomaticallySearchMoviePrompt),
      2
    )]
    #[case(
      ActiveRadarrBlock::FileInfo,
      Some(ActiveRadarrBlock::UpdateAndScanPrompt),
      2
    )]
    #[case(ActiveRadarrBlock::Cast, None, 3)]
    #[case(
      ActiveRadarrBlock::Cast,
      Some(ActiveRadarrBlock::AutomaticallySearchMoviePrompt),
      3
    )]
    #[case(
      ActiveRadarrBlock::Cast,
      Some(ActiveRadarrBlock::UpdateAndScanPrompt),
      3
    )]
    #[case(ActiveRadarrBlock::Crew, None, 4)]
    #[case(
      ActiveRadarrBlock::Crew,
      Some(ActiveRadarrBlock::AutomaticallySearchMoviePrompt),
      4
    )]
    #[case(
      ActiveRadarrBlock::Crew,
      Some(ActiveRadarrBlock::UpdateAndScanPrompt),
      4
    )]
    #[case(ActiveRadarrBlock::ManualSearch, None, 5)]
    #[case(
      ActiveRadarrBlock::ManualSearch,
      Some(ActiveRadarrBlock::AutomaticallySearchMoviePrompt),
      5
    )]
    #[case(
      ActiveRadarrBlock::ManualSearch,
      Some(ActiveRadarrBlock::UpdateAndScanPrompt),
      5
    )]
    #[case(ActiveRadarrBlock::ManualSearchSortPrompt, None, 5)]
    #[case(ActiveRadarrBlock::ManualSearchConfirmPrompt, None, 5)]
    fn test_movie_details_ui_renders_movie_details_tab(
      #[case] active_radarr_block: ActiveRadarrBlock,
      #[case] context: Option<ActiveRadarrBlock>,
      #[case] index: usize,
    ) {
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack((active_radarr_block, context).into());
      app.data.radarr_data.movie_info_tabs.set_index(index);

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        MovieDetailsUi::draw(f, app, f.area());
      });

      if let Some(context) = context {
        insta::assert_snapshot!(
          format!("movie_details_render_{active_radarr_block}_{context}"),
          output
        );
      } else {
        insta::assert_snapshot!(
          format!("movie_details_render_{active_radarr_block}"),
          output
        );
      }
    }

    #[rstest]
    fn test_movie_details_ui_renders_movie_details_tabs_loading(
      #[values(
        ActiveRadarrBlock::MovieDetails,
        ActiveRadarrBlock::MovieHistory,
        ActiveRadarrBlock::FileInfo,
        ActiveRadarrBlock::Cast,
        ActiveRadarrBlock::Crew,
        ActiveRadarrBlock::ManualSearch
      )]
      active_radarr_block: ActiveRadarrBlock,
    ) {
      let mut app = App::test_default();
      app.is_loading = true;
      app.push_navigation_stack(active_radarr_block.into());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        MovieDetailsUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(
        format!("movie_details_loading_{active_radarr_block}"),
        output
      );
    }
  }
}
