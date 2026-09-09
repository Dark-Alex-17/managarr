#[cfg(test)]
mod tests {
  use strum::IntoEnumIterator;

  use crate::app::App;
  use crate::app::key_binding::DEFAULT_KEYBINDINGS;
  use crate::handlers::KeyEventHandler;
  use crate::handlers::lidarr_handlers::library::artist_overview_handler::ArtistOverviewHandler;
  use crate::models::ScrollableText;
  use crate::models::servarr_data::lidarr::lidarr_data::{
    ARTIST_OVERVIEW_BLOCKS, ActiveLidarrBlock,
  };
  use crate::models::servarr_data::lidarr::modals::ArtistOverviewModal;

  mod test_handle_scroll_up_and_down {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_artist_overview_scroll_up() {
      let mut app = app_with_artist_overview();
      set_offset(&mut app, 2);

      ArtistOverviewHandler::new(
        DEFAULT_KEYBINDINGS.up.key,
        &mut app,
        ActiveLidarrBlock::ArtistOverview,
        None,
      )
      .handle();

      assert_eq!(overview_offset(&app), 1);
    }

    #[test]
    fn test_artist_overview_scroll_up_no_op_when_not_ready() {
      let mut app = app_with_artist_overview();
      set_offset(&mut app, 2);
      app.is_loading = true;

      ArtistOverviewHandler::new(
        DEFAULT_KEYBINDINGS.up.key,
        &mut app,
        ActiveLidarrBlock::ArtistOverview,
        None,
      )
      .handle();

      assert_eq!(overview_offset(&app), 2);
    }

    #[test]
    fn test_artist_overview_scroll_down() {
      let mut app = app_with_artist_overview();

      ArtistOverviewHandler::new(
        DEFAULT_KEYBINDINGS.down.key,
        &mut app,
        ActiveLidarrBlock::ArtistOverview,
        None,
      )
      .handle();

      assert_eq!(overview_offset(&app), 1);
    }

