#[cfg(test)]
pub mod utils {
  use crate::models::{
    servarr_data::sonarr::{
      modals::{EpisodeDetailsModal, SeasonDetailsModal},
      sonarr_data::SonarrData,
    },
    sonarr_models::{AddSeriesSearchResult, Episode, Season, SonarrHistoryItem, SonarrRelease},
    stateful_table::StatefulTable,
    HorizontallyScrollableText, ScrollableText,
  };
  use crate::models::servarr_models::{Indexer, RootFolder};
  use crate::models::sonarr_models::{BlocklistItem, Series};

  pub fn create_test_sonarr_data<'a>() -> SonarrData<'a> {
    let mut episode_details_modal = EpisodeDetailsModal {
      episode_details: ScrollableText::with_string("test episode details".to_owned()),
      ..EpisodeDetailsModal::default()
    };
    episode_details_modal
      .episode_history
      .set_items(vec![SonarrHistoryItem::default()]);
    episode_details_modal
      .episode_releases
      .set_items(vec![SonarrRelease::default()]);
    let mut season_details_modal = SeasonDetailsModal::default();
    season_details_modal
      .episodes
      .set_items(vec![Episode::default()]);
    season_details_modal.season_history.set_items(vec![SonarrHistoryItem::default()]);
    season_details_modal
      .season_releases
      .set_items(vec![SonarrRelease::default()]);
    season_details_modal.episode_details_modal = Some(episode_details_modal);

    let mut seasons = StatefulTable::default();
    seasons.set_items(vec![Season::default()]);
    let mut series_history = StatefulTable::default();
    series_history.set_items(vec![SonarrHistoryItem::default()]);

    let mut sonarr_data = SonarrData {
      delete_series_files: true,
      add_list_exclusion: true,
      add_series_search: Some("test search".into()),
      edit_root_folder: Some("test path".into()),
      seasons,
      series_history: Some(series_history),
      season_details_modal: Some(season_details_modal),
      add_searched_series: Some(StatefulTable::default()),
      ..SonarrData::default()
    };
    sonarr_data.series.set_items(vec![Series::default()]);
    sonarr_data.history.set_items(vec![SonarrHistoryItem::default()]);
    sonarr_data.blocklist.set_items(vec![BlocklistItem::default()]);
    sonarr_data.root_folders.set_items(vec![RootFolder::default()]);
    sonarr_data.indexers.set_items(vec![Indexer::default()]);
    sonarr_data.series_info_tabs.index = 1;
    sonarr_data
      .add_searched_series
      .as_mut()
      .unwrap()
      .set_items(vec![AddSeriesSearchResult::default()]);
    sonarr_data
      .log_details
      .set_items(vec![HorizontallyScrollableText::default()]);

    sonarr_data
  }
}
