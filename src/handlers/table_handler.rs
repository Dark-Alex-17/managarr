use crate::app::App;
use crate::app::key_binding::DEFAULT_KEYBINDINGS;
use crate::event::Key;
use crate::matches_key;
use crate::models::stateful_table::{SortOption, StatefulTable};
use crate::models::{HorizontallyScrollableText, Paginated, Route, Scrollable};
use derive_setters::Setters;
use std::fmt::Debug;
use std::marker::PhantomData;

#[cfg(test)]
#[path = "table_handler_tests.rs"]
mod table_handler_tests;

#[derive(Setters)]
pub struct TableHandlingConfig<T>
where
  T: Clone + PartialEq + Eq + Debug + Default,
{
  #[setters(strip_option)]
  pub sorting_block: Option<Route>,
  #[setters(strip_option)]
  pub sort_options: Option<Vec<SortOption<T>>>,
  #[setters(strip_option)]
  pub searching_block: Option<Route>,
  #[setters(strip_option)]
  pub search_error_block: Option<Route>,
  #[setters(strip_option)]
  pub search_field_fn: Option<fn(&T) -> &str>,
  #[setters(strip_option)]
  pub filtering_block: Option<Route>,
  #[setters(strip_option)]
  pub filter_error_block: Option<Route>,
  #[setters(strip_option)]
  pub filter_field_fn: Option<fn(&T) -> &str>,
  #[setters(skip)]
  pub table_block: Route,
}

impl<T> TableHandlingConfig<T>
where
  T: Clone + PartialEq + Eq + Debug + Default,
{
  pub fn new(table_block: Route) -> Self {
    Self {
      table_block,
      sorting_block: None,
      sort_options: None,
      searching_block: None,
      search_error_block: None,
      search_field_fn: None,
      filtering_block: None,
      filter_error_block: None,
      filter_field_fn: None,
    }
  }
}

