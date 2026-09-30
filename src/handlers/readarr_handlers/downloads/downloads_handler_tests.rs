#[cfg(test)]
mod tests {
  use pretty_assertions::assert_eq;
  use rstest::rstest;
  use serde_json::Number;
  use strum::IntoEnumIterator;

  use crate::app::App;
  use crate::app::key_binding::DEFAULT_KEYBINDINGS;
  use crate::assert_navigation_pushed;
  use crate::event::Key;
  use crate::handlers::KeyEventHandler;
  use crate::handlers::readarr_handlers::downloads::DownloadsHandler;
  use crate::models::readarr_models::DownloadRecord;
  use crate::models::servarr_data::readarr::readarr_data::{ActiveReadarrBlock, DOWNLOADS_BLOCKS};
  use crate::models::servarr_models::DownloadStatus;

  mod test_handle_delete {
    use pretty_assertions::assert_eq;

    use super::*;

    const DELETE_KEY: Key = DEFAULT_KEYBINDINGS.delete.key;

    #[test]
    fn test_delete_download_prompt() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveReadarrBlock::Downloads.into());
      app.data.readarr_data.downloads.set_items(downloads_vec());

      DownloadsHandler::new(DELETE_KEY, &mut app, ActiveReadarrBlock::Downloads, None).handle();

