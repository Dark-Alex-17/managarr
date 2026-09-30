#[cfg(test)]
mod tests {
  use strum::IntoEnumIterator;

  use crate::app::App;
  use crate::app::key_binding::DEFAULT_KEYBINDINGS;
  use crate::handlers::KeyEventHandler;
  use crate::handlers::readarr_handlers::library::edition_details_handler::EditionDetailsHandler;
  use crate::models::ScrollableText;
  use crate::models::servarr_data::readarr::modals::{BookDetailsModal, EditionDetailsModal};
  use crate::models::servarr_data::readarr::readarr_data::{
    ActiveReadarrBlock, EDITION_DETAILS_BLOCKS,
  };

  mod test_handle_scroll_up_and_down {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_edition_details_scroll_down() {
      let mut app = app_with_edition_details();

      EditionDetailsHandler::new(
        DEFAULT_KEYBINDINGS.down.key,
        &mut app,
        ActiveReadarrBlock::BookEditionDetails,
        None,
      )
      .handle();

      assert_eq!(edition_details_offset(&app), 1);
    }

    #[test]
    fn test_edition_details_scroll_up() {
      let mut app = app_with_edition_details();
      set_offset(&mut app, 2);

      EditionDetailsHandler::new(
        DEFAULT_KEYBINDINGS.up.key,
        &mut app,
        ActiveReadarrBlock::BookEditionDetails,
        None,
      )
      .handle();

      assert_eq!(edition_details_offset(&app), 1);
    }

    #[test]
    fn test_edition_details_scroll_down_no_op_when_not_ready() {
      let mut app = app_with_edition_details();
      app.is_loading = true;

      EditionDetailsHandler::new(
        DEFAULT_KEYBINDINGS.down.key,
        &mut app,
        ActiveReadarrBlock::BookEditionDetails,
        None,
      )
      .handle();

      assert_eq!(edition_details_offset(&app), 0);
    }

    #[test]
    fn test_edition_details_scroll_up_no_op_when_not_ready() {
      let mut app = app_with_edition_details();
      set_offset(&mut app, 2);
      app.is_loading = true;

      EditionDetailsHandler::new(
        DEFAULT_KEYBINDINGS.up.key,
        &mut app,
        ActiveReadarrBlock::BookEditionDetails,
        None,
      )
      .handle();

      assert_eq!(edition_details_offset(&app), 2);
    }

    #[test]
    fn test_edition_details_scrolls_past_the_first_screen() {
      let mut app = app_with_edition_details();

      for _ in 0..4 {
        EditionDetailsHandler::new(
          DEFAULT_KEYBINDINGS.down.key,
          &mut app,
          ActiveReadarrBlock::BookEditionDetails,
          None,
        )
        .handle();
      }

      assert_eq!(edition_details_offset(&app), 4);
    }
  }

  mod test_handle_home_end {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_edition_details_end() {
      let mut app = app_with_edition_details();

      EditionDetailsHandler::new(
        DEFAULT_KEYBINDINGS.end.key,
        &mut app,
        ActiveReadarrBlock::BookEditionDetails,
        None,
      )
      .handle();

      assert_eq!(edition_details_offset(&app), 5);
    }

    #[test]
    fn test_edition_details_home() {
      let mut app = app_with_edition_details();
      set_offset(&mut app, 4);

      EditionDetailsHandler::new(
        DEFAULT_KEYBINDINGS.home.key,
        &mut app,
        ActiveReadarrBlock::BookEditionDetails,
        None,
      )
      .handle();

      assert_eq!(edition_details_offset(&app), 0);
    }

    #[test]
    fn test_edition_details_end_no_op_when_not_ready() {
      let mut app = app_with_edition_details();
      app.is_loading = true;

      EditionDetailsHandler::new(
        DEFAULT_KEYBINDINGS.end.key,
        &mut app,
        ActiveReadarrBlock::BookEditionDetails,
        None,
      )
      .handle();

      assert_eq!(edition_details_offset(&app), 0);
    }

    #[test]
    fn test_edition_details_home_no_op_when_not_ready() {
      let mut app = app_with_edition_details();
      set_offset(&mut app, 4);
      app.is_loading = true;

      EditionDetailsHandler::new(
        DEFAULT_KEYBINDINGS.home.key,
        &mut app,
        ActiveReadarrBlock::BookEditionDetails,
        None,
      )
      .handle();

      assert_eq!(edition_details_offset(&app), 4);
    }
  }

  mod test_handle_esc {
    use super::*;
    use crate::{assert_modal_absent, assert_modal_present, assert_navigation_popped};

    #[test]
    fn test_edition_details_esc_tears_down_the_edition_details_modal() {
      let mut app = app_with_edition_details();

      EditionDetailsHandler::new(
        DEFAULT_KEYBINDINGS.esc.key,
        &mut app,
        ActiveReadarrBlock::BookEditionDetails,
        None,
      )
      .handle();

      assert_navigation_popped!(app, ActiveReadarrBlock::BookDetails.into());
      assert_modal_absent!(
        app
          .data
          .readarr_data
          .book_details_modal
          .as_ref()
          .unwrap()
          .edition_details_modal
      );
      assert_modal_present!(app.data.readarr_data.book_details_modal);
    }

    #[test]
    fn test_edition_details_esc_tears_down_the_modal_even_when_loading() {
      let mut app = app_with_edition_details();
      app.is_loading = true;

      EditionDetailsHandler::new(
        DEFAULT_KEYBINDINGS.esc.key,
        &mut app,
        ActiveReadarrBlock::BookEditionDetails,
        None,
      )
      .handle();

      assert_navigation_popped!(app, ActiveReadarrBlock::BookDetails.into());
      assert_modal_absent!(
        app
          .data
          .readarr_data
          .book_details_modal
          .as_ref()
          .unwrap()
          .edition_details_modal
      );
    }
  }

