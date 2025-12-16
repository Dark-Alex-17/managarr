#[cfg(test)]
mod tests {
  use crate::models::radarr_models::{
    AddMovieBody, AddMovieOptions, Credit, DeleteMovieParams, DownloadRecord, EditMovieParams,
    MinimumAvailability, Movie, MovieHistoryItem, RadarrReleaseDownloadBody,
  };
  use crate::models::servarr_data::radarr::modals::MovieDetailsModal;
  use crate::models::servarr_data::radarr::radarr_data::ActiveRadarrBlock;
  use crate::models::stateful_table::SortOption;
  use crate::network::NetworkResource;
  use crate::network::network_tests::test_utils::{MockServarrApi, test_network};
  use crate::network::radarr_network::RadarrEvent;
  use crate::network::radarr_network::RadarrSerdeable;
  use crate::network::radarr_network::library::get_movie_status;
  use crate::network::radarr_network::radarr_network_test_utils::test_utils::MOVIE_JSON;
  use crate::network::radarr_network::radarr_network_test_utils::test_utils::{
    add_movie_search_result, cast_credit, crew_credit, movie, movie_history_item, release,
  };
  use bimap::BiMap;
  use indoc::formatdoc;
  use mockito::Matcher;
  use pretty_assertions::{assert_eq, assert_str_eq};
  use rstest::rstest;
  use serde_json::{Value, json};
  use std::slice;

  #[tokio::test]
  async fn test_handle_add_movie_event() {
    let body = json!({
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
    });
    let (mock, app, _server) = MockServarrApi::post()
      .with_request_body(body)
      .returns(json!({}))
      .build_for(RadarrEvent::AddMovie(AddMovieBody::default()))
      .await;
    app.lock().await.data.radarr_data.tags_map =
      BiMap::from_iter([(1, "usenet".to_owned()), (2, "testing".to_owned())]);
    let add_movie_body = AddMovieBody {
      tmdb_id: 1234,
      title: "Test".to_owned(),
      root_folder_path: "/nfs2".to_owned(),
      minimum_availability: "announced".to_owned(),
      monitored: true,
      quality_profile_id: 2222,
      tags: vec![1, 2],
      tag_input_string: Some("usenet, testing".into()),
      add_options: AddMovieOptions {
        monitor: "movieOnly".to_owned(),
        search_for_movie: true,
      },
    };

    let mut network = test_network(&app);

    assert!(
      network
        .handle_radarr_event(RadarrEvent::AddMovie(add_movie_body))
        .await
        .is_ok()
    );

    mock.assert_async().await;
  }

  #[tokio::test]
  async fn test_handle_add_movie_event_does_not_overwrite_tags_field_if_tag_input_string_is_none() {
    let body = json!({
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
    });
    let (mock, app, _server) = MockServarrApi::post()
      .with_request_body(body)
      .returns(json!({}))
      .build_for(RadarrEvent::AddMovie(AddMovieBody::default()))
      .await;
    app.lock().await.data.radarr_data.tags_map =
      BiMap::from_iter([(1, "usenet".to_owned()), (2, "testing".to_owned())]);
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
    let mut network = test_network(&app);

    assert!(
      network
        .handle_radarr_event(RadarrEvent::AddMovie(add_movie_body))
        .await
        .is_ok()
    );

    mock.assert_async().await;
  }

  #[tokio::test]
  async fn test_handle_delete_movie_event() {
    let delete_movie_params = DeleteMovieParams {
      id: 1,
      delete_movie_files: true,
      add_list_exclusion: true,
    };
    let (async_server, app_arc, _server) = MockServarrApi::delete()
      .path("/1")
      .query("deleteFiles=true&addImportExclusion=true")
      .build_for(RadarrEvent::DeleteMovie(delete_movie_params.clone()))
      .await;
    let mut network = test_network(&app_arc);

    assert!(
      network
        .handle_radarr_event(RadarrEvent::DeleteMovie(delete_movie_params))
        .await
        .is_ok()
    );

    async_server.assert_async().await;
  }

