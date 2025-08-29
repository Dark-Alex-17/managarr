use crate::models::radarr_models::{
  AddMovieBody, AddMovieSearchResult, Credit, CreditType, DeleteMovieParams, DownloadRecord,
  EditMovieParams, Movie, MovieCommandBody, MovieHistoryItem, RadarrRelease,
  RadarrReleaseDownloadBody,
};
use crate::models::servarr_data::radarr::modals::MovieDetailsModal;
use crate::models::servarr_data::radarr::radarr_data::ActiveRadarrBlock;
use crate::models::stateful_table::StatefulTable;
use crate::models::{Route, ScrollableText};
use crate::network::radarr_network::RadarrEvent;
use crate::network::{Network, RequestMethod};
use crate::utils::{convert_runtime, convert_to_gb};
use anyhow::Result;
use indoc::formatdoc;
use log::{debug, info, warn};
use serde_json::{json, Value};
use urlencoding::encode;

#[cfg(test)]
#[path = "radarr_library_network_tests.rs"]
mod radarr_library_network_tests;

impl Network<'_, '_> {
  pub(in crate::network::radarr_network) async fn add_movie(
    &mut self,
    mut add_movie_body: AddMovieBody,
  ) -> Result<Value> {
    info!("Adding new movie to Radarr");
    let event = RadarrEvent::AddMovie(AddMovieBody::default());
    if let Some(tag_input_str) = add_movie_body.tag_input_string.as_ref() {
      let tag_ids_vec = self.extract_and_add_radarr_tag_ids_vec(tag_input_str).await;
      add_movie_body.tags = tag_ids_vec;
    }

    debug!("Add movie body: {add_movie_body:?}");

    let request_props = self
      .request_props_from(event, RequestMethod::Post, Some(add_movie_body), None, None)
      .await;

    self
      .handle_request::<AddMovieBody, Value>(request_props, |_, _| ())
      .await
  }

  pub(in crate::network::radarr_network) async fn delete_movie(
    &mut self,
    delete_movie_params: DeleteMovieParams,
  ) -> Result<()> {
    let event = RadarrEvent::DeleteMovie(DeleteMovieParams::default());
    let DeleteMovieParams {
      id,
      delete_movie_files,
      add_list_exclusion,
    } = delete_movie_params;
    info!("Deleting Radarr movie with ID: {id} with deleteFiles={delete_movie_files} and addImportExclusion={add_list_exclusion}");

    let request_props = self
      .request_props_from(
        event,
        RequestMethod::Delete,
        None::<()>,
        Some(format!("/{id}")),
        Some(format!(
          "deleteFiles={delete_movie_files}&addImportExclusion={add_list_exclusion}"
        )),
      )
      .await;

    self
      .handle_request::<(), ()>(request_props, |_, _| ())
      .await
  }

  pub(in crate::network::radarr_network) async fn download_radarr_release(
    &mut self,
    params: RadarrReleaseDownloadBody,
  ) -> Result<Value> {
    let event = RadarrEvent::DownloadRelease(RadarrReleaseDownloadBody::default());
    info!("Downloading Radarr release with params: {params:?}");

    let request_props = self
      .request_props_from(event, RequestMethod::Post, Some(params), None, None)
      .await;

    self
      .handle_request::<RadarrReleaseDownloadBody, Value>(request_props, |_, _| ())
      .await
  }

