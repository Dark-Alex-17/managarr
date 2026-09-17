use serde_json::Number;

use super::modals::{
  AddAuthorModal, AddReadarrRootFolderModal, AuthorOverviewModal, BookDetailsModal, EditAuthorModal,
};
use crate::app::context_clues::{
  BLOCKLIST_CONTEXT_CLUES, DOWNLOADS_CONTEXT_CLUES, HISTORY_CONTEXT_CLUES, INDEXERS_CONTEXT_CLUES,
  ROOT_FOLDERS_CONTEXT_CLUES, SYSTEM_CONTEXT_CLUES,
};
use crate::app::readarr::readarr_context_clues::{
  AUTHOR_DETAILS_CONTEXT_CLUES, AUTHOR_HISTORY_CONTEXT_CLUES, AUTHORS_CONTEXT_CLUES,
  MANUAL_AUTHOR_SEARCH_CONTEXT_CLUES,
};
use crate::models::readarr_models::{BlocklistItem, ReadarrRelease, ReadarrTask};
use crate::models::servarr_data::modals::EditIndexerModal;
use crate::models::servarr_models::{IndexerSettings, QueueEvent};
use crate::models::stateful_list::StatefulList;
use crate::models::{
  BlockSelectionState, HorizontallyScrollableText, Route, ScrollableText, TabRoute, TabState,
  readarr_models::{AddAuthorSearchResult, Author, Book, DownloadRecord, ReadarrHistoryItem},
  servarr_data::modals::IndexerTestResultModalItem,
  servarr_models::{DiskSpace, Indexer, RootFolder},
  stateful_table::StatefulTable,
};
use crate::network::readarr_network::ReadarrEvent;
use bimap::BiMap;
use chrono::{DateTime, Utc};
use itertools::Itertools;
use strum::EnumIter;
#[cfg(test)]
use {
  super::modals::EditionDetailsModal,
  crate::models::readarr_models::{
    AuthorStatus, BookFile, BookStatistics, DownloadStatus, Edition, MediaInfo, MonitorType,
    NewItemMonitorType, Ratings, ReadarrHistoryEventType, ReadarrTaskName,
  },
  crate::models::servarr_models::{IndexerField, Quality, QualityWrapper},
  crate::models::stateful_table::SortOption,
  crate::network::servarr_test_utils::{
    diskspace, indexer_settings, indexer_test_result, queued_event,
  },
  crate::network::sonarr_network::sonarr_network_test_utils::test_utils::updates,
  crate::sort_option,
  serde_json::json,
  strum::{Display, EnumString, IntoEnumIterator},
};

#[cfg(test)]
#[path = "readarr_data_tests.rs"]
mod readarr_data_tests;

pub struct ReadarrData<'a> {
  pub add_author_modal: Option<AddAuthorModal>,
  pub add_author_search: Option<HorizontallyScrollableText>,
  pub add_import_list_exclusion: bool,
  pub add_root_folder_modal: Option<AddReadarrRootFolderModal>,
  pub add_searched_authors: Option<StatefulTable<AddAuthorSearchResult>>,
  pub author_history: StatefulTable<ReadarrHistoryItem>,
  pub author_info_tabs: TabState,
  pub author_overview_modal: Option<AuthorOverviewModal>,
  pub author_releases: StatefulTable<ReadarrRelease>,
  pub authors: StatefulTable<Author>,
  pub blocklist: StatefulTable<BlocklistItem>,
  pub book_details_modal: Option<BookDetailsModal>,
  pub books: StatefulTable<Book>,
  pub delete_files: bool,
  pub disk_space_vec: Vec<DiskSpace>,
  pub downloads: StatefulTable<DownloadRecord>,
  pub edit_author_modal: Option<EditAuthorModal>,
  pub edit_indexer_modal: Option<EditIndexerModal>,
  pub history: StatefulTable<ReadarrHistoryItem>,
  pub indexer_settings: Option<IndexerSettings>,
  pub indexer_test_all_results: Option<StatefulTable<IndexerTestResultModalItem>>,
  pub indexer_test_errors: Option<String>,
  pub indexers: StatefulTable<Indexer>,
  pub log_details: StatefulList<HorizontallyScrollableText>,
  pub logs: StatefulList<HorizontallyScrollableText>,
  pub main_tabs: TabState,
  pub metadata_profile_map: BiMap<i64, String>,
  pub prompt_confirm: bool,
  pub prompt_confirm_action: Option<ReadarrEvent>,
  pub quality_profile_map: BiMap<i64, String>,
  pub queued_events: StatefulTable<QueueEvent>,
  pub root_folders: StatefulTable<RootFolder>,
  pub selected_block: BlockSelectionState<'a, ActiveReadarrBlock>,
  pub start_time: DateTime<Utc>,
  pub tags_map: BiMap<i64, String>,
  pub tasks: StatefulTable<ReadarrTask>,
  pub updates: ScrollableText,
  pub version: String,
}

impl ReadarrData<'_> {
  pub fn reset_delete_preferences(&mut self) {
    self.delete_files = false;
    self.add_import_list_exclusion = false;
  }

  pub fn reset_author_info_tabs(&mut self) {
    self.books = StatefulTable::default();
    self.author_releases = StatefulTable::default();
    self.author_history = StatefulTable::default();
    self.author_info_tabs.index = 0;
  }

  pub fn tag_ids_to_display(&self, tag_ids: &[Number]) -> String {
    tag_ids
      .iter()
      .filter_map(|id| {
        let id = id.as_i64()?;
        self.tags_map.get_by_left(&id).cloned()
      })
      .collect::<Vec<String>>()
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

  pub fn sorted_metadata_profile_names(&self) -> Vec<String> {
    self
      .metadata_profile_map
      .iter()
      .sorted_by_key(|(id, _)| *id)
      .map(|(_, name)| name)
      .cloned()
      .collect()
  }
}

