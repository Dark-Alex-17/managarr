#[cfg(test)]
mod tests {
  use strum::IntoEnumIterator;

  use crate::app::App;
  use crate::app::key_binding::DEFAULT_KEYBINDINGS;
  use crate::handlers::KeyEventHandler;
  use crate::handlers::radarr_handlers::collections::movie_overview_handler::MovieOverviewHandler;
  use crate::models::servarr_data::radarr::radarr_data::ActiveRadarrBlock;

  mod test_handle_scroll_up_and_down {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_movie_overview_scroll_up() {
      let mut app = app_with_movie_overview();
      set_offset(&mut app, 2);

      MovieOverviewHandler::new(
        DEFAULT_KEYBINDINGS.up.key,
        &mut app,
        ActiveRadarrBlock::ViewMovieOverview,
        None,
      )
      .handle();

      assert_eq!(overview_offset(&app), 1);
    }

    #[test]
    fn test_movie_overview_scroll_up_no_op_when_not_ready() {
      let mut app = app_with_movie_overview();
      set_offset(&mut app, 2);
      app.is_loading = true;

      MovieOverviewHandler::new(
        DEFAULT_KEYBINDINGS.up.key,
        &mut app,
        ActiveRadarrBlock::ViewMovieOverview,
        None,
      )
      .handle();

      assert_eq!(overview_offset(&app), 2);
    }

    #[test]
    fn test_movie_overview_scroll_down() {
      let mut app = app_with_movie_overview();

      MovieOverviewHandler::new(
        DEFAULT_KEYBINDINGS.down.key,
        &mut app,
        ActiveRadarrBlock::ViewMovieOverview,
        None,
      )
      .handle();

      assert_eq!(overview_offset(&app), 1);
    }

    #[test]
    fn test_movie_overview_scroll_down_no_op_when_not_ready() {
      let mut app = app_with_movie_overview();
      app.is_loading = true;

      MovieOverviewHandler::new(
        DEFAULT_KEYBINDINGS.down.key,
        &mut app,
        ActiveRadarrBlock::ViewMovieOverview,
        None,
      )
      .handle();

      assert_eq!(overview_offset(&app), 0);
    }
  }

  mod test_handle_home_end {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_movie_overview_home() {
      let mut app = app_with_movie_overview();
      set_offset(&mut app, 3);

      MovieOverviewHandler::new(
        DEFAULT_KEYBINDINGS.home.key,
        &mut app,
        ActiveRadarrBlock::ViewMovieOverview,
        None,
      )
      .handle();

      assert_eq!(overview_offset(&app), 0);
    }

    #[test]
    fn test_movie_overview_end() {
      let mut app = app_with_movie_overview();

      MovieOverviewHandler::new(
        DEFAULT_KEYBINDINGS.end.key,
        &mut app,
        ActiveRadarrBlock::ViewMovieOverview,
        None,
      )
      .handle();

      assert_eq!(overview_offset(&app), 5);
    }

    #[test]
    fn test_movie_overview_home_no_op_when_not_ready() {
      let mut app = app_with_movie_overview();
      set_offset(&mut app, 3);
      app.is_loading = true;

      MovieOverviewHandler::new(
        DEFAULT_KEYBINDINGS.home.key,
        &mut app,
        ActiveRadarrBlock::ViewMovieOverview,
        None,
      )
      .handle();

      assert_eq!(overview_offset(&app), 3);
    }

    #[test]
    fn test_movie_overview_end_no_op_when_not_ready() {
      let mut app = app_with_movie_overview();
      app.is_loading = true;

      MovieOverviewHandler::new(
        DEFAULT_KEYBINDINGS.end.key,
        &mut app,
        ActiveRadarrBlock::ViewMovieOverview,
        None,
      )
      .handle();

      assert_eq!(overview_offset(&app), 0);
    }
  }

  mod test_handle_esc {
    use super::*;
    use crate::{assert_modal_absent, assert_navigation_popped};

    #[test]
    fn test_movie_overview_esc_tears_down_the_movie_overview_modal() {
      let mut app = app_with_movie_overview();

      MovieOverviewHandler::new(
        DEFAULT_KEYBINDINGS.esc.key,
        &mut app,
        ActiveRadarrBlock::ViewMovieOverview,
        None,
      )
      .handle();

      assert_navigation_popped!(app, ActiveRadarrBlock::CollectionDetails.into());
      assert_modal_absent!(app.data.radarr_data.movie_overview_modal);
    }

    #[test]
    fn test_movie_overview_esc_tears_down_the_modal_even_when_loading() {
      let mut app = app_with_movie_overview();
      app.is_loading = true;

      MovieOverviewHandler::new(
        DEFAULT_KEYBINDINGS.esc.key,
        &mut app,
        ActiveRadarrBlock::ViewMovieOverview,
        None,
      )
      .handle();

      assert_navigation_popped!(app, ActiveRadarrBlock::CollectionDetails.into());
      assert_modal_absent!(app.data.radarr_data.movie_overview_modal);
    }
  }