  #[tokio::test]
  async fn test_handle_download_radarr_release_event() {
    let expected_body = RadarrReleaseDownloadBody {
      guid: "1234".to_owned(),
      indexer_id: 2,
      movie_id: 1,
    };
    let body = json!({
      "guid": "1234",
      "indexerId": 2,
      "movieId": 1
    });
    let (mock, app, _server) = MockServarrApi::post()
      .with_request_body(body)
      .returns(json!({}))
      .build_for(RadarrEvent::DownloadRelease(expected_body.clone()))
      .await;
    let mut network = test_network(&app);

    assert!(
      network
        .handle_radarr_event(RadarrEvent::DownloadRelease(expected_body))
        .await
        .is_ok()
    );

    mock.assert_async().await;
  }

  #[tokio::test]
  async fn test_handle_edit_movie_event() {
    let mut expected_body: Value = serde_json::from_str(MOVIE_JSON).unwrap();
    *expected_body.get_mut("monitored").unwrap() = json!(false);
    *expected_body.get_mut("minimumAvailability").unwrap() = json!("announced");
    *expected_body.get_mut("qualityProfileId").unwrap() = json!(1111);
    *expected_body.get_mut("path").unwrap() = json!("/nfs/Test Path");
    *expected_body.get_mut("tags").unwrap() = json!([1, 2]);
    let edit_movie_params = EditMovieParams {
      movie_id: 1,
      monitored: Some(false),
      minimum_availability: Some(MinimumAvailability::Announced),
      quality_profile_id: Some(1111),
      root_folder_path: Some("/nfs/Test Path".to_owned()),
      tag_input_string: Some("usenet, testing".into()),
      ..EditMovieParams::default()
    };

    let (async_details_server, app_arc, mut server) = MockServarrApi::get()
      .returns(serde_json::from_str(MOVIE_JSON).unwrap())
      .path("/1")
      .build_for(RadarrEvent::GetMovieDetails(1))
      .await;
    let async_edit_server = server
      .mock(
        "PUT",
        format!(
          "/api/v3{}/1",
          RadarrEvent::EditMovie(edit_movie_params.clone()).resource()
        )
        .as_str(),
      )
      .with_status(202)
      .match_header("X-Api-Key", "test1234")
      .match_body(Matcher::Json(expected_body))
      .create_async()
      .await;
    app_arc.lock().await.data.radarr_data.tags_map =
      BiMap::from_iter([(1, "usenet".to_owned()), (2, "testing".to_owned())]);
    let mut network = test_network(&app_arc);

    assert!(
      network
        .handle_radarr_event(RadarrEvent::EditMovie(edit_movie_params))
        .await
        .is_ok()
    );

    async_details_server.assert_async().await;
    async_edit_server.assert_async().await;
  }

  #[tokio::test]
  async fn test_handle_edit_movie_event_does_not_overwrite_tags_vec_if_tag_input_string_is_none() {
    let mut expected_body: Value = serde_json::from_str(MOVIE_JSON).unwrap();
    *expected_body.get_mut("monitored").unwrap() = json!(false);
    *expected_body.get_mut("minimumAvailability").unwrap() = json!("announced");
    *expected_body.get_mut("qualityProfileId").unwrap() = json!(1111);
    *expected_body.get_mut("path").unwrap() = json!("/nfs/Test Path");
    *expected_body.get_mut("tags").unwrap() = json!([1, 2]);
    let edit_movie_params = EditMovieParams {
      movie_id: 1,
      monitored: Some(false),
      minimum_availability: Some(MinimumAvailability::Announced),
      quality_profile_id: Some(1111),
      root_folder_path: Some("/nfs/Test Path".to_owned()),
      tags: Some(vec![1, 2]),
      ..EditMovieParams::default()
    };
    let (async_details_server, app_arc, mut server) = MockServarrApi::get()
      .returns(serde_json::from_str(MOVIE_JSON).unwrap())
      .path("/1")
      .build_for(RadarrEvent::GetMovieDetails(1))
      .await;
    let async_edit_server = server
      .mock(
        "PUT",
        format!(
          "/api/v3{}/1",
          RadarrEvent::EditMovie(edit_movie_params.clone()).resource()
        )
        .as_str(),
      )
      .with_status(202)
      .match_header("X-Api-Key", "test1234")
      .match_body(Matcher::Json(expected_body))
      .create_async()
      .await;
    app_arc.lock().await.data.radarr_data.tags_map =
      BiMap::from_iter([(1, "usenet".to_owned()), (2, "testing".to_owned())]);
    let mut network = test_network(&app_arc);

    assert!(
      network
        .handle_radarr_event(RadarrEvent::EditMovie(edit_movie_params))
        .await
        .is_ok()
    );

    async_details_server.assert_async().await;
    async_edit_server.assert_async().await;
  }