impl<'a> Default for ReadarrData<'a> {
  fn default() -> ReadarrData<'a> {
    ReadarrData {
      add_author_modal: None,
      add_author_search: None,
      add_import_list_exclusion: false,
      add_root_folder_modal: None,
      add_searched_authors: None,
      author_history: StatefulTable::default(),
      author_overview_modal: None,
      author_releases: StatefulTable::default(),
      authors: StatefulTable::default(),
      blocklist: StatefulTable::default(),
      book_details_modal: None,
      books: StatefulTable::default(),
      delete_files: false,
      disk_space_vec: Vec::new(),
      downloads: StatefulTable::default(),
      edit_author_modal: None,
      edit_indexer_modal: None,
      history: StatefulTable::default(),
      indexer_settings: None,
      indexer_test_all_results: None,
      indexer_test_errors: None,
      indexers: StatefulTable::default(),
      log_details: StatefulList::default(),
      logs: StatefulList::default(),
      metadata_profile_map: BiMap::new(),
      prompt_confirm: false,
      prompt_confirm_action: None,
      quality_profile_map: BiMap::new(),
      queued_events: StatefulTable::default(),
      root_folders: StatefulTable::default(),
      selected_block: BlockSelectionState::default(),
      start_time: DateTime::default(),
      tags_map: BiMap::new(),
      tasks: StatefulTable::default(),
      updates: ScrollableText::default(),
      version: String::new(),
      main_tabs: TabState::new(vec![
        TabRoute {
          title: "Library".to_string(),
          route: ActiveReadarrBlock::Authors.into(),
          contextual_help: Some(&AUTHORS_CONTEXT_CLUES),
          config: None,
        },
        TabRoute {
          title: "Downloads".to_string(),
          route: ActiveReadarrBlock::Downloads.into(),
          contextual_help: Some(&DOWNLOADS_CONTEXT_CLUES),
          config: None,
        },
        TabRoute {
          title: "Blocklist".to_string(),
          route: ActiveReadarrBlock::Blocklist.into(),
          contextual_help: Some(&BLOCKLIST_CONTEXT_CLUES),
          config: None,
        },
        TabRoute {
          title: "History".to_string(),
          route: ActiveReadarrBlock::History.into(),
          contextual_help: Some(&HISTORY_CONTEXT_CLUES),
          config: None,
        },
        TabRoute {
          title: "Root Folders".to_string(),
          route: ActiveReadarrBlock::RootFolders.into(),
          contextual_help: Some(&ROOT_FOLDERS_CONTEXT_CLUES),
          config: None,
        },
        TabRoute {
          title: "Indexers".to_string(),
          route: ActiveReadarrBlock::Indexers.into(),
          contextual_help: Some(&INDEXERS_CONTEXT_CLUES),
          config: None,
        },
        TabRoute {
          title: "System".to_string(),
          route: ActiveReadarrBlock::System.into(),
          contextual_help: Some(&SYSTEM_CONTEXT_CLUES),
          config: None,
        },
      ]),
      author_info_tabs: TabState::new(vec![
        TabRoute {
          title: "Books".to_string(),
          route: ActiveReadarrBlock::AuthorDetails.into(),
          contextual_help: Some(&AUTHOR_DETAILS_CONTEXT_CLUES),
          config: None,
        },
        TabRoute {
          title: "History".to_string(),
          route: ActiveReadarrBlock::AuthorHistory.into(),
          contextual_help: Some(&AUTHOR_HISTORY_CONTEXT_CLUES),
          config: None,
        },
        TabRoute {
          title: "Author Search".to_string(),
          route: ActiveReadarrBlock::ManualAuthorSearch.into(),
          contextual_help: Some(&MANUAL_AUTHOR_SEARCH_CONTEXT_CLUES),
          config: None,
        },
      ]),
    }
  }
}