pub trait TableEventHandler<'b, T>
where
  T: Clone + PartialEq + Eq + Debug + Default,
{
  /// Returns a mutable reference to the table being managed.
  fn table_mut(&mut self) -> &mut StatefulTable<T>;

  /// Returns the configuration for this table's event handling.
  fn config(&self) -> &TableHandlingConfig<T>;

  /// Returns whether the handler is ready to process events.
  ///
  /// Typically, checks if data is loaded and the table is not empty.
  fn is_ready(&self) -> bool;

  /// Returns the current navigation route.
  fn current_route(&self) -> Route;

  /// Returns whether special keys should be ignored for textbox input.
  fn ignore_special_keys(&self) -> bool;

  /// Returns a mutable reference to the application state.
  fn app_mut(&mut self) -> &mut App<'b>;

  /// Returns the current key event being processed.
  fn key(&self) -> Key;

  /// Main entry point for table event handling.
  ///
  /// Returns `true` if the event was handled by table logic, `false` otherwise.
  /// When `false` is returned, the caller should delegate to other handlers.
  fn handle_table_events(&mut self) -> bool {
    if !self.is_ready() {
      return false;
    }

    let key = self.key();
    let config = self.config();
    let current_route = self.current_route();
    let ignore_special = self.ignore_special_keys();

    match key {
      _ if matches_key!(up, key, ignore_special) => self.handle_scroll_up(),
      _ if matches_key!(down, key, ignore_special) => self.handle_scroll_down(),
      _ if matches_key!(pg_up, key, ignore_special) => self.handle_page_up(),
      _ if matches_key!(pg_down, key, ignore_special) => self.handle_page_down(),
      _ if matches_key!(home, key) => self.handle_home(),
      _ if matches_key!(end, key) => self.handle_end(),
      _ if matches_key!(left, key, ignore_special) || matches_key!(right, key, ignore_special) => {
        self.handle_left_right()
      }
      _ if matches_key!(submit, key) => self.handle_submit(),
      _ if matches_key!(esc, key) => self.handle_esc(),
      _ if config.searching_block.is_some()
        && current_route == *config.searching_block.as_ref().unwrap() =>
      {
        self.handle_search_box_input()
      }
      _ if config.filtering_block.is_some()
        && current_route == *config.filtering_block.as_ref().unwrap() =>
      {
        self.handle_filter_box_input()
      }
      _ if matches_key!(filter, key) && config.filtering_block.is_some() => {
        self.handle_filter_key()
      }
      _ if matches_key!(search, key) && config.searching_block.is_some() => {
        self.handle_search_key()
      }
      _ if matches_key!(sort, key) && config.sorting_block.is_some() => self.handle_sort_key(),
      _ => false,
    }
  }

  fn handle_scroll_up(&mut self) -> bool {
    let config = self.config();
    let current_route = self.current_route();

    match current_route {
      _ if config.table_block == current_route => {
        self.table_mut().scroll_up();
        true
      }
      _ if config.sorting_block.is_some()
        && current_route == *config.sorting_block.as_ref().unwrap() =>
      {
        if let Some(ref mut sort) = self.table_mut().sort {
          sort.scroll_up();
        }
        true
      }
      _ => false,
    }
  }

  fn handle_scroll_down(&mut self) -> bool {
    let config = self.config();
    let current_route = self.current_route();

    match current_route {
      _ if config.table_block == current_route => {
        self.table_mut().scroll_down();
        true
      }
      _ if config.sorting_block.is_some()
        && current_route == *config.sorting_block.as_ref().unwrap() =>
      {
        if let Some(ref mut sort) = self.table_mut().sort {
          sort.scroll_down();
        }
        true
      }
      _ => false,
    }
  }

  fn handle_page_up(&mut self) -> bool {
    let config = self.config();
    let current_route = self.current_route();

    if config.table_block == current_route {
      self.table_mut().page_up();
      true
    } else {
      false
    }
  }

  fn handle_page_down(&mut self) -> bool {
    let config = self.config();
    let current_route = self.current_route();

    if config.table_block == current_route {
      self.table_mut().page_down();
      true
    } else {
      false
    }
  }

  fn handle_home(&mut self) -> bool {
    let config = self.config();
    let current_route = self.current_route();

    match current_route {
      _ if config.table_block == current_route => {
        self.table_mut().scroll_to_top();
        true
      }
      _ if config.sorting_block.is_some()
        && current_route == *config.sorting_block.as_ref().unwrap() =>
      {
        if let Some(ref mut sort) = self.table_mut().sort {
          sort.scroll_to_top();
        }
        true
      }
      _ if config.searching_block.is_some()
        && current_route == *config.searching_block.as_ref().unwrap() =>
      {
        if let Some(ref mut search) = self.table_mut().search {
          search.scroll_home();
        }
        true
      }
      _ if config.filtering_block.is_some()
        && current_route == *config.filtering_block.as_ref().unwrap() =>
      {
        if let Some(ref mut filter) = self.table_mut().filter {
          filter.scroll_home();
        }
        true
      }
      _ => false,
    }
  }

  fn handle_end(&mut self) -> bool {
    let config = self.config();
    let current_route = self.current_route();

    match current_route {
      _ if config.table_block == current_route => {
        self.table_mut().scroll_to_bottom();
        true
      }
      _ if config.sorting_block.is_some()
        && current_route == *config.sorting_block.as_ref().unwrap() =>
      {
        if let Some(ref mut sort) = self.table_mut().sort {
          sort.scroll_to_bottom();
        }
        true
      }
      _ if config.searching_block.is_some()
        && current_route == *config.searching_block.as_ref().unwrap() =>
      {
        if let Some(ref mut search) = self.table_mut().search {
          search.reset_offset();
        }
        true
      }
      _ if config.filtering_block.is_some()
        && current_route == *config.filtering_block.as_ref().unwrap() =>
      {
        if let Some(ref mut filter) = self.table_mut().filter {
          filter.reset_offset();
        }
        true
      }
      _ => false,
    }
  }

  fn handle_left_right(&mut self) -> bool {
    let config = self.config();
    let current_route = self.current_route();
    let key = self.key();

    match current_route {
      _ if config.searching_block.is_some()
        && current_route == *config.searching_block.as_ref().unwrap() =>
      {
        if let Some(ref mut search) = self.table_mut().search {
          if key == DEFAULT_KEYBINDINGS.left.key {
            search.scroll_left();
          } else if key == DEFAULT_KEYBINDINGS.right.key {
            search.scroll_right();
          }
        }
        true
      }
      _ if config.filtering_block.is_some()
        && current_route == *config.filtering_block.as_ref().unwrap() =>
      {
        if let Some(ref mut filter) = self.table_mut().filter {
          if key == DEFAULT_KEYBINDINGS.left.key {
            filter.scroll_left();
          } else if key == DEFAULT_KEYBINDINGS.right.key {
            filter.scroll_right();
          }
        }
        true
      }
      _ => false,
    }
  }

  fn handle_submit(&mut self) -> bool {
    let config = self.config();
    let current_route = self.current_route();

    let sorting_block = config.sorting_block;
    let searching_block = config.searching_block;
    let search_field_fn = config.search_field_fn;
    let search_error_block = config.search_error_block;
    let filtering_block = config.filtering_block;
    let filter_field_fn = config.filter_field_fn;
    let filter_error_block = config.filter_error_block;

    match current_route {
      _ if sorting_block.is_some() && current_route == *sorting_block.as_ref().unwrap() => {
        self.table_mut().apply_sorting();
        self.app_mut().pop_navigation_stack();
        true
      }
      _ if searching_block.is_some() && current_route == *searching_block.as_ref().unwrap() => {
        let app = self.app_mut();
        app.pop_navigation_stack();
        app.ignore_special_keys_for_textbox_input = false;

        if self.table_mut().search.is_some() {
          let search_fn = search_field_fn.expect("Search field function is required");
          let has_match = self.table_mut().apply_search(search_fn);

          if !has_match {
            self
              .app_mut()
              .push_navigation_stack(search_error_block.expect("Search error block is undefined"));
          }
        }

        true
      }
      _ if filtering_block.is_some() && current_route == *filtering_block.as_ref().unwrap() => {
        let app = self.app_mut();
        app.pop_navigation_stack();
        app.ignore_special_keys_for_textbox_input = false;

        if self.table_mut().filter.is_some() {
          let filter_fn = filter_field_fn.expect("Filter field function is required");
          let has_match = self.table_mut().apply_filter(filter_fn);

          if !has_match {
            self
              .app_mut()
              .push_navigation_stack(filter_error_block.expect("Filter error block is undefined"));
          }
        }

        true
      }
      _ => false,
    }
  }

  fn handle_esc(&mut self) -> bool {
    let config = self.config();
    let current_route = self.current_route();

    let sorting_block = config.sorting_block;
    let searching_block = config.searching_block;
    let search_error_block = config.search_error_block;
    let filtering_block = config.filtering_block;
    let filter_error_block = config.filter_error_block;
    let table_block = config.table_block;

    match current_route {
      _ if sorting_block.is_some() && current_route == *sorting_block.as_ref().unwrap() => {
        self.app_mut().pop_navigation_stack();
        true
      }
      _ if (searching_block.is_some() && current_route == *searching_block.as_ref().unwrap())
        || (search_error_block.is_some()
          && current_route == *search_error_block.as_ref().unwrap()) =>
      {
        self.app_mut().pop_navigation_stack();
        self.table_mut().reset_search();
        self.app_mut().ignore_special_keys_for_textbox_input = false;
        true
      }
      _ if (filtering_block.is_some() && current_route == *filtering_block.as_ref().unwrap())
        || (filter_error_block.is_some()
          && current_route == *filter_error_block.as_ref().unwrap()) =>
      {
        self.app_mut().pop_navigation_stack();
        self.table_mut().reset_filter();
        self.app_mut().ignore_special_keys_for_textbox_input = false;
        true
      }
      _ if table_block == current_route && self.table_mut().filtered_items.is_some() => {
        self.table_mut().reset_filter();
        true
      }
      _ => false,
    }
  }

  fn handle_search_box_input(&mut self) -> bool {
    let key = self.key();
    let Some(ref mut search) = self.table_mut().search else {
      return false;
    };

    match key {
      _ if matches_key!(backspace, key) => {
        search.pop();
      }
      Key::Char(character) => {
        search.push(character);
      }
      _ => (),
    }
    true
  }

  fn handle_filter_box_input(&mut self) -> bool {
    let key = self.key();
    let Some(ref mut filter) = self.table_mut().filter else {
      return false;
    };

    match key {
      _ if matches_key!(backspace, key) => {
        filter.pop();
      }
      Key::Char(character) => {
        filter.push(character);
      }
      _ => (),
    }
    true
  }

  fn handle_filter_key(&mut self) -> bool {
    let config = self.config();
    let current_route = self.current_route();

    if current_route != config.table_block {
      return false;
    }

    let Some(filtering_block) = config.filtering_block else {
      return false;
    };

    let filter = HorizontallyScrollableText::default();
    self.table_mut().filter = Some(filter);
    let app = self.app_mut();
    app.push_navigation_stack(filtering_block);
    app.ignore_special_keys_for_textbox_input = true;
    true
  }

  fn handle_search_key(&mut self) -> bool {
    let config = self.config();
    let current_route = self.current_route();

    if current_route != config.table_block {
      return false;
    }

    let Some(searching_block) = config.searching_block else {
      return false;
    };

    let search = HorizontallyScrollableText::default();
    self.table_mut().search = Some(search);
    let app = self.app_mut();
    app.push_navigation_stack(searching_block);
    app.ignore_special_keys_for_textbox_input = true;
    true
  }

  fn handle_sort_key(&mut self) -> bool {
    let config = self.config();
    let current_route = self.current_route();

    if current_route != config.table_block {
      return false;
    }

    let (Some(sorting_block), Some(sort_options)) =
      (config.sorting_block, config.sort_options.as_ref())
    else {
      return false;
    };

    let sort_options = sort_options.clone();
    self.table_mut().sorting(sort_options);
    self.app_mut().push_navigation_stack(sorting_block);
    true
  }
}

