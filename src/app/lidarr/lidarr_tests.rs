#[cfg(test)]
mod tests {
  use crate::app::App;
  use crate::models::lidarr_models::{Album, Artist, LidarrRelease};
  use crate::models::servarr_data::lidarr::lidarr_data::ActiveLidarrBlock;
  use crate::models::servarr_data::lidarr::modals::AlbumDetailsModal;
  use crate::models::servarr_models::Indexer;
  use crate::network::NetworkEvent;
  use crate::network::lidarr_network::LidarrEvent;
  use crate::network::lidarr_network::lidarr_network_test_utils::test_utils::artist;
  use pretty_assertions::{assert_eq, assert_str_eq};
  use tokio::sync::mpsc;

  #[tokio::test]
  async fn test_dispatch_by_lidarr_block_artists() {
    let (tx, mut rx) = mpsc::channel::<NetworkEvent>(500);
    let mut app = App::test_default();
    app.data.lidarr_data.prompt_confirm = true;
    app.network_tx = Some(tx);

    app
      .dispatch_by_lidarr_block(&ActiveLidarrBlock::Artists)
      .await;

    assert!(app.is_loading);
    assert_eq!(
      rx.recv().await.unwrap(),
      LidarrEvent::GetQualityProfiles.into()
    );
    assert_eq!(
      rx.recv().await.unwrap(),
      LidarrEvent::GetMetadataProfiles.into()
    );
    assert_eq!(rx.recv().await.unwrap(), LidarrEvent::GetTags.into());
    assert_eq!(rx.recv().await.unwrap(), LidarrEvent::ListArtists.into());
    assert!(!app.data.lidarr_data.prompt_confirm);
    assert_eq!(app.tick_count, 0);
  }

  #[tokio::test]
  async fn test_dispatch_by_lidarr_block_artist_details() {
    let (tx, mut rx) = mpsc::channel::<NetworkEvent>(500);
    let mut app = App::test_default();
    app.data.lidarr_data.prompt_confirm = true;
    app.data.lidarr_data.artists.set_items(vec![artist()]);
    app.network_tx = Some(tx);

    app
      .dispatch_by_lidarr_block(&ActiveLidarrBlock::ArtistDetails)
      .await;

    assert!(app.is_loading);
    assert_eq!(rx.recv().await.unwrap(), LidarrEvent::GetAlbums(1).into());
    assert!(!app.data.lidarr_data.prompt_confirm);
    assert_eq!(app.tick_count, 0);
  }

  #[tokio::test]
  async fn test_dispatch_by_artist_history_block() {
    let (tx, mut rx) = mpsc::channel::<NetworkEvent>(500);
    let mut app = App::test_default();
    app.data.lidarr_data.prompt_confirm = true;
    app.network_tx = Some(tx);
    app.data.lidarr_data.artists.set_items(vec![Artist {
      id: 1,
      ..Artist::default()
    }]);

    app
      .dispatch_by_lidarr_block(&ActiveLidarrBlock::ArtistHistory)
      .await;

    assert!(app.is_loading);
    assert_eq!(
      rx.recv().await.unwrap(),
      LidarrEvent::GetArtistHistory(1).into()
    );
    assert!(!app.data.lidarr_data.prompt_confirm);
    assert_eq!(app.tick_count, 0);
  }

  #[tokio::test]
  async fn test_dispatch_by_manual_artist_search_block() {
    let (tx, mut rx) = mpsc::channel::<NetworkEvent>(500);
    let mut app = App::test_default();
    app.data.lidarr_data.prompt_confirm = true;
    app.network_tx = Some(tx);
    app.data.lidarr_data.artists.set_items(vec![Artist {
      id: 1,
      ..Artist::default()
    }]);

    app
      .dispatch_by_lidarr_block(&ActiveLidarrBlock::ManualArtistSearch)
      .await;

    assert!(app.is_loading);
    assert_eq!(
      rx.recv().await.unwrap(),
      LidarrEvent::GetDiscographyReleases(1).into()
    );
    assert!(!app.data.lidarr_data.prompt_confirm);
    assert_eq!(app.tick_count, 0);
  }

