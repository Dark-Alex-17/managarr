use crate::app::key_binding::DEFAULT_KEYBINDINGS;
use crate::app::App;
use crate::event::Key;
use crate::handlers::radarr_handlers::collections::collection_details_handler::CollectionDetailsHandler;
use crate::handlers::radarr_handlers::collections::edit_collection_handler::EditCollectionHandler;
use crate::handlers::radarr_handlers::handle_change_tab_left_right_keys;
use crate::handlers::{handle_clear_errors, handle_prompt_toggle, KeyEventHandler};
use crate::models::servarr_data::radarr::radarr_data::{
  ActiveRadarrBlock, COLLECTIONS_BLOCKS, EDIT_COLLECTION_SELECTION_BLOCKS,
};
use crate::models::{BlockSelectionState, HorizontallyScrollableText, Scrollable};
use crate::network::radarr_network::RadarrEvent;
use crate::utils::strip_non_search_characters;
use crate::{filter_table, handle_text_box_keys, handle_text_box_left_right_keys, search_table};

mod collection_details_handler;
mod edit_collection_handler;

#[cfg(test)]
#[path = "collections_handler_tests.rs"]
mod collections_handler_tests;

pub(super) struct CollectionsHandler<'a, 'b> {
  key: &'a Key,
  app: &'a mut App<'b>,
  active_radarr_block: &'a ActiveRadarrBlock,
  context: &'a Option<ActiveRadarrBlock>,
}

impl<'a, 'b> KeyEventHandler<'a, 'b, ActiveRadarrBlock> for CollectionsHandler<'a, 'b> {
  fn handle(&mut self) {
    match self.active_radarr_block {
      _ if CollectionDetailsHandler::accepts(self.active_radarr_block) => {
        CollectionDetailsHandler::with(self.key, self.app, self.active_radarr_block, self.context)
          .handle();
      }
      _ if EditCollectionHandler::accepts(self.active_radarr_block) => {
        EditCollectionHandler::with(self.key, self.app, self.active_radarr_block, self.context)
          .handle();
      }
      _ => self.handle_key_event(),
    }
  }

  fn accepts(active_block: &'a ActiveRadarrBlock) -> bool {
    CollectionDetailsHandler::accepts(active_block)
      || EditCollectionHandler::accepts(active_block)
      || COLLECTIONS_BLOCKS.contains(active_block)
  }

  fn with(
    key: &'a Key,
    app: &'a mut App<'b>,
    active_block: &'a ActiveRadarrBlock,
    context: &'a Option<ActiveRadarrBlock>,
  ) -> CollectionsHandler<'a, 'b> {
    CollectionsHandler {
      key,
      app,
      active_radarr_block: active_block,
      context,
    }
  }

  fn get_key(&self) -> &Key {
    self.key
  }

  fn handle_scroll_up(&mut self) {
    if self.active_radarr_block == &ActiveRadarrBlock::Collections {
      if !self
        .app
        .data
        .radarr_data
        .filtered_collections
        .items
        .is_empty()
      {
        self.app.data.radarr_data.filtered_collections.scroll_up();
      } else {
        self.app.data.radarr_data.collections.scroll_up()
      }
    }
  }

  fn handle_scroll_down(&mut self) {
    if self.active_radarr_block == &ActiveRadarrBlock::Collections {
      if !self
        .app
        .data
        .radarr_data
        .filtered_collections
        .items
        .is_empty()
      {
        self.app.data.radarr_data.filtered_collections.scroll_down();
      } else {
        self.app.data.radarr_data.collections.scroll_down()
      }
    }
  }

  fn handle_home(&mut self) {
    match self.active_radarr_block {
      ActiveRadarrBlock::Collections => {
        if !self
          .app
          .data
          .radarr_data
          .filtered_collections
          .items
          .is_empty()
        {
          self
            .app
            .data
            .radarr_data
            .filtered_collections
            .scroll_to_top();
        } else {
          self.app.data.radarr_data.collections.scroll_to_top()
        }
      }
      ActiveRadarrBlock::SearchCollection => self
        .app
        .data
        .radarr_data
        .search
        .as_mut()
        .unwrap()
        .scroll_home(),
      ActiveRadarrBlock::FilterCollections => self
        .app
        .data
        .radarr_data
        .filter
        .as_mut()
        .unwrap()
        .scroll_home(),
      _ => (),
    }
  }

  fn handle_end(&mut self) {
    match self.active_radarr_block {
      ActiveRadarrBlock::Collections => {
        if !self
          .app
          .data
          .radarr_data
          .filtered_collections
          .items
          .is_empty()
        {
          self
            .app
            .data
            .radarr_data
            .filtered_collections
            .scroll_to_bottom();
        } else {
          self.app.data.radarr_data.collections.scroll_to_bottom()
        }
      }
      ActiveRadarrBlock::SearchCollection => self
        .app
        .data
        .radarr_data
        .search
        .as_mut()
        .unwrap()
        .reset_offset(),
      ActiveRadarrBlock::FilterCollections => self
        .app
        .data
        .radarr_data
        .filter
        .as_mut()
        .unwrap()
        .reset_offset(),
      _ => (),
    }
  }

