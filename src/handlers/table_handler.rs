use crate::models::stateful_table::SortOption;
use crate::models::Route;
use derive_setters::Setters;
use std::cmp::Ordering;
use std::fmt::Debug;

#[derive(Setters)]
pub struct TableHandlingProps<T>
where
  T: Clone + PartialEq + Eq + Debug + Default,
{
  #[setters(strip_option)]
  pub sorting_block: Option<Route>,
  #[setters(strip_option)]
  pub sort_options: Option<Vec<SortOption<T>>>,
  #[setters(strip_option)]
  pub sort_by_fn: Option<fn(&T, &T) -> Ordering>,
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
      fn [<handle_ $name _table_events>](&mut $self, props: $crate::handlers::table_handler::TableHandlingProps<$row>) -> bool {
        if $self.is_ready() {
          match $self.key {
            _ if $self.key == $crate::app::key_binding::DEFAULT_KEYBINDINGS.up.key => $self.[<handle_ $name _table_scroll_up>](props),
            _ if $self.key == $crate::app::key_binding::DEFAULT_KEYBINDINGS.down.key => $self.[<handle_ $name _table_scroll_down>](props),
            _ if $self.key == $crate::app::key_binding::DEFAULT_KEYBINDINGS.home.key => $self.[<handle_ $name _table_home>](props),
            _ if $self.key == $crate::app::key_binding::DEFAULT_KEYBINDINGS.end.key => $self.[<handle_ $name _table_end>](props),
            _ if $self.key == $crate::app::key_binding::DEFAULT_KEYBINDINGS.left.key
              || $self.key == $crate::app::key_binding::DEFAULT_KEYBINDINGS.right.key =>
            {
              $self.[<handle_ $name _table_left_right>](props)
            }
            _ if $self.key == $crate::app::key_binding::DEFAULT_KEYBINDINGS.submit.key => $self.[<handle_ $name _table_submit>](props),
            _ if $self.key == $crate::app::key_binding::DEFAULT_KEYBINDINGS.esc.key => $self.[<handle_ $name _table_esc>](props),
            _ if props.searching_block.is_some()
              && $self.app.get_current_route() == *props.searching_block.as_ref().unwrap() =>
            {
              $self.[<handle_ $name _table_search_box_input>]()
            }
            _ if props.filtering_block.is_some()
              && $self.app.get_current_route() == *props.filtering_block.as_ref().unwrap() =>
            {
              $self.[<handle_ $name _table_filter_box_input>]()
            }
            _ if $self.key == $crate::app::key_binding::DEFAULT_KEYBINDINGS.filter.key
              && props.filtering_block.is_some() => $self.[<handle_ $name _table_filter_key>](props),
            _ if $self.key == $crate::app::key_binding::DEFAULT_KEYBINDINGS.search.key
              && props.searching_block.is_some() => $self.[<handle_ $name _table_search_key>](props),
            _ if $self.key == $crate::app::key_binding::DEFAULT_KEYBINDINGS.sort.key
              && props.sorting_block.is_some() => $self.[<handle_ $name _table_sort_key>](props),
            _ => false,
          }
        } else {
          false
        }
      }

      fn [<handle_ $name _table_scroll_up>](&mut $self, props: $crate::handlers::table_handler::TableHandlingProps<$row>) -> bool {
        match $self.app.get_current_route() {
          _ if props.table_block == $self.app.get_current_route() => {
            $table.scroll_up();
            true
          }
          _ if props.sorting_block.is_some()
            && $self.app.get_current_route() == *props.sorting_block.as_ref().unwrap() =>
          {
            $table.sort.as_mut().unwrap().scroll_up();
            true
          }
          _ => false,
        }
      }

      fn [<handle_ $name _table_scroll_down>](&mut $self, props: $crate::handlers::table_handler::TableHandlingProps<$row>) -> bool {
        match $self.app.get_current_route() {
          _ if props.table_block == $self.app.get_current_route() => {
            $table.scroll_down();
            true
          }
          _ if props.sorting_block.is_some()
            && $self.app.get_current_route() == *props.sorting_block.as_ref().unwrap() =>
          {
            $table
              .sort
              .as_mut()
              .unwrap()
              .scroll_down();
            true
          }
          _ => false,
        }
      }

      fn [<handle_ $name _table_home>](&mut $self, props: $crate::handlers::table_handler::TableHandlingProps<$row>) -> bool {
        match $self.app.get_current_route() {
          _ if props.table_block == $self.app.get_current_route() => {
            $table.scroll_to_top();
            true
          }
          _ if props.sorting_block.is_some()
            && $self.app.get_current_route() == *props.sorting_block.as_ref().unwrap() =>
          {
            $table
              .sort
              .as_mut()
              .unwrap()
              .scroll_to_top();
            true
          }
          _ if props.searching_block.is_some()
            && $self.app.get_current_route() == *props.searching_block.as_ref().unwrap() =>
          {
            $table
              .search
              .as_mut()
              .unwrap()
              .scroll_home();
            true
          }
          _ if props.filtering_block.is_some()
            && $self.app.get_current_route() == *props.filtering_block.as_ref().unwrap() =>
          {
            $table
              .filter
              .as_mut()
              .unwrap()
              .scroll_home();
            true
          }
          _ => false,
        }
      }

      fn [<handle_ $name _table_end>](&mut $self, props: $crate::handlers::table_handler::TableHandlingProps<$row>) -> bool {
        match $self.app.get_current_route() {
          _ if props.table_block == $self.app.get_current_route() => {
            $table.scroll_to_bottom();
            true
          }
          _ if props.sorting_block.is_some()
            && $self.app.get_current_route() == *props.sorting_block.as_ref().unwrap() =>
          {
            $table
              .sort
              .as_mut()
              .unwrap()
              .scroll_to_bottom();
            true
          }
          _ if props.searching_block.is_some()
            && $self.app.get_current_route() == *props.searching_block.as_ref().unwrap() =>
          {
            $table
              .search
              .as_mut()
              .unwrap()
              .reset_offset();
            true
          }
          _ if props.filtering_block.is_some()
            && $self.app.get_current_route() == *props.filtering_block.as_ref().unwrap() =>
          {
            $table
              .filter
              .as_mut()
              .unwrap()
              .reset_offset();
            true
          }
          _ => false,
        }
      }

      fn [<handle_ $name _table_left_right>](&mut $self, props: $crate::handlers::table_handler::TableHandlingProps<$row>) -> bool {
        match $self.app.get_current_route() {
          _ if props.searching_block.is_some()
            && $self.app.get_current_route() == *props.searching_block.as_ref().unwrap() =>
          {
            $crate::handle_text_box_left_right_keys!(
              $self,
              $self.key,
              $table.search.as_mut().unwrap()
            );
            true
          }
          _ if props.filtering_block.is_some()
            && $self.app.get_current_route() == *props.filtering_block.as_ref().unwrap() =>
          {
            $crate::handle_text_box_left_right_keys!(
              $self,
              $self.key,
              $table.filter.as_mut().unwrap()
            );
            true
          }
          _ => false,
        }
      }

      fn [<handle _$name _table_submit>](&mut $self, props: $crate::handlers::table_handler::TableHandlingProps<$row>) -> bool {
        match $self.app.get_current_route() {
          _ if props.sorting_block.is_some()
            && $self.app.get_current_route() == *props.sorting_block.as_ref().unwrap() =>
          {
            if let Some(sort_by_fn) = props.sort_by_fn {
              $table.items.sort_by(sort_by_fn);
            }

            $table.apply_sorting();
            $self.app.pop_navigation_stack();

            true
          }
          _ if props.searching_block.is_some()
            && $self.app.get_current_route() == *props.searching_block.as_ref().unwrap() =>
          {
            $self.app.pop_navigation_stack();
            $self.app.should_ignore_quit_key = false;

            if $table.search.is_some() {
              let search_field_fn = props
                .search_field_fn
                .expect("Search field function is required");
              let has_match = $table.apply_search(search_field_fn);

              if !has_match {
                $self.app.push_navigation_stack(
                  props
                    .search_error_block
                    .expect("Search error block is undefined"),
                );
              }
            }

            true
          }
          _ if props.filtering_block.is_some()
            && $self.app.get_current_route() == *props.filtering_block.as_ref().unwrap() =>
          {
            $self.app.pop_navigation_stack();
            $self.app.should_ignore_quit_key = false;

            if $table.filter.is_some() {
              let filter_field_fn = props
                .filter_field_fn
                .expect("Search field function is required");
              let has_match = $table.apply_filter(filter_field_fn);

              if !has_match {
                $self.app.push_navigation_stack(
                  props
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

      fn [<handle_ $name _table_esc>](&mut $self, props: $crate::handlers::table_handler::TableHandlingProps<$row>) -> bool {
        match $self.app.get_current_route() {
          _ if props.sorting_block.is_some()
            && $self.app.get_current_route() == *props.sorting_block.as_ref().unwrap() =>
          {
            $self.app.pop_navigation_stack();
            true
          }
          _ if (props.searching_block.is_some()
            && $self.app.get_current_route() == *props.searching_block.as_ref().unwrap())
            || (props.search_error_block.is_some()
              && $self.app.get_current_route() == *props.search_error_block.as_ref().unwrap()) =>
          {
            $self.app.pop_navigation_stack();
            $table.reset_search();
            $self.app.should_ignore_quit_key = false;
            true
          }
          _ if (props.filtering_block.is_some()
            && $self.app.get_current_route() == *props.filtering_block.as_ref().unwrap())
            || (props.filter_error_block.is_some()
              && $self.app.get_current_route() == *props.filter_error_block.as_ref().unwrap()) =>
          {
            $self.app.pop_navigation_stack();
            $table.reset_filter();
            $self.app.should_ignore_quit_key = false;
            true
          }
          _ if props.table_block == $self.app.get_current_route()
            && $table.filtered_items.is_some() =>
          {
            $table.reset_filter();
            true
          }
          _ => false,
        }
      }

      fn [<handle_ $name _table_filter_key>](&mut $self, props: $crate::handlers::table_handler::TableHandlingProps<$row>) -> bool {
        if matches!($self.app.get_current_route(), _ if props.table_block == $self.app.get_current_route()) {
          $self
            .app
            .push_navigation_stack(props.filtering_block.expect("Filtering block is undefined").into());
          $table.reset_filter();
          $table.filter = Some($crate::models::HorizontallyScrollableText::default());
          $self.app.should_ignore_quit_key = true;

          true
        } else {
          false
        }
      }

      fn [<handle_ $name _table_search_key>](&mut $self, props: $crate::handlers::table_handler::TableHandlingProps<$row>) -> bool {
        if matches!($self.app.get_current_route(), _ if props.table_block == $self.app.get_current_route()) {
          $self
            .app
            .push_navigation_stack(props.searching_block.expect("Searching block is undefined"));
          $table.search = Some($crate::models::HorizontallyScrollableText::default());
          $self.app.should_ignore_quit_key = true;

          true
        } else {
          false
        }
      }

      fn [<handle_ $name _table_sort_key>](&mut $self, props: $crate::handlers::table_handler::TableHandlingProps<$row>) -> bool {
        if matches!($self.app.get_current_route(), _ if props.table_block == $self.app.get_current_route()) {
          $table.sorting(
            props
              .sort_options
              .as_ref()
              .expect("Sort options are undefined")
              .clone(),
          );
          $self
            .app
            .push_navigation_stack(props.sorting_block.expect("Sorting block is undefined"));
          true
        } else {
          false
        }
      }

      fn [<handle_ $name _table_search_box_input>](&mut $self) -> bool {
        $crate::handle_text_box_keys!(
          $self,
          $self.key,
          $table.search.as_mut().unwrap()
        );
        true
      }

      fn [<handle_ $name _table_filter_box_input>](&mut $self) -> bool {
        $crate::handle_text_box_keys!(
          $self,
          $self.key,
          $table.filter.as_mut().unwrap()
        );
        true
      }
    }
  };
}

impl<T> TableHandlingProps<T>
where
  T: Clone + PartialEq + Eq + Debug + Default,
{
  pub fn new(table_block: Route) -> Self {
    TableHandlingProps {
      sorting_block: None,
      sort_options: None,
      sort_by_fn: None,
      searching_block: None,
      search_error_block: None,
      search_field_fn: None,
      filtering_block: None,
      filter_error_block: None,
      filter_field_fn: None,
      table_block,
    }
  }
}
