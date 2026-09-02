#[cfg(test)]
mod tests {
  use crate::app::App;
  use crate::models::readarr_models::{Author, Book, ReadarrRelease};
  use crate::models::servarr_data::readarr::modals::BookDetailsModal;
  use crate::models::servarr_data::readarr::readarr_data::ActiveReadarrBlock;
  use crate::models::servarr_models::Indexer;
  use crate::network::NetworkEvent;
  use crate::network::readarr_network::ReadarrEvent;
  use pretty_assertions::{assert_eq, assert_str_eq};
  use tokio::sync::mpsc;

  #[tokio::test]
  async fn test_dispatch_by_readarr_block_authors() {
    let (tx, mut rx) = mpsc::channel::<NetworkEvent>(500);
    let mut app = App::test_default();
    app.data.readarr_data.prompt_confirm = true;
    app.network_tx = Some(tx);

    app
      .dispatch_by_readarr_block(&ActiveReadarrBlock::Authors)
      .await;

    assert!(app.is_loading);
    assert_eq!(
      rx.recv().await.unwrap(),
      ReadarrEvent::GetQualityProfiles.into()
    );
    assert_eq!(
      rx.recv().await.unwrap(),
      ReadarrEvent::GetMetadataProfiles.into()
    );
    assert_eq!(rx.recv().await.unwrap(), ReadarrEvent::GetTags.into());
    assert_eq!(rx.recv().await.unwrap(), ReadarrEvent::ListAuthors.into());
    assert!(!app.data.readarr_data.prompt_confirm);
    assert_eq!(app.tick_count, 0);
  }

  #[tokio::test]
  async fn test_dispatch_by_readarr_block_author_details() {
    let (tx, mut rx) = mpsc::channel::<NetworkEvent>(500);
    let mut app = App::test_default();
    app.data.readarr_data.prompt_confirm = true;
    app.data.readarr_data.authors.set_items(vec![Author {
      id: 1,
      ..Author::default()
    }]);
    app.network_tx = Some(tx);

    app
      .dispatch_by_readarr_block(&ActiveReadarrBlock::AuthorDetails)
      .await;

    assert!(app.is_loading);
    assert_eq!(rx.recv().await.unwrap(), ReadarrEvent::GetBooks(1).into());
    assert!(!app.data.readarr_data.prompt_confirm);
    assert_eq!(app.tick_count, 0);
  }

  #[tokio::test]
  async fn test_dispatch_by_blocklist_block() {
    let (tx, mut rx) = mpsc::channel::<NetworkEvent>(500);
    let mut app = App::test_default();
    app.data.readarr_data.prompt_confirm = true;
    app.network_tx = Some(tx);

    app
      .dispatch_by_readarr_block(&ActiveReadarrBlock::Blocklist)
      .await;

    assert!(app.is_loading);
    assert_eq!(rx.recv().await.unwrap(), ReadarrEvent::GetBlocklist.into());
    assert!(!app.data.readarr_data.prompt_confirm);
    assert_eq!(app.tick_count, 0);
  }

  #[tokio::test]
  async fn test_dispatch_by_author_history_block() {
    let (tx, mut rx) = mpsc::channel::<NetworkEvent>(500);
    let mut app = App::test_default();
    app.data.readarr_data.prompt_confirm = true;
    app.network_tx = Some(tx);
    app.data.readarr_data.authors.set_items(vec![Author {
      id: 1,
      ..Author::default()
    }]);

    app
      .dispatch_by_readarr_block(&ActiveReadarrBlock::AuthorHistory)
      .await;

    assert!(app.is_loading);
    assert_eq!(
      rx.recv().await.unwrap(),
      ReadarrEvent::GetAuthorHistory(1).into()
    );
    assert!(!app.data.readarr_data.prompt_confirm);
    assert_eq!(app.tick_count, 0);
  }

