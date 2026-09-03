use super::readarr_data::{ActiveReadarrBlock, ReadarrData};
use crate::app::readarr::readarr_context_clues::{
  BOOK_DETAILS_CONTEXT_CLUES, BOOK_FILE_CONTEXT_CLUES, BOOK_HISTORY_CONTEXT_CLUES,
  MANUAL_BOOK_SEARCH_CONTEXT_CLUES,
};
use crate::models::readarr_models::{BookFile, Edition, ReadarrHistoryItem, ReadarrRelease};
use crate::models::servarr_data::modals::EditIndexerModal;
use crate::models::servarr_models::Indexer;
use crate::models::stateful_table::StatefulTable;
use crate::models::{
  HorizontallyScrollableText, ScrollableText, TabRoute, TabState,
  readarr_models::{MonitorType, NewItemMonitorType},
  servarr_models::RootFolder,
  stateful_list::StatefulList,
};
use strum::IntoEnumIterator;

#[cfg(test)]
#[path = "modals_tests.rs"]
mod modals_tests;

#[derive(Default)]
#[cfg_attr(test, derive(Debug))]
pub struct AddAuthorModal {
  pub root_folder_list: StatefulList<RootFolder>,
  pub monitor_list: StatefulList<MonitorType>,
  pub monitor_new_items_list: StatefulList<NewItemMonitorType>,
  pub quality_profile_list: StatefulList<String>,
  pub metadata_profile_list: StatefulList<String>,
  pub tags: HorizontallyScrollableText,
}

impl From<&ReadarrData<'_>> for AddAuthorModal {
  fn from(readarr_data: &ReadarrData<'_>) -> AddAuthorModal {
    let mut add_author_modal = AddAuthorModal::default();
    add_author_modal
      .monitor_list
      .set_items(Vec::from_iter(MonitorType::iter()));
    add_author_modal
      .monitor_new_items_list
      .set_items(Vec::from_iter(NewItemMonitorType::iter()));
    add_author_modal
      .quality_profile_list
      .set_items(readarr_data.sorted_quality_profile_names());
    add_author_modal
      .metadata_profile_list
      .set_items(readarr_data.sorted_metadata_profile_names());
    add_author_modal
      .root_folder_list
      .set_items(readarr_data.root_folders.items.to_vec());

    add_author_modal
  }
}

#[derive(Default)]
#[cfg_attr(test, derive(Debug))]
pub struct EditAuthorModal {
  pub monitor_list: StatefulList<NewItemMonitorType>,
  pub quality_profile_list: StatefulList<String>,
  pub metadata_profile_list: StatefulList<String>,
  pub monitored: Option<bool>,
  pub path: HorizontallyScrollableText,
  pub tags: HorizontallyScrollableText,
}

impl From<&ReadarrData<'_>> for EditAuthorModal {
  fn from(readarr_data: &ReadarrData<'_>) -> EditAuthorModal {
    let mut edit_author_modal = EditAuthorModal::default();
    let author = readarr_data.authors.current_selection();

    edit_author_modal
      .monitor_list
      .set_items(Vec::from_iter(NewItemMonitorType::iter()));
    edit_author_modal.path = author.path.clone().into();
    edit_author_modal.tags = readarr_data.tag_ids_to_display(&author.tags).into();
    edit_author_modal.monitored = Some(author.monitored);

    let monitor_index = edit_author_modal
      .monitor_list
      .items
      .iter()
      .position(|m| *m == author.monitor_new_items);
    edit_author_modal.monitor_list.state.select(monitor_index);

    edit_author_modal
      .quality_profile_list
      .set_items(readarr_data.sorted_quality_profile_names());
    let quality_profile_name = readarr_data
      .quality_profile_map
      .get_by_left(&author.quality_profile_id)
      .unwrap();
    let quality_profile_index = edit_author_modal
      .quality_profile_list
      .items
      .iter()
      .position(|profile| profile == quality_profile_name);
    edit_author_modal
      .quality_profile_list
      .state
      .select(quality_profile_index);

    edit_author_modal
      .metadata_profile_list
      .set_items(readarr_data.sorted_metadata_profile_names());
    let metadata_profile_name = readarr_data
      .metadata_profile_map
      .get_by_left(&author.metadata_profile_id)
      .unwrap();
    let metadata_profile_index = edit_author_modal
      .metadata_profile_list
      .items
      .iter()
      .position(|profile| profile == metadata_profile_name);
    edit_author_modal
      .metadata_profile_list
      .state
      .select(metadata_profile_index);

    edit_author_modal
  }
}

