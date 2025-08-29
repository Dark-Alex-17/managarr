#[cfg(test)]
mod tests {
  use pretty_assertions::assert_eq;
  use tokio::sync::mpsc;

  use crate::app::radarr::ActiveRadarrBlock;
  use crate::app::App;
  use crate::models::radarr_models::{
    AddMovieBody, AddMovieOptions, Collection, CollectionMovie, Credit, Movie, RadarrRelease,
  };
  use crate::models::servarr_data::radarr::modals::MovieDetailsModal;
  use crate::models::servarr_models::Indexer;
  use crate::network::radarr_network::RadarrEvent;
  use crate::network::NetworkEvent;

  #[tokio::test]
  async fn test_dispatch_by_blocklist_block() {
    let (mut app, mut sync_network_rx) = construct_app_unit();

    app
      .dispatch_by_radarr_block(&ActiveRadarrBlock::Blocklist)
      .await;

    assert!(app.is_loading);
    assert_eq!(
      sync_network_rx.recv().await.unwrap(),
      RadarrEvent::GetBlocklist.into()
    );
    assert!(!app.data.radarr_data.prompt_confirm);
    assert_eq!(app.tick_count, 0);
  }

  #[tokio::test]
  async fn test_dispatch_by_collections_block() {
    let (mut app, mut sync_network_rx) = construct_app_unit();

    app
      .dispatch_by_radarr_block(&ActiveRadarrBlock::Collections)
      .await;

    assert!(app.is_loading);
    assert_eq!(
      sync_network_rx.recv().await.unwrap(),
      RadarrEvent::GetQualityProfiles.into()
    );
    assert_eq!(
      sync_network_rx.recv().await.unwrap(),
      RadarrEvent::GetCollections.into()
    );
    assert_eq!(
      sync_network_rx.recv().await.unwrap(),
      RadarrEvent::GetMovies.into()
    );
    assert!(!app.data.radarr_data.prompt_confirm);
    assert_eq!(app.tick_count, 0);
  }

  #[tokio::test]
  async fn test_dispatch_by_collection_details_block() {
    let (mut app, mut sync_network_rx) = construct_app_unit();

    app.data.radarr_data.collections.set_items(vec![Collection {
      movies: Some(vec![CollectionMovie::default()]),
      ..Collection::default()
    }]);

    app
      .dispatch_by_radarr_block(&ActiveRadarrBlock::CollectionDetails)
      .await;

    assert!(!app.is_loading);
    assert_eq!(
      sync_network_rx.recv().await.unwrap(),
      RadarrEvent::GetQualityProfiles.into()
    );
    assert_eq!(
      sync_network_rx.recv().await.unwrap(),
      RadarrEvent::GetTags.into()
    );
    assert!(!app.data.radarr_data.collection_movies.items.is_empty());
    assert_eq!(app.tick_count, 0);
    assert!(!app.data.radarr_data.prompt_confirm);
  }

  #[tokio::test]
  async fn test_dispatch_by_collection_details_block_with_add_movie() {
    let add_movie_body = AddMovieBody {
      tmdb_id: 1234,
      title: "Test".to_owned(),
      root_folder_path: "/nfs2".to_owned(),
      minimum_availability: "announced".to_owned(),
      monitored: true,
      quality_profile_id: 2222,
      tags: vec![1, 2],
      tag_input_string: None,
      add_options: AddMovieOptions {
        monitor: "movieOnly".to_owned(),
        search_for_movie: true,
      },
    };
    let (mut app, mut sync_network_rx) = construct_app_unit();
    app.data.radarr_data.prompt_confirm_action =
      Some(RadarrEvent::AddMovie(add_movie_body.clone()));

    app.data.radarr_data.collections.set_items(vec![Collection {
      movies: Some(vec![CollectionMovie::default()]),
      ..Collection::default()
    }]);

    app
      .dispatch_by_radarr_block(&ActiveRadarrBlock::CollectionDetails)
      .await;

    assert!(app.is_loading);
    assert_eq!(
      sync_network_rx.recv().await.unwrap(),
      RadarrEvent::GetQualityProfiles.into()
    );
    assert_eq!(
      sync_network_rx.recv().await.unwrap(),
      RadarrEvent::GetTags.into()
    );
    assert_eq!(
      sync_network_rx.recv().await.unwrap(),
      RadarrEvent::AddMovie(add_movie_body).into()
    );
    assert!(!app.data.radarr_data.collection_movies.items.is_empty());
    assert_eq!(app.tick_count, 0);
    assert!(!app.data.radarr_data.prompt_confirm);
  }

