use crate::models::Route;
use crate::models::stateful_table::SortOption;
use derive_setters::Setters;
use std::fmt::Debug;

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

#[macro_export]
macro_rules! handle_table_events {
  ($self:expr, $name:ty, $table:expr, $row:ident) => {
    paste::paste! {
      fn [<handle_ $name _table_events>](&mut $self, config: $crate::handlers::table_handler::TableHandlingConfig<$row>) -> bool {
        if $self.is_ready() {
          match $self.key {
            _ if $crate::matches_key!(up, $self.key, $self.ignore_special_keys()) => $self.[<handle_ $name _table_scroll_up>](config),
            _ if $crate::matches_key!(down, $self.key, $self.ignore_special_keys()) => $self.[<handle_ $name _table_scroll_down>](config),
            _ if $crate::matches_key!(pg_up, $self.key, $self.ignore_special_keys()) => $self.[<handle_ $name _table_page_up>](config),
            _ if $crate::matches_key!(pg_down, $self.key, $self.ignore_special_keys()) => $self.[<handle_ $name _table_page_down>](config),
            _ if $crate::matches_key!(home, $self.key) => $self.[<handle_ $name _table_home>](config),
            _ if $crate::matches_key!(end, $self.key) => $self.[<handle_ $name _table_end>](config),
            _ if $crate::matches_key!(left, $self.key, $self.ignore_special_keys())
              || $crate::matches_key!(right, $self.key, $self.ignore_special_keys()) =>
            {
              $self.[<handle_ $name _table_left_right>](config)
            }
            _ if $crate::matches_key!(submit, $self.key) => $self.[<handle_ $name _table_submit>](config),
            _ if $crate::matches_key!(esc, $self.key) => $self.[<handle_ $name _table_esc>](config),
            _ if config.searching_block.is_some()
              && $self.app.get_current_route() == *config.searching_block.as_ref().unwrap() =>
            {
              $self.[<handle_ $name _table_search_box_input>]()
            }
            _ if config.filtering_block.is_some()
              && $self.app.get_current_route() == *config.filtering_block.as_ref().unwrap() =>
            {
              $self.[<handle_ $name _table_filter_box_input>]()
            }
            _ if $crate::matches_key!(filter, $self.key)
              && config.filtering_block.is_some() => $self.[<handle_ $name _table_filter_key>](config),
            _ if $crate::matches_key!(search, $self.key)
              && config.searching_block.is_some() => $self.[<handle_ $name _table_search_key>](config),
            _ if $crate::matches_key!(sort, $self.key)
              && config.sorting_block.is_some() => $self.[<handle_ $name _table_sort_key>](config),
            _ => false,
          }
        } else {
          false
        }
      }

      fn [<handle_ $name _table_scroll_up>](&mut $self, config: $crate::handlers::table_handler::TableHandlingConfig<$row>) -> bool {
        use $crate::models::Scrollable;

        match $self.app.get_current_route() {
          _ if config.table_block == $self.app.get_current_route() => {
            $table.scroll_up();
            true
          }
          _ if config.sorting_block.is_some()
            && $self.app.get_current_route() == *config.sorting_block.as_ref().unwrap() =>
          {
            if let Some(ref mut sort) = $table.sort {
              sort.scroll_up();
            }
            true
          }
          _ => false,
        }
      }

      fn [<handle_ $name _table_scroll_down>](&mut $self, config: $crate::handlers::table_handler::TableHandlingConfig<$row>) -> bool {
        use $crate::models::Scrollable;

        match $self.app.get_current_route() {
          _ if config.table_block == $self.app.get_current_route() => {
            $table.scroll_down();
            true
          }
          _ if config.sorting_block.is_some()
            && $self.app.get_current_route() == *config.sorting_block.as_ref().unwrap() =>
          {
            if let Some(ref mut sort) = $table.sort {
              sort
                .scroll_down();
            }
            true
          }
          _ => false,
        }
      }

      fn [<handle_ $name _table_page_up>](&mut $self, config: $crate::handlers::table_handler::TableHandlingConfig<$row>) -> bool {
        use $crate::models::Paginated;

        if config.table_block == $self.app.get_current_route() {
          $table.page_up();
          true
        } else {
          false
        }
      }

      fn [<handle_ $name _table_page_down>](&mut $self, config: $crate::handlers::table_handler::TableHandlingConfig<$row>) -> bool {
        use $crate::models::Paginated;

        if config.table_block == $self.app.get_current_route() {
          $table.page_down();
          true
        } else {
          false
        }
      }

      fn [<handle_ $name _table_home>](&mut $self, config: $crate::handlers::table_handler::TableHandlingConfig<$row>) -> bool {
        use $crate::models::Scrollable;

        match $self.app.get_current_route() {
          _ if config.table_block == $self.app.get_current_route() => {
            $table.scroll_to_top();
            true
          }
          _ if config.sorting_block.is_some()
            && $self.app.get_current_route() == *config.sorting_block.as_ref().unwrap() =>
          {
            if let Some(ref mut sort) = $table.sort {
              sort.scroll_to_top();
            }
            true
          }
          _ if config.searching_block.is_some()
            && $self.app.get_current_route() == *config.searching_block.as_ref().unwrap() =>
          {
            if let Some(ref mut search) = $table.search {
              search.scroll_home();
            }
            true
          }
          _ if config.filtering_block.is_some()
            && $self.app.get_current_route() == *config.filtering_block.as_ref().unwrap() =>
          {
            if let Some(ref mut filter) = $table.filter {
              filter.scroll_home();
            }
            true
          }
          _ => false,
        }
      }

      fn [<handle_ $name _table_end>](&mut $self, config: $crate::handlers::table_handler::TableHandlingConfig<$row>) -> bool {
        use $crate::models::Scrollable;

        match $self.app.get_current_route() {
          _ if config.table_block == $self.app.get_current_route() => {
            $table.scroll_to_bottom();
            true
          }
          _ if config.sorting_block.is_some()
            && $self.app.get_current_route() == *config.sorting_block.as_ref().unwrap() =>
          {
            if let Some(ref mut sort) = $table.sort {
              sort.scroll_to_bottom();
            }
            true
          }
          _ if config.searching_block.is_some()
            && $self.app.get_current_route() == *config.searching_block.as_ref().unwrap() =>
          {
            if let Some(ref mut search) = $table.search {
              search.reset_offset();
            }
            true
          }
          _ if config.filtering_block.is_some()
            && $self.app.get_current_route() == *config.filtering_block.as_ref().unwrap() =>
          {
            if let Some(ref mut filter) = $table.filter {
              filter.reset_offset();
            }
            true
          }
          _ => false,
        }
      }

      fn [<handle_ $name _table_left_right>](&mut $self, config: $crate::handlers::table_handler::TableHandlingConfig<$row>) -> bool {
        match $self.app.get_current_route() {
          _ if config.searching_block.is_some()
            && $self.app.get_current_route() == *config.searching_block.as_ref().unwrap() =>
          {
            if let Some(ref mut search) = $table.search {
              $crate::handle_text_box_left_right_keys!($self, $self.key, search);
            }
            true
          }
          _ if config.filtering_block.is_some()
            && $self.app.get_current_route() == *config.filtering_block.as_ref().unwrap() =>
          {
            if let Some(ref mut filter) = $table.filter {
              $crate::handle_text_box_left_right_keys!($self, $self.key, filter);
            }
            true
          }
          _ => false,
        }
      }

      fn [<handle_ $name _table_submit>](&mut $self, config: $crate::handlers::table_handler::TableHandlingConfig<$row>) -> bool {
        match $self.app.get_current_route() {
          _ if config.sorting_block.is_some()
            && $self.app.get_current_route() == *config.sorting_block.as_ref().unwrap() =>
          {
            $table.apply_sorting();
            $self.app.pop_navigation_stack();
            true
          }
          _ if config.searching_block.is_some()
            && $self.app.get_current_route() == *config.searching_block.as_ref().unwrap() =>
          {
            $self.app.pop_navigation_stack();
            $self.app.ignore_special_keys_for_textbox_input = false;

            if $table.search.is_some() {
              let search_field_fn = config
                .search_field_fn
                .expect("Search field function is required");
              let has_match = $table.apply_search(search_field_fn);

              if !has_match {
                $self.app.push_navigation_stack(
                  config
                    .search_error_block
                    .expect("Search error block is undefined"),
                );
              }
            }

            true
          }
          _ if config.filtering_block.is_some()
            && $self.app.get_current_route() == *config.filtering_block.as_ref().unwrap() =>
          {
            $self.app.pop_navigation_stack();
            $self.app.ignore_special_keys_for_textbox_input = false;

            if $table.filter.is_some() {
              let filter_field_fn = config
                .filter_field_fn
                .expect("Search field function is required");
              let has_match = $table.apply_filter(filter_field_fn);

              if !has_match {
                $self.app.push_navigation_stack(
                  config
                    .filter_error_block
                    .expect("Search error block is undefined"),
                );
              }
            }

            true
          }
          _ => false,
        }
      }

      fn [<handle_ $name _table_esc>](&mut $self, config: $crate::handlers::table_handler::TableHandlingConfig<$row>) -> bool {
        match $self.app.get_current_route() {
          _ if config.sorting_block.is_some()
            && $self.app.get_current_route() == *config.sorting_block.as_ref().unwrap() =>
          {
            $self.app.pop_navigation_stack();
            true
          }
          _ if (config.searching_block.is_some()
            && $self.app.get_current_route() == *config.searching_block.as_ref().unwrap())
            || (config.search_error_block.is_some()
              && $self.app.get_current_route() == *config.search_error_block.as_ref().unwrap()) =>
          {
            $self.app.pop_navigation_stack();
            $table.reset_search();
            $self.app.ignore_special_keys_for_textbox_input = false;
            true
          }
          _ if (config.filtering_block.is_some()
            && $self.app.get_current_route() == *config.filtering_block.as_ref().unwrap())
            || (config.filter_error_block.is_some()
              && $self.app.get_current_route() == *config.filter_error_block.as_ref().unwrap()) =>
          {
            $self.app.pop_navigation_stack();
            $table.reset_filter();
            $self.app.ignore_special_keys_for_textbox_input = false;
            true
          }
          _ if config.table_block == $self.app.get_current_route()
            && $table.filtered_items.is_some() =>
          {
            $table.reset_filter();
            true
          }
          _ => false,
        }
      }

      fn [<handle_ $name _table_search_box_input>](&mut $self) -> bool {
        let Some(ref mut search) = $table.search else {
          return false;
        };

        $crate::handle_text_box_keys!($self, $self.key, search);
        true
      }

      fn [<handle_ $name _table_filter_box_input>](&mut $self) -> bool {
        let Some(ref mut filter) = $table.filter else {
          return false;
        };

        $crate::handle_text_box_keys!($self, $self.key, filter);
        true
      }

      fn [<handle_ $name _table_filter_key>](&mut $self, config: $crate::handlers::table_handler::TableHandlingConfig<$row>) -> bool {
				if $self.app.get_current_route() != config.table_block {
					return false;
				}

        let Some(ref filtering_block) = config.filtering_block else {
          return false;
        };

        let filter = $crate::models::HorizontallyScrollableText::default();
        $table.filter = Some(filter);
        $self.app.push_navigation_stack(*filtering_block);
        $self.app.ignore_special_keys_for_textbox_input = true;
        true
      }

      fn [<handle_ $name _table_search_key>](&mut $self, config: $crate::handlers::table_handler::TableHandlingConfig<$row>) -> bool {
				if $self.app.get_current_route() != config.table_block {
					return false;
				}

        let Some(ref searching_block) = config.searching_block else {
          return false;
        };

        let search = $crate::models::HorizontallyScrollableText::default();
        $table.search = Some(search);
        $self.app.push_navigation_stack(*searching_block);
        $self.app.ignore_special_keys_for_textbox_input = true;
        true
      }

      fn [<handle_ $name _table_sort_key>](&mut $self, config: $crate::handlers::table_handler::TableHandlingConfig<$row>) -> bool {
				if $self.app.get_current_route() != config.table_block {
					return false;
				}

        let (Some(ref sorting_block), Some(sort_options)) = (config.sorting_block, config.sort_options.as_ref()) else {
          return false;
        };

        $table.sorting(sort_options.clone());
        $self.app.push_navigation_stack(*sorting_block);
        true
      }
    }
  };
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