  #[tokio::test]
  async fn test_dispatch_by_manual_author_search_block() {
    let (tx, mut rx) = mpsc::channel::<NetworkEvent>(500);
    let mut app = App::test_default();
    app.data.readarr_data.prompt_confirm = true;
    app.network_tx = Some(tx);
    app.data.readarr_data.authors.set_items(vec![Author {
      id: 1,
      ..Author::default()
    }]);

    app
      .dispatch_by_readarr_block(&ActiveReadarrBlock::ManualAuthorSearch)
      .await;

    assert!(app.is_loading);
    assert_eq!(
      rx.recv().await.unwrap(),
      ReadarrEvent::GetAuthorReleases(1).into()
    );
    assert!(!app.data.readarr_data.prompt_confirm);
    assert_eq!(app.tick_count, 0);
  }

  #[tokio::test]
  async fn test_dispatch_by_manual_author_search_block_author_releases_non_empty() {
    let mut app = App::test_default();
    app
      .data
      .readarr_data
      .author_releases
      .set_items(vec![ReadarrRelease::default()]);

    app
      .dispatch_by_readarr_block(&ActiveReadarrBlock::ManualAuthorSearch)
      .await;

    assert!(!app.is_loading);
    assert!(!app.data.readarr_data.prompt_confirm);
    assert_eq!(app.tick_count, 0);
  }

  #[tokio::test]
  async fn test_dispatch_by_book_details_block() {
    let (tx, mut rx) = mpsc::channel::<NetworkEvent>(500);
    let mut app = App::test_default();
    app.data.readarr_data.prompt_confirm = true;
    app.network_tx = Some(tx);
    app.data.readarr_data.authors.set_items(vec![Author {
      id: 1,
      ..Author::default()
    }]);
    app.data.readarr_data.books.set_items(vec![Book {
      id: 2,
      ..Book::default()
    }]);

    app
      .dispatch_by_readarr_block(&ActiveReadarrBlock::BookDetails)
      .await;

    assert!(app.is_loading);
    assert_eq!(
      rx.recv().await.unwrap(),
      ReadarrEvent::GetBookEditions(2).into()
    );
    assert_eq!(
      rx.recv().await.unwrap(),
      ReadarrEvent::GetBookFiles(2).into()
    );
    assert_eq!(
      rx.recv().await.unwrap(),
      ReadarrEvent::GetDownloads(500).into()
    );
    assert!(!app.data.readarr_data.prompt_confirm);
    assert_eq!(app.tick_count, 0);
  }

  #[tokio::test]
  async fn test_dispatch_by_book_history_block() {
    let (tx, mut rx) = mpsc::channel::<NetworkEvent>(500);
    let mut app = App::test_default();
    app.data.readarr_data.prompt_confirm = true;
    app.network_tx = Some(tx);
    app.data.readarr_data.authors.set_items(vec![Author {
      id: 1,
      ..Author::default()
    }]);
    app.data.readarr_data.books.set_items(vec![Book {
      id: 2,
      ..Book::default()
    }]);

    app
      .dispatch_by_readarr_block(&ActiveReadarrBlock::BookHistory)
      .await;

    assert!(app.is_loading);
    assert_eq!(
      rx.recv().await.unwrap(),
      ReadarrEvent::GetBookHistory(1, 2).into()
    );
    assert!(!app.data.readarr_data.prompt_confirm);
    assert_eq!(app.tick_count, 0);
  }

  #[tokio::test]
  async fn test_dispatch_by_book_history_block_no_op_when_books_table_is_empty() {
    let (tx, _) = mpsc::channel::<NetworkEvent>(500);
    let mut app = App::test_default();
    app.data.readarr_data.prompt_confirm = true;
    app.network_tx = Some(tx);
    app.data.readarr_data.authors.set_items(vec![Author {
      id: 1,
      ..Author::default()
    }]);

    app
      .dispatch_by_readarr_block(&ActiveReadarrBlock::BookHistory)
      .await;

    assert!(!app.is_loading);
    assert!(!app.data.readarr_data.prompt_confirm);
    assert_eq!(app.tick_count, 0);
  }

