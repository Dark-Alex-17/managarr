#[cfg(test)]
mod test {
  use std::sync::Arc;

  use bimap::BiMap;
  use chrono::{DateTime, Utc};
  use mockito::{Matcher, Server};
  use pretty_assertions::{assert_eq, assert_str_eq};
  use reqwest::Client;
  use rstest::rstest;
  use serde_json::{json, Number, Value};
  use strum::IntoEnumIterator;
  use tokio::sync::Mutex;
  use tokio_util::sync::CancellationToken;

  use crate::app::ServarrConfig;
  use crate::models::radarr_models::{
    BlocklistItem, BlocklistItemMovie, CollectionMovie, MediaInfo, MinimumAvailability, Monitor,
    MovieCollection, MovieFile, Rating, RatingsList,
  };
  use crate::models::servarr_data::radarr::radarr_data::ActiveRadarrBlock;
  use crate::models::servarr_models::{
    HostConfig, IndexerField, Language, Quality, QualityWrapper,
  };
  use crate::models::stateful_table::SortOption;
  use crate::models::HorizontallyScrollableText;
  use crate::network::network_tests::test_utils::mock_servarr_api;
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
      RadarrEvent::AddMovie(None),
      RadarrEvent::EditMovie(None),
      RadarrEvent::GetMovies,
      RadarrEvent::GetMovieDetails(None),
      RadarrEvent::DeleteMovie(None)
    )]
    event: RadarrEvent,
  ) {
    assert_str_eq!(event.resource(), "/movie");
  }

  #[rstest]
  fn test_resource_collection(
    #[values(RadarrEvent::GetCollections, RadarrEvent::EditCollection(None))] event: RadarrEvent,
  ) {
    assert_str_eq!(event.resource(), "/collection");
  }

  #[rstest]
  fn test_resource_indexer(
    #[values(RadarrEvent::GetIndexers, RadarrEvent::DeleteIndexer(None))] event: RadarrEvent,
  ) {
    assert_str_eq!(event.resource(), "/indexer");
  }

  #[rstest]
  fn test_resource_all_indexer_settings(
    #[values(
      RadarrEvent::GetAllIndexerSettings,
      RadarrEvent::EditAllIndexerSettings(None)
    )]
    event: RadarrEvent,
  ) {
    assert_str_eq!(event.resource(), "/config/indexer");
  }

  #[rstest]
  fn test_resource_root_folder(
    #[values(
      RadarrEvent::AddRootFolder(None),
      RadarrEvent::GetRootFolders,
      RadarrEvent::DeleteRootFolder(None)
    )]
    event: RadarrEvent,
  ) {
    assert_str_eq!(event.resource(), "/rootfolder");
  }

  #[rstest]
  fn test_resource_tag(
    #[values(
      RadarrEvent::AddTag(String::new()),
      RadarrEvent::GetTags,
      RadarrEvent::DeleteTag(0)
    )]
    event: RadarrEvent,
  ) {
    assert_str_eq!(event.resource(), "/tag");
  }

  #[rstest]
  fn test_resource_release(
    #[values(RadarrEvent::GetReleases(None), RadarrEvent::DownloadRelease(None))]
    event: RadarrEvent,
  ) {
    assert_str_eq!(event.resource(), "/release");
  }

  #[rstest]
  fn test_resource_queue(
    #[values(RadarrEvent::GetDownloads, RadarrEvent::DeleteDownload(None))] event: RadarrEvent,
  ) {
    assert_str_eq!(event.resource(), "/queue");
  }

  #[rstest]
  fn test_resource_host_config(
    #[values(RadarrEvent::GetHostConfig, RadarrEvent::GetSecurityConfig)] event: RadarrEvent,
  ) {
    assert_str_eq!(event.resource(), "/config/host");
  }

  #[rstest]
  fn test_resource_command(
    #[values(
      RadarrEvent::StartTask(None),
      RadarrEvent::GetQueuedEvents,
      RadarrEvent::TriggerAutomaticSearch(None),
      RadarrEvent::UpdateAndScan(None),
      RadarrEvent::UpdateAllMovies,
      RadarrEvent::UpdateDownloads,
      RadarrEvent::UpdateCollections
    )]
    event: RadarrEvent,
  ) {
    assert_str_eq!(event.resource(), "/command");
  }

  #[rstest]
  #[case(RadarrEvent::ClearBlocklist, "/blocklist/bulk")]
  #[case(RadarrEvent::DeleteBlocklistItem(None), "/blocklist")]
  #[case(RadarrEvent::GetBlocklist, "/blocklist?page=1&pageSize=10000")]
  #[case(RadarrEvent::GetLogs(Some(500)), "/log")]
  #[case(RadarrEvent::SearchNewMovie(None), "/movie/lookup")]
  #[case(RadarrEvent::GetMovieCredits(None), "/credit")]
  #[case(RadarrEvent::GetMovieHistory(None), "/history/movie")]
  #[case(RadarrEvent::GetOverview, "/diskspace")]
  #[case(RadarrEvent::GetQualityProfiles, "/qualityprofile")]
  #[case(RadarrEvent::GetStatus, "/system/status")]
  #[case(RadarrEvent::GetTasks, "/system/task")]
  #[case(RadarrEvent::GetUpdates, "/update")]
  #[case(RadarrEvent::TestIndexer(None), "/indexer/test")]
  #[case(RadarrEvent::TestAllIndexers, "/indexer/testall")]
  #[case(RadarrEvent::HealthCheck, "/health")]
  fn test_resource(#[case] event: RadarrEvent, #[case] expected_uri: String) {
    assert_str_eq!(event.resource(), expected_uri);
  }

  #[test]
  fn test_from_radarr_event() {
    assert_eq!(
      NetworkEvent::Radarr(RadarrEvent::HealthCheck),
      NetworkEvent::from(RadarrEvent::HealthCheck)
    );
  }

  #[tokio::test]
  async fn test_handle_get_radarr_healthcheck_event() {
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Get,
      None,
      None,
      None,
      RadarrEvent::HealthCheck,
      None,
      None,
    )
    .await;
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    let _ = network.handle_radarr_event(RadarrEvent::HealthCheck).await;

    async_server.assert_async().await;
  }

  #[tokio::test]
  async fn test_handle_get_diskspace_event() {
    let (async_server, app_arc, _server) = mock_servarr_api(
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
      None,
      RadarrEvent::GetOverview,
      None,
      None,
    )
    .await;
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());
    let disk_space_vec = vec![
      DiskSpace {
        free_space: 1111,
        total_space: 2222,
      },
      DiskSpace {
        free_space: 3333,
        total_space: 4444,
      },
    ];

    if let RadarrSerdeable::DiskSpaces(disk_space) = network
      .handle_radarr_event(RadarrEvent::GetOverview)
      .await
      .unwrap()
    {
      async_server.assert_async().await;
      assert_eq!(
        app_arc.lock().await.data.radarr_data.disk_space_vec,
        disk_space_vec
      );
      assert_eq!(disk_space, disk_space_vec);
    }
  }

  #[tokio::test]
  async fn test_handle_get_status_event() {
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Get,
      None,
      Some(json!({
        "version": "v1",
        "startTime": "2023-02-25T20:16:43Z"
      })),
      None,
      RadarrEvent::GetStatus,
      None,
      None,
    )
    .await;
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());
    let date_time = DateTime::from(DateTime::parse_from_rfc3339("2023-02-25T20:16:43Z").unwrap())
      as DateTime<Utc>;

    if let RadarrSerdeable::SystemStatus(status) = network
      .handle_radarr_event(RadarrEvent::GetStatus)
      .await
      .unwrap()
    {
      async_server.assert_async().await;
      assert_str_eq!(app_arc.lock().await.data.radarr_data.version, "v1");
      assert_eq!(app_arc.lock().await.data.radarr_data.start_time, date_time);
      assert_eq!(
        status,
        SystemStatus {
          version: "v1".to_owned(),
          start_time: date_time
        }
      );
    }
  }

  #[rstest]
  #[tokio::test]
  async fn test_handle_get_movies_event(#[values(true, false)] use_custom_sorting: bool) {
    let mut movie_1: Value = serde_json::from_str(MOVIE_JSON).unwrap();
    let mut movie_2: Value = serde_json::from_str(MOVIE_JSON).unwrap();
    *movie_1.get_mut("id").unwrap() = json!(1);
    *movie_1.get_mut("title").unwrap() = json!("z test");
    *movie_2.get_mut("id").unwrap() = json!(2);
    *movie_2.get_mut("title").unwrap() = json!("A test");
    let expected_movies = vec![
      Movie {
        id: 1,
        title: "z test".into(),
        ..movie()
      },
      Movie {
        id: 2,
        title: "A test".into(),
        ..movie()
      },
    ];
    let mut expected_sorted_movies = vec![
      Movie {
        id: 1,
        title: "z test".into(),
        ..movie()
      },
      Movie {
        id: 2,
        title: "A test".into(),
        ..movie()
      },
    ];
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Get,
      None,
      Some(json!([movie_1, movie_2])),
      None,
      RadarrEvent::GetMovies,
      None,
      None,
    )
    .await;
    app_arc.lock().await.data.radarr_data.movies.sort_asc = true;
    if use_custom_sorting {
      let cmp_fn = |a: &Movie, b: &Movie| {
        a.title
          .text
          .to_lowercase()
          .cmp(&b.title.text.to_lowercase())
      };
      expected_sorted_movies.sort_by(cmp_fn);
      let title_sort_option = SortOption {
        name: "Title",
        cmp_fn: Some(cmp_fn),
      };
      app_arc
        .lock()
        .await
        .data
        .radarr_data
        .movies
        .sorting(vec![title_sort_option]);
    }
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    if let RadarrSerdeable::Movies(movies) = network
      .handle_radarr_event(RadarrEvent::GetMovies)
      .await
      .unwrap()
    {
      async_server.assert_async().await;
      assert_eq!(
        app_arc.lock().await.data.radarr_data.movies.items,
        expected_sorted_movies
      );
      assert!(app_arc.lock().await.data.radarr_data.movies.sort_asc);
      assert_eq!(movies, expected_movies);
    }
  }

  #[tokio::test]
  async fn test_handle_get_movies_event_no_op_while_user_is_selecting_sort_options() {
    let mut movie_1: Value = serde_json::from_str(MOVIE_JSON).unwrap();
    let mut movie_2: Value = serde_json::from_str(MOVIE_JSON).unwrap();
    *movie_1.get_mut("id").unwrap() = json!(1);
    *movie_1.get_mut("title").unwrap() = json!("z test");
    *movie_2.get_mut("id").unwrap() = json!(2);
    *movie_2.get_mut("title").unwrap() = json!("A test");
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Get,
      None,
      Some(json!([movie_1, movie_2])),
      None,
      RadarrEvent::GetMovies,
      None,
      None,
    )
    .await;
    app_arc
      .lock()
      .await
      .push_navigation_stack(ActiveRadarrBlock::MoviesSortPrompt.into());
    app_arc.lock().await.data.radarr_data.movies.sort_asc = true;
    let cmp_fn = |a: &Movie, b: &Movie| {
      a.title
        .text
        .to_lowercase()
        .cmp(&b.title.text.to_lowercase())
    };
    let title_sort_option = SortOption {
      name: "Title",
      cmp_fn: Some(cmp_fn),
    };
    app_arc
      .lock()
      .await
      .data
      .radarr_data
      .movies
      .sorting(vec![title_sort_option]);
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    assert!(network
      .handle_radarr_event(RadarrEvent::GetMovies)
      .await
      .is_ok());

    async_server.assert_async().await;
    assert!(app_arc
      .lock()
      .await
      .data
      .radarr_data
      .movies
      .items
      .is_empty());
    assert!(app_arc.lock().await.data.radarr_data.movies.sort_asc);
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
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Get,
      None,
      Some(release_json),
      None,
      RadarrEvent::GetReleases(None),
      None,
      Some("movieId=1"),
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
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    if let RadarrSerdeable::Releases(releases_vec) = network
      .handle_radarr_event(RadarrEvent::GetReleases(None))
      .await
      .unwrap()
    {
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
      assert_eq!(releases_vec, vec![release()]);
    }
  }

  #[tokio::test]
  async fn test_handle_get_releases_event_empty_movie_details_modal() {
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
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Get,
      None,
      Some(release_json),
      None,
      RadarrEvent::GetReleases(None),
      None,
      Some("movieId=1"),
    )
    .await;
    app_arc
      .lock()
      .await
      .data
      .radarr_data
      .movies
      .set_items(vec![movie()]);
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    assert!(network
      .handle_radarr_event(RadarrEvent::GetReleases(None))
      .await
      .is_ok());

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
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Get,
      None,
      Some(add_movie_search_result_json),
      None,
      RadarrEvent::SearchNewMovie(None),
      None,
      Some("term=test%20term"),
    )
    .await;
    app_arc.lock().await.data.radarr_data.add_movie_search = Some("test term".into());
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    if let RadarrSerdeable::AddMovieSearchResults(add_movie_search_results) = network
      .handle_radarr_event(RadarrEvent::SearchNewMovie(None))
      .await
      .unwrap()
    {
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
      assert_eq!(add_movie_search_results, vec![add_movie_search_result()]);
    }
  }

  #[tokio::test]
  async fn test_handle_search_new_movie_event_uses_provided_query() {
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
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Get,
      None,
      Some(add_movie_search_result_json),
      None,
      RadarrEvent::SearchNewMovie(None),
      None,
      Some("term=test%20term"),
    )
    .await;
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    if let RadarrSerdeable::AddMovieSearchResults(add_movie_search_results) = network
      .handle_radarr_event(RadarrEvent::SearchNewMovie(Some("test term".into())))
      .await
      .unwrap()
    {
      async_server.assert_async().await;
      assert_eq!(add_movie_search_results, vec![add_movie_search_result()]);
    }
  }

  #[tokio::test]
  async fn test_handle_start_task_event() {
    let response = json!({ "test": "test"});
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Post,
      Some(json!({
        "name": "ApplicationCheckUpdate"
      })),
      Some(response.clone()),
      None,
      RadarrEvent::StartTask(None),
      None,
      None,
    )
    .await;
    app_arc
      .lock()
      .await
      .data
      .radarr_data
      .tasks
      .set_items(vec![Task {
        task_name: TaskName::default(),
        ..Task::default()
      }]);
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    if let RadarrSerdeable::Value(value) = network
      .handle_radarr_event(RadarrEvent::StartTask(None))
      .await
      .unwrap()
    {
      async_server.assert_async().await;
      assert_eq!(value, response);
    }
  }

  #[tokio::test]
  async fn test_handle_start_task_event_uses_provided_task_name() {
    let response = json!({ "test": "test"});
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Post,
      Some(json!({
        "name": "ApplicationCheckUpdate"
      })),
      Some(response.clone()),
      None,
      RadarrEvent::StartTask(None),
      None,
      None,
    )
    .await;
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    if let RadarrSerdeable::Value(value) = network
      .handle_radarr_event(RadarrEvent::StartTask(Some(TaskName::default())))
      .await
      .unwrap()
    {
      async_server.assert_async().await;
      assert_eq!(value, response);
    }
  }

  #[tokio::test]
  async fn test_handle_search_new_movie_event_no_results() {
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Get,
      None,
      Some(json!([])),
      None,
      RadarrEvent::SearchNewMovie(None),
      None,
      Some("term=test%20term"),
    )
    .await;
    app_arc.lock().await.data.radarr_data.add_movie_search = Some("test term".into());
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    assert!(network
      .handle_radarr_event(RadarrEvent::SearchNewMovie(None))
      .await
      .is_ok());

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
      RadarrEvent::SearchNewMovie(None).resource()
    );
    let mut server = Server::new_async().await;
    let mut async_server = server
      .mock(
        &RequestMethod::Get.to_string().to_uppercase(),
        format!("/api/v3{resource}").as_str(),
      )
      .match_header("X-Api-Key", "test1234");
    async_server = async_server.expect_at_most(0).create_async().await;

    let host = Some(server.host_with_port().split(':').collect::<Vec<&str>>()[0].to_owned());
    let port = Some(
      server.host_with_port().split(':').collect::<Vec<&str>>()[1]
        .parse()
        .unwrap(),
    );
    let mut app = App::default();
    let radarr_config = ServarrConfig {
      host,
      port,
      api_token: "test1234".to_owned(),
      ..ServarrConfig::default()
    };
    app.config.radarr = Some(radarr_config);
    let app_arc = Arc::new(Mutex::new(app));
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    assert!(network
      .handle_radarr_event(RadarrEvent::SearchNewMovie(None))
      .await
      .is_ok());

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
  async fn test_handle_test_indexer_event_error() {
    let indexer_details_json = json!({
        "enableRss": true,
        "enableAutomaticSearch": true,
        "enableInteractiveSearch": true,
        "name": "Test Indexer",
        "fields": [
            {
                "name": "baseUrl",
                "value": "https://test.com",
            },
            {
                "name": "apiKey",
                "value": "",
            },
            {
                "name": "seedCriteria.seedRatio",
                "value": "1.2",
            },
        ],
        "tags": [1],
        "id": 1
    });
    let response_json = json!([
    {
        "isWarning": false,
        "propertyName": "",
        "errorMessage": "test failure",
        "severity": "error"
    }]);
    let (async_details_server, app_arc, mut server) = mock_servarr_api(
      RequestMethod::Get,
      None,
      Some(indexer_details_json.clone()),
      None,
      RadarrEvent::GetIndexers,
      Some("/1"),
      None,
    )
    .await;
    let async_test_server = server
      .mock(
        "POST",
        format!("/api/v3{}", RadarrEvent::TestIndexer(None).resource()).as_str(),
      )
      .with_status(400)
      .match_header("X-Api-Key", "test1234")
      .match_body(Matcher::Json(indexer_details_json.clone()))
      .with_body(response_json.to_string())
      .create_async()
      .await;
    app_arc
      .lock()
      .await
      .data
      .radarr_data
      .indexers
      .set_items(vec![indexer()]);
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    if let RadarrSerdeable::Value(value) = network
      .handle_radarr_event(RadarrEvent::TestIndexer(None))
      .await
      .unwrap()
    {
      async_details_server.assert_async().await;
      async_test_server.assert_async().await;
      assert_eq!(
        app_arc.lock().await.data.radarr_data.indexer_test_error,
        Some("\"test failure\"".to_owned())
      );
      assert_eq!(value, response_json)
    }
  }

  #[tokio::test]
  async fn test_handle_test_indexer_event_success() {
    let indexer_details_json = json!({
        "enableRss": true,
        "enableAutomaticSearch": true,
        "enableInteractiveSearch": true,
        "name": "Test Indexer",
        "fields": [
            {
                "name": "baseUrl",
                "value": "https://test.com",
            },
            {
                "name": "apiKey",
                "value": "",
            },
            {
                "name": "seedCriteria.seedRatio",
                "value": "1.2",
            },
        ],
        "tags": [1],
        "id": 1
    });
    let (async_details_server, app_arc, mut server) = mock_servarr_api(
      RequestMethod::Get,
      None,
      Some(indexer_details_json.clone()),
      None,
      RadarrEvent::GetIndexers,
      Some("/1"),
      None,
    )
    .await;
    let async_test_server = server
      .mock(
        "POST",
        format!("/api/v3{}", RadarrEvent::TestIndexer(None).resource()).as_str(),
      )
      .with_status(200)
      .match_header("X-Api-Key", "test1234")
      .match_body(Matcher::Json(indexer_details_json.clone()))
      .with_body("{}")
      .create_async()
      .await;
    app_arc
      .lock()
      .await
      .data
      .radarr_data
      .indexers
      .set_items(vec![indexer()]);
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    if let RadarrSerdeable::Value(value) = network
      .handle_radarr_event(RadarrEvent::TestIndexer(None))
      .await
      .unwrap()
    {
      async_details_server.assert_async().await;
      async_test_server.assert_async().await;
      assert_eq!(
        app_arc.lock().await.data.radarr_data.indexer_test_error,
        None
      );
      assert_eq!(value, json!({}));
    }
  }

  #[tokio::test]
  async fn test_handle_test_indexer_event_success_uses_provided_id() {
    let indexer_details_json = json!({
        "enableRss": true,
        "enableAutomaticSearch": true,
        "enableInteractiveSearch": true,
        "name": "Test Indexer",
        "fields": [
            {
                "name": "baseUrl",
                "value": "https://test.com",
            },
            {
                "name": "apiKey",
                "value": "",
            },
            {
                "name": "seedCriteria.seedRatio",
                "value": "1.2",
            },
        ],
        "tags": [1],
        "id": 1
    });
    let (async_details_server, app_arc, mut server) = mock_servarr_api(
      RequestMethod::Get,
      None,
      Some(indexer_details_json.clone()),
      None,
      RadarrEvent::GetIndexers,
      Some("/1"),
      None,
    )
    .await;
    let async_test_server = server
      .mock(
        "POST",
        format!("/api/v3{}", RadarrEvent::TestIndexer(None).resource()).as_str(),
      )
      .with_status(200)
      .match_header("X-Api-Key", "test1234")
      .match_body(Matcher::Json(indexer_details_json.clone()))
      .with_body("{}")
      .create_async()
      .await;
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    if let RadarrSerdeable::Value(value) = network
      .handle_radarr_event(RadarrEvent::TestIndexer(Some(1)))
      .await
      .unwrap()
    {
      async_details_server.assert_async().await;
      async_test_server.assert_async().await;
      assert_eq!(value, json!({}));
    }
  }

  #[tokio::test]
  async fn test_handle_test_all_indexers_event() {
    let indexers = vec![
      Indexer {
        id: 1,
        name: Some("Test 1".to_owned()),
        ..Indexer::default()
      },
      Indexer {
        id: 2,
        name: Some("Test 2".to_owned()),
        ..Indexer::default()
      },
    ];
    let indexer_test_results_modal_items = vec![
			IndexerTestResultModalItem {
				name: "Test 1".to_owned(),
				is_valid: true,
				validation_failures: HorizontallyScrollableText::default(),
			},
			IndexerTestResultModalItem {
				name: "Test 2".to_owned(),
				is_valid: false,
				validation_failures: "Failure for field 'test field 1': test error message, Failure for field 'test field 2': test error message 2".into(),
			},
		];
    let response_json = json!([
    {
      "id": 1,
      "isValid": true,
      "validationFailures": []
    },
    {
      "id": 2,
      "isValid": false,
      "validationFailures": [
          {
              "propertyName": "test field 1",
              "errorMessage": "test error message",
              "severity": "error"
          },
          {
              "propertyName": "test field 2",
              "errorMessage": "test error message 2",
              "severity": "error"
          },
      ]
    }]);
    let response: Vec<IndexerTestResult> = serde_json::from_value(response_json.clone()).unwrap();
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Post,
      None,
      Some(response_json),
      Some(400),
      RadarrEvent::TestAllIndexers,
      None,
      None,
    )
    .await;
    app_arc
      .lock()
      .await
      .data
      .radarr_data
      .indexers
      .set_items(indexers);
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    if let RadarrSerdeable::IndexerTestResults(results) = network
      .handle_radarr_event(RadarrEvent::TestAllIndexers)
      .await
      .unwrap()
    {
      async_server.assert_async().await;
      assert!(app_arc
        .lock()
        .await
        .data
        .radarr_data
        .indexer_test_all_results
        .is_some());
      assert_eq!(
        app_arc
          .lock()
          .await
          .data
          .radarr_data
          .indexer_test_all_results
          .as_ref()
          .unwrap()
          .items,
        indexer_test_results_modal_items
      );
      assert_eq!(results, response);
    }
  }

  #[tokio::test]
  async fn test_handle_trigger_automatic_search_event() {
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Post,
      Some(json!({
        "name": "MoviesSearch",
        "movieIds": [ 1 ]
      })),
      Some(json!({})),
      None,
      RadarrEvent::TriggerAutomaticSearch(None),
      None,
      None,
    )
    .await;
    app_arc
      .lock()
      .await
      .data
      .radarr_data
      .movies
      .set_items(vec![movie()]);
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    assert!(network
      .handle_radarr_event(RadarrEvent::TriggerAutomaticSearch(None))
      .await
      .is_ok());

    async_server.assert_async().await;
  }

  #[tokio::test]
  async fn test_handle_trigger_automatic_search_event_uses_provided_id() {
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Post,
      Some(json!({
        "name": "MoviesSearch",
        "movieIds": [ 1 ]
      })),
      Some(json!({})),
      None,
      RadarrEvent::TriggerAutomaticSearch(None),
      None,
      None,
    )
    .await;
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    assert!(network
      .handle_radarr_event(RadarrEvent::TriggerAutomaticSearch(Some(1)))
      .await
      .is_ok());

    async_server.assert_async().await;
  }

  #[tokio::test]
  async fn test_handle_update_and_scan_event() {
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Post,
      Some(json!({
        "name": "RefreshMovie",
        "movieIds": [ 1 ]
      })),
      Some(json!({})),
      None,
      RadarrEvent::UpdateAndScan(None),
      None,
      None,
    )
    .await;
    app_arc
      .lock()
      .await
      .data
      .radarr_data
      .movies
      .set_items(vec![movie()]);
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    assert!(network
      .handle_radarr_event(RadarrEvent::UpdateAndScan(None))
      .await
      .is_ok());

    async_server.assert_async().await;
  }

  #[tokio::test]
  async fn test_handle_update_and_scan_event_uses_provied_movie_id() {
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Post,
      Some(json!({
        "name": "RefreshMovie",
        "movieIds": [ 1 ]
      })),
      Some(json!({})),
      None,
      RadarrEvent::UpdateAndScan(None),
      None,
      None,
    )
    .await;
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    assert!(network
      .handle_radarr_event(RadarrEvent::UpdateAndScan(Some(1)))
      .await
      .is_ok());

    async_server.assert_async().await;
  }

  #[tokio::test]
  async fn test_handle_update_all_movies_event() {
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Post,
      Some(json!({
        "name": "RefreshMovie",
        "movieIds": []
      })),
      Some(json!({})),
      None,
      RadarrEvent::UpdateAllMovies,
      None,
      None,
    )
    .await;
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    assert!(network
      .handle_radarr_event(RadarrEvent::UpdateAllMovies)
      .await
      .is_ok());

    async_server.assert_async().await;
  }

  #[tokio::test]
  async fn test_handle_update_downloads_event() {
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Post,
      Some(json!({
        "name": "RefreshMonitoredDownloads"
      })),
      Some(json!({})),
      None,
      RadarrEvent::UpdateDownloads,
      None,
      None,
    )
    .await;
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    assert!(network
      .handle_radarr_event(RadarrEvent::UpdateDownloads)
      .await
      .is_ok());

    async_server.assert_async().await;
  }

  #[tokio::test]
  async fn test_handle_update_collections_event() {
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Post,
      Some(json!({
        "name": "RefreshCollections"
      })),
      Some(json!({})),
      None,
      RadarrEvent::UpdateCollections,
      None,
      None,
    )
    .await;
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    assert!(network
      .handle_radarr_event(RadarrEvent::UpdateCollections)
      .await
      .is_ok());

    async_server.assert_async().await;
  }

  #[tokio::test]
  async fn test_handle_get_movie_details_event() {
    let response: Movie = serde_json::from_str(MOVIE_JSON).unwrap();
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Get,
      None,
      Some(serde_json::from_str(MOVIE_JSON).unwrap()),
      None,
      RadarrEvent::GetMovieDetails(None),
      Some("/1"),
      None,
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
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    if let RadarrSerdeable::Movie(movie) = network
      .handle_radarr_event(RadarrEvent::GetMovieDetails(None))
      .await
      .unwrap()
    {
      async_server.assert_async().await;
      assert!(app_arc
        .lock()
        .await
        .data
        .radarr_data
        .movie_details_modal
        .is_some());
      assert_eq!(movie, response);

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
  }

  #[tokio::test]
  async fn test_handle_get_movie_details_event_uses_provided_id() {
    let response: Movie = serde_json::from_str(MOVIE_JSON).unwrap();
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Get,
      None,
      Some(serde_json::from_str(MOVIE_JSON).unwrap()),
      None,
      RadarrEvent::GetMovieDetails(None),
      Some("/1"),
      None,
    )
    .await;
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    if let RadarrSerdeable::Movie(movie) = network
      .handle_radarr_event(RadarrEvent::GetMovieDetails(Some(1)))
      .await
      .unwrap()
    {
      async_server.assert_async().await;
      assert_eq!(movie, response);
    }
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
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Get,
      None,
      Some(movie_json_with_missing_fields),
      None,
      RadarrEvent::GetMovieDetails(None),
      Some("/1"),
      None,
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
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    assert!(network
      .handle_radarr_event(RadarrEvent::GetMovieDetails(None))
      .await
      .is_ok());

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
    let response: Vec<MovieHistoryItem> =
      serde_json::from_value(movie_history_item_json.clone()).unwrap();
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Get,
      None,
      Some(movie_history_item_json),
      None,
      RadarrEvent::GetMovieHistory(None),
      None,
      Some("movieId=1"),
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
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    if let RadarrSerdeable::MovieHistoryItems(history) = network
      .handle_radarr_event(RadarrEvent::GetMovieHistory(None))
      .await
      .unwrap()
    {
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
      assert_eq!(history, response);
    }
  }

  #[tokio::test]
  async fn test_handle_get_movie_history_event_uses_provided_id() {
    let movie_history_item_json = json!([{
      "sourceTitle": "Test",
      "quality": { "quality": { "name": "HD - 1080p" }},
      "languages": [ { "name": "English" } ],
      "date": "2022-12-30T07:37:56Z",
      "eventType": "grabbed"
    }]);
    let response: Vec<MovieHistoryItem> =
      serde_json::from_value(movie_history_item_json.clone()).unwrap();
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Get,
      None,
      Some(movie_history_item_json),
      None,
      RadarrEvent::GetMovieHistory(None),
      None,
      Some("movieId=1"),
    )
    .await;
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    if let RadarrSerdeable::MovieHistoryItems(history) = network
      .handle_radarr_event(RadarrEvent::GetMovieHistory(Some(1)))
      .await
      .unwrap()
    {
      async_server.assert_async().await;
      assert_eq!(history, response);
    }
  }

  #[tokio::test]
  async fn test_handle_get_movie_history_event_empty_movie_details_modal() {
    let movie_history_item_json = json!([{
      "sourceTitle": "Test",
      "quality": { "quality": { "name": "HD - 1080p" }},
      "languages": [ { "name": "English" } ],
      "date": "2022-12-30T07:37:56Z",
      "eventType": "grabbed"
    }]);
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Get,
      None,
      Some(movie_history_item_json),
      None,
      RadarrEvent::GetMovieHistory(None),
      None,
      Some("movieId=1"),
    )
    .await;
    app_arc
      .lock()
      .await
      .data
      .radarr_data
      .movies
      .set_items(vec![movie()]);
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    assert!(network
      .handle_radarr_event(RadarrEvent::GetMovieHistory(None))
      .await
      .is_ok());

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

  #[rstest]
  #[tokio::test]
  async fn test_handle_get_radarr_blocklist_event(#[values(true, false)] use_custom_sorting: bool) {
    let blocklist_json = json!({"records": [{
        "id": 123,
        "movieId": 1007,
        "sourceTitle": "z movie",
        "languages": [{"name": "English"}],
        "quality": {"quality": {"name": "HD - 1080p"}},
        "customFormats": [{"name": "English"}],
        "date": "2024-02-10T07:28:45Z",
        "protocol": "usenet",
        "indexer": "DrunkenSlug (Prowlarr)",
        "message": "test message",
        "movie": {
          "id": 1007,
          "title": "z movie",
          "tmdbId": 1234,
          "originalLanguage": {"name": "English"},
          "sizeOnDisk": 3543348019i64,
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
            "imdb": {"value": 9.9},
            "tmdb": {"value": 9.9},
            "rottenTomatoes": {"value": 9.9}
          },
        },
      }, {
        "id": 456,
        "movieId": 2001,
        "sourceTitle": "A Movie",
        "languages": [{"name": "English"}],
        "quality": {"quality": {"name": "HD - 1080p"}},
        "customFormats": [{"name": "English"}],
        "date": "2024-02-10T07:28:45Z",
        "protocol": "usenet",
        "indexer": "DrunkenSlug (Prowlarr)",
        "message": "test message",
        "movie": {
          "id": 2001,
          "title": "A Movie",
          "tmdbId": 1234,
          "originalLanguage": {"name": "English"},
          "sizeOnDisk": 3543348019i64,
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
            "imdb": {"value": 9.9},
            "tmdb": {"value": 9.9},
            "rottenTomatoes": {"value": 9.9}
          },
        },
    }]});
    let response: BlocklistResponse = serde_json::from_value(blocklist_json.clone()).unwrap();
    let mut expected_blocklist = vec![
      BlocklistItem {
        id: 123,
        movie_id: 1007,
        source_title: "z movie".into(),
        movie: BlocklistItemMovie {
          title: "z movie".into(),
        },
        ..blocklist_item()
      },
      BlocklistItem {
        id: 456,
        movie_id: 2001,
        source_title: "A Movie".into(),
        movie: BlocklistItemMovie {
          title: "A Movie".into(),
        },
        ..blocklist_item()
      },
    ];
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Get,
      None,
      Some(blocklist_json),
      None,
      RadarrEvent::GetBlocklist,
      None,
      None,
    )
    .await;
    app_arc.lock().await.data.radarr_data.blocklist.sort_asc = true;
    if use_custom_sorting {
      let cmp_fn = |a: &BlocklistItem, b: &BlocklistItem| {
        a.source_title
          .to_lowercase()
          .cmp(&b.source_title.to_lowercase())
      };
      expected_blocklist.sort_by(cmp_fn);

      let blocklist_sort_option = SortOption {
        name: "Source Title",
        cmp_fn: Some(cmp_fn),
      };
      app_arc
        .lock()
        .await
        .data
        .radarr_data
        .blocklist
        .sorting(vec![blocklist_sort_option]);
    }
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    if let RadarrSerdeable::BlocklistResponse(blocklist) = network
      .handle_radarr_event(RadarrEvent::GetBlocklist)
      .await
      .unwrap()
    {
      async_server.assert_async().await;
      assert_eq!(
        app_arc.lock().await.data.radarr_data.blocklist.items,
        expected_blocklist
      );
      assert!(app_arc.lock().await.data.radarr_data.blocklist.sort_asc);
      assert_eq!(blocklist, response);
    }
  }

  #[tokio::test]
  async fn test_handle_get_blocklist_event_no_op_when_user_is_selecting_sort_options() {
    let blocklist_json = json!({"records": [{
        "id": 123,
        "movieId": 1007,
        "sourceTitle": "z movie",
        "languages": [{"name": "English"}],
        "quality": {"quality": {"name": "HD - 1080p"}},
        "customFormats": [{"name": "English"}],
        "date": "2024-02-10T07:28:45Z",
        "protocol": "usenet",
        "indexer": "DrunkenSlug (Prowlarr)",
        "message": "test message",
        "movie": {
          "id": 1007,
          "title": "z movie",
          "tmdbId": 1234,
          "originalLanguage": {"name": "English"},
          "sizeOnDisk": 3543348019i64,
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
            "imdb": {"value": 9.9},
            "tmdb": {"value": 9.9},
            "rottenTomatoes": {"value": 9.9}
          },
        },
      }, {
        "id": 456,
        "movieId": 2001,
        "sourceTitle": "A Movie",
        "languages": [{"name": "English"}],
        "quality": {"quality": {"name": "HD - 1080p"}},
        "customFormats": [{"name": "English"}],
        "date": "2024-02-10T07:28:45Z",
        "protocol": "usenet",
        "indexer": "DrunkenSlug (Prowlarr)",
        "message": "test message",
        "movie": {
          "id": 2001,
          "title": "A Movie",
          "tmdbId": 1234,
          "originalLanguage": {"name": "English"},
          "sizeOnDisk": 3543348019i64,
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
            "imdb": {"value": 9.9},
            "tmdb": {"value": 9.9},
            "rottenTomatoes": {"value": 9.9}
          },
        },
    }]});
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Get,
      None,
      Some(blocklist_json),
      None,
      RadarrEvent::GetBlocklist,
      None,
      None,
    )
    .await;
    app_arc.lock().await.data.radarr_data.blocklist.sort_asc = true;
    app_arc
      .lock()
      .await
      .push_navigation_stack(ActiveRadarrBlock::BlocklistSortPrompt.into());
    let cmp_fn = |a: &BlocklistItem, b: &BlocklistItem| {
      a.source_title
        .to_lowercase()
        .cmp(&b.source_title.to_lowercase())
    };
    let blocklist_sort_option = SortOption {
      name: "Source Title",
      cmp_fn: Some(cmp_fn),
    };
    app_arc
      .lock()
      .await
      .data
      .radarr_data
      .blocklist
      .sorting(vec![blocklist_sort_option]);
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    assert!(network
      .handle_radarr_event(RadarrEvent::GetBlocklist)
      .await
      .is_ok());

    async_server.assert_async().await;
    assert!(app_arc
      .lock()
      .await
      .data
      .radarr_data
      .blocklist
      .items
      .is_empty());
    assert!(app_arc.lock().await.data.radarr_data.blocklist.sort_asc);
  }

  #[rstest]
  #[tokio::test]
  async fn test_handle_get_collections_event(#[values(true, false)] use_custom_sorting: bool) {
    let collections_json = json!([{
      "id": 123,
      "title": "z Collection",
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
    },
    {
      "id": 456,
      "title": "A Collection",
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
    let response: Vec<Collection> = serde_json::from_value(collections_json.clone()).unwrap();
    let mut expected_collections = vec![
      Collection {
        id: 123,
        title: "z Collection".into(),
        ..collection()
      },
      Collection {
        id: 456,
        title: "A Collection".into(),
        ..collection()
      },
    ];
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Get,
      None,
      Some(collections_json),
      None,
      RadarrEvent::GetCollections,
      None,
      None,
    )
    .await;
    app_arc.lock().await.data.radarr_data.collections.sort_asc = true;
    if use_custom_sorting {
      let cmp_fn = |a: &Collection, b: &Collection| {
        a.title
          .text
          .to_lowercase()
          .cmp(&b.title.text.to_lowercase())
      };
      expected_collections.sort_by(cmp_fn);

      let collection_sort_option = SortOption {
        name: "Collection",
        cmp_fn: Some(cmp_fn),
      };
      app_arc
        .lock()
        .await
        .data
        .radarr_data
        .collections
        .sorting(vec![collection_sort_option]);
    }
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    if let RadarrSerdeable::Collections(collections) = network
      .handle_radarr_event(RadarrEvent::GetCollections)
      .await
      .unwrap()
    {
      async_server.assert_async().await;
      assert_eq!(
        app_arc.lock().await.data.radarr_data.collections.items,
        expected_collections
      );
      assert!(app_arc.lock().await.data.radarr_data.collections.sort_asc);
      assert_eq!(collections, response);
    }
  }

  #[tokio::test]
  async fn test_handle_get_collections_event_no_op_when_user_is_selecting_sort_options() {
    let collections_json = json!([{
      "id": 123,
      "title": "z Collection",
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
    },
    {
      "id": 456,
      "title": "A Collection",
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
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Get,
      None,
      Some(collections_json),
      None,
      RadarrEvent::GetCollections,
      None,
      None,
    )
    .await;
    app_arc.lock().await.data.radarr_data.collections.sort_asc = true;
    app_arc
      .lock()
      .await
      .push_navigation_stack(ActiveRadarrBlock::CollectionsSortPrompt.into());
    let cmp_fn = |a: &Collection, b: &Collection| {
      a.title
        .text
        .to_lowercase()
        .cmp(&b.title.text.to_lowercase())
    };
    let collection_sort_option = SortOption {
      name: "Collection",
      cmp_fn: Some(cmp_fn),
    };
    app_arc
      .lock()
      .await
      .data
      .radarr_data
      .collections
      .sorting(vec![collection_sort_option]);
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    assert!(network
      .handle_radarr_event(RadarrEvent::GetCollections)
      .await
      .is_ok());

    async_server.assert_async().await;
    assert!(app_arc
      .lock()
      .await
      .data
      .radarr_data
      .collections
      .items
      .is_empty());
    assert!(app_arc.lock().await.data.radarr_data.collections.sort_asc);
  }

  #[tokio::test]
  async fn test_handle_get_radarr_downloads_event() {
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
    let response: DownloadsResponse =
      serde_json::from_value(downloads_response_json.clone()).unwrap();
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Get,
      None,
      Some(downloads_response_json),
      None,
      RadarrEvent::GetDownloads,
      None,
      None,
    )
    .await;
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    if let RadarrSerdeable::DownloadsResponse(downloads) = network
      .handle_radarr_event(RadarrEvent::GetDownloads)
      .await
      .unwrap()
    {
      async_server.assert_async().await;
      assert_eq!(
        app_arc.lock().await.data.radarr_data.downloads.items,
        downloads_response().records
      );
      assert_eq!(downloads, response);
    }
  }

  #[tokio::test]
  async fn test_handle_get_radarr_host_config_event() {
    let host_config_response = json!({
      "bindAddress": "*",
      "port": 7878,
      "urlBase": "some.test.site/radarr",
      "instanceName": "Radarr",
      "applicationUrl": "https://some.test.site:7878/radarr",
      "enableSsl": true,
      "sslPort": 9898,
      "sslCertPath": "/app/radarr.pfx",
      "sslCertPassword": "test"
    });
    let response: HostConfig = serde_json::from_value(host_config_response.clone()).unwrap();
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Get,
      None,
      Some(host_config_response),
      None,
      RadarrEvent::GetHostConfig,
      None,
      None,
    )
    .await;
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    if let RadarrSerdeable::HostConfig(host_config) = network
      .handle_radarr_event(RadarrEvent::GetHostConfig)
      .await
      .unwrap()
    {
      async_server.assert_async().await;
      assert_eq!(host_config, response);
    }
  }

  #[tokio::test]
  async fn test_handle_get_radarr_indexers_event() {
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
                "name": "baseUrl",
                "value": "https://test.com",
            },
            {
                "name": "apiKey",
                "value": "",
            },
            {
                "name": "seedCriteria.seedRatio",
                "value": "1.2",
            },
        ],
        "implementationName": "Torznab",
        "implementation": "Torznab",
        "configContract": "TorznabSettings",
        "tags": [1],
        "id": 1
    }]);
    let response: Vec<Indexer> = serde_json::from_value(indexers_response_json.clone()).unwrap();
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Get,
      None,
      Some(indexers_response_json),
      None,
      RadarrEvent::GetIndexers,
      None,
      None,
    )
    .await;
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    if let RadarrSerdeable::Indexers(indexers) = network
      .handle_radarr_event(RadarrEvent::GetIndexers)
      .await
      .unwrap()
    {
      async_server.assert_async().await;
      assert_eq!(
        app_arc.lock().await.data.radarr_data.indexers.items,
        vec![indexer()]
      );
      assert_eq!(indexers, response);
    }
  }

  #[tokio::test]
  async fn test_handle_get_all_indexer_settings_event() {
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
    let response: IndexerSettings =
      serde_json::from_value(indexer_settings_response_json.clone()).unwrap();
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Get,
      None,
      Some(indexer_settings_response_json),
      None,
      RadarrEvent::GetAllIndexerSettings,
      None,
      None,
    )
    .await;
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    if let RadarrSerdeable::IndexerSettings(settings) = network
      .handle_radarr_event(RadarrEvent::GetAllIndexerSettings)
      .await
      .unwrap()
    {
      async_server.assert_async().await;
      assert_eq!(
        app_arc.lock().await.data.radarr_data.indexer_settings,
        Some(indexer_settings())
      );
      assert_eq!(settings, response);
    }
  }

  #[tokio::test]
  async fn test_handle_get_all_indexer_settings_event_no_op_if_already_present() {
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
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Get,
      None,
      Some(indexer_settings_response_json),
      None,
      RadarrEvent::GetAllIndexerSettings,
      None,
      None,
    )
    .await;
    app_arc.lock().await.data.radarr_data.indexer_settings = Some(IndexerSettings::default());
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    assert!(network
      .handle_radarr_event(RadarrEvent::GetAllIndexerSettings)
      .await
      .is_ok());

    async_server.assert_async().await;
    assert_eq!(
      app_arc.lock().await.data.radarr_data.indexer_settings,
      Some(IndexerSettings::default())
    );
  }

  #[tokio::test]
  async fn test_handle_get_queued_radarr_events_event() {
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
    let response: Vec<QueueEvent> = serde_json::from_value(queued_events_json.clone()).unwrap();
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

    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Get,
      None,
      Some(queued_events_json),
      None,
      RadarrEvent::GetQueuedEvents,
      None,
      None,
    )
    .await;
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    if let RadarrSerdeable::QueueEvents(events) = network
      .handle_radarr_event(RadarrEvent::GetQueuedEvents)
      .await
      .unwrap()
    {
      async_server.assert_async().await;
      assert_eq!(
        app_arc.lock().await.data.radarr_data.queued_events.items,
        vec![expected_event]
      );
      assert_eq!(events, response);
    }
  }

  #[tokio::test]
  async fn test_handle_get_radarr_logs_event() {
    let expected_logs = vec![
      HorizontallyScrollableText::from(
        "2023-05-20 21:29:16 UTC|FATAL|RadarrError|Some.Big.Bad.Exception|test exception",
      ),
      HorizontallyScrollableText::from("2023-05-20 21:29:16 UTC|INFO|TestLogger|test message"),
    ];
    let logs_response_json = json!({
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
    });
    let response: LogResponse = serde_json::from_value(logs_response_json.clone()).unwrap();
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Get,
      None,
      Some(logs_response_json),
      None,
      RadarrEvent::GetLogs(None),
      None,
      Some("pageSize=500&sortDirection=descending&sortKey=time"),
    )
    .await;
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    if let RadarrSerdeable::LogResponse(logs) = network
      .handle_radarr_event(RadarrEvent::GetLogs(None))
      .await
      .unwrap()
    {
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
      assert_eq!(logs, response);
    }
  }

  #[tokio::test]
  async fn test_handle_get_radarr_logs_event_uses_provided_events() {
    let expected_logs = vec![
      HorizontallyScrollableText::from(
        "2023-05-20 21:29:16 UTC|FATAL|RadarrError|Some.Big.Bad.Exception|test exception",
      ),
      HorizontallyScrollableText::from("2023-05-20 21:29:16 UTC|INFO|TestLogger|test message"),
    ];
    let logs_response_json = json!({
      "page": 1,
      "pageSize": 1000,
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
    });
    let response: LogResponse = serde_json::from_value(logs_response_json.clone()).unwrap();
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Get,
      None,
      Some(logs_response_json),
      None,
      RadarrEvent::GetLogs(Some(1000)),
      None,
      Some("pageSize=1000&sortDirection=descending&sortKey=time"),
    )
    .await;
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    if let RadarrSerdeable::LogResponse(logs) = network
      .handle_radarr_event(RadarrEvent::GetLogs(Some(1000)))
      .await
      .unwrap()
    {
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
      assert_eq!(logs, response);
    }
  }

  #[tokio::test]
  async fn test_handle_get_radarr_quality_profiles_event() {
    let quality_profile_json = json!([{
      "id": 2222,
      "name": "HD - 1080p"
    }]);
    let response: Vec<QualityProfile> =
      serde_json::from_value(quality_profile_json.clone()).unwrap();
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Get,
      None,
      Some(quality_profile_json),
      None,
      RadarrEvent::GetQualityProfiles,
      None,
      None,
    )
    .await;
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    if let RadarrSerdeable::QualityProfiles(quality_profiles) = network
      .handle_radarr_event(RadarrEvent::GetQualityProfiles)
      .await
      .unwrap()
    {
      async_server.assert_async().await;
      assert_eq!(
        app_arc.lock().await.data.radarr_data.quality_profile_map,
        BiMap::from_iter([(2222i64, "HD - 1080p".to_owned())])
      );
      assert_eq!(quality_profiles, response);
    }
  }

  #[tokio::test]
  async fn test_handle_get_tags_event() {
    let tags_json = json!([{
      "id": 2222,
      "label": "usenet"
    }]);
    let response: Vec<Tag> = serde_json::from_value(tags_json.clone()).unwrap();
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Get,
      None,
      Some(tags_json),
      None,
      RadarrEvent::GetTags,
      None,
      None,
    )
    .await;
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    if let RadarrSerdeable::Tags(tags) = network
      .handle_radarr_event(RadarrEvent::GetTags)
      .await
      .unwrap()
    {
      async_server.assert_async().await;
      assert_eq!(
        app_arc.lock().await.data.radarr_data.tags_map,
        BiMap::from_iter([(2222i64, "usenet".to_owned())])
      );
      assert_eq!(tags, response);
    }
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
    let response: Vec<Task> = serde_json::from_value(tasks_json.clone()).unwrap();
    let timestamp = DateTime::from(DateTime::parse_from_rfc3339("2023-05-20T21:29:16Z").unwrap());
    let expected_tasks = vec![
      Task {
        name: "Application Check Update".to_owned(),
        task_name: TaskName::ApplicationCheckUpdate,
        interval: 360,
        last_execution: timestamp,
        next_execution: timestamp,
        last_duration: "00:00:00.5111547".to_owned(),
      },
      Task {
        name: "Backup".to_owned(),
        task_name: TaskName::Backup,
        interval: 10080,
        last_execution: timestamp,
        next_execution: timestamp,
        last_duration: "00:00:00.5111547".to_owned(),
      },
    ];
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Get,
      None,
      Some(tasks_json),
      None,
      RadarrEvent::GetTasks,
      None,
      None,
    )
    .await;
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    if let RadarrSerdeable::Tasks(tasks) = network
      .handle_radarr_event(RadarrEvent::GetTasks)
      .await
      .unwrap()
    {
      async_server.assert_async().await;
      assert_eq!(
        app_arc.lock().await.data.radarr_data.tasks.items,
        expected_tasks
      );
      assert_eq!(tasks, response);
    }
  }

  #[tokio::test]
  async fn test_handle_get_updates_event() {
    let updates_json = json!([{
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
    let response: Vec<Update> = serde_json::from_value(updates_json.clone()).unwrap();
    let line_break = "-".repeat(200);
    let expected_text = ScrollableText::with_string(formatdoc!(
      "
    The latest version of Radarr is already installed

    4.3.2.1 - 2023-04-15 02:02:53 UTC (Currently Installed)
    {line_break}
    New:
      * Cool new thing
    Fixed:
      * Some bugs killed
    
    
    3.2.1.0 - 2023-04-15 02:02:53 UTC (Previously Installed)
    {line_break}
    New:
      * Cool new thing (old)
      * Other cool new thing (old)
    
    
    2.1.0 - 2023-04-15 02:02:53 UTC 
    {line_break}
    Fixed:
      * Killed bug 1
      * Fixed bug 2"
    ));
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Get,
      None,
      Some(updates_json),
      None,
      RadarrEvent::GetUpdates,
      None,
      None,
    )
    .await;
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    if let RadarrSerdeable::Updates(updates) = network
      .handle_radarr_event(RadarrEvent::GetUpdates)
      .await
      .unwrap()
    {
      async_server.assert_async().await;
      assert_str_eq!(
        app_arc.lock().await.data.radarr_data.updates.get_text(),
        expected_text.get_text()
      );
      assert_eq!(updates, response);
    }
  }

  #[tokio::test]
  async fn test_handle_add_radarr_tag() {
    let tag_json = json!({ "id": 3, "label": "testing" });
    let response: Tag = serde_json::from_value(tag_json.clone()).unwrap();
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Post,
      Some(json!({ "label": "testing" })),
      Some(tag_json),
      None,
      RadarrEvent::AddTag(String::new()),
      None,
      None,
    )
    .await;
    app_arc.lock().await.data.radarr_data.tags_map =
      BiMap::from_iter([(1, "usenet".to_owned()), (2, "test".to_owned())]);
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    if let RadarrSerdeable::Tag(tag) = network
      .handle_radarr_event(RadarrEvent::AddTag("testing".to_owned()))
      .await
      .unwrap()
    {
      async_server.assert_async().await;
      assert_eq!(
        app_arc.lock().await.data.radarr_data.tags_map,
        BiMap::from_iter([
          (1, "usenet".to_owned()),
          (2, "test".to_owned()),
          (3, "testing".to_owned())
        ])
      );
      assert_eq!(tag, response);
    }
  }

  #[tokio::test]
  async fn test_handle_delete_tag_event() {
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Delete,
      None,
      None,
      None,
      RadarrEvent::DeleteTag(1),
      Some("/1"),
      None,
    )
    .await;
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    assert!(network
      .handle_radarr_event(RadarrEvent::DeleteTag(1))
      .await
      .is_ok());

    async_server.assert_async().await;
  }

  #[tokio::test]
  async fn test_handle_get_radarr_root_folders_event() {
    let root_folder_json = json!([{
      "id": 1,
      "path": "/nfs",
      "accessible": true,
      "freeSpace": 219902325555200u64,
    }]);
    let response: Vec<RootFolder> = serde_json::from_value(root_folder_json.clone()).unwrap();
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Get,
      None,
      Some(root_folder_json),
      None,
      RadarrEvent::GetRootFolders,
      None,
      None,
    )
    .await;
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    if let RadarrSerdeable::RootFolders(root_folders) = network
      .handle_radarr_event(RadarrEvent::GetRootFolders)
      .await
      .unwrap()
    {
      async_server.assert_async().await;
      assert_eq!(
        app_arc.lock().await.data.radarr_data.root_folders.items,
        vec![root_folder()]
      );
      assert_eq!(root_folders, response);
    }
  }

  #[tokio::test]
  async fn test_handle_get_radarr_security_config_event() {
    let security_config_response = json!({
      "authenticationMethod": "forms",
      "authenticationRequired": "disabledForLocalAddresses",
      "username": "test",
      "password": "some password",
      "apiKey": "someApiKey12345",
      "certificateValidation": "disabledForLocalAddresses",
    });
    let response: SecurityConfig =
      serde_json::from_value(security_config_response.clone()).unwrap();
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Get,
      None,
      Some(security_config_response),
      None,
      RadarrEvent::GetSecurityConfig,
      None,
      None,
    )
    .await;
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    if let RadarrSerdeable::SecurityConfig(security_config) = network
      .handle_radarr_event(RadarrEvent::GetSecurityConfig)
      .await
      .unwrap()
    {
      async_server.assert_async().await;
      assert_eq!(security_config, response);
    }
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
    let response: Vec<Credit> = serde_json::from_value(credits_json.clone()).unwrap();
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Get,
      None,
      Some(credits_json),
      None,
      RadarrEvent::GetMovieCredits(None),
      None,
      Some("movieId=1"),
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
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    if let RadarrSerdeable::Credits(credits) = network
      .handle_radarr_event(RadarrEvent::GetMovieCredits(None))
      .await
      .unwrap()
    {
      let app = app_arc.lock().await;
      let movie_details_modal = app.data.radarr_data.movie_details_modal.as_ref().unwrap();

      async_server.assert_async().await;
      assert_eq!(movie_details_modal.movie_cast.items, vec![cast_credit()]);
      assert_eq!(movie_details_modal.movie_crew.items, vec![crew_credit()]);
      assert_eq!(credits, response);
    }
  }

  #[tokio::test]
  async fn test_handle_get_movie_credits_event_uses_provided_id() {
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
    let response: Vec<Credit> = serde_json::from_value(credits_json.clone()).unwrap();
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Get,
      None,
      Some(credits_json),
      None,
      RadarrEvent::GetMovieCredits(None),
      None,
      Some("movieId=1"),
    )
    .await;
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    if let RadarrSerdeable::Credits(credits) = network
      .handle_radarr_event(RadarrEvent::GetMovieCredits(Some(1)))
      .await
      .unwrap()
    {
      async_server.assert_async().await;
      assert_eq!(credits, response);
    }
  }

  #[tokio::test]
  async fn test_handle_get_movie_credits_event_empty_movie_details_modal() {
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
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Get,
      None,
      Some(credits_json),
      None,
      RadarrEvent::GetMovieCredits(None),
      None,
      Some("movieId=1"),
    )
    .await;
    app_arc
      .lock()
      .await
      .data
      .radarr_data
      .movies
      .set_items(vec![movie()]);
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    assert!(network
      .handle_radarr_event(RadarrEvent::GetMovieCredits(None))
      .await
      .is_ok());

    let app = app_arc.lock().await;
    let movie_details_modal = app.data.radarr_data.movie_details_modal.as_ref().unwrap();

    async_server.assert_async().await;
    assert_eq!(movie_details_modal.movie_cast.items, vec![cast_credit()]);
    assert_eq!(movie_details_modal.movie_crew.items, vec![crew_credit()]);
  }

  #[tokio::test]
  async fn test_handle_delete_movie_event() {
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Delete,
      None,
      None,
      None,
      RadarrEvent::DeleteMovie(None),
      Some("/1"),
      Some("deleteFiles=true&addImportExclusion=true"),
    )
    .await;
    {
      let mut app = app_arc.lock().await;
      app.data.radarr_data.movies.set_items(vec![movie()]);
      app.data.radarr_data.delete_movie_files = true;
      app.data.radarr_data.add_list_exclusion = true;
    }
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    assert!(network
      .handle_radarr_event(RadarrEvent::DeleteMovie(None))
      .await
      .is_ok());

    async_server.assert_async().await;
    assert!(!app_arc.lock().await.data.radarr_data.delete_movie_files);
    assert!(!app_arc.lock().await.data.radarr_data.add_list_exclusion);
  }

  #[tokio::test]
  async fn test_handle_delete_movie_event_use_provided_params() {
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Delete,
      None,
      None,
      None,
      RadarrEvent::DeleteMovie(None),
      Some("/1"),
      Some("deleteFiles=true&addImportExclusion=true"),
    )
    .await;
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());
    let delete_movie_params = DeleteMovieParams {
      id: 1,
      delete_movie_files: true,
      add_list_exclusion: true,
    };

    assert!(network
      .handle_radarr_event(RadarrEvent::DeleteMovie(Some(delete_movie_params)))
      .await
      .is_ok());

    async_server.assert_async().await;
    assert!(!app_arc.lock().await.data.radarr_data.delete_movie_files);
    assert!(!app_arc.lock().await.data.radarr_data.add_list_exclusion);
  }

  #[tokio::test]
  async fn test_handle_clear_radarr_blocklist_event() {
    let blocklist_items = vec![
      BlocklistItem {
        id: 1,
        ..blocklist_item()
      },
      BlocklistItem {
        id: 2,
        ..blocklist_item()
      },
      BlocklistItem {
        id: 3,
        ..blocklist_item()
      },
    ];
    let expected_request_json = json!({ "ids": [1, 2, 3]});
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Delete,
      Some(expected_request_json),
      None,
      None,
      RadarrEvent::ClearBlocklist,
      None,
      None,
    )
    .await;
    app_arc
      .lock()
      .await
      .data
      .radarr_data
      .blocklist
      .set_items(blocklist_items);
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    assert!(network
      .handle_radarr_event(RadarrEvent::ClearBlocklist)
      .await
      .is_ok());

    async_server.assert_async().await;
  }

  #[tokio::test]
  async fn test_handle_delete_radarr_blocklist_item_event() {
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Delete,
      None,
      None,
      None,
      RadarrEvent::DeleteBlocklistItem(None),
      Some("/1"),
      None,
    )
    .await;
    app_arc
      .lock()
      .await
      .data
      .radarr_data
      .blocklist
      .set_items(vec![blocklist_item()]);
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    assert!(network
      .handle_radarr_event(RadarrEvent::DeleteBlocklistItem(None))
      .await
      .is_ok());

    async_server.assert_async().await;
  }

  #[tokio::test]
  async fn test_handle_delete_blocklist_item_event_uses_provided_id() {
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Delete,
      None,
      None,
      None,
      RadarrEvent::DeleteBlocklistItem(None),
      Some("/1"),
      None,
    )
    .await;
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    assert!(network
      .handle_radarr_event(RadarrEvent::DeleteBlocklistItem(Some(1)))
      .await
      .is_ok());

    async_server.assert_async().await;
  }

  #[tokio::test]
  async fn test_handle_delete_radarr_download_event() {
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Delete,
      None,
      None,
      None,
      RadarrEvent::DeleteDownload(None),
      Some("/1"),
      None,
    )
    .await;
    app_arc
      .lock()
      .await
      .data
      .radarr_data
      .downloads
      .set_items(vec![download_record()]);
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    assert!(network
      .handle_radarr_event(RadarrEvent::DeleteDownload(None))
      .await
      .is_ok());

    async_server.assert_async().await;
  }

  #[tokio::test]
  async fn test_handle_delete_radarr_download_event_uses_provided_id() {
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Delete,
      None,
      None,
      None,
      RadarrEvent::DeleteDownload(None),
      Some("/1"),
      None,
    )
    .await;
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    assert!(network
      .handle_radarr_event(RadarrEvent::DeleteDownload(Some(1)))
      .await
      .is_ok());

    async_server.assert_async().await;
  }

  #[tokio::test]
  async fn test_handle_delete_radarr_indexer_event() {
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Delete,
      None,
      None,
      None,
      RadarrEvent::DeleteIndexer(None),
      Some("/1"),
      None,
    )
    .await;
    app_arc
      .lock()
      .await
      .data
      .radarr_data
      .indexers
      .set_items(vec![indexer()]);
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    assert!(network
      .handle_radarr_event(RadarrEvent::DeleteIndexer(None))
      .await
      .is_ok());

    async_server.assert_async().await;
  }

  #[tokio::test]
  async fn test_handle_delete_radarr_indexer_event_uses_provided_id() {
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Delete,
      None,
      None,
      None,
      RadarrEvent::DeleteIndexer(None),
      Some("/1"),
      None,
    )
    .await;
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    assert!(network
      .handle_radarr_event(RadarrEvent::DeleteIndexer(Some(1)))
      .await
      .is_ok());

    async_server.assert_async().await;
  }

  #[tokio::test]
  async fn test_handle_delete_radarr_root_folder_event() {
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Delete,
      None,
      None,
      None,
      RadarrEvent::DeleteRootFolder(None),
      Some("/1"),
      None,
    )
    .await;
    app_arc
      .lock()
      .await
      .data
      .radarr_data
      .root_folders
      .set_items(vec![root_folder()]);
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    assert!(network
      .handle_radarr_event(RadarrEvent::DeleteRootFolder(None))
      .await
      .is_ok());

    async_server.assert_async().await;
  }

  #[tokio::test]
  async fn test_handle_delete_radarr_root_folder_event_uses_provided_id() {
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Delete,
      None,
      None,
      None,
      RadarrEvent::DeleteRootFolder(None),
      Some("/1"),
      None,
    )
    .await;
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    assert!(network
      .handle_radarr_event(RadarrEvent::DeleteRootFolder(Some(1)))
      .await
      .is_ok());

    async_server.assert_async().await;
  }

  #[rstest]
  #[tokio::test]
  async fn test_handle_add_movie_event(#[values(true, false)] movie_details_context: bool) {
    let (async_server, app_arc, _server) = mock_servarr_api(
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
      Some(json!({})),
      None,
      RadarrEvent::AddMovie(None),
      None,
      None,
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
          id: 1,
          path: "/nfs".to_owned(),
          accessible: true,
          free_space: 219902325555200,
          unmapped_folders: None,
        },
        RootFolder {
          id: 2,
          path: "/nfs2".to_owned(),
          accessible: true,
          free_space: 21990232555520,
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
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    assert!(network
      .handle_radarr_event(RadarrEvent::AddMovie(None))
      .await
      .is_ok());

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
  async fn test_handle_add_movie_event_uses_provided_body() {
    let (async_server, app_arc, _server) = mock_servarr_api(
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
      Some(json!({})),
      None,
      RadarrEvent::AddMovie(None),
      None,
      None,
    )
    .await;
    let body = AddMovieBody {
      tmdb_id: 1234,
      title: "Test".to_owned(),
      root_folder_path: "/nfs2".to_owned(),
      minimum_availability: "announced".to_owned(),
      monitored: true,
      quality_profile_id: 2222,
      tags: vec![1, 2],
      add_options: AddOptions {
        monitor: "movieOnly".to_owned(),
        search_for_movie: true,
      },
    };

    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    assert!(network
      .handle_radarr_event(RadarrEvent::AddMovie(Some(body)))
      .await
      .is_ok());

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
    let (async_server, app_arc, _server) = mock_servarr_api(
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
      Some(json!({})),
      None,
      RadarrEvent::AddMovie(None),
      None,
      None,
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
          id: 1,
          path: "/nfs".to_owned(),
          accessible: true,
          free_space: 219902325555200,
          unmapped_folders: None,
        },
        RootFolder {
          id: 2,
          path: "/nfs2".to_owned(),
          accessible: true,
          free_space: 21990232555520,
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
        tmdb_id: 5678,
        ..add_movie_search_result()
      };
      let mut add_searched_movies = StatefulTable::default();
      add_searched_movies.set_items(vec![add_movie_search_result(), secondary_search_result]);
      add_searched_movies.scroll_to_bottom();
      app.data.radarr_data.add_searched_movies = Some(add_searched_movies);
    }
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    assert!(network
      .handle_radarr_event(RadarrEvent::AddMovie(None))
      .await
      .is_ok());

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
        .tmdb_id,
      5678
    );
  }

  #[tokio::test]
  async fn test_handle_add_radarr_root_folder_event() {
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Post,
      Some(json!({
        "path": "/nfs/test"
      })),
      Some(json!({})),
      None,
      RadarrEvent::AddRootFolder(None),
      None,
      None,
    )
    .await;

    app_arc.lock().await.data.radarr_data.edit_root_folder = Some("/nfs/test".into());
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    assert!(network
      .handle_radarr_event(RadarrEvent::AddRootFolder(None))
      .await
      .is_ok());

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
  async fn test_handle_add_radarr_root_folder_event_uses_provided_path() {
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Post,
      Some(json!({
        "path": "/test/test"
      })),
      Some(json!({})),
      None,
      RadarrEvent::AddRootFolder(None),
      None,
      None,
    )
    .await;

    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    assert!(network
      .handle_radarr_event(RadarrEvent::AddRootFolder(Some("/test/test".to_owned())))
      .await
      .is_ok());

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
  async fn test_handle_edit_all_indexer_settings_event() {
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
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Put,
      Some(indexer_settings_json),
      None,
      None,
      RadarrEvent::EditAllIndexerSettings(None),
      None,
      None,
    )
    .await;

    app_arc.lock().await.data.radarr_data.indexer_settings = Some(indexer_settings());
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    assert!(network
      .handle_radarr_event(RadarrEvent::EditAllIndexerSettings(None))
      .await
      .is_ok());

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
  async fn test_handle_edit_all_indexer_settings_event_uses_provided_settings() {
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
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Put,
      Some(indexer_settings_json),
      None,
      None,
      RadarrEvent::EditAllIndexerSettings(None),
      None,
      None,
    )
    .await;

    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    assert!(network
      .handle_radarr_event(RadarrEvent::EditAllIndexerSettings(
        Some(indexer_settings())
      ))
      .await
      .is_ok());

    async_server.assert_async().await;
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

    let (async_details_server, app_arc, mut server) = mock_servarr_api(
      RequestMethod::Get,
      None,
      Some(detailed_collection_body),
      None,
      RadarrEvent::GetCollections,
      Some("/123"),
      None,
    )
    .await;
    let async_edit_server = server
      .mock(
        "PUT",
        format!(
          "/api/v3{}/123",
          RadarrEvent::EditCollection(None).resource()
        )
        .as_str(),
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
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    assert!(network
      .handle_radarr_event(RadarrEvent::EditCollection(None))
      .await
      .is_ok());

    async_details_server.assert_async().await;
    async_edit_server.assert_async().await;

    let app = app_arc.lock().await;
    assert!(app.data.radarr_data.edit_collection_modal.is_none());
  }

  #[tokio::test]
  async fn test_handle_edit_collection_event_uses_provided_parameters() {
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

    let (async_details_server, app_arc, mut server) = mock_servarr_api(
      RequestMethod::Get,
      None,
      Some(detailed_collection_body),
      None,
      RadarrEvent::GetCollections,
      Some("/123"),
      None,
    )
    .await;
    let async_edit_server = server
      .mock(
        "PUT",
        format!(
          "/api/v3{}/123",
          RadarrEvent::EditCollection(None).resource()
        )
        .as_str(),
      )
      .with_status(202)
      .match_header("X-Api-Key", "test1234")
      .match_body(Matcher::Json(expected_body))
      .create_async()
      .await;
    let edit_collection_params = EditCollectionParams {
      collection_id: 123,
      monitored: Some(false),
      minimum_availability: Some(MinimumAvailability::Announced),
      quality_profile_id: Some(1111),
      root_folder_path: Some("/nfs/Test Path".to_owned()),
      search_on_add: Some(false),
    };
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    assert!(network
      .handle_radarr_event(RadarrEvent::EditCollection(Some(edit_collection_params)))
      .await
      .is_ok());

    async_details_server.assert_async().await;
    async_edit_server.assert_async().await;
  }

  #[tokio::test]
  async fn test_handle_edit_collection_event_uses_provided_parameters_defaults_to_previous_values_when_none(
  ) {
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
    *expected_body.get_mut("monitored").unwrap() = json!(true);
    *expected_body.get_mut("minimumAvailability").unwrap() = json!("released");
    *expected_body.get_mut("qualityProfileId").unwrap() = json!(2222);
    *expected_body.get_mut("rootFolderPath").unwrap() = json!("/nfs/movies");
    *expected_body.get_mut("searchOnAdd").unwrap() = json!(true);

    let (async_details_server, app_arc, mut server) = mock_servarr_api(
      RequestMethod::Get,
      None,
      Some(detailed_collection_body),
      None,
      RadarrEvent::GetCollections,
      Some("/123"),
      None,
    )
    .await;
    let async_edit_server = server
      .mock(
        "PUT",
        format!(
          "/api/v3{}/123",
          RadarrEvent::EditCollection(None).resource()
        )
        .as_str(),
      )
      .with_status(202)
      .match_header("X-Api-Key", "test1234")
      .match_body(Matcher::Json(expected_body))
      .create_async()
      .await;
    let edit_collection_params = EditCollectionParams {
      collection_id: 123,
      ..EditCollectionParams::default()
    };
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    assert!(network
      .handle_radarr_event(RadarrEvent::EditCollection(Some(edit_collection_params)))
      .await
      .is_ok());

    async_details_server.assert_async().await;
    async_edit_server.assert_async().await;
  }

  #[tokio::test]
  async fn test_handle_edit_indexer_event() {
    let indexer_details_json = json!({
        "enableRss": true,
        "enableAutomaticSearch": true,
        "enableInteractiveSearch": true,
        "name": "Test Indexer",
        "priority": 1,
        "fields": [
            {
                "name": "baseUrl",
                "value": "https://test.com",
            },
            {
                "name": "apiKey",
                "value": "",
            },
            {
                "name": "seedCriteria.seedRatio",
                "value": "1.2",
            },
        ],
        "tags": [1],
        "id": 1
    });
    let expected_indexer_edit_body_json = json!({
        "enableRss": false,
        "enableAutomaticSearch": false,
        "enableInteractiveSearch": false,
        "name": "Test Update",
        "priority": 1,
        "fields": [
            {
                "name": "baseUrl",
                "value": "https://localhost:9696/1/",
            },
            {
                "name": "apiKey",
                "value": "test1234",
            },
            {
                "name": "seedCriteria.seedRatio",
                "value": "1.3",
            },
        ],
        "tags": [1, 2],
        "id": 1
    });

    let (async_details_server, app_arc, mut server) = mock_servarr_api(
      RequestMethod::Get,
      None,
      Some(indexer_details_json),
      None,
      RadarrEvent::GetIndexers,
      Some("/1"),
      None,
    )
    .await;
    let async_edit_server = server
      .mock(
        "PUT",
        format!("/api/v3{}/1", RadarrEvent::EditIndexer(None).resource()).as_str(),
      )
      .with_status(202)
      .match_header("X-Api-Key", "test1234")
      .match_body(Matcher::Json(expected_indexer_edit_body_json))
      .create_async()
      .await;
    {
      let mut app = app_arc.lock().await;
      app.data.radarr_data.tags_map =
        BiMap::from_iter([(1, "usenet".to_owned()), (2, "testing".to_owned())]);
      let edit_indexer_modal = EditIndexerModal {
        name: "Test Update".into(),
        enable_rss: Some(false),
        enable_automatic_search: Some(false),
        enable_interactive_search: Some(false),
        url: "https://localhost:9696/1/".into(),
        api_key: "test1234".into(),
        seed_ratio: "1.3".into(),
        tags: "usenet, testing".into(),
      };
      app.data.radarr_data.edit_indexer_modal = Some(edit_indexer_modal);
      app.data.radarr_data.indexers.set_items(vec![indexer()]);
    }
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    assert!(network
      .handle_radarr_event(RadarrEvent::EditIndexer(None))
      .await
      .is_ok());

    async_details_server.assert_async().await;
    async_edit_server.assert_async().await;

    let app = app_arc.lock().await;
    assert!(app.data.radarr_data.edit_indexer_modal.is_none());
  }

  #[tokio::test]
  async fn test_handle_edit_indexer_event_does_not_add_seed_ratio_when_seed_ratio_field_is_none_in_details(
  ) {
    let indexer_details_json = json!({
        "enableRss": true,
        "enableAutomaticSearch": true,
        "enableInteractiveSearch": true,
        "name": "Test Indexer",
        "priority": 1,
        "fields": [
            {
                "name": "baseUrl",
                "value": "https://test.com",
            },
            {
                "name": "apiKey",
                "value": "",
            },
        ],
        "tags": [1],
        "id": 1
    });
    let expected_indexer_edit_body_json = json!({
        "enableRss": false,
        "enableAutomaticSearch": false,
        "enableInteractiveSearch": false,
        "name": "Test Update",
        "priority": 1,
        "fields": [
            {
                "name": "baseUrl",
                "value": "https://localhost:9696/1/",
            },
            {
                "name": "apiKey",
                "value": "test1234",
            },
        ],
        "tags": [1, 2],
        "id": 1
    });

    let (async_details_server, app_arc, mut server) = mock_servarr_api(
      RequestMethod::Get,
      None,
      Some(indexer_details_json),
      None,
      RadarrEvent::GetIndexers,
      Some("/1"),
      None,
    )
    .await;
    let async_edit_server = server
      .mock(
        "PUT",
        format!("/api/v3{}/1", RadarrEvent::EditIndexer(None).resource()).as_str(),
      )
      .with_status(202)
      .match_header("X-Api-Key", "test1234")
      .match_body(Matcher::Json(expected_indexer_edit_body_json))
      .create_async()
      .await;
    {
      let mut app = app_arc.lock().await;
      app.data.radarr_data.tags_map =
        BiMap::from_iter([(1, "usenet".to_owned()), (2, "testing".to_owned())]);
      let edit_indexer_modal = EditIndexerModal {
        name: "Test Update".into(),
        enable_rss: Some(false),
        enable_automatic_search: Some(false),
        enable_interactive_search: Some(false),
        url: "https://localhost:9696/1/".into(),
        api_key: "test1234".into(),
        seed_ratio: "1.3".into(),
        tags: "usenet, testing".into(),
      };
      app.data.radarr_data.edit_indexer_modal = Some(edit_indexer_modal);
      let mut indexer = indexer();
      indexer.fields = Some(
        indexer
          .fields
          .unwrap()
          .into_iter()
          .filter(|field| field.name != Some("seedCriteria.seedRatio".to_string()))
          .collect(),
      );
      app.data.radarr_data.indexers.set_items(vec![indexer]);
    }
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    assert!(network
      .handle_radarr_event(RadarrEvent::EditIndexer(None))
      .await
      .is_ok());

    async_details_server.assert_async().await;
    async_edit_server.assert_async().await;

    let app = app_arc.lock().await;
    assert!(app.data.radarr_data.edit_indexer_modal.is_none());
  }

  #[tokio::test]
  async fn test_handle_edit_indexer_event_populates_the_seed_ratio_value_when_seed_ratio_field_is_present_in_details(
  ) {
    let indexer_details_json = json!({
        "enableRss": true,
        "enableAutomaticSearch": true,
        "enableInteractiveSearch": true,
        "name": "Test Indexer",
        "priority": 1,
        "fields": [
            {
                "name": "baseUrl",
                "value": "https://test.com",
            },
            {
                "name": "apiKey",
                "value": "",
            },
            {
                "name": "seedCriteria.seedRatio",
            },
        ],
        "tags": [1],
        "id": 1
    });
    let expected_indexer_edit_body_json = json!({
        "enableRss": false,
        "enableAutomaticSearch": false,
        "enableInteractiveSearch": false,
        "name": "Test Update",
        "priority": 1,
        "fields": [
            {
                "name": "baseUrl",
                "value": "https://localhost:9696/1/",
            },
            {
                "name": "apiKey",
                "value": "test1234",
            },
            {
                "name": "seedCriteria.seedRatio",
                "value": "1.3",
            },
        ],
        "tags": [1, 2],
        "id": 1
    });

    let (async_details_server, app_arc, mut server) = mock_servarr_api(
      RequestMethod::Get,
      None,
      Some(indexer_details_json),
      None,
      RadarrEvent::GetIndexers,
      Some("/1"),
      None,
    )
    .await;
    let async_edit_server = server
      .mock(
        "PUT",
        format!("/api/v3{}/1", RadarrEvent::EditIndexer(None).resource()).as_str(),
      )
      .with_status(202)
      .match_header("X-Api-Key", "test1234")
      .match_body(Matcher::Json(expected_indexer_edit_body_json))
      .create_async()
      .await;
    {
      let mut app = app_arc.lock().await;
      app.data.radarr_data.tags_map =
        BiMap::from_iter([(1, "usenet".to_owned()), (2, "testing".to_owned())]);
      let edit_indexer_modal = EditIndexerModal {
        name: "Test Update".into(),
        enable_rss: Some(false),
        enable_automatic_search: Some(false),
        enable_interactive_search: Some(false),
        url: "https://localhost:9696/1/".into(),
        api_key: "test1234".into(),
        seed_ratio: "1.3".into(),
        tags: "usenet, testing".into(),
      };
      app.data.radarr_data.edit_indexer_modal = Some(edit_indexer_modal);
      let mut indexer = indexer();
      indexer.fields = Some(
        indexer
          .fields
          .unwrap()
          .into_iter()
          .map(|mut field| {
            if field.name == Some("seedCriteria.seedRatio".to_string()) {
              field.value = None;
              field
            } else {
              field
            }
          })
          .collect(),
      );
      app.data.radarr_data.indexers.set_items(vec![indexer]);
    }
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    assert!(network
      .handle_radarr_event(RadarrEvent::EditIndexer(None))
      .await
      .is_ok());

    async_details_server.assert_async().await;
    async_edit_server.assert_async().await;

    let app = app_arc.lock().await;
    assert!(app.data.radarr_data.edit_indexer_modal.is_none());
  }

  #[tokio::test]
  async fn test_handle_edit_indexer_event_uses_provided_parameters() {
    let indexer_details_json = json!({
        "enableRss": true,
        "enableAutomaticSearch": true,
        "enableInteractiveSearch": true,
        "name": "Test Indexer",
        "priority": 1,
        "fields": [
            {
                "name": "baseUrl",
                "value": "https://test.com",
            },
            {
                "name": "apiKey",
                "value": "",
            },
            {
                "name": "seedCriteria.seedRatio",
                "value": "1.2",
            },
        ],
        "tags": [1],
        "id": 1
    });
    let expected_indexer_edit_body_json = json!({
        "enableRss": false,
        "enableAutomaticSearch": false,
        "enableInteractiveSearch": false,
        "name": "Test Update",
        "priority": 25,
        "fields": [
            {
                "name": "baseUrl",
                "value": "https://localhost:9696/1/",
            },
            {
                "name": "apiKey",
                "value": "test1234",
            },
            {
                "name": "seedCriteria.seedRatio",
                "value": "1.3",
            },
        ],
        "tags": [1, 2],
        "id": 1
    });
    let edit_indexer_params = EditIndexerParams {
      indexer_id: 1,
      name: Some("Test Update".to_owned()),
      enable_rss: Some(false),
      enable_automatic_search: Some(false),
      enable_interactive_search: Some(false),
      url: Some("https://localhost:9696/1/".to_owned()),
      api_key: Some("test1234".to_owned()),
      seed_ratio: Some("1.3".to_owned()),
      tags: Some(vec![1, 2]),
      priority: Some(25),
      ..EditIndexerParams::default()
    };

    let (async_details_server, app_arc, mut server) = mock_servarr_api(
      RequestMethod::Get,
      None,
      Some(indexer_details_json),
      None,
      RadarrEvent::GetIndexers,
      Some("/1"),
      None,
    )
    .await;
    let async_edit_server = server
      .mock(
        "PUT",
        format!("/api/v3{}/1", RadarrEvent::EditIndexer(None).resource()).as_str(),
      )
      .with_status(202)
      .match_header("X-Api-Key", "test1234")
      .match_body(Matcher::Json(expected_indexer_edit_body_json))
      .create_async()
      .await;
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    assert!(network
      .handle_radarr_event(RadarrEvent::EditIndexer(Some(edit_indexer_params)))
      .await
      .is_ok());

    async_details_server.assert_async().await;
    async_edit_server.assert_async().await;
  }

  #[tokio::test]
  async fn test_handle_edit_indexer_event_uses_provided_parameters_defaults_to_previous_values() {
    let indexer_details_json = json!({
        "enableRss": true,
        "enableAutomaticSearch": true,
        "enableInteractiveSearch": true,
        "name": "Test Indexer",
        "priority": 1,
        "fields": [
            {
                "name": "baseUrl",
                "value": "https://test.com",
            },
            {
                "name": "apiKey",
                "value": "",
            },
            {
                "name": "seedCriteria.seedRatio",
                "value": "1.2",
            },
        ],
        "tags": [1],
        "id": 1
    });
    let edit_indexer_params = EditIndexerParams {
      indexer_id: 1,
      ..EditIndexerParams::default()
    };

    let (async_details_server, app_arc, mut server) = mock_servarr_api(
      RequestMethod::Get,
      None,
      Some(indexer_details_json.clone()),
      None,
      RadarrEvent::GetIndexers,
      Some("/1"),
      None,
    )
    .await;
    let async_edit_server = server
      .mock(
        "PUT",
        format!("/api/v3{}/1", RadarrEvent::EditIndexer(None).resource()).as_str(),
      )
      .with_status(202)
      .match_header("X-Api-Key", "test1234")
      .match_body(Matcher::Json(indexer_details_json))
      .create_async()
      .await;
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    assert!(network
      .handle_radarr_event(RadarrEvent::EditIndexer(Some(edit_indexer_params)))
      .await
      .is_ok());

    async_details_server.assert_async().await;
    async_edit_server.assert_async().await;
  }

  #[tokio::test]
  async fn test_handle_edit_indexer_event_uses_provided_parameters_clears_tags_when_clear_tags_is_true(
  ) {
    let indexer_details_json = json!({
        "enableRss": true,
        "enableAutomaticSearch": true,
        "enableInteractiveSearch": true,
        "name": "Test Indexer",
        "priority": 1,
        "fields": [
            {
                "name": "baseUrl",
                "value": "https://test.com",
            },
            {
                "name": "apiKey",
                "value": "",
            },
            {
                "name": "seedCriteria.seedRatio",
                "value": "1.2",
            },
        ],
        "tags": [1, 2],
        "id": 1
    });
    let expected_edit_indexer_body = json!({
        "enableRss": true,
        "enableAutomaticSearch": true,
        "enableInteractiveSearch": true,
        "name": "Test Indexer",
        "priority": 1,
        "fields": [
            {
                "name": "baseUrl",
                "value": "https://test.com",
            },
            {
                "name": "apiKey",
                "value": "",
            },
            {
                "name": "seedCriteria.seedRatio",
                "value": "1.2",
            },
        ],
        "tags": [],
        "id": 1
    });
    let edit_indexer_params = EditIndexerParams {
      indexer_id: 1,
      clear_tags: true,
      ..EditIndexerParams::default()
    };

    let (async_details_server, app_arc, mut server) = mock_servarr_api(
      RequestMethod::Get,
      None,
      Some(indexer_details_json),
      None,
      RadarrEvent::GetIndexers,
      Some("/1"),
      None,
    )
    .await;
    let async_edit_server = server
      .mock(
        "PUT",
        format!("/api/v3{}/1", RadarrEvent::EditIndexer(None).resource()).as_str(),
      )
      .with_status(202)
      .match_header("X-Api-Key", "test1234")
      .match_body(Matcher::Json(expected_edit_indexer_body))
      .create_async()
      .await;
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    assert!(network
      .handle_radarr_event(RadarrEvent::EditIndexer(Some(edit_indexer_params)))
      .await
      .is_ok());

    async_details_server.assert_async().await;
    async_edit_server.assert_async().await;
  }

  #[tokio::test]
  async fn test_handle_edit_movie_event() {
    let mut expected_body: Value = serde_json::from_str(MOVIE_JSON).unwrap();
    *expected_body.get_mut("monitored").unwrap() = json!(false);
    *expected_body.get_mut("minimumAvailability").unwrap() = json!("announced");
    *expected_body.get_mut("qualityProfileId").unwrap() = json!(1111);
    *expected_body.get_mut("path").unwrap() = json!("/nfs/Test Path");
    *expected_body.get_mut("tags").unwrap() = json!([1, 2]);

    let (async_details_server, app_arc, mut server) = mock_servarr_api(
      RequestMethod::Get,
      None,
      Some(serde_json::from_str(MOVIE_JSON).unwrap()),
      None,
      RadarrEvent::GetMovieDetails(None),
      Some("/1"),
      None,
    )
    .await;
    let async_edit_server = server
      .mock(
        "PUT",
        format!("/api/v3{}/1", RadarrEvent::EditMovie(None).resource()).as_str(),
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
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    assert!(network
      .handle_radarr_event(RadarrEvent::EditMovie(None))
      .await
      .is_ok());

    async_details_server.assert_async().await;
    async_edit_server.assert_async().await;

    let app = app_arc.lock().await;
    assert!(app.data.radarr_data.edit_movie_modal.is_none());
  }

  #[tokio::test]
  async fn test_handle_edit_movie_event_uses_provided_parameters() {
    let mut expected_body: Value = serde_json::from_str(MOVIE_JSON).unwrap();
    *expected_body.get_mut("monitored").unwrap() = json!(false);
    *expected_body.get_mut("minimumAvailability").unwrap() = json!("announced");
    *expected_body.get_mut("qualityProfileId").unwrap() = json!(1111);
    *expected_body.get_mut("path").unwrap() = json!("/nfs/Test Path");
    *expected_body.get_mut("tags").unwrap() = json!([1, 2]);

    let (async_details_server, app_arc, mut server) = mock_servarr_api(
      RequestMethod::Get,
      None,
      Some(serde_json::from_str(MOVIE_JSON).unwrap()),
      None,
      RadarrEvent::GetMovieDetails(None),
      Some("/1"),
      None,
    )
    .await;
    let async_edit_server = server
      .mock(
        "PUT",
        format!("/api/v3{}/1", RadarrEvent::EditMovie(None).resource()).as_str(),
      )
      .with_status(202)
      .match_header("X-Api-Key", "test1234")
      .match_body(Matcher::Json(expected_body))
      .create_async()
      .await;
    let edit_movie_params = EditMovieParams {
      movie_id: 1,
      monitored: Some(false),
      minimum_availability: Some(MinimumAvailability::Announced),
      quality_profile_id: Some(1111),
      root_folder_path: Some("/nfs/Test Path".to_owned()),
      tags: Some(vec![1, 2]),
      ..EditMovieParams::default()
    };
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    assert!(network
      .handle_radarr_event(RadarrEvent::EditMovie(Some(edit_movie_params)))
      .await
      .is_ok());

    async_details_server.assert_async().await;
    async_edit_server.assert_async().await;
  }

  #[tokio::test]
  async fn test_handle_edit_movie_event_uses_provided_parameters_defaults_to_previous_values() {
    let expected_body: Value = serde_json::from_str(MOVIE_JSON).unwrap();
    let (async_details_server, app_arc, mut server) = mock_servarr_api(
      RequestMethod::Get,
      None,
      Some(serde_json::from_str(MOVIE_JSON).unwrap()),
      None,
      RadarrEvent::GetMovieDetails(None),
      Some("/1"),
      None,
    )
    .await;
    let async_edit_server = server
      .mock(
        "PUT",
        format!("/api/v3{}/1", RadarrEvent::EditMovie(None).resource()).as_str(),
      )
      .with_status(202)
      .match_header("X-Api-Key", "test1234")
      .match_body(Matcher::Json(expected_body))
      .create_async()
      .await;
    let edit_movie_params = EditMovieParams {
      movie_id: 1,
      ..EditMovieParams::default()
    };
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    assert!(network
      .handle_radarr_event(RadarrEvent::EditMovie(Some(edit_movie_params)))
      .await
      .is_ok());

    async_details_server.assert_async().await;
    async_edit_server.assert_async().await;
  }

  #[tokio::test]
  async fn test_handle_edit_movie_event_uses_provided_parameters_returns_empty_tags_vec_when_clear_tags_is_true(
  ) {
    let mut expected_body: Value = serde_json::from_str(MOVIE_JSON).unwrap();
    *expected_body.get_mut("tags").unwrap() = json!([]);

    let (async_details_server, app_arc, mut server) = mock_servarr_api(
      RequestMethod::Get,
      None,
      Some(serde_json::from_str(MOVIE_JSON).unwrap()),
      None,
      RadarrEvent::GetMovieDetails(None),
      Some("/1"),
      None,
    )
    .await;
    let async_edit_server = server
      .mock(
        "PUT",
        format!("/api/v3{}/1", RadarrEvent::EditMovie(None).resource()).as_str(),
      )
      .with_status(202)
      .match_header("X-Api-Key", "test1234")
      .match_body(Matcher::Json(expected_body))
      .create_async()
      .await;
    let edit_movie_params = EditMovieParams {
      movie_id: 1,
      clear_tags: true,
      ..EditMovieParams::default()
    };
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    assert!(network
      .handle_radarr_event(RadarrEvent::EditMovie(Some(edit_movie_params)))
      .await
      .is_ok());

    async_details_server.assert_async().await;
    async_edit_server.assert_async().await;
  }

  #[tokio::test]
  async fn test_handle_download_release_event() {
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Post,
      Some(json!({
        "guid": "1234",
        "indexerId": 2,
        "movieId": 1
      })),
      Some(json!({})),
      None,
      RadarrEvent::DownloadRelease(None),
      None,
      None,
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
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    assert!(network
      .handle_radarr_event(RadarrEvent::DownloadRelease(None))
      .await
      .is_ok());

    async_server.assert_async().await;
  }

  #[tokio::test]
  async fn test_handle_download_release_event_uses_provided_params() {
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Post,
      Some(json!({
        "guid": "1234",
        "indexerId": 2,
        "movieId": 1
      })),
      Some(json!({})),
      None,
      RadarrEvent::DownloadRelease(None),
      None,
      None,
    )
    .await;
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());
    let params = ReleaseDownloadBody {
      guid: "1234".to_owned(),
      indexer_id: 2,
      movie_id: 1,
    };

    assert!(network
      .handle_radarr_event(RadarrEvent::DownloadRelease(Some(params)))
      .await
      .is_ok());

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
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    assert_eq!(
      network.extract_and_add_tag_ids_vec(tags).await,
      vec![2, 3, 1]
    );
  }

  #[tokio::test]
  async fn test_extract_and_add_tag_ids_vec_add_missing_tags_first() {
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Post,
      Some(json!({ "label": "testing" })),
      Some(json!({ "id": 3, "label": "testing" })),
      None,
      RadarrEvent::GetTags,
      None,
      None,
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
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

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
        id: 1,
        ..Movie::default()
      }]);
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    let (id, movie_id_param) = network.extract_movie_id(None).await;

    assert_eq!(id, 1);
    assert_str_eq!(movie_id_param, "movieId=1");
  }

  #[tokio::test]
  async fn test_extract_movie_id_uses_provided_id() {
    let app_arc = Arc::new(Mutex::new(App::default()));
    app_arc
      .lock()
      .await
      .data
      .radarr_data
      .movies
      .set_items(vec![Movie {
        id: 1,
        ..Movie::default()
      }]);
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    let (id, movie_id_param) = network.extract_movie_id(Some(2)).await;

    assert_eq!(id, 2);
    assert_str_eq!(movie_id_param, "movieId=2");
  }

  #[tokio::test]
  async fn test_extract_movie_id_filtered_movies() {
    let app_arc = Arc::new(Mutex::new(App::default()));
    let mut filtered_movies = StatefulTable::default();
    filtered_movies.set_filtered_items(vec![Movie {
      id: 1,
      ..Movie::default()
    }]);
    app_arc.lock().await.data.radarr_data.movies = filtered_movies;
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    let (id, movie_id_param) = network.extract_movie_id(None).await;

    assert_eq!(id, 1);
    assert_str_eq!(movie_id_param, "movieId=1");
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
        id: 1,
        ..Collection::default()
      }]);
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    assert_eq!(network.extract_collection_id().await, 1);
  }

  #[tokio::test]
  async fn test_extract_collection_id_filtered_collection() {
    let app_arc = Arc::new(Mutex::new(App::default()));
    let mut filtered_collections = StatefulTable::default();
    filtered_collections.set_filtered_items(vec![Collection {
      id: 1,
      ..Collection::default()
    }]);
    app_arc.lock().await.data.radarr_data.collections = filtered_collections;
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    assert_eq!(network.extract_collection_id().await, 1);
  }

  #[test]
  fn test_get_movie_status_downloaded() {
    assert_str_eq!(get_movie_status(true, &[], 0), "Downloaded");
  }

  #[test]
  fn test_get_movie_status_missing() {
    let download_record = DownloadRecord {
      movie_id: 1,
      ..DownloadRecord::default()
    };

    assert_str_eq!(
      get_movie_status(false, &[download_record.clone()], 0),
      "Missing"
    );

    assert_str_eq!(get_movie_status(false, &[download_record], 1), "Missing");
  }

  #[test]
  fn test_get_movie_status_downloading() {
    assert_str_eq!(
      get_movie_status(
        false,
        &[DownloadRecord {
          movie_id: 1,
          status: "downloading".to_owned(),
          ..DownloadRecord::default()
        }],
        1
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
          movie_id: 1,
          status: "completed".to_owned(),
          ..DownloadRecord::default()
        }],
        1
      ),
      "Awaiting Import"
    );
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
      audio_bitrate: 0,
      audio_channels: Number::from_f64(7.1).unwrap(),
      audio_codec: Some("AAC".to_owned()),
      audio_languages: Some("eng".to_owned()),
      audio_stream_count: 1,
      video_bit_depth: 10,
      video_bitrate: 0,
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
      year: 2023,
      runtime: 120,
      tmdb_id: 1234,
      genres: genres(),
      ratings: ratings_list(),
    }
  }

  fn blocklist_item() -> BlocklistItem {
    BlocklistItem {
      id: 1,
      movie_id: 1,
      source_title: "z movie".to_owned(),
      languages: vec![language()],
      quality: quality_wrapper(),
      custom_formats: Some(vec![language()]),
      date: DateTime::from(DateTime::parse_from_rfc3339("2024-02-10T07:28:45Z").unwrap()),
      protocol: "usenet".to_owned(),
      indexer: "DrunkenSlug (Prowlarr)".to_owned(),
      message: "test message".to_owned(),
      movie: blocklist_item_movie(),
    }
  }

  fn blocklist_item_movie() -> BlocklistItemMovie {
    BlocklistItemMovie {
      title: "Test".into(),
    }
  }

  fn collection() -> Collection {
    Collection {
      id: 123,
      title: "Test Collection".to_owned().into(),
      root_folder_path: Some("/nfs/movies".to_owned()),
      search_on_add: true,
      monitored: true,
      minimum_availability: MinimumAvailability::Released,
      overview: Some("Collection blah blah blah".to_owned()),
      quality_profile_id: 2222,
      movies: Some(vec![collection_movie()]),
    }
  }

  fn movie() -> Movie {
    Movie {
      id: 1,
      title: "Test".to_owned().into(),
      original_language: language(),
      size_on_disk: 3543348019,
      status: "Downloaded".to_owned(),
      overview: "Blah blah blah".to_owned(),
      path: "/nfs/movies".to_owned(),
      studio: "21st Century Alex".to_owned(),
      genres: genres(),
      year: 2023,
      monitored: true,
      has_file: true,
      runtime: 120,
      tmdb_id: 1234,
      quality_profile_id: 2222,
      minimum_availability: MinimumAvailability::Announced,
      certification: Some("R".to_owned()),
      tags: vec![Number::from(1)],
      ratings: ratings_list(),
      movie_file: Some(movie_file()),
      collection: Some(movie_collection()),
    }
  }

  fn movie_collection() -> MovieCollection {
    MovieCollection {
      title: Some("Test Collection".to_owned()),
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
      age: 1,
      title: HorizontallyScrollableText::from("Test Release"),
      indexer: "kickass torrents".to_owned(),
      indexer_id: 2,
      size: 1234,
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
      tmdb_id: 1234,
      title: HorizontallyScrollableText::from("Test"),
      original_language: language(),
      status: "released".to_owned(),
      overview: "New movie blah blah blah".to_owned(),
      genres: genres(),
      year: 2023,
      runtime: 120,
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
      id: 1,
      movie_id: 1,
      size: 3543348019,
      sizeleft: 1771674009,
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
      id: 1,
      path: "/nfs".to_owned(),
      accessible: true,
      free_space: 219902325555200,
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
      priority: 25,
      download_client_id: 0,
      name: Some("Test Indexer".to_owned()),
      implementation_name: Some("Torznab".to_owned()),
      implementation: Some("Torznab".to_owned()),
      config_contract: Some("TorznabSettings".to_owned()),
      tags: vec![Number::from(1)],
      id: 1,
      fields: Some(vec![
        IndexerField {
          name: Some("baseUrl".to_owned()),
          value: Some(json!("https://test.com")),
        },
        IndexerField {
          name: Some("apiKey".to_owned()),
          value: Some(json!("")),
        },
        IndexerField {
          name: Some("seedCriteria.seedRatio".to_owned()),
          value: Some(json!("1.2")),
        },
      ]),
    }
  }

  fn indexer_settings() -> IndexerSettings {
    IndexerSettings {
      rss_sync_interval: 60,
      allow_hardcoded_subs: true,
      id: 1,
      ..IndexerSettings::default()
    }
  }
}