  pub(in crate::network::radarr_network) async fn edit_movie(
    &mut self,
    mut edit_movie_params: EditMovieParams,
  ) -> Result<()> {
    info!("Editing Radarr movie");
    let movie_id = edit_movie_params.movie_id;
    let detail_event = RadarrEvent::GetMovieDetails(movie_id);
    let event = RadarrEvent::EditMovie(EditMovieParams::default());
    if let Some(tag_input_str) = edit_movie_params.tag_input_string.as_ref() {
      let tag_ids_vec = self.extract_and_add_radarr_tag_ids_vec(tag_input_str).await;
      edit_movie_params.tags = Some(tag_ids_vec);
    }

    info!("Fetching movie details for movie with ID: {movie_id}");

    let request_props = self
      .request_props_from(
        detail_event,
        RequestMethod::Get,
        None::<()>,
        Some(format!("/{movie_id}")),
        None,
      )
      .await;

    let mut response = String::new();

    self
      .handle_request::<(), Value>(request_props, |detailed_movie_body, _| {
        response = detailed_movie_body.to_string()
      })
      .await?;

    info!("Constructing edit movie body");

    let mut detailed_movie_body: Value = serde_json::from_str(&response)?;
    let (monitored, minimum_availability, quality_profile_id, root_folder_path, tags) = {
      let monitored = edit_movie_params.monitored.unwrap_or(
        detailed_movie_body["monitored"]
          .as_bool()
          .expect("Unable to deserialize 'monitored'"),
      );
      let minimum_availability = edit_movie_params
        .minimum_availability
        .unwrap_or_else(|| {
          serde_json::from_value(detailed_movie_body["minimumAvailability"].clone())
            .expect("Unable to deserialize 'minimumAvailability'")
        })
        .to_string();
      let quality_profile_id = edit_movie_params.quality_profile_id.unwrap_or_else(|| {
        detailed_movie_body["qualityProfileId"]
          .as_i64()
          .expect("Unable to deserialize 'qualityProfileId'")
      });
      let root_folder_path = edit_movie_params.root_folder_path.unwrap_or_else(|| {
        detailed_movie_body["path"]
          .as_str()
          .expect("Unable to deserialize 'path'")
          .to_owned()
      });
      let tags = if edit_movie_params.clear_tags {
        vec![]
      } else {
        edit_movie_params.tags.unwrap_or(
          detailed_movie_body["tags"]
            .as_array()
            .expect("Unable to deserialize 'tags'")
            .iter()
            .map(|item| item.as_i64().expect("Unable to deserialize tag ID"))
            .collect(),
        )
      };

      (
        monitored,
        minimum_availability,
        quality_profile_id,
        root_folder_path,
        tags,
      )
    };

    *detailed_movie_body.get_mut("monitored").unwrap() = json!(monitored);
    *detailed_movie_body.get_mut("minimumAvailability").unwrap() = json!(minimum_availability);
    *detailed_movie_body.get_mut("qualityProfileId").unwrap() = json!(quality_profile_id);
    *detailed_movie_body.get_mut("path").unwrap() = json!(root_folder_path);
    *detailed_movie_body.get_mut("tags").unwrap() = json!(tags);

    debug!("Edit movie body: {detailed_movie_body:?}");

    let request_props = self
      .request_props_from(
        event,
        RequestMethod::Put,
        Some(detailed_movie_body),
        Some(format!("/{movie_id}")),
        None,
      )
      .await;

    self
      .handle_request::<Value, ()>(request_props, |_, _| ())
      .await
  }

  pub(in crate::network::radarr_network) async fn get_credits(
    &mut self,
    movie_id: i64,
  ) -> Result<Vec<Credit>> {
    info!("Fetching Radarr movie credits");
    let event = RadarrEvent::GetMovieCredits(movie_id);

    let request_props = self
      .request_props_from(
        event,
        RequestMethod::Get,
        None::<()>,
        None,
        Some(format!("movieId={movie_id}")),
      )
      .await;

    self
      .handle_request::<(), Vec<Credit>>(request_props, |credit_vec, mut app| {
        let cast_vec: Vec<Credit> = credit_vec
          .iter()
          .filter(|&credit| credit.credit_type == CreditType::Cast)
          .cloned()
          .collect();
        let crew_vec: Vec<Credit> = credit_vec
          .iter()
          .filter(|&credit| credit.credit_type == CreditType::Crew)
          .cloned()
          .collect();

        if app.data.radarr_data.movie_details_modal.is_none() {
          app.data.radarr_data.movie_details_modal = Some(MovieDetailsModal::default());
        }

        app
          .data
          .radarr_data
          .movie_details_modal
          .as_mut()
          .unwrap()
          .movie_cast
          .set_items(cast_vec);
        app
          .data
          .radarr_data
          .movie_details_modal
          .as_mut()
          .unwrap()
          .movie_crew
          .set_items(crew_vec);
      })
      .await
  }

