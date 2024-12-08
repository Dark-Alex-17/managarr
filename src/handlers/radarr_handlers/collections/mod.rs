use crate::app::key_binding::DEFAULT_KEYBINDINGS;
use crate::app::App;
use crate::event::Key;
use crate::handlers::radarr_handlers::collections::collection_details_handler::CollectionDetailsHandler;
use crate::handlers::radarr_handlers::collections::edit_collection_handler::EditCollectionHandler;
use crate::handlers::radarr_handlers::handle_change_tab_left_right_keys;
use crate::handlers::table_handler::TableHandlingProps;
use crate::handlers::{handle_clear_errors, handle_prompt_toggle, KeyEventHandler};
use crate::models::radarr_models::Collection;
use crate::models::servarr_data::radarr::radarr_data::{
  ActiveRadarrBlock, COLLECTIONS_BLOCKS, EDIT_COLLECTION_SELECTION_BLOCKS,
};
use crate::models::stateful_table::SortOption;
use crate::models::{BlockSelectionState, HorizontallyScrollableText, Scrollable};
use crate::network::radarr_network::RadarrEvent;
use crate::handle_table_events;

mod collection_details_handler;
mod edit_collection_handler;

#[cfg(test)]
#[path = "collections_handler_tests.rs"]
mod collections_handler_tests;

pub(super) struct CollectionsHandler<'a, 'b> {
  key: Key,
  app: &'a mut App<'b>,
  active_radarr_block: ActiveRadarrBlock,
  context: Option<ActiveRadarrBlock>,
}

impl<'a, 'b> CollectionsHandler<'a, 'b> {
  handle_table_events!(self, collections, self.app.data.radarr_data.collections, Collection);
}

impl<'a, 'b> KeyEventHandler<'a, 'b, ActiveRadarrBlock> for CollectionsHandler<'a, 'b> {
  fn handle(&mut self) {
    let collections_table_handling_props = TableHandlingProps::new(ActiveRadarrBlock::Collections.into())
      .sorting_block(ActiveRadarrBlock::CollectionsSortPrompt.into())
      .sort_by_fn(|a: &Collection, b: &Collection| a.id.cmp(&b.id))
      .sort_options(collections_sorting_options())
      .searching_block(ActiveRadarrBlock::SearchCollection.into())
      .search_error_block(ActiveRadarrBlock::SearchCollectionError.into())
      .search_field_fn(|collection| &collection.title.text)
      .filtering_block(ActiveRadarrBlock::FilterCollections.into())
      .filter_error_block(ActiveRadarrBlock::FilterCollectionsError.into())
      .filter_field_fn(|collection| &collection.title.text);
    
    if !self.handle_collections_table_events(collections_table_handling_props) {
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
  }

  fn accepts(active_block: ActiveRadarrBlock) -> bool {
    CollectionDetailsHandler::accepts(active_block)
      || EditCollectionHandler::accepts(active_block)
      || COLLECTIONS_BLOCKS.contains(&active_block)
  }

  fn with(
    key: Key,
    app: &'a mut App<'b>,
    active_block: ActiveRadarrBlock,
    context: Option<ActiveRadarrBlock>,
  ) -> CollectionsHandler<'a, 'b> {
    CollectionsHandler {
      key,
      app,
      active_radarr_block: active_block,
      context,
    }
  }

  fn get_key(&self) -> Key {
    self.key
  }

  fn is_ready(&self) -> bool {
    !self.app.is_loading && !self.app.data.radarr_data.collections.is_empty()
  }

  fn handle_scroll_up(&mut self) {
  }

  fn handle_scroll_down(&mut self) {
  }

  fn handle_home(&mut self) {
  }

  fn handle_end(&mut self) {
  }

  fn handle_delete(&mut self) {}

  fn handle_left_right_action(&mut self) {
    match self.active_radarr_block {
      ActiveRadarrBlock::Collections => handle_change_tab_left_right_keys(self.app, self.key),
      ActiveRadarrBlock::UpdateAllCollectionsPrompt => handle_prompt_toggle(self.app, self.key),
      _ => (),
    }
  }

  fn handle_submit(&mut self) {
    match self.active_radarr_block {
      ActiveRadarrBlock::Collections => self
        .app
        .push_navigation_stack(ActiveRadarrBlock::CollectionDetails.into()),
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
      ActiveRadarrBlock::UpdateAllCollectionsPrompt => {
        self.app.pop_navigation_stack();
        self.app.data.radarr_data.prompt_confirm = false;
      }
      _ => {
        handle_clear_errors(self.app);
      }
    }
  }

  fn handle_char_key_event(&mut self) {
    let key = self.key;
    match self.active_radarr_block {
      ActiveRadarrBlock::Collections => match self.key {
        _ if key == DEFAULT_KEYBINDINGS.edit.key => {
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
            BlockSelectionState::new(EDIT_COLLECTION_SELECTION_BLOCKS);
        }
        _ if key == DEFAULT_KEYBINDINGS.update.key => {
          self
            .app
            .push_navigation_stack(ActiveRadarrBlock::UpdateAllCollectionsPrompt.into());
        }
        _ if key == DEFAULT_KEYBINDINGS.refresh.key => {
          self.app.should_refresh = true;
        }
        _ => (),
      },
      ActiveRadarrBlock::UpdateAllCollectionsPrompt => {
        if key == DEFAULT_KEYBINDINGS.confirm.key {
          self.app.data.radarr_data.prompt_confirm = true;
          self.app.data.radarr_data.prompt_confirm_action = Some(RadarrEvent::UpdateCollections);

          self.app.pop_navigation_stack();
        }
      }
      _ => (),
    }
  }
}

fn collections_sorting_options() -> Vec<SortOption<Collection>> {
  vec![
    SortOption {
      name: "Collection",
      cmp_fn: Some(|a, b| {
        a.title
          .text
          .to_lowercase()
          .cmp(&b.title.text.to_lowercase())
      }),
    },
    SortOption {
      name: "Number of Movies",
      cmp_fn: Some(|a, b| {
        let a_movie_count = a.movies.as_ref().unwrap_or(&Vec::new()).len();
        let b_movie_count = b.movies.as_ref().unwrap_or(&Vec::new()).len();

        a_movie_count.cmp(&b_movie_count)
      }),
    },
    SortOption {
      name: "Root Folder Path",
      cmp_fn: Some(|a, b| {
        let a_root_folder = a
          .root_folder_path
          .as_ref()
          .unwrap_or(&String::new())
          .to_owned();
        let b_root_folder = b
          .root_folder_path
          .as_ref()
          .unwrap_or(&String::new())
          .to_owned();

        a_root_folder.cmp(&b_root_folder)
      }),
    },
    SortOption {
      name: "Quality Profile",
      cmp_fn: Some(|a, b| a.quality_profile_id.cmp(&b.quality_profile_id)),
    },
    SortOption {
      name: "Search on Add",
      cmp_fn: Some(|a, b| a.search_on_add.cmp(&b.search_on_add)),
    },
    SortOption {
      name: "Monitored",
      cmp_fn: Some(|a, b| a.monitored.cmp(&b.monitored)),
    },
  ]
}