#[cfg(test)]
impl ReadarrData<'_> {
  pub fn test_default_fully_populated() -> Self {
    let quality_profile_map = BiMap::from_iter([(1111i64, "Standard".to_owned())]);
    let metadata_profile_map = BiMap::from_iter([(3333i64, "Comprehensive".to_owned())]);
    let tags_map = BiMap::from_iter([(1i64, "alex".to_owned())]);
    let root_folder = RootFolder {
      id: 1,
      path: "/nfs".to_owned(),
      accessible: true,
      free_space: 219902325555200,
      unmapped_folders: None,
    };
    let quality = QualityWrapper {
      quality: Quality {
        name: "EPUB".to_owned(),
      },
    };
    let date = DateTime::from(DateTime::parse_from_rfc3339("2023-05-20T21:29:16Z").unwrap());
    let author = Author {
      id: 1,
      author_name: "Test Author".into(),
      foreign_author_id: "test-foreign-id".to_owned(),
      status: AuthorStatus::Continuing,
      overview: Some(
        "some interesting description of the author\r\n\
         \r\n\
         \tHe was born in Madison, Wisconsin: a city he has never really left. \r\n\
         \r\n\
         His first novel took him seven years to finish.\r\n"
          .to_owned(),
      ),
      path: "/nfs/books/Test Author".to_owned(),
      quality_profile_id: 1111,
      metadata_profile_id: 3333,
      monitored: true,
      monitor_new_items: NewItemMonitorType::All,
      genres: vec!["science fiction".to_owned()],
      tags: vec![Number::from(1)],
      ..Author::default()
    };
    let book = Book {
      id: 1,
      title: "Test Book".into(),
      author_id: 1,
      foreign_book_id: "test-foreign-book-id".to_owned(),
      monitored: true,
      any_edition_ok: true,
      page_count: Some(288),
      release_date: Some(date),
      statistics: Some(BookStatistics {
        book_file_count: 3,
        total_book_count: 7,
        size_on_disk: 2469606195,
        percent_of_books: 42.86,
      }),
      ..Book::default()
    };
    let readarr_history_item = ReadarrHistoryItem {
      id: 1,
      author_id: 1,
      book_id: 1,
      source_title: "Test Source Title".into(),
      quality: quality.clone(),
      date,
      event_type: ReadarrHistoryEventType::Grabbed,
      ..ReadarrHistoryItem::default()
    };
    let torrent_release = ReadarrRelease {
      guid: "1234".to_owned(),
      protocol: "torrent".to_owned(),
      age: 1,
      title: "Test Torrent Release".into(),
      author_name: Some("Test Author".to_owned()),
      book_title: Some("Test Book".to_owned()),
      indexer: "kickass torrents".to_owned(),
      indexer_id: 1,
      size: 1234,
      rejected: true,
      rejections: Some(vec!["Unknown quality profile".to_owned()]),
      seeders: Some(Number::from(2)),
      leechers: Some(Number::from(1)),
      quality: quality.clone(),
    };
    let usenet_release = ReadarrRelease {
      guid: "5678".to_owned(),
      protocol: "usenet".to_owned(),
      title: "Test Usenet Release".into(),
      indexer: "DrunkenSlug".to_owned(),
      indexer_id: 2,
      seeders: None,
      leechers: None,
      ..torrent_release.clone()
    };
    let blocklist_item = BlocklistItem {
      id: 1,
      author_id: 1,
      book_ids: Some(vec![Number::from(1)]),
      source_title: "Test Source Title".to_owned(),
      quality: quality.clone(),
      date,
      protocol: "usenet".to_owned(),
      indexer: "DrunkenSlug".to_owned(),
      message: "test message".to_owned(),
      author: author.clone(),
    };
    let download_record = DownloadRecord {
      title: "Test Download Title".to_owned(),
      status: DownloadStatus::Downloading,
      id: 1,
      book_id: Some(Number::from(1)),
      author_id: Some(Number::from(1)),
      size: 3543348019f64,
      sizeleft: 1771674009f64,
      output_path: Some("/nfs/books/Test Author".into()),
      indexer: "kickass torrents".to_owned(),
      download_client: Some("transmission".to_owned()),
    };
    let indexer = Indexer {
      id: 1,
      name: Some("Test Indexer".to_owned()),
      implementation: Some("Torznab".to_owned()),
      implementation_name: Some("Torznab".to_owned()),
      config_contract: Some("TorznabSettings".to_owned()),
      supports_rss: true,
      supports_search: true,
      enable_rss: true,
      enable_automatic_search: true,
      enable_interactive_search: true,
      protocol: "torrent".to_owned(),
      priority: 25,
      download_client_id: 0,
      tags: vec![Number::from(1)],
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
    };
    let add_author_search_result = AddAuthorSearchResult {
      foreign_author_id: "test-foreign-id".to_owned(),
      author_name: "Test Author".into(),
      status: AuthorStatus::Continuing,
      overview: Some(
        "Test Author is an American novelist whose science fiction has been translated into more \
         than thirty languages. Born in a small river town, the author spent a decade as a marine \
         biologist before turning to fiction, and the sea remains a constant presence in the work. \
         The debut novel won several awards and introduced the sprawling shared universe that later \
         books would return to again and again. Critics have praised the meticulous world building \
         and the quiet, humane attention to ordinary people caught up in extraordinary events. The \
         author also writes essays on the history of science, teaches an annual writing workshop, \
         and lives with a large and opinionated cat. A new trilogy is currently in progress and is \
         expected to conclude the long running saga that began with the first book. The final \
         volume has already been announced for next year."
          .to_owned(),
      ),
      author_type: Some("Person".to_owned()),
      disambiguation: Some("American novelist".to_owned()),
      ratings: Some(Ratings {
        votes: 1024,
        value: 4.5,
        popularity: Some(9.75),
      }),
      genres: vec!["science fiction".to_owned()],
      ..AddAuthorSearchResult::default()
    };
    let unadded_author_search_result = AddAuthorSearchResult {
      foreign_author_id: "unadded-foreign-id".to_owned(),
      author_name: "Unadded Author".into(),
      status: AuthorStatus::Ended,
      ended: true,
      overview: Some("an author who is not yet in the library".to_owned()),
      author_type: Some("Group".to_owned()),
      genres: vec!["fantasy".to_owned(), "horror".to_owned()],
      ..AddAuthorSearchResult::default()
    };
    let task = ReadarrTask {
      name: "Backup".to_owned(),
      task_name: ReadarrTaskName::Backup,
      interval: 60,
      last_execution: date,
      last_duration: "00:00:00.5111547".to_owned(),
      next_execution: DateTime::from(DateTime::parse_from_rfc3339("2023-05-20T22:29:16Z").unwrap()),
    };
    let log_line = "2025-12-16 16:40:59 UTC|INFO|ImportListSyncService|No list items to process";

    let mut add_author_modal = AddAuthorModal {
      tags: "usenet, testing".into(),
      ..AddAuthorModal::default()
    };
    add_author_modal
      .monitor_list
      .set_items(Vec::from_iter(MonitorType::iter()));
    add_author_modal
      .monitor_new_items_list
      .set_items(Vec::from_iter(NewItemMonitorType::iter()));
    add_author_modal
      .metadata_profile_list
      .set_items(vec!["Standard".to_owned()]);
    add_author_modal
      .quality_profile_list
      .set_items(vec!["Standard".to_owned()]);
    add_author_modal
      .root_folder_list
      .set_items(vec![root_folder.clone()]);

    let mut edit_author_modal = EditAuthorModal {
      monitored: Some(true),
      path: "/nfs/books".into(),
      tags: "alex".into(),
      ..EditAuthorModal::default()
    };
    edit_author_modal
      .monitor_list
      .set_items(NewItemMonitorType::iter().collect());
    edit_author_modal
      .quality_profile_list
      .set_items(vec!["Standard".to_owned()]);
    edit_author_modal
      .metadata_profile_list
      .set_items(vec!["Standard".to_owned()]);

    let mut add_root_folder_modal = AddReadarrRootFolderModal {
      name: "Test Root Folder".into(),
      path: "/nfs/books".into(),
      tags: "test".into(),
      ..AddReadarrRootFolderModal::default()
    };
    add_root_folder_modal
      .monitor_list
      .set_items(Vec::from_iter(MonitorType::iter()));
    add_root_folder_modal
      .monitor_new_items_list
      .set_items(Vec::from_iter(NewItemMonitorType::iter()));
    add_root_folder_modal
      .quality_profile_list
      .set_items(vec!["Standard".to_owned()]);
    add_root_folder_modal
      .metadata_profile_list
      .set_items(vec!["Standard".to_owned()]);

    let edition_details_modal = EditionDetailsModal {
      edition_details: ScrollableText::with_string(
        "Title: Test Edition\n\
         Format: Hardcover\n\
         Language: English\n\
         Publisher: DAW Books\n\
         Page Count: 288\n\
         Release Date: 2023-05-20 00:00:00 UTC\n\
         ISBN13: 9780756404079\n\
         ASIN: B0043RSJ9S\n\
         Ratings: 4.5 (1000 votes)\n\
         Monitored: true\n\
         Overview: original cover of ISBN 075640407X\r\n\
         \r\n\
         My name is Kvothe, pronounced nearly the same as \"quothe.\"\r\n\
         \r\n\
         The Adem call me Maedre: which means The Flame.\r\n"
          .to_owned(),
      ),
    };

    let mut book_details_modal = BookDetailsModal {
      edition_details_modal: Some(edition_details_modal),
      ..BookDetailsModal::default()
    };
    book_details_modal.editions.set_items(vec![Edition {
      id: 1,
      book_id: 1,
      foreign_edition_id: "test-foreign-edition-id".to_owned(),
      monitored: true,
      is_ebook: true,
      title: "Test Edition".to_owned(),
      format: Some("Hardcover".to_owned()),
      publisher: Some("DAW Books".to_owned()),
      isbn13: Some("9780756404079".to_owned()),
      asin: Some("B0043RSJ9S".to_owned()),
      page_count: Some(288),
      release_date: Some(date),
      ..Edition::default()
    }]);
    book_details_modal.editions.search = Some("edition search".into());
    book_details_modal.book_files.set_items(vec![BookFile {
      id: 1,
      author_id: 1,
      book_id: 1,
      path: "/nfs/books/Test Author/Test Book.epub".to_owned(),
      size: 3543348019,
      date_added: date,
      quality: quality.clone(),
      media_info: Some(MediaInfo {
        audio_bit_rate: Some("128 kbps".to_owned()),
        audio_channels: 2,
        audio_codec: Some("MP3".to_owned()),
        audio_bits: Some("16".to_owned()),
        audio_sample_rate: Some("44100".to_owned()),
      }),
      quality_cutoff_not_met: false,
    }]);
    book_details_modal
      .book_history
      .set_items(vec![readarr_history_item.clone()]);
    book_details_modal.book_history.search = Some("book history search".into());
    book_details_modal.book_history.filter = Some("book history filter".into());
    book_details_modal
      .book_history
      .sorting(vec![sort_option!(id)]);
    book_details_modal
      .book_releases
      .set_items(vec![torrent_release.clone(), usenet_release.clone()]);
    book_details_modal
      .book_releases
      .sorting(vec![sort_option!(indexer_id)]);

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

    let mut indexer_test_all_results = StatefulTable::default();
    indexer_test_all_results.set_items(vec![indexer_test_result()]);

    let author_overview_modal = AuthorOverviewModal {
      overview: ScrollableText::with_string(author.overview.clone().unwrap_or_default()),
    };

    let mut readarr_data = ReadarrData {
      add_author_modal: Some(add_author_modal),
      add_root_folder_modal: Some(add_root_folder_modal),
      author_overview_modal: Some(author_overview_modal),
      book_details_modal: Some(book_details_modal),
      delete_files: true,
      disk_space_vec: vec![diskspace()],
      edit_author_modal: Some(edit_author_modal),
      edit_indexer_modal: Some(edit_indexer_modal),
      indexer_settings: Some(indexer_settings()),
      indexer_test_all_results: Some(indexer_test_all_results),
      indexer_test_errors: Some("error".to_string()),
      metadata_profile_map,
      quality_profile_map,
      start_time: date,
      tags_map,
      updates: updates(),
      version: "1.2.3.4".to_owned(),
      ..ReadarrData::default()
    };
    readarr_data
      .author_history
      .set_items(vec![readarr_history_item.clone()]);
    readarr_data.author_history.sorting(vec![sort_option!(id)]);
    readarr_data.author_history.search = Some("author history search".into());
    readarr_data.author_history.filter = Some("author history filter".into());
    readarr_data
      .author_releases
      .set_items(vec![torrent_release, usenet_release]);
    readarr_data
      .author_releases
      .sorting(vec![sort_option!(indexer_id)]);
    readarr_data.authors.set_items(vec![author]);
    readarr_data.authors.sorting(vec![SortOption {
      name: "Name",
      cmp_fn: Some(|a: &Author, b: &Author| a.author_name.text.cmp(&b.author_name.text)),
    }]);
    readarr_data.authors.search = Some("author search".into());
    readarr_data.authors.filter = Some("author filter".into());
    readarr_data.blocklist.set_items(vec![blocklist_item]);
    readarr_data.blocklist.sorting(vec![sort_option!(id)]);
    readarr_data.books.set_items(vec![book]);
    readarr_data.books.search = Some("book search".into());
    readarr_data.downloads.set_items(vec![download_record]);
    readarr_data.history.set_items(vec![readarr_history_item]);
    readarr_data.history.sorting(vec![SortOption {
      name: "Date",
      cmp_fn: Some(|a: &ReadarrHistoryItem, b: &ReadarrHistoryItem| a.date.cmp(&b.date)),
    }]);
    readarr_data.history.search = Some("test search".into());
    readarr_data.history.filter = Some("test filter".into());
    readarr_data.indexers.set_items(vec![indexer]);
    readarr_data.root_folders.set_items(vec![root_folder]);
    readarr_data.queued_events.set_items(vec![queued_event()]);
    readarr_data.add_author_search = Some("Test Author".into());
    let mut add_searched_authors = StatefulTable::default();
    add_searched_authors.set_items(vec![add_author_search_result, unadded_author_search_result]);
    readarr_data.add_searched_authors = Some(add_searched_authors);
    readarr_data.logs.set_items(vec![log_line.into()]);
    readarr_data.log_details.set_items(vec![log_line.into()]);
    readarr_data.tasks.set_items(vec![task]);

    readarr_data
  }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default, EnumIter)]
