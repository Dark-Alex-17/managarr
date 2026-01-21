use super::modals::{AddSeriesModal, EditSeriesModal, SeasonDetailsModal};
use crate::{
  app::{
    context_clues::{
      BLOCKLIST_CONTEXT_CLUES, DOWNLOADS_CONTEXT_CLUES, HISTORY_CONTEXT_CLUES,
      INDEXERS_CONTEXT_CLUES, ROOT_FOLDERS_CONTEXT_CLUES, SYSTEM_CONTEXT_CLUES,
    },
    sonarr::sonarr_context_clues::{
      SERIES_CONTEXT_CLUES, SERIES_DETAILS_CONTEXT_CLUES, SERIES_HISTORY_CONTEXT_CLUES,
    },
  },
  models::{
    BlockSelectionState, HorizontallyScrollableText, Route, ScrollableText, TabRoute, TabState,
    servarr_data::modals::{EditIndexerModal, IndexerTestResultModalItem},
    servarr_models::{DiskSpace, Indexer, IndexerSettings, QueueEvent, RootFolder},
    sonarr_models::{
      AddSeriesSearchResult, BlocklistItem, DownloadRecord, Season, Series, SonarrHistoryItem,
      SonarrTask,
    },
    stateful_list::StatefulList,
    stateful_table::StatefulTable,
  },
  network::sonarr_network::SonarrEvent,
};
use bimap::BiMap;
use chrono::{DateTime, Utc};
use itertools::Itertools;
use serde_json::Number;
use strum::EnumIter;
#[cfg(test)]
use {
  super::modals::EpisodeDetailsModal,
  crate::models::sonarr_models::{SeriesMonitor, SeriesType},
  crate::models::stateful_table::SortOption,
  crate::network::servarr_test_utils::diskspace,
  crate::network::servarr_test_utils::indexer_settings,
  crate::network::servarr_test_utils::indexer_test_result,
  crate::network::servarr_test_utils::queued_event,
  crate::network::sonarr_network::sonarr_network_test_utils::test_utils::{
    add_series_search_result, blocklist_item, download_record, indexer, log_line, root_folder,
    sonarr_history_item,
  },
  crate::network::sonarr_network::sonarr_network_test_utils::test_utils::{
    episode, episode_file, language_profiles_map, quality_profile_map, season, series, tags_map,
    task, torrent_release, updates, usenet_release,
  },
  crate::sort_option,
  strum::IntoEnumIterator,
  strum_macros::{Display, EnumString},
};

#[cfg(test)]
#[path = "sonarr_data_tests.rs"]
mod sonarr_data_tests;

#[cfg(test)]
#[path = "sonarr_test_utils.rs"]
pub mod sonarr_test_utils;

pub struct SonarrData<'a> {
  pub add_list_exclusion: bool,
  pub add_searched_series: Option<StatefulTable<AddSeriesSearchResult>>,
  pub add_series_modal: Option<AddSeriesModal>,
  pub add_series_search: Option<HorizontallyScrollableText>,
  pub blocklist: StatefulTable<BlocklistItem>,
  pub delete_series_files: bool,
  pub downloads: StatefulTable<DownloadRecord>,
  pub disk_space_vec: Vec<DiskSpace>,
  pub edit_indexer_modal: Option<EditIndexerModal>,
  pub edit_root_folder: Option<HorizontallyScrollableText>,
  pub edit_series_modal: Option<EditSeriesModal>,
  pub history: StatefulTable<SonarrHistoryItem>,
  pub indexers: StatefulTable<Indexer>,
  pub indexer_settings: Option<IndexerSettings>,
  pub indexer_test_all_results: Option<StatefulTable<IndexerTestResultModalItem>>,
  pub indexer_test_errors: Option<String>,
  pub language_profiles_map: BiMap<i64, String>,
  pub logs: StatefulList<HorizontallyScrollableText>,
  pub log_details: StatefulList<HorizontallyScrollableText>,
  pub main_tabs: TabState,
  pub prompt_confirm: bool,
  pub prompt_confirm_action: Option<SonarrEvent>,
  pub quality_profile_map: BiMap<i64, String>,
  pub queued_events: StatefulTable<QueueEvent>,
  pub root_folders: StatefulTable<RootFolder>,
  pub seasons: StatefulTable<Season>,
  pub season_details_modal: Option<SeasonDetailsModal>,
  pub selected_block: BlockSelectionState<'a, ActiveSonarrBlock>,
  pub series: StatefulTable<Series>,
  pub series_history: Option<StatefulTable<SonarrHistoryItem>>,
  pub series_info_tabs: TabState,
  pub start_time: DateTime<Utc>,
  pub tags_map: BiMap<i64, String>,
  pub tasks: StatefulTable<SonarrTask>,
  pub updates: ScrollableText,
  pub version: String,
}