  pub(in crate::network::radarr_network) async fn get_movies(&mut self) -> Result<Vec<Movie>> {
    info!("Fetching Radarr library");
    let event = RadarrEvent::GetMovies;

    let request_props = self
      .request_props_from(event, RequestMethod::Get, None::<()>, None, None)
      .await;

    self
      .handle_request::<(), Vec<Movie>>(request_props, |mut movie_vec, mut app| {
        if !matches!(
          app.get_current_route(),
          Route::Radarr(ActiveRadarrBlock::MoviesSortPrompt, _)
        ) {
          movie_vec.sort_by(|a, b| a.id.cmp(&b.id));
          app.data.radarr_data.movies.set_items(movie_vec);
          app.data.radarr_data.movies.apply_sorting_toggle(false);
        }
      })
      .await
  }

  pub(in crate::network::radarr_network) async fn get_movie_details(
    &mut self,
    movie_id: i64,
  ) -> Result<Movie> {
    info!("Fetching Radarr movie details");
    let event = RadarrEvent::GetMovieDetails(movie_id);

    info!("Fetching movie details for movie with ID: {movie_id}");

    let request_props = self
      .request_props_from(
        event,
        RequestMethod::Get,
        None::<()>,
        Some(format!("/{movie_id}")),
        None,
      )
      .await;

    self
      .handle_request::<(), Movie>(request_props, |movie_response, mut app| {
        let Movie {
          id,
          title,
          year,
          overview,
          path,
          studio,
          has_file,
          quality_profile_id,
          size_on_disk,
          genres,
          runtime,
          certification,
          ratings,
          movie_file,
          collection,
          ..
        } = movie_response;
        let (hours, minutes) = convert_runtime(runtime);
        let size = convert_to_gb(size_on_disk);
        let studio = studio.clone().unwrap_or_default();
        let quality_profile = app
          .data
          .radarr_data
          .quality_profile_map
          .get_by_left(&quality_profile_id)
          .unwrap_or(&"".to_owned())
          .to_owned();
        let imdb_rating = if let Some(rating) = ratings.imdb {
          if let Some(value) = rating.value.as_f64() {
            format!("{value:.1}")
          } else {
            String::new()
          }
        } else {
          String::new()
        };

        let tmdb_rating = if let Some(rating) = ratings.tmdb {
          if let Some(value) = rating.value.as_f64() {
            format!("{}%", (value * 10f64).ceil())
          } else {
            String::new()
          }
        } else {
          String::new()
        };

        let rotten_tomatoes_rating = if let Some(rating) = ratings.rotten_tomatoes {
          if let Some(value) = rating.value.as_u64() {
            format!("{value}%")
          } else {
            String::new()
          }
        } else {
          String::new()
        };

        let status = get_movie_status(has_file, &app.data.radarr_data.downloads.items, id);
        let collection = collection.unwrap_or_default();

        let mut movie_details_modal = MovieDetailsModal {
          movie_details: ScrollableText::with_string(formatdoc!(
            "Title: {title}
            Year: {year}
            Runtime: {hours}h {minutes}m
            Rating: {}
            Collection: {}
            Status: {status}
            Description: {overview}
            TMDB: {tmdb_rating}
            IMDB: {imdb_rating}
            Rotten Tomatoes: {rotten_tomatoes_rating}
            Quality Profile: {quality_profile}
            Size: {size:.2} GB
            Path: {path}
            Studio: {studio}
            Genres: {}",
            certification.unwrap_or_default(),
            collection
              .title
              .as_ref()
              .unwrap_or(&String::new())
              .to_owned(),
            genres.join(", ")
          )),
          ..MovieDetailsModal::default()
        };

        if let Some(file) = movie_file {
          movie_details_modal.file_details = formatdoc!(
            "Relative Path: {}
              Absolute Path: {}
              Size: {size:.2} GB
              Date Added: {}",
            file.relative_path,
            file.path,
            file.date_added
          );

          if let Some(media_info) = file.media_info {
            movie_details_modal.audio_details = formatdoc!(
              "Bitrate: {}
              Channels: {:.1}
              Codec: {}
              Languages: {}
              Stream Count: {}",
              media_info.audio_bitrate,
              media_info.audio_channels.as_f64().unwrap(),
              media_info.audio_codec.unwrap_or_default(),
              media_info.audio_languages.unwrap_or_default(),
              media_info.audio_stream_count
            );

            movie_details_modal.video_details = formatdoc!(
              "Bit Depth: {}
              Bitrate: {}
              Codec: {}
              FPS: {}
              Resolution: {}
              Scan Type: {}
              Runtime: {}",
              media_info.video_bit_depth,
              media_info.video_bitrate,
              media_info.video_codec.unwrap_or_default(),
              media_info.video_fps.as_f64().unwrap(),
              media_info.resolution,
              media_info.scan_type,
              media_info.run_time
            );
          }
        }

        app.data.radarr_data.movie_details_modal = Some(movie_details_modal);
      })
      .await
  }