  #[tokio::test]
  async fn test_dispatch_by_downloads_block() {
    let (mut app, mut sync_network_rx) = construct_app_unit();

    app
      .dispatch_by_radarr_block(&ActiveRadarrBlock::Downloads)
      .await;

    assert!(app.is_loading);
    assert_eq!(
      sync_network_rx.recv().await.unwrap(),
      RadarrEvent::GetDownloads(500).into()
    );
    assert!(!app.data.radarr_data.prompt_confirm);
    assert_eq!(app.tick_count, 0);
  }

  #[tokio::test]
  async fn test_dispatch_by_root_folders_block() {
    let (mut app, mut sync_network_rx) = construct_app_unit();

    app
      .dispatch_by_radarr_block(&ActiveRadarrBlock::RootFolders)
      .await;

    assert!(app.is_loading);
    assert_eq!(
      sync_network_rx.recv().await.unwrap(),
      RadarrEvent::GetRootFolders.into()
    );
    assert!(!app.data.radarr_data.prompt_confirm);
    assert_eq!(app.tick_count, 0);
  }

  #[tokio::test]
  async fn test_dispatch_by_movies_block() {
    let (mut app, mut sync_network_rx) = construct_app_unit();

    app
      .dispatch_by_radarr_block(&ActiveRadarrBlock::Movies)
      .await;

    assert!(app.is_loading);
    assert_eq!(
      sync_network_rx.recv().await.unwrap(),
      RadarrEvent::GetQualityProfiles.into()
    );
    assert_eq!(
      sync_network_rx.recv().await.unwrap(),
      RadarrEvent::GetTags.into()
    );
    assert_eq!(
      sync_network_rx.recv().await.unwrap(),
      RadarrEvent::GetMovies.into()
    );
    assert_eq!(
      sync_network_rx.recv().await.unwrap(),
      RadarrEvent::GetDownloads(500).into()
    );
    assert!(!app.data.radarr_data.prompt_confirm);
    assert_eq!(app.tick_count, 0);
  }

  #[tokio::test]
  async fn test_dispatch_by_indexers_block() {
    let (mut app, mut sync_network_rx) = construct_app_unit();

    app
      .dispatch_by_radarr_block(&ActiveRadarrBlock::Indexers)
      .await;

    assert!(app.is_loading);
    assert_eq!(
      sync_network_rx.recv().await.unwrap(),
      RadarrEvent::GetTags.into()
    );
    assert_eq!(
      sync_network_rx.recv().await.unwrap(),
      RadarrEvent::GetIndexers.into()
    );
    assert!(!app.data.radarr_data.prompt_confirm);
    assert_eq!(app.tick_count, 0);
  }

  #[tokio::test]
  async fn test_dispatch_by_all_indexer_settings_block() {
    let (mut app, mut sync_network_rx) = construct_app_unit();

    app
      .dispatch_by_radarr_block(&ActiveRadarrBlock::AllIndexerSettingsPrompt)
      .await;

    assert!(app.is_loading);
    assert_eq!(
      sync_network_rx.recv().await.unwrap(),
      RadarrEvent::GetAllIndexerSettings.into()
    );
    assert!(!app.data.radarr_data.prompt_confirm);
    assert_eq!(app.tick_count, 0);
  }

  #[tokio::test]
  async fn test_dispatch_by_test_indexer_block() {
    let (mut app, mut sync_network_rx) = construct_app_unit();
    app.data.radarr_data.indexers.set_items(vec![Indexer {
      id: 1,
      ..Indexer::default()
    }]);

    app
      .dispatch_by_radarr_block(&ActiveRadarrBlock::TestIndexer)
      .await;

    assert!(app.is_loading);
    assert_eq!(
      sync_network_rx.recv().await.unwrap(),
      RadarrEvent::TestIndexer(1).into()
    );
    assert_eq!(app.tick_count, 0);
  }

  #[tokio::test]
  async fn test_dispatch_by_test_all_indexers_block() {
    let (mut app, mut sync_network_rx) = construct_app_unit();

    app
      .dispatch_by_radarr_block(&ActiveRadarrBlock::TestAllIndexers)
      .await;

    assert!(app.is_loading);
    assert_eq!(
      sync_network_rx.recv().await.unwrap(),
      RadarrEvent::TestAllIndexers.into()
    );
    assert_eq!(app.tick_count, 0);
  }