impl SonarrData<'_> {
  pub fn reset_delete_series_preferences(&mut self) {
    self.delete_series_files = false;
    self.add_list_exclusion = false;
  }

  pub fn reset_series_info_tabs(&mut self) {
    self.series_history = None;
    self.seasons = StatefulTable::default();
    self.series_info_tabs.index = 0;
  }

  pub fn tag_ids_to_display(&self, tag_ids: &[Number]) -> String {
    tag_ids
      .iter()
      .filter_map(|tag_id| {
        let id = tag_id.as_i64()?;
        self.tags_map.get_by_left(&id).cloned()
      })
      .collect::<Vec<_>>()
      .join(", ")
  }

  pub fn sorted_quality_profile_names(&self) -> Vec<String> {
    self
      .quality_profile_map
      .iter()
      .sorted_by_key(|(id, _)| *id)
      .map(|(_, name)| name)
      .cloned()
      .collect()
  }

  pub fn sorted_language_profile_names(&self) -> Vec<String> {
    self
      .language_profiles_map
      .iter()
      .sorted_by_key(|(id, _)| *id)
      .map(|(_, name)| name)
      .cloned()
      .collect()
  }
}

impl<'a> Default for SonarrData<'a> {
  fn default() -> SonarrData<'a> {
    SonarrData {
      add_list_exclusion: false,
      add_searched_series: None,
      add_series_search: None,
      add_series_modal: None,
      blocklist: StatefulTable::default(),
      downloads: StatefulTable::default(),
      delete_series_files: false,
      disk_space_vec: Vec::new(),
      edit_indexer_modal: None,
      edit_root_folder: None,
      edit_series_modal: None,
      history: StatefulTable::default(),
      indexers: StatefulTable::default(),
      indexer_settings: None,
      indexer_test_errors: None,
      indexer_test_all_results: None,
      language_profiles_map: BiMap::new(),
      logs: StatefulList::default(),
      log_details: StatefulList::default(),
      prompt_confirm: false,
      prompt_confirm_action: None,
      quality_profile_map: BiMap::new(),
      queued_events: StatefulTable::default(),
      root_folders: StatefulTable::default(),
      seasons: StatefulTable::default(),
      season_details_modal: None,
      selected_block: BlockSelectionState::default(),
      series: StatefulTable::default(),
      series_history: None,
      start_time: DateTime::default(),
      tags_map: BiMap::default(),
      tasks: StatefulTable::default(),
      updates: ScrollableText::default(),
      version: String::new(),
      main_tabs: TabState::new(vec![
        TabRoute {
          title: "Library".to_string(),
          route: ActiveSonarrBlock::Series.into(),
          contextual_help: Some(&SERIES_CONTEXT_CLUES),
          config: None,
        },
        TabRoute {
          title: "Downloads".to_string(),
          route: ActiveSonarrBlock::Downloads.into(),
          contextual_help: Some(&DOWNLOADS_CONTEXT_CLUES),
          config: None,
        },
        TabRoute {
          title: "Blocklist".to_string(),
          route: ActiveSonarrBlock::Blocklist.into(),
          contextual_help: Some(&BLOCKLIST_CONTEXT_CLUES),
          config: None,
        },
        TabRoute {
          title: "History".to_string(),
          route: ActiveSonarrBlock::History.into(),
          contextual_help: Some(&HISTORY_CONTEXT_CLUES),
          config: None,
        },
        TabRoute {
          title: "Root Folders".to_string(),
          route: ActiveSonarrBlock::RootFolders.into(),
          contextual_help: Some(&ROOT_FOLDERS_CONTEXT_CLUES),
          config: None,
        },
        TabRoute {
          title: "Indexers".to_string(),
          route: ActiveSonarrBlock::Indexers.into(),
          contextual_help: Some(&INDEXERS_CONTEXT_CLUES),
          config: None,
        },
        TabRoute {
          title: "System".to_string(),
          route: ActiveSonarrBlock::System.into(),
          contextual_help: Some(&SYSTEM_CONTEXT_CLUES),
          config: None,
        },
      ]),
      series_info_tabs: TabState::new(vec![
        TabRoute {
          title: "Seasons".to_string(),
          route: ActiveSonarrBlock::SeriesDetails.into(),
          contextual_help: Some(&SERIES_DETAILS_CONTEXT_CLUES),
          config: None,
        },
        TabRoute {
          title: "History".to_string(),
          route: ActiveSonarrBlock::SeriesHistory.into(),
          contextual_help: Some(&SERIES_HISTORY_CONTEXT_CLUES),
          config: None,
        },
      ]),
    }
  }
}

