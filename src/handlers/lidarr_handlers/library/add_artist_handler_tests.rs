#[cfg(test)]
mod tests {
  use pretty_assertions::assert_str_eq;
  use rstest::rstest;
  use std::sync::atomic::Ordering;
  use strum::IntoEnumIterator;

  use crate::app::App;
  use crate::app::key_binding::DEFAULT_KEYBINDINGS;
  use crate::assert_navigation_popped;
  use crate::event::Key;
  use crate::handlers::KeyEventHandler;
  use crate::handlers::lidarr_handlers::library::add_artist_handler::AddArtistHandler;
  use crate::models::HorizontallyScrollableText;
  use crate::models::servarr_data::lidarr::lidarr_data::{ADD_ARTIST_BLOCKS, ActiveLidarrBlock};
  use crate::models::stateful_table::StatefulTable;
  use crate::network::lidarr_network::lidarr_network_test_utils::test_utils::add_artist_search_result;

  mod test_handle_home_end {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_add_artist_search_input_home_end_keys() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveLidarrBlock::AddArtistSearchInput.into());
      app.data.lidarr_data.add_artist_search = Some("Test".into());

      AddArtistHandler::new(
        DEFAULT_KEYBINDINGS.home.key,
        &mut app,
        ActiveLidarrBlock::AddArtistSearchInput,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .lidarr_data
          .add_artist_search
          .as_ref()
          .unwrap()
          .offset
          .load(Ordering::SeqCst),
        4
      );

