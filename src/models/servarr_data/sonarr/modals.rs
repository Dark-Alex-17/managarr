use strum::IntoEnumIterator;

use super::sonarr_data::{ActiveSonarrBlock, SonarrData};
use crate::app::sonarr::sonarr_context_clues::SELECTABLE_EPISODE_DETAILS_CONTEXT_CLUES;
use crate::models::sonarr_models::EpisodeFile;
use crate::{
  app::sonarr::sonarr_context_clues::{
    EPISODE_DETAILS_CONTEXT_CLUES, MANUAL_EPISODE_SEARCH_CONTEXT_CLUES,
    MANUAL_SEASON_SEARCH_CONTEXT_CLUES, SEASON_DETAILS_CONTEXT_CLUES, SEASON_HISTORY_CONTEXT_CLUES,
  },
  models::{
    HorizontallyScrollableText, ScrollableText, TabRoute, TabState,
    servarr_data::modals::EditIndexerModal,
    servarr_models::{Indexer, RootFolder},
    sonarr_models::{Episode, Series, SeriesMonitor, SeriesType, SonarrHistoryItem, SonarrRelease},
    stateful_list::StatefulList,
    stateful_table::StatefulTable,
  },
};

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

impl From<&SonarrData<'_>> for AddSeriesModal {
  fn from(sonarr_data: &SonarrData<'_>) -> AddSeriesModal {
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

impl From<&SonarrData<'_>> for EditIndexerModal {
  fn from(sonarr_data: &SonarrData<'_>) -> EditIndexerModal {
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
    edit_indexer_modal.priority = *priority;
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

    if let Some(seed_ratio_value) = seed_ratio_value_option {
      edit_indexer_modal.seed_ratio = seed_ratio_value.as_f64().unwrap().to_string().into();
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

impl From<&SonarrData<'_>> for EditSeriesModal {
  fn from(sonarr_data: &SonarrData<'_>) -> EditSeriesModal {
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

pub struct EpisodeDetailsModal {
  pub episode_details: ScrollableText,
  pub file_details: String,
  pub audio_details: String,
  pub video_details: String,
  pub episode_history: StatefulTable<SonarrHistoryItem>,
  pub episode_releases: StatefulTable<SonarrRelease>,
  pub episode_details_tabs: TabState,
}

impl Default for EpisodeDetailsModal {
  fn default() -> EpisodeDetailsModal {
    EpisodeDetailsModal {
      episode_details: ScrollableText::default(),
      file_details: String::new(),
      audio_details: String::new(),
      video_details: String::new(),
      episode_history: StatefulTable::default(),
      episode_releases: StatefulTable::default(),
      episode_details_tabs: TabState::new(vec![
        TabRoute {
          title: "Details".to_string(),
          route: ActiveSonarrBlock::EpisodeDetails.into(),
          contextual_help: Some(&EPISODE_DETAILS_CONTEXT_CLUES),
          config: None,
        },
        TabRoute {
          title: "History".to_string(),
          route: ActiveSonarrBlock::EpisodeHistory.into(),
          contextual_help: Some(&SELECTABLE_EPISODE_DETAILS_CONTEXT_CLUES),
          config: None,
        },
        TabRoute {
          title: "File".to_string(),
          route: ActiveSonarrBlock::EpisodeFile.into(),
          contextual_help: Some(&EPISODE_DETAILS_CONTEXT_CLUES),
          config: None,
        },
        TabRoute {
          title: "Manual Search".to_string(),
          route: ActiveSonarrBlock::ManualEpisodeSearch.into(),
          contextual_help: Some(&MANUAL_EPISODE_SEARCH_CONTEXT_CLUES),
          config: None,
        },
      ]),
    }
  }
}

pub struct SeasonDetailsModal {
  pub episodes: StatefulTable<Episode>,
  pub episode_files: StatefulTable<EpisodeFile>,
  pub episode_details_modal: Option<EpisodeDetailsModal>,
  pub season_history: StatefulTable<SonarrHistoryItem>,
  pub season_releases: StatefulTable<SonarrRelease>,
  pub season_details_tabs: TabState,
}

impl Default for SeasonDetailsModal {
  fn default() -> SeasonDetailsModal {
    SeasonDetailsModal {
      episodes: StatefulTable::default(),
      episode_details_modal: None,
      episode_files: StatefulTable::default(),
      season_releases: StatefulTable::default(),
      season_history: StatefulTable::default(),
      season_details_tabs: TabState::new(vec![
        TabRoute {
          title: "Episodes".to_string(),
          route: ActiveSonarrBlock::SeasonDetails.into(),
          contextual_help: Some(&SEASON_DETAILS_CONTEXT_CLUES),
          config: None,
        },
        TabRoute {
          title: "History".to_string(),
          route: ActiveSonarrBlock::SeasonHistory.into(),
          contextual_help: Some(&SEASON_HISTORY_CONTEXT_CLUES),
          config: None,
        },
        TabRoute {
          title: "Manual Search".to_string(),
          route: ActiveSonarrBlock::ManualSeasonSearch.into(),
          contextual_help: Some(&MANUAL_SEASON_SEARCH_CONTEXT_CLUES),
          config: None,
        },
      ]),
    }
  }
}
