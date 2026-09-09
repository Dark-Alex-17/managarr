#[cfg(test)]
mod tests {
  use crate::app::readarr::readarr_context_clues::{
    BOOK_DETAILS_CONTEXT_CLUES, BOOK_FILE_CONTEXT_CLUES, BOOK_HISTORY_CONTEXT_CLUES,
    MANUAL_BOOK_SEARCH_CONTEXT_CLUES,
  };
  use crate::models::readarr_models::{Author, MonitorType, NewItemMonitorType};
  use crate::models::servarr_data::modals::EditIndexerModal;
  use crate::models::servarr_data::readarr::modals::{
    AddAuthorModal, AddReadarrRootFolderModal, AuthorOverviewModal, BookDetailsModal,
    EditAuthorModal, EditionDetailsModal,
  };
  use crate::models::servarr_data::readarr::readarr_data::{ActiveReadarrBlock, ReadarrData};
  use crate::models::servarr_models::{Indexer, IndexerField, RootFolder};
  use bimap::BiMap;
  use pretty_assertions::{assert_eq, assert_str_eq};
  use rstest::rstest;
  use serde_json::{Number, Value};

  #[test]
  fn test_add_author_modal_from_readarr_data() {
    let mut readarr_data = ReadarrData {
      quality_profile_map: BiMap::from_iter([
        (2i64, "Lossless".to_owned()),
        (1i64, "Standard".to_owned()),
      ]),
      metadata_profile_map: BiMap::from_iter([
        (2i64, "None".to_owned()),
        (1i64, "Standard".to_owned()),
      ]),
      ..ReadarrData::default()
    };
    let root_folder_1 = RootFolder {
      id: 1,
      path: "/nfs".to_owned(),
      accessible: true,
      free_space: 219902325555200,
      unmapped_folders: None,
    };
    readarr_data.root_folders.set_items(vec![
      root_folder_1.clone(),
      RootFolder {
        id: 2,
        path: "/nfs2".to_owned(),
        accessible: true,
        free_space: 21990232555520,
        unmapped_folders: None,
      },
    ]);

    let add_author_modal = AddAuthorModal::from(&readarr_data);

    assert_eq!(
      *add_author_modal.monitor_list.current_selection(),
      MonitorType::default()
    );
    assert_eq!(
      *add_author_modal.monitor_new_items_list.current_selection(),
      NewItemMonitorType::default()
    );
    assert_str_eq!(
      add_author_modal.quality_profile_list.current_selection(),
      "Standard"
    );
    assert_str_eq!(
      add_author_modal.metadata_profile_list.current_selection(),
      "Standard"
    );
    assert_eq!(
      add_author_modal.root_folder_list.current_selection(),
      &root_folder_1
    );
    assert_is_empty!(add_author_modal.tags.text);
  }

  #[test]
  fn test_edit_author_modal_from_readarr_data() {
    let mut readarr_data = ReadarrData {
      quality_profile_map: BiMap::from_iter([
        (1i64, "HD - 1080p".to_owned()),
        (2i64, "Any".to_owned()),
      ]),
      metadata_profile_map: BiMap::from_iter([
        (1i64, "Standard".to_owned()),
        (2i64, "None".to_owned()),
      ]),
      tags_map: BiMap::from_iter([(1i64, "usenet".to_owned())]),
      ..ReadarrData::default()
    };
    let author = Author {
      id: 1,
      monitored: true,
      monitor_new_items: NewItemMonitorType::New,
      quality_profile_id: 1,
      metadata_profile_id: 1,
      path: "/nfs/books/test_author".to_owned(),
      tags: vec![Number::from(1)],
      ..Author::default()
    };
    readarr_data.authors.set_items(vec![author]);

    let edit_author_modal = EditAuthorModal::from(&readarr_data);

    assert_some_eq_x!(&edit_author_modal.monitored, &true);
    assert_eq!(
      *edit_author_modal.monitor_list.current_selection(),
      NewItemMonitorType::New
    );
    assert_str_eq!(
      edit_author_modal.quality_profile_list.current_selection(),
      "HD - 1080p"
    );
    assert_str_eq!(
      edit_author_modal.metadata_profile_list.current_selection(),
      "Standard"
    );
    assert_str_eq!(edit_author_modal.path.text, "/nfs/books/test_author");
    assert_str_eq!(edit_author_modal.tags.text, "usenet");
  }