    #[test]
    fn test_artist_overview_scroll_down_no_op_when_not_ready() {
      let mut app = app_with_artist_overview();
      app.is_loading = true;

      ArtistOverviewHandler::new(
        DEFAULT_KEYBINDINGS.down.key,
        &mut app,
        ActiveLidarrBlock::ArtistOverview,
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
    fn test_artist_overview_home() {
      let mut app = app_with_artist_overview();
      set_offset(&mut app, 3);

      ArtistOverviewHandler::new(
        DEFAULT_KEYBINDINGS.home.key,
        &mut app,
        ActiveLidarrBlock::ArtistOverview,
        None,
      )
      .handle();

      assert_eq!(overview_offset(&app), 0);
    }

    #[test]
    fn test_artist_overview_end() {
      let mut app = app_with_artist_overview();

      ArtistOverviewHandler::new(
        DEFAULT_KEYBINDINGS.end.key,
        &mut app,
        ActiveLidarrBlock::ArtistOverview,
        None,
      )
      .handle();

      assert_eq!(overview_offset(&app), 5);
    }

    #[test]
    fn test_artist_overview_home_no_op_when_not_ready() {
      let mut app = app_with_artist_overview();
      set_offset(&mut app, 3);
      app.is_loading = true;

      ArtistOverviewHandler::new(
        DEFAULT_KEYBINDINGS.home.key,
        &mut app,
        ActiveLidarrBlock::ArtistOverview,
        None,
      )
      .handle();

      assert_eq!(overview_offset(&app), 3);
    }

    #[test]
    fn test_artist_overview_end_no_op_when_not_ready() {
      let mut app = app_with_artist_overview();
      app.is_loading = true;

      ArtistOverviewHandler::new(
        DEFAULT_KEYBINDINGS.end.key,
        &mut app,
        ActiveLidarrBlock::ArtistOverview,
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
    fn test_artist_overview_esc_tears_down_the_artist_overview_modal() {
      let mut app = app_with_artist_overview();

      ArtistOverviewHandler::new(
        DEFAULT_KEYBINDINGS.esc.key,
        &mut app,
        ActiveLidarrBlock::ArtistOverview,
        None,
      )
      .handle();

      assert_navigation_popped!(app, ActiveLidarrBlock::ArtistDetails.into());
      assert_modal_absent!(app.data.lidarr_data.artist_overview_modal);
    }

    #[test]
    fn test_artist_overview_esc_tears_down_the_modal_even_when_loading() {
      let mut app = app_with_artist_overview();
      app.is_loading = true;

      ArtistOverviewHandler::new(
        DEFAULT_KEYBINDINGS.esc.key,
        &mut app,
        ActiveLidarrBlock::ArtistOverview,
        None,
      )
      .handle();

      assert_navigation_popped!(app, ActiveLidarrBlock::ArtistDetails.into());
      assert_modal_absent!(app.data.lidarr_data.artist_overview_modal);
    }
  }

  mod test_handle_key_char {
    use super::*;
    use crate::assert_navigation_pushed;

    #[test]
    fn test_artist_overview_refresh_key() {
      let mut app = app_with_artist_overview();
      app.is_routing = false;

      ArtistOverviewHandler::new(
        DEFAULT_KEYBINDINGS.refresh.key,
        &mut app,
        ActiveLidarrBlock::ArtistOverview,
        None,
      )
      .handle();

      assert_navigation_pushed!(app, ActiveLidarrBlock::ArtistOverview.into());
      assert!(app.is_routing);
    }

    #[test]
    fn test_artist_overview_refresh_key_no_op_when_not_ready() {
      let mut app = app_with_artist_overview();
      app.is_loading = true;
      app.is_routing = false;

      ArtistOverviewHandler::new(
        DEFAULT_KEYBINDINGS.refresh.key,
        &mut app,
        ActiveLidarrBlock::ArtistOverview,
        None,
      )
      .handle();

      assert!(!app.is_routing);
    }
  }

  #[test]
  fn test_scrolling_the_overview_does_not_mutate_the_artist_record() {
    let mut app = app_with_artist_overview();
    let overview_before = app
      .data
      .lidarr_data
      .artists
      .current_selection()
      .overview
      .clone();

    ArtistOverviewHandler::new(
      DEFAULT_KEYBINDINGS.down.key,
      &mut app,
      ActiveLidarrBlock::ArtistOverview,
      None,
    )
    .handle();

    assert_eq!(overview_offset(&app), 1);
    assert_eq!(
      app.data.lidarr_data.artists.current_selection().overview,
      overview_before
    );
  }

  #[test]
  fn test_artist_overview_handler_accepts() {
    ActiveLidarrBlock::iter().for_each(|active_lidarr_block| {
      if ARTIST_OVERVIEW_BLOCKS.contains(&active_lidarr_block) {
        assert!(
          ArtistOverviewHandler::accepts(active_lidarr_block),
          "{active_lidarr_block} is not accepted by the ArtistOverviewHandler"
        );
      } else {
        assert!(
          !ArtistOverviewHandler::accepts(active_lidarr_block),
          "{active_lidarr_block} is wrongly accepted by the ArtistOverviewHandler"
        );
      }
    });
  }

  #[test]
  fn test_artist_overview_handler_dispatches_no_network_events() {
    let mut app = app_with_artist_overview();

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
      ArtistOverviewHandler::new(key, &mut app, ActiveLidarrBlock::ArtistOverview, None).handle();

      assert_none!(app.data.lidarr_data.prompt_confirm_action);
      assert!(!app.data.lidarr_data.prompt_confirm);
    }
  }

  #[test]
  fn test_artist_overview_handler_is_not_ready_when_loading() {
    let mut app = app_with_artist_overview();
    app.is_loading = true;

    let handler = ArtistOverviewHandler::new(
      DEFAULT_KEYBINDINGS.up.key,
      &mut app,
      ActiveLidarrBlock::ArtistOverview,
      None,
    );

    assert!(!handler.is_ready());
  }

  #[test]
  fn test_artist_overview_handler_is_not_ready_when_the_modal_is_absent() {
    let mut app = app_with_artist_overview();
    app.data.lidarr_data.artist_overview_modal = None;

    let handler = ArtistOverviewHandler::new(
      DEFAULT_KEYBINDINGS.up.key,
      &mut app,
      ActiveLidarrBlock::ArtistOverview,
      None,
    );

    assert!(!handler.is_ready());
  }

  #[test]
  fn test_artist_overview_handler_is_ready() {
    let mut app = app_with_artist_overview();

    let handler = ArtistOverviewHandler::new(
      DEFAULT_KEYBINDINGS.up.key,
      &mut app,
      ActiveLidarrBlock::ArtistOverview,
      None,
    );

    assert!(handler.is_ready());
  }

  #[test]
  fn test_artist_overview_handler_ignores_special_keys_for_textbox_input() {
    let mut app = app_with_artist_overview();
    app.ignore_special_keys_for_textbox_input = true;

    let handler = ArtistOverviewHandler::new(
      DEFAULT_KEYBINDINGS.up.key,
      &mut app,
      ActiveLidarrBlock::ArtistOverview,
      None,
    );

    assert!(handler.ignore_special_keys());
  }

  fn app_with_artist_overview() -> App<'static> {
    let mut app = App::test_default_fully_populated();
    app.data.lidarr_data.artist_overview_modal = Some(ArtistOverviewModal {
      overview: ScrollableText::with_string(
        "some interesting description of the artist\r\n\
         \r\n\
         She was born in Madison, Wisconsin: a city she has never really left.\r\n\
         \r\n\
         Her first album took her seven years to finish.\r\n"
          .to_owned(),
      ),
    });
    app.push_navigation_stack(ActiveLidarrBlock::ArtistDetails.into());
    app.push_navigation_stack(ActiveLidarrBlock::ArtistOverview.into());

    app
  }

  fn overview_offset(app: &App<'_>) -> u16 {
    app
      .data
      .lidarr_data
      .artist_overview_modal
      .as_ref()
      .unwrap()
      .overview
      .offset
  }

  fn set_offset(app: &mut App<'_>, offset: u16) {
    app
      .data
      .lidarr_data
      .artist_overview_modal
      .as_mut()
      .unwrap()
      .overview
      .offset = offset;
  }
}