  #[tokio::test]
  async fn test_dispatch_by_manual_book_search_block() {
    let (tx, mut rx) = mpsc::channel::<NetworkEvent>(500);
    let mut app = App::test_default();
    app.data.readarr_data.prompt_confirm = true;
    app.network_tx = Some(tx);
    app.data.readarr_data.authors.set_items(vec![Author {
      id: 1,
      ..Author::default()
    }]);
    app.data.readarr_data.books.set_items(vec![Book {
      id: 2,
      ..Book::default()
    }]);
    app.data.readarr_data.book_details_modal = Some(BookDetailsModal::default());

    app
      .dispatch_by_readarr_block(&ActiveReadarrBlock::ManualBookSearch)
      .await;

    assert!(app.is_loading);
    assert_eq!(
      rx.recv().await.unwrap(),
      ReadarrEvent::GetBookReleases(2).into()
    );
    assert!(!app.data.readarr_data.prompt_confirm);
    assert_eq!(app.tick_count, 0);
  }

  #[tokio::test]
  async fn test_dispatch_by_manual_book_search_block_is_loading() {
    let mut app = App {
      is_loading: true,
      ..App::test_default()
    };

    app
      .dispatch_by_readarr_block(&ActiveReadarrBlock::ManualBookSearch)
      .await;

    assert!(app.is_loading);
    assert!(!app.data.readarr_data.prompt_confirm);
    assert_eq!(app.tick_count, 0);
  }

  #[tokio::test]
  async fn test_dispatch_by_manual_book_search_block_book_releases_non_empty() {
    let mut app = App::test_default();
    let mut book_details_modal = BookDetailsModal::default();
    book_details_modal
      .book_releases
      .set_items(vec![ReadarrRelease::default()]);
    app.data.readarr_data.book_details_modal = Some(book_details_modal);

    app
      .dispatch_by_readarr_block(&ActiveReadarrBlock::ManualBookSearch)
      .await;

    assert!(!app.is_loading);
    assert!(!app.data.readarr_data.prompt_confirm);
    assert_eq!(app.tick_count, 0);
  }

  #[tokio::test]
  async fn test_dispatch_by_book_edition_details_block_does_not_dispatch() {
    let mut app = App::test_default_fully_populated();
    app.data.readarr_data.prompt_confirm = false;

    app
      .dispatch_by_readarr_block(&ActiveReadarrBlock::BookEditionDetails)
      .await;

    assert!(!app.is_loading);
    assert_eq!(app.tick_count, 0);
  }

  #[tokio::test]
  async fn test_dispatch_by_downloads_block() {
    let (tx, mut rx) = mpsc::channel::<NetworkEvent>(500);
    let mut app = App::test_default();
    app.data.readarr_data.prompt_confirm = true;
    app.network_tx = Some(tx);

    app
      .dispatch_by_readarr_block(&ActiveReadarrBlock::Downloads)
      .await;

    assert!(app.is_loading);
    assert_eq!(
      rx.recv().await.unwrap(),
      ReadarrEvent::GetDownloads(500).into()
    );
    assert!(!app.data.readarr_data.prompt_confirm);
    assert_eq!(app.tick_count, 0);
  }

  #[tokio::test]
  async fn test_dispatch_by_readarr_block_add_author_search_results() {
    let (tx, mut rx) = mpsc::channel::<NetworkEvent>(500);
    let mut app = App::test_default();
    app.data.readarr_data.prompt_confirm = true;
    app.network_tx = Some(tx);
    app.data.readarr_data.add_author_search = Some("test author".into());

    app
      .dispatch_by_readarr_block(&ActiveReadarrBlock::AddAuthorSearchResults)
      .await;

    assert!(app.is_loading);
    assert_eq!(
      rx.recv().await.unwrap(),
      ReadarrEvent::SearchNewAuthor("test author".to_owned()).into()
    );
    assert!(!app.data.readarr_data.prompt_confirm);
    assert_eq!(app.tick_count, 0);
  }