  pub(in crate::network::radarr_network) async fn get_movie_history(
    &mut self,
    movie_id: i64,
  ) -> Result<Vec<MovieHistoryItem>> {
    info!("Fetching Radarr movie history");
    let event = RadarrEvent::GetMovieHistory(movie_id);

    let request_props = self
      .request_props_from(
        event,
        RequestMethod::Get,
        None::<()>,
        None,
        Some(format!("movieId={movie_id}")),
      )
      .await;

    self
      .handle_request::<(), Vec<MovieHistoryItem>>(request_props, |movie_history_vec, mut app| {
        let mut reversed_movie_history_vec = movie_history_vec.to_vec();
        reversed_movie_history_vec.reverse();

        if app.data.radarr_data.movie_details_modal.is_none() {
          app.data.radarr_data.movie_details_modal = Some(MovieDetailsModal::default())
        }

        app
          .data
          .radarr_data
          .movie_details_modal
          .as_mut()
          .unwrap()
          .movie_history
          .set_items(reversed_movie_history_vec)
      })
      .await
  }

  pub(in crate::network::radarr_network) async fn get_movie_releases(
    &mut self,
    movie_id: i64,
  ) -> Result<Vec<RadarrRelease>> {
    info!("Fetching releases for movie with ID: {movie_id}");
    let event = RadarrEvent::GetReleases(movie_id);

    let request_props = self
      .request_props_from(
        event,
        RequestMethod::Get,
        None::<()>,
        None,
        Some(format!("movieId={movie_id}")),
      )
      .await;

    self
      .handle_request::<(), Vec<RadarrRelease>>(request_props, |release_vec, mut app| {
        if app.data.radarr_data.movie_details_modal.is_none() {
          app.data.radarr_data.movie_details_modal = Some(MovieDetailsModal::default());
        }

        app
          .data
          .radarr_data
          .movie_details_modal
          .as_mut()
          .unwrap()
          .movie_releases
          .set_items(release_vec);
      })
      .await
  }

  pub(in crate::network::radarr_network) async fn search_movie(
    &mut self,
    query: String,
  ) -> Result<Vec<AddMovieSearchResult>> {
    info!("Searching for specific Radarr movie");
    let event = RadarrEvent::SearchNewMovie(String::new());

    let request_props = self
      .request_props_from(
        event,
        RequestMethod::Get,
        None::<()>,
        None,
        Some(format!("term={}", encode(&query))),
      )
      .await;

    self
      .handle_request::<(), Vec<AddMovieSearchResult>>(request_props, |movie_vec, mut app| {
        if movie_vec.is_empty() {
          app.pop_and_push_navigation_stack(ActiveRadarrBlock::AddMovieEmptySearchResults.into());
        } else if let Some(add_searched_movies) = app.data.radarr_data.add_searched_movies.as_mut()
        {
          add_searched_movies.set_items(movie_vec);
        } else {
          let mut add_searched_movies = StatefulTable::default();
          add_searched_movies.set_items(movie_vec);
          app.data.radarr_data.add_searched_movies = Some(add_searched_movies);
        }
      })
      .await
  }