#[cfg_attr(test, derive(Display, EnumString))]
pub enum ActiveReadarrBlock {
  #[default]
  Authors,
  AddAuthorAlreadyInLibrary,
  AddAuthorConfirmPrompt,
  AddAuthorEmptySearchResults,
  AddAuthorPrompt,
  AddAuthorSearchInput,
  AddAuthorSearchResults,
  AddAuthorSelectMetadataProfile,
  AddAuthorSelectMonitor,
  AddAuthorSelectMonitorNewItems,
  AddAuthorSelectQualityProfile,
  AddAuthorSelectRootFolder,
  AddAuthorTagsInput,
  AddRootFolderConfirmPrompt,
  AddRootFolderNameInput,
  AddRootFolderPathInput,
  AddRootFolderPrompt,
  AddRootFolderSelectMetadataProfile,
  AddRootFolderSelectMonitor,
  AddRootFolderSelectMonitorNewItems,
  AddRootFolderSelectQualityProfile,
  AddRootFolderTagsInput,
  AllIndexerSettingsPrompt,
  AuthorDetails,
  AuthorHistory,
  AuthorHistoryDetails,
  AuthorHistorySortPrompt,
  AuthorOverview,
  AuthorsSortPrompt,
  AutomaticallySearchAuthorPrompt,
  AutomaticallySearchBookPrompt,
  Blocklist,
  BlocklistClearAllItemsPrompt,
  BlocklistItemDetails,
  BlocklistSortPrompt,
  BookDetails,
  BookEditionDetails,
  BookFileInfo,
  BookHistory,
  BookHistoryDetails,
  BookHistorySortPrompt,
  DeleteAuthorConfirmPrompt,
  DeleteAuthorPrompt,
  DeleteAuthorToggleAddListExclusion,
  DeleteAuthorToggleDeleteFile,
  DeleteBlocklistItemPrompt,
  DeleteBookConfirmPrompt,
  DeleteBookFilePrompt,
  DeleteBookPrompt,
  DeleteBookToggleAddListExclusion,
  DeleteBookToggleDeleteFile,
  DeleteDownloadPrompt,
  DeleteIndexerPrompt,
  DeleteRootFolderPrompt,
  Downloads,
  EditAuthorConfirmPrompt,
  EditAuthorPathInput,
  EditAuthorPrompt,
  EditAuthorSelectMetadataProfile,
  EditAuthorSelectMonitorNewItems,
  EditAuthorSelectQualityProfile,
  EditAuthorTagsInput,
  EditAuthorToggleMonitored,
  EditIndexerApiKeyInput,
  EditIndexerConfirmPrompt,
  EditIndexerNameInput,
  EditIndexerPriorityInput,
  EditIndexerPrompt,
  EditIndexerSeedRatioInput,
  EditIndexerTagsInput,
  EditIndexerToggleEnableAutomaticSearch,
  EditIndexerToggleEnableInteractiveSearch,
  EditIndexerToggleEnableRss,
  EditIndexerUrlInput,
  FilterAuthorHistory,
  FilterAuthorHistoryError,
  FilterAuthors,
  FilterAuthorsError,
  FilterBookHistory,
  FilterBookHistoryError,
  FilterHistory,
  FilterHistoryError,
  History,
  HistoryItemDetails,
  HistorySortPrompt,
  Indexers,
  IndexerSettingsConfirmPrompt,
  IndexerSettingsMaximumSizeInput,
  IndexerSettingsMinimumAgeInput,
  IndexerSettingsRetentionInput,
  IndexerSettingsRssSyncIntervalInput,
  ManualAuthorSearch,
  ManualAuthorSearchConfirmPrompt,
  ManualAuthorSearchSortPrompt,
  ManualBookSearch,
  ManualBookSearchConfirmPrompt,
  ManualBookSearchSortPrompt,
  RootFolders,
  SearchAuthorHistory,
  SearchAuthorHistoryError,
  SearchAuthors,
  SearchAuthorsError,
  SearchBookHistory,
  SearchBookHistoryError,
  SearchBooks,
  SearchBooksError,
  SearchEditions,
  SearchEditionsError,
  SearchHistory,
  SearchHistoryError,
  System,
  SystemLogs,
  SystemQueuedEvents,
  SystemTasks,
  SystemTaskStartConfirmPrompt,
  SystemUpdates,
  TestAllIndexers,
  TestIndexer,
  UpdateAllAuthorsPrompt,
  UpdateAndScanAuthorPrompt,
  UpdateDownloadsPrompt,
}