  #[rstest]
  fn test_edit_indexer_modal_from_readarr_data(#[values(true, false)] seed_ratio_present: bool) {
    let mut readarr_data = ReadarrData {
      tags_map: BiMap::from_iter([(1, "usenet".to_owned()), (2, "test".to_owned())]),
      ..ReadarrData::default()
    };
    let mut fields = vec![
      IndexerField {
        name: Some("baseUrl".to_owned()),
        value: Some(Value::String("https://test.com".to_owned())),
      },
      IndexerField {
        name: Some("apiKey".to_owned()),
        value: Some(Value::String("1234".to_owned())),
      },
    ];

    if seed_ratio_present {
      fields.push(IndexerField {
        name: Some("seedCriteria.seedRatio".to_owned()),
        value: Some(Value::from(1.2f64)),
      });
    }

    let indexer = Indexer {
      name: Some("Test".to_owned()),
      enable_rss: true,
      enable_automatic_search: true,
      enable_interactive_search: true,
      tags: vec![Number::from(1), Number::from(2)],
      fields: Some(fields),
      priority: 1,
      ..Indexer::default()
    };
    readarr_data.indexers.set_items(vec![indexer]);

    let edit_indexer_modal = EditIndexerModal::from(&readarr_data);

    assert_str_eq!(edit_indexer_modal.name.text, "Test");
    assert_some_eq_x!(&edit_indexer_modal.enable_rss, &true);
    assert_some_eq_x!(&edit_indexer_modal.enable_automatic_search, &true);
    assert_some_eq_x!(&edit_indexer_modal.enable_interactive_search, &true);
    assert_eq!(edit_indexer_modal.priority, 1);
    assert_str_eq!(edit_indexer_modal.url.text, "https://test.com");
    assert_str_eq!(edit_indexer_modal.api_key.text, "1234");

    if seed_ratio_present {
      assert_str_eq!(edit_indexer_modal.seed_ratio.text, "1.2");
    } else {
      assert_is_empty!(edit_indexer_modal.seed_ratio.text);
    }
  }

  #[test]
  fn test_edit_indexer_modal_from_readarr_data_seed_ratio_value_is_none() {
    let mut readarr_data = ReadarrData {
      tags_map: BiMap::from_iter([(1, "usenet".to_owned()), (2, "test".to_owned())]),
      ..ReadarrData::default()
    };
    let fields = vec![
      IndexerField {
        name: Some("baseUrl".to_owned()),
        value: Some(Value::String("https://test.com".to_owned())),
      },
      IndexerField {
        name: Some("apiKey".to_owned()),
        value: Some(Value::String("1234".to_owned())),
      },
      IndexerField {
        name: Some("seedCriteria.seedRatio".to_owned()),
        value: None,
      },
    ];

    let indexer = Indexer {
      name: Some("Test".to_owned()),
      enable_rss: true,
      enable_automatic_search: true,
      enable_interactive_search: true,
      tags: vec![Number::from(1), Number::from(2)],
      fields: Some(fields),
      priority: 1,
      ..Indexer::default()
    };
    readarr_data.indexers.set_items(vec![indexer]);

    let edit_indexer_modal = EditIndexerModal::from(&readarr_data);

    assert_str_eq!(edit_indexer_modal.name.text, "Test");
    assert_some_eq_x!(&edit_indexer_modal.enable_rss, &true);
    assert_some_eq_x!(&edit_indexer_modal.enable_automatic_search, &true);
    assert_some_eq_x!(&edit_indexer_modal.enable_interactive_search, &true);
    assert_eq!(edit_indexer_modal.priority, 1);
    assert_str_eq!(edit_indexer_modal.url.text, "https://test.com");
    assert_str_eq!(edit_indexer_modal.api_key.text, "1234");
    assert_is_empty!(edit_indexer_modal.seed_ratio.text);
  }