#[cfg(test)]
impl SonarrData<'_> {
  pub fn test_default_fully_populated() -> Self {
    let quality_profile_name = "Bluray-1080p".to_owned();
    let language_profile_name = "English".to_owned();
    let mut add_searched_series = StatefulTable::default();
    add_searched_series.set_items(vec![add_series_search_result()]);

    let mut add_series_modal = AddSeriesModal {
      use_season_folder: true,
      tags: "alex".into(),
      ..AddSeriesModal::default()
    };
    add_series_modal
      .root_folder_list
      .set_items(vec![root_folder()]);
    add_series_modal
      .monitor_list
      .set_items(SeriesMonitor::iter().collect());
    add_series_modal
      .quality_profile_list
      .set_items(vec![quality_profile_name.clone()]);
    add_series_modal
      .language_profile_list
      .set_items(vec![language_profile_name.clone()]);
    add_series_modal
      .series_type_list
      .set_items(SeriesType::iter().collect());

    let edit_indexer_modal = EditIndexerModal {
      name: "DrunkenSlug".into(),
      enable_rss: Some(true),
      enable_automatic_search: Some(true),
      enable_interactive_search: Some(true),
      url: "http://127.0.0.1:9696/1/".into(),
      api_key: "someApiKey".into(),
      seed_ratio: "ratio".into(),
      tags: "25".into(),
      priority: 1,
    };

    let mut edit_series_modal = EditSeriesModal {
      monitored: Some(true),
      use_season_folders: Some(true),
      path: "/nfs/tv".into(),
      tags: "alex".into(),
      ..EditSeriesModal::default()
    };
    edit_series_modal
      .series_type_list
      .set_items(SeriesType::iter().collect());
    edit_series_modal
      .quality_profile_list
      .set_items(vec![quality_profile_name.clone()]);
    edit_series_modal
      .language_profile_list
      .set_items(vec![language_profile_name.clone()]);

    let mut indexer_test_all_results = StatefulTable::default();
    indexer_test_all_results.set_items(vec![indexer_test_result()]);

    let mut episode_details_modal = EpisodeDetailsModal {
      episode_details: ScrollableText::with_string("Some episode details".into()),
      file_details: "Some file details".to_owned(),
      audio_details: "Some audio details".to_owned(),
      video_details: "Some video details".to_owned(),
      ..EpisodeDetailsModal::default()
    };
    episode_details_modal
      .episode_history
      .set_items(vec![sonarr_history_item()]);
    episode_details_modal
      .episode_releases
      .set_items(vec![torrent_release(), usenet_release()]);
    episode_details_modal
      .episode_releases
      .sorting(vec![sort_option!(indexer_id)]);

    let mut season_details_modal = SeasonDetailsModal {
      episode_details_modal: Some(episode_details_modal),
      ..SeasonDetailsModal::default()
    };
    season_details_modal.episodes.set_items(vec![episode()]);
    season_details_modal.episodes.search = Some("episode search".into());
    season_details_modal
      .episode_files
      .set_items(vec![episode_file()]);
    season_details_modal
      .season_history
      .set_items(vec![sonarr_history_item()]);
    season_details_modal.season_history.search = Some("season history search".into());
    season_details_modal.season_history.filter = Some("season history filter".into());
    season_details_modal
      .season_history
      .sorting(vec![sort_option!(id)]);
    season_details_modal
      .season_releases
      .set_items(vec![torrent_release(), usenet_release()]);
    season_details_modal
      .season_releases
      .sorting(vec![sort_option!(indexer_id)]);

    let mut series_history = StatefulTable::default();
    series_history.set_items(vec![sonarr_history_item()]);
    series_history.sorting(vec![sort_option!(id)]);
    series_history.search = Some("series history search".into());
    series_history.filter = Some("series history filter".into());

    let mut sonarr_data = SonarrData {
      add_list_exclusion: true,
      add_searched_series: Some(add_searched_series),
      add_series_modal: Some(add_series_modal),
      add_series_search: Some("something".into()),
      delete_series_files: true,
      disk_space_vec: vec![diskspace()],
      edit_indexer_modal: Some(edit_indexer_modal),
      edit_root_folder: Some("/nfs/tv".into()),
      edit_series_modal: Some(edit_series_modal),
      indexer_settings: Some(indexer_settings()),
      indexer_test_all_results: Some(indexer_test_all_results),
      indexer_test_errors: Some("error".to_string()),
      language_profiles_map: language_profiles_map(),
      quality_profile_map: quality_profile_map(),
      season_details_modal: Some(season_details_modal),
      series_history: Some(series_history),
      start_time: DateTime::from(DateTime::parse_from_rfc3339("2023-05-20T21:29:16Z").unwrap()),
      tags_map: tags_map(),
      updates: updates(),
      version: "1.2.3.4".to_owned(),
      ..SonarrData::default()
    };

    sonarr_data.blocklist.set_items(vec![blocklist_item()]);
    sonarr_data.blocklist.sorting(vec![sort_option!(id)]);
    sonarr_data.downloads.set_items(vec![download_record()]);
    sonarr_data.history.set_items(vec![sonarr_history_item()]);
    sonarr_data.history.sorting(vec![sort_option!(id)]);
    sonarr_data.history.search = Some("test search".into());
    sonarr_data.history.filter = Some("test filter".into());
    sonarr_data.indexers.set_items(vec![indexer()]);
    sonarr_data.queued_events.set_items(vec![queued_event()]);
    sonarr_data.root_folders.set_items(vec![root_folder()]);
    sonarr_data.seasons.set_items(vec![season()]);
    sonarr_data.seasons.search = Some("season search".into());
    sonarr_data.series.set_items(vec![series()]);
    sonarr_data.series.sorting(vec![sort_option!(id)]);
    sonarr_data.series.search = Some("series search".into());
    sonarr_data.series.filter = Some("series filter".into());
    sonarr_data.logs.set_items(vec![log_line().into()]);
    sonarr_data.log_details.set_items(vec![log_line().into()]);
    sonarr_data.tasks.set_items(vec![task()]);

    sonarr_data
  }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default, EnumIter)]