  #[tokio::test]
  async fn test_dispatch_by_manual_artist_search_block_discography_releases_non_empty() {
    let mut app = App::test_default();
    app
      .data
      .lidarr_data
      .discography_releases
      .set_items(vec![LidarrRelease::default()]);

    app
      .dispatch_by_lidarr_block(&ActiveLidarrBlock::ManualArtistSearch)
      .await;

    assert!(!app.is_loading);
    assert!(!app.data.lidarr_data.prompt_confirm);
    assert_eq!(app.tick_count, 0);
  }

  #[tokio::test]
  async fn test_dispatch_by_album_details_block() {
    let (tx, mut rx) = mpsc::channel::<NetworkEvent>(500);
    let mut app = App::test_default();
    app.data.lidarr_data.prompt_confirm = true;
    app.network_tx = Some(tx);
    app.data.lidarr_data.artists.set_items(vec![Artist {
      id: 1,
      ..Artist::default()
    }]);
    app.data.lidarr_data.albums.set_items(vec![Album {
      id: 1,
      ..Album::default()
    }]);

    app
      .dispatch_by_lidarr_block(&ActiveLidarrBlock::AlbumDetails)
      .await;

    assert!(app.is_loading);
    assert_eq!(
      rx.recv().await.unwrap(),
      LidarrEvent::GetTracks(1, 1).into()
    );
    assert_eq!(
      rx.recv().await.unwrap(),
      LidarrEvent::GetTrackFiles(1).into()
    );
    assert_eq!(
      rx.recv().await.unwrap(),
      LidarrEvent::GetDownloads(500).into()
    );
    assert!(!app.data.lidarr_data.prompt_confirm);
    assert_eq!(app.tick_count, 0);
  }

  #[tokio::test]
  async fn test_dispatch_by_album_history_block() {
    let (tx, mut rx) = mpsc::channel::<NetworkEvent>(500);
    let mut app = App::test_default();
    app.data.lidarr_data.prompt_confirm = true;
    app.network_tx = Some(tx);
    app.data.lidarr_data.artists.set_items(vec![Artist {
      id: 1,
      ..Artist::default()
    }]);
    app.data.lidarr_data.albums.set_items(vec![Album {
      id: 1,
      ..Album::default()
    }]);

    app
      .dispatch_by_lidarr_block(&ActiveLidarrBlock::AlbumHistory)
      .await;

    assert!(app.is_loading);
    assert_eq!(
      rx.recv().await.unwrap(),
      LidarrEvent::GetAlbumHistory(1, 1).into()
    );
    assert!(!app.data.lidarr_data.prompt_confirm);
    assert_eq!(app.tick_count, 0);
  }

  #[tokio::test]
  async fn test_dispatch_by_album_history_block_no_op_when_albums_table_is_empty() {
    let (tx, _) = mpsc::channel::<NetworkEvent>(500);
    let mut app = App::test_default();
    app.data.lidarr_data.prompt_confirm = true;
    app.network_tx = Some(tx);
    app.data.lidarr_data.artists.set_items(vec![Artist {
      id: 1,
      ..Artist::default()
    }]);

    app
      .dispatch_by_lidarr_block(&ActiveLidarrBlock::AlbumHistory)
      .await;

    assert!(!app.is_loading);
    assert!(!app.data.lidarr_data.prompt_confirm);
    assert_eq!(app.tick_count, 0);
  }

  #[tokio::test]
  async fn test_dispatch_by_manual_album_search_block() {
    let (tx, mut rx) = mpsc::channel::<NetworkEvent>(500);
    let mut app = App::test_default();
    app.data.lidarr_data.prompt_confirm = true;
    app.network_tx = Some(tx);
    app.data.lidarr_data.artists.set_items(vec![Artist {
      id: 1,
      ..Artist::default()
    }]);
    app.data.lidarr_data.albums.set_items(vec![Album {
      id: 1,
      ..Album::default()
    }]);
    app.data.lidarr_data.album_details_modal = Some(AlbumDetailsModal::default());

    app
      .dispatch_by_lidarr_block(&ActiveLidarrBlock::ManualAlbumSearch)
      .await;

    assert!(app.is_loading);
    assert_eq!(
      rx.recv().await.unwrap(),
      LidarrEvent::GetAlbumReleases(1, 1).into()
    );
    assert!(!app.data.lidarr_data.prompt_confirm);
    assert_eq!(app.tick_count, 0);
  }