  #[test]
  fn test_add_readarr_root_folder_modal_from_readarr_data() {
    let readarr_data = ReadarrData {
      quality_profile_map: BiMap::from_iter([
        (2i64, "Lossless".to_owned()),
        (1i64, "Standard".to_owned()),
      ]),
      metadata_profile_map: BiMap::from_iter([
        (2i64, "None".to_owned()),
        (1i64, "Standard".to_owned()),
      ]),
      ..ReadarrData::default()
    };

    let add_root_folder_modal = AddReadarrRootFolderModal::from(&readarr_data);

    assert_eq!(
      *add_root_folder_modal.monitor_list.current_selection(),
      MonitorType::default()
    );
    assert_eq!(
      *add_root_folder_modal
        .monitor_new_items_list
        .current_selection(),
      NewItemMonitorType::default()
    );
    assert_str_eq!(
      add_root_folder_modal
        .quality_profile_list
        .current_selection(),
      "Standard"
    );
    assert_str_eq!(
      add_root_folder_modal
        .metadata_profile_list
        .current_selection(),
      "Standard"
    );
    assert_is_empty!(add_root_folder_modal.name.text);
    assert_is_empty!(add_root_folder_modal.path.text);
    assert_is_empty!(add_root_folder_modal.tags.text);
  }

  #[test]
  fn test_book_details_modal_default() {
    let book_details_modal = BookDetailsModal::default();

    assert_is_empty!(book_details_modal.editions);
    assert_none!(book_details_modal.edition_details_modal);
    assert_is_empty!(book_details_modal.book_files);
    assert_is_empty!(book_details_modal.book_releases);
    assert_is_empty!(book_details_modal.book_history);

    assert_eq!(book_details_modal.book_details_tabs.tabs.len(), 4);

    assert_str_eq!(
      book_details_modal.book_details_tabs.tabs[0].title,
      "Editions"
    );
    assert_eq!(
      book_details_modal.book_details_tabs.tabs[0].route,
      ActiveReadarrBlock::BookDetails.into()
    );
    assert_some_eq_x!(
      &book_details_modal.book_details_tabs.tabs[0].contextual_help,
      &BOOK_DETAILS_CONTEXT_CLUES
    );
    assert_none!(book_details_modal.book_details_tabs.tabs[0].config);

    assert_str_eq!(
      book_details_modal.book_details_tabs.tabs[1].title,
      "History"
    );
    assert_eq!(
      book_details_modal.book_details_tabs.tabs[1].route,
      ActiveReadarrBlock::BookHistory.into()
    );
    assert_some_eq_x!(
      &book_details_modal.book_details_tabs.tabs[1].contextual_help,
      &BOOK_HISTORY_CONTEXT_CLUES
    );
    assert_none!(book_details_modal.book_details_tabs.tabs[1].config);

    assert_str_eq!(book_details_modal.book_details_tabs.tabs[2].title, "File");
    assert_eq!(
      book_details_modal.book_details_tabs.tabs[2].route,
      ActiveReadarrBlock::BookFileInfo.into()
    );
    assert_some_eq_x!(
      &book_details_modal.book_details_tabs.tabs[2].contextual_help,
      &BOOK_FILE_CONTEXT_CLUES
    );
    assert_none!(book_details_modal.book_details_tabs.tabs[2].config);

    assert_str_eq!(
      book_details_modal.book_details_tabs.tabs[3].title,
      "Manual Search"
    );
    assert_eq!(
      book_details_modal.book_details_tabs.tabs[3].route,
      ActiveReadarrBlock::ManualBookSearch.into()
    );
    assert_some_eq_x!(
      &book_details_modal.book_details_tabs.tabs[3].contextual_help,
      &MANUAL_BOOK_SEARCH_CONTEXT_CLUES
    );
    assert_none!(book_details_modal.book_details_tabs.tabs[3].config);
  }

  #[test]
  fn test_edition_details_modal_default() {
    let edition_details_modal = EditionDetailsModal::default();

    assert_is_empty!(edition_details_modal.edition_details);
  }

  #[test]
  fn test_author_overview_modal_default() {
    let author_overview_modal = AuthorOverviewModal::default();

    assert_is_empty!(author_overview_modal.overview);
  }
}
