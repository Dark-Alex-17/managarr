#[cfg(test)]
mod tests {
  use pretty_assertions::assert_str_eq;
  use strum::IntoEnumIterator;

  use crate::app::key_binding::DEFAULT_KEYBINDINGS;
  use crate::app::App;
  use crate::event::Key;
  use crate::handlers::radarr_handlers::downloads::DownloadsHandler;
  use crate::handlers::KeyEventHandler;
  use crate::models::radarr_models::DownloadRecord;
  use crate::models::servarr_data::radarr::radarr_data::{ActiveRadarrBlock, DOWNLOADS_BLOCKS};

  mod test_handle_scroll_up_and_down {
    use rstest::rstest;

    use crate::models::radarr_models::DownloadRecord;
    use crate::{simple_stateful_iterable_vec, test_iterable_scroll};

    use super::*;

    test_iterable_scroll!(
      test_downloads_scroll,
      DownloadsHandler,
      downloads,
      DownloadRecord,
      ActiveRadarrBlock::Downloads,
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
      app.is_loading = true;
      app
        .data
        .radarr_data
        .downloads
        .set_items(simple_stateful_iterable_vec!(DownloadRecord));

      DownloadsHandler::with(key, &mut app, ActiveRadarrBlock::Downloads, None).handle();

      assert_str_eq!(
        app.data.radarr_data.downloads.current_selection().title,
        "Test 1"
      );

      DownloadsHandler::with(key, &mut app, ActiveRadarrBlock::Downloads, None).handle();

      assert_str_eq!(
        app.data.radarr_data.downloads.current_selection().title,
        "Test 1"
      );
    }
  }

  mod test_handle_home_end {
    use crate::models::radarr_models::DownloadRecord;
    use crate::{extended_stateful_iterable_vec, test_iterable_home_and_end};

    use super::*;

    test_iterable_home_and_end!(
      test_downloads_home_end,
      DownloadsHandler,
      downloads,
      DownloadRecord,
      ActiveRadarrBlock::Downloads,
      None,
      title
    );

    #[test]
    fn test_downloads_home_end_no_op_when_not_ready() {
      let mut app = App::default();
      app.is_loading = true;
      app
        .data
        .radarr_data
        .downloads
        .set_items(extended_stateful_iterable_vec!(DownloadRecord));

      DownloadsHandler::with(
        DEFAULT_KEYBINDINGS.end.key,
        &mut app,
        ActiveRadarrBlock::Downloads,
        None,
      )
      .handle();

      assert_str_eq!(
        app.data.radarr_data.downloads.current_selection().title,
        "Test 1"
      );

      DownloadsHandler::with(
        DEFAULT_KEYBINDINGS.home.key,
        &mut app,
        ActiveRadarrBlock::Downloads,
        None,
      )
      .handle();

      assert_str_eq!(
        app.data.radarr_data.downloads.current_selection().title,
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
      app
        .data
        .radarr_data
        .downloads
        .set_items(vec![DownloadRecord::default()]);

      DownloadsHandler::with(DELETE_KEY, &mut app, ActiveRadarrBlock::Downloads, None).handle();

      assert_eq!(
        app.get_current_route(),
        ActiveRadarrBlock::DeleteDownloadPrompt.into()
      );
    }

    #[test]
    fn test_delete_download_prompt_no_op_when_not_ready() {
      let mut app = App::default();
      app.is_loading = true;
      app.push_navigation_stack(ActiveRadarrBlock::Downloads.into());
      app
        .data
        .radarr_data
        .downloads
        .set_items(vec![DownloadRecord::default()]);

      DownloadsHandler::with(DELETE_KEY, &mut app, ActiveRadarrBlock::Downloads, None).handle();

      assert_eq!(app.get_current_route(), ActiveRadarrBlock::Downloads.into());
    }
  }

  mod test_handle_left_right_action {
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    use super::*;

    #[rstest]
    fn test_downloads_tab_left(#[values(true, false)] is_ready: bool) {
      let mut app = App::default();
      app.is_loading = is_ready;
      app.data.radarr_data.main_tabs.set_index(2);

      DownloadsHandler::with(
        DEFAULT_KEYBINDINGS.left.key,
        &mut app,
        ActiveRadarrBlock::Downloads,
        None,
      )
      .handle();

      assert_eq!(
        app.data.radarr_data.main_tabs.get_active_route(),
        ActiveRadarrBlock::Collections.into()
      );
      assert_eq!(
        app.get_current_route(),
        ActiveRadarrBlock::Collections.into()
      );
    }

    #[rstest]
    fn test_downloads_tab_right(#[values(true, false)] is_ready: bool) {
      let mut app = App::default();
      app.is_loading = is_ready;
      app.data.radarr_data.main_tabs.set_index(2);

      DownloadsHandler::with(
        DEFAULT_KEYBINDINGS.right.key,
        &mut app,
        ActiveRadarrBlock::Downloads,
        None,
      )
      .handle();

      assert_eq!(
        app.data.radarr_data.main_tabs.get_active_route(),
        ActiveRadarrBlock::Blocklist.into()
      );
      assert_eq!(app.get_current_route(), ActiveRadarrBlock::Blocklist.into());
    }