  #[tokio::test]
  async fn test_dispatch_by_history_block() {
    let (tx, mut rx) = mpsc::channel::<NetworkEvent>(500);
    let mut app = App::test_default();
    app.data.readarr_data.prompt_confirm = true;
    app.network_tx = Some(tx);

    app
      .dispatch_by_readarr_block(&ActiveReadarrBlock::History)
      .await;

    assert!(app.is_loading);
    assert_eq!(
      rx.recv().await.unwrap(),
      ReadarrEvent::GetHistory(500).into()
    );
    assert!(!app.data.readarr_data.prompt_confirm);
    assert_eq!(app.tick_count, 0);
  }

  #[tokio::test]
  async fn test_dispatch_by_root_folders_block() {
    let (tx, mut rx) = mpsc::channel::<NetworkEvent>(500);
    let mut app = App::test_default();
    app.data.readarr_data.prompt_confirm = true;
    app.network_tx = Some(tx);

    app
      .dispatch_by_readarr_block(&ActiveReadarrBlock::RootFolders)
      .await;

    assert!(app.is_loading);
    assert_eq!(
      rx.recv().await.unwrap(),
      ReadarrEvent::GetRootFolders.into()
    );
    assert!(!app.data.readarr_data.prompt_confirm);
    assert_eq!(app.tick_count, 0);
  }

  #[tokio::test]
  async fn test_dispatch_by_indexers_block() {
    let (tx, mut rx) = mpsc::channel::<NetworkEvent>(500);
    let mut app = App::test_default();
    app.data.readarr_data.prompt_confirm = true;
    app.network_tx = Some(tx);

    app
      .dispatch_by_readarr_block(&ActiveReadarrBlock::Indexers)
      .await;

    assert!(app.is_loading);
    assert_eq!(rx.recv().await.unwrap(), ReadarrEvent::GetTags.into());
    assert_eq!(rx.recv().await.unwrap(), ReadarrEvent::GetIndexers.into());
    assert!(!app.data.readarr_data.prompt_confirm);
    assert_eq!(app.tick_count, 0);
  }

  #[tokio::test]
  async fn test_dispatch_by_all_indexer_settings_block() {
    let (tx, mut rx) = mpsc::channel::<NetworkEvent>(500);
    let mut app = App::test_default();
    app.data.readarr_data.prompt_confirm = true;
    app.network_tx = Some(tx);

    app
      .dispatch_by_readarr_block(&ActiveReadarrBlock::AllIndexerSettingsPrompt)
      .await;

    assert!(app.is_loading);
    assert_eq!(
      rx.recv().await.unwrap(),
      ReadarrEvent::GetAllIndexerSettings.into()
    );
    assert!(!app.data.readarr_data.prompt_confirm);
    assert_eq!(app.tick_count, 0);
  }

  #[tokio::test]
  async fn test_dispatch_by_test_indexer_block() {
    let (tx, mut rx) = mpsc::channel::<NetworkEvent>(500);
    let mut app = App::test_default();
    app.data.readarr_data.prompt_confirm = true;
    app.network_tx = Some(tx);
    app.data.readarr_data.indexers.set_items(vec![Indexer {
      id: 3,
      ..Indexer::default()
    }]);

    app
      .dispatch_by_readarr_block(&ActiveReadarrBlock::TestIndexer)
      .await;

    assert!(app.is_loading);
    assert!(!app.data.readarr_data.prompt_confirm);
    assert_eq!(
      rx.recv().await.unwrap(),
      ReadarrEvent::TestIndexer(3).into()
    );
    assert_eq!(app.tick_count, 0);
  }

  #[tokio::test]
  async fn test_dispatch_by_test_all_indexers_block() {
    let (tx, mut rx) = mpsc::channel::<NetworkEvent>(500);
    let mut app = App::test_default();
    app.data.readarr_data.prompt_confirm = true;
    app.network_tx = Some(tx);

    app
      .dispatch_by_readarr_block(&ActiveReadarrBlock::TestAllIndexers)
      .await;

    assert!(app.is_loading);
    assert!(!app.data.readarr_data.prompt_confirm);
    assert_eq!(
      rx.recv().await.unwrap(),
      ReadarrEvent::TestAllIndexers.into()
    );
    assert_eq!(app.tick_count, 0);
  }