#[cfg_attr(test, derive(Display, EnumString))]
pub enum ActiveSonarrBlock {
  AddRootFolderPrompt,
  AddSeriesAlreadyInLibrary,
  AddSeriesConfirmPrompt,
  AddSeriesEmptySearchResults,
  AddSeriesPrompt,
  AddSeriesSearchInput,
  AddSeriesSearchResults,
  AddSeriesSelectLanguageProfile,
  AddSeriesSelectMonitor,
  AddSeriesSelectQualityProfile,
  AddSeriesSelectRootFolder,
  AddSeriesSelectSeriesType,
  AddSeriesTagsInput,
  AddSeriesToggleUseSeasonFolder,
  AllIndexerSettingsPrompt,
  AutomaticallySearchEpisodePrompt,
  AutomaticallySearchSeasonPrompt,
  AutomaticallySearchSeriesPrompt,
  Blocklist,
  BlocklistClearAllItemsPrompt,
  BlocklistItemDetails,
  BlocklistSortPrompt,
  DeleteBlocklistItemPrompt,
  DeleteDownloadPrompt,
  DeleteEpisodeFilePrompt,
  DeleteIndexerPrompt,
  DeleteRootFolderPrompt,
  DeleteSeriesConfirmPrompt,
  DeleteSeriesPrompt,
  DeleteSeriesToggleAddListExclusion,
  DeleteSeriesToggleDeleteFile,
  Downloads,
  EditIndexerPrompt,
  EditIndexerConfirmPrompt,
  EditIndexerApiKeyInput,
  EditIndexerNameInput,
  EditIndexerSeedRatioInput,
  EditIndexerToggleEnableRss,
  EditIndexerToggleEnableAutomaticSearch,
  EditIndexerToggleEnableInteractiveSearch,
  EditIndexerUrlInput,
  EditIndexerPriorityInput,
  EditIndexerTagsInput,
  EditSeriesPrompt,
  EditSeriesConfirmPrompt,
  EditSeriesPathInput,
  EditSeriesSelectSeriesType,
  EditSeriesSelectQualityProfile,
  EditSeriesSelectLanguageProfile,
  EditSeriesTagsInput,
  EditSeriesToggleMonitored,
  EditSeriesToggleSeasonFolder,
  EpisodeDetails,
  EpisodeFile,
  EpisodeHistory,
  EpisodeHistoryDetails,
  EpisodesSortPrompt,
  FilterEpisodes,
  FilterEpisodesError,
  FilterHistory,
  FilterHistoryError,
  FilterSeries,
  FilterSeriesError,
  FilterSeriesHistory,
  FilterSeriesHistoryError,
  FilterSeasonHistory,
  FilterSeasonHistoryError,
  History,
  HistoryItemDetails,
  HistorySortPrompt,
  Indexers,
  IndexerSettingsConfirmPrompt,
  IndexerSettingsMaximumSizeInput,
  IndexerSettingsMinimumAgeInput,
  IndexerSettingsRetentionInput,
  IndexerSettingsRssSyncIntervalInput,
  ManualEpisodeSearch,
  ManualEpisodeSearchConfirmPrompt,
  ManualEpisodeSearchSortPrompt,
  ManualSeasonSearch,
  ManualSeasonSearchConfirmPrompt,
  ManualSeasonSearchSortPrompt,
  RootFolders,
  SearchEpisodes,
  SearchEpisodesError,
  SearchHistory,
  SearchHistoryError,
  SearchSeason,
  SearchSeasonError,
  SearchSeries,
  SearchSeriesError,
  SearchSeriesHistory,
  SearchSeriesHistoryError,
  SearchSeasonHistory,
  SearchSeasonHistoryError,
  SeasonDetails,
  SeasonHistory,
  SeasonHistoryDetails,
  SeasonHistorySortPrompt,
  #[default]
  Series,
  SeriesDetails,
  SeriesHistory,
  SeriesHistoryDetails,
  SeriesHistorySortPrompt,
  SeriesSortPrompt,
  System,
  SystemLogs,
  SystemQueuedEvents,
  SystemTasks,
  SystemTaskStartConfirmPrompt,
  SystemUpdates,
  TestAllIndexers,
  TestIndexer,
  UpdateAllSeriesPrompt,
  UpdateAndScanSeriesPrompt,
  UpdateDownloadsPrompt,
}