pub static LIBRARY_BLOCKS: [ActiveReadarrBlock; 7] = [
  ActiveReadarrBlock::Authors,
  ActiveReadarrBlock::AuthorsSortPrompt,
  ActiveReadarrBlock::FilterAuthors,
  ActiveReadarrBlock::FilterAuthorsError,
  ActiveReadarrBlock::SearchAuthors,
  ActiveReadarrBlock::SearchAuthorsError,
  ActiveReadarrBlock::UpdateAllAuthorsPrompt,
];

pub static AUTHOR_DETAILS_BLOCKS: [ActiveReadarrBlock; 15] = [
  ActiveReadarrBlock::AuthorDetails,
  ActiveReadarrBlock::AuthorHistory,
  ActiveReadarrBlock::AuthorHistoryDetails,
  ActiveReadarrBlock::AuthorHistorySortPrompt,
  ActiveReadarrBlock::AutomaticallySearchAuthorPrompt,
  ActiveReadarrBlock::FilterAuthorHistory,
  ActiveReadarrBlock::FilterAuthorHistoryError,
  ActiveReadarrBlock::ManualAuthorSearch,
  ActiveReadarrBlock::ManualAuthorSearchConfirmPrompt,
  ActiveReadarrBlock::ManualAuthorSearchSortPrompt,
  ActiveReadarrBlock::SearchBooks,
  ActiveReadarrBlock::SearchBooksError,
  ActiveReadarrBlock::SearchAuthorHistory,
  ActiveReadarrBlock::SearchAuthorHistoryError,
  ActiveReadarrBlock::UpdateAndScanAuthorPrompt,
];