      AddArtistHandler::new(
        DEFAULT_KEYBINDINGS.end.key,
        &mut app,
        ActiveLidarrBlock::AddArtistSearchInput,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .lidarr_data
          .add_artist_search
          .as_ref()
          .unwrap()
          .offset
          .load(Ordering::SeqCst),
        0
      );
    }
  }

  mod test_handle_left_right_action {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_add_artist_search_input_left_right_keys() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveLidarrBlock::AddArtistSearchInput.into());
      app.data.lidarr_data.add_artist_search = Some("Test".into());

      AddArtistHandler::new(
        DEFAULT_KEYBINDINGS.left.key,
        &mut app,
        ActiveLidarrBlock::AddArtistSearchInput,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .lidarr_data
          .add_artist_search
          .as_ref()
          .unwrap()
          .offset
          .load(Ordering::SeqCst),
        1
      );

      AddArtistHandler::new(
        DEFAULT_KEYBINDINGS.right.key,
        &mut app,
        ActiveLidarrBlock::AddArtistSearchInput,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .lidarr_data
          .add_artist_search
          .as_ref()
          .unwrap()
          .offset
          .load(Ordering::SeqCst),
        0
      );
    }
  }

  mod test_handle_submit {
    use super::*;
    use pretty_assertions::assert_eq;

    const SUBMIT_KEY: Key = DEFAULT_KEYBINDINGS.submit.key;

    #[test]
    fn test_add_artist_search_input_submit() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveLidarrBlock::AddArtistSearchInput.into());
      app.ignore_special_keys_for_textbox_input = true;
      app.data.lidarr_data.add_artist_search = Some("test".into());

      AddArtistHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveLidarrBlock::AddArtistSearchInput,
        None,
      )
      .handle();

      assert!(!app.ignore_special_keys_for_textbox_input);
      assert_eq!(
        app.get_current_route(),
        ActiveLidarrBlock::AddArtistSearchResults.into()
      );
    }

    #[test]
    fn test_add_artist_search_input_submit_noop_on_empty_search() {
      let mut app = App::test_default();
      app.data.lidarr_data.add_artist_search = Some(HorizontallyScrollableText::default());
      app.push_navigation_stack(ActiveLidarrBlock::Artists.into());
      app.push_navigation_stack(ActiveLidarrBlock::AddArtistSearchInput.into());
      app.ignore_special_keys_for_textbox_input = true;

      AddArtistHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveLidarrBlock::AddArtistSearchInput,
        None,
      )
      .handle();

      assert!(app.ignore_special_keys_for_textbox_input);
      assert_eq!(
        app.get_current_route(),
        ActiveLidarrBlock::AddArtistSearchInput.into()
      );
    }
  }

  mod test_handle_esc {
    use super::*;
    use crate::assert_modal_absent;
    use rstest::rstest;

    const ESC_KEY: Key = DEFAULT_KEYBINDINGS.esc.key;

    #[rstest]
    fn test_add_artist_search_input_esc(#[values(true, false)] is_ready: bool) {
      let mut app = App::test_default();
      app.is_loading = is_ready;
      app.data.lidarr_data.add_artist_search = Some("test".into());
      app.ignore_special_keys_for_textbox_input = true;
      app.push_navigation_stack(ActiveLidarrBlock::Artists.into());
      app.push_navigation_stack(ActiveLidarrBlock::AddArtistSearchInput.into());

      AddArtistHandler::new(
        ESC_KEY,
        &mut app,
        ActiveLidarrBlock::AddArtistSearchInput,
        None,
      )
      .handle();

      assert!(!app.ignore_special_keys_for_textbox_input);
      assert_navigation_popped!(app, ActiveLidarrBlock::Artists.into());
      assert_modal_absent!(app.data.lidarr_data.add_artist_search);
    }

    #[rstest]
    fn test_add_artist_search_results_esc(
      #[values(
        ActiveLidarrBlock::AddArtistSearchResults,
        ActiveLidarrBlock::AddArtistEmptySearchResults
      )]
      active_lidarr_block: ActiveLidarrBlock,
    ) {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveLidarrBlock::AddArtistSearchInput.into());
      app.push_navigation_stack(active_lidarr_block.into());
      let mut add_searched_artists = StatefulTable::default();
      add_searched_artists.set_items(vec![add_artist_search_result()]);
      app.data.lidarr_data.add_searched_artists = Some(add_searched_artists);

      AddArtistHandler::new(ESC_KEY, &mut app, active_lidarr_block, None).handle();

      assert_navigation_popped!(app, ActiveLidarrBlock::AddArtistSearchInput.into());
      assert_modal_absent!(app.data.lidarr_data.add_searched_artists);
      assert!(app.ignore_special_keys_for_textbox_input);
    }
  }

  mod test_handle_key_char {
    use super::*;

    #[test]
    fn test_add_artist_search_input_backspace() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveLidarrBlock::Artists.into());
      app.data.lidarr_data.add_artist_search = Some("Test".into());

      AddArtistHandler::new(
        DEFAULT_KEYBINDINGS.backspace.key,
        &mut app,
        ActiveLidarrBlock::AddArtistSearchInput,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .lidarr_data
          .add_artist_search
          .as_ref()
          .unwrap()
          .text,
        "Tes"
      );
    }

    #[test]
    fn test_add_artist_search_input_char_key() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveLidarrBlock::Artists.into());
      app.data.lidarr_data.add_artist_search = Some(HorizontallyScrollableText::default());

      AddArtistHandler::new(
        Key::Char('a'),
        &mut app,
        ActiveLidarrBlock::AddArtistSearchInput,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .lidarr_data
          .add_artist_search
          .as_ref()
          .unwrap()
          .text,
        "a"
      );
    }
  }

  #[test]
  fn test_add_artist_handler_accepts() {
    ActiveLidarrBlock::iter().for_each(|active_lidarr_block| {
      if ADD_ARTIST_BLOCKS.contains(&active_lidarr_block) {
        assert!(AddArtistHandler::accepts(active_lidarr_block));
      } else {
        assert!(!AddArtistHandler::accepts(active_lidarr_block));
      }
    });
  }

  #[rstest]
  fn test_add_artist_handler_ignore_special_keys(
    #[values(true, false)] ignore_special_keys_for_textbox_input: bool,
  ) {
    let mut app = App::test_default();
    app.ignore_special_keys_for_textbox_input = ignore_special_keys_for_textbox_input;
    let handler = AddArtistHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveLidarrBlock::default(),
      None,
    );

    assert_eq!(
      handler.ignore_special_keys(),
      ignore_special_keys_for_textbox_input
    );
  }

  #[test]
  fn test_add_artist_search_no_panic_on_none_search_result() {
    let mut app = App::test_default();
    app.data.lidarr_data.add_searched_artists = None;

    AddArtistHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveLidarrBlock::AddArtistSearchResults,
      None,
    )
    .handle();
  }

  #[test]
  fn test_add_artist_handler_is_not_ready_when_loading() {
    let mut app = App::test_default();
    app.is_loading = true;

    let handler = AddArtistHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveLidarrBlock::AddArtistSearchInput,
      None,
    );

    assert!(!handler.is_ready());
  }

  #[test]
  fn test_add_artist_handler_is_ready_when_not_loading() {
    let mut app = App::test_default();
    app.is_loading = false;

    let handler = AddArtistHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveLidarrBlock::AddArtistSearchInput,
      None,
    );

    assert!(handler.is_ready());
  }
}