  #[tokio::test]
  async fn test_dispatch_by_manual_album_search_block_is_loading() {
    let mut app = App {
      is_loading: true,
      ..App::test_default()
    };

    app
      .dispatch_by_lidarr_block(&ActiveLidarrBlock::ManualAlbumSearch)
      .await;

    assert!(app.is_loading);
    assert!(!app.data.lidarr_data.prompt_confirm);
    assert_eq!(app.tick_count, 0);
  }

  #[tokio::test]
  async fn test_dispatch_by_manual_album_search_block_album_releases_non_empty() {
    let mut app = App::test_default();
    let mut album_details_modal = AlbumDetailsModal::default();
    album_details_modal
      .album_releases
      .set_items(vec![LidarrRelease::default()]);
    app.data.lidarr_data.album_details_modal = Some(album_details_modal);

    app
      .dispatch_by_lidarr_block(&ActiveLidarrBlock::ManualAlbumSearch)
      .await;

    assert!(!app.is_loading);
    assert!(!app.data.lidarr_data.prompt_confirm);
    assert_eq!(app.tick_count, 0);
  }

  #[tokio::test]
  async fn test_dispatch_by_downloads_block() {
    let (tx, mut rx) = mpsc::channel::<NetworkEvent>(500);
    let mut app = App::test_default();
    app.data.lidarr_data.prompt_confirm = true;
    app.network_tx = Some(tx);

    app
      .dispatch_by_lidarr_block(&ActiveLidarrBlock::Downloads)
      .await;

    assert!(app.is_loading);
    assert_eq!(
      rx.recv().await.unwrap(),
      LidarrEvent::GetDownloads(500).into()
    );
    assert!(!app.data.lidarr_data.prompt_confirm);
    assert_eq!(app.tick_count, 0);
  }

  #[tokio::test]
  async fn test_dispatch_by_lidarr_block_add_artist_search_results() {
    let (tx, mut rx) = mpsc::channel::<NetworkEvent>(500);
    let mut app = App::test_default();
    app.data.lidarr_data.prompt_confirm = true;
    app.network_tx = Some(tx);
    app.data.lidarr_data.add_artist_search = Some("test artist".into());

    app
      .dispatch_by_lidarr_block(&ActiveLidarrBlock::AddArtistSearchResults)
      .await;

    assert!(app.is_loading);
    assert_eq!(
      rx.recv().await.unwrap(),
      LidarrEvent::SearchNewArtist("test artist".to_owned()).into()
    );
    assert!(!app.data.lidarr_data.prompt_confirm);
    assert_eq!(app.tick_count, 0);
  }

  #[tokio::test]
  async fn test_dispatch_by_history_block() {
    let (tx, mut rx) = mpsc::channel::<NetworkEvent>(500);
    let mut app = App::test_default();
    app.data.lidarr_data.prompt_confirm = true;
    app.network_tx = Some(tx);

    app
      .dispatch_by_lidarr_block(&ActiveLidarrBlock::History)
      .await;

    assert!(app.is_loading);
    assert_eq!(
      rx.recv().await.unwrap(),
      LidarrEvent::GetHistory(500).into()
    );
    assert!(!app.data.lidarr_data.prompt_confirm);
    assert_eq!(app.tick_count, 0);
  }

  #[tokio::test]
  async fn test_dispatch_by_root_folders_block() {
    let (tx, mut rx) = mpsc::channel::<NetworkEvent>(500);
    let mut app = App::test_default();
    app.data.lidarr_data.prompt_confirm = true;
    app.network_tx = Some(tx);

    app
      .dispatch_by_lidarr_block(&ActiveLidarrBlock::RootFolders)
      .await;

    assert!(app.is_loading);
    assert_eq!(rx.recv().await.unwrap(), LidarrEvent::GetRootFolders.into());
    assert!(!app.data.lidarr_data.prompt_confirm);
    assert_eq!(app.tick_count, 0);
  }