pub static LIBRARY_BLOCKS: [ActiveSonarrBlock; 7] = [
  ActiveSonarrBlock::Series,
  ActiveSonarrBlock::SeriesSortPrompt,
  ActiveSonarrBlock::SearchSeries,
  ActiveSonarrBlock::SearchSeriesError,
  ActiveSonarrBlock::FilterSeries,
  ActiveSonarrBlock::FilterSeriesError,
  ActiveSonarrBlock::UpdateAllSeriesPrompt,
];

pub static SERIES_DETAILS_BLOCKS: [ActiveSonarrBlock; 12] = [
  ActiveSonarrBlock::SeriesDetails,
  ActiveSonarrBlock::SeriesHistory,
  ActiveSonarrBlock::SearchSeason,
  ActiveSonarrBlock::SearchSeasonError,
  ActiveSonarrBlock::UpdateAndScanSeriesPrompt,
  ActiveSonarrBlock::AutomaticallySearchSeriesPrompt,
  ActiveSonarrBlock::SearchSeriesHistory,
  ActiveSonarrBlock::SearchSeriesHistoryError,
  ActiveSonarrBlock::FilterSeriesHistory,
  ActiveSonarrBlock::FilterSeriesHistoryError,
  ActiveSonarrBlock::SeriesHistorySortPrompt,
  ActiveSonarrBlock::SeriesHistoryDetails,
];