  #[tokio::test]
  async fn test_dispatch_by_system_block() {
    let (tx, mut rx) = mpsc::channel::<NetworkEvent>(500);
    let mut app = App::test_default();
    app.data.readarr_data.prompt_confirm = true;
    app.network_tx = Some(tx);

    app
      .dispatch_by_readarr_block(&ActiveReadarrBlock::System)
      .await;

    assert!(app.is_loading);
    assert_eq!(rx.recv().await.unwrap(), ReadarrEvent::GetTasks.into());
    assert_eq!(
      rx.recv().await.unwrap(),
      ReadarrEvent::GetQueuedEvents.into()
    );
    assert_eq!(rx.recv().await.unwrap(), ReadarrEvent::GetLogs(500).into());
    assert!(!app.data.readarr_data.prompt_confirm);
    assert_eq!(app.tick_count, 0);
  }

  #[tokio::test]
  async fn test_dispatch_by_system_updates_block() {
    let (tx, mut rx) = mpsc::channel::<NetworkEvent>(500);
    let mut app = App::test_default();
    app.data.readarr_data.prompt_confirm = true;
    app.network_tx = Some(tx);

    app
      .dispatch_by_readarr_block(&ActiveReadarrBlock::SystemUpdates)
      .await;

    assert!(app.is_loading);
    assert_eq!(rx.recv().await.unwrap(), ReadarrEvent::GetUpdates.into());
    assert!(!app.data.readarr_data.prompt_confirm);
    assert_eq!(app.tick_count, 0);
  }

  #[tokio::test]
  async fn test_dispatch_by_readarr_block_resets_the_tick_count() {
    let mut app = App {
      tick_count: 2,
      ..App::test_default()
    };

    app
      .dispatch_by_readarr_block(&ActiveReadarrBlock::Authors)
      .await;

    assert_eq!(app.tick_count, 0);
  }

  #[tokio::test]
  async fn test_dispatch_by_readarr_block_clears_the_pending_prompt_confirmation() {
    let mut app = App::test_default();
    app.data.readarr_data.prompt_confirm = true;

    app
      .dispatch_by_readarr_block(&ActiveReadarrBlock::Authors)
      .await;

    assert!(!app.data.readarr_data.prompt_confirm);
    assert!(!app.should_refresh);
  }

  #[tokio::test]
  async fn test_check_for_readarr_prompt_action_no_prompt_confirm() {
    let mut app = App::test_default();
    app.data.readarr_data.prompt_confirm = false;

    app.check_for_readarr_prompt_action().await;

    assert!(!app.data.readarr_data.prompt_confirm);
    assert!(!app.should_refresh);
  }

  #[tokio::test]
  async fn test_check_for_readarr_prompt_action_with_no_prompt_confirm_action() {
    let mut app = App::test_default();
    app.data.readarr_data.prompt_confirm = true;

    app.check_for_readarr_prompt_action().await;

    assert!(!app.data.readarr_data.prompt_confirm);
    assert_none!(app.data.readarr_data.prompt_confirm_action);
    assert!(!app.should_refresh);
  }

  #[tokio::test]
  async fn test_check_for_readarr_prompt_action() {
    let (tx, mut rx) = mpsc::channel::<NetworkEvent>(500);
    let mut app = App::test_default();
    app.data.readarr_data.prompt_confirm = true;
    app.network_tx = Some(tx);
    app.data.readarr_data.prompt_confirm_action = Some(ReadarrEvent::GetStatus);

    app.check_for_readarr_prompt_action().await;

    assert!(!app.data.readarr_data.prompt_confirm);
    assert_eq!(rx.recv().await.unwrap(), ReadarrEvent::GetStatus.into());
    assert!(app.should_refresh);
    assert_eq!(app.data.readarr_data.prompt_confirm_action, None);
  }

