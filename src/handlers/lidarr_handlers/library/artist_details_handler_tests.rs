#[cfg(test)]
mod tests {
  use pretty_assertions::{assert_eq, assert_str_eq};
  use rstest::rstest;
  use serde_json::Number;
  use std::cmp::Ordering;
  use strum::IntoEnumIterator;

  use crate::app::App;
  use crate::app::key_binding::DEFAULT_KEYBINDINGS;
  use crate::handlers::KeyEventHandler;
  use crate::handlers::lidarr_handlers::library::artist_details_handler::{
    ArtistDetailsHandler, releases_sorting_options,
  };
  use crate::models::HorizontallyScrollableText;
  use crate::models::lidarr_models::{Album, LidarrHistoryItem, LidarrRelease};
  use crate::models::servarr_data::lidarr::lidarr_data::{
    ARTIST_DETAILS_BLOCKS, ActiveLidarrBlock, DELETE_ALBUM_BLOCKS,
  };
  use crate::models::servarr_models::{Quality, QualityWrapper};

  mod test_handle_delete {
    use super::*;
    use crate::assert_delete_prompt;
    use crate::event::Key;
    use crate::models::lidarr_models::Album;
    use crate::models::servarr_data::lidarr::lidarr_data::DELETE_ALBUM_SELECTION_BLOCKS;
    use pretty_assertions::assert_eq;

    const DELETE_KEY: Key = DEFAULT_KEYBINDINGS.delete.key;

    #[test]
    fn test_album_delete() {
      let mut app = App::test_default();
      app
        .data
        .lidarr_data
        .albums
        .set_items(vec![Album::default()]);
      app.push_navigation_stack(ActiveLidarrBlock::ArtistDetails.into());

      assert_delete_prompt!(
        ArtistDetailsHandler,
        app,
        ActiveLidarrBlock::ArtistDetails,
        ActiveLidarrBlock::DeleteAlbumPrompt
      );
      assert_eq!(
        app.data.lidarr_data.selected_block.blocks,
        DELETE_ALBUM_SELECTION_BLOCKS
      );
    }
  }

  mod test_handle_left_right_action {
    use rstest::rstest;

    use crate::app::App;
    use crate::app::key_binding::DEFAULT_KEYBINDINGS;
    use crate::assert_navigation_pushed;
    use crate::event::Key;
    use crate::handlers::KeyEventHandler;
    use crate::handlers::lidarr_handlers::library::artist_details_handler::ArtistDetailsHandler;
    use crate::models::servarr_data::lidarr::lidarr_data::ActiveLidarrBlock;