  #[tokio::test]
  async fn test_dispatch_by_indexers_block() {
    let (tx, mut rx) = mpsc::channel::<NetworkEvent>(500);
    let mut app = App::test_default();
    app.data.lidarr_data.prompt_confirm = true;
    app.network_tx = Some(tx);

    app
      .dispatch_by_lidarr_block(&ActiveLidarrBlock::Indexers)
      .await;

    assert!(app.is_loading);
    assert_eq!(rx.recv().await.unwrap(), LidarrEvent::GetTags.into());
    assert_eq!(rx.recv().await.unwrap(), LidarrEvent::GetIndexers.into());
    assert!(!app.data.lidarr_data.prompt_confirm);
    assert_eq!(app.tick_count, 0);
  }

  #[tokio::test]
  async fn test_dispatch_by_all_indexer_settings_block() {
    let (tx, mut rx) = mpsc::channel::<NetworkEvent>(500);
    let mut app = App::test_default();
    app.data.lidarr_data.prompt_confirm = true;
    app.network_tx = Some(tx);

    app
      .dispatch_by_lidarr_block(&ActiveLidarrBlock::AllIndexerSettingsPrompt)
      .await;

    assert!(app.is_loading);
    assert_eq!(
      rx.recv().await.unwrap(),
      LidarrEvent::GetAllIndexerSettings.into()
    );
    assert!(!app.data.lidarr_data.prompt_confirm);
    assert_eq!(app.tick_count, 0);
  }

  #[tokio::test]
  async fn test_dispatch_by_test_indexer_block() {
    let (tx, mut rx) = mpsc::channel::<NetworkEvent>(500);
    let mut app = App::test_default();
    app.data.lidarr_data.prompt_confirm = true;
    app.network_tx = Some(tx);
    app.data.lidarr_data.indexers.set_items(vec![Indexer {
      id: 1,
      ..Indexer::default()
    }]);

    app
      .dispatch_by_lidarr_block(&ActiveLidarrBlock::TestIndexer)
      .await;

    assert!(app.is_loading);
    assert!(!app.data.lidarr_data.prompt_confirm);
    assert_eq!(rx.recv().await.unwrap(), LidarrEvent::TestIndexer(1).into());
    assert_eq!(app.tick_count, 0);
  }

  #[tokio::test]
  async fn test_dispatch_by_test_all_indexers_block() {
    let (tx, mut rx) = mpsc::channel::<NetworkEvent>(500);
    let mut app = App::test_default();
    app.data.lidarr_data.prompt_confirm = true;
    app.network_tx = Some(tx);

    app
      .dispatch_by_lidarr_block(&ActiveLidarrBlock::TestAllIndexers)
      .await;

    assert!(app.is_loading);
    assert!(!app.data.lidarr_data.prompt_confirm);
    assert_eq!(
      rx.recv().await.unwrap(),
      LidarrEvent::TestAllIndexers.into()
    );
    assert_eq!(app.tick_count, 0);
  }

  #[tokio::test]
  async fn test_dispatch_by_system_block() {
    let (tx, mut rx) = mpsc::channel::<NetworkEvent>(500);
    let mut app = App::test_default();
    app.data.lidarr_data.prompt_confirm = true;
    app.network_tx = Some(tx);

    app
      .dispatch_by_lidarr_block(&ActiveLidarrBlock::System)
      .await;

    assert!(app.is_loading);
    assert_eq!(rx.recv().await.unwrap(), LidarrEvent::GetTasks.into());
    assert_eq!(
      rx.recv().await.unwrap(),
      LidarrEvent::GetQueuedEvents.into()
    );
    assert_eq!(rx.recv().await.unwrap(), LidarrEvent::GetLogs(500).into());
    assert!(!app.data.lidarr_data.prompt_confirm);
    assert_eq!(app.tick_count, 0);
  }

  #[tokio::test]
  async fn test_dispatch_by_system_updates_block() {
    let (tx, mut rx) = mpsc::channel::<NetworkEvent>(500);
    let mut app = App::test_default();
    app.data.lidarr_data.prompt_confirm = true;
    app.network_tx = Some(tx);

    app
      .dispatch_by_lidarr_block(&ActiveLidarrBlock::SystemUpdates)
      .await;

    assert!(app.is_loading);
    assert_eq!(rx.recv().await.unwrap(), LidarrEvent::GetUpdates.into());
    assert!(!app.data.lidarr_data.prompt_confirm);
    assert_eq!(app.tick_count, 0);
  }

