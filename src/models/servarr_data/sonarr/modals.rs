use strum::IntoEnumIterator;

use crate::models::{
  servarr_models::RootFolder,
  sonarr_models::{Episode, SeriesMonitor, SeriesType, SonarrHistoryItem, SonarrRelease},
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

#[derive(Default)]
pub struct EpisodeDetailsModal {
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