  fn handle_delete(&mut self) {}

  fn handle_left_right_action(&mut self) {
    match self.active_radarr_block {
      ActiveRadarrBlock::Collections => handle_change_tab_left_right_keys(self.app, self.key),
      ActiveRadarrBlock::UpdateAllCollectionsPrompt => handle_prompt_toggle(self.app, self.key),
      ActiveRadarrBlock::SearchCollection => {
        handle_text_box_left_right_keys!(
          self,
          self.key,
          self.app.data.radarr_data.search.as_mut().unwrap()
        )
      }
      ActiveRadarrBlock::FilterCollections => {
        handle_text_box_left_right_keys!(
          self,
          self.key,
          self.app.data.radarr_data.filter.as_mut().unwrap()
        )
      }
      _ => (),
    }
  }

  fn handle_submit(&mut self) {
    match self.active_radarr_block {
      ActiveRadarrBlock::Collections => self
        .app
        .push_navigation_stack(ActiveRadarrBlock::CollectionDetails.into()),
      ActiveRadarrBlock::SearchCollection => {
        if self
          .app
          .data
          .radarr_data
          .filtered_collections
          .items
          .is_empty()
        {
          search_table!(
            self.app,
            collections,
            ActiveRadarrBlock::SearchCollectionError
          );
        } else {
          search_table!(
            self.app,
            filtered_collections,
            ActiveRadarrBlock::SearchCollectionError
          );
        }
      }
      ActiveRadarrBlock::FilterCollections => {
        filter_table!(
          self.app,
          collections,
          filtered_collections,
          ActiveRadarrBlock::FilterCollectionsError
        );
      }
      ActiveRadarrBlock::UpdateAllCollectionsPrompt => {
        if self.app.data.radarr_data.prompt_confirm {
          self.app.data.radarr_data.prompt_confirm_action = Some(RadarrEvent::UpdateCollections);
        }

        self.app.pop_navigation_stack();
      }
      _ => (),
    }
  }

  fn handle_esc(&mut self) {
    match self.active_radarr_block {
      ActiveRadarrBlock::FilterCollections | ActiveRadarrBlock::FilterCollectionsError => {
        self.app.pop_navigation_stack();
        self.app.data.radarr_data.reset_filter();
        self.app.should_ignore_quit_key = false;
      }
      ActiveRadarrBlock::SearchCollection | ActiveRadarrBlock::SearchCollectionError => {
        self.app.pop_navigation_stack();
        self.app.data.radarr_data.reset_search();
        self.app.should_ignore_quit_key = false;
      }
      ActiveRadarrBlock::UpdateAllCollectionsPrompt => {
        self.app.pop_navigation_stack();
        self.app.data.radarr_data.prompt_confirm = false;
      }
      _ => {
        self.app.data.radarr_data.reset_search();
        self.app.data.radarr_data.reset_filter();
        handle_clear_errors(self.app);
      }
    }
  }

  fn handle_char_key_event(&mut self) {
    let key = self.key;
    match self.active_radarr_block {
      ActiveRadarrBlock::Collections => match self.key {
        _ if *key == DEFAULT_KEYBINDINGS.search.key => {
          self
            .app
            .push_navigation_stack(ActiveRadarrBlock::SearchCollection.into());
          self.app.data.radarr_data.search = Some(HorizontallyScrollableText::default());
          self.app.data.radarr_data.is_searching = true;
          self.app.should_ignore_quit_key = true;
        }
        _ if *key == DEFAULT_KEYBINDINGS.filter.key => {
          self
            .app
            .push_navigation_stack(ActiveRadarrBlock::FilterCollections.into());
          self.app.data.radarr_data.reset_filter();
          self.app.data.radarr_data.filter = Some(HorizontallyScrollableText::default());
          self.app.data.radarr_data.is_filtering = true;
          self.app.should_ignore_quit_key = true;
        }
        _ if *key == DEFAULT_KEYBINDINGS.edit.key => {
          self.app.push_navigation_stack(
            (
              ActiveRadarrBlock::EditCollectionPrompt,
              Some(ActiveRadarrBlock::Collections),
            )
              .into(),
          );
          self.app.data.radarr_data.edit_collection_modal =
            Some((&self.app.data.radarr_data).into());
          self.app.data.radarr_data.selected_block =
            BlockSelectionState::new(&EDIT_COLLECTION_SELECTION_BLOCKS);
        }
        _ if *key == DEFAULT_KEYBINDINGS.update.key => {
          self
            .app
            .push_navigation_stack(ActiveRadarrBlock::UpdateAllCollectionsPrompt.into());
        }
        _ if *key == DEFAULT_KEYBINDINGS.refresh.key => {
          self.app.should_refresh = true;
        }
        _ => (),
      },
      ActiveRadarrBlock::SearchCollection => {
        handle_text_box_keys!(
          self,
          key,
          self.app.data.radarr_data.search.as_mut().unwrap()
        )
      }
      ActiveRadarrBlock::FilterCollections => {
        handle_text_box_keys!(
          self,
          key,
          self.app.data.radarr_data.filter.as_mut().unwrap()
        )
      }
      _ => (),
    }
  }
}