impl From<&ReadarrData<'_>> for EditIndexerModal {
  fn from(readarr_data: &ReadarrData<'_>) -> EditIndexerModal {
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
    } = readarr_data.indexers.current_selection();
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

    edit_indexer_modal.tags = readarr_data.tag_ids_to_display(tags).into();

    edit_indexer_modal
  }
}

#[derive(Default)]
#[cfg_attr(test, derive(Debug))]
pub struct AddReadarrRootFolderModal {
  pub name: HorizontallyScrollableText,
  pub path: HorizontallyScrollableText,
  pub monitor_list: StatefulList<MonitorType>,
  pub monitor_new_items_list: StatefulList<NewItemMonitorType>,
  pub quality_profile_list: StatefulList<String>,
  pub metadata_profile_list: StatefulList<String>,
  pub tags: HorizontallyScrollableText,
}

impl From<&ReadarrData<'_>> for AddReadarrRootFolderModal {
  fn from(readarr_data: &ReadarrData<'_>) -> AddReadarrRootFolderModal {
    let mut add_root_folder_modal = AddReadarrRootFolderModal::default();
    add_root_folder_modal
      .monitor_list
      .set_items(Vec::from_iter(MonitorType::iter()));
    add_root_folder_modal
      .monitor_new_items_list
      .set_items(Vec::from_iter(NewItemMonitorType::iter()));
    add_root_folder_modal
      .quality_profile_list
      .set_items(readarr_data.sorted_quality_profile_names());
    add_root_folder_modal
      .metadata_profile_list
      .set_items(readarr_data.sorted_metadata_profile_names());

    add_root_folder_modal
  }
}

#[cfg_attr(test, derive(Debug))]
pub struct BookDetailsModal {
  pub editions: StatefulTable<Edition>,
  pub book_files: StatefulTable<BookFile>,
  pub edition_details_modal: Option<EditionDetailsModal>,
  pub book_history: StatefulTable<ReadarrHistoryItem>,
  pub book_releases: StatefulTable<ReadarrRelease>,
  pub book_details_tabs: TabState,
}

impl Default for BookDetailsModal {
  fn default() -> BookDetailsModal {
    BookDetailsModal {
      editions: StatefulTable::default(),
      edition_details_modal: None,
      book_files: StatefulTable::default(),
      book_releases: StatefulTable::default(),
      book_history: StatefulTable::default(),
      book_details_tabs: TabState::new(vec![
        TabRoute {
          title: "Editions".to_string(),
          route: ActiveReadarrBlock::BookDetails.into(),
          contextual_help: Some(&BOOK_DETAILS_CONTEXT_CLUES),
          config: None,
        },
        TabRoute {
          title: "History".to_string(),
          route: ActiveReadarrBlock::BookHistory.into(),
          contextual_help: Some(&BOOK_HISTORY_CONTEXT_CLUES),
          config: None,
        },
        TabRoute {
          title: "File".to_string(),
          route: ActiveReadarrBlock::BookFileInfo.into(),
          contextual_help: Some(&BOOK_FILE_CONTEXT_CLUES),
          config: None,
        },
        TabRoute {
          title: "Manual Search".to_string(),
          route: ActiveReadarrBlock::ManualBookSearch.into(),
          contextual_help: Some(&MANUAL_BOOK_SEARCH_CONTEXT_CLUES),
          config: None,
        },
      ]),
    }
  }
}

#[derive(Default)]
#[cfg_attr(test, derive(Debug))]
pub struct EditionDetailsModal {
  pub edition_details: ScrollableText,
}