    #[rstest]
    fn test_left_right_prompt_toggle(
      #[values(
        ActiveLidarrBlock::UpdateAndScanArtistPrompt,
        ActiveLidarrBlock::AutomaticallySearchArtistPrompt,
        ActiveLidarrBlock::ManualArtistSearchConfirmPrompt
      )]
      active_lidarr_block: ActiveLidarrBlock,
      #[values(Key::Left, Key::Right)] key: Key,
    ) {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveLidarrBlock::ArtistDetails.into());
      app.push_navigation_stack(active_lidarr_block.into());

      ArtistDetailsHandler::new(key, &mut app, active_lidarr_block, None).handle();

      assert!(app.data.lidarr_data.prompt_confirm);

      ArtistDetailsHandler::new(key, &mut app, active_lidarr_block, None).handle();

      assert!(!app.data.lidarr_data.prompt_confirm);
    }

    #[rstest]
    #[case(ActiveLidarrBlock::ArtistDetails, ActiveLidarrBlock::ArtistHistory)]
    #[case(
      ActiveLidarrBlock::ArtistHistory,
      ActiveLidarrBlock::ManualArtistSearch
    )]
    #[case(
      ActiveLidarrBlock::ManualArtistSearch,
      ActiveLidarrBlock::ArtistDetails
    )]
    fn test_artist_details_tabs_left_right_action(
      #[case] left_block: ActiveLidarrBlock,
      #[case] right_block: ActiveLidarrBlock,
      #[values(true, false)] is_loading: bool,
    ) {
      let mut app = App::test_default_fully_populated();
      app.is_loading = is_loading;
      app.push_navigation_stack(right_block.into());
      app.data.lidarr_data.artist_info_tabs.index = app
        .data
        .lidarr_data
        .artist_info_tabs
        .tabs
        .iter()
        .position(|tab_route| tab_route.route == right_block.into())
        .unwrap_or_default();

      ArtistDetailsHandler::new(DEFAULT_KEYBINDINGS.left.key, &mut app, right_block, None).handle();

      assert_eq!(
        app.get_current_route(),
        app.data.lidarr_data.artist_info_tabs.get_active_route()
      );
      assert_navigation_pushed!(app, left_block.into());

      ArtistDetailsHandler::new(DEFAULT_KEYBINDINGS.right.key, &mut app, left_block, None).handle();

      assert_eq!(
        app.get_current_route(),
        app.data.lidarr_data.artist_info_tabs.get_active_route()
      );
      assert_navigation_pushed!(app, right_block.into());
    }
  }

  mod test_handle_submit {
    use crate::app::App;
    use crate::app::key_binding::DEFAULT_KEYBINDINGS;
    use crate::event::Key;
    use crate::handlers::KeyEventHandler;
    use crate::handlers::lidarr_handlers::library::artist_details_handler::ArtistDetailsHandler;
    use crate::models::lidarr_models::{LidarrHistoryItem, LidarrReleaseDownloadBody};
    use crate::models::servarr_data::lidarr::lidarr_data::ActiveLidarrBlock;
    use crate::network::lidarr_network::LidarrEvent;
    use crate::network::lidarr_network::lidarr_network_test_utils::test_utils::{
      artist, torrent_release,
    };
    use crate::{assert_navigation_popped, assert_navigation_pushed};
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    const SUBMIT_KEY: Key = DEFAULT_KEYBINDINGS.submit.key;

    #[rstest]
    #[case(
      ActiveLidarrBlock::AutomaticallySearchArtistPrompt,
      LidarrEvent::TriggerAutomaticArtistSearch(1)
    )]
    #[case(
      ActiveLidarrBlock::UpdateAndScanArtistPrompt,
      LidarrEvent::UpdateAndScanArtist(1)
    )]
    fn test_artist_details_prompt_confirm_submit(
      #[case] prompt_block: ActiveLidarrBlock,
      #[case] expected_action: LidarrEvent,
    ) {
      let mut app = App::test_default();
      app.data.lidarr_data.prompt_confirm = true;
      app.data.lidarr_data.artists.set_items(vec![artist()]);
      app.push_navigation_stack(ActiveLidarrBlock::ArtistDetails.into());
      app.push_navigation_stack(prompt_block.into());

      ArtistDetailsHandler::new(SUBMIT_KEY, &mut app, prompt_block, None).handle();

      assert!(app.data.lidarr_data.prompt_confirm);
      assert_navigation_popped!(app, ActiveLidarrBlock::ArtistDetails.into());
      assert_some_eq_x!(
        &app.data.lidarr_data.prompt_confirm_action,
        &expected_action
      );
    }

    #[rstest]
    fn test_artist_details_prompt_decline_submit(
      #[values(
        ActiveLidarrBlock::AutomaticallySearchArtistPrompt,
        ActiveLidarrBlock::UpdateAndScanArtistPrompt
      )]
      prompt_block: ActiveLidarrBlock,
    ) {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveLidarrBlock::ArtistDetails.into());
      app.push_navigation_stack(prompt_block.into());

      ArtistDetailsHandler::new(SUBMIT_KEY, &mut app, prompt_block, None).handle();

      assert!(!app.data.lidarr_data.prompt_confirm);
      assert_navigation_popped!(app, ActiveLidarrBlock::ArtistDetails.into());
      assert_none!(app.data.lidarr_data.prompt_confirm_action);
    }

    #[test]
    fn test_artist_history_submit() {
      let mut app = App::test_default();
      app
        .data
        .lidarr_data
        .artist_history
        .set_items(vec![LidarrHistoryItem::default()]);

      ArtistDetailsHandler::new(SUBMIT_KEY, &mut app, ActiveLidarrBlock::ArtistHistory, None)
        .handle();

      assert_navigation_pushed!(app, ActiveLidarrBlock::ArtistHistoryDetails.into());
    }

    #[test]
    fn test_artist_history_submit_no_op_when_artist_history_is_empty() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveLidarrBlock::ArtistHistory.into());

      ArtistDetailsHandler::new(SUBMIT_KEY, &mut app, ActiveLidarrBlock::ArtistHistory, None)
        .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveLidarrBlock::ArtistHistory.into()
      );
    }

    #[test]
    fn test_artist_history_submit_no_op_when_not_ready() {
      let mut app = App::test_default();
      app.is_loading = true;
      app.push_navigation_stack(ActiveLidarrBlock::Artists.into());

      ArtistDetailsHandler::new(SUBMIT_KEY, &mut app, ActiveLidarrBlock::ArtistHistory, None)
        .handle();

      assert_eq!(app.get_current_route(), ActiveLidarrBlock::Artists.into());
    }

    #[test]
    fn test_manual_artist_search_submit() {
      let mut app = App::test_default();
      app
        .data
        .lidarr_data
        .discography_releases
        .set_items(vec![torrent_release()]);
      app.push_navigation_stack(ActiveLidarrBlock::ManualArtistSearch.into());

      ArtistDetailsHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveLidarrBlock::ManualArtistSearch,
        None,
      )
      .handle();

      assert_navigation_pushed!(
        app,
        ActiveLidarrBlock::ManualArtistSearchConfirmPrompt.into()
      );
    }

    #[test]
    fn test_manual_artist_search_submit_no_op_when_not_ready() {
      let mut app = App::test_default();
      app.is_loading = true;
      app.push_navigation_stack(ActiveLidarrBlock::ManualArtistSearch.into());

      ArtistDetailsHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveLidarrBlock::ManualArtistSearch,
        None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveLidarrBlock::ManualArtistSearch.into()
      );
    }

    #[test]
    fn test_manual_artist_search_confirm_prompt_confirm_submit() {
      let mut app = App::test_default();
      let release = torrent_release();
      app
        .data
        .lidarr_data
        .discography_releases
        .set_items(vec![release.clone()]);
      app.data.lidarr_data.prompt_confirm = true;
      app.push_navigation_stack(ActiveLidarrBlock::ManualArtistSearch.into());
      app.push_navigation_stack(ActiveLidarrBlock::ManualArtistSearchConfirmPrompt.into());

      ArtistDetailsHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveLidarrBlock::ManualArtistSearchConfirmPrompt,
        None,
      )
      .handle();

      assert!(app.data.lidarr_data.prompt_confirm);
      assert_navigation_popped!(app, ActiveLidarrBlock::ManualArtistSearch.into());
      assert_eq!(
        app.data.lidarr_data.prompt_confirm_action,
        Some(LidarrEvent::DownloadRelease(LidarrReleaseDownloadBody {
          guid: release.guid,
          indexer_id: release.indexer_id,
        }))
      );
    }

    #[test]
    fn test_manual_artist_search_confirm_prompt_decline_submit() {
      let mut app = App::test_default();
      app
        .data
        .lidarr_data
        .discography_releases
        .set_items(vec![torrent_release()]);
      app.push_navigation_stack(ActiveLidarrBlock::ManualArtistSearch.into());
      app.push_navigation_stack(ActiveLidarrBlock::ManualArtistSearchConfirmPrompt.into());

      ArtistDetailsHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveLidarrBlock::ManualArtistSearchConfirmPrompt,
        None,
      )
      .handle();

      assert!(!app.data.lidarr_data.prompt_confirm);
      assert_navigation_popped!(app, ActiveLidarrBlock::ManualArtistSearch.into());
      assert_none!(app.data.lidarr_data.prompt_confirm_action);
    }
  }

  mod test_handle_esc {
    use crate::app::App;
    use crate::app::key_binding::DEFAULT_KEYBINDINGS;
    use crate::assert_navigation_popped;
    use crate::event::Key;
    use crate::handlers::KeyEventHandler;
    use crate::handlers::lidarr_handlers::library::artist_details_handler::ArtistDetailsHandler;
    use crate::models::lidarr_models::LidarrHistoryItem;
    use crate::models::servarr_data::lidarr::lidarr_data::ActiveLidarrBlock;
    use crate::models::stateful_table::StatefulTable;
    use pretty_assertions::assert_eq;
    use ratatui::widgets::TableState;
    use rstest::rstest;

    const ESC_KEY: Key = DEFAULT_KEYBINDINGS.esc.key;

    #[test]
    fn test_artist_history_details_block_esc() {
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack(ActiveLidarrBlock::ArtistHistory.into());
      app.push_navigation_stack(ActiveLidarrBlock::ArtistHistoryDetails.into());

      ArtistDetailsHandler::new(
        ESC_KEY,
        &mut app,
        ActiveLidarrBlock::ArtistHistoryDetails,
        None,
      )
      .handle();

      assert_navigation_popped!(app, ActiveLidarrBlock::ArtistHistory.into());
    }

    #[test]
    fn test_artist_history_esc_resets_filter_if_one_is_set_instead_of_closing_the_window() {
      let mut app = App::test_default();
      app.data.lidarr_data.artist_history = StatefulTable {
        filter: Some("Test".into()),
        filtered_items: Some(vec![LidarrHistoryItem::default()]),
        filtered_state: Some(TableState::default()),
        ..StatefulTable::default()
      };
      app.push_navigation_stack(ActiveLidarrBlock::Artists.into());
      app.push_navigation_stack(ActiveLidarrBlock::ArtistHistory.into());

      ArtistDetailsHandler::new(ESC_KEY, &mut app, ActiveLidarrBlock::ArtistHistory, None).handle();

      assert_eq!(
        app.get_current_route(),
        ActiveLidarrBlock::ArtistHistory.into()
      );
      assert_none!(app.data.lidarr_data.artist_history.filter);
      assert_none!(app.data.lidarr_data.artist_history.filtered_items);
      assert_none!(app.data.lidarr_data.artist_history.filtered_state);
    }

    #[rstest]
    fn test_artist_details_esc(
      #[values(
        ActiveLidarrBlock::AutomaticallySearchArtistPrompt,
        ActiveLidarrBlock::UpdateAndScanArtistPrompt,
        ActiveLidarrBlock::ManualArtistSearchConfirmPrompt
      )]
      prompt_block: ActiveLidarrBlock,
      #[values(true, false)] is_ready: bool,
    ) {
      let mut app = App::test_default();
      app.is_loading = is_ready;
      app.data.lidarr_data.prompt_confirm = true;
      app.push_navigation_stack(ActiveLidarrBlock::ArtistDetails.into());
      app.push_navigation_stack(prompt_block.into());

      ArtistDetailsHandler::new(ESC_KEY, &mut app, prompt_block, None).handle();

      assert!(!app.data.lidarr_data.prompt_confirm);
      assert_navigation_popped!(app, ActiveLidarrBlock::ArtistDetails.into());
    }

    #[rstest]
    fn test_artist_details_blocks_esc(
      #[values(
        ActiveLidarrBlock::ArtistDetails,
        ActiveLidarrBlock::ArtistHistory,
        ActiveLidarrBlock::ManualArtistSearch
      )]
      active_lidarr_block: ActiveLidarrBlock,
    ) {
      let mut app = App::test_default();
      app.data.lidarr_data.artist_history.filter = None;
      app.data.lidarr_data.artist_history.filtered_items = None;
      app.data.lidarr_data.artist_history.filtered_state = None;
      app.push_navigation_stack(ActiveLidarrBlock::Artists.into());
      app.push_navigation_stack(active_lidarr_block.into());

      ArtistDetailsHandler::new(ESC_KEY, &mut app, active_lidarr_block, None).handle();

      assert_navigation_popped!(app, ActiveLidarrBlock::Artists.into());
      assert_is_empty!(app.data.lidarr_data.albums);
      assert_is_empty!(app.data.lidarr_data.discography_releases);
      assert_is_empty!(app.data.lidarr_data.artist_history);
      assert_eq!(app.data.lidarr_data.artist_info_tabs.index, 0);
    }
  }

  mod test_handle_char_key_event {
    use crate::app::App;
    use crate::app::key_binding::DEFAULT_KEYBINDINGS;
    use crate::assert_navigation_pushed;
    use crate::handlers::KeyEventHandler;
    use crate::handlers::lidarr_handlers::library::artist_details_handler::ArtistDetailsHandler;
    use crate::models::lidarr_models::{Artist, LidarrReleaseDownloadBody};
    use crate::models::servarr_data::lidarr::lidarr_data::{
      ActiveLidarrBlock, EDIT_ARTIST_SELECTION_BLOCKS,
    };
    use crate::network::lidarr_network::LidarrEvent;
    use crate::network::lidarr_network::lidarr_network_test_utils::test_utils::torrent_release;
    use crate::{assert_modal_absent, assert_modal_present, assert_navigation_popped};
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    #[rstest]
    fn test_artist_details_edit_key(
      #[values(
        ActiveLidarrBlock::ArtistDetails,
        ActiveLidarrBlock::ArtistHistory,
        ActiveLidarrBlock::ManualArtistSearch
      )]
      active_lidarr_block: ActiveLidarrBlock,
    ) {
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack(ActiveLidarrBlock::ArtistDetails.into());
      app.push_navigation_stack(active_lidarr_block.into());

      ArtistDetailsHandler::new(
        DEFAULT_KEYBINDINGS.edit.key,
        &mut app,
        active_lidarr_block,
        None,
      )
      .handle();

      assert_navigation_pushed!(
        app,
        (
          ActiveLidarrBlock::EditArtistPrompt,
          Some(active_lidarr_block)
        )
          .into()
      );
      assert_modal_present!(app.data.lidarr_data.edit_artist_modal);
      assert_some!(app.data.lidarr_data.edit_artist_modal);
      assert_eq!(
        app.data.lidarr_data.selected_block.blocks,
        EDIT_ARTIST_SELECTION_BLOCKS
      );
    }

    #[rstest]
    fn test_artist_details_edit_key_no_op_when_not_ready(
      #[values(
        ActiveLidarrBlock::ArtistDetails,
        ActiveLidarrBlock::ArtistHistory,
        ActiveLidarrBlock::ManualArtistSearch
      )]
      active_lidarr_block: ActiveLidarrBlock,
    ) {
      let mut app = App::test_default();
      app.is_loading = true;
      app.push_navigation_stack(ActiveLidarrBlock::ArtistDetails.into());
      app.push_navigation_stack(active_lidarr_block.into());

      ArtistDetailsHandler::new(
        DEFAULT_KEYBINDINGS.edit.key,
        &mut app,
        active_lidarr_block,
        None,
      )
      .handle();

      assert_eq!(app.get_current_route(), active_lidarr_block.into());
      assert_modal_absent!(app.data.lidarr_data.edit_artist_modal);
    }

    #[test]
    fn test_artist_details_toggle_monitoring_key() {
      let mut app = App::test_default_fully_populated();
      app.is_routing = false;
      app.push_navigation_stack(ActiveLidarrBlock::Artists.into());
      app.push_navigation_stack(ActiveLidarrBlock::ArtistDetails.into());

      ArtistDetailsHandler::new(
        DEFAULT_KEYBINDINGS.toggle_monitoring.key,
        &mut app,
        ActiveLidarrBlock::ArtistDetails,
        None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveLidarrBlock::ArtistDetails.into()
      );
      assert!(app.data.lidarr_data.prompt_confirm);
      assert!(app.is_routing);
      assert_eq!(
        app.data.lidarr_data.prompt_confirm_action,
        Some(LidarrEvent::ToggleAlbumMonitoring(1))
      );
    }

    #[test]
    fn test_artist_details_toggle_monitoring_key_no_op_when_not_ready() {
      let mut app = App::test_default();
      app.is_loading = true;
      app.data.lidarr_data.prompt_confirm = false;
      app.push_navigation_stack(ActiveLidarrBlock::ArtistDetails.into());

      ArtistDetailsHandler::new(
        DEFAULT_KEYBINDINGS.toggle_monitoring.key,
        &mut app,
        ActiveLidarrBlock::ArtistDetails,
        None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveLidarrBlock::ArtistDetails.into()
      );
      assert!(!app.data.lidarr_data.prompt_confirm);
      assert_none!(app.data.lidarr_data.prompt_confirm_action);
    }

    #[test]
    fn test_artist_details_toggle_monitoring_key_no_op_when_albums_empty() {
      let mut app = App::test_default();
      app.data.lidarr_data.artists.set_items(vec![Artist {
        id: 1,
        ..Artist::default()
      }]);
      app.push_navigation_stack(ActiveLidarrBlock::ArtistDetails.into());

      ArtistDetailsHandler::new(
        DEFAULT_KEYBINDINGS.toggle_monitoring.key,
        &mut app,
        ActiveLidarrBlock::ArtistDetails,
        None,
      )
      .handle();

      assert!(!app.data.lidarr_data.prompt_confirm);
      assert!(app.data.lidarr_data.prompt_confirm_action.is_none());
    }

    #[rstest]
    fn test_artist_details_auto_search_key(
      #[values(
        ActiveLidarrBlock::ArtistDetails,
        ActiveLidarrBlock::ArtistHistory,
        ActiveLidarrBlock::ManualArtistSearch
      )]
      active_lidarr_block: ActiveLidarrBlock,
    ) {
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack(active_lidarr_block.into());

      ArtistDetailsHandler::new(
        DEFAULT_KEYBINDINGS.auto_search.key,
        &mut app,
        active_lidarr_block,
        None,
      )
      .handle();

      assert_navigation_pushed!(
        app,
        ActiveLidarrBlock::AutomaticallySearchArtistPrompt.into()
      );
    }

    #[rstest]
    fn test_artist_details_auto_search_key_no_op_when_not_ready(
      #[values(
        ActiveLidarrBlock::ArtistDetails,
        ActiveLidarrBlock::ArtistHistory,
        ActiveLidarrBlock::ManualArtistSearch
      )]
      active_lidarr_block: ActiveLidarrBlock,
    ) {
      let mut app = App::test_default_fully_populated();
      app.is_loading = true;
      app.push_navigation_stack(active_lidarr_block.into());

      ArtistDetailsHandler::new(
        DEFAULT_KEYBINDINGS.auto_search.key,
        &mut app,
        active_lidarr_block,
        None,
      )
      .handle();

      assert_eq!(app.get_current_route(), active_lidarr_block.into());
    }

    #[rstest]
    fn test_artist_details_update_key(
      #[values(
        ActiveLidarrBlock::ArtistDetails,
        ActiveLidarrBlock::ArtistHistory,
        ActiveLidarrBlock::ManualArtistSearch
      )]
      active_lidarr_block: ActiveLidarrBlock,
    ) {
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack(active_lidarr_block.into());

      ArtistDetailsHandler::new(
        DEFAULT_KEYBINDINGS.update.key,
        &mut app,
        active_lidarr_block,
        None,
      )
      .handle();

      assert_navigation_pushed!(app, ActiveLidarrBlock::UpdateAndScanArtistPrompt.into());
    }

    #[rstest]
    fn test_artist_details_update_key_no_op_when_not_ready(
      #[values(
        ActiveLidarrBlock::ArtistDetails,
        ActiveLidarrBlock::ArtistHistory,
        ActiveLidarrBlock::ManualArtistSearch
      )]
      active_lidarr_block: ActiveLidarrBlock,
    ) {
      let mut app = App::test_default_fully_populated();
      app.is_loading = true;
      app.push_navigation_stack(active_lidarr_block.into());

      ArtistDetailsHandler::new(
        DEFAULT_KEYBINDINGS.update.key,
        &mut app,
        active_lidarr_block,
        None,
      )
      .handle();

      assert_eq!(app.get_current_route(), active_lidarr_block.into());
    }

    #[rstest]
    fn test_artist_details_refresh_key(
      #[values(
        ActiveLidarrBlock::ArtistDetails,
        ActiveLidarrBlock::ArtistHistory,
        ActiveLidarrBlock::ManualArtistSearch
      )]
      active_lidarr_block: ActiveLidarrBlock,
    ) {
      let mut app = App::test_default_fully_populated();
      app.is_routing = false;
      app.push_navigation_stack(ActiveLidarrBlock::Artists.into());
      app.push_navigation_stack(active_lidarr_block.into());

      ArtistDetailsHandler::new(
        DEFAULT_KEYBINDINGS.refresh.key,
        &mut app,
        active_lidarr_block,
        None,
      )
      .handle();

      assert_navigation_pushed!(app, active_lidarr_block.into());
      assert!(app.is_routing);
    }

    #[rstest]
    fn test_artist_details_refresh_key_no_op_when_not_ready(
      #[values(
        ActiveLidarrBlock::ArtistDetails,
        ActiveLidarrBlock::ArtistHistory,
        ActiveLidarrBlock::ManualArtistSearch
      )]
      active_lidarr_block: ActiveLidarrBlock,
    ) {
      let mut app = App::test_default();
      app.is_loading = true;
      app.push_navigation_stack(active_lidarr_block.into());
      app.is_routing = false;

      ArtistDetailsHandler::new(
        DEFAULT_KEYBINDINGS.refresh.key,
        &mut app,
        active_lidarr_block,
        None,
      )
      .handle();

      assert_eq!(app.get_current_route(), active_lidarr_block.into());
      assert!(!app.is_routing);
    }

    #[rstest]
    #[case(
      ActiveLidarrBlock::AutomaticallySearchArtistPrompt,
      LidarrEvent::TriggerAutomaticArtistSearch(1)
    )]
    #[case(
      ActiveLidarrBlock::UpdateAndScanArtistPrompt,
      LidarrEvent::UpdateAndScanArtist(1)
    )]
    fn test_artist_details_prompt_confirm_key(
      #[case] prompt_block: ActiveLidarrBlock,
      #[case] expected_action: LidarrEvent,
      #[values(ActiveLidarrBlock::ArtistDetails, ActiveLidarrBlock::ArtistHistory)]
      active_lidarr_block: ActiveLidarrBlock,
    ) {
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack(active_lidarr_block.into());
      app.push_navigation_stack(prompt_block.into());

      ArtistDetailsHandler::new(
        DEFAULT_KEYBINDINGS.confirm.key,
        &mut app,
        prompt_block,
        None,
      )
      .handle();

      assert!(app.data.lidarr_data.prompt_confirm);
      assert_navigation_popped!(app, active_lidarr_block.into());
      assert_some_eq_x!(
        &app.data.lidarr_data.prompt_confirm_action,
        &expected_action
      );
    }

    #[test]
    fn test_manual_artist_search_confirm_prompt_confirm_key() {
      let mut app = App::test_default();
      let release = torrent_release();
      app
        .data
        .lidarr_data
        .discography_releases
        .set_items(vec![release.clone()]);
      app.push_navigation_stack(ActiveLidarrBlock::ManualArtistSearch.into());
      app.push_navigation_stack(ActiveLidarrBlock::ManualArtistSearchConfirmPrompt.into());

      ArtistDetailsHandler::new(
        DEFAULT_KEYBINDINGS.confirm.key,
        &mut app,
        ActiveLidarrBlock::ManualArtistSearchConfirmPrompt,
        None,
      )
      .handle();

      assert!(app.data.lidarr_data.prompt_confirm);
      assert_navigation_popped!(app, ActiveLidarrBlock::ManualArtistSearch.into());
      assert_eq!(
        app.data.lidarr_data.prompt_confirm_action,
        Some(LidarrEvent::DownloadRelease(LidarrReleaseDownloadBody {
          guid: release.guid,
          indexer_id: release.indexer_id,
        }))
      );
    }
  }

  #[test]
  fn test_artist_details_handler_accepts() {
    let mut artist_details_blocks = ARTIST_DETAILS_BLOCKS.clone().to_vec();
    artist_details_blocks.extend(DELETE_ALBUM_BLOCKS);

    ActiveLidarrBlock::iter().for_each(|active_lidarr_block| {
      if artist_details_blocks.contains(&active_lidarr_block) {
        assert!(ArtistDetailsHandler::accepts(active_lidarr_block));
      } else {
        assert!(!ArtistDetailsHandler::accepts(active_lidarr_block));
      }
    });
  }

  #[test]
  fn test_extract_artist_id() {
    let mut app = App::test_default_fully_populated();

    let artist_id = ArtistDetailsHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveLidarrBlock::ArtistDetails,
      None,
    )
    .extract_artist_id();

    assert_eq!(artist_id, 1);
  }

  #[test]
  fn test_extract_album_id() {
    let mut app = App::test_default_fully_populated();

    let album_id = ArtistDetailsHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveLidarrBlock::ArtistDetails,
      None,
    )
    .extract_album_id();

    assert_eq!(album_id, 1);
  }

  #[rstest]
  fn test_artist_details_handler_ignore_special_keys(
    #[values(true, false)] ignore_special_keys_for_textbox_input: bool,
  ) {
    let mut app = App::test_default();
    app.ignore_special_keys_for_textbox_input = ignore_special_keys_for_textbox_input;

    let handler = ArtistDetailsHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveLidarrBlock::ArtistDetails,
      None,
    );

    assert_eq!(
      handler.ignore_special_keys(),
      ignore_special_keys_for_textbox_input
    );
  }

  #[test]
  fn test_artist_details_handler_is_not_ready_when_loading() {
    let mut app = App::test_default();
    app.push_navigation_stack(ActiveLidarrBlock::ArtistDetails.into());
    app.is_loading = true;

    let handler = ArtistDetailsHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveLidarrBlock::ArtistDetails,
      None,
    );

    assert!(!handler.is_ready());
  }

  #[test]
  fn test_artist_details_handler_is_ready_when_not_loading() {
    let mut app = App::test_default();
    app.push_navigation_stack(ActiveLidarrBlock::ArtistDetails.into());
    app.is_loading = false;

    let handler = ArtistDetailsHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveLidarrBlock::ArtistDetails,
      None,
    );

    assert!(handler.is_ready());
  }

  #[test]
  fn test_artist_details_handler_is_not_ready_when_not_loading_and_artist_history_is_none() {
    let mut app = App::test_default();
    app.push_navigation_stack(ActiveLidarrBlock::Artists.into());

    let handler = ArtistDetailsHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveLidarrBlock::ArtistHistory,
      None,
    );

    assert!(!handler.is_ready());
  }

  #[test]
  fn test_artist_details_handler_ready_when_not_loading_and_artist_history_is_non_empty() {
    let mut app = App::test_default();
    app.push_navigation_stack(ActiveLidarrBlock::Artists.into());
    app
      .data
      .lidarr_data
      .artist_history
      .set_items(vec![LidarrHistoryItem::default()]);

    let handler = ArtistDetailsHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveLidarrBlock::ArtistHistory,
      None,
    );

    assert!(handler.is_ready());
  }

  #[test]
  fn test_artist_details_handler_is_not_ready_when_not_loading_and_discography_releases_is_empty() {
    let mut app = App::test_default();
    app.push_navigation_stack(ActiveLidarrBlock::Artists.into());

    let handler = ArtistDetailsHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveLidarrBlock::ManualArtistSearch,
      None,
    );

    assert!(!handler.is_ready());
  }

  #[test]
  fn test_artist_details_handler_ready_when_not_loading_and_discography_releases_is_non_empty() {
    let mut app = App::test_default();
    app.push_navigation_stack(ActiveLidarrBlock::Artists.into());
    app
      .data
      .lidarr_data
      .discography_releases
      .set_items(vec![LidarrRelease::default()]);

    let handler = ArtistDetailsHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveLidarrBlock::ManualArtistSearch,
      None,
    );

    assert!(handler.is_ready());
  }

  #[test]
  fn test_delegates_delete_album_blocks_to_delete_album_handler() {
    let mut app = App::test_default();
    app
      .data
      .lidarr_data
      .albums
      .set_items(vec![Album::default()]);
    app.push_navigation_stack(ActiveLidarrBlock::ArtistDetails.into());
    app.push_navigation_stack(ActiveLidarrBlock::DeleteAlbumPrompt.into());

    ArtistDetailsHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveLidarrBlock::DeleteAlbumPrompt,
      None,
    )
    .handle();

    assert_eq!(
      app.get_current_route(),
      ActiveLidarrBlock::ArtistDetails.into()
    );
  }

  #[test]
  fn test_releases_sorting_options_source() {
    let expected_cmp_fn: fn(&LidarrRelease, &LidarrRelease) -> Ordering =
      |a, b| a.protocol.cmp(&b.protocol);
    let mut expected_releases_vec = release_vec();
    expected_releases_vec.sort_by(expected_cmp_fn);

    let sort_option = releases_sorting_options()[0].clone();
    let mut sorted_releases_vec = release_vec();
    sorted_releases_vec.sort_by(sort_option.cmp_fn.unwrap());

    assert_eq!(sorted_releases_vec, expected_releases_vec);
    assert_str_eq!(sort_option.name, "Source");
  }

  #[test]
  fn test_releases_sorting_options_age() {
    let expected_cmp_fn: fn(&LidarrRelease, &LidarrRelease) -> Ordering = |a, b| a.age.cmp(&b.age);
    let mut expected_releases_vec = release_vec();
    expected_releases_vec.sort_by(expected_cmp_fn);

    let sort_option = releases_sorting_options()[1].clone();
    let mut sorted_releases_vec = release_vec();
    sorted_releases_vec.sort_by(sort_option.cmp_fn.unwrap());

    assert_eq!(sorted_releases_vec, expected_releases_vec);
    assert_str_eq!(sort_option.name, "Age");
  }

  #[test]
  fn test_releases_sorting_options_rejected() {
    let expected_cmp_fn: fn(&LidarrRelease, &LidarrRelease) -> Ordering =
      |a, b| a.rejected.cmp(&b.rejected);
    let mut expected_releases_vec = release_vec();
    expected_releases_vec.sort_by(expected_cmp_fn);

    let sort_option = releases_sorting_options()[2].clone();
    let mut sorted_releases_vec = release_vec();
    sorted_releases_vec.sort_by(sort_option.cmp_fn.unwrap());

    assert_eq!(sorted_releases_vec, expected_releases_vec);
    assert_str_eq!(sort_option.name, "Rejected");
  }

  #[test]
  fn test_releases_sorting_options_title() {
    let expected_cmp_fn: fn(&LidarrRelease, &LidarrRelease) -> Ordering = |a, b| {
      a.title
        .text
        .to_lowercase()
        .cmp(&b.title.text.to_lowercase())
    };
    let mut expected_releases_vec = release_vec();
    expected_releases_vec.sort_by(expected_cmp_fn);

    let sort_option = releases_sorting_options()[3].clone();
    let mut sorted_releases_vec = release_vec();
    sorted_releases_vec.sort_by(sort_option.cmp_fn.unwrap());

    assert_eq!(sorted_releases_vec, expected_releases_vec);
    assert_str_eq!(sort_option.name, "Title");
  }

  #[test]
  fn test_releases_sorting_options_indexer() {
    let expected_cmp_fn: fn(&LidarrRelease, &LidarrRelease) -> Ordering =
      |a, b| a.indexer.to_lowercase().cmp(&b.indexer.to_lowercase());
    let mut expected_releases_vec = release_vec();
    expected_releases_vec.sort_by(expected_cmp_fn);

    let sort_option = releases_sorting_options()[4].clone();
    let mut sorted_releases_vec = release_vec();
    sorted_releases_vec.sort_by(sort_option.cmp_fn.unwrap());

    assert_eq!(sorted_releases_vec, expected_releases_vec);
    assert_str_eq!(sort_option.name, "Indexer");
  }

  #[test]
  fn test_releases_sorting_options_size() {
    let expected_cmp_fn: fn(&LidarrRelease, &LidarrRelease) -> Ordering =
      |a, b| a.size.cmp(&b.size);
    let mut expected_releases_vec = release_vec();
    expected_releases_vec.sort_by(expected_cmp_fn);

    let sort_option = releases_sorting_options()[5].clone();
    let mut sorted_releases_vec = release_vec();
    sorted_releases_vec.sort_by(sort_option.cmp_fn.unwrap());

    assert_eq!(sorted_releases_vec, expected_releases_vec);
    assert_str_eq!(sort_option.name, "Size");
  }

  #[test]
  fn test_releases_sorting_options_peers() {
    let expected_cmp_fn: fn(&LidarrRelease, &LidarrRelease) -> Ordering = |a, b| {
      let default_number = Number::from(i64::MAX);
      let seeder_a = a
        .seeders
        .as_ref()
        .unwrap_or(&default_number)
        .as_u64()
        .unwrap();
      let seeder_b = b
        .seeders
        .as_ref()
        .unwrap_or(&default_number)
        .as_u64()
        .unwrap();

      seeder_a.cmp(&seeder_b)
    };
    let mut expected_releases_vec = release_vec();
    expected_releases_vec.sort_by(expected_cmp_fn);

    let sort_option = releases_sorting_options()[6].clone();
    let mut sorted_releases_vec = release_vec();
    sorted_releases_vec.sort_by(sort_option.cmp_fn.unwrap());

    assert_eq!(sorted_releases_vec, expected_releases_vec);
    assert_str_eq!(sort_option.name, "Peers");
  }

  #[test]
  fn test_releases_sorting_options_quality() {
    let expected_cmp_fn: fn(&LidarrRelease, &LidarrRelease) -> Ordering =
      |a, b| a.quality.cmp(&b.quality);
    let mut expected_releases_vec = release_vec();
    expected_releases_vec.sort_by(expected_cmp_fn);

    let sort_option = releases_sorting_options()[7].clone();
    let mut sorted_releases_vec = release_vec();
    sorted_releases_vec.sort_by(sort_option.cmp_fn.unwrap());

    assert_eq!(sorted_releases_vec, expected_releases_vec);
    assert_str_eq!(sort_option.name, "Quality");
  }

  fn release_vec() -> Vec<LidarrRelease> {
    let release_a = LidarrRelease {
      protocol: "Protocol A".to_owned(),
      age: 1,
      title: HorizontallyScrollableText::from("Title A"),
      indexer: "Indexer A".to_owned(),
      size: 1,
      rejected: true,
      seeders: Some(Number::from(1)),
      quality: QualityWrapper {
        quality: Quality {
          name: "Quality A".to_owned(),
        },
      },
      ..LidarrRelease::default()
    };
    let release_b = LidarrRelease {
      protocol: "Protocol B".to_owned(),
      age: 2,
      title: HorizontallyScrollableText::from("title B"),
      indexer: "indexer B".to_owned(),
      size: 2,
      rejected: false,
      seeders: Some(Number::from(2)),
      quality: QualityWrapper {
        quality: Quality {
          name: "Quality B".to_owned(),
        },
      },
      ..LidarrRelease::default()
    };
    let release_c = LidarrRelease {
      protocol: "Protocol C".to_owned(),
      age: 3,
      title: HorizontallyScrollableText::from("Title C"),
      indexer: "Indexer C".to_owned(),
      size: 3,
      rejected: false,
      seeders: None,
      quality: QualityWrapper {
        quality: Quality {
          name: "Quality C".to_owned(),
        },
      },
      ..LidarrRelease::default()
    };

    vec![release_a, release_b, release_c]
  }
}