  #[tokio::test]
  async fn test_readarr_refresh_metadata() {
    let (tx, mut rx) = mpsc::channel::<NetworkEvent>(500);
    let mut app = App::test_default();
    app.data.readarr_data.prompt_confirm = true;
    app.network_tx = Some(tx);
    app.is_routing = true;

    app.refresh_readarr_metadata().await;

    assert_eq!(
      rx.recv().await.unwrap(),
      ReadarrEvent::GetQualityProfiles.into()
    );
    assert_eq!(
      rx.recv().await.unwrap(),
      ReadarrEvent::GetMetadataProfiles.into()
    );
    assert_eq!(rx.recv().await.unwrap(), ReadarrEvent::GetTags.into());
    assert_eq!(
      rx.recv().await.unwrap(),
      ReadarrEvent::GetRootFolders.into()
    );
    assert_eq!(
      rx.recv().await.unwrap(),
      ReadarrEvent::GetDownloads(500).into()
    );
    assert_eq!(rx.recv().await.unwrap(), ReadarrEvent::GetDiskSpace.into());
    assert_eq!(rx.recv().await.unwrap(), ReadarrEvent::GetStatus.into());
    assert!(app.is_loading);
  }

  #[tokio::test]
  async fn test_readarr_on_tick_first_render() {
    let (tx, mut rx) = mpsc::channel::<NetworkEvent>(500);
    let mut app = App::test_default();
    app.data.readarr_data.prompt_confirm = true;
    app.network_tx = Some(tx);
    app.is_first_render = true;

    app.readarr_on_tick(ActiveReadarrBlock::Downloads).await;

    assert_eq!(
      rx.recv().await.unwrap(),
      ReadarrEvent::GetQualityProfiles.into()
    );
    assert_eq!(
      rx.recv().await.unwrap(),
      ReadarrEvent::GetMetadataProfiles.into()
    );
    assert_eq!(rx.recv().await.unwrap(), ReadarrEvent::GetTags.into());
    assert_eq!(
      rx.recv().await.unwrap(),
      ReadarrEvent::GetRootFolders.into()
    );
    assert_eq!(
      rx.recv().await.unwrap(),
      ReadarrEvent::GetDownloads(500).into()
    );
    assert_eq!(rx.recv().await.unwrap(), ReadarrEvent::GetDiskSpace.into());
    assert_eq!(rx.recv().await.unwrap(), ReadarrEvent::GetStatus.into());
    assert!(app.is_loading);
    assert!(!app.data.readarr_data.prompt_confirm);
    assert!(!app.is_first_render);
  }

  #[tokio::test]
  async fn test_readarr_on_tick_routing() {
    let (tx, mut rx) = mpsc::channel::<NetworkEvent>(500);
    let mut app = App::test_default();
    app.data.readarr_data.prompt_confirm = true;
    app.network_tx = Some(tx);
    app.is_routing = true;
    app.should_refresh = true;
    app.is_first_render = false;
    app.tick_count = 1;

    app.readarr_on_tick(ActiveReadarrBlock::Downloads).await;

    assert_eq!(
      rx.recv().await.unwrap(),
      ReadarrEvent::GetDownloads(500).into()
    );
    assert!(!app.data.readarr_data.prompt_confirm);
  }

  #[tokio::test]
  async fn test_readarr_on_tick_routing_while_long_request_is_running_should_cancel_request() {
    let (tx, _) = mpsc::channel::<NetworkEvent>(500);
    let mut app = App::test_default();
    app.data.readarr_data.prompt_confirm = true;
    app.network_tx = Some(tx);
    app.is_routing = true;
    app.should_refresh = false;
    app.is_first_render = false;
    app.tick_count = 1;

    app.readarr_on_tick(ActiveReadarrBlock::Downloads).await;

    assert!(app.cancellation_token.is_cancelled());
  }