  #[tokio::test]
  async fn test_handle_edit_movie_event_defaults_to_previous_values() {
    let expected_body: Value = serde_json::from_str(MOVIE_JSON).unwrap();
    let (async_details_server, app_arc, mut server) = MockServarrApi::get()
      .returns(serde_json::from_str(MOVIE_JSON).unwrap())
      .path("/1")
      .build_for(RadarrEvent::GetMovieDetails(1))
      .await;
    let edit_movie_params = EditMovieParams {
      movie_id: 1,
      ..EditMovieParams::default()
    };
    let async_edit_server = server
      .mock(
        "PUT",
        format!(
          "/api/v3{}/1",
          RadarrEvent::EditMovie(edit_movie_params.clone()).resource()
        )
        .as_str(),
      )
      .with_status(202)
      .match_header("X-Api-Key", "test1234")
      .match_body(Matcher::Json(expected_body))
      .create_async()
      .await;
    let mut network = test_network(&app_arc);

    assert!(
      network
        .handle_radarr_event(RadarrEvent::EditMovie(edit_movie_params))
        .await
        .is_ok()
    );

    async_details_server.assert_async().await;
    async_edit_server.assert_async().await;
  }

  #[tokio::test]
  async fn test_handle_edit_movie_event_uses_provided_parameters_returns_empty_tags_vec_when_clear_tags_is_true()
   {
    let mut expected_body: Value = serde_json::from_str(MOVIE_JSON).unwrap();
    *expected_body.get_mut("tags").unwrap() = json!([]);
    let (async_details_server, app_arc, mut server) = MockServarrApi::get()
      .returns(serde_json::from_str(MOVIE_JSON).unwrap())
      .path("/1")
      .build_for(RadarrEvent::GetMovieDetails(1))
      .await;
    let edit_movie_params = EditMovieParams {
      movie_id: 1,
      clear_tags: true,
      ..EditMovieParams::default()
    };
    let async_edit_server = server
      .mock(
        "PUT",
        format!(
          "/api/v3{}/1",
          RadarrEvent::EditMovie(edit_movie_params.clone()).resource()
        )
        .as_str(),
      )
      .with_status(202)
      .match_header("X-Api-Key", "test1234")
      .match_body(Matcher::Json(expected_body))
      .create_async()
      .await;
    let mut network = test_network(&app_arc);

    assert!(
      network
        .handle_radarr_event(RadarrEvent::EditMovie(edit_movie_params))
        .await
        .is_ok()
    );

    async_details_server.assert_async().await;
    async_edit_server.assert_async().await;
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
    let (async_server, app_arc, _server) = MockServarrApi::get()
      .returns(credits_json)
      .query("movieId=1")
      .build_for(RadarrEvent::GetMovieCredits(1))
      .await;
    app_arc.lock().await.data.radarr_data.movie_details_modal = Some(MovieDetailsModal::default());
    let mut network = test_network(&app_arc);

    let RadarrSerdeable::Credits(credits) = network
      .handle_radarr_event(RadarrEvent::GetMovieCredits(1))
      .await
      .unwrap()
    else {
      panic!("Expected Credits")
    };
    let app = app_arc.lock().await;
    let movie_details_modal = app.data.radarr_data.movie_details_modal.as_ref().unwrap();

    async_server.assert_async().await;
    assert_eq!(movie_details_modal.movie_cast.items, vec![cast_credit()]);
    assert_eq!(movie_details_modal.movie_crew.items, vec![crew_credit()]);
    assert_eq!(credits, response);
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
    let (async_server, app_arc, _server) = MockServarrApi::get()
      .returns(credits_json)
      .query("movieId=1")
      .build_for(RadarrEvent::GetMovieCredits(1))
      .await;
    let mut network = test_network(&app_arc);

    assert!(
      network
        .handle_radarr_event(RadarrEvent::GetMovieCredits(1))
        .await
        .is_ok()
    );

    let app = app_arc.lock().await;
    let movie_details_modal = app.data.radarr_data.movie_details_modal.as_ref().unwrap();

    async_server.assert_async().await;
    assert_eq!(movie_details_modal.movie_cast.items, vec![cast_credit()]);
    assert_eq!(movie_details_modal.movie_crew.items, vec![crew_credit()]);
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
    let (mock, app, _server) = MockServarrApi::get()
      .returns(json!([movie_1, movie_2]))
      .build_for(RadarrEvent::GetMovies)
      .await;
    app.lock().await.data.radarr_data.movies.sort_asc = true;
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
      app
        .lock()
        .await
        .data
        .radarr_data
        .movies
        .sorting(vec![title_sort_option]);
    }
    let mut network = test_network(&app);

