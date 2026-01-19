#[cfg(test)]
mod tests {
  use pretty_assertions::assert_eq;
  use rstest::rstest;
  use strum::IntoEnumIterator;

  use crate::app::App;
  use crate::app::key_binding::DEFAULT_KEYBINDINGS;
  use crate::assert_navigation_pushed;
  use crate::event::Key;
  use crate::handlers::KeyEventHandler;
  use crate::handlers::lidarr_handlers::downloads::DownloadsHandler;
  use crate::models::lidarr_models::DownloadRecord;
  use crate::models::servarr_data::lidarr::lidarr_data::{ActiveLidarrBlock, DOWNLOADS_BLOCKS};
  use crate::network::lidarr_network::lidarr_network_test_utils::test_utils::download_record;

  mod test_handle_delete {
    use pretty_assertions::assert_eq;

    use super::*;

    const DELETE_KEY: Key = DEFAULT_KEYBINDINGS.delete.key;

    #[test]
    fn test_delete_download_prompt() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveLidarrBlock::Downloads.into());
      app
        .data
        .lidarr_data
        .downloads
        .set_items(vec![DownloadRecord::default()]);

      DownloadsHandler::new(DELETE_KEY, &mut app, ActiveLidarrBlock::Downloads, None).handle();

      assert_navigation_pushed!(app, ActiveLidarrBlock::DeleteDownloadPrompt.into());
    }

    #[test]
    fn test_delete_download_prompt_no_op_when_not_ready() {
      let mut app = App::test_default();
      app.is_loading = true;
      app.push_navigation_stack(ActiveLidarrBlock::Downloads.into());
      app
        .data
        .lidarr_data
        .downloads
        .set_items(vec![DownloadRecord::default()]);

      DownloadsHandler::new(DELETE_KEY, &mut app, ActiveLidarrBlock::Downloads, None).handle();

      assert_eq!(app.get_current_route(), ActiveLidarrBlock::Downloads.into());
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
      app.push_navigation_stack(ActiveLidarrBlock::Downloads.into());
      app.is_loading = is_ready;
      app.data.lidarr_data.main_tabs.set_index(1);

      DownloadsHandler::new(
        DEFAULT_KEYBINDINGS.left.key,
        &mut app,
        ActiveLidarrBlock::Downloads,
        None,
      )
      .handle();

      assert_eq!(
        app.data.lidarr_data.main_tabs.get_active_route(),
        ActiveLidarrBlock::Artists.into()
      );
      assert_navigation_pushed!(app, ActiveLidarrBlock::Artists.into());
    }

    #[rstest]
    fn test_downloads_tab_right(#[values(true, false)] is_ready: bool) {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveLidarrBlock::Downloads.into());
      app.is_loading = is_ready;
      app.data.lidarr_data.main_tabs.set_index(1);

      DownloadsHandler::new(
        DEFAULT_KEYBINDINGS.right.key,
        &mut app,
        ActiveLidarrBlock::Downloads,
        None,
      )
      .handle();

      assert_eq!(
        app.data.lidarr_data.main_tabs.get_active_route(),
        ActiveLidarrBlock::Blocklist.into()
      );
      assert_navigation_pushed!(app, ActiveLidarrBlock::Blocklist.into());
    }

    #[rstest]
    fn test_downloads_left_right_prompt_toggle(
      #[values(
        ActiveLidarrBlock::DeleteDownloadPrompt,
        ActiveLidarrBlock::UpdateDownloadsPrompt
      )]
      active_lidarr_block: ActiveLidarrBlock,
      #[values(DEFAULT_KEYBINDINGS.left.key, DEFAULT_KEYBINDINGS.right.key)] key: Key,
    ) {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveLidarrBlock::Downloads.into());

      DownloadsHandler::new(key, &mut app, active_lidarr_block, None).handle();

      assert!(app.data.lidarr_data.prompt_confirm);

      DownloadsHandler::new(key, &mut app, active_lidarr_block, None).handle();

      assert!(!app.data.lidarr_data.prompt_confirm);
    }
  }

  mod test_handle_submit {
    use rstest::rstest;

    use crate::network::lidarr_network::LidarrEvent;

    use super::*;
    use crate::assert_navigation_popped;
    use crate::network::lidarr_network::lidarr_network_test_utils::test_utils::download_record;

    const SUBMIT_KEY: Key = DEFAULT_KEYBINDINGS.submit.key;

    #[rstest]
    #[case(
      ActiveLidarrBlock::Downloads,
      ActiveLidarrBlock::DeleteDownloadPrompt,
      LidarrEvent::DeleteDownload(1)
    )]
    #[case(
      ActiveLidarrBlock::Downloads,
      ActiveLidarrBlock::UpdateDownloadsPrompt,
      LidarrEvent::UpdateDownloads
    )]
    fn test_downloads_prompt_confirm_submit(
      #[case] base_route: ActiveLidarrBlock,
      #[case] prompt_block: ActiveLidarrBlock,
      #[case] expected_action: LidarrEvent,
    ) {
      let mut app = App::test_default();
      app
        .data
        .lidarr_data
        .downloads
        .set_items(vec![download_record()]);
      app.data.lidarr_data.prompt_confirm = true;
      app.push_navigation_stack(base_route.into());
      app.push_navigation_stack(prompt_block.into());

      DownloadsHandler::new(SUBMIT_KEY, &mut app, prompt_block, None).handle();

      assert!(app.data.lidarr_data.prompt_confirm);
      assert_some_eq_x!(
        &app.data.lidarr_data.prompt_confirm_action,
        &expected_action
      );
      assert_navigation_popped!(app, base_route.into());
    }

    #[rstest]
    #[case(ActiveLidarrBlock::Downloads, ActiveLidarrBlock::DeleteDownloadPrompt)]
    #[case(ActiveLidarrBlock::Downloads, ActiveLidarrBlock::UpdateDownloadsPrompt)]
    fn test_downloads_prompt_decline_submit(
      #[case] base_route: ActiveLidarrBlock,
      #[case] prompt_block: ActiveLidarrBlock,
    ) {
      let mut app = App::test_default();
      app
        .data
        .lidarr_data
        .downloads
        .set_items(vec![DownloadRecord::default()]);
      app.push_navigation_stack(base_route.into());
      app.push_navigation_stack(prompt_block.into());

      DownloadsHandler::new(SUBMIT_KEY, &mut app, prompt_block, None).handle();

      assert!(!app.data.lidarr_data.prompt_confirm);
      assert_none!(app.data.lidarr_data.prompt_confirm_action);
      assert_navigation_popped!(app, base_route.into());
    }
  }

  mod test_handle_esc {
    use rstest::rstest;

    use super::*;
    use crate::assert_navigation_popped;

    const ESC_KEY: Key = DEFAULT_KEYBINDINGS.esc.key;

    #[rstest]
    #[case(ActiveLidarrBlock::Downloads, ActiveLidarrBlock::DeleteDownloadPrompt)]
    #[case(ActiveLidarrBlock::Downloads, ActiveLidarrBlock::UpdateDownloadsPrompt)]
    fn test_downloads_prompt_blocks_esc(
      #[case] base_block: ActiveLidarrBlock,
      #[case] prompt_block: ActiveLidarrBlock,
    ) {
      let mut app = App::test_default();
      app.push_navigation_stack(base_block.into());
      app.push_navigation_stack(prompt_block.into());
      app.data.lidarr_data.prompt_confirm = true;

      DownloadsHandler::new(ESC_KEY, &mut app, prompt_block, None).handle();

      assert_navigation_popped!(app, base_block.into());
      assert!(!app.data.lidarr_data.prompt_confirm);
    }

    #[rstest]
    fn test_default_esc(#[values(true, false)] is_ready: bool) {
      let mut app = App::test_default();
      app.is_loading = is_ready;
      app.error = "test error".to_owned().into();
      app.push_navigation_stack(ActiveLidarrBlock::Downloads.into());
      app.push_navigation_stack(ActiveLidarrBlock::Downloads.into());

      DownloadsHandler::new(ESC_KEY, &mut app, ActiveLidarrBlock::Downloads, None).handle();

      assert_navigation_popped!(app, ActiveLidarrBlock::Downloads.into());
      assert_is_empty!(app.error.text);
    }
  }

  mod test_handle_key_char {
    use super::*;
    use crate::assert_navigation_popped;
    use crate::network::lidarr_network::LidarrEvent;
    use crate::network::lidarr_network::lidarr_network_test_utils::test_utils::download_record;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    #[test]
    fn test_update_downloads_key() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveLidarrBlock::Downloads.into());
      app
        .data
        .lidarr_data
        .downloads
        .set_items(vec![DownloadRecord::default()]);

      DownloadsHandler::new(
        DEFAULT_KEYBINDINGS.update.key,
        &mut app,
        ActiveLidarrBlock::Downloads,
        None,
      )
      .handle();

      assert_navigation_pushed!(app, ActiveLidarrBlock::UpdateDownloadsPrompt.into());
    }

    #[test]
    fn test_update_downloads_key_no_op_when_not_ready() {
      let mut app = App::test_default();
      app.is_loading = true;
      app.push_navigation_stack(ActiveLidarrBlock::Downloads.into());
      app
        .data
        .lidarr_data
        .downloads
        .set_items(vec![DownloadRecord::default()]);

      DownloadsHandler::new(
        DEFAULT_KEYBINDINGS.update.key,
        &mut app,
        ActiveLidarrBlock::Downloads,
        None,
      )
      .handle();

      assert_eq!(app.get_current_route(), ActiveLidarrBlock::Downloads.into());
    }

    #[test]
    fn test_refresh_downloads_key() {
      let mut app = App::test_default();
      app
        .data
        .lidarr_data
        .downloads
        .set_items(vec![DownloadRecord::default()]);
      app.push_navigation_stack(ActiveLidarrBlock::Downloads.into());

      DownloadsHandler::new(
        DEFAULT_KEYBINDINGS.refresh.key,
        &mut app,
        ActiveLidarrBlock::Downloads,
        None,
      )
      .handle();

      assert_navigation_pushed!(app, ActiveLidarrBlock::Downloads.into());
      assert!(app.should_refresh);
    }

    #[test]
    fn test_refresh_downloads_key_no_op_when_not_ready() {
      let mut app = App::test_default();
      app.is_loading = true;
      app.push_navigation_stack(ActiveLidarrBlock::Downloads.into());
      app
        .data
        .lidarr_data
        .downloads
        .set_items(vec![DownloadRecord::default()]);

      DownloadsHandler::new(
        DEFAULT_KEYBINDINGS.refresh.key,
        &mut app,
        ActiveLidarrBlock::Downloads,
        None,
      )
      .handle();

      assert_eq!(app.get_current_route(), ActiveLidarrBlock::Downloads.into());
      assert!(!app.should_refresh);
    }

    #[rstest]
    #[case(
      ActiveLidarrBlock::Downloads,
      ActiveLidarrBlock::DeleteDownloadPrompt,
      LidarrEvent::DeleteDownload(1)
    )]
    #[case(
      ActiveLidarrBlock::Downloads,
      ActiveLidarrBlock::UpdateDownloadsPrompt,
      LidarrEvent::UpdateDownloads
    )]
    fn test_downloads_prompt_confirm_submit(
      #[case] base_route: ActiveLidarrBlock,
      #[case] prompt_block: ActiveLidarrBlock,
      #[case] expected_action: LidarrEvent,
    ) {
      let mut app = App::test_default();
      app
        .data
        .lidarr_data
        .downloads
        .set_items(vec![download_record()]);
      app.push_navigation_stack(base_route.into());
      app.push_navigation_stack(prompt_block.into());

      DownloadsHandler::new(
        DEFAULT_KEYBINDINGS.confirm.key,
        &mut app,
        prompt_block,
        None,
      )
      .handle();

      assert!(app.data.lidarr_data.prompt_confirm);
      assert_some_eq_x!(
        &app.data.lidarr_data.prompt_confirm_action,
        &expected_action
      );
      assert_navigation_popped!(app, base_route.into());
    }
  }

  #[test]
  fn test_downloads_handler_accepts() {
    ActiveLidarrBlock::iter().for_each(|active_lidarr_block| {
      if DOWNLOADS_BLOCKS.contains(&active_lidarr_block) {
        assert!(DownloadsHandler::accepts(active_lidarr_block));
      } else {
        assert!(!DownloadsHandler::accepts(active_lidarr_block));
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
      ActiveLidarrBlock::default(),
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
    app
      .data
      .lidarr_data
      .downloads
      .set_items(vec![download_record()]);

    let download_id = DownloadsHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveLidarrBlock::Downloads,
      None,
    )
    .extract_download_id();

    assert_eq!(download_id, 1);
  }

  #[test]
  fn test_downloads_handler_not_ready_when_loading() {
    let mut app = App::test_default();
    app.push_navigation_stack(ActiveLidarrBlock::Downloads.into());
    app.is_loading = true;

    let handler = DownloadsHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveLidarrBlock::Downloads,
      None,
    );

    assert!(!handler.is_ready());
  }

  #[test]
  fn test_downloads_handler_not_ready_when_downloads_is_empty() {
    let mut app = App::test_default();
    app.push_navigation_stack(ActiveLidarrBlock::Downloads.into());
    app.is_loading = false;

    let handler = DownloadsHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveLidarrBlock::Downloads,
      None,
    );

    assert!(!handler.is_ready());
  }

  #[test]
  fn test_downloads_handler_ready_when_not_loading_and_downloads_is_not_empty() {
    let mut app = App::test_default();
    app.push_navigation_stack(ActiveLidarrBlock::Downloads.into());
    app.is_loading = false;

    app
      .data
      .lidarr_data
      .downloads
      .set_items(vec![DownloadRecord::default()]);
    let handler = DownloadsHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveLidarrBlock::Downloads,
      None,
    );

    assert!(handler.is_ready());
  }
}