pub static SEASON_DETAILS_BLOCKS: [ActiveSonarrBlock; 15] = [
  ActiveSonarrBlock::SeasonDetails,
  ActiveSonarrBlock::SeasonHistory,
  ActiveSonarrBlock::SearchEpisodes,
  ActiveSonarrBlock::SearchEpisodesError,
  ActiveSonarrBlock::AutomaticallySearchSeasonPrompt,
  ActiveSonarrBlock::SearchSeasonHistory,
  ActiveSonarrBlock::SearchSeasonHistoryError,
  ActiveSonarrBlock::FilterSeasonHistory,
  ActiveSonarrBlock::FilterSeasonHistoryError,
  ActiveSonarrBlock::SeasonHistorySortPrompt,
  ActiveSonarrBlock::SeasonHistoryDetails,
  ActiveSonarrBlock::ManualSeasonSearch,
  ActiveSonarrBlock::ManualSeasonSearchConfirmPrompt,
  ActiveSonarrBlock::ManualSeasonSearchSortPrompt,
  ActiveSonarrBlock::DeleteEpisodeFilePrompt,
];

pub static EPISODE_DETAILS_BLOCKS: [ActiveSonarrBlock; 8] = [
  ActiveSonarrBlock::EpisodeDetails,
  ActiveSonarrBlock::EpisodeHistory,
  ActiveSonarrBlock::EpisodeHistoryDetails,
  ActiveSonarrBlock::EpisodeFile,
  ActiveSonarrBlock::ManualEpisodeSearch,
  ActiveSonarrBlock::ManualEpisodeSearchConfirmPrompt,
  ActiveSonarrBlock::ManualEpisodeSearchSortPrompt,
  ActiveSonarrBlock::AutomaticallySearchEpisodePrompt,
];

pub static ADD_SERIES_BLOCKS: [ActiveSonarrBlock; 13] = [
  ActiveSonarrBlock::AddSeriesAlreadyInLibrary,
  ActiveSonarrBlock::AddSeriesConfirmPrompt,
  ActiveSonarrBlock::AddSeriesEmptySearchResults,
  ActiveSonarrBlock::AddSeriesPrompt,
  ActiveSonarrBlock::AddSeriesSearchInput,
  ActiveSonarrBlock::AddSeriesSearchResults,
  ActiveSonarrBlock::AddSeriesSelectLanguageProfile,
  ActiveSonarrBlock::AddSeriesSelectMonitor,
  ActiveSonarrBlock::AddSeriesSelectQualityProfile,
  ActiveSonarrBlock::AddSeriesSelectRootFolder,
  ActiveSonarrBlock::AddSeriesSelectSeriesType,
  ActiveSonarrBlock::AddSeriesTagsInput,
  ActiveSonarrBlock::AddSeriesToggleUseSeasonFolder,
];

pub const ADD_SERIES_SELECTION_BLOCKS: &[&[ActiveSonarrBlock]] = &[
  &[ActiveSonarrBlock::AddSeriesSelectRootFolder],
  &[ActiveSonarrBlock::AddSeriesSelectMonitor],
  &[ActiveSonarrBlock::AddSeriesSelectQualityProfile],
  &[ActiveSonarrBlock::AddSeriesSelectLanguageProfile],
  &[ActiveSonarrBlock::AddSeriesSelectSeriesType],
  &[ActiveSonarrBlock::AddSeriesToggleUseSeasonFolder],
  &[ActiveSonarrBlock::AddSeriesTagsInput],
  &[ActiveSonarrBlock::AddSeriesConfirmPrompt],
];

pub static BLOCKLIST_BLOCKS: [ActiveSonarrBlock; 5] = [
  ActiveSonarrBlock::Blocklist,
  ActiveSonarrBlock::BlocklistItemDetails,
  ActiveSonarrBlock::DeleteBlocklistItemPrompt,
  ActiveSonarrBlock::BlocklistClearAllItemsPrompt,
  ActiveSonarrBlock::BlocklistSortPrompt,
];

