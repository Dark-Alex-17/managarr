#[cfg(test)]
mod tests {
  use pretty_assertions::assert_str_eq;
  use strum::IntoEnumIterator;

  use crate::app::key_binding::DEFAULT_KEYBINDINGS;
  use crate::app::App;
  use crate::event::Key;
  use crate::handlers::sonarr_handlers::downloads::DownloadsHandler;
  use crate::handlers::KeyEventHandler;
  use crate::models::servarr_data::sonarr::sonarr_data::{ActiveSonarrBlock, DOWNLOADS_BLOCKS};
  use crate::models::sonarr_models::DownloadRecord;

  mod test_handle_scroll_up_and_down {
    use rstest::rstest;

    use crate::models::sonarr_models::DownloadRecord;
    use crate::{simple_stateful_iterable_vec, test_iterable_scroll};

    use super::*;

    test_iterable_scroll!(
      test_downloads_scroll,
      DownloadsHandler,
      sonarr_data,
      downloads,
      DownloadRecord,
      ActiveSonarrBlock::Downloads,
      None,
      title
    );

    #[rstest]
    fn test_downloads_scroll_no_op_when_not_ready(
      #[values(
			DEFAULT_KEYBINDINGS.up.key, DEFAULT_KEYBINDINGS.down.key
		)]
      key: Key,
    ) {
      let mut app = App::default();
      app.push_navigation_stack(ActiveSonarrBlock::Downloads.into());
      app.is_loading = true;
      app
        .data
        .sonarr_data
        .downloads
        .set_items(simple_stateful_iterable_vec!(DownloadRecord));

      DownloadsHandler::with(key, &mut app, ActiveSonarrBlock::Downloads, None).handle();

      assert_str_eq!(
        app.data.sonarr_data.downloads.current_selection().title,
        "Test 1"
      );

      DownloadsHandler::with(key, &mut app, ActiveSonarrBlock::Downloads, None).handle();

      assert_str_eq!(
        app.data.sonarr_data.downloads.current_selection().title,
        "Test 1"
      );
    }
  }

  mod test_handle_home_end {
    use crate::models::sonarr_models::DownloadRecord;
    use crate::{extended_stateful_iterable_vec, test_iterable_home_and_end};

    use super::*;

    test_iterable_home_and_end!(
      test_downloads_home_end,
      DownloadsHandler,
      sonarr_data,
      downloads,
      DownloadRecord,
      ActiveSonarrBlock::Downloads,
      None,
      title
    );

    #[test]
    fn test_downloads_home_end_no_op_when_not_ready() {
      let mut app = App::default();
      app.push_navigation_stack(ActiveSonarrBlock::Downloads.into());
      app.is_loading = true;
      app
        .data
        .sonarr_data
        .downloads
        .set_items(extended_stateful_iterable_vec!(DownloadRecord));

      DownloadsHandler::with(
        DEFAULT_KEYBINDINGS.end.key,
        &mut app,
        ActiveSonarrBlock::Downloads,
        None,
      )
      .handle();

      assert_str_eq!(
        app.data.sonarr_data.downloads.current_selection().title,
        "Test 1"
      );

      DownloadsHandler::with(
        DEFAULT_KEYBINDINGS.home.key,
        &mut app,
        ActiveSonarrBlock::Downloads,
        None,
      )
      .handle();

      assert_str_eq!(
        app.data.sonarr_data.downloads.current_selection().title,
        "Test 1"
      );
    }
  }

  mod test_handle_delete {
    use pretty_assertions::assert_eq;

    use super::*;

    const DELETE_KEY: Key = DEFAULT_KEYBINDINGS.delete.key;

    #[test]
    fn test_delete_download_prompt() {
      let mut app = App::default();
      app.push_navigation_stack(ActiveSonarrBlock::Downloads.into());
      app
        .data
        .sonarr_data
        .downloads
        .set_items(vec![DownloadRecord::default()]);

      DownloadsHandler::with(DELETE_KEY, &mut app, ActiveSonarrBlock::Downloads, None).handle();

      assert_eq!(
        app.get_current_route(),
        ActiveSonarrBlock::DeleteDownloadPrompt.into()
      );
    }

    #[test]
    fn test_delete_download_prompt_no_op_when_not_ready() {
      let mut app = App::default();
      app.is_loading = true;
      app.push_navigation_stack(ActiveSonarrBlock::Downloads.into());
      app
        .data
        .sonarr_data
        .downloads
        .set_items(vec![DownloadRecord::default()]);

      DownloadsHandler::with(DELETE_KEY, &mut app, ActiveSonarrBlock::Downloads, None).handle();

      assert_eq!(app.get_current_route(), ActiveSonarrBlock::Downloads.into());
    }
  }

  mod test_handle_left_right_action {
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    use super::*;

    #[rstest]
    fn test_downloads_tab_left(#[values(true, false)] is_ready: bool) {
      let mut app = App::default();
      app.push_navigation_stack(ActiveSonarrBlock::Downloads.into());
      app.is_loading = is_ready;
      app.data.sonarr_data.main_tabs.set_index(1);

      DownloadsHandler::with(
        DEFAULT_KEYBINDINGS.left.key,
        &mut app,
        ActiveSonarrBlock::Downloads,
        None,
      )
      .handle();

      assert_eq!(
        app.data.sonarr_data.main_tabs.get_active_route(),
        ActiveSonarrBlock::Series.into()
      );
      assert_eq!(app.get_current_route(), ActiveSonarrBlock::Series.into());
    }

    #[rstest]
    fn test_downloads_tab_right(#[values(true, false)] is_ready: bool) {
      let mut app = App::default();
      app.push_navigation_stack(ActiveSonarrBlock::Downloads.into());
      app.is_loading = is_ready;
      app.data.sonarr_data.main_tabs.set_index(1);

      DownloadsHandler::with(
        DEFAULT_KEYBINDINGS.right.key,
        &mut app,
        ActiveSonarrBlock::Downloads,
        None,
      )
      .handle();

      assert_eq!(
        app.data.sonarr_data.main_tabs.get_active_route(),
        ActiveSonarrBlock::Blocklist.into()
      );
      assert_eq!(app.get_current_route(), ActiveSonarrBlock::Blocklist.into());
    }

    #[rstest]
    fn test_downloads_left_right_prompt_toggle(
      #[values(
        ActiveSonarrBlock::DeleteDownloadPrompt,
        ActiveSonarrBlock::UpdateDownloadsPrompt
      )]
      active_sonarr_block: ActiveSonarrBlock,
      #[values(DEFAULT_KEYBINDINGS.left.key, DEFAULT_KEYBINDINGS.right.key)] key: Key,
    ) {
      let mut app = App::default();
      app.push_navigation_stack(ActiveSonarrBlock::Downloads.into());

      DownloadsHandler::with(key, &mut app, active_sonarr_block, None).handle();

      assert!(app.data.sonarr_data.prompt_confirm);

      DownloadsHandler::with(key, &mut app, active_sonarr_block, None).handle();

      assert!(!app.data.sonarr_data.prompt_confirm);
    }
  }

  mod test_handle_submit {
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    use crate::network::sonarr_network::SonarrEvent;

    use super::*;

    const SUBMIT_KEY: Key = DEFAULT_KEYBINDINGS.submit.key;

    #[rstest]
    #[case(
      ActiveSonarrBlock::Downloads,
      ActiveSonarrBlock::DeleteDownloadPrompt,
      SonarrEvent::DeleteDownload(None)
    )]
    #[case(
      ActiveSonarrBlock::Downloads,
      ActiveSonarrBlock::UpdateDownloadsPrompt,
      SonarrEvent::UpdateDownloads
    )]
    fn test_downloads_prompt_confirm_submit(
      #[case] base_route: ActiveSonarrBlock,
      #[case] prompt_block: ActiveSonarrBlock,
      #[case] expected_action: SonarrEvent,
    ) {
      let mut app = App::default();
      app
        .data
        .sonarr_data
        .downloads
        .set_items(vec![DownloadRecord::default()]);
      app.data.sonarr_data.prompt_confirm = true;
      app.push_navigation_stack(base_route.into());
      app.push_navigation_stack(prompt_block.into());

      DownloadsHandler::with(SUBMIT_KEY, &mut app, prompt_block, None).handle();

      assert!(app.data.sonarr_data.prompt_confirm);
      assert_eq!(
        app.data.sonarr_data.prompt_confirm_action,
        Some(expected_action)
      );
      assert_eq!(app.get_current_route(), base_route.into());
    }

    #[rstest]
    #[case(ActiveSonarrBlock::Downloads, ActiveSonarrBlock::DeleteDownloadPrompt)]
    #[case(ActiveSonarrBlock::Downloads, ActiveSonarrBlock::UpdateDownloadsPrompt)]
    fn test_downloads_prompt_decline_submit(
      #[case] base_route: ActiveSonarrBlock,
      #[case] prompt_block: ActiveSonarrBlock,
    ) {
      let mut app = App::default();
      app
        .data
        .sonarr_data
        .downloads
        .set_items(vec![DownloadRecord::default()]);
      app.push_navigation_stack(base_route.into());
      app.push_navigation_stack(prompt_block.into());

      DownloadsHandler::with(SUBMIT_KEY, &mut app, prompt_block, None).handle();

      assert!(!app.data.sonarr_data.prompt_confirm);
      assert_eq!(app.data.sonarr_data.prompt_confirm_action, None);
      assert_eq!(app.get_current_route(), base_route.into());
    }
  }

  mod test_handle_esc {
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    use super::*;

    const ESC_KEY: Key = DEFAULT_KEYBINDINGS.esc.key;

    #[rstest]
    #[case(ActiveSonarrBlock::Downloads, ActiveSonarrBlock::DeleteDownloadPrompt)]
    #[case(ActiveSonarrBlock::Downloads, ActiveSonarrBlock::UpdateDownloadsPrompt)]
    fn test_downloads_prompt_blocks_esc(
      #[case] base_block: ActiveSonarrBlock,
      #[case] prompt_block: ActiveSonarrBlock,
    ) {
      let mut app = App::default();
      app.push_navigation_stack(base_block.into());
      app.push_navigation_stack(prompt_block.into());
      app.data.sonarr_data.prompt_confirm = true;

      DownloadsHandler::with(ESC_KEY, &mut app, prompt_block, None).handle();

      assert_eq!(app.get_current_route(), base_block.into());
      assert!(!app.data.sonarr_data.prompt_confirm);
    }

    #[rstest]
    fn test_default_esc(#[values(true, false)] is_ready: bool) {
      let mut app = App::default();
      app.is_loading = is_ready;
      app.error = "test error".to_owned().into();
      app.push_navigation_stack(ActiveSonarrBlock::Downloads.into());
      app.push_navigation_stack(ActiveSonarrBlock::Downloads.into());

      DownloadsHandler::with(ESC_KEY, &mut app, ActiveSonarrBlock::Downloads, None).handle();

      assert_eq!(app.get_current_route(), ActiveSonarrBlock::Downloads.into());
      assert!(app.error.text.is_empty());
    }
  }

  mod test_handle_key_char {
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    use crate::network::sonarr_network::SonarrEvent;

    use super::*;

    #[test]
    fn test_update_downloads_key() {
      let mut app = App::default();
      app.push_navigation_stack(ActiveSonarrBlock::Downloads.into());
      app
        .data
        .sonarr_data
        .downloads
        .set_items(vec![DownloadRecord::default()]);

      DownloadsHandler::with(
        DEFAULT_KEYBINDINGS.update.key,
        &mut app,
        ActiveSonarrBlock::Downloads,
        None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveSonarrBlock::UpdateDownloadsPrompt.into()
      );
    }

    #[test]
    fn test_update_downloads_key_no_op_when_not_ready() {
      let mut app = App::default();
      app.is_loading = true;
      app.push_navigation_stack(ActiveSonarrBlock::Downloads.into());
      app
        .data
        .sonarr_data
        .downloads
        .set_items(vec![DownloadRecord::default()]);

      DownloadsHandler::with(
        DEFAULT_KEYBINDINGS.update.key,
        &mut app,
        ActiveSonarrBlock::Downloads,
        None,
      )
      .handle();

      assert_eq!(app.get_current_route(), ActiveSonarrBlock::Downloads.into());
    }

    #[test]
    fn test_refresh_downloads_key() {
      let mut app = App::default();
      app
        .data
        .sonarr_data
        .downloads
        .set_items(vec![DownloadRecord::default()]);
      app.push_navigation_stack(ActiveSonarrBlock::Downloads.into());

      DownloadsHandler::with(
        DEFAULT_KEYBINDINGS.refresh.key,
        &mut app,
        ActiveSonarrBlock::Downloads,
        None,
      )
      .handle();

      assert_eq!(app.get_current_route(), ActiveSonarrBlock::Downloads.into());
      assert!(app.should_refresh);
    }

    #[test]
    fn test_refresh_downloads_key_no_op_when_not_ready() {
      let mut app = App::default();
      app.is_loading = true;
      app.push_navigation_stack(ActiveSonarrBlock::Downloads.into());
      app
        .data
        .sonarr_data
        .downloads
        .set_items(vec![DownloadRecord::default()]);

      DownloadsHandler::with(
        DEFAULT_KEYBINDINGS.refresh.key,
        &mut app,
        ActiveSonarrBlock::Downloads,
        None,
      )
      .handle();

      assert_eq!(app.get_current_route(), ActiveSonarrBlock::Downloads.into());
      assert!(!app.should_refresh);
    }

    #[rstest]
    #[case(
      ActiveSonarrBlock::Downloads,
      ActiveSonarrBlock::DeleteDownloadPrompt,
      SonarrEvent::DeleteDownload(None)
    )]
    #[case(
      ActiveSonarrBlock::Downloads,
      ActiveSonarrBlock::UpdateDownloadsPrompt,
      SonarrEvent::UpdateDownloads
    )]
    fn test_downloads_prompt_confirm_submit(
      #[case] base_route: ActiveSonarrBlock,
      #[case] prompt_block: ActiveSonarrBlock,
      #[case] expected_action: SonarrEvent,
    ) {
      let mut app = App::default();
      app
        .data
        .sonarr_data
        .downloads
        .set_items(vec![DownloadRecord::default()]);
      app.push_navigation_stack(base_route.into());
      app.push_navigation_stack(prompt_block.into());

      DownloadsHandler::with(
        DEFAULT_KEYBINDINGS.confirm.key,
        &mut app,
        prompt_block,
        None,
      )
      .handle();

      assert!(app.data.sonarr_data.prompt_confirm);
      assert_eq!(
        app.data.sonarr_data.prompt_confirm_action,
        Some(expected_action)
      );
      assert_eq!(app.get_current_route(), base_route.into());
    }
  }

  #[test]
  fn test_downloads_handler_accepts() {
    ActiveSonarrBlock::iter().for_each(|active_sonarr_block| {
      if DOWNLOADS_BLOCKS.contains(&active_sonarr_block) {
        assert!(DownloadsHandler::accepts(active_sonarr_block));
      } else {
        assert!(!DownloadsHandler::accepts(active_sonarr_block));
      }
    })
  }

  #[test]
  fn test_downloads_handler_not_ready_when_loading() {
    let mut app = App::default();
    app.push_navigation_stack(ActiveSonarrBlock::Downloads.into());
    app.is_loading = true;

    let handler = DownloadsHandler::with(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveSonarrBlock::Downloads,
      None,
    );

    assert!(!handler.is_ready());
  }

  #[test]
  fn test_downloads_handler_not_ready_when_downloads_is_empty() {
    let mut app = App::default();
    app.push_navigation_stack(ActiveSonarrBlock::Downloads.into());
    app.is_loading = false;

    let handler = DownloadsHandler::with(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveSonarrBlock::Downloads,
      None,
    );

    assert!(!handler.is_ready());
  }

  #[test]
  fn test_downloads_handler_ready_when_not_loading_and_downloads_is_not_empty() {
    let mut app = App::default();
    app.push_navigation_stack(ActiveSonarrBlock::Downloads.into());
    app.is_loading = false;

    app
      .data
      .sonarr_data
      .downloads
      .set_items(vec![DownloadRecord::default()]);
    let handler = DownloadsHandler::with(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveSonarrBlock::Downloads,
      None,
    );

    assert!(handler.is_ready());
  }
}