  #[tokio::test]
  async fn test_dispatch_by_system_block() {
    let (mut app, mut sync_network_rx) = construct_app_unit();

    app
      .dispatch_by_radarr_block(&ActiveRadarrBlock::System)
      .await;

    assert!(app.is_loading);
    assert_eq!(
      sync_network_rx.recv().await.unwrap(),
      RadarrEvent::GetTasks.into()
    );
    assert_eq!(
      sync_network_rx.recv().await.unwrap(),
      RadarrEvent::GetQueuedEvents.into()
    );
    assert_eq!(
      sync_network_rx.recv().await.unwrap(),
      RadarrEvent::GetLogs(500).into()
    );
    assert!(!app.data.radarr_data.prompt_confirm);
    assert_eq!(app.tick_count, 0);
  }

  #[tokio::test]
  async fn test_dispatch_by_system_updates_block() {
    let (mut app, mut sync_network_rx) = construct_app_unit();

    app
      .dispatch_by_radarr_block(&ActiveRadarrBlock::SystemUpdates)
      .await;

    assert!(app.is_loading);
    assert_eq!(
      sync_network_rx.recv().await.unwrap(),
      RadarrEvent::GetUpdates.into()
    );
    assert!(!app.data.radarr_data.prompt_confirm);
    assert_eq!(app.tick_count, 0);
  }

  #[tokio::test]
  async fn test_dispatch_by_add_movie_search_results_block() {
    let (mut app, mut sync_network_rx) = construct_app_unit();
    app.data.radarr_data.add_movie_search = Some("test".into());

    app
      .dispatch_by_radarr_block(&ActiveRadarrBlock::AddMovieSearchResults)
      .await;

    assert!(app.is_loading);
    assert_eq!(
      sync_network_rx.recv().await.unwrap(),
      RadarrEvent::SearchNewMovie("test".into()).into()
    );
    assert!(!app.data.radarr_data.prompt_confirm);
    assert_eq!(app.tick_count, 0);
  }

  #[tokio::test]
  async fn test_dispatch_by_movie_details_block() {
    let (mut app, mut sync_network_rx) = construct_app_unit();
    app.data.radarr_data.movies.set_items(vec![Movie {
      id: 1,
      ..Movie::default()
    }]);

    app
      .dispatch_by_radarr_block(&ActiveRadarrBlock::MovieDetails)
      .await;

    assert!(app.is_loading);
    assert_eq!(
      sync_network_rx.recv().await.unwrap(),
      RadarrEvent::GetMovieDetails(1).into()
    );
    assert!(!app.data.radarr_data.prompt_confirm);
    assert_eq!(app.tick_count, 0);
  }

  #[tokio::test]
  async fn test_dispatch_by_file_info_block() {
    let (mut app, mut sync_network_rx) = construct_app_unit();
    app.data.radarr_data.movies.set_items(vec![Movie {
      id: 1,
      ..Movie::default()
    }]);

    app
      .dispatch_by_radarr_block(&ActiveRadarrBlock::FileInfo)
      .await;

    assert!(app.is_loading);
    assert_eq!(
      sync_network_rx.recv().await.unwrap(),
      RadarrEvent::GetMovieDetails(1).into()
    );
    assert!(!app.data.radarr_data.prompt_confirm);
    assert_eq!(app.tick_count, 0);
  }

  #[tokio::test]
  async fn test_dispatch_by_movie_history_block() {
    let (mut app, mut sync_network_rx) = construct_app_unit();
    app.data.radarr_data.movies.set_items(vec![Movie {
      id: 1,
      ..Movie::default()
    }]);

    app
      .dispatch_by_radarr_block(&ActiveRadarrBlock::MovieHistory)
      .await;

    assert!(app.is_loading);
    assert_eq!(
      sync_network_rx.recv().await.unwrap(),
      RadarrEvent::GetMovieHistory(1).into()
    );
    assert!(!app.data.radarr_data.prompt_confirm);
    assert_eq!(app.tick_count, 0);
  }