pub static AUTHOR_OVERVIEW_BLOCKS: [ActiveReadarrBlock; 1] = [ActiveReadarrBlock::AuthorOverview];

pub static BOOK_DETAILS_BLOCKS: [ActiveReadarrBlock; 16] = [
  ActiveReadarrBlock::BookDetails,
  ActiveReadarrBlock::BookHistory,
  ActiveReadarrBlock::BookHistoryDetails,
  ActiveReadarrBlock::BookHistorySortPrompt,
  ActiveReadarrBlock::AutomaticallySearchBookPrompt,
  ActiveReadarrBlock::DeleteBookFilePrompt,
  ActiveReadarrBlock::FilterBookHistory,
  ActiveReadarrBlock::FilterBookHistoryError,
  ActiveReadarrBlock::BookFileInfo,
  ActiveReadarrBlock::ManualBookSearch,
  ActiveReadarrBlock::ManualBookSearchConfirmPrompt,
  ActiveReadarrBlock::ManualBookSearchSortPrompt,
  ActiveReadarrBlock::SearchBookHistory,
  ActiveReadarrBlock::SearchBookHistoryError,
  ActiveReadarrBlock::SearchEditions,
  ActiveReadarrBlock::SearchEditionsError,
];

pub static EDITION_DETAILS_BLOCKS: [ActiveReadarrBlock; 1] =
  [ActiveReadarrBlock::BookEditionDetails];

pub static ADD_AUTHOR_BLOCKS: [ActiveReadarrBlock; 12] = [
  ActiveReadarrBlock::AddAuthorAlreadyInLibrary,
  ActiveReadarrBlock::AddAuthorConfirmPrompt,
  ActiveReadarrBlock::AddAuthorEmptySearchResults,
  ActiveReadarrBlock::AddAuthorPrompt,
  ActiveReadarrBlock::AddAuthorSearchInput,
  ActiveReadarrBlock::AddAuthorSearchResults,
  ActiveReadarrBlock::AddAuthorSelectMetadataProfile,
  ActiveReadarrBlock::AddAuthorSelectMonitor,
  ActiveReadarrBlock::AddAuthorSelectMonitorNewItems,
  ActiveReadarrBlock::AddAuthorSelectQualityProfile,
  ActiveReadarrBlock::AddAuthorSelectRootFolder,
  ActiveReadarrBlock::AddAuthorTagsInput,
];

pub const ADD_AUTHOR_SELECTION_BLOCKS: &[&[ActiveReadarrBlock]] = &[
  &[ActiveReadarrBlock::AddAuthorSelectRootFolder],
  &[ActiveReadarrBlock::AddAuthorSelectMonitor],
  &[ActiveReadarrBlock::AddAuthorSelectMonitorNewItems],
  &[ActiveReadarrBlock::AddAuthorSelectQualityProfile],
  &[ActiveReadarrBlock::AddAuthorSelectMetadataProfile],
  &[ActiveReadarrBlock::AddAuthorTagsInput],
  &[ActiveReadarrBlock::AddAuthorConfirmPrompt],
];