    #[rstest]
    fn test_downloads_left_right_prompt_toggle(
      #[values(
        ActiveRadarrBlock::DeleteDownloadPrompt,
        ActiveRadarrBlock::UpdateDownloadsPrompt
      )]
      active_radarr_block: ActiveRadarrBlock,
      #[values(DEFAULT_KEYBINDINGS.left.key, DEFAULT_KEYBINDINGS.right.key)] key: Key,
    ) {
      let mut app = App::default();

      DownloadsHandler::with(key, &mut app, active_radarr_block, None).handle();

      assert!(app.data.radarr_data.prompt_confirm);

      DownloadsHandler::with(key, &mut app, active_radarr_block, None).handle();

      assert!(!app.data.radarr_data.prompt_confirm);
    }
  }

  mod test_handle_submit {
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    use crate::network::radarr_network::RadarrEvent;

    use super::*;

    const SUBMIT_KEY: Key = DEFAULT_KEYBINDINGS.submit.key;

    #[rstest]
    #[case(
      ActiveRadarrBlock::Downloads,
      ActiveRadarrBlock::DeleteDownloadPrompt,
      RadarrEvent::DeleteDownload(None)
    )]
    #[case(
      ActiveRadarrBlock::Downloads,
      ActiveRadarrBlock::UpdateDownloadsPrompt,
      RadarrEvent::UpdateDownloads
    )]
    fn test_downloads_prompt_confirm_submit(
      #[case] base_route: ActiveRadarrBlock,
      #[case] prompt_block: ActiveRadarrBlock,
      #[case] expected_action: RadarrEvent,
    ) {
      let mut app = App::default();
      app
        .data
        .radarr_data
        .downloads
        .set_items(vec![DownloadRecord::default()]);
      app.data.radarr_data.prompt_confirm = true;
      app.push_navigation_stack(base_route.into());
      app.push_navigation_stack(prompt_block.into());

      DownloadsHandler::with(SUBMIT_KEY, &mut app, prompt_block, None).handle();

      assert!(app.data.radarr_data.prompt_confirm);
      assert_eq!(
        app.data.radarr_data.prompt_confirm_action,
        Some(expected_action)
      );
      assert_eq!(app.get_current_route(), base_route.into());
    }

    #[rstest]
    #[case(ActiveRadarrBlock::Downloads, ActiveRadarrBlock::DeleteDownloadPrompt)]
    #[case(ActiveRadarrBlock::Downloads, ActiveRadarrBlock::UpdateDownloadsPrompt)]
    fn test_downloads_prompt_decline_submit(
      #[case] base_route: ActiveRadarrBlock,
      #[case] prompt_block: ActiveRadarrBlock,
    ) {
      let mut app = App::default();
      app
        .data
        .radarr_data
        .downloads
        .set_items(vec![DownloadRecord::default()]);
      app.push_navigation_stack(base_route.into());
      app.push_navigation_stack(prompt_block.into());

      DownloadsHandler::with(SUBMIT_KEY, &mut app, prompt_block, None).handle();

      assert!(!app.data.radarr_data.prompt_confirm);
      assert_eq!(app.data.radarr_data.prompt_confirm_action, None);
      assert_eq!(app.get_current_route(), base_route.into());
    }
  }

  mod test_handle_esc {
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    use super::*;

    const ESC_KEY: Key = DEFAULT_KEYBINDINGS.esc.key;

    #[rstest]
    #[case(ActiveRadarrBlock::Downloads, ActiveRadarrBlock::DeleteDownloadPrompt)]
    #[case(ActiveRadarrBlock::Downloads, ActiveRadarrBlock::UpdateDownloadsPrompt)]
    fn test_downloads_prompt_blocks_esc(
      #[case] base_block: ActiveRadarrBlock,
      #[case] prompt_block: ActiveRadarrBlock,
    ) {
      let mut app = App::default();
      app.push_navigation_stack(base_block.into());
      app.push_navigation_stack(prompt_block.into());
      app.data.radarr_data.prompt_confirm = true;

      DownloadsHandler::with(ESC_KEY, &mut app, prompt_block, None).handle();

      assert_eq!(app.get_current_route(), base_block.into());
      assert!(!app.data.radarr_data.prompt_confirm);
    }

    #[rstest]
    fn test_default_esc(#[values(true, false)] is_ready: bool) {
      let mut app = App::default();
      app.is_loading = is_ready;
      app.error = "test error".to_owned().into();
      app.push_navigation_stack(ActiveRadarrBlock::Downloads.into());
      app.push_navigation_stack(ActiveRadarrBlock::Downloads.into());

      DownloadsHandler::with(ESC_KEY, &mut app, ActiveRadarrBlock::Downloads, None).handle();

      assert_eq!(app.get_current_route(), ActiveRadarrBlock::Downloads.into());
      assert!(app.error.text.is_empty());
    }
  }

  mod test_handle_key_char {
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    use crate::network::radarr_network::RadarrEvent;

    use super::*;

    #[test]
    fn test_update_downloads_key() {
      let mut app = App::default();
      app
        .data
        .radarr_data
        .downloads
        .set_items(vec![DownloadRecord::default()]);

      DownloadsHandler::with(
        DEFAULT_KEYBINDINGS.update.key,
        &mut app,
        ActiveRadarrBlock::Downloads,
        None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveRadarrBlock::UpdateDownloadsPrompt.into()
      );
    }

    #[test]
    fn test_update_downloads_key_no_op_when_not_ready() {
      let mut app = App::default();
      app.is_loading = true;
      app.push_navigation_stack(ActiveRadarrBlock::Downloads.into());
      app
        .data
        .radarr_data
        .downloads
        .set_items(vec![DownloadRecord::default()]);

      DownloadsHandler::with(
        DEFAULT_KEYBINDINGS.update.key,
        &mut app,
        ActiveRadarrBlock::Downloads,
        None,
      )
      .handle();

      assert_eq!(app.get_current_route(), ActiveRadarrBlock::Downloads.into());
    }

    #[test]
    fn test_refresh_downloads_key() {
      let mut app = App::default();
      app
        .data
        .radarr_data
        .downloads
        .set_items(vec![DownloadRecord::default()]);
      app.push_navigation_stack(ActiveRadarrBlock::Downloads.into());

      DownloadsHandler::with(
        DEFAULT_KEYBINDINGS.refresh.key,
        &mut app,
        ActiveRadarrBlock::Downloads,
        None,
      )
      .handle();

      assert_eq!(app.get_current_route(), ActiveRadarrBlock::Downloads.into());
      assert!(app.should_refresh);
    }

    #[test]
    fn test_refresh_downloads_key_no_op_when_not_ready() {
      let mut app = App::default();
      app.is_loading = true;
      app.push_navigation_stack(ActiveRadarrBlock::Downloads.into());
      app
        .data
        .radarr_data
        .downloads
        .set_items(vec![DownloadRecord::default()]);

      DownloadsHandler::with(
        DEFAULT_KEYBINDINGS.refresh.key,
        &mut app,
        ActiveRadarrBlock::Downloads,
        None,
      )
      .handle();

      assert_eq!(app.get_current_route(), ActiveRadarrBlock::Downloads.into());
      assert!(!app.should_refresh);
    }

    #[rstest]
    #[case(
      ActiveRadarrBlock::Downloads,
      ActiveRadarrBlock::DeleteDownloadPrompt,
      RadarrEvent::DeleteDownload(None)
    )]
    #[case(
      ActiveRadarrBlock::Downloads,
      ActiveRadarrBlock::UpdateDownloadsPrompt,
      RadarrEvent::UpdateDownloads
    )]
    fn test_downloads_prompt_confirm_submit(
      #[case] base_route: ActiveRadarrBlock,
      #[case] prompt_block: ActiveRadarrBlock,
      #[case] expected_action: RadarrEvent,
    ) {
      let mut app = App::default();
      app
        .data
        .radarr_data
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

      assert!(app.data.radarr_data.prompt_confirm);
      assert_eq!(
        app.data.radarr_data.prompt_confirm_action,
        Some(expected_action)
      );
      assert_eq!(app.get_current_route(), base_route.into());
    }
  }

  #[test]
  fn test_downloads_handler_accepts() {
    ActiveRadarrBlock::iter().for_each(|active_radarr_block| {
      if DOWNLOADS_BLOCKS.contains(&active_radarr_block) {
        assert!(DownloadsHandler::accepts(active_radarr_block));
      } else {
        assert!(!DownloadsHandler::accepts(active_radarr_block));
      }
    })
  }

  #[test]
  fn test_downloads_handler_not_ready_when_loading() {
    let mut app = App::default();
    app.is_loading = true;

    let handler = DownloadsHandler::with(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveRadarrBlock::Downloads,
      None,
    );

    assert!(!handler.is_ready());
  }

  #[test]
  fn test_downloads_handler_not_ready_when_downloads_is_empty() {
    let mut app = App::default();
    app.is_loading = false;

    let handler = DownloadsHandler::with(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveRadarrBlock::Downloads,
      None,
    );

    assert!(!handler.is_ready());
  }

  #[test]
  fn test_downloads_handler_ready_when_not_loading_and_downloads_is_not_empty() {
    let mut app = App::default();
    app.is_loading = false;

    app
      .data
      .radarr_data
      .downloads
      .set_items(vec![DownloadRecord::default()]);
    let handler = DownloadsHandler::with(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveRadarrBlock::Downloads,
      None,
    );

    assert!(handler.is_ready());
  }
}