  #[test]
  fn test_scrolling_the_overview_does_not_mutate_the_collection_movie_record() {
    let mut app = app_with_movie_overview();
    let overview_before = app
      .data
      .radarr_data
      .collection_movies
      .current_selection()
      .overview
      .clone();

    MovieOverviewHandler::new(
      DEFAULT_KEYBINDINGS.down.key,
      &mut app,
      ActiveRadarrBlock::ViewMovieOverview,
      None,
    )
    .handle();

    assert_eq!(overview_offset(&app), 1);
    assert_eq!(
      app
        .data
        .radarr_data
        .collection_movies
        .current_selection()
        .overview,
      overview_before
    );
  }

  #[test]
  fn test_movie_overview_handler_accepts() {
    ActiveRadarrBlock::iter().for_each(|active_radarr_block| {
      if active_radarr_block == ActiveRadarrBlock::ViewMovieOverview {
        assert!(
          MovieOverviewHandler::accepts(active_radarr_block),
          "{active_radarr_block} is not accepted by the MovieOverviewHandler"
        );
      } else {
        assert!(
          !MovieOverviewHandler::accepts(active_radarr_block),
          "{active_radarr_block} is wrongly accepted by the MovieOverviewHandler"
        );
      }
    });
  }

  #[test]
  fn test_movie_overview_handler_dispatches_no_network_events() {
    let mut app = app_with_movie_overview();

    for key in [
      DEFAULT_KEYBINDINGS.up.key,
      DEFAULT_KEYBINDINGS.down.key,
      DEFAULT_KEYBINDINGS.home.key,
      DEFAULT_KEYBINDINGS.end.key,
      DEFAULT_KEYBINDINGS.delete.key,
      DEFAULT_KEYBINDINGS.left.key,
      DEFAULT_KEYBINDINGS.right.key,
      DEFAULT_KEYBINDINGS.submit.key,
      DEFAULT_KEYBINDINGS.refresh.key,
    ] {
      MovieOverviewHandler::new(key, &mut app, ActiveRadarrBlock::ViewMovieOverview, None).handle();

      assert_none!(app.data.radarr_data.prompt_confirm_action);
      assert!(!app.data.radarr_data.prompt_confirm);
    }
  }

  #[test]
  fn test_movie_overview_handler_is_not_ready_when_loading() {
    let mut app = app_with_movie_overview();
    app.is_loading = true;

    let handler = MovieOverviewHandler::new(
      DEFAULT_KEYBINDINGS.up.key,
      &mut app,
      ActiveRadarrBlock::ViewMovieOverview,
      None,
    );

    assert!(!handler.is_ready());
  }

  #[test]
  fn test_movie_overview_handler_is_not_ready_when_the_modal_is_absent() {
    let mut app = app_with_movie_overview();
    app.data.radarr_data.movie_overview_modal = None;

    let handler = MovieOverviewHandler::new(
      DEFAULT_KEYBINDINGS.up.key,
      &mut app,
      ActiveRadarrBlock::ViewMovieOverview,
      None,
    );

    assert!(!handler.is_ready());
  }

  #[test]
  fn test_movie_overview_handler_is_ready() {
    let mut app = app_with_movie_overview();

    let handler = MovieOverviewHandler::new(
      DEFAULT_KEYBINDINGS.up.key,
      &mut app,
      ActiveRadarrBlock::ViewMovieOverview,
      None,
    );

    assert!(handler.is_ready());
  }

  #[test]
  fn test_movie_overview_handler_ignores_special_keys_for_textbox_input() {
    let mut app = app_with_movie_overview();
    app.ignore_special_keys_for_textbox_input = true;

    let handler = MovieOverviewHandler::new(
      DEFAULT_KEYBINDINGS.up.key,
      &mut app,
      ActiveRadarrBlock::ViewMovieOverview,
      None,
    );

    assert!(handler.ignore_special_keys());
  }

  fn app_with_movie_overview() -> App<'static> {
    let mut app = App::test_default_fully_populated();
    app.push_navigation_stack(ActiveRadarrBlock::CollectionDetails.into());
    app.push_navigation_stack(ActiveRadarrBlock::ViewMovieOverview.into());

    app
  }

  fn overview_offset(app: &App<'_>) -> u16 {
    app
      .data
      .radarr_data
      .movie_overview_modal
      .as_ref()
      .unwrap()
      .overview
      .offset
  }

  fn set_offset(app: &mut App<'_>, offset: u16) {
    app
      .data
      .radarr_data
      .movie_overview_modal
      .as_mut()
      .unwrap()
      .overview
      .offset = offset;
  }
}