pub static DELETE_AUTHOR_BLOCKS: [ActiveReadarrBlock; 4] = [
  ActiveReadarrBlock::DeleteAuthorPrompt,
  ActiveReadarrBlock::DeleteAuthorConfirmPrompt,
  ActiveReadarrBlock::DeleteAuthorToggleDeleteFile,
  ActiveReadarrBlock::DeleteAuthorToggleAddListExclusion,
];

pub const DELETE_AUTHOR_SELECTION_BLOCKS: &[&[ActiveReadarrBlock]] = &[
  &[ActiveReadarrBlock::DeleteAuthorToggleDeleteFile],
  &[ActiveReadarrBlock::DeleteAuthorToggleAddListExclusion],
  &[ActiveReadarrBlock::DeleteAuthorConfirmPrompt],
];

pub static DELETE_BOOK_BLOCKS: [ActiveReadarrBlock; 4] = [
  ActiveReadarrBlock::DeleteBookPrompt,
  ActiveReadarrBlock::DeleteBookConfirmPrompt,
  ActiveReadarrBlock::DeleteBookToggleDeleteFile,
  ActiveReadarrBlock::DeleteBookToggleAddListExclusion,
];

pub const DELETE_BOOK_SELECTION_BLOCKS: &[&[ActiveReadarrBlock]] = &[
  &[ActiveReadarrBlock::DeleteBookToggleDeleteFile],
  &[ActiveReadarrBlock::DeleteBookToggleAddListExclusion],
  &[ActiveReadarrBlock::DeleteBookConfirmPrompt],
];

pub static EDIT_AUTHOR_BLOCKS: [ActiveReadarrBlock; 8] = [
  ActiveReadarrBlock::EditAuthorPrompt,
  ActiveReadarrBlock::EditAuthorConfirmPrompt,
  ActiveReadarrBlock::EditAuthorPathInput,
  ActiveReadarrBlock::EditAuthorSelectMetadataProfile,
  ActiveReadarrBlock::EditAuthorSelectMonitorNewItems,
  ActiveReadarrBlock::EditAuthorSelectQualityProfile,
  ActiveReadarrBlock::EditAuthorTagsInput,
  ActiveReadarrBlock::EditAuthorToggleMonitored,
];

pub const EDIT_AUTHOR_SELECTION_BLOCKS: &[&[ActiveReadarrBlock]] = &[
  &[ActiveReadarrBlock::EditAuthorToggleMonitored],
  &[ActiveReadarrBlock::EditAuthorSelectMonitorNewItems],
  &[ActiveReadarrBlock::EditAuthorSelectQualityProfile],
  &[ActiveReadarrBlock::EditAuthorSelectMetadataProfile],
  &[ActiveReadarrBlock::EditAuthorPathInput],
  &[ActiveReadarrBlock::EditAuthorTagsInput],
  &[ActiveReadarrBlock::EditAuthorConfirmPrompt],
];

pub static ADD_ROOT_FOLDER_BLOCKS: [ActiveReadarrBlock; 9] = [
  ActiveReadarrBlock::AddRootFolderPrompt,
  ActiveReadarrBlock::AddRootFolderConfirmPrompt,
  ActiveReadarrBlock::AddRootFolderNameInput,
  ActiveReadarrBlock::AddRootFolderPathInput,
  ActiveReadarrBlock::AddRootFolderSelectMonitor,
  ActiveReadarrBlock::AddRootFolderSelectMonitorNewItems,
  ActiveReadarrBlock::AddRootFolderSelectQualityProfile,
  ActiveReadarrBlock::AddRootFolderSelectMetadataProfile,
  ActiveReadarrBlock::AddRootFolderTagsInput,
];

pub const ADD_ROOT_FOLDER_SELECTION_BLOCKS: &[&[ActiveReadarrBlock]] = &[
  &[ActiveReadarrBlock::AddRootFolderNameInput],
  &[ActiveReadarrBlock::AddRootFolderPathInput],
  &[ActiveReadarrBlock::AddRootFolderSelectMonitor],
  &[ActiveReadarrBlock::AddRootFolderSelectMonitorNewItems],
  &[ActiveReadarrBlock::AddRootFolderSelectQualityProfile],
  &[ActiveReadarrBlock::AddRootFolderSelectMetadataProfile],
  &[ActiveReadarrBlock::AddRootFolderTagsInput],
  &[ActiveReadarrBlock::AddRootFolderConfirmPrompt],
];

pub static ROOT_FOLDERS_BLOCKS: [ActiveReadarrBlock; 2] = [
  ActiveReadarrBlock::RootFolders,
  ActiveReadarrBlock::DeleteRootFolderPrompt,
];

pub static BLOCKLIST_BLOCKS: [ActiveReadarrBlock; 5] = [
  ActiveReadarrBlock::Blocklist,
  ActiveReadarrBlock::BlocklistItemDetails,
  ActiveReadarrBlock::DeleteBlocklistItemPrompt,
  ActiveReadarrBlock::BlocklistClearAllItemsPrompt,
  ActiveReadarrBlock::BlocklistSortPrompt,
];

pub static DOWNLOADS_BLOCKS: [ActiveReadarrBlock; 3] = [
  ActiveReadarrBlock::Downloads,
  ActiveReadarrBlock::DeleteDownloadPrompt,
  ActiveReadarrBlock::UpdateDownloadsPrompt,
];

