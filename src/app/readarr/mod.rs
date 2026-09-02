use super::App;
use crate::{
  models::servarr_data::readarr::readarr_data::ActiveReadarrBlock,
  network::readarr_network::ReadarrEvent,
};

pub mod readarr_context_clues;

#[cfg(test)]
#[path = "readarr_tests.rs"]
mod readarr_tests;

impl App<'_> {
  pub(super) async fn dispatch_by_readarr_block(
    &mut self,
    active_readarr_block: &ActiveReadarrBlock,
  ) {
    match active_readarr_block {
      ActiveReadarrBlock::Authors => {
        self
          .dispatch_network_event(ReadarrEvent::GetQualityProfiles.into())
          .await;
        self
          .dispatch_network_event(ReadarrEvent::GetMetadataProfiles.into())
          .await;
        self
          .dispatch_network_event(ReadarrEvent::GetTags.into())
          .await;
        self
          .dispatch_network_event(ReadarrEvent::ListAuthors.into())
          .await;
      }
      ActiveReadarrBlock::Blocklist => {
        self
          .dispatch_network_event(ReadarrEvent::GetBlocklist.into())
          .await;
      }
      ActiveReadarrBlock::Downloads => {
        self
          .dispatch_network_event(ReadarrEvent::GetDownloads(500).into())
          .await;
      }
      ActiveReadarrBlock::AuthorDetails => {
        self
          .dispatch_network_event(ReadarrEvent::GetBooks(self.extract_author_id().await).into())
          .await;
      }
      ActiveReadarrBlock::AuthorHistory => {
        self
          .dispatch_network_event(
            ReadarrEvent::GetAuthorHistory(self.extract_author_id().await).into(),
          )
          .await;
      }
      ActiveReadarrBlock::ManualAuthorSearch => {
        if self.data.readarr_data.author_releases.is_empty() {
          self
            .dispatch_network_event(
              ReadarrEvent::GetAuthorReleases(self.extract_author_id().await).into(),
            )
            .await;
        }
      }
      ActiveReadarrBlock::BookDetails => {
        let book_id = self.extract_book_id().await;
        self
          .dispatch_network_event(ReadarrEvent::GetBookEditions(book_id).into())
          .await;
        self
          .dispatch_network_event(ReadarrEvent::GetBookFiles(book_id).into())
          .await;
        self
          .dispatch_network_event(ReadarrEvent::GetDownloads(500).into())
          .await;
      }
      ActiveReadarrBlock::BookHistory => {
        if !self.data.readarr_data.books.is_empty() {
          self
            .dispatch_network_event(
              ReadarrEvent::GetBookHistory(
                self.extract_author_id().await,
                self.extract_book_id().await,
              )
              .into(),
            )
            .await;
        }
      }
      ActiveReadarrBlock::ManualBookSearch => {
        match self.data.readarr_data.book_details_modal.as_ref() {
          Some(book_details_modal) if book_details_modal.book_releases.is_empty() => {
            self
              .dispatch_network_event(
                ReadarrEvent::GetBookReleases(self.extract_book_id().await).into(),
              )
              .await;
          }
          _ => (),
        }
      }
      ActiveReadarrBlock::AddAuthorSearchResults => {
        self
          .dispatch_network_event(
            ReadarrEvent::SearchNewAuthor(self.extract_add_new_author_search_query().await).into(),
          )
          .await;
      }
      ActiveReadarrBlock::History => {
        self
          .dispatch_network_event(ReadarrEvent::GetHistory(500).into())
          .await
      }
      ActiveReadarrBlock::RootFolders => {
        self
          .dispatch_network_event(ReadarrEvent::GetRootFolders.into())
          .await;
      }
      ActiveReadarrBlock::Indexers => {
        self
          .dispatch_network_event(ReadarrEvent::GetTags.into())
          .await;
        self
          .dispatch_network_event(ReadarrEvent::GetIndexers.into())
          .await;
      }
      ActiveReadarrBlock::AllIndexerSettingsPrompt => {
        self
          .dispatch_network_event(ReadarrEvent::GetAllIndexerSettings.into())
          .await;
      }
      ActiveReadarrBlock::TestIndexer => {
        self
          .dispatch_network_event(
            ReadarrEvent::TestIndexer(self.extract_readarr_indexer_id().await).into(),
          )
          .await;
      }
      ActiveReadarrBlock::TestAllIndexers => {
        self
          .dispatch_network_event(ReadarrEvent::TestAllIndexers.into())
          .await;
      }
      ActiveReadarrBlock::System => {
        self
          .dispatch_network_event(ReadarrEvent::GetTasks.into())
          .await;
        self
          .dispatch_network_event(ReadarrEvent::GetQueuedEvents.into())
          .await;
        self
          .dispatch_network_event(ReadarrEvent::GetLogs(500).into())
          .await;
      }
      ActiveReadarrBlock::SystemUpdates => {
        self
          .dispatch_network_event(ReadarrEvent::GetUpdates.into())
          .await;
      }
      _ => (),
    }

    self.check_for_readarr_prompt_action().await;
    self.reset_tick_count();
  }

  async fn extract_add_new_author_search_query(&self) -> String {
    self
      .data
      .readarr_data
      .add_author_search
      .as_ref()
      .expect("Add author search is empty")
      .text
      .clone()
  }

  async fn extract_author_id(&self) -> i64 {
    self.data.readarr_data.authors.current_selection().id
  }

  async fn extract_book_id(&self) -> i64 {
    self.data.readarr_data.books.current_selection().id
  }

  async fn extract_readarr_indexer_id(&self) -> i64 {
    self.data.readarr_data.indexers.current_selection().id
  }

  async fn check_for_readarr_prompt_action(&mut self) {
    if self.data.readarr_data.prompt_confirm {
      self.data.readarr_data.prompt_confirm = false;
      if let Some(readarr_event) = self.data.readarr_data.prompt_confirm_action.take() {
        self.dispatch_network_event(readarr_event.into()).await;
        self.should_refresh = true;
      }
    }
  }

  pub(super) async fn readarr_on_tick(&mut self, active_readarr_block: ActiveReadarrBlock) {
    if self.is_first_render {
      self.refresh_readarr_metadata().await;
      self.dispatch_by_readarr_block(&active_readarr_block).await;
      self.is_first_render = false;
      return;
    }

    if self.should_refresh {
      self.dispatch_by_readarr_block(&active_readarr_block).await;
      self.refresh_readarr_metadata().await;
    }

    if self.is_routing {
      if !self.should_refresh {
        self.cancellation_token.cancel();
      } else {
        self.dispatch_by_readarr_block(&active_readarr_block).await;
      }
    }

    if self.tick_count.is_multiple_of(self.tick_until_poll) {
      self.refresh_readarr_metadata().await;
    }
  }

  async fn refresh_readarr_metadata(&mut self) {
    self
      .dispatch_network_event(ReadarrEvent::GetQualityProfiles.into())
      .await;
    self
      .dispatch_network_event(ReadarrEvent::GetMetadataProfiles.into())
      .await;
    self
      .dispatch_network_event(ReadarrEvent::GetTags.into())
      .await;
    self
      .dispatch_network_event(ReadarrEvent::GetRootFolders.into())
      .await;
    self
      .dispatch_network_event(ReadarrEvent::GetDownloads(500).into())
      .await;
    self
      .dispatch_network_event(ReadarrEvent::GetDiskSpace.into())
      .await;
    self
      .dispatch_network_event(ReadarrEvent::GetStatus.into())
      .await;
  }
}