pub static EDIT_SERIES_BLOCKS: [ActiveSonarrBlock; 9] = [
  ActiveSonarrBlock::EditSeriesPrompt,
  ActiveSonarrBlock::EditSeriesConfirmPrompt,
  ActiveSonarrBlock::EditSeriesPathInput,
  ActiveSonarrBlock::EditSeriesSelectSeriesType,
  ActiveSonarrBlock::EditSeriesSelectQualityProfile,
  ActiveSonarrBlock::EditSeriesSelectLanguageProfile,
  ActiveSonarrBlock::EditSeriesTagsInput,
  ActiveSonarrBlock::EditSeriesToggleMonitored,
  ActiveSonarrBlock::EditSeriesToggleSeasonFolder,
];

pub static EDIT_SERIES_SELECTION_BLOCKS: &[&[ActiveSonarrBlock]] = &[
  &[ActiveSonarrBlock::EditSeriesToggleMonitored],
  &[ActiveSonarrBlock::EditSeriesToggleSeasonFolder],
  &[ActiveSonarrBlock::EditSeriesSelectQualityProfile],
  &[ActiveSonarrBlock::EditSeriesSelectLanguageProfile],
  &[ActiveSonarrBlock::EditSeriesSelectSeriesType],
  &[ActiveSonarrBlock::EditSeriesPathInput],
  &[ActiveSonarrBlock::EditSeriesTagsInput],
  &[ActiveSonarrBlock::EditSeriesConfirmPrompt],
];

pub static DOWNLOADS_BLOCKS: [ActiveSonarrBlock; 3] = [
  ActiveSonarrBlock::Downloads,
  ActiveSonarrBlock::DeleteDownloadPrompt,
  ActiveSonarrBlock::UpdateDownloadsPrompt,
];

pub static DELETE_SERIES_BLOCKS: [ActiveSonarrBlock; 4] = [
  ActiveSonarrBlock::DeleteSeriesPrompt,
  ActiveSonarrBlock::DeleteSeriesConfirmPrompt,
  ActiveSonarrBlock::DeleteSeriesToggleDeleteFile,
  ActiveSonarrBlock::DeleteSeriesToggleAddListExclusion,
];

pub const DELETE_SERIES_SELECTION_BLOCKS: &[&[ActiveSonarrBlock]] = &[
  &[ActiveSonarrBlock::DeleteSeriesToggleDeleteFile],
  &[ActiveSonarrBlock::DeleteSeriesToggleAddListExclusion],
  &[ActiveSonarrBlock::DeleteSeriesConfirmPrompt],
];

pub static EDIT_INDEXER_BLOCKS: [ActiveSonarrBlock; 11] = [
  ActiveSonarrBlock::EditIndexerPrompt,
  ActiveSonarrBlock::EditIndexerConfirmPrompt,
  ActiveSonarrBlock::EditIndexerApiKeyInput,
  ActiveSonarrBlock::EditIndexerNameInput,
  ActiveSonarrBlock::EditIndexerSeedRatioInput,
  ActiveSonarrBlock::EditIndexerToggleEnableRss,
  ActiveSonarrBlock::EditIndexerToggleEnableAutomaticSearch,
  ActiveSonarrBlock::EditIndexerToggleEnableInteractiveSearch,
  ActiveSonarrBlock::EditIndexerPriorityInput,
  ActiveSonarrBlock::EditIndexerUrlInput,
  ActiveSonarrBlock::EditIndexerTagsInput,
];

pub const EDIT_INDEXER_TORRENT_SELECTION_BLOCKS: &[&[ActiveSonarrBlock]] = &[
  &[
    ActiveSonarrBlock::EditIndexerNameInput,
    ActiveSonarrBlock::EditIndexerUrlInput,
  ],
  &[
    ActiveSonarrBlock::EditIndexerToggleEnableRss,
    ActiveSonarrBlock::EditIndexerApiKeyInput,
  ],
  &[
    ActiveSonarrBlock::EditIndexerToggleEnableAutomaticSearch,
    ActiveSonarrBlock::EditIndexerSeedRatioInput,
  ],
  &[
    ActiveSonarrBlock::EditIndexerToggleEnableInteractiveSearch,
    ActiveSonarrBlock::EditIndexerTagsInput,
  ],
  &[
    ActiveSonarrBlock::EditIndexerPriorityInput,
    ActiveSonarrBlock::EditIndexerConfirmPrompt,
  ],
  &[
    ActiveSonarrBlock::EditIndexerConfirmPrompt,
    ActiveSonarrBlock::EditIndexerConfirmPrompt,
  ],
];