pub static HISTORY_BLOCKS: [ActiveReadarrBlock; 7] = [
  ActiveReadarrBlock::History,
  ActiveReadarrBlock::HistoryItemDetails,
  ActiveReadarrBlock::HistorySortPrompt,
  ActiveReadarrBlock::SearchHistory,
  ActiveReadarrBlock::SearchHistoryError,
  ActiveReadarrBlock::FilterHistory,
  ActiveReadarrBlock::FilterHistoryError,
];

pub static EDIT_INDEXER_BLOCKS: [ActiveReadarrBlock; 11] = [
  ActiveReadarrBlock::EditIndexerPrompt,
  ActiveReadarrBlock::EditIndexerConfirmPrompt,
  ActiveReadarrBlock::EditIndexerApiKeyInput,
  ActiveReadarrBlock::EditIndexerNameInput,
  ActiveReadarrBlock::EditIndexerSeedRatioInput,
  ActiveReadarrBlock::EditIndexerToggleEnableRss,
  ActiveReadarrBlock::EditIndexerToggleEnableAutomaticSearch,
  ActiveReadarrBlock::EditIndexerToggleEnableInteractiveSearch,
  ActiveReadarrBlock::EditIndexerPriorityInput,
  ActiveReadarrBlock::EditIndexerUrlInput,
  ActiveReadarrBlock::EditIndexerTagsInput,
];

pub const EDIT_INDEXER_TORRENT_SELECTION_BLOCKS: &[&[ActiveReadarrBlock]] = &[
  &[
    ActiveReadarrBlock::EditIndexerNameInput,
    ActiveReadarrBlock::EditIndexerUrlInput,
  ],
  &[
    ActiveReadarrBlock::EditIndexerToggleEnableRss,
    ActiveReadarrBlock::EditIndexerApiKeyInput,
  ],
  &[
    ActiveReadarrBlock::EditIndexerToggleEnableAutomaticSearch,
    ActiveReadarrBlock::EditIndexerSeedRatioInput,
  ],
  &[
    ActiveReadarrBlock::EditIndexerToggleEnableInteractiveSearch,
    ActiveReadarrBlock::EditIndexerTagsInput,
  ],
  &[
    ActiveReadarrBlock::EditIndexerPriorityInput,
    ActiveReadarrBlock::EditIndexerConfirmPrompt,
  ],
  &[
    ActiveReadarrBlock::EditIndexerConfirmPrompt,
    ActiveReadarrBlock::EditIndexerConfirmPrompt,
  ],
];

pub const EDIT_INDEXER_NZB_SELECTION_BLOCKS: &[&[ActiveReadarrBlock]] = &[
  &[
    ActiveReadarrBlock::EditIndexerNameInput,
    ActiveReadarrBlock::EditIndexerUrlInput,
  ],
  &[
    ActiveReadarrBlock::EditIndexerToggleEnableRss,
    ActiveReadarrBlock::EditIndexerApiKeyInput,
  ],
  &[
    ActiveReadarrBlock::EditIndexerToggleEnableAutomaticSearch,
    ActiveReadarrBlock::EditIndexerTagsInput,
  ],
  &[
    ActiveReadarrBlock::EditIndexerToggleEnableInteractiveSearch,
    ActiveReadarrBlock::EditIndexerPriorityInput,
  ],
  &[
    ActiveReadarrBlock::EditIndexerConfirmPrompt,
    ActiveReadarrBlock::EditIndexerConfirmPrompt,
  ],
];

pub static INDEXER_SETTINGS_BLOCKS: [ActiveReadarrBlock; 6] = [
  ActiveReadarrBlock::AllIndexerSettingsPrompt,
  ActiveReadarrBlock::IndexerSettingsConfirmPrompt,
  ActiveReadarrBlock::IndexerSettingsMaximumSizeInput,
  ActiveReadarrBlock::IndexerSettingsMinimumAgeInput,
  ActiveReadarrBlock::IndexerSettingsRetentionInput,
  ActiveReadarrBlock::IndexerSettingsRssSyncIntervalInput,
];

pub const INDEXER_SETTINGS_SELECTION_BLOCKS: &[&[ActiveReadarrBlock]] = &[
  &[ActiveReadarrBlock::IndexerSettingsMinimumAgeInput],
  &[ActiveReadarrBlock::IndexerSettingsRetentionInput],
  &[ActiveReadarrBlock::IndexerSettingsMaximumSizeInput],
  &[ActiveReadarrBlock::IndexerSettingsRssSyncIntervalInput],
  &[ActiveReadarrBlock::IndexerSettingsConfirmPrompt],
];

pub static INDEXERS_BLOCKS: [ActiveReadarrBlock; 3] = [
  ActiveReadarrBlock::Indexers,
  ActiveReadarrBlock::DeleteIndexerPrompt,
  ActiveReadarrBlock::TestIndexer,
];

pub static SYSTEM_DETAILS_BLOCKS: [ActiveReadarrBlock; 5] = [
  ActiveReadarrBlock::SystemLogs,
  ActiveReadarrBlock::SystemQueuedEvents,
  ActiveReadarrBlock::SystemTasks,
  ActiveReadarrBlock::SystemTaskStartConfirmPrompt,
  ActiveReadarrBlock::SystemUpdates,
];

impl From<ActiveReadarrBlock> for Route {
  fn from(active_readarr_block: ActiveReadarrBlock) -> Route {
    Route::Readarr(active_readarr_block, None)
  }
}

impl From<(ActiveReadarrBlock, Option<ActiveReadarrBlock>)> for Route {
  fn from(value: (ActiveReadarrBlock, Option<ActiveReadarrBlock>)) -> Route {
    Route::Readarr(value.0, value.1)
  }
}