  #[tokio::test]
  async fn test_dispatch_by_cast_crew_blocks() {
    let (mut app, mut sync_network_rx) = construct_app_unit();
    app.data.radarr_data.movies.set_items(vec![Movie {
      id: 1,
      ..Movie::default()
    }]);

    for active_radarr_block in &[ActiveRadarrBlock::Cast, ActiveRadarrBlock::Crew] {
      app.data.radarr_data.movie_details_modal = Some(MovieDetailsModal::default());
      app.dispatch_by_radarr_block(active_radarr_block).await;

      assert!(app.is_loading);
      assert_eq!(
        sync_network_rx.recv().await.unwrap(),
        RadarrEvent::GetMovieCredits(1).into()
      );
      assert!(!app.data.radarr_data.prompt_confirm);
      assert_eq!(app.tick_count, 0);
    }
  }

  #[tokio::test]
  async fn test_dispatch_by_cast_crew_blocks_movie_cast_non_empty() {
    let (mut app, mut sync_network_rx) = construct_app_unit();
    app.data.radarr_data.movies.set_items(vec![Movie {
      id: 1,
      ..Movie::default()
    }]);

    for active_radarr_block in &[ActiveRadarrBlock::Cast, ActiveRadarrBlock::Crew] {
      let mut movie_details_modal = MovieDetailsModal::default();
      movie_details_modal
        .movie_cast
        .set_items(vec![Credit::default()]);
      app.data.radarr_data.movie_details_modal = Some(movie_details_modal);

      app.dispatch_by_radarr_block(active_radarr_block).await;

      assert!(app.is_loading);
      assert_eq!(
        sync_network_rx.recv().await.unwrap(),
        RadarrEvent::GetMovieCredits(1).into()
      );
      assert!(!app.data.radarr_data.prompt_confirm);
      assert_eq!(app.tick_count, 0);
    }
  }

  #[tokio::test]
  async fn test_dispatch_by_cast_crew_blocks_movie_crew_non_empty() {
    let (mut app, mut sync_network_rx) = construct_app_unit();
    app.data.radarr_data.movies.set_items(vec![Movie {
      id: 1,
      ..Movie::default()
    }]);

    for active_radarr_block in &[ActiveRadarrBlock::Cast, ActiveRadarrBlock::Crew] {
      let mut movie_details_modal = MovieDetailsModal::default();
      movie_details_modal
        .movie_crew
        .set_items(vec![Credit::default()]);
      app.data.radarr_data.movie_details_modal = Some(movie_details_modal);

      app.dispatch_by_radarr_block(active_radarr_block).await;

      assert!(app.is_loading);
      assert_eq!(
        sync_network_rx.recv().await.unwrap(),
        RadarrEvent::GetMovieCredits(1).into()
      );
      assert!(!app.data.radarr_data.prompt_confirm);
      assert_eq!(app.tick_count, 0);
    }
  }

  #[tokio::test]
  async fn test_dispatch_by_cast_crew_blocks_cast_and_crew_non_empty() {
    let mut app = App::test_default();

    for active_radarr_block in &[ActiveRadarrBlock::Cast, ActiveRadarrBlock::Crew] {
      let mut movie_details_modal = MovieDetailsModal::default();
      movie_details_modal
        .movie_cast
        .set_items(vec![Credit::default()]);
      movie_details_modal
        .movie_crew
        .set_items(vec![Credit::default()]);
      app.data.radarr_data.movie_details_modal = Some(movie_details_modal);

      app.dispatch_by_radarr_block(active_radarr_block).await;

      assert!(!app.is_loading);
      assert!(!app.data.radarr_data.prompt_confirm);
      assert_eq!(app.tick_count, 0);
    }
  }

  #[tokio::test]
  async fn test_dispatch_by_manual_search_block() {
    let (mut app, mut sync_network_rx) = construct_app_unit();
    app.data.radarr_data.movies.set_items(vec![Movie {
      id: 1,
      ..Movie::default()
    }]);
    app.data.radarr_data.movie_details_modal = Some(MovieDetailsModal::default());

    app
      .dispatch_by_radarr_block(&ActiveRadarrBlock::ManualSearch)
      .await;

    assert!(app.is_loading);
    assert_eq!(
      sync_network_rx.recv().await.unwrap(),
      RadarrEvent::GetReleases(1).into()
    );
    assert!(!app.data.radarr_data.prompt_confirm);
    assert_eq!(app.tick_count, 0);
  }

