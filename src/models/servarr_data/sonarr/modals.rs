use strum::IntoEnumIterator;

use crate::models::{
  servarr_data::modals::EditIndexerModal,
  servarr_models::{Indexer, RootFolder},
  sonarr_models::{Episode, Series, SeriesMonitor, SeriesType, SonarrHistoryItem, SonarrRelease},
  stateful_list::StatefulList,
  stateful_table::StatefulTable,
  HorizontallyScrollableText, ScrollableText,
};

use super::sonarr_data::SonarrData;

#[cfg(test)]
#[path = "modals_tests.rs"]
mod modals_tests;

#[derive(Default)]
pub struct AddSeriesModal {
  pub root_folder_list: StatefulList<RootFolder>,
  pub monitor_list: StatefulList<SeriesMonitor>,
  pub quality_profile_list: StatefulList<String>,
  pub language_profile_list: StatefulList<String>,
  pub series_type_list: StatefulList<SeriesType>,
  pub use_season_folder: bool,
  pub tags: HorizontallyScrollableText,
}

impl From<&SonarrData> for AddSeriesModal {
  fn from(sonarr_data: &SonarrData) -> AddSeriesModal {
    let mut add_series_modal = AddSeriesModal {
      use_season_folder: true,
      ..AddSeriesModal::default()
    };
    add_series_modal
      .monitor_list
      .set_items(Vec::from_iter(SeriesMonitor::iter()));
    add_series_modal
      .series_type_list
      .set_items(Vec::from_iter(SeriesType::iter()));
    let mut quality_profile_names: Vec<String> = sonarr_data
      .quality_profile_map
      .right_values()
      .cloned()
      .collect();
    quality_profile_names.sort();
    add_series_modal
      .quality_profile_list
      .set_items(quality_profile_names);
    let mut language_profile_names: Vec<String> = sonarr_data
      .language_profiles_map
      .right_values()
      .cloned()
      .collect();
    language_profile_names.sort();
    add_series_modal
      .language_profile_list
      .set_items(language_profile_names);
    add_series_modal
      .root_folder_list
      .set_items(sonarr_data.root_folders.items.to_vec());

    add_series_modal
  }
}

impl From<&SonarrData> for EditIndexerModal {
  fn from(sonarr_data: &SonarrData) -> EditIndexerModal {
    let mut edit_indexer_modal = EditIndexerModal::default();
    let Indexer {
      name,
      enable_rss,
      enable_automatic_search,
      enable_interactive_search,
      tags,
      fields,
      ..
    } = sonarr_data.indexers.current_selection();
    let seed_ratio_field_option = fields
      .as_ref()
      .unwrap()
      .iter()
      .find(|field| field.name.as_ref().unwrap() == "seedCriteria.seedRatio");
    let seed_ratio_value_option = if let Some(seed_ratio_field) = seed_ratio_field_option {
      seed_ratio_field.value.clone()
    } else {
      None
    };

    edit_indexer_modal.name = name.clone().unwrap().into();
    edit_indexer_modal.enable_rss = Some(*enable_rss);
    edit_indexer_modal.enable_automatic_search = Some(*enable_automatic_search);
    edit_indexer_modal.enable_interactive_search = Some(*enable_interactive_search);
    edit_indexer_modal.url = fields
      .as_ref()
      .unwrap()
      .iter()
      .find(|field| field.name.as_ref().unwrap() == "baseUrl")
      .unwrap()
      .value
      .clone()
      .unwrap()
      .as_str()
      .unwrap()
      .into();
    edit_indexer_modal.api_key = fields
      .as_ref()
      .unwrap()
      .iter()
      .find(|field| field.name.as_ref().unwrap() == "apiKey")
      .unwrap()
      .value
      .clone()
      .unwrap()
      .as_str()
      .unwrap()
      .into();

    if seed_ratio_value_option.is_some() {
      edit_indexer_modal.seed_ratio = seed_ratio_value_option
        .unwrap()
        .as_f64()
        .unwrap()
        .to_string()
        .into();
    }

    edit_indexer_modal.tags = tags
      .iter()
      .map(|tag_id| {
        sonarr_data
          .tags_map
          .get_by_left(&tag_id.as_i64().unwrap())
          .unwrap()
          .clone()
      })
      .collect::<Vec<String>>()
      .join(", ")
      .into();

    edit_indexer_modal
  }
}