  pub(in crate::network) async fn toggle_movie_monitoring(&mut self, movie_id: i64) -> Result<()> {
    let event = RadarrEvent::ToggleMovieMonitoring(movie_id);

    let detail_event = RadarrEvent::GetMovieDetails(movie_id);
    info!("Toggling movie monitoring for movie with ID: {movie_id}");
    info!("Fetching movie details for movie with ID: {movie_id}");

    let request_props = self
      .request_props_from(
        detail_event,
        RequestMethod::Get,
        None::<()>,
        Some(format!("/{movie_id}")),
        None,
      )
      .await;

    let mut response = String::new();

    self
      .handle_request::<(), Value>(request_props, |detailed_movie_body, _| {
        response = detailed_movie_body.to_string()
      })
      .await?;

    info!("Constructing toggle movie monitoring body");

    match serde_json::from_str::<Value>(&response) {
      Ok(mut detailed_movie_body) => {
        let monitored = detailed_movie_body
          .get("monitored")
          .unwrap()
          .as_bool()
          .unwrap();

        *detailed_movie_body.get_mut("monitored").unwrap() = json!(!monitored);

        debug!("Toggle movie monitoring body: {detailed_movie_body:?}");

        let request_props = self
          .request_props_from(
            event,
            RequestMethod::Put,
            Some(detailed_movie_body),
            Some(format!("/{movie_id}")),
            None,
          )
          .await;

        self
          .handle_request::<Value, ()>(request_props, |_, _| ())
          .await
      }
      Err(_) => {
        warn!("Request for detailed movie body was interrupted");
        Ok(())
      }
    }
  }

  pub(in crate::network) async fn trigger_automatic_movie_search(
    &mut self,
    movie_id: i64,
  ) -> Result<Value> {
    let event = RadarrEvent::TriggerAutomaticSearch(movie_id);
    info!("Searching indexers for movie with ID: {movie_id}");
    let body = MovieCommandBody {
      name: "MoviesSearch".to_owned(),
      movie_ids: vec![movie_id],
    };

    let request_props = self
      .request_props_from(event, RequestMethod::Post, Some(body), None, None)
      .await;

    self
      .handle_request::<MovieCommandBody, Value>(request_props, |_, _| ())
      .await
  }

  pub(in crate::network) async fn update_all_movies(&mut self) -> Result<Value> {
    info!("Updating all movies");
    let event = RadarrEvent::UpdateAllMovies;
    let body = MovieCommandBody {
      name: "RefreshMovie".to_owned(),
      movie_ids: Vec::new(),
    };

    let request_props = self
      .request_props_from(event, RequestMethod::Post, Some(body), None, None)
      .await;

    self
      .handle_request::<MovieCommandBody, Value>(request_props, |_, _| ())
      .await
  }

  pub(in crate::network) async fn update_and_scan_movie(&mut self, movie_id: i64) -> Result<Value> {
    let event = RadarrEvent::UpdateAndScan(movie_id);
    info!("Updating and scanning movie with ID: {movie_id}");
    let body = MovieCommandBody {
      name: "RefreshMovie".to_owned(),
      movie_ids: vec![movie_id],
    };

    let request_props = self
      .request_props_from(event, RequestMethod::Post, Some(body), None, None)
      .await;

    self
      .handle_request::<MovieCommandBody, Value>(request_props, |_, _| ())
      .await
  }
}

pub(in crate::network::radarr_network::library) fn get_movie_status(
  has_file: bool,
  downloads_vec: &[DownloadRecord],
  movie_id: i64,
) -> String {
  if !has_file {
    if let Some(download) = downloads_vec
      .iter()
      .find(|&download| download.movie_id == movie_id)
    {
      if download.status == "downloading" {
        return "Downloading".to_owned();
      }

      if download.status == "completed" {
        return "Awaiting Import".to_owned();
      }
    }

    return "Missing".to_owned();
  }

  "Downloaded".to_owned()
}
