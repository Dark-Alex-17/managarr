use strum::IntoEnumIterator;

use crate::models::radarr_models::{
  Collection, Credit, MinimumAvailability, Movie, MovieHistoryItem, MovieMonitor, RadarrRelease,
};
use crate::models::servarr_data::modals::EditIndexerModal;
use crate::models::servarr_data::radarr::radarr_data::RadarrData;
use crate::models::servarr_models::{Indexer, RootFolder};
use crate::models::stateful_list::StatefulList;
use crate::models::stateful_table::StatefulTable;
use crate::models::{HorizontallyScrollableText, ScrollableText};

#[cfg(test)]
#[path = "modals_tests.rs"]
mod modals_tests;

#[derive(Default)]
#[cfg_attr(test, derive(Debug))]
pub struct MovieDetailsModal {
  pub movie_details: ScrollableText,
  pub file_details: String,
  pub audio_details: String,
  pub video_details: String,
  pub movie_history: StatefulTable<MovieHistoryItem>,
  pub movie_cast: StatefulTable<Credit>,
  pub movie_crew: StatefulTable<Credit>,
  pub movie_releases: StatefulTable<RadarrRelease>,
}

impl From<&RadarrData<'_>> for EditIndexerModal {
  fn from(radarr_data: &RadarrData<'_>) -> EditIndexerModal {
    let mut edit_indexer_modal = EditIndexerModal::default();
    let Indexer {
      name,
      enable_rss,
      enable_automatic_search,
      enable_interactive_search,
      tags,
      fields,
      priority,
      ..
    } = radarr_data.indexers.current_selection();
    let seed_ratio_field_option = fields
      .as_ref()
      .expect("indexer fields must exist")
      .iter()
      .find(|field| {
        field.name.as_ref().expect("indexer field name must exist") == "seedCriteria.seedRatio"
      });
    let seed_ratio_value_option = if let Some(seed_ratio_field) = seed_ratio_field_option {
      seed_ratio_field.value.clone()
    } else {
      None
    };

    edit_indexer_modal.name = name.clone().expect("indexer name must exist").into();
    edit_indexer_modal.enable_rss = Some(*enable_rss);
    edit_indexer_modal.enable_automatic_search = Some(*enable_automatic_search);
    edit_indexer_modal.enable_interactive_search = Some(*enable_interactive_search);
    edit_indexer_modal.priority = *priority;
    edit_indexer_modal.url = fields
      .as_ref()
      .expect("indexer fields must exist")
      .iter()
      .find(|field| field.name.as_ref().expect("indexer field name must exist") == "baseUrl")
      .expect("baseUrl field must exist")
      .value
      .clone()
      .expect("baseUrl field value must exist")
      .as_str()
      .expect("baseUrl field value must be a string")
      .into();
    edit_indexer_modal.api_key = fields
      .as_ref()
      .expect("indexer fields must exist")
      .iter()
      .find(|field| field.name.as_ref().expect("indexer field name must exist") == "apiKey")
      .expect("apiKey field must exist")
      .value
      .clone()
      .expect("apiKey field value must exist")
      .as_str()
      .expect("apiKey field value must be a string")
      .into();

    if let Some(seed_ratio_value) = seed_ratio_value_option {
      edit_indexer_modal.seed_ratio = seed_ratio_value
        .as_f64()
        .expect("Seed ratio value must be a valid f64")
        .to_string()
        .into();
    }

    edit_indexer_modal.tags = radarr_data.tag_ids_to_display(tags).into();

    edit_indexer_modal
  }
}

#[derive(Default)]
#[cfg_attr(test, derive(Debug))]
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
    } = radarr_data.movies.current_selection();

    edit_movie_modal
      .minimum_availability_list
      .set_items(Vec::from_iter(MinimumAvailability::iter()));
    edit_movie_modal.path = path.clone().into();
    edit_movie_modal.tags = radarr_data.tag_ids_to_display(tags).into();

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

    edit_movie_modal
      .quality_profile_list
      .set_items(radarr_data.sorted_quality_profile_names());
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
#[cfg_attr(test, derive(Debug))]
pub struct AddMovieModal {
  pub root_folder_list: StatefulList<RootFolder>,
  pub monitor_list: StatefulList<MovieMonitor>,
  pub minimum_availability_list: StatefulList<MinimumAvailability>,
  pub quality_profile_list: StatefulList<String>,
  pub tags: HorizontallyScrollableText,
}

impl From<&RadarrData<'_>> for AddMovieModal {
  fn from(radarr_data: &RadarrData<'_>) -> AddMovieModal {
    let mut add_movie_modal = AddMovieModal::default();
    add_movie_modal
      .monitor_list
      .set_items(Vec::from_iter(MovieMonitor::iter()));
    add_movie_modal
      .minimum_availability_list
      .set_items(Vec::from_iter(MinimumAvailability::iter()));
    add_movie_modal
      .quality_profile_list
      .set_items(radarr_data.sorted_quality_profile_names());
    add_movie_modal
      .root_folder_list
      .set_items(radarr_data.root_folders.items.to_vec());

    add_movie_modal
  }
}

#[derive(Default)]
#[cfg_attr(test, derive(Debug))]
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
    } = radarr_data.collections.current_selection();

    edit_collection_modal.path = root_folder_path.clone().unwrap_or_default().into();
    edit_collection_modal.monitored = Some(*monitored);
    edit_collection_modal.search_on_add = Some(*search_on_add);
    edit_collection_modal
      .minimum_availability_list
      .set_items(Vec::from_iter(MinimumAvailability::iter()));
    edit_collection_modal
      .quality_profile_list
      .set_items(radarr_data.sorted_quality_profile_names());

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