  #[tokio::test]
  async fn test_check_for_lidarr_prompt_action_no_prompt_confirm() {
    let mut app = App::test_default();
    app.data.lidarr_data.prompt_confirm = false;

    app.check_for_lidarr_prompt_action().await;

    assert!(!app.data.lidarr_data.prompt_confirm);
    assert!(!app.should_refresh);
  }

  #[tokio::test]
  async fn test_check_for_lidarr_prompt_action() {
    let (tx, mut rx) = mpsc::channel::<NetworkEvent>(500);
    let mut app = App::test_default();
    app.data.lidarr_data.prompt_confirm = true;
    app.network_tx = Some(tx);
    app.data.lidarr_data.prompt_confirm_action = Some(LidarrEvent::GetStatus);

    app.check_for_lidarr_prompt_action().await;

    assert!(!app.data.lidarr_data.prompt_confirm);
    assert_eq!(rx.recv().await.unwrap(), LidarrEvent::GetStatus.into());
    assert!(app.should_refresh);
    assert_eq!(app.data.lidarr_data.prompt_confirm_action, None);
  }

  #[tokio::test]
  async fn test_lidarr_refresh_metadata() {
    let (tx, mut rx) = mpsc::channel::<NetworkEvent>(500);
    let mut app = App::test_default();
    app.data.lidarr_data.prompt_confirm = true;
    app.network_tx = Some(tx);
    app.is_routing = true;

    app.refresh_lidarr_metadata().await;

    assert_eq!(
      rx.recv().await.unwrap(),
      LidarrEvent::GetQualityProfiles.into()
    );
    assert_eq!(
      rx.recv().await.unwrap(),
      LidarrEvent::GetMetadataProfiles.into()
    );
    assert_eq!(rx.recv().await.unwrap(), LidarrEvent::GetTags.into());
    assert_eq!(rx.recv().await.unwrap(), LidarrEvent::GetRootFolders.into());
    assert_eq!(
      rx.recv().await.unwrap(),
      LidarrEvent::GetDownloads(500).into()
    );
    assert_eq!(rx.recv().await.unwrap(), LidarrEvent::GetDiskSpace.into());
    assert_eq!(rx.recv().await.unwrap(), LidarrEvent::GetStatus.into());
    assert!(app.is_loading);
  }

  #[tokio::test]
  async fn test_lidarr_on_tick_first_render() {
    let (tx, mut rx) = mpsc::channel::<NetworkEvent>(500);
    let mut app = App::test_default();
    app.data.lidarr_data.prompt_confirm = true;
    app.network_tx = Some(tx);
    app.is_first_render = true;

    app.lidarr_on_tick(ActiveLidarrBlock::Downloads).await;

    assert_eq!(
      rx.recv().await.unwrap(),
      LidarrEvent::GetQualityProfiles.into()
    );
    assert_eq!(
      rx.recv().await.unwrap(),
      LidarrEvent::GetMetadataProfiles.into()
    );
    assert_eq!(rx.recv().await.unwrap(), LidarrEvent::GetTags.into());
    assert_eq!(rx.recv().await.unwrap(), LidarrEvent::GetRootFolders.into());
    assert_eq!(
      rx.recv().await.unwrap(),
      LidarrEvent::GetDownloads(500).into()
    );
    assert_eq!(rx.recv().await.unwrap(), LidarrEvent::GetDiskSpace.into());
    assert_eq!(rx.recv().await.unwrap(), LidarrEvent::GetStatus.into());
    assert!(app.is_loading);
    assert!(!app.data.lidarr_data.prompt_confirm);
    assert!(!app.is_first_render);
  }

  #[tokio::test]
  async fn test_lidarr_on_tick_routing() {
    let (tx, mut rx) = mpsc::channel::<NetworkEvent>(500);
    let mut app = App::test_default();
    app.data.lidarr_data.prompt_confirm = true;
    app.network_tx = Some(tx);
    app.is_routing = true;
    app.should_refresh = true;
    app.is_first_render = false;
    app.tick_count = 1;

    app.lidarr_on_tick(ActiveLidarrBlock::Downloads).await;

    assert_eq!(
      rx.recv().await.unwrap(),
      LidarrEvent::GetDownloads(500).into()
    );
    assert!(!app.data.lidarr_data.prompt_confirm);
  }

