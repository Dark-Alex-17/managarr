#[cfg(test)]
mod tests {
  use crate::app::App;
  use crate::models::servarr_data::lidarr::lidarr_data::ActiveLidarrBlock;
  use crate::network::NetworkEvent;
  use crate::network::lidarr_network::LidarrEvent;
  use pretty_assertions::{assert_eq, assert_str_eq};
  use tokio::sync::mpsc;

  #[tokio::test]
  async fn test_dispatch_by_lidarr_block_artists() {
    let (tx, mut rx) = mpsc::channel::<NetworkEvent>(500);
    let mut app = App::test_default();
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
    assert!(!app.data.sonarr_data.prompt_confirm);
    assert_eq!(app.tick_count, 0);
  }

  #[tokio::test]
  async fn test_dispatch_by_lidarr_block_add_artist_search_results() {
    let (tx, mut rx) = mpsc::channel::<NetworkEvent>(500);
    let mut app = App::test_default();
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
  async fn test_extract_add_new_artist_search_query() {
    let app = App::test_default_fully_populated();

    let query = app.extract_add_new_artist_search_query().await;

    assert_str_eq!(query, "Test Artist");
  }
}