  #[tokio::test]
  async fn test_dispatch_by_manual_search_block_movie_releases_non_empty() {
    let mut app = App::test_default();
    let mut movie_details_modal = MovieDetailsModal::default();
    movie_details_modal
      .movie_releases
      .set_items(vec![RadarrRelease::default()]);
    app.data.radarr_data.movie_details_modal = Some(movie_details_modal);

    app
      .dispatch_by_radarr_block(&ActiveRadarrBlock::ManualSearch)
      .await;

    assert!(!app.is_loading);
    assert!(!app.data.radarr_data.prompt_confirm);
    assert_eq!(app.tick_count, 0);
  }

  #[tokio::test]
  async fn test_dispatch_by_manual_search_block_is_loading() {
    let mut app = App {
      is_loading: true,
      ..App::test_default()
    };

    app
      .dispatch_by_radarr_block(&ActiveRadarrBlock::ManualSearch)
      .await;

    assert!(app.is_loading);
    assert!(!app.data.radarr_data.prompt_confirm);
    assert_eq!(app.tick_count, 0);
  }

  #[tokio::test]
  async fn test_check_for_radarr_prompt_action_no_prompt_confirm() {
    let mut app = App::test_default();
    app.data.radarr_data.prompt_confirm = false;

    app.check_for_radarr_prompt_action().await;

    assert!(!app.data.radarr_data.prompt_confirm);
    assert!(!app.should_refresh);
  }

  #[tokio::test]
  async fn test_check_for_radarr_prompt_action() {
    let (mut app, mut sync_network_rx) = construct_app_unit();
    app.data.radarr_data.prompt_confirm_action = Some(RadarrEvent::GetStatus);

    app.check_for_radarr_prompt_action().await;

    assert!(!app.data.radarr_data.prompt_confirm);
    assert_eq!(
      sync_network_rx.recv().await.unwrap(),
      RadarrEvent::GetStatus.into()
    );
    assert!(app.should_refresh);
    assert_eq!(app.data.radarr_data.prompt_confirm_action, None);
  }

  #[tokio::test]
  async fn test_radarr_refresh_metadata() {
    let (mut app, mut sync_network_rx) = construct_app_unit();
    app.is_routing = true;

    app.refresh_radarr_metadata().await;

    assert_eq!(
      sync_network_rx.recv().await.unwrap(),
      RadarrEvent::GetQualityProfiles.into()
    );
    assert_eq!(
      sync_network_rx.recv().await.unwrap(),
      RadarrEvent::GetTags.into()
    );
    assert_eq!(
      sync_network_rx.recv().await.unwrap(),
      RadarrEvent::GetRootFolders.into()
    );
    assert_eq!(
      sync_network_rx.recv().await.unwrap(),
      RadarrEvent::GetDownloads(500).into()
    );
    assert_eq!(
      sync_network_rx.recv().await.unwrap(),
      RadarrEvent::GetDiskSpace.into()
    );
    assert_eq!(
      sync_network_rx.recv().await.unwrap(),
      RadarrEvent::GetStatus.into()
    );
    assert!(app.is_loading);
  }

  #[tokio::test]
  async fn test_radarr_on_tick_first_render() {
    let (mut app, mut sync_network_rx) = construct_app_unit();
    app.is_first_render = true;

    app.radarr_on_tick(ActiveRadarrBlock::Downloads).await;

    assert_eq!(
      sync_network_rx.recv().await.unwrap(),
      RadarrEvent::GetQualityProfiles.into()
    );
    assert_eq!(
      sync_network_rx.recv().await.unwrap(),
      RadarrEvent::GetTags.into()
    );
    assert_eq!(
      sync_network_rx.recv().await.unwrap(),
      RadarrEvent::GetRootFolders.into()
    );
    assert_eq!(
      sync_network_rx.recv().await.unwrap(),
      RadarrEvent::GetDownloads(500).into()
    );
    assert_eq!(
      sync_network_rx.recv().await.unwrap(),
      RadarrEvent::GetDiskSpace.into()
    );
    assert_eq!(
      sync_network_rx.recv().await.unwrap(),
      RadarrEvent::GetStatus.into()
    );
    assert!(app.is_loading);
    assert!(!app.data.radarr_data.prompt_confirm);
    assert!(!app.is_first_render);
  }

  #[tokio::test]
  async fn test_radarr_on_tick_routing() {
    let (mut app, mut sync_network_rx) = construct_app_unit();
    app.is_routing = true;
    app.should_refresh = true;

    app.radarr_on_tick(ActiveRadarrBlock::Downloads).await;

    assert_eq!(
      sync_network_rx.recv().await.unwrap(),
      RadarrEvent::GetDownloads(500).into()
    );
    assert!(!app.data.radarr_data.prompt_confirm);
  }