      assert_navigation_pushed!(app, ActiveReadarrBlock::DeleteDownloadPrompt.into());
    }

    #[test]
    fn test_delete_download_prompt_no_op_when_not_ready() {
      let mut app = App::test_default();
      app.is_loading = true;
      app.push_navigation_stack(ActiveReadarrBlock::Downloads.into());
      app.data.readarr_data.downloads.set_items(downloads_vec());

      DownloadsHandler::new(DELETE_KEY, &mut app, ActiveReadarrBlock::Downloads, None).handle();

      assert_eq!(
        app.get_current_route(),
        ActiveReadarrBlock::Downloads.into()
      );
    }
  }

  mod test_handle_left_right_action {
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    use super::*;
    use crate::assert_navigation_pushed;

    #[rstest]
    fn test_downloads_tab_left(#[values(true, false)] is_ready: bool) {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveReadarrBlock::Downloads.into());
      app.is_loading = is_ready;
      app.data.readarr_data.main_tabs.set_index(1);

      DownloadsHandler::new(
        DEFAULT_KEYBINDINGS.left.key,
        &mut app,
        ActiveReadarrBlock::Downloads,
        None,
      )
      .handle();

      assert_eq!(
        app.data.readarr_data.main_tabs.get_active_route(),
        ActiveReadarrBlock::Authors.into()
      );
      assert_navigation_pushed!(app, ActiveReadarrBlock::Authors.into());
    }

    #[rstest]
    fn test_downloads_tab_right(#[values(true, false)] is_ready: bool) {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveReadarrBlock::Downloads.into());
      app.is_loading = is_ready;
      app.data.readarr_data.main_tabs.set_index(1);

      DownloadsHandler::new(
        DEFAULT_KEYBINDINGS.right.key,
        &mut app,
        ActiveReadarrBlock::Downloads,
        None,
      )
      .handle();

      assert_eq!(
        app.data.readarr_data.main_tabs.get_active_route(),
        ActiveReadarrBlock::Blocklist.into()
      );
      assert_navigation_pushed!(app, ActiveReadarrBlock::Blocklist.into());
    }

    #[rstest]
    fn test_downloads_left_right_prompt_toggle(
      #[values(
        ActiveReadarrBlock::DeleteDownloadPrompt,
        ActiveReadarrBlock::UpdateDownloadsPrompt
      )]
      active_readarr_block: ActiveReadarrBlock,
      #[values(DEFAULT_KEYBINDINGS.left.key, DEFAULT_KEYBINDINGS.right.key)] key: Key,
    ) {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveReadarrBlock::Downloads.into());

      DownloadsHandler::new(key, &mut app, active_readarr_block, None).handle();

      assert!(app.data.readarr_data.prompt_confirm);

      DownloadsHandler::new(key, &mut app, active_readarr_block, None).handle();

      assert!(!app.data.readarr_data.prompt_confirm);
    }
  }

  mod test_handle_submit {
    use rstest::rstest;

    use super::*;
    use crate::assert_navigation_popped;
    use crate::network::readarr_network::ReadarrEvent;

    const SUBMIT_KEY: Key = DEFAULT_KEYBINDINGS.submit.key;

    #[rstest]
    #[case(
      ActiveReadarrBlock::Downloads,
      ActiveReadarrBlock::DeleteDownloadPrompt,
      ReadarrEvent::DeleteDownload(3)
    )]
    #[case(
      ActiveReadarrBlock::Downloads,
      ActiveReadarrBlock::UpdateDownloadsPrompt,
      ReadarrEvent::UpdateDownloads
    )]
    fn test_downloads_prompt_confirm_submit(
      #[case] base_route: ActiveReadarrBlock,
      #[case] prompt_block: ActiveReadarrBlock,
      #[case] expected_action: ReadarrEvent,
    ) {
      let mut app = App::test_default();
      app.data.readarr_data.downloads.set_items(downloads_vec());
      app.data.readarr_data.prompt_confirm = true;
      app.push_navigation_stack(base_route.into());
      app.push_navigation_stack(prompt_block.into());

      DownloadsHandler::new(SUBMIT_KEY, &mut app, prompt_block, None).handle();

      assert!(app.data.readarr_data.prompt_confirm);
      assert_some_eq_x!(
        &app.data.readarr_data.prompt_confirm_action,
        &expected_action
      );
      assert_navigation_popped!(app, base_route.into());
    }

    #[rstest]
    #[case(
      ActiveReadarrBlock::Downloads,
      ActiveReadarrBlock::DeleteDownloadPrompt
    )]
    #[case(
      ActiveReadarrBlock::Downloads,
      ActiveReadarrBlock::UpdateDownloadsPrompt
    )]
    fn test_downloads_prompt_decline_submit(
      #[case] base_route: ActiveReadarrBlock,
      #[case] prompt_block: ActiveReadarrBlock,
    ) {
      let mut app = App::test_default();
      app.data.readarr_data.downloads.set_items(downloads_vec());
      app.push_navigation_stack(base_route.into());
      app.push_navigation_stack(prompt_block.into());

      DownloadsHandler::new(SUBMIT_KEY, &mut app, prompt_block, None).handle();

      assert!(!app.data.readarr_data.prompt_confirm);
      assert_none!(app.data.readarr_data.prompt_confirm_action);
      assert_navigation_popped!(app, base_route.into());
    }
  }

  mod test_handle_esc {
    use rstest::rstest;

    use super::*;
    use crate::assert_navigation_popped;

    const ESC_KEY: Key = DEFAULT_KEYBINDINGS.esc.key;

    #[rstest]
    #[case(
      ActiveReadarrBlock::Downloads,
      ActiveReadarrBlock::DeleteDownloadPrompt
    )]
    #[case(
      ActiveReadarrBlock::Downloads,
      ActiveReadarrBlock::UpdateDownloadsPrompt
    )]
    fn test_downloads_prompt_blocks_esc(
      #[case] base_block: ActiveReadarrBlock,
      #[case] prompt_block: ActiveReadarrBlock,
    ) {
      let mut app = App::test_default();
      app.push_navigation_stack(base_block.into());
      app.push_navigation_stack(prompt_block.into());
      app.data.readarr_data.prompt_confirm = true;

      DownloadsHandler::new(ESC_KEY, &mut app, prompt_block, None).handle();

      assert_navigation_popped!(app, base_block.into());
      assert!(!app.data.readarr_data.prompt_confirm);
    }

    #[rstest]
    fn test_default_esc(#[values(true, false)] is_ready: bool) {
      let mut app = App::test_default();
      app.is_loading = is_ready;
      app.error = "test error".to_owned().into();
      app.push_navigation_stack(ActiveReadarrBlock::Downloads.into());
      app.push_navigation_stack(ActiveReadarrBlock::Downloads.into());

      DownloadsHandler::new(ESC_KEY, &mut app, ActiveReadarrBlock::Downloads, None).handle();

      assert_navigation_popped!(app, ActiveReadarrBlock::Downloads.into());
      assert_is_empty!(app.error.text);
    }
  }

  mod test_handle_key_char {
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    use super::*;
    use crate::network::readarr_network::ReadarrEvent;
    use crate::{assert_navigation_popped, assert_navigation_pushed};

    #[test]
    fn test_update_downloads_key() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveReadarrBlock::Downloads.into());
      app.data.readarr_data.downloads.set_items(downloads_vec());

      DownloadsHandler::new(
        DEFAULT_KEYBINDINGS.update.key,
        &mut app,
        ActiveReadarrBlock::Downloads,
        None,
      )
      .handle();

      assert_navigation_pushed!(app, ActiveReadarrBlock::UpdateDownloadsPrompt.into());
    }

    #[test]
    fn test_update_downloads_key_no_op_when_not_ready() {
      let mut app = App::test_default();
      app.is_loading = true;
      app.push_navigation_stack(ActiveReadarrBlock::Downloads.into());
      app.data.readarr_data.downloads.set_items(downloads_vec());

      DownloadsHandler::new(
        DEFAULT_KEYBINDINGS.update.key,
        &mut app,
        ActiveReadarrBlock::Downloads,
        None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveReadarrBlock::Downloads.into()
      );
    }

    #[test]
    fn test_refresh_downloads_key() {
      let mut app = App::test_default();
      app.data.readarr_data.downloads.set_items(downloads_vec());
      app.push_navigation_stack(ActiveReadarrBlock::Downloads.into());

      DownloadsHandler::new(
        DEFAULT_KEYBINDINGS.refresh.key,
        &mut app,
        ActiveReadarrBlock::Downloads,
        None,
      )
      .handle();

      assert_navigation_pushed!(app, ActiveReadarrBlock::Downloads.into());
      assert!(app.should_refresh);
    }

    #[test]
    fn test_refresh_downloads_key_no_op_when_not_ready() {
      let mut app = App::test_default();
      app.is_loading = true;
      app.push_navigation_stack(ActiveReadarrBlock::Downloads.into());
      app.data.readarr_data.downloads.set_items(downloads_vec());

      DownloadsHandler::new(
        DEFAULT_KEYBINDINGS.refresh.key,
        &mut app,
        ActiveReadarrBlock::Downloads,
        None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveReadarrBlock::Downloads.into()
      );
      assert!(!app.should_refresh);
    }

    #[rstest]
    #[case(
      ActiveReadarrBlock::Downloads,
      ActiveReadarrBlock::DeleteDownloadPrompt,
      ReadarrEvent::DeleteDownload(3)
    )]
    #[case(
      ActiveReadarrBlock::Downloads,
      ActiveReadarrBlock::UpdateDownloadsPrompt,
      ReadarrEvent::UpdateDownloads
    )]
    fn test_downloads_prompt_confirm(
      #[case] base_route: ActiveReadarrBlock,
      #[case] prompt_block: ActiveReadarrBlock,
      #[case] expected_action: ReadarrEvent,
    ) {
      let mut app = App::test_default();
      app.data.readarr_data.downloads.set_items(downloads_vec());
      app.push_navigation_stack(base_route.into());
      app.push_navigation_stack(prompt_block.into());

      DownloadsHandler::new(
        DEFAULT_KEYBINDINGS.confirm.key,
        &mut app,
        prompt_block,
        None,
      )
      .handle();

      assert!(app.data.readarr_data.prompt_confirm);
      assert_some_eq_x!(
        &app.data.readarr_data.prompt_confirm_action,
        &expected_action
      );
      assert_navigation_popped!(app, base_route.into());
    }
  }

  #[test]
  fn test_downloads_handler_accepts() {
    ActiveReadarrBlock::iter().for_each(|active_readarr_block| {
      if DOWNLOADS_BLOCKS.contains(&active_readarr_block) {
        assert!(DownloadsHandler::accepts(active_readarr_block));
      } else {
        assert!(!DownloadsHandler::accepts(active_readarr_block));
      }
    })
  }

  #[rstest]
  fn test_downloads_handler_ignore_special_keys(
    #[values(true, false)] ignore_special_keys_for_textbox_input: bool,
  ) {
    let mut app = App::test_default();
    app.ignore_special_keys_for_textbox_input = ignore_special_keys_for_textbox_input;
    let handler = DownloadsHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveReadarrBlock::default(),
      None,
    );

    assert_eq!(
      handler.ignore_special_keys(),
      ignore_special_keys_for_textbox_input
    );
  }

  #[test]
  fn test_extract_download_id() {
    let mut app = App::test_default();
    app.data.readarr_data.downloads.set_items(downloads_vec());
    app.data.readarr_data.downloads.select_index(Some(1));

    let download_id = DownloadsHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveReadarrBlock::Downloads,
      None,
    )
    .extract_download_id();

    assert_eq!(download_id, 2);
  }

  #[test]
  fn test_downloads_handler_not_ready_when_loading() {
    let mut app = App::test_default();
    app.push_navigation_stack(ActiveReadarrBlock::Downloads.into());
    app.is_loading = true;

    let handler = DownloadsHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveReadarrBlock::Downloads,
      None,
    );

    assert!(!handler.is_ready());
  }

  #[test]
  fn test_downloads_handler_not_ready_when_downloads_is_empty() {
    let mut app = App::test_default();
    app.push_navigation_stack(ActiveReadarrBlock::Downloads.into());
    app.is_loading = false;

    let handler = DownloadsHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveReadarrBlock::Downloads,
      None,
    );

    assert!(!handler.is_ready());
  }

  #[test]
  fn test_downloads_handler_ready_when_not_loading_and_downloads_is_not_empty() {
    let mut app = App::test_default();
    app.push_navigation_stack(ActiveReadarrBlock::Downloads.into());
    app.is_loading = false;
    app
      .data
      .readarr_data
      .downloads
      .set_items(vec![DownloadRecord::default()]);

    let handler = DownloadsHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveReadarrBlock::Downloads,
      None,
    );

    assert!(handler.is_ready());
  }

  fn downloads_vec() -> Vec<DownloadRecord> {
    vec![
      DownloadRecord {
        title: "Test Book Download Three".to_owned(),
        status: DownloadStatus::Downloading,
        id: 3,
        book_id: Some(Number::from(13)),
        author_id: Some(Number::from(23)),
        size: 3000.0,
        sizeleft: 750.0,
        output_path: Some("/nfs/nzbget/completed/books/Zora Neale - Test Book Three".into()),
        indexer: "test-indexer-three".to_owned(),
        download_client: Some("NZBGet".to_owned()),
      },
      DownloadRecord {
        title: "Test Book Download Two".to_owned(),
        status: DownloadStatus::Queued,
        id: 2,
        book_id: Some(Number::from(12)),
        author_id: Some(Number::from(22)),
        size: 2000.0,
        sizeleft: 500.0,
        output_path: Some("/nfs/nzbget/completed/books/Mary Shelley - Test Book Two".into()),
        indexer: "test-indexer-two".to_owned(),
        download_client: Some("Transmission".to_owned()),
      },
      DownloadRecord {
        title: "Test Book Download One".to_owned(),
        status: DownloadStatus::Paused,
        id: 1,
        book_id: Some(Number::from(11)),
        author_id: Some(Number::from(21)),
        size: 1000.0,
        sizeleft: 250.0,
        output_path: Some("/nfs/nzbget/completed/books/Ursula Le Guin - Test Book One".into()),
        indexer: "test-indexer-one".to_owned(),
        download_client: Some("SABnzbd".to_owned()),
      },
    ]
  }
}