#[derive(Default)]
pub struct EditSeriesModal {
  pub series_type_list: StatefulList<SeriesType>,
  pub quality_profile_list: StatefulList<String>,
  pub language_profile_list: StatefulList<String>,
  pub monitored: Option<bool>,
  pub use_season_folders: Option<bool>,
  pub path: HorizontallyScrollableText,
  pub tags: HorizontallyScrollableText,
}

impl From<&SonarrData> for EditSeriesModal {
  fn from(sonarr_data: &SonarrData) -> EditSeriesModal {
    let mut edit_series_modal = EditSeriesModal::default();
    let Series {
      path,
      tags,
      monitored,
      season_folder,
      series_type,
      quality_profile_id,
      language_profile_id,
      ..
    } = sonarr_data.series.current_selection();

    edit_series_modal
      .series_type_list
      .set_items(Vec::from_iter(SeriesType::iter()));
    edit_series_modal.path = path.clone().into();
    edit_series_modal.tags = tags
      .iter()
      .map(|tag_id| {
        sonarr_data
          .tags_map
          .get_by_left(&tag_id.as_i64().unwrap())
          .unwrap()
          .clone()
      })
      .collect::<Vec<String>>()
      .join(", ")
      .into();

    edit_series_modal.monitored = Some(*monitored);
    edit_series_modal.use_season_folders = Some(*season_folder);

    let series_type_index = edit_series_modal
      .series_type_list
      .items
      .iter()
      .position(|st| st == series_type);
    edit_series_modal
      .series_type_list
      .state
      .select(series_type_index);

    let mut quality_profile_names: Vec<String> = sonarr_data
      .quality_profile_map
      .right_values()
      .cloned()
      .collect();
    quality_profile_names.sort();
    edit_series_modal
      .quality_profile_list
      .set_items(quality_profile_names);
    let quality_profile_name = sonarr_data
      .quality_profile_map
      .get_by_left(quality_profile_id)
      .unwrap();
    let quality_profile_index = edit_series_modal
      .quality_profile_list
      .items
      .iter()
      .position(|profile| profile == quality_profile_name);
    edit_series_modal
      .quality_profile_list
      .state
      .select(quality_profile_index);
    let mut language_profile_names: Vec<String> = sonarr_data
      .language_profiles_map
      .right_values()
      .cloned()
      .collect();
    language_profile_names.sort();
    edit_series_modal
      .language_profile_list
      .set_items(language_profile_names);
    let language_profile_name = sonarr_data
      .language_profiles_map
      .get_by_left(language_profile_id)
      .unwrap();
    let language_profile_index = edit_series_modal
      .language_profile_list
      .items
      .iter()
      .position(|profile| profile == language_profile_name);
    edit_series_modal
      .language_profile_list
      .state
      .select(language_profile_index);

    edit_series_modal
  }
}

#[derive(Default)]
pub struct EpisodeDetailsModal {
  // Temporarily allowing this, since the value is only current written and not read.
  // This will be read from once I begin the UI work for Sonarr
  #[allow(dead_code)]
  pub episode_details: ScrollableText,
  pub file_details: String,
  pub audio_details: String,
  pub video_details: String,
  pub episode_history: StatefulTable<SonarrHistoryItem>,
  pub episode_releases: StatefulTable<SonarrRelease>,
}

#[derive(Default)]
pub struct SeasonDetailsModal {
  pub episodes: StatefulTable<Episode>,
  pub episode_details_modal: Option<EpisodeDetailsModal>,
  pub season_releases: StatefulTable<SonarrRelease>,
}