/// Adapter struct that implements `TableEventHandler` for any `KeyEventHandler`.
///
/// This struct enables table handling for existing handlers via composition rather than
/// inheritance. It wraps a handler reference and uses a closure to access the table,
/// allowing flexible access patterns (direct, optional, nested).
///
/// # Type Parameters
///
/// - `'a`, `'b`: Lifetimes from the handler
/// - `'handler`: Lifetime of the handler reference
/// - `T`: The table row type
/// - `H`: The handler type that implements `KeyEventHandler`
/// - `R`: The route type that converts into `Route`
///
/// # Usage
///
/// This struct is typically created by the `handle_table` helper function and should
/// not be constructed directly:
///
/// ```rust,ignore
/// if !handle_table(self, |h| &mut h.app.data.my_table, config) {
///   self.handle_key_event();
/// }
/// ```
pub struct TableHandlerAdapter<'handler, 'a, 'b, T, H, R, F>
where
  T: Clone + PartialEq + Eq + Debug + Default,
  H: crate::handlers::KeyEventHandler<'a, 'b, R>,
  R: Into<Route> + Copy,
  F: for<'c> FnMut(&'c mut App<'b>) -> &'c mut StatefulTable<T>,
{
  handler: &'handler mut H,
  table_accessor: F,
  config: TableHandlingConfig<T>,
  _phantom: PhantomData<(&'a (), &'b (), R)>,
}