  #[tokio::test]
  async fn test_readarr_on_tick_should_refresh() {
    let (tx, mut rx) = mpsc::channel::<NetworkEvent>(500);
    let mut app = App::test_default();
    app.data.readarr_data.prompt_confirm = true;
    app.network_tx = Some(tx);
    app.should_refresh = true;
    app.is_first_render = false;
    app.tick_count = 1;

    app.readarr_on_tick(ActiveReadarrBlock::Downloads).await;

    assert_eq!(
      rx.recv().await.unwrap(),
      ReadarrEvent::GetDownloads(500).into()
    );
    assert!(app.should_refresh);
    assert!(!app.data.readarr_data.prompt_confirm);
  }

  #[tokio::test]
  async fn test_readarr_on_tick_should_refresh_does_not_cancel_prompt_requests() {
    let (tx, mut rx) = mpsc::channel::<NetworkEvent>(500);
    let mut app = App::test_default();
    app.data.readarr_data.prompt_confirm = true;
    app.network_tx = Some(tx);
    app.is_loading = true;
    app.is_routing = true;
    app.should_refresh = true;
    app.is_first_render = false;
    app.tick_count = 1;

    app.readarr_on_tick(ActiveReadarrBlock::Downloads).await;

    assert_eq!(
      rx.recv().await.unwrap(),
      ReadarrEvent::GetDownloads(500).into()
    );
    assert!(app.is_loading);
    assert!(app.should_refresh);
    assert!(!app.data.readarr_data.prompt_confirm);
    assert!(!app.cancellation_token.is_cancelled());
  }

  #[tokio::test]
  async fn test_readarr_on_tick_network_tick_frequency() {
    let (tx, mut rx) = mpsc::channel::<NetworkEvent>(500);
    let mut app = App::test_default();
    app.data.readarr_data.prompt_confirm = true;
    app.network_tx = Some(tx);
    app.is_first_render = false;
    app.tick_count = 2;
    app.tick_until_poll = 2;

    app.readarr_on_tick(ActiveReadarrBlock::Downloads).await;

    assert_eq!(
      rx.recv().await.unwrap(),
      ReadarrEvent::GetQualityProfiles.into()
    );
    assert_eq!(
      rx.recv().await.unwrap(),
      ReadarrEvent::GetMetadataProfiles.into()
    );
    assert_eq!(rx.recv().await.unwrap(), ReadarrEvent::GetTags.into());
    assert_eq!(
      rx.recv().await.unwrap(),
      ReadarrEvent::GetRootFolders.into()
    );
    assert_eq!(
      rx.recv().await.unwrap(),
      ReadarrEvent::GetDownloads(500).into()
    );
    assert!(app.is_loading);
  }

  #[tokio::test]
  async fn test_extract_add_new_author_search_query() {
    let app = App::test_default_fully_populated();

    let query = app.extract_add_new_author_search_query().await;

    assert_str_eq!(query, "Test Author");
  }

  #[tokio::test]
  #[should_panic(expected = "Add author search is empty")]
  async fn test_extract_add_new_author_search_query_panics_when_the_query_is_not_set() {
    let app = App::test_default();

    app.extract_add_new_author_search_query().await;
  }

  #[tokio::test]
  async fn test_extract_author_id() {
    let mut app = App::test_default();
    app.data.readarr_data.authors.set_items(vec![Author {
      id: 1,
      ..Author::default()
    }]);

    assert_eq!(app.extract_author_id().await, 1);
  }

  #[tokio::test]
  async fn test_extract_book_id() {
    let mut app = App::test_default();
    app.data.readarr_data.books.set_items(vec![Book {
      id: 2,
      ..Book::default()
    }]);

    assert_eq!(app.extract_book_id().await, 2);
  }

  #[tokio::test]
  async fn test_extract_readarr_indexer_id() {
    let mut app = App::test_default();
    app.data.readarr_data.indexers.set_items(vec![Indexer {
      id: 3,
      ..Indexer::default()
    }]);

    assert_eq!(app.extract_readarr_indexer_id().await, 3);
  }
}