  #[tokio::test]
  async fn test_lidarr_on_tick_routing_while_long_request_is_running_should_cancel_request() {
    let (tx, _) = mpsc::channel::<NetworkEvent>(500);
    let mut app = App::test_default();
    app.data.lidarr_data.prompt_confirm = true;
    app.network_tx = Some(tx);
    app.is_routing = true;
    app.should_refresh = false;
    app.is_first_render = false;
    app.tick_count = 1;

    app.lidarr_on_tick(ActiveLidarrBlock::Downloads).await;

    assert!(app.cancellation_token.is_cancelled());
  }

  #[tokio::test]
  async fn test_lidarr_on_tick_should_refresh() {
    let (tx, mut rx) = mpsc::channel::<NetworkEvent>(500);
    let mut app = App::test_default();
    app.data.lidarr_data.prompt_confirm = true;
    app.network_tx = Some(tx);
    app.should_refresh = true;
    app.is_first_render = false;
    app.tick_count = 1;

    app.lidarr_on_tick(ActiveLidarrBlock::Downloads).await;

    assert_eq!(
      rx.recv().await.unwrap(),
      LidarrEvent::GetDownloads(500).into()
    );
    assert!(app.should_refresh);
    assert!(!app.data.lidarr_data.prompt_confirm);
  }

  #[tokio::test]
  async fn test_lidarr_on_tick_should_refresh_does_not_cancel_prompt_requests() {
    let (tx, mut rx) = mpsc::channel::<NetworkEvent>(500);
    let mut app = App::test_default();
    app.data.lidarr_data.prompt_confirm = true;
    app.network_tx = Some(tx);
    app.is_loading = true;
    app.is_routing = true;
    app.should_refresh = true;
    app.is_first_render = false;
    app.tick_count = 1;

    app.lidarr_on_tick(ActiveLidarrBlock::Downloads).await;

    assert_eq!(
      rx.recv().await.unwrap(),
      LidarrEvent::GetDownloads(500).into()
    );
    assert!(app.is_loading);
    assert!(app.should_refresh);
    assert!(!app.data.lidarr_data.prompt_confirm);
    assert!(!app.cancellation_token.is_cancelled());
  }

  #[tokio::test]
  async fn test_lidarr_on_tick_network_tick_frequency() {
    let (tx, mut rx) = mpsc::channel::<NetworkEvent>(500);
    let mut app = App::test_default();
    app.data.lidarr_data.prompt_confirm = true;
    app.network_tx = Some(tx);
    app.tick_count = 2;
    app.tick_until_poll = 2;

    app.lidarr_on_tick(ActiveLidarrBlock::Downloads).await;

    assert_eq!(
      rx.recv().await.unwrap(),
      LidarrEvent::GetQualityProfiles.into()
    );
    assert_eq!(
      rx.recv().await.unwrap(),
      LidarrEvent::GetMetadataProfiles.into()
    );
    assert_eq!(rx.recv().await.unwrap(), LidarrEvent::GetTags.into());
    assert_eq!(rx.recv().await.unwrap(), LidarrEvent::GetRootFolders.into());
    assert_eq!(
      rx.recv().await.unwrap(),
      LidarrEvent::GetDownloads(500).into()
    );
    assert!(app.is_loading);
  }

  #[tokio::test]
  async fn test_extract_add_new_artist_search_query() {
    let app = App::test_default_fully_populated();

    let query = app.extract_add_new_artist_search_query().await;

    assert_str_eq!(query, "Test Artist");
  }

  #[tokio::test]
  #[should_panic(expected = "Add artist search is empty")]
  async fn test_extract_add_new_artist_search_query_panics_when_the_query_is_not_set() {
    let app = App::test_default();

    app.extract_add_new_artist_search_query().await;
  }

  #[tokio::test]
  async fn test_extract_artist_id() {
    let mut app = App::test_default();
    app.data.lidarr_data.artists.set_items(vec![artist()]);

    assert_eq!(app.extract_artist_id().await, 1);
  }

  #[tokio::test]
  async fn test_extract_lidarr_indexer_id() {
    let mut app = App::test_default();
    app.data.lidarr_data.indexers.set_items(vec![Indexer {
      id: 1,
      ..Indexer::default()
    }]);

    assert_eq!(app.extract_lidarr_indexer_id().await, 1);
  }
}