    let result = network.handle_radarr_event(RadarrEvent::GetMovies).await;

    mock.assert_async().await;

    let RadarrSerdeable::Movies(movies) = result.unwrap() else {
      panic!("Expected Movies variant")
    };
    mock.assert_async().await;
    assert_eq!(
      app.lock().await.data.radarr_data.movies.items,
      expected_sorted_movies
    );
    assert!(app.lock().await.data.radarr_data.movies.sort_asc);
    assert_eq!(movies, expected_movies);
  }

  #[tokio::test]
  async fn test_handle_get_movies_event_no_op_while_user_is_selecting_sort_options() {
    let mut movie_1: Value = serde_json::from_str(MOVIE_JSON).unwrap();
    let mut movie_2: Value = serde_json::from_str(MOVIE_JSON).unwrap();
    *movie_1.get_mut("id").unwrap() = json!(1);
    *movie_1.get_mut("title").unwrap() = json!("z test");
    *movie_2.get_mut("id").unwrap() = json!(2);
    *movie_2.get_mut("title").unwrap() = json!("A test");
    let (mock, app, _server) = MockServarrApi::get()
      .returns(json!([movie_1, movie_2]))
      .build_for(RadarrEvent::GetMovies)
      .await;
    app
      .lock()
      .await
      .push_navigation_stack(ActiveRadarrBlock::MoviesSortPrompt.into());
    app.lock().await.data.radarr_data.movies.sort_asc = true;
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
    app
      .lock()
      .await
      .data
      .radarr_data
      .movies
      .sorting(vec![title_sort_option]);
    let mut network = test_network(&app);

    assert!(
      network
        .handle_radarr_event(RadarrEvent::GetMovies)
        .await
        .is_ok()
    );

    mock.assert_async().await;
    assert!(app.lock().await.data.radarr_data.movies.items.is_empty());
    assert!(app.lock().await.data.radarr_data.movies.sort_asc);
  }

  #[tokio::test]
  async fn test_handle_get_movie_details_event() {
    let response: Movie = serde_json::from_str(MOVIE_JSON).unwrap();
    let (async_server, app_arc, _server) = MockServarrApi::get()
      .returns(serde_json::from_str(MOVIE_JSON).unwrap())
      .path("/1")
      .build_for(RadarrEvent::GetMovieDetails(1))
      .await;
    app_arc.lock().await.data.radarr_data.quality_profile_map =
      BiMap::from_iter([(2222, "HD - 1080p".to_owned())]);
    let mut network = test_network(&app_arc);

    let RadarrSerdeable::Movie(movie) = network
      .handle_radarr_event(RadarrEvent::GetMovieDetails(1))
      .await
      .unwrap()
    else {
      panic!("Expected Movie")
    };
    async_server.assert_async().await;
    assert!(
      app_arc
        .lock()
        .await
        .data
        .radarr_data
        .movie_details_modal
        .is_some()
    );
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
        Rotten Tomatoes: 99%
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
        "id": 1,
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
    let (async_server, app_arc, _server) = MockServarrApi::get()
      .returns(movie_json_with_missing_fields)
      .path("/1")
      .build_for(RadarrEvent::GetMovieDetails(1))
      .await;
    app_arc.lock().await.data.radarr_data.quality_profile_map =
      BiMap::from_iter([(2222, "HD - 1080p".to_owned())]);
    let mut network = test_network(&app_arc);

    assert!(
      network
        .handle_radarr_event(RadarrEvent::GetMovieDetails(1))
        .await
        .is_ok()
    );

    async_server.assert_async().await;
    assert!(
      app_arc
        .lock()
        .await
        .data
        .radarr_data
        .movie_details_modal
        .is_some()
    );

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
      "languages": [ { "id": 1, "name": "English" } ],
      "date": "2022-12-30T07:37:56Z",
      "eventType": "grabbed"
    }]);
    let response: Vec<MovieHistoryItem> =
      serde_json::from_value(movie_history_item_json.clone()).unwrap();
    let (async_server, app_arc, _server) = MockServarrApi::get()
      .returns(movie_history_item_json)
      .query("movieId=1")
      .build_for(RadarrEvent::GetMovieHistory(1))
      .await;
    app_arc.lock().await.data.radarr_data.movie_details_modal = Some(MovieDetailsModal::default());
    let mut network = test_network(&app_arc);

    let RadarrSerdeable::MovieHistoryItems(history) = network
      .handle_radarr_event(RadarrEvent::GetMovieHistory(1))
      .await
      .unwrap()
    else {
      panic!("Expected MovieHistoryItems")
    };
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

  #[tokio::test]
  async fn test_handle_get_movie_history_event_empty_movie_details_modal() {
    let movie_history_item_json = json!([{
      "sourceTitle": "Test",
      "quality": { "quality": { "name": "HD - 1080p" }},
      "languages": [ { "id": 1, "name": "English" } ],
      "date": "2022-12-30T07:37:56Z",
      "eventType": "grabbed"
    }]);
    let (async_server, app_arc, _server) = MockServarrApi::get()
      .returns(movie_history_item_json)
      .query("movieId=1")
      .build_for(RadarrEvent::GetMovieHistory(1))
      .await;
    let mut network = test_network(&app_arc);

    assert!(
      network
        .handle_radarr_event(RadarrEvent::GetMovieHistory(1))
        .await
        .is_ok()
    );

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
  async fn test_handle_get_movie_releases_event() {
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
      "languages": [ { "id": 1, "name": "English" } ],
      "quality": { "quality": { "name": "HD - 1080p" }}
    }]);
    let (async_server, app_arc, _server) = MockServarrApi::get()
      .returns(release_json)
      .query("movieId=1")
      .build_for(RadarrEvent::GetReleases(1))
      .await;
    app_arc.lock().await.data.radarr_data.movie_details_modal = Some(MovieDetailsModal::default());
    let mut network = test_network(&app_arc);

    let RadarrSerdeable::Releases(releases_vec) = network
      .handle_radarr_event(RadarrEvent::GetReleases(1))
      .await
      .unwrap()
    else {
      panic!("Expected Releases")
    };
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

  #[tokio::test]
  async fn test_handle_get_movie_releases_event_empty_movie_details_modal() {
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
      "languages": [ { "id": 1, "name": "English" } ],
      "quality": { "quality": { "name": "HD - 1080p" }}
    }]);
    let (async_server, app_arc, _server) = MockServarrApi::get()
      .returns(release_json)
      .query("movieId=1")
      .build_for(RadarrEvent::GetReleases(1))
      .await;
    let mut network = test_network(&app_arc);

    assert!(
      network
        .handle_radarr_event(RadarrEvent::GetReleases(1))
        .await
        .is_ok()
    );

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
      "originalLanguage": { "id": 1, "name": "English" },
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
          "value": 99
        }
      }
    }]);
    let (async_server, app_arc, _server) = MockServarrApi::get()
      .returns(add_movie_search_result_json)
      .query("term=test%20term")
      .build_for(RadarrEvent::SearchNewMovie("test term".into()))
      .await;
    let mut network = test_network(&app_arc);

    let RadarrSerdeable::AddMovieSearchResults(add_movie_search_results) = network
      .handle_radarr_event(RadarrEvent::SearchNewMovie("test term".into()))
      .await
      .unwrap()
    else {
      panic!("Expected AddMovieSearchResults")
    };
    async_server.assert_async().await;
    assert!(
      app_arc
        .lock()
        .await
        .data
        .radarr_data
        .add_searched_movies
        .is_some()
    );
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

  #[tokio::test]
  async fn test_handle_search_new_movie_event_no_results() {
    let (async_server, app_arc, _server) = MockServarrApi::get()
      .returns(json!([]))
      .query("term=test%20term")
      .build_for(RadarrEvent::SearchNewMovie("test term".into()))
      .await;
    let mut network = test_network(&app_arc);

    assert!(
      network
        .handle_radarr_event(RadarrEvent::SearchNewMovie("test term".into()))
        .await
        .is_ok()
    );

    async_server.assert_async().await;
    assert!(
      app_arc
        .lock()
        .await
        .data
        .radarr_data
        .add_searched_movies
        .is_none()
    );
    assert_eq!(
      app_arc.lock().await.get_current_route(),
      ActiveRadarrBlock::AddMovieEmptySearchResults.into()
    );
  }

  #[tokio::test]
  async fn test_handle_toggle_movie_monitoring_event() {
    let mut expected_body: Value = serde_json::from_str(MOVIE_JSON).unwrap();
    *expected_body.get_mut("monitored").unwrap() = json!(false);

    let (async_details_server, app_arc, mut server) = MockServarrApi::get()
      .returns(serde_json::from_str(MOVIE_JSON).unwrap())
      .path("/1")
      .build_for(RadarrEvent::GetMovieDetails(1))
      .await;
    let async_toggle_server = server
      .mock(
        "PUT",
        format!(
          "/api/v3{}/1",
          RadarrEvent::ToggleMovieMonitoring(1).resource()
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
      app.data.radarr_data.movies.set_items(vec![movie()]);
    }
    let mut network = test_network(&app_arc);

    assert!(
      network
        .handle_radarr_event(RadarrEvent::ToggleMovieMonitoring(1))
        .await
        .is_ok()
    );

    async_details_server.assert_async().await;
    async_toggle_server.assert_async().await;
  }

  #[tokio::test]
  async fn test_handle_trigger_automatic_movie_search_event() {
    let body = json!({
      "name": "MoviesSearch",
      "movieIds": [ 1 ]
    });
    let (mock, app, _server) = MockServarrApi::post()
      .with_request_body(body)
      .returns(json!({}))
      .build_for(RadarrEvent::TriggerAutomaticSearch(1))
      .await;
    let mut network = test_network(&app);

    assert!(
      network
        .handle_radarr_event(RadarrEvent::TriggerAutomaticSearch(1))
        .await
        .is_ok()
    );

    mock.assert_async().await;
  }

  #[tokio::test]
  async fn test_handle_update_all_movies_event() {
    let body = json!({
      "name": "RefreshMovie",
      "movieIds": []
    });
    let (mock, app, _server) = MockServarrApi::post()
      .with_request_body(body)
      .returns(json!({}))
      .build_for(RadarrEvent::UpdateAllMovies)
      .await;
    let mut network = test_network(&app);

    assert!(
      network
        .handle_radarr_event(RadarrEvent::UpdateAllMovies)
        .await
        .is_ok()
    );

    mock.assert_async().await;
  }

  #[tokio::test]
  async fn test_handle_update_and_scan_movie_event() {
    let body = json!({
      "name": "RefreshMovie",
      "movieIds": [ 1 ]
    });
    let (mock, app, _server) = MockServarrApi::post()
      .with_request_body(body)
      .returns(json!({}))
      .build_for(RadarrEvent::UpdateAndScan(1))
      .await;
    let mut network = test_network(&app);

    assert!(
      network
        .handle_radarr_event(RadarrEvent::UpdateAndScan(1))
        .await
        .is_ok()
    );

    mock.assert_async().await;
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
      get_movie_status(false, slice::from_ref(&download_record), 0),
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
}
