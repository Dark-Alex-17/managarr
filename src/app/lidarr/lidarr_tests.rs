#[cfg(test)]
mod tests {
  use crate::app::App;
  use crate::models::servarr_data::lidarr::lidarr_data::ActiveLidarrBlock;
  use crate::network::NetworkEvent;
  use crate::network::lidarr_network::LidarrEvent;
  use tokio::sync::mpsc;

  #[tokio::test]
  async fn test_dispatch_by_lidarr_block_artists() {
    let (tx, mut rx) = mpsc::channel::<NetworkEvent>(500);
    let mut app = App::test_default();
    app.network_tx = Some(tx);

    app.dispatch_by_lidarr_block(&ActiveLidarrBlock::Artists).await;

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
  }
}
