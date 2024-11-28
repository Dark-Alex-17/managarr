use crate::app::key_binding::DEFAULT_KEYBINDINGS;
use crate::app::App;
use crate::event::Key;
use crate::handlers::radarr_handlers::collections::collection_details_handler::CollectionDetailsHandler;
use crate::handlers::radarr_handlers::collections::edit_collection_handler::EditCollectionHandler;
use crate::handlers::radarr_handlers::handle_change_tab_left_right_keys;
use crate::handlers::{handle_clear_errors, handle_prompt_toggle, KeyEventHandler};
use crate::models::radarr_models::Collection;
use crate::models::servarr_data::radarr::radarr_data::{
  ActiveRadarrBlock, COLLECTIONS_BLOCKS, EDIT_COLLECTION_SELECTION_BLOCKS,
};
use crate::models::stateful_table::SortOption;
use crate::models::{BlockSelectionState, HorizontallyScrollableText, Scrollable};
use crate::network::radarr_network::RadarrEvent;
use crate::{handle_text_box_keys, handle_text_box_left_right_keys};

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
    match self.active_radarr_block {
      ActiveRadarrBlock::Collections => self.app.data.radarr_data.collections.scroll_up(),
      ActiveRadarrBlock::CollectionsSortPrompt => self
        .app
        .data
        .radarr_data
        .collections
        .sort
        .as_mut()
        .unwrap()
        .scroll_up(),
      _ => (),
    }
  }

  fn handle_scroll_down(&mut self) {
    match self.active_radarr_block {
      ActiveRadarrBlock::Collections => self.app.data.radarr_data.collections.scroll_down(),
      ActiveRadarrBlock::CollectionsSortPrompt => self
        .app
        .data
        .radarr_data
        .collections
        .sort
        .as_mut()
        .unwrap()
        .scroll_down(),
      _ => (),
    }
  }

  fn handle_home(&mut self) {
    match self.active_radarr_block {
      ActiveRadarrBlock::Collections => self.app.data.radarr_data.collections.scroll_to_top(),
      ActiveRadarrBlock::SearchCollection => self
        .app
        .data
        .radarr_data
        .collections
        .search
        .as_mut()
        .unwrap()
        .scroll_home(),
      ActiveRadarrBlock::FilterCollections => self
        .app
        .data
        .radarr_data
        .collections
        .filter
        .as_mut()
        .unwrap()
        .scroll_home(),
      ActiveRadarrBlock::CollectionsSortPrompt => self
        .app
        .data
        .radarr_data
        .collections
        .sort
        .as_mut()
        .unwrap()
        .scroll_to_top(),
      _ => (),
    }
  }

  fn handle_end(&mut self) {
    match self.active_radarr_block {
      ActiveRadarrBlock::Collections => self.app.data.radarr_data.collections.scroll_to_bottom(),
      ActiveRadarrBlock::SearchCollection => self
        .app
        .data
        .radarr_data
        .collections
        .search
        .as_mut()
        .unwrap()
        .reset_offset(),
      ActiveRadarrBlock::FilterCollections => self
        .app
        .data
        .radarr_data
        .collections
        .filter
        .as_mut()
        .unwrap()
        .reset_offset(),
      ActiveRadarrBlock::CollectionsSortPrompt => self
        .app
        .data
        .radarr_data
        .collections
        .sort
        .as_mut()
        .unwrap()
        .scroll_to_bottom(),
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
          self
            .app
            .data
            .radarr_data
            .collections
            .search
            .as_mut()
            .unwrap()
        )
      }
      ActiveRadarrBlock::FilterCollections => {
        handle_text_box_left_right_keys!(
          self,
          self.key,
          self
            .app
            .data
            .radarr_data
            .collections
            .filter
            .as_mut()
            .unwrap()
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
        self.app.pop_navigation_stack();
        self.app.should_ignore_quit_key = false;

        if self.app.data.radarr_data.collections.search.is_some() {
          let has_match = self
            .app
            .data
            .radarr_data
            .collections
            .apply_search(|collection| &collection.title.text);

          if !has_match {
            self
              .app
              .push_navigation_stack(ActiveRadarrBlock::SearchCollectionError.into());
          }
        }
      }
      ActiveRadarrBlock::FilterCollections => {
        self.app.pop_navigation_stack();
        self.app.should_ignore_quit_key = false;

        if self.app.data.radarr_data.collections.filter.is_some() {
          let has_matches = self
            .app
            .data
            .radarr_data
            .collections
            .apply_filter(|collection| &collection.title.text);

          if !has_matches {
            self
              .app
              .push_navigation_stack(ActiveRadarrBlock::FilterCollectionsError.into());
          }
        }
      }
      ActiveRadarrBlock::UpdateAllCollectionsPrompt => {
        if self.app.data.radarr_data.prompt_confirm {
          self.app.data.radarr_data.prompt_confirm_action = Some(RadarrEvent::UpdateCollections);
        }

        self.app.pop_navigation_stack();
      }
      ActiveRadarrBlock::CollectionsSortPrompt => {
        self
          .app
          .data
          .radarr_data
          .collections
          .items
          .sort_by(|a, b| a.id.cmp(&b.id));
        self.app.data.radarr_data.collections.apply_sorting();

        self.app.pop_navigation_stack();
      }
      _ => (),
    }
  }

  fn handle_esc(&mut self) {
    match self.active_radarr_block {
      ActiveRadarrBlock::FilterCollections | ActiveRadarrBlock::FilterCollectionsError => {
        self.app.pop_navigation_stack();
        self.app.data.radarr_data.collections.reset_filter();
        self.app.should_ignore_quit_key = false;
      }
      ActiveRadarrBlock::SearchCollection | ActiveRadarrBlock::SearchCollectionError => {
        self.app.pop_navigation_stack();
        self.app.data.radarr_data.collections.reset_search();
        self.app.should_ignore_quit_key = false;
      }
      ActiveRadarrBlock::UpdateAllCollectionsPrompt => {
        self.app.pop_navigation_stack();
        self.app.data.radarr_data.prompt_confirm = false;
      }
      ActiveRadarrBlock::CollectionsSortPrompt => {
        self.app.pop_navigation_stack();
      }
      _ => {
        self.app.data.radarr_data.collections.reset_search();
        self.app.data.radarr_data.collections.reset_filter();
        handle_clear_errors(self.app);
      }
    }
  }

  fn handle_char_key_event(&mut self) {
    let key = self.key;
    match self.active_radarr_block {
      ActiveRadarrBlock::Collections => match self.key {
        _ if key == DEFAULT_KEYBINDINGS.search.key => {
          self
            .app
            .push_navigation_stack(ActiveRadarrBlock::SearchCollection.into());
          self.app.data.radarr_data.collections.search =
            Some(HorizontallyScrollableText::default());
          self.app.should_ignore_quit_key = true;
        }
        _ if key == DEFAULT_KEYBINDINGS.filter.key => {
          self
            .app
            .push_navigation_stack(ActiveRadarrBlock::FilterCollections.into());
          self.app.data.radarr_data.collections.reset_filter();
          self.app.data.radarr_data.collections.filter =
            Some(HorizontallyScrollableText::default());
          self.app.should_ignore_quit_key = true;
        }
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
            BlockSelectionState::new(&EDIT_COLLECTION_SELECTION_BLOCKS);
        }
        _ if key == DEFAULT_KEYBINDINGS.update.key => {
          self
            .app
            .push_navigation_stack(ActiveRadarrBlock::UpdateAllCollectionsPrompt.into());
        }
        _ if key == DEFAULT_KEYBINDINGS.refresh.key => {
          self.app.should_refresh = true;
        }
        _ if key == DEFAULT_KEYBINDINGS.sort.key => {
          self
            .app
            .data
            .radarr_data
            .collections
            .sorting(collections_sorting_options());
          self
            .app
            .push_navigation_stack(ActiveRadarrBlock::CollectionsSortPrompt.into());
        }
        _ => (),
      },
      ActiveRadarrBlock::SearchCollection => {
        handle_text_box_keys!(
          self,
          key,
          self
            .app
            .data
            .radarr_data
            .collections
            .search
            .as_mut()
            .unwrap()
        )
      }
      ActiveRadarrBlock::FilterCollections => {
        handle_text_box_keys!(
          self,
          key,
          self
            .app
            .data
            .radarr_data
            .collections
            .filter
            .as_mut()
            .unwrap()
        )
      }
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
