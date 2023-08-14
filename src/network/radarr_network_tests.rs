#[cfg(test)]
mod test {
  use std::sync::Arc;

  use bimap::BiMap;
  use chrono::{DateTime, Utc};
  use mockito::{Matcher, Mock, Server, ServerGuard};
  use pretty_assertions::{assert_eq, assert_str_eq};
  use rstest::rstest;
  use serde_json::{json, Value};
  use strum::IntoEnumIterator;
  use tokio::sync::Mutex;
  use tokio_util::sync::CancellationToken;

  use crate::models::radarr_models::{
    CollectionMovie, IndexerField, IndexerSelectOption, Language, MediaInfo, MinimumAvailability,
    Monitor, MovieFile, Quality, QualityWrapper, Rating, RatingsList,
  };
  use crate::models::servarr_data::radarr::radarr_data::ActiveRadarrBlock;
  use crate::models::{HorizontallyScrollableText, StatefulTable};
  use crate::App;

  use super::super::*;

  const MOVIE_JSON: &str = r#"{
        "id": 1,
        "title": "Test",
        "tmdbId": 1234,
        "originalLanguage": {
          "name": "English"
        },
        "sizeOnDisk": 3543348019,
        "status": "Downloaded",
        "overview": "Blah blah blah",
        "path": "/nfs/movies",
        "studio": "21st Century Alex",
        "genres": ["cool", "family", "fun"],
        "year": 2023,
        "monitored": true,
        "hasFile": true,
        "runtime": 120,
        "qualityProfileId": 2222,
        "minimumAvailability": "announced",
        "certification": "R",
        "tags": [1],
        "ratings": {
          "imdb": {
            "value": 9.9
          },
          "tmdb": {
            "value": 9.9
          },
          "rottenTomatoes": {
            "value": 9.9
          }
        },
        "movieFile": {
          "relativePath": "Test.mkv",
          "path": "/nfs/movies/Test.mkv",
          "dateAdded": "2022-12-30T07:37:56Z",
          "mediaInfo": {
            "audioBitrate": 0,
            "audioChannels": 7.1,
            "audioCodec": "AAC",
            "audioLanguages": "eng",
            "audioStreamCount": 1,
            "videoBitDepth": 10,
            "videoBitrate": 0,
            "videoCodec": "x265",
            "videoFps": 23.976,
            "resolution": "1920x804",
            "runTime": "2:00:00",
            "scanType": "Progressive"
          }
        },
        "collection": {
          "id": 123,
          "title": "Test Collection",
          "rootFolderPath": "/nfs/movies",
          "searchOnAdd": true,
          "monitored": true,
          "minimumAvailability": "released",
          "overview": "Collection blah blah blah",
          "qualityProfileId": 2222,
          "movies": [
            {
              "title": "Test",
              "overview": "Collection blah blah blah",
              "year": 2023,
              "runtime": 120,
              "tmdbId": 1234,
              "genres": ["cool", "family", "fun"],
              "ratings": {
                "imdb": {
                  "value": 9.9
                },
                "tmdb": {
                  "value": 9.9
                },
                "rottenTomatoes": {
                  "value": 9.9
                }
              }
            }
          ]
        }
      }"#;

  #[rstest]
  fn test_resource_movie(
    #[values(
      RadarrEvent::AddMovie,
      RadarrEvent::GetMovies,
      RadarrEvent::GetMovieDetails,
      RadarrEvent::DeleteMovie
    )]
    event: RadarrEvent,
  ) {
    assert_str_eq!(event.resource(), "/movie");
  }

  #[rstest]
  fn test_resource_release(
    #[values(RadarrEvent::GetReleases, RadarrEvent::DownloadRelease)] event: RadarrEvent,
  ) {
    assert_str_eq!(event.resource(), "/release");
  }

  #[rstest]
  fn test_resource_queue(
    #[values(RadarrEvent::GetDownloads, RadarrEvent::DeleteDownload)] event: RadarrEvent,
  ) {
    assert_str_eq!(event.resource(), "/queue");
  }

  #[rstest]
  fn test_resource_command(
    #[values(
      RadarrEvent::TriggerAutomaticSearch,
      RadarrEvent::UpdateAndScan,
      RadarrEvent::UpdateAllMovies,
      RadarrEvent::UpdateDownloads,
      RadarrEvent::UpdateCollections
    )]
    event: RadarrEvent,
  ) {
    assert_str_eq!(event.resource(), "/command");
  }

  #[rstest]
  fn test_resource(
    #[values(
      RadarrEvent::GetCollections,
      RadarrEvent::SearchNewMovie,
      RadarrEvent::GetMovieCredits,
      RadarrEvent::GetMovieHistory,
      RadarrEvent::GetOverview,
      RadarrEvent::GetQualityProfiles,
      RadarrEvent::GetRootFolders,
      RadarrEvent::GetStatus,
      RadarrEvent::HealthCheck
    )]
    event: RadarrEvent,
  ) {
    let expected_resource = match event {
      RadarrEvent::GetCollections => "/collection",
      RadarrEvent::SearchNewMovie => "/movie/lookup",
      RadarrEvent::GetMovieCredits => "/credit",
      RadarrEvent::GetMovieHistory => "/history/movie",
      RadarrEvent::GetOverview => "/diskspace",
      RadarrEvent::GetQualityProfiles => "/qualityprofile",
      RadarrEvent::GetRootFolders => "/rootfolder",
      RadarrEvent::GetStatus => "/system/status",
      RadarrEvent::HealthCheck => "/health",
      _ => "",
    };

    assert_str_eq!(event.resource(), expected_resource);
  }

  #[test]
  fn test_from_radarr_event() {
    assert_eq!(
      NetworkEvent::Radarr(RadarrEvent::HealthCheck),
      NetworkEvent::from(RadarrEvent::HealthCheck)
    );
  }

  #[tokio::test]
  async fn test_handle_get_healthcheck_event() {
    let (async_server, app_arc, _server) = mock_radarr_api(
      RequestMethod::Get,
      None,
      None,
      RadarrEvent::HealthCheck.resource(),
    )
    .await;
    let mut network = Network::new(&app_arc, CancellationToken::new());

    network.handle_radarr_event(RadarrEvent::HealthCheck).await;

    async_server.assert_async().await;
  }

  #[tokio::test]
  async fn test_handle_get_diskspace_event() {
    let (async_server, app_arc, _server) = mock_radarr_api(
      RequestMethod::Get,
      None,
      Some(json!([
        {
          "freeSpace": 1111,
          "totalSpace": 2222,
        },
        {
          "freeSpace": 3333,
          "totalSpace": 4444
        }
      ])),
      RadarrEvent::GetOverview.resource(),
    )
    .await;
    let mut network = Network::new(&app_arc, CancellationToken::new());

    network.handle_radarr_event(RadarrEvent::GetOverview).await;

    async_server.assert_async().await;
    assert_eq!(
      app_arc.lock().await.data.radarr_data.disk_space_vec,
      vec![
        DiskSpace {
          free_space: Number::from(1111),
          total_space: Number::from(2222),
        },
        DiskSpace {
          free_space: Number::from(3333),
          total_space: Number::from(4444),
        },
      ]
    );
  }

  #[tokio::test]
  async fn test_handle_get_status_event() {
    let (async_server, app_arc, _server) = mock_radarr_api(
      RequestMethod::Get,
      None,
      Some(json!({
        "version": "v1",
        "startTime": "2023-02-25T20:16:43Z"
      })),
      RadarrEvent::GetStatus.resource(),
    )
    .await;
    let mut network = Network::new(&app_arc, CancellationToken::new());

    network.handle_radarr_event(RadarrEvent::GetStatus).await;

    async_server.assert_async().await;
    assert_str_eq!(app_arc.lock().await.data.radarr_data.version, "v1");
    assert_eq!(
      app_arc.lock().await.data.radarr_data.start_time,
      DateTime::from(DateTime::parse_from_rfc3339("2023-02-25T20:16:43Z").unwrap())
        as DateTime<Utc>
    );
  }

  #[tokio::test]
  async fn test_handle_get_movies_event() {
    let (async_server, app_arc, _server) = mock_radarr_api(
      RequestMethod::Get,
      None,
      Some(serde_json::from_str(format!("[ {} ]", MOVIE_JSON).as_str()).unwrap()),
      RadarrEvent::GetMovies.resource(),
    )
    .await;
    let mut network = Network::new(&app_arc, CancellationToken::new());

    network.handle_radarr_event(RadarrEvent::GetMovies).await;

    async_server.assert_async().await;
    assert_eq!(
      app_arc.lock().await.data.radarr_data.movies.items,
      vec![movie()]
    );
  }

  #[tokio::test]
  async fn test_handle_get_releases_event() {
    let release_json = json!([{
      "guid": "1234",
      "protocol": "torrent",
      "age": 1,
      "title": "Test Release",
      "indexer": "kickass torrents",
      "indexerId": 2,
      "size": 1234,
      "rejected": true,
      "rejections": [ "Unknown quality profile", "Release is already mapped" ],
      "seeders": 2,
      "leechers": 1,
      "languages": [ { "name": "English" } ],
      "quality": { "quality": { "name": "HD - 1080p" }}
    }]);
    let resource = format!("{}?movieId=1", RadarrEvent::GetReleases.resource());
    let (async_server, app_arc, _server) =
      mock_radarr_api(RequestMethod::Get, None, Some(release_json), &resource).await;
    app_arc
      .lock()
      .await
      .data
      .radarr_data
      .movies
      .set_items(vec![movie()]);
    app_arc.lock().await.data.radarr_data.movie_details_modal = Some(MovieDetailsModal::default());
    let mut network = Network::new(&app_arc, CancellationToken::new());

    network.handle_radarr_event(RadarrEvent::GetReleases).await;

    async_server.assert_async().await;
    assert_eq!(
      app_arc
        .lock()
        .await
        .data
        .radarr_data
        .movie_details_modal
        .as_ref()
        .unwrap()
        .movie_releases
        .items,
      vec![release()]
    );
  }

  #[tokio::test]
  async fn test_handle_search_new_movie_event() {
    let add_movie_search_result_json = json!([{
      "tmdbId": 1234,
      "title": "Test",
      "originalLanguage": { "name": "English" },
      "status": "released",
      "overview": "New movie blah blah blah",
      "genres": ["cool", "family", "fun"],
      "year": 2023,
      "runtime": 120,
      "ratings": {
        "imdb": {
          "value": 9.9
        },
        "tmdb": {
          "value": 9.9
        },
        "rottenTomatoes": {
          "value": 9.9
        }
      }
    }]);
    let resource = format!(
      "{}?term=test%20term",
      RadarrEvent::SearchNewMovie.resource()
    );
    let (async_server, app_arc, _server) = mock_radarr_api(
      RequestMethod::Get,
      None,
      Some(add_movie_search_result_json),
      &resource,
    )
    .await;
    app_arc.lock().await.data.radarr_data.search = Some("test term".into());
    let mut network = Network::new(&app_arc, CancellationToken::new());

    network
      .handle_radarr_event(RadarrEvent::SearchNewMovie)
      .await;

    async_server.assert_async().await;
    assert!(app_arc
      .lock()
      .await
      .data
      .radarr_data
      .add_searched_movies
      .is_some());
    assert_eq!(
      app_arc
        .lock()
        .await
        .data
        .radarr_data
        .add_searched_movies
        .as_ref()
        .unwrap()
        .items,
      vec![add_movie_search_result()]
    );
  }

  #[tokio::test]
  async fn test_handle_start_task_event() {
    let (async_server, app_arc, _server) = mock_radarr_api(
      RequestMethod::Post,
      Some(json!({
        "name": "TestTask"
      })),
      None,
      RadarrEvent::StartTask.resource(),
    )
    .await;
    app_arc
      .lock()
      .await
      .data
      .radarr_data
      .tasks
      .set_items(vec![Task {
        task_name: "TestTask".to_owned(),
        ..Task::default()
      }]);
    let mut network = Network::new(&app_arc, CancellationToken::new());

    network.handle_radarr_event(RadarrEvent::StartTask).await;

    async_server.assert_async().await;
  }

  #[tokio::test]
  async fn test_handle_search_new_movie_event_no_results() {
    let resource = format!(
      "{}?term=test%20term",
      RadarrEvent::SearchNewMovie.resource()
    );
    let (async_server, app_arc, _server) =
      mock_radarr_api(RequestMethod::Get, None, Some(json!([])), &resource).await;
    app_arc.lock().await.data.radarr_data.search = Some("test term".into());
    let mut network = Network::new(&app_arc, CancellationToken::new());

    network
      .handle_radarr_event(RadarrEvent::SearchNewMovie)
      .await;

    async_server.assert_async().await;
    assert!(app_arc
      .lock()
      .await
      .data
      .radarr_data
      .add_searched_movies
      .is_none());
    assert_eq!(
      app_arc.lock().await.get_current_route(),
      &ActiveRadarrBlock::AddMovieEmptySearchResults.into()
    );
  }

  #[tokio::test]
  async fn test_handle_search_new_movie_event_no_panic_on_race_condition() {
    let resource = format!(
      "{}?term=test%20term",
      RadarrEvent::SearchNewMovie.resource()
    );
    let mut server = Server::new_async().await;
    let mut async_server = server
      .mock(
        &RequestMethod::Get.to_string().to_uppercase(),
        format!("/api/v3{}", resource).as_str(),
      )
      .match_header("X-Api-Key", "test1234");
    async_server = async_server.expect_at_most(0).create_async().await;

    let host = server.host_with_port().split(':').collect::<Vec<&str>>()[0].to_owned();
    let port = Some(
      server.host_with_port().split(':').collect::<Vec<&str>>()[1]
        .parse()
        .unwrap(),
    );
    let mut app = App::default();
    let radarr_config = RadarrConfig {
      host,
      port,
      api_token: "test1234".to_owned(),
    };
    app.config.radarr = radarr_config;
    let app_arc = Arc::new(Mutex::new(app));
    let mut network = Network::new(&app_arc, CancellationToken::new());

    network
      .handle_radarr_event(RadarrEvent::SearchNewMovie)
      .await;

    async_server.assert_async().await;
    assert!(app_arc
      .lock()
      .await
      .data
      .radarr_data
      .add_searched_movies
      .is_none());
    assert_eq!(
      app_arc.lock().await.get_current_route(),
      &ActiveRadarrBlock::Movies.into()
    );
  }

  #[tokio::test]
  async fn test_handle_trigger_automatic_search_event() {
    let (async_server, app_arc, _server) = mock_radarr_api(
      RequestMethod::Post,
      Some(json!({
        "name": "MoviesSearch",
        "movieIds": [ 1 ]
      })),
      None,
      RadarrEvent::TriggerAutomaticSearch.resource(),
    )
    .await;
    app_arc
      .lock()
      .await
      .data
      .radarr_data
      .movies
      .set_items(vec![movie()]);
    let mut network = Network::new(&app_arc, CancellationToken::new());

    network
      .handle_radarr_event(RadarrEvent::TriggerAutomaticSearch)
      .await;

    async_server.assert_async().await;
  }

  #[tokio::test]
  async fn test_handle_update_and_scan_event() {
    let (async_server, app_arc, _server) = mock_radarr_api(
      RequestMethod::Post,
      Some(json!({
        "name": "RefreshMovie",
        "movieIds": [ 1 ]
      })),
      None,
      RadarrEvent::UpdateAndScan.resource(),
    )
    .await;
    app_arc
      .lock()
      .await
      .data
      .radarr_data
      .movies
      .set_items(vec![movie()]);
    let mut network = Network::new(&app_arc, CancellationToken::new());

    network
      .handle_radarr_event(RadarrEvent::UpdateAndScan)
      .await;

    async_server.assert_async().await;
  }

  #[tokio::test]
  async fn test_handle_update_all_movies_event() {
    let (async_server, app_arc, _server) = mock_radarr_api(
      RequestMethod::Post,
      Some(json!({
        "name": "RefreshMovie",
        "movieIds": []
      })),
      None,
      RadarrEvent::UpdateAllMovies.resource(),
    )
    .await;
    let mut network = Network::new(&app_arc, CancellationToken::new());

    network
      .handle_radarr_event(RadarrEvent::UpdateAllMovies)
      .await;

    async_server.assert_async().await;
  }

  #[tokio::test]
  async fn test_handle_update_downloads_event() {
    let (async_server, app_arc, _server) = mock_radarr_api(
      RequestMethod::Post,
      Some(json!({
        "name": "RefreshMonitoredDownloads"
      })),
      None,
      RadarrEvent::UpdateDownloads.resource(),
    )
    .await;
    let mut network = Network::new(&app_arc, CancellationToken::new());

    network
      .handle_radarr_event(RadarrEvent::UpdateDownloads)
      .await;

    async_server.assert_async().await;
  }

  #[tokio::test]
  async fn test_handle_update_indexer_settings_event() {
    let indexer_settings_json = json!({
        "minimumAge": 0,
        "maximumSize": 0,
        "retention": 0,
        "rssSyncInterval": 60,
        "preferIndexerFlags": false,
        "availabilityDelay": 0,
        "allowHardcodedSubs": true,
        "whitelistedHardcodedSubs": "",
        "id": 1
    });
    let (async_server, app_arc, _server) = mock_radarr_api(
      RequestMethod::Put,
      Some(indexer_settings_json),
      None,
      RadarrEvent::UpdateIndexerSettings.resource(),
    )
    .await;

    app_arc.lock().await.data.radarr_data.indexer_settings = Some(indexer_settings());
    let mut network = Network::new(&app_arc, CancellationToken::new());

    network
      .handle_radarr_event(RadarrEvent::UpdateIndexerSettings)
      .await;

    async_server.assert_async().await;
    assert!(app_arc
      .lock()
      .await
      .data
      .radarr_data
      .indexer_settings
      .is_none());
  }

  #[tokio::test]
  async fn test_handle_update_collections_event() {
    let (async_server, app_arc, _server) = mock_radarr_api(
      RequestMethod::Post,
      Some(json!({
        "name": "RefreshCollections"
      })),
      None,
      RadarrEvent::UpdateCollections.resource(),
    )
    .await;
    let mut network = Network::new(&app_arc, CancellationToken::new());

    network
      .handle_radarr_event(RadarrEvent::UpdateCollections)
      .await;

    async_server.assert_async().await;
  }

  #[tokio::test]
  async fn test_handle_get_movie_details_event() {
    let resource = format!("{}/1", RadarrEvent::GetMovieDetails.resource());
    let (async_server, app_arc, _server) = mock_radarr_api(
      RequestMethod::Get,
      None,
      Some(serde_json::from_str(MOVIE_JSON).unwrap()),
      &resource,
    )
    .await;
    app_arc
      .lock()
      .await
      .data
      .radarr_data
      .movies
      .set_items(vec![movie()]);
    app_arc.lock().await.data.radarr_data.quality_profile_map =
      BiMap::from_iter([(2222, "HD - 1080p".to_owned())]);
    let mut network = Network::new(&app_arc, CancellationToken::new());

    network
      .handle_radarr_event(RadarrEvent::GetMovieDetails)
      .await;

    async_server.assert_async().await;
    assert!(app_arc
      .lock()
      .await
      .data
      .radarr_data
      .movie_details_modal
      .is_some());

    let app = app_arc.lock().await;
    let movie_details_modal = app.data.radarr_data.movie_details_modal.as_ref().unwrap();
    assert_str_eq!(
      movie_details_modal.movie_details.get_text(),
      formatdoc!(
        "Title: Test
          Year: 2023
          Runtime: 2h 0m
          Rating: R
          Collection: Test Collection
          Status: Downloaded
          Description: Blah blah blah
          TMDB: 99%
          IMDB: 9.9
          Rotten Tomatoes: 
          Quality Profile: HD - 1080p
          Size: 3.30 GB
          Path: /nfs/movies
          Studio: 21st Century Alex
          Genres: cool, family, fun"
      )
    );
    assert_str_eq!(
      movie_details_modal.file_details,
      formatdoc!(
        "Relative Path: Test.mkv
        Absolute Path: /nfs/movies/Test.mkv
        Size: 3.30 GB
        Date Added: 2022-12-30 07:37:56 UTC"
      )
    );
    assert_str_eq!(
      movie_details_modal.audio_details,
      formatdoc!(
        "Bitrate: 0
        Channels: 7.1
        Codec: AAC
        Languages: eng
        Stream Count: 1"
      )
    );
    assert_str_eq!(
      movie_details_modal.video_details,
      formatdoc!(
        "Bit Depth: 10
        Bitrate: 0
        Codec: x265
        FPS: 23.976
        Resolution: 1920x804
        Scan Type: Progressive
        Runtime: 2:00:00"
      )
    );
  }

  #[tokio::test]
  async fn test_handle_get_movie_details_event_empty_options_give_correct_defaults() {
    let movie_json_with_missing_fields = json!({
      "id": 1,
      "title": "Test",
      "originalLanguage": {
        "name": "English"
      },
      "sizeOnDisk": 0,
      "status": "Downloaded",
      "overview": "Blah blah blah",
      "path": "/nfs/movies",
      "studio": "21st Century Alex",
      "genres": ["cool", "family", "fun"],
      "year": 2023,
      "monitored": true,
      "hasFile": false,
      "runtime": 120,
      "tmdbId": 1234,
      "qualityProfileId": 2222,
      "tags": [1],
      "minimumAvailability": "released",
      "ratings": {}
    });
    let resource = format!("{}/1", RadarrEvent::GetMovieDetails.resource());
    let (async_server, app_arc, _server) = mock_radarr_api(
      RequestMethod::Get,
      None,
      Some(movie_json_with_missing_fields),
      &resource,
    )
    .await;
    app_arc
      .lock()
      .await
      .data
      .radarr_data
      .movies
      .set_items(vec![movie()]);
    app_arc.lock().await.data.radarr_data.quality_profile_map =
      BiMap::from_iter([(2222, "HD - 1080p".to_owned())]);
    let mut network = Network::new(&app_arc, CancellationToken::new());

    network
      .handle_radarr_event(RadarrEvent::GetMovieDetails)
      .await;

    async_server.assert_async().await;
    assert!(app_arc
      .lock()
      .await
      .data
      .radarr_data
      .movie_details_modal
      .is_some());

    let app = app_arc.lock().await;
    let movie_details_modal = app.data.radarr_data.movie_details_modal.as_ref().unwrap();
    assert_str_eq!(
      movie_details_modal.movie_details.get_text(),
      formatdoc!(
        "Title: Test
          Year: 2023
          Runtime: 2h 0m
          Rating: 
          Collection: 
          Status: Missing
          Description: Blah blah blah
          TMDB: 
          IMDB: 
          Rotten Tomatoes: 
          Quality Profile: HD - 1080p
          Size: 0.00 GB
          Path: /nfs/movies
          Studio: 21st Century Alex
          Genres: cool, family, fun"
      )
    );
    assert!(movie_details_modal.file_details.is_empty());
    assert!(movie_details_modal.audio_details.is_empty());
    assert!(movie_details_modal.video_details.is_empty());
  }

  #[tokio::test]
  async fn test_handle_get_movie_history_event() {
    let movie_history_item_json = json!([{
      "sourceTitle": "Test",
      "quality": { "quality": { "name": "HD - 1080p" }},
      "languages": [ { "name": "English" } ],
      "date": "2022-12-30T07:37:56Z",
      "eventType": "grabbed"
    }]);
    let resource = format!("{}?movieId=1", RadarrEvent::GetMovieHistory.resource());
    let (async_server, app_arc, _server) = mock_radarr_api(
      RequestMethod::Get,
      None,
      Some(movie_history_item_json),
      &resource,
    )
    .await;
    app_arc
      .lock()
      .await
      .data
      .radarr_data
      .movies
      .set_items(vec![movie()]);
    app_arc.lock().await.data.radarr_data.movie_details_modal = Some(MovieDetailsModal::default());
    let mut network = Network::new(&app_arc, CancellationToken::new());

    network
      .handle_radarr_event(RadarrEvent::GetMovieHistory)
      .await;

    async_server.assert_async().await;
    assert_eq!(
      app_arc
        .lock()
        .await
        .data
        .radarr_data
        .movie_details_modal
        .as_ref()
        .unwrap()
        .movie_history
        .items,
      vec![movie_history_item()]
    );
  }

  #[tokio::test]
  async fn test_handle_get_collections_event() {
    let collection_json = json!([{
      "id": 123,
      "title": "Test Collection",
      "rootFolderPath": "/nfs/movies",
      "searchOnAdd": true,
      "monitored": true,
      "minimumAvailability": "released",
      "overview": "Collection blah blah blah",
      "qualityProfileId": 2222,
      "movies": [{
        "title": "Test",
        "overview": "Collection blah blah blah",
        "year": 2023,
        "runtime": 120,
        "tmdbId": 1234,
        "genres": ["cool", "family", "fun"],
        "ratings": {
          "imdb": {
            "value": 9.9
          },
          "tmdb": {
            "value": 9.9
          },
          "rottenTomatoes": {
            "value": 9.9
          }
        }
      }],
    }]);
    let (async_server, app_arc, _server) = mock_radarr_api(
      RequestMethod::Get,
      None,
      Some(collection_json),
      RadarrEvent::GetCollections.resource(),
    )
    .await;
    let mut network = Network::new(&app_arc, CancellationToken::new());

    network
      .handle_radarr_event(RadarrEvent::GetCollections)
      .await;

    async_server.assert_async().await;
    assert_eq!(
      app_arc.lock().await.data.radarr_data.collections.items,
      vec![collection()]
    );
  }

  #[tokio::test]
  async fn test_handle_get_downloads_event() {
    let downloads_response_json = json!({
      "records": [{
        "title": "Test Download Title",
        "status": "downloading",
        "id": 1,
        "movieId": 1,
        "size": 3543348019u64,
        "sizeleft": 1771674009,
        "outputPath": "/nfs/movies/Test",
        "indexer": "kickass torrents",
        "downloadClient": "transmission",
      }]
    });
    let (async_server, app_arc, _server) = mock_radarr_api(
      RequestMethod::Get,
      None,
      Some(downloads_response_json),
      RadarrEvent::GetDownloads.resource(),
    )
    .await;
    let mut network = Network::new(&app_arc, CancellationToken::new());

    network.handle_radarr_event(RadarrEvent::GetDownloads).await;

    async_server.assert_async().await;
    assert_eq!(
      app_arc.lock().await.data.radarr_data.downloads.items,
      downloads_response().records
    );
  }

  #[tokio::test]
  async fn test_handle_get_indexers_event() {
    let indexers_response_json = json!([{
        "enableRss": true,
        "enableAutomaticSearch": true,
        "enableInteractiveSearch": true,
        "supportsRss": true,
        "supportsSearch": true,
        "protocol": "torrent",
        "priority": 25,
        "downloadClientId": 0,
        "name": "Test Indexer",
        "fields": [
            {
                "order": 0,
                "name": "valueIsString",
                "label": "Value Is String",
                "value": "hello",
                "type": "textbox",
            },
            {
                "order": 1,
                "name": "emptyValueWithSelectOptions",
                "label": "Empty Value With Select Options",
                "type": "select",
                "selectOptions": [
                    {
                        "value": -2,
                        "name": "Original",
                        "order": 0,
                    }
                ]
            },
            {
                "order": 2,
                "name": "valueIsAnArray",
                "label": "Value is an array",
                "value": [1, 2],
                "type": "select",
            },
        ],
        "implementationName": "Torznab",
        "implementation": "Torznab",
        "configContract": "TorznabSettings",
        "tags": ["test_tag"],
        "id": 1
    }]);
    let (async_server, app_arc, _server) = mock_radarr_api(
      RequestMethod::Get,
      None,
      Some(indexers_response_json),
      RadarrEvent::GetIndexers.resource(),
    )
    .await;
    let mut network = Network::new(&app_arc, CancellationToken::new());

    network.handle_radarr_event(RadarrEvent::GetIndexers).await;

    async_server.assert_async().await;
    assert_eq!(
      app_arc.lock().await.data.radarr_data.indexers.items,
      vec![indexer()]
    );
  }

  #[tokio::test]
  async fn test_handle_get_indexer_settings_event() {
    let indexer_settings_response_json = json!({
        "minimumAge": 0,
        "maximumSize": 0,
        "retention": 0,
        "rssSyncInterval": 60,
        "preferIndexerFlags": false,
        "availabilityDelay": 0,
        "allowHardcodedSubs": true,
        "whitelistedHardcodedSubs": "",
        "id": 1
    });
    let (async_server, app_arc, _server) = mock_radarr_api(
      RequestMethod::Get,
      None,
      Some(indexer_settings_response_json),
      RadarrEvent::GetIndexerSettings.resource(),
    )
    .await;
    let mut network = Network::new(&app_arc, CancellationToken::new());

    network
      .handle_radarr_event(RadarrEvent::GetIndexerSettings)
      .await;

    async_server.assert_async().await;
    assert_eq!(
      app_arc.lock().await.data.radarr_data.indexer_settings,
      Some(indexer_settings())
    );
  }

  #[tokio::test]
  async fn test_handle_get_queued_events_event() {
    let queued_events_json = json!([{
        "name": "RefreshMonitoredDownloads",
        "commandName": "Refresh Monitored Downloads",
        "status": "completed",
        "queued": "2023-05-20T21:29:16Z",
        "started": "2023-05-20T21:29:16Z",
        "ended": "2023-05-20T21:29:16Z",
        "duration": "00:00:00.5111547",
        "trigger": "scheduled",
    }]);
    let timestamp = DateTime::from(DateTime::parse_from_rfc3339("2023-05-20T21:29:16Z").unwrap());
    let expected_event = QueueEvent {
      name: "RefreshMonitoredDownloads".to_owned(),
      command_name: "Refresh Monitored Downloads".to_owned(),
      status: "completed".to_owned(),
      queued: timestamp,
      started: Some(timestamp),
      ended: Some(timestamp),
      duration: Some("00:00:00.5111547".to_owned()),
      trigger: "scheduled".to_owned(),
    };

    let (async_server, app_arc, _server) = mock_radarr_api(
      RequestMethod::Get,
      None,
      Some(queued_events_json),
      RadarrEvent::GetQueuedEvents.resource(),
    )
    .await;
    let mut network = Network::new(&app_arc, CancellationToken::new());

    network
      .handle_radarr_event(RadarrEvent::GetQueuedEvents)
      .await;

    async_server.assert_async().await;
    assert_eq!(
      app_arc.lock().await.data.radarr_data.queued_events.items,
      vec![expected_event]
    );
  }

  #[tokio::test]
  async fn test_handle_get_logs_event() {
    let resource = format!(
      "{}?pageSize=500&sortDirection=descending&sortKey=time",
      RadarrEvent::GetLogs.resource()
    );
    let expected_logs = vec![
      HorizontallyScrollableText::from(
        "2023-05-20 21:29:16 UTC|FATAL|RadarrError|Some.Big.Bad.Exception|test exception",
      ),
      HorizontallyScrollableText::from("2023-05-20 21:29:16 UTC|INFO|TestLogger|test message"),
    ];
    let (async_server, app_arc, _server) = mock_radarr_api(
      RequestMethod::Get,
      None,
      Some(json!({
        "page": 1,
        "pageSize": 500,
        "sortKey": "time",
        "sortDirection": "descending",
        "totalRecords": 2,
        "records": [
            {
                "time": "2023-05-20T21:29:16Z",
                "level": "info",
                "logger": "TestLogger",
                "message": "test message",
                "id": 1
            },
            {
                "time": "2023-05-20T21:29:16Z",
                "level": "fatal",
                "logger": "RadarrError",
                "exception": "test exception",
                "exceptionType": "Some.Big.Bad.Exception",
                "id": 2
            }
          ]
      })),
      &resource,
    )
    .await;
    let mut network = Network::new(&app_arc, CancellationToken::new());

    network.handle_radarr_event(RadarrEvent::GetLogs).await;

    async_server.assert_async().await;
    assert_eq!(
      app_arc.lock().await.data.radarr_data.logs.items,
      expected_logs
    );
    assert!(app_arc
      .lock()
      .await
      .data
      .radarr_data
      .logs
      .current_selection()
      .text
      .contains("INFO"));
  }

  #[tokio::test]
  async fn test_handle_get_quality_profiles_event() {
    let quality_profile_json = json!([{
      "id": 2222,
      "name": "HD - 1080p"
    }]);
    let (async_server, app_arc, _server) = mock_radarr_api(
      RequestMethod::Get,
      None,
      Some(quality_profile_json),
      RadarrEvent::GetQualityProfiles.resource(),
    )
    .await;
    let mut network = Network::new(&app_arc, CancellationToken::new());

    network
      .handle_radarr_event(RadarrEvent::GetQualityProfiles)
      .await;

    async_server.assert_async().await;
    assert_eq!(
      app_arc.lock().await.data.radarr_data.quality_profile_map,
      BiMap::from_iter([(2222u64, "HD - 1080p".to_owned())])
    );
  }

  #[tokio::test]
  async fn test_handle_get_tags_event() {
    let tags_json = json!([{
      "id": 2222,
      "label": "usenet"
    }]);
    let (async_server, app_arc, _server) = mock_radarr_api(
      RequestMethod::Get,
      None,
      Some(tags_json),
      RadarrEvent::GetTags.resource(),
    )
    .await;
    let mut network = Network::new(&app_arc, CancellationToken::new());

    network.handle_radarr_event(RadarrEvent::GetTags).await;

    async_server.assert_async().await;
    assert_eq!(
      app_arc.lock().await.data.radarr_data.tags_map,
      BiMap::from_iter([(2222u64, "usenet".to_owned())])
    );
  }

  #[tokio::test]
  async fn test_handle_get_tasks_event() {
    let tasks_json = json!([{
        "name": "Application Check Update",
        "taskName": "ApplicationCheckUpdate",
        "interval": 360,
        "lastExecution": "2023-05-20T21:29:16Z",
        "nextExecution": "2023-05-20T21:29:16Z",
        "lastDuration": "00:00:00.5111547",
    },
    {
        "name": "Backup",
        "taskName": "Backup",
        "interval": 10080,
        "lastExecution": "2023-05-20T21:29:16Z",
        "nextExecution": "2023-05-20T21:29:16Z",
        "lastDuration": "00:00:00.5111547",
    }]);
    let timestamp = DateTime::from(DateTime::parse_from_rfc3339("2023-05-20T21:29:16Z").unwrap());
    let expected_tasks = vec![
      Task {
        name: "Application Check Update".to_owned(),
        task_name: "ApplicationCheckUpdate".to_owned(),
        interval: Number::from(360),
        last_execution: timestamp,
        next_execution: timestamp,
        last_duration: "00:00:00.5111547".to_owned(),
      },
      Task {
        name: "Backup".to_owned(),
        task_name: "Backup".to_owned(),
        interval: Number::from(10080),
        last_execution: timestamp,
        next_execution: timestamp,
        last_duration: "00:00:00.5111547".to_owned(),
      },
    ];
    let (async_server, app_arc, _server) = mock_radarr_api(
      RequestMethod::Get,
      None,
      Some(tasks_json),
      RadarrEvent::GetTasks.resource(),
    )
    .await;
    let mut network = Network::new(&app_arc, CancellationToken::new());

    network.handle_radarr_event(RadarrEvent::GetTasks).await;

    async_server.assert_async().await;
    assert_eq!(
      app_arc.lock().await.data.radarr_data.tasks.items,
      expected_tasks
    );
  }

  #[tokio::test]
  async fn test_handle_get_updates_event() {
    let tasks_json = json!([{
      "version": "4.3.2.1",
      "releaseDate": "2023-04-15T02:02:53Z",
      "installed": true,
      "installedOn": "2023-04-15T02:02:53Z",
      "latest": true,
      "changes": {
        "new": [
          "Cool new thing"
        ],
        "fixed": [
          "Some bugs killed"
        ]
      },
    },
      {
        "version": "3.2.1.0",
        "releaseDate": "2023-04-15T02:02:53Z",
        "installed": false,
        "installedOn": "2023-04-15T02:02:53Z",
        "latest": false,
        "changes": {
          "new": [
            "Cool new thing (old)",
            "Other cool new thing (old)"
            ],
        },
    },
    {
      "version": "2.1.0",
      "releaseDate": "2023-04-15T02:02:53Z",
      "installed": false,
      "latest": false,
      "changes": {
        "fixed": [
          "Killed bug 1",
          "Fixed bug 2"
        ]
      },
    }]);
    let line_break = "-".repeat(200);
    let expected_text = ScrollableText::with_string(formatdoc!(
      "
    The latest version of Radarr is already installed

    4.3.2.1 - 2023-04-15 02:02:53 UTC (Currently Installed)
    {}
    New:
      * Cool new thing
    Fixed:
      * Some bugs killed
    
    
    3.2.1.0 - 2023-04-15 02:02:53 UTC (Previously Installed)
    {}
    New:
      * Cool new thing (old)
      * Other cool new thing (old)
    
    
    2.1.0 - 2023-04-15 02:02:53 UTC 
    {}
    Fixed:
      * Killed bug 1
      * Fixed bug 2",
      line_break.clone(),
      line_break.clone(),
      line_break
    ));
    let (async_server, app_arc, _server) = mock_radarr_api(
      RequestMethod::Get,
      None,
      Some(tasks_json),
      RadarrEvent::GetUpdates.resource(),
    )
    .await;
    let mut network = Network::new(&app_arc, CancellationToken::new());

    network.handle_radarr_event(RadarrEvent::GetUpdates).await;

    async_server.assert_async().await;
    assert_str_eq!(
      app_arc.lock().await.data.radarr_data.updates.get_text(),
      expected_text.get_text()
    );
  }

  #[tokio::test]
  async fn test_add_tag() {
    let (async_server, app_arc, _server) = mock_radarr_api(
      RequestMethod::Post,
      Some(json!({ "label": "testing" })),
      Some(json!({ "id": 3, "label": "testing" })),
      RadarrEvent::GetTags.resource(),
    )
    .await;
    app_arc.lock().await.data.radarr_data.tags_map =
      BiMap::from_iter([(1, "usenet".to_owned()), (2, "test".to_owned())]);
    let mut network = Network::new(&app_arc, CancellationToken::new());

    network.add_tag("testing".to_owned()).await;

    async_server.assert_async().await;
    assert_eq!(
      app_arc.lock().await.data.radarr_data.tags_map,
      BiMap::from_iter([
        (1, "usenet".to_owned()),
        (2, "test".to_owned()),
        (3, "testing".to_owned())
      ])
    );
  }

  #[tokio::test]
  async fn test_handle_get_root_folders_event() {
    let root_folder_json = json!([{
      "id": 1,
      "path": "/nfs",
      "accessible": true,
      "freeSpace": 219902325555200u64,
    }]);
    let (async_server, app_arc, _server) = mock_radarr_api(
      RequestMethod::Get,
      None,
      Some(root_folder_json),
      RadarrEvent::GetRootFolders.resource(),
    )
    .await;
    let mut network = Network::new(&app_arc, CancellationToken::new());

    network
      .handle_radarr_event(RadarrEvent::GetRootFolders)
      .await;

    async_server.assert_async().await;
    assert_eq!(
      app_arc.lock().await.data.radarr_data.root_folders.items,
      vec![root_folder()]
    );
  }

  #[tokio::test]
  async fn test_handle_get_movie_credits_event() {
    let credits_json = json!([
        {
          "personName": "Madison Clarke",
          "character": "Johnny Blaze",
          "type": "cast",
        },
        {
          "personName": "Alex Clarke",
          "department": "Music",
          "job": "Composition",
          "type": "crew",
        }
    ]);
    let resource = format!("{}?movieId=1", RadarrEvent::GetMovieCredits.resource());
    let (async_server, app_arc, _server) =
      mock_radarr_api(RequestMethod::Get, None, Some(credits_json), &resource).await;
    app_arc
      .lock()
      .await
      .data
      .radarr_data
      .movies
      .set_items(vec![movie()]);
    app_arc.lock().await.data.radarr_data.movie_details_modal = Some(MovieDetailsModal::default());
    let mut network = Network::new(&app_arc, CancellationToken::new());

    network
      .handle_radarr_event(RadarrEvent::GetMovieCredits)
      .await;

    let app = app_arc.lock().await;
    let movie_details_modal = app.data.radarr_data.movie_details_modal.as_ref().unwrap();

    async_server.assert_async().await;
    assert_eq!(movie_details_modal.movie_cast.items, vec![cast_credit()]);
    assert_eq!(movie_details_modal.movie_crew.items, vec![crew_credit()]);
  }

  #[tokio::test]
  async fn test_handle_delete_movie_event() {
    let resource = format!(
      "{}/1?deleteFiles=true&addImportExclusion=true",
      RadarrEvent::DeleteMovie.resource()
    );
    let (async_server, app_arc, _server) =
      mock_radarr_api(RequestMethod::Delete, None, None, &resource).await;
    {
      let mut app = app_arc.lock().await;
      app.data.radarr_data.movies.set_items(vec![movie()]);
      app.data.radarr_data.delete_movie_files = true;
      app.data.radarr_data.add_list_exclusion = true;
    }
    let mut network = Network::new(&app_arc, CancellationToken::new());

    network.handle_radarr_event(RadarrEvent::DeleteMovie).await;

    async_server.assert_async().await;
    assert!(!app_arc.lock().await.data.radarr_data.delete_movie_files);
    assert!(!app_arc.lock().await.data.radarr_data.add_list_exclusion);
  }

  #[tokio::test]
  async fn test_handle_delete_download_event() {
    let resource = format!("{}/1", RadarrEvent::DeleteDownload.resource());
    let (async_server, app_arc, _server) =
      mock_radarr_api(RequestMethod::Delete, None, None, &resource).await;
    app_arc
      .lock()
      .await
      .data
      .radarr_data
      .downloads
      .set_items(vec![download_record()]);
    let mut network = Network::new(&app_arc, CancellationToken::new());

    network
      .handle_radarr_event(RadarrEvent::DeleteDownload)
      .await;

    async_server.assert_async().await;
  }

  #[tokio::test]
  async fn test_handle_delete_indexer_event() {
    let resource = format!("{}/1", RadarrEvent::DeleteIndexer.resource());
    let (async_server, app_arc, _server) =
      mock_radarr_api(RequestMethod::Delete, None, None, &resource).await;
    app_arc
      .lock()
      .await
      .data
      .radarr_data
      .indexers
      .set_items(vec![indexer()]);
    let mut network = Network::new(&app_arc, CancellationToken::new());

    network
      .handle_radarr_event(RadarrEvent::DeleteIndexer)
      .await;

    async_server.assert_async().await;
  }

  #[tokio::test]
  async fn test_handle_delete_root_folder_event() {
    let resource = format!("{}/1", RadarrEvent::DeleteRootFolder.resource());
    let (async_server, app_arc, _server) =
      mock_radarr_api(RequestMethod::Delete, None, None, &resource).await;
    app_arc
      .lock()
      .await
      .data
      .radarr_data
      .root_folders
      .set_items(vec![root_folder()]);
    let mut network = Network::new(&app_arc, CancellationToken::new());

    network
      .handle_radarr_event(RadarrEvent::DeleteRootFolder)
      .await;

    async_server.assert_async().await;
  }

  #[rstest]
  #[tokio::test]
  async fn test_handle_add_movie_event(#[values(true, false)] movie_details_context: bool) {
    let (async_server, app_arc, _server) = mock_radarr_api(
      RequestMethod::Post,
      Some(json!({
        "tmdbId": 1234,
        "title": "Test",
        "rootFolderPath": "/nfs2",
        "minimumAvailability": "announced",
        "monitored": true,
        "qualityProfileId": 2222,
        "tags": [1, 2],
        "addOptions": {
          "monitor": "movieOnly",
          "searchForMovie": true
        }
      })),
      None,
      RadarrEvent::AddMovie.resource(),
    )
    .await;

    {
      let mut app = app_arc.lock().await;
      let mut add_movie_modal = AddMovieModal {
        tags: "usenet, testing".into(),
        ..AddMovieModal::default()
      };
      add_movie_modal.root_folder_list.set_items(vec![
        RootFolder {
          id: Number::from(1),
          path: "/nfs".to_owned(),
          accessible: true,
          free_space: Number::from(219902325555200u64),
          unmapped_folders: None,
        },
        RootFolder {
          id: Number::from(2),
          path: "/nfs2".to_owned(),
          accessible: true,
          free_space: Number::from(21990232555520u64),
          unmapped_folders: None,
        },
      ]);
      add_movie_modal.root_folder_list.state.select(Some(1));
      add_movie_modal
        .quality_profile_list
        .set_items(vec!["HD - 1080p".to_owned()]);
      add_movie_modal
        .monitor_list
        .set_items(Vec::from_iter(Monitor::iter()));
      add_movie_modal
        .minimum_availability_list
        .set_items(Vec::from_iter(MinimumAvailability::iter()));
      app.data.radarr_data.add_movie_modal = Some(add_movie_modal);
      app.data.radarr_data.quality_profile_map =
        BiMap::from_iter([(2222, "HD - 1080p".to_owned())]);
      app.data.radarr_data.tags_map =
        BiMap::from_iter([(1, "usenet".to_owned()), (2, "testing".to_owned())]);
      if movie_details_context {
        app
          .data
          .radarr_data
          .collection_movies
          .set_items(vec![collection_movie()]);
        app.push_navigation_stack(ActiveRadarrBlock::CollectionDetails.into());
      } else {
        let mut add_searched_movies = StatefulTable::default();
        add_searched_movies.set_items(vec![add_movie_search_result()]);
        app.data.radarr_data.add_searched_movies = Some(add_searched_movies);
      }
    }
    let mut network = Network::new(&app_arc, CancellationToken::new());

    network.handle_radarr_event(RadarrEvent::AddMovie).await;

    async_server.assert_async().await;
    assert!(app_arc
      .lock()
      .await
      .data
      .radarr_data
      .add_movie_modal
      .is_none());
  }

  #[tokio::test]
  async fn test_handle_add_movie_event_reuse_existing_table_if_search_already_performed() {
    let (async_server, app_arc, _server) = mock_radarr_api(
      RequestMethod::Post,
      Some(json!({
        "tmdbId": 5678,
        "title": "Test",
        "rootFolderPath": "/nfs2",
        "minimumAvailability": "announced",
        "monitored": true,
        "qualityProfileId": 2222,
        "tags": [1, 2],
        "addOptions": {
          "monitor": "movieOnly",
          "searchForMovie": true
        }
      })),
      None,
      RadarrEvent::AddMovie.resource(),
    )
    .await;

    {
      let mut app = app_arc.lock().await;
      let mut add_movie_modal = AddMovieModal {
        tags: "usenet, testing".into(),
        ..AddMovieModal::default()
      };
      add_movie_modal.root_folder_list.set_items(vec![
        RootFolder {
          id: Number::from(1),
          path: "/nfs".to_owned(),
          accessible: true,
          free_space: Number::from(219902325555200u64),
          unmapped_folders: None,
        },
        RootFolder {
          id: Number::from(2),
          path: "/nfs2".to_owned(),
          accessible: true,
          free_space: Number::from(21990232555520u64),
          unmapped_folders: None,
        },
      ]);
      add_movie_modal.root_folder_list.state.select(Some(1));
      add_movie_modal
        .quality_profile_list
        .set_items(vec!["HD - 1080p".to_owned()]);
      add_movie_modal
        .monitor_list
        .set_items(Vec::from_iter(Monitor::iter()));
      add_movie_modal
        .minimum_availability_list
        .set_items(Vec::from_iter(MinimumAvailability::iter()));
      app.data.radarr_data.add_movie_modal = Some(add_movie_modal);
      app.data.radarr_data.quality_profile_map =
        BiMap::from_iter([(2222, "HD - 1080p".to_owned())]);
      app.data.radarr_data.tags_map =
        BiMap::from_iter([(1, "usenet".to_owned()), (2, "testing".to_owned())]);
      let secondary_search_result = AddMovieSearchResult {
        tmdb_id: Number::from(5678),
        ..add_movie_search_result()
      };
      let mut add_searched_movies = StatefulTable::default();
      add_searched_movies.set_items(vec![add_movie_search_result(), secondary_search_result]);
      add_searched_movies.scroll_to_bottom();
      app.data.radarr_data.add_searched_movies = Some(add_searched_movies);
    }
    let mut network = Network::new(&app_arc, CancellationToken::new());

    network.handle_radarr_event(RadarrEvent::AddMovie).await;

    async_server.assert_async().await;
    assert!(app_arc
      .lock()
      .await
      .data
      .radarr_data
      .add_movie_modal
      .is_none());
    assert_eq!(
      app_arc
        .lock()
        .await
        .data
        .radarr_data
        .add_searched_movies
        .as_ref()
        .unwrap()
        .current_selection()
        .tmdb_id
        .as_u64()
        .unwrap(),
      5678
    );
  }

  #[tokio::test]
  async fn test_handle_add_root_folder_event() {
    let (async_server, app_arc, _server) = mock_radarr_api(
      RequestMethod::Post,
      Some(json!({
        "path": "/nfs/test"
      })),
      None,
      RadarrEvent::AddRootFolder.resource(),
    )
    .await;

    app_arc.lock().await.data.radarr_data.edit_root_folder = Some("/nfs/test".into());
    let mut network = Network::new(&app_arc, CancellationToken::new());

    network
      .handle_radarr_event(RadarrEvent::AddRootFolder)
      .await;

    async_server.assert_async().await;
    assert!(app_arc
      .lock()
      .await
      .data
      .radarr_data
      .edit_root_folder
      .is_none());
  }

  #[tokio::test]
  async fn test_handle_edit_movie_event() {
    let mut expected_body: Value = serde_json::from_str(MOVIE_JSON).unwrap();
    *expected_body.get_mut("monitored").unwrap() = json!(false);
    *expected_body.get_mut("minimumAvailability").unwrap() = json!("announced");
    *expected_body.get_mut("qualityProfileId").unwrap() = json!(1111);
    *expected_body.get_mut("path").unwrap() = json!("/nfs/Test Path");
    *expected_body.get_mut("tags").unwrap() = json!([1, 2]);

    let resource = format!("{}/1", RadarrEvent::GetMovieDetails.resource());
    let (async_details_server, app_arc, mut server) = mock_radarr_api(
      RequestMethod::Get,
      None,
      Some(serde_json::from_str(MOVIE_JSON).unwrap()),
      &resource,
    )
    .await;
    let async_edit_server = server
      .mock(
        "PUT",
        format!("/api/v3{}/1", RadarrEvent::EditMovie.resource()).as_str(),
      )
      .with_status(202)
      .match_header("X-Api-Key", "test1234")
      .match_body(Matcher::Json(expected_body))
      .create_async()
      .await;
    {
      let mut app = app_arc.lock().await;
      app.data.radarr_data.tags_map =
        BiMap::from_iter([(1, "usenet".to_owned()), (2, "testing".to_owned())]);
      let mut edit_movie = EditMovieModal {
        tags: "usenet, testing".to_owned().into(),
        path: "/nfs/Test Path".to_owned().into(),
        monitored: Some(false),
        ..EditMovieModal::default()
      };
      edit_movie
        .quality_profile_list
        .set_items(vec!["Any".to_owned(), "HD - 1080p".to_owned()]);
      edit_movie
        .minimum_availability_list
        .set_items(Vec::from_iter(MinimumAvailability::iter()));
      app.data.radarr_data.edit_movie_modal = Some(edit_movie);
      app.data.radarr_data.movies.set_items(vec![Movie {
        monitored: false,
        ..movie()
      }]);
      app.data.radarr_data.quality_profile_map =
        BiMap::from_iter([(1111, "Any".to_owned()), (2222, "HD - 1080p".to_owned())]);
    }
    let mut network = Network::new(&app_arc, CancellationToken::new());

    network.handle_radarr_event(RadarrEvent::EditMovie).await;

    async_details_server.assert_async().await;
    async_edit_server.assert_async().await;

    let app = app_arc.lock().await;
    assert!(app.data.radarr_data.edit_movie_modal.is_none());
  }

  #[tokio::test]
  async fn test_handle_edit_collection_event() {
    let detailed_collection_body = json!({
      "id": 123,
      "title": "Test Collection",
      "rootFolderPath": "/nfs/movies",
      "searchOnAdd": true,
      "monitored": true,
      "minimumAvailability": "released",
      "overview": "Collection blah blah blah",
      "qualityProfileId": 2222,
      "movies": [
        {
          "title": "Test",
          "overview": "Collection blah blah blah",
          "year": 2023,
          "runtime": 120,
          "tmdbId": 1234,
          "genres": ["cool", "family", "fun"],
          "ratings": {
            "imdb": {
              "value": 9.9
            },
            "tmdb": {
              "value": 9.9
            },
            "rottenTomatoes": {
              "value": 9.9
            }
          }
        }
      ]
    });
    let mut expected_body = detailed_collection_body.clone();
    *expected_body.get_mut("monitored").unwrap() = json!(false);
    *expected_body.get_mut("minimumAvailability").unwrap() = json!("announced");
    *expected_body.get_mut("qualityProfileId").unwrap() = json!(1111);
    *expected_body.get_mut("rootFolderPath").unwrap() = json!("/nfs/Test Path");
    *expected_body.get_mut("searchOnAdd").unwrap() = json!(false);

    let resource = format!("{}/123", RadarrEvent::GetCollections.resource());
    let (async_details_server, app_arc, mut server) = mock_radarr_api(
      RequestMethod::Get,
      None,
      Some(detailed_collection_body),
      &resource,
    )
    .await;
    let async_edit_server = server
      .mock(
        "PUT",
        format!("/api/v3{}/123", RadarrEvent::EditCollection.resource()).as_str(),
      )
      .with_status(202)
      .match_header("X-Api-Key", "test1234")
      .match_body(Matcher::Json(expected_body))
      .create_async()
      .await;
    {
      let mut app = app_arc.lock().await;
      let mut edit_collection_modal = EditCollectionModal {
        path: "/nfs/Test Path".into(),
        monitored: Some(false),
        search_on_add: Some(false),
        ..EditCollectionModal::default()
      };
      edit_collection_modal
        .quality_profile_list
        .set_items(vec!["Any".to_owned(), "HD - 1080p".to_owned()]);
      edit_collection_modal
        .minimum_availability_list
        .set_items(Vec::from_iter(MinimumAvailability::iter()));
      app.data.radarr_data.edit_collection_modal = Some(edit_collection_modal);
      app.data.radarr_data.collections.set_items(vec![Collection {
        monitored: false,
        search_on_add: false,
        ..collection()
      }]);
      app.data.radarr_data.quality_profile_map =
        BiMap::from_iter([(1111, "Any".to_owned()), (2222, "HD - 1080p".to_owned())]);
    }
    let mut network = Network::new(&app_arc, CancellationToken::new());

    network
      .handle_radarr_event(RadarrEvent::EditCollection)
      .await;

    async_details_server.assert_async().await;
    async_edit_server.assert_async().await;

    let app = app_arc.lock().await;
    assert!(app.data.radarr_data.edit_collection_modal.is_none());
  }

  #[tokio::test]
  async fn test_handle_download_release_event() {
    let (async_server, app_arc, _server) = mock_radarr_api(
      RequestMethod::Post,
      Some(json!({
        "guid": "1234",
        "indexerId": 2,
        "movieId": 1
      })),
      None,
      RadarrEvent::DownloadRelease.resource(),
    )
    .await;
    let mut movie_details_modal = MovieDetailsModal::default();
    movie_details_modal
      .movie_releases
      .set_items(vec![release()]);
    app_arc.lock().await.data.radarr_data.movie_details_modal = Some(movie_details_modal);
    app_arc
      .lock()
      .await
      .data
      .radarr_data
      .movies
      .set_items(vec![movie()]);
    let mut network = Network::new(&app_arc, CancellationToken::new());

    network
      .handle_radarr_event(RadarrEvent::DownloadRelease)
      .await;

    async_server.assert_async().await;
  }

  #[tokio::test]
  async fn test_extract_and_add_tag_ids_vec() {
    let app_arc = Arc::new(Mutex::new(App::default()));
    let tags = "    test,hi ,, usenet ".to_owned();
    {
      let mut app = app_arc.lock().await;
      app.data.radarr_data.tags_map = BiMap::from_iter([
        (1, "usenet".to_owned()),
        (2, "test".to_owned()),
        (3, "hi".to_owned()),
      ]);
    }
    let mut network = Network::new(&app_arc, CancellationToken::new());

    assert_eq!(
      network.extract_and_add_tag_ids_vec(tags).await,
      vec![2, 3, 1]
    );
  }

  #[tokio::test]
  async fn test_extract_and_add_tag_ids_vec_add_missing_tags_first() {
    let (async_server, app_arc, _server) = mock_radarr_api(
      RequestMethod::Post,
      Some(json!({ "label": "testing" })),
      Some(json!({ "id": 3, "label": "testing" })),
      RadarrEvent::GetTags.resource(),
    )
    .await;
    let tags = "usenet, test, testing".to_owned();
    {
      let mut app = app_arc.lock().await;
      app.data.radarr_data.edit_movie_modal = Some(EditMovieModal {
        tags: tags.clone().into(),
        ..EditMovieModal::default()
      });
      app.data.radarr_data.tags_map =
        BiMap::from_iter([(1, "usenet".to_owned()), (2, "test".to_owned())]);
    }
    let mut network = Network::new(&app_arc, CancellationToken::new());

    let tag_ids_vec = network.extract_and_add_tag_ids_vec(tags).await;

    async_server.assert_async().await;
    assert_eq!(tag_ids_vec, vec![1, 2, 3]);
    assert_eq!(
      app_arc.lock().await.data.radarr_data.tags_map,
      BiMap::from_iter([
        (1, "usenet".to_owned()),
        (2, "test".to_owned()),
        (3, "testing".to_owned())
      ])
    );
  }

  #[tokio::test]
  async fn test_extract_movie_id() {
    let app_arc = Arc::new(Mutex::new(App::default()));
    app_arc
      .lock()
      .await
      .data
      .radarr_data
      .movies
      .set_items(vec![Movie {
        id: Number::from(1),
        tmdb_id: Number::from(2),
        ..Movie::default()
      }]);
    let mut network = Network::new(&app_arc, CancellationToken::new());

    assert_eq!(network.extract_movie_id().await, (1, 2));
  }

  #[tokio::test]
  async fn test_extract_movie_id_filtered_movies() {
    let app_arc = Arc::new(Mutex::new(App::default()));
    let mut filtered_movies = StatefulTable::default();
    filtered_movies.set_items(vec![Movie {
      id: Number::from(1),
      tmdb_id: Number::from(2),
      ..Movie::default()
    }]);
    app_arc.lock().await.data.radarr_data.filtered_movies = Some(filtered_movies);
    let mut network = Network::new(&app_arc, CancellationToken::new());

    assert_eq!(network.extract_movie_id().await, (1, 2));
  }

  #[tokio::test]
  async fn test_extract_collection_id() {
    let app_arc = Arc::new(Mutex::new(App::default()));
    app_arc
      .lock()
      .await
      .data
      .radarr_data
      .collections
      .set_items(vec![Collection {
        id: Number::from(1),
        ..Collection::default()
      }]);
    let mut network = Network::new(&app_arc, CancellationToken::new());

    assert_eq!(network.extract_collection_id().await, 1);
  }

  #[tokio::test]
  async fn test_extract_collection_id_filtered_collection() {
    let app_arc = Arc::new(Mutex::new(App::default()));
    let mut filtered_collections = StatefulTable::default();
    filtered_collections.set_items(vec![Collection {
      id: Number::from(1),
      ..Collection::default()
    }]);
    app_arc.lock().await.data.radarr_data.filtered_collections = Some(filtered_collections);
    let mut network = Network::new(&app_arc, CancellationToken::new());

    assert_eq!(network.extract_collection_id().await, 1);
  }

  #[tokio::test]
  async fn test_append_movie_id_param() {
    let app_arc = Arc::new(Mutex::new(App::default()));
    app_arc
      .lock()
      .await
      .data
      .radarr_data
      .movies
      .set_items(vec![Movie {
        id: Number::from(1),
        ..Movie::default()
      }]);
    let mut network = Network::new(&app_arc, CancellationToken::new());

    assert_str_eq!(
      network.append_movie_id_param("/test").await,
      "/test?movieId=1"
    );
  }

  #[tokio::test]
  async fn test_radarr_request_props_from_default_radarr_config() {
    let app_arc = Arc::new(Mutex::new(App::default()));
    let network = Network::new(&app_arc, CancellationToken::new());

    let request_props = network
      .radarr_request_props_from("/test", RequestMethod::Get, None::<()>)
      .await;

    assert_str_eq!(request_props.uri, "http://localhost:7878/api/v3/test");
    assert_eq!(request_props.method, RequestMethod::Get);
    assert_eq!(request_props.body, None);
    assert!(request_props.api_token.is_empty());

    app_arc.lock().await.config.radarr = RadarrConfig {
      host: "192.168.0.123".to_owned(),
      port: Some(8080),
      api_token: "testToken1234".to_owned(),
    };
  }

  #[tokio::test]
  async fn test_radarr_request_props_from_custom_radarr_config() {
    let api_token = "testToken1234".to_owned();
    let app_arc = Arc::new(Mutex::new(App::default()));
    app_arc.lock().await.config.radarr = RadarrConfig {
      host: "192.168.0.123".to_owned(),
      port: Some(8080),
      api_token: api_token.clone(),
    };
    let network = Network::new(&app_arc, CancellationToken::new());

    let request_props = network
      .radarr_request_props_from("/test", RequestMethod::Get, None::<()>)
      .await;

    assert_str_eq!(request_props.uri, "http://192.168.0.123:8080/api/v3/test");
    assert_eq!(request_props.method, RequestMethod::Get);
    assert_eq!(request_props.body, None);
    assert_str_eq!(request_props.api_token, api_token);
  }

  #[test]
  fn test_get_movie_status_downloaded() {
    assert_str_eq!(get_movie_status(true, &[], Number::from(0)), "Downloaded");
  }

  #[test]
  fn test_get_movie_status_missing() {
    let download_record = DownloadRecord {
      movie_id: 1.into(),
      ..DownloadRecord::default()
    };

    assert_str_eq!(
      get_movie_status(false, &[download_record.clone()], 0.into()),
      "Missing"
    );

    assert_str_eq!(
      get_movie_status(false, &[download_record], 1.into()),
      "Missing"
    );
  }

  #[test]
  fn test_get_movie_status_downloading() {
    assert_str_eq!(
      get_movie_status(
        false,
        &[DownloadRecord {
          movie_id: 1.into(),
          status: "downloading".to_owned(),
          ..DownloadRecord::default()
        }],
        1.into()
      ),
      "Downloading"
    );
  }

  #[test]
  fn test_get_movie_status_awaiting_import() {
    assert_str_eq!(
      get_movie_status(
        false,
        &[DownloadRecord {
          movie_id: 1.into(),
          status: "completed".to_owned(),
          ..DownloadRecord::default()
        }],
        1.into()
      ),
      "Awaiting Import"
    );
  }

  async fn mock_radarr_api(
    method: RequestMethod,
    request_body: Option<Value>,
    response_body: Option<Value>,
    resource: &str,
  ) -> (Mock, Arc<Mutex<App<'_>>>, ServerGuard) {
    let mut server = Server::new_async().await;
    let mut async_server = server
      .mock(
        &method.to_string().to_uppercase(),
        format!("/api/v3{}", resource).as_str(),
      )
      .match_header("X-Api-Key", "test1234");

    if let Some(body) = request_body {
      async_server = async_server.match_body(Matcher::Json(body));
    }

    if let Some(body) = response_body {
      async_server = async_server.with_body(body.to_string());
    }

    async_server = async_server.create_async().await;

    let host = server.host_with_port().split(':').collect::<Vec<&str>>()[0].to_owned();
    let port = Some(
      server.host_with_port().split(':').collect::<Vec<&str>>()[1]
        .parse()
        .unwrap(),
    );
    let mut app = App::default();
    let radarr_config = RadarrConfig {
      host,
      port,
      api_token: "test1234".to_owned(),
    };
    app.config.radarr = radarr_config;
    let app_arc = Arc::new(Mutex::new(app));

    (async_server, app_arc, server)
  }

  fn language() -> Language {
    Language {
      name: "English".to_owned(),
    }
  }

  fn genres() -> Vec<String> {
    vec!["cool".to_owned(), "family".to_owned(), "fun".to_owned()]
  }

  fn rating() -> Rating {
    Rating {
      value: Number::from_f64(9.9).unwrap(),
    }
  }

  fn ratings_list() -> RatingsList {
    RatingsList {
      imdb: Some(rating()),
      tmdb: Some(rating()),
      rotten_tomatoes: Some(rating()),
    }
  }

  fn media_info() -> MediaInfo {
    MediaInfo {
      audio_bitrate: Number::from(0),
      audio_channels: Number::from_f64(7.1).unwrap(),
      audio_codec: Some("AAC".to_owned()),
      audio_languages: Some("eng".to_owned()),
      audio_stream_count: Number::from(1),
      video_bit_depth: Number::from(10),
      video_bitrate: Number::from(0),
      video_codec: "x265".to_owned(),
      video_fps: Number::from_f64(23.976).unwrap(),
      resolution: "1920x804".to_owned(),
      run_time: "2:00:00".to_owned(),
      scan_type: "Progressive".to_owned(),
    }
  }

  fn movie_file() -> MovieFile {
    MovieFile {
      relative_path: "Test.mkv".to_owned(),
      path: "/nfs/movies/Test.mkv".to_owned(),
      date_added: DateTime::from(DateTime::parse_from_rfc3339("2022-12-30T07:37:56Z").unwrap()),
      media_info: Some(media_info()),
    }
  }

  fn collection_movie() -> CollectionMovie {
    CollectionMovie {
      title: "Test".to_owned().into(),
      overview: "Collection blah blah blah".to_owned(),
      year: Number::from(2023),
      runtime: Number::from(120),
      tmdb_id: Number::from(1234),
      genres: genres(),
      ratings: ratings_list(),
    }
  }

  fn collection() -> Collection {
    Collection {
      id: Number::from(123),
      title: "Test Collection".to_owned().into(),
      root_folder_path: Some("/nfs/movies".to_owned()),
      search_on_add: true,
      monitored: true,
      minimum_availability: MinimumAvailability::Released,
      overview: Some("Collection blah blah blah".to_owned()),
      quality_profile_id: Number::from(2222),
      movies: Some(vec![collection_movie()]),
    }
  }

  fn movie() -> Movie {
    Movie {
      id: Number::from(1),
      title: "Test".to_owned().into(),
      original_language: language(),
      size_on_disk: Number::from(3543348019u64),
      status: "Downloaded".to_owned(),
      overview: "Blah blah blah".to_owned(),
      path: "/nfs/movies".to_owned(),
      studio: "21st Century Alex".to_owned(),
      genres: genres(),
      year: Number::from(2023),
      monitored: true,
      has_file: true,
      runtime: Number::from(120),
      tmdb_id: Number::from(1234),
      quality_profile_id: Number::from(2222),
      minimum_availability: MinimumAvailability::Announced,
      certification: Some("R".to_owned()),
      tags: vec![Number::from(1)],
      ratings: ratings_list(),
      movie_file: Some(movie_file()),
      collection: Some(collection()),
    }
  }

  fn rejections() -> Vec<String> {
    vec![
      "Unknown quality profile".to_owned(),
      "Release is already mapped".to_owned(),
    ]
  }

  fn quality() -> Quality {
    Quality {
      name: "HD - 1080p".to_owned(),
    }
  }

  fn quality_wrapper() -> QualityWrapper {
    QualityWrapper { quality: quality() }
  }

  fn release() -> Release {
    Release {
      guid: "1234".to_owned(),
      protocol: "torrent".to_owned(),
      age: Number::from(1),
      title: HorizontallyScrollableText::from("Test Release"),
      indexer: "kickass torrents".to_owned(),
      indexer_id: Number::from(2),
      size: Number::from(1234),
      rejected: true,
      rejections: Some(rejections()),
      seeders: Some(Number::from(2)),
      leechers: Some(Number::from(1)),
      languages: Some(vec![language()]),
      quality: quality_wrapper(),
    }
  }

  fn add_movie_search_result() -> AddMovieSearchResult {
    AddMovieSearchResult {
      tmdb_id: Number::from(1234),
      title: HorizontallyScrollableText::from("Test"),
      original_language: language(),
      status: "released".to_owned(),
      overview: "New movie blah blah blah".to_owned(),
      genres: genres(),
      year: Number::from(2023),
      runtime: Number::from(120),
      ratings: ratings_list(),
    }
  }

  fn movie_history_item() -> MovieHistoryItem {
    MovieHistoryItem {
      source_title: HorizontallyScrollableText::from("Test"),
      quality: quality_wrapper(),
      languages: vec![language()],
      date: DateTime::from(DateTime::parse_from_rfc3339("2022-12-30T07:37:56Z").unwrap()),
      event_type: "grabbed".to_owned(),
    }
  }

  fn download_record() -> DownloadRecord {
    DownloadRecord {
      title: "Test Download Title".to_owned(),
      status: "downloading".to_owned(),
      id: Number::from(1),
      movie_id: Number::from(1),
      size: Number::from(3543348019u64),
      sizeleft: Number::from(1771674009u64),
      output_path: Some(HorizontallyScrollableText::from("/nfs/movies/Test")),
      indexer: "kickass torrents".to_owned(),
      download_client: "transmission".to_owned(),
    }
  }

  fn downloads_response() -> DownloadsResponse {
    DownloadsResponse {
      records: vec![download_record()],
    }
  }

  fn root_folder() -> RootFolder {
    RootFolder {
      id: Number::from(1),
      path: "/nfs".to_owned(),
      accessible: true,
      free_space: Number::from(219902325555200u64),
      unmapped_folders: None,
    }
  }

  fn cast_credit() -> Credit {
    Credit {
      person_name: "Madison Clarke".to_owned(),
      character: Some("Johnny Blaze".to_owned()),
      department: None,
      job: None,
      credit_type: CreditType::Cast,
    }
  }

  fn crew_credit() -> Credit {
    Credit {
      person_name: "Alex Clarke".to_owned(),
      character: None,
      department: Some("Music".to_owned()),
      job: Some("Composition".to_owned()),
      credit_type: CreditType::Crew,
    }
  }

  fn indexer() -> Indexer {
    Indexer {
      enable_rss: true,
      enable_automatic_search: true,
      enable_interactive_search: true,
      supports_rss: true,
      supports_search: true,
      protocol: "torrent".to_owned(),
      priority: Number::from(25),
      download_client_id: Number::from(0),
      name: Some("Test Indexer".to_owned()),
      implementation_name: Some("Torznab".to_owned()),
      implementation: Some("Torznab".to_owned()),
      config_contract: Some("TorznabSettings".to_owned()),
      tags: Some(vec!["test_tag".to_owned()]),
      id: Number::from(1),
      fields: Some(vec![
        IndexerField {
          order: Number::from(0),
          name: Some("valueIsString".to_owned()),
          label: Some("Value Is String".to_owned()),
          value: Some(json!("hello")),
          field_type: Some("textbox".to_owned()),
          select_options: None,
        },
        IndexerField {
          order: Number::from(1),
          name: Some("emptyValueWithSelectOptions".to_owned()),
          label: Some("Empty Value With Select Options".to_owned()),
          value: None,
          field_type: Some("select".to_owned()),
          select_options: Some(vec![IndexerSelectOption {
            value: Number::from(-2),
            name: Some("Original".to_owned()),
            order: Number::from(0),
          }]),
        },
        IndexerField {
          order: Number::from(2),
          name: Some("valueIsAnArray".to_owned()),
          label: Some("Value is an array".to_owned()),
          value: Some(json!([1, 2])),
          field_type: Some("select".to_owned()),
          select_options: None,
        },
      ]),
    }
  }

  fn indexer_settings() -> IndexerSettings {
    IndexerSettings {
      rss_sync_interval: Number::from(60),
      allow_hardcoded_subs: true,
      id: Number::from(1),
      ..IndexerSettings::default()
    }
  }
}