pub const EDIT_INDEXER_NZB_SELECTION_BLOCKS: &[&[ActiveSonarrBlock]] = &[
  &[
    ActiveSonarrBlock::EditIndexerNameInput,
    ActiveSonarrBlock::EditIndexerUrlInput,
  ],
  &[
    ActiveSonarrBlock::EditIndexerToggleEnableRss,
    ActiveSonarrBlock::EditIndexerApiKeyInput,
  ],
  &[
    ActiveSonarrBlock::EditIndexerToggleEnableAutomaticSearch,
    ActiveSonarrBlock::EditIndexerTagsInput,
  ],
  &[
    ActiveSonarrBlock::EditIndexerToggleEnableInteractiveSearch,
    ActiveSonarrBlock::EditIndexerPriorityInput,
  ],
  &[
    ActiveSonarrBlock::EditIndexerConfirmPrompt,
    ActiveSonarrBlock::EditIndexerConfirmPrompt,
  ],
];

pub static INDEXER_SETTINGS_BLOCKS: [ActiveSonarrBlock; 6] = [
  ActiveSonarrBlock::AllIndexerSettingsPrompt,
  ActiveSonarrBlock::IndexerSettingsConfirmPrompt,
  ActiveSonarrBlock::IndexerSettingsMaximumSizeInput,
  ActiveSonarrBlock::IndexerSettingsMinimumAgeInput,
  ActiveSonarrBlock::IndexerSettingsRetentionInput,
  ActiveSonarrBlock::IndexerSettingsRssSyncIntervalInput,
];

pub const INDEXER_SETTINGS_SELECTION_BLOCKS: &[&[ActiveSonarrBlock]] = &[
  &[ActiveSonarrBlock::IndexerSettingsMinimumAgeInput],
  &[ActiveSonarrBlock::IndexerSettingsRetentionInput],
  &[ActiveSonarrBlock::IndexerSettingsMaximumSizeInput],
  &[ActiveSonarrBlock::IndexerSettingsRssSyncIntervalInput],
  &[ActiveSonarrBlock::IndexerSettingsConfirmPrompt],
];

pub static HISTORY_BLOCKS: [ActiveSonarrBlock; 7] = [
  ActiveSonarrBlock::History,
  ActiveSonarrBlock::HistoryItemDetails,
  ActiveSonarrBlock::HistorySortPrompt,
  ActiveSonarrBlock::FilterHistory,
  ActiveSonarrBlock::FilterHistoryError,
  ActiveSonarrBlock::SearchHistory,
  ActiveSonarrBlock::SearchHistoryError,
];

pub static ROOT_FOLDERS_BLOCKS: [ActiveSonarrBlock; 3] = [
  ActiveSonarrBlock::RootFolders,
  ActiveSonarrBlock::AddRootFolderPrompt,
  ActiveSonarrBlock::DeleteRootFolderPrompt,
];

pub static INDEXERS_BLOCKS: [ActiveSonarrBlock; 3] = [
  ActiveSonarrBlock::DeleteIndexerPrompt,
  ActiveSonarrBlock::Indexers,
  ActiveSonarrBlock::TestIndexer,
];

pub static SYSTEM_DETAILS_BLOCKS: [ActiveSonarrBlock; 5] = [
  ActiveSonarrBlock::SystemLogs,
  ActiveSonarrBlock::SystemQueuedEvents,
  ActiveSonarrBlock::SystemTasks,
  ActiveSonarrBlock::SystemTaskStartConfirmPrompt,
  ActiveSonarrBlock::SystemUpdates,
];

impl From<ActiveSonarrBlock> for Route {
  fn from(active_sonarr_block: ActiveSonarrBlock) -> Route {
    Route::Sonarr(active_sonarr_block, None)
  }
}

impl From<(ActiveSonarrBlock, Option<ActiveSonarrBlock>)> for Route {
  fn from(value: (ActiveSonarrBlock, Option<ActiveSonarrBlock>)) -> Route {
    Route::Sonarr(value.0, value.1)
  }
}