impl<'handler, 'a, 'b, T, H, R, F> TableHandlerAdapter<'handler, 'a, 'b, T, H, R, F>
where
  T: Clone + PartialEq + Eq + Debug + Default,
  H: crate::handlers::KeyEventHandler<'a, 'b, R>,
  R: Into<Route> + Copy,
  F: for<'c> FnMut(&'c mut App<'b>) -> &'c mut StatefulTable<T>,
{
  fn new(handler: &'handler mut H, table_accessor: F, config: TableHandlingConfig<T>) -> Self {
    Self {
      handler,
      table_accessor,
      config,
      _phantom: PhantomData,
    }
  }
}

impl<'handler, 'a, 'b, T, H, R, F> TableEventHandler<'b, T>
  for TableHandlerAdapter<'handler, 'a, 'b, T, H, R, F>
where
  T: Clone + PartialEq + Eq + Debug + Default,
  H: crate::handlers::KeyEventHandler<'a, 'b, R>,
  R: Into<Route> + Copy,
  F: for<'c> FnMut(&'c mut App<'b>) -> &'c mut StatefulTable<T>,
{
  fn table_mut(&mut self) -> &mut StatefulTable<T> {
    (self.table_accessor)(self.handler.app_mut())
  }

  fn config(&self) -> &TableHandlingConfig<T> {
    &self.config
  }

  fn is_ready(&self) -> bool {
    self.handler.is_ready()
  }

  fn current_route(&self) -> Route {
    self.handler.current_route()
  }

  fn ignore_special_keys(&self) -> bool {
    self.handler.ignore_special_keys()
  }

  fn app_mut(&mut self) -> &mut App<'b> {
    self.handler.app_mut()
  }

  fn key(&self) -> Key {
    self.handler.get_key()
  }
}