  mod test_handle_key_char {
    use super::*;
    use crate::assert_navigation_pushed;

    #[test]
    fn test_refresh_key() {
      let mut app = app_with_edition_details();

      EditionDetailsHandler::new(
        DEFAULT_KEYBINDINGS.refresh.key,
        &mut app,
        ActiveReadarrBlock::BookEditionDetails,
        None,
      )
      .handle();

      assert_navigation_pushed!(app, ActiveReadarrBlock::BookEditionDetails.into());
    }

    #[test]
    fn test_refresh_key_no_op_when_not_ready() {
      let mut app = app_with_edition_details();
      app.is_loading = true;
      app.push_navigation_stack(ActiveReadarrBlock::BookHistory.into());

      EditionDetailsHandler::new(
        DEFAULT_KEYBINDINGS.refresh.key,
        &mut app,
        ActiveReadarrBlock::BookEditionDetails,
        None,
      )
      .handle();

      assert_navigation_pushed!(app, ActiveReadarrBlock::BookHistory.into());
    }
  }

  #[test]
  fn test_edition_details_handler_accepts() {
    ActiveReadarrBlock::iter().for_each(|active_readarr_block| {
      if EDITION_DETAILS_BLOCKS.contains(&active_readarr_block) {
        assert!(
          EditionDetailsHandler::accepts(active_readarr_block),
          "{active_readarr_block} is not accepted by the EditionDetailsHandler"
        );
      } else {
        assert!(
          !EditionDetailsHandler::accepts(active_readarr_block),
          "{active_readarr_block} is wrongly accepted by the EditionDetailsHandler"
        );
      }
    });
  }

  #[test]
  fn test_edition_details_handler_dispatches_no_network_events() {
    let mut app = app_with_edition_details();

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
      EditionDetailsHandler::new(key, &mut app, ActiveReadarrBlock::BookEditionDetails, None)
        .handle();

      assert_none!(app.data.readarr_data.prompt_confirm_action);
      assert!(!app.data.readarr_data.prompt_confirm);
    }
  }

  #[test]
  fn test_edition_details_handler_is_not_ready_when_loading() {
    let mut app = app_with_edition_details();
    app.is_loading = true;

    let handler = EditionDetailsHandler::new(
      DEFAULT_KEYBINDINGS.up.key,
      &mut app,
      ActiveReadarrBlock::BookEditionDetails,
      None,
    );

    assert!(!handler.is_ready());
  }

  #[test]
  fn test_edition_details_handler_is_not_ready_when_the_book_details_modal_is_absent() {
    let mut app = App::test_default();

    let handler = EditionDetailsHandler::new(
      DEFAULT_KEYBINDINGS.up.key,
      &mut app,
      ActiveReadarrBlock::BookEditionDetails,
      None,
    );

    assert!(!handler.is_ready());
  }

  #[test]
  fn test_edition_details_handler_is_not_ready_when_the_edition_details_modal_is_absent() {
    let mut app = App::test_default();
    app.data.readarr_data.book_details_modal = Some(BookDetailsModal::default());

    let handler = EditionDetailsHandler::new(
      DEFAULT_KEYBINDINGS.up.key,
      &mut app,
      ActiveReadarrBlock::BookEditionDetails,
      None,
    );

    assert!(!handler.is_ready());
  }

  #[test]
  fn test_edition_details_handler_is_ready() {
    let mut app = app_with_edition_details();

    let handler = EditionDetailsHandler::new(
      DEFAULT_KEYBINDINGS.up.key,
      &mut app,
      ActiveReadarrBlock::BookEditionDetails,
      None,
    );

    assert!(handler.is_ready());
  }

  #[test]
  fn test_edition_details_handler_ignores_special_keys_for_textbox_input() {
    let mut app = app_with_edition_details();
    app.ignore_special_keys_for_textbox_input = true;

    let handler = EditionDetailsHandler::new(
      DEFAULT_KEYBINDINGS.up.key,
      &mut app,
      ActiveReadarrBlock::BookEditionDetails,
      None,
    );

    assert!(handler.ignore_special_keys());
  }

  fn app_with_edition_details() -> App<'static> {
    let mut app = App::test_default();
    let book_details_modal = BookDetailsModal {
      edition_details_modal: Some(EditionDetailsModal {
        edition_details: ScrollableText::with_string(
          "Title: Test Edition\nFormat: Paperback\nLanguage: eng\nPublisher: Test\nPage Count: 662\nOverview: A long overview"
            .to_owned(),
        ),
      }),
      ..BookDetailsModal::default()
    };
    app.data.readarr_data.book_details_modal = Some(book_details_modal);
    app.push_navigation_stack(ActiveReadarrBlock::BookDetails.into());
    app.push_navigation_stack(ActiveReadarrBlock::BookEditionDetails.into());

    app
  }

  fn edition_details_offset(app: &App<'_>) -> u16 {
    app
      .data
      .readarr_data
      .book_details_modal
      .as_ref()
      .unwrap()
      .edition_details_modal
      .as_ref()
      .unwrap()
      .edition_details
      .offset
  }

  fn set_offset(app: &mut App<'_>, offset: u16) {
    app
      .data
      .readarr_data
      .book_details_modal
      .as_mut()
      .unwrap()
      .edition_details_modal
      .as_mut()
      .unwrap()
      .edition_details
      .offset = offset;
  }
}
