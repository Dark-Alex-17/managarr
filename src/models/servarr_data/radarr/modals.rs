use crate::models::radarr_models::{
  Collection, Credit, MinimumAvailability, Monitor, Movie, MovieHistoryItem, Release, ReleaseField,
  RootFolder,
};
use crate::models::servarr_data::radarr::radarr_data::RadarrData;
use crate::models::{HorizontallyScrollableText, ScrollableText, StatefulList, StatefulTable};
use strum::IntoEnumIterator;

#[cfg(test)]
#[path = "modals_tests.rs"]
mod modals_tests;

#[derive(Default)]
pub struct MovieDetailsModal {
  pub movie_details: ScrollableText,
  pub file_details: String,
  pub audio_details: String,
  pub video_details: String,
  pub movie_history: StatefulTable<MovieHistoryItem>,
  pub movie_cast: StatefulTable<Credit>,
  pub movie_crew: StatefulTable<Credit>,
  pub movie_releases: StatefulTable<Release>,
  pub movie_releases_sort: StatefulList<ReleaseField>,
  pub sort_ascending: Option<bool>,
}

#[derive(Default)]
pub struct EditMovieModal {
  pub minimum_availability_list: StatefulList<MinimumAvailability>,
  pub quality_profile_list: StatefulList<String>,
  pub monitored: Option<bool>,
  pub path: HorizontallyScrollableText,
  pub tags: HorizontallyScrollableText,
}

impl From<&RadarrData<'_>> for EditMovieModal {
  fn from(radarr_data: &RadarrData<'_>) -> EditMovieModal {
    let mut edit_movie_modal = EditMovieModal::default();
    let Movie {
      path,
      tags,
      monitored,
      minimum_availability,
      quality_profile_id,
      ..
    } = if let Some(filtered_movies) = radarr_data.filtered_movies.as_ref() {
      filtered_movies.current_selection()
    } else {
      radarr_data.movies.current_selection()
    };

    edit_movie_modal
      .minimum_availability_list
      .set_items(Vec::from_iter(MinimumAvailability::iter()));
    edit_movie_modal.path = path.clone().into();
    edit_movie_modal.tags = tags
      .iter()
      .map(|tag_id| {
        radarr_data
          .tags_map
          .get_by_left(&tag_id.as_i64().unwrap())
          .unwrap()
          .clone()
      })
      .collect::<Vec<String>>()
      .join(", ")
      .into();

    edit_movie_modal.monitored = Some(*monitored);

    let minimum_availability_index = edit_movie_modal
      .minimum_availability_list
      .items
      .iter()
      .position(|ma| ma == minimum_availability);
    edit_movie_modal
      .minimum_availability_list
      .state
      .select(minimum_availability_index);

    let mut quality_profile_names: Vec<String> = radarr_data
      .quality_profile_map
      .right_values()
      .cloned()
      .collect();
    quality_profile_names.sort();
    edit_movie_modal
      .quality_profile_list
      .set_items(quality_profile_names);
    let quality_profile_name = radarr_data
      .quality_profile_map
      .get_by_left(quality_profile_id)
      .unwrap();
    let quality_profile_index = edit_movie_modal
      .quality_profile_list
      .items
      .iter()
      .position(|profile| profile == quality_profile_name);
    edit_movie_modal
      .quality_profile_list
      .state
      .select(quality_profile_index);

    edit_movie_modal
  }
}

#[derive(Default)]
pub struct AddMovieModal {
  pub root_folder_list: StatefulList<RootFolder>,
  pub monitor_list: StatefulList<Monitor>,
  pub minimum_availability_list: StatefulList<MinimumAvailability>,
  pub quality_profile_list: StatefulList<String>,
  pub tags: HorizontallyScrollableText,
}

impl From<&RadarrData<'_>> for AddMovieModal {
  fn from(radarr_data: &RadarrData<'_>) -> AddMovieModal {
    let mut add_movie_modal = AddMovieModal::default();
    add_movie_modal
      .monitor_list
      .set_items(Vec::from_iter(Monitor::iter()));
    add_movie_modal
      .minimum_availability_list
      .set_items(Vec::from_iter(MinimumAvailability::iter()));
    let mut quality_profile_names: Vec<String> = radarr_data
      .quality_profile_map
      .right_values()
      .cloned()
      .collect();
    quality_profile_names.sort();
    add_movie_modal
      .quality_profile_list
      .set_items(quality_profile_names);
    add_movie_modal
      .root_folder_list
      .set_items(radarr_data.root_folders.items.to_vec());

    add_movie_modal
  }
}

#[derive(Default)]
pub struct EditCollectionModal {
  pub monitored: Option<bool>,
  pub minimum_availability_list: StatefulList<MinimumAvailability>,
  pub quality_profile_list: StatefulList<String>,
  pub path: HorizontallyScrollableText,
  pub search_on_add: Option<bool>,
}

impl From<&RadarrData<'_>> for EditCollectionModal {
  fn from(radarr_data: &RadarrData<'_>) -> EditCollectionModal {
    let mut edit_collection_modal = EditCollectionModal::default();
    let Collection {
      root_folder_path,
      monitored,
      search_on_add,
      minimum_availability,
      quality_profile_id,
      ..
    } = if let Some(filtered_collections) = radarr_data.filtered_collections.as_ref() {
      filtered_collections.current_selection()
    } else {
      radarr_data.collections.current_selection()
    };

    edit_collection_modal.path = root_folder_path.clone().unwrap_or_default().into();
    edit_collection_modal.monitored = Some(*monitored);
    edit_collection_modal.search_on_add = Some(*search_on_add);
    edit_collection_modal
      .minimum_availability_list
      .set_items(Vec::from_iter(MinimumAvailability::iter()));
    let mut quality_profile_names: Vec<String> = radarr_data
      .quality_profile_map
      .right_values()
      .cloned()
      .collect();
    quality_profile_names.sort();
    edit_collection_modal
      .quality_profile_list
      .set_items(quality_profile_names);

    let minimum_availability_index = edit_collection_modal
      .minimum_availability_list
      .items
      .iter()
      .position(|ma| ma == minimum_availability);
    edit_collection_modal
      .minimum_availability_list
      .state
      .select(minimum_availability_index);

    let quality_profile_name = radarr_data
      .quality_profile_map
      .get_by_left(quality_profile_id)
      .unwrap();
    let quality_profile_index = edit_collection_modal
      .quality_profile_list
      .items
      .iter()
      .position(|profile| profile == quality_profile_name);
    edit_collection_modal
      .quality_profile_list
      .state
      .select(quality_profile_index);

    edit_collection_modal
  }
}