/// Helper function for ergonomic table event handling.
///
/// This is the primary entry point for using trait-based table handling. It creates
/// a `TableHandlerAdapter`, calls `handle_table_events()`, and returns the result.
///
/// # Parameters
///
/// - `handler`: Mutable reference to a handler implementing `KeyEventHandler`
/// - `table_accessor`: Closure that extracts the table from the handler
/// - `config`: Table handling configuration
///
/// # Returns
///
/// `true` if the event was handled by table logic, `false` otherwise.
/// When `false` is returned, the caller should delegate to other event handlers.
///
/// # Examples
///
/// ## Single Table Handler
///
/// ```rust,ignore
/// use crate::handlers::table_handler::{handle_table, TableHandlingConfig};
///
/// impl KeyEventHandler for LibraryHandler {
///   fn handle(&mut self) {
///     let config = TableHandlingConfig::new(ActiveBlock::Movies.into())
///       .sorting_block(ActiveBlock::MoviesSortPrompt.into())
///       .sort_options(movies_sorting_options())
///       .searching_block(ActiveBlock::SearchMovie.into())
///       .search_field_fn(|movie| &movie.title.text)
///       .filtering_block(ActiveBlock::FilterMovies.into())
///       .filter_field_fn(|movie| &movie.title.text);
///
///     if !handle_table(self, |h| &mut h.app.data.radarr_data.movies, config) {
///       // Event not handled by table, delegate to other handlers
///       match self.active_block {
///         _ if SubHandler::accepts(self.active_block) => {
///           SubHandler::new(self.key, self.app, self.active_block, self.context).handle();
///         }
///         _ => self.handle_key_event(),
///       }
///     }
///   }
/// }
/// ```
///
/// ## Multiple Tables Handler
///
/// ```rust,ignore
/// fn handle(&mut self) {
///   let releases_config = TableHandlingConfig::new(ActiveBlock::Releases.into());
///   let history_config = TableHandlingConfig::new(ActiveBlock::History.into());
///
///   // Short-circuit evaluation: try each table in sequence
///   if !handle_table(self, |h| &mut h.app.data.movie_releases, releases_config)
///     && !handle_table(self, |h| &mut h.app.data.movie_history, history_config)
///   {
///     self.handle_key_event();
///   }
/// }
/// ```
///
/// ## Optional Table Access
///
/// ```rust,ignore
/// fn handle(&mut self) {
///   let config = TableHandlingConfig::new(ActiveBlock::SearchResults.into());
///
///   if !handle_table(
///     self,
///     |h| h.app.data.add_searched_movies.as_mut().expect("modal should be initialized"),
///     config,
///   ) {
///     self.handle_key_event();
///   }
/// }
/// ```
pub fn handle_table<'a, 'b, T, H, R, F>(
  handler: &mut H,
  table_accessor: F,
  config: TableHandlingConfig<T>,
) -> bool
where
  T: Clone + PartialEq + Eq + Debug + Default,
  H: crate::handlers::KeyEventHandler<'a, 'b, R>,
  R: Into<Route> + Copy,
  F: for<'c> FnMut(&'c mut App<'b>) -> &'c mut StatefulTable<T>,
{
  let mut adapter = TableHandlerAdapter::new(handler, table_accessor, config);
  adapter.handle_table_events()
}