  #[tokio::test]
  async fn test_radarr_on_tick_routing_while_long_request_is_running_should_cancel_request() {
    let (mut app, _) = construct_app_unit();
    app.is_routing = true;
    app.should_refresh = false;

    app.radarr_on_tick(ActiveRadarrBlock::Downloads).await;

    assert!(app.cancellation_token.is_cancelled());
  }

  #[tokio::test]
  async fn test_radarr_on_tick_should_refresh() {
    let (mut app, mut sync_network_rx) = construct_app_unit();
    app.should_refresh = true;

    app.radarr_on_tick(ActiveRadarrBlock::Downloads).await;

    assert_eq!(
      sync_network_rx.recv().await.unwrap(),
      RadarrEvent::GetDownloads(500).into()
    );
    assert!(app.should_refresh);
    assert!(!app.data.radarr_data.prompt_confirm);
  }

  #[tokio::test]
  async fn test_radarr_on_tick_should_refresh_does_not_cancel_prompt_requests() {
    let (mut app, mut sync_network_rx) = construct_app_unit();
    app.is_loading = true;
    app.is_routing = true;
    app.should_refresh = true;

    app.radarr_on_tick(ActiveRadarrBlock::Downloads).await;

    assert_eq!(
      sync_network_rx.recv().await.unwrap(),
      RadarrEvent::GetDownloads(500).into()
    );
    assert!(app.is_loading);
    assert!(app.should_refresh);
    assert!(!app.data.radarr_data.prompt_confirm);
    assert!(!app.cancellation_token.is_cancelled());
  }

  #[tokio::test]
  async fn test_radarr_on_tick_network_tick_frequency() {
    let (mut app, mut sync_network_rx) = construct_app_unit();
    app.tick_count = 2;
    app.tick_until_poll = 2;

    app.radarr_on_tick(ActiveRadarrBlock::Downloads).await;

    assert_eq!(
      sync_network_rx.recv().await.unwrap(),
      RadarrEvent::GetQualityProfiles.into()
    );
    assert_eq!(
      sync_network_rx.recv().await.unwrap(),
      RadarrEvent::GetTags.into()
    );
    assert_eq!(
      sync_network_rx.recv().await.unwrap(),
      RadarrEvent::GetRootFolders.into()
    );
    assert_eq!(
      sync_network_rx.recv().await.unwrap(),
      RadarrEvent::GetDownloads(500).into()
    );
    assert!(app.is_loading);
  }

  #[tokio::test]
  async fn test_populate_movie_collection_table_unfiltered() {
    let mut app = App::test_default();
    app.data.radarr_data.collections.set_items(vec![Collection {
      movies: Some(vec![CollectionMovie::default()]),
      ..Collection::default()
    }]);

    app.populate_movie_collection_table().await;

    assert!(!app.data.radarr_data.collection_movies.items.is_empty());
  }

  #[tokio::test]
  async fn test_populate_movie_collection_table_filtered() {
    let mut app = App::test_default();
    app
      .data
      .radarr_data
      .collections
      .set_filtered_items(vec![Collection {
        movies: Some(vec![CollectionMovie::default()]),
        ..Collection::default()
      }]);

    app.populate_movie_collection_table().await;

    assert!(!app.data.radarr_data.collection_movies.items.is_empty());
  }

  #[tokio::test]
  async fn test_extract_movie_id() {
    let mut app = App::test_default();
    app.data.radarr_data.movies.set_items(vec![Movie {
      id: 1,
      ..Movie::default()
    }]);

    assert_eq!(app.extract_movie_id().await, 1);
  }

  #[tokio::test]
  async fn test_extract_radarr_indexer_id() {
    let mut app = App::test_default();
    app.data.radarr_data.indexers.set_items(vec![Indexer {
      id: 1,
      ..Indexer::default()
    }]);

    assert_eq!(app.extract_radarr_indexer_id().await, 1);
  }

  fn construct_app_unit<'a>() -> (App<'a>, mpsc::Receiver<NetworkEvent>) {
    let (sync_network_tx, sync_network_rx) = mpsc::channel::<NetworkEvent>(500);
    let mut app = App {
      network_tx: Some(sync_network_tx),
      tick_count: 1,
      is_first_render: false,
      ..App::test_default()
    };
    app.data.radarr_data.prompt_confirm = true;

    (app, sync_network_rx)
  }
}
