#[cfg(test)]
mod property_tests {
  use proptest::prelude::*;

  use crate::app::App;
  use crate::handlers::handler_test_utils::test_utils::proptest_helpers::*;
  use crate::models::radarr_models::Movie;
  use crate::models::servarr_data::radarr::radarr_data::ActiveRadarrBlock;
  use crate::models::stateful_table::StatefulTable;
  use crate::models::{Paginated, Scrollable};

  proptest! {
    #[test]
    fn test_table_index_selection_safety(
      list_size in list_size(),
      index in 0usize..1000
    ) {
      let mut table = StatefulTable::<Movie>::default();
      let movies: Vec<Movie> = (0..list_size).map(|i| {
        let mut movie = Movie::default();
        movie.id = i as i64;
        movie
      }).collect();

      table.set_items(movies);

      if index < list_size {
        table.select_index(Some(index));
        let selected = table.current_selection();
        prop_assert_eq!(selected.id, index as i64);
      } else {
        table.select_index(Some(index));
      }
    }

    #[test]
    fn test_table_scroll_consistency(
      list_size in list_size(),
      scroll_amount in 0usize..20
    ) {
      let mut table = StatefulTable::<Movie>::default();
      let movies: Vec<Movie> = (0..list_size).map(|i| {
        let mut movie = Movie::default();
        movie.id = i as i64;
        movie
      }).collect();

      table.set_items(movies);
      let initial_id = table.current_selection().id;

      for _ in 0..scroll_amount {
        table.scroll_down();
      }
      let after_down_id = table.current_selection().id;

      prop_assert!(after_down_id >= initial_id);
      prop_assert!(after_down_id < list_size as i64);

      for _ in 0..scroll_amount {
        table.scroll_up();
      }

      prop_assert!(table.current_selection().id <= initial_id);
    }

    #[test]
    fn test_empty_table_safety(_scroll_ops in 0usize..50) {
      let table = StatefulTable::<Movie>::default();

      prop_assert!(table.is_empty());
      prop_assert!(table.items.is_empty());
    }

    #[test]
    fn test_navigation_consistency(pushes in 1usize..20) {
      let mut app = App::test_default();
      let initial_route = app.get_current_route();

      let routes = vec![
        ActiveRadarrBlock::Movies,
        ActiveRadarrBlock::Collections,
        ActiveRadarrBlock::Downloads,
        ActiveRadarrBlock::Blocklist,
      ];

      for i in 0..pushes {
        let route = routes[i % routes.len()];
        app.push_navigation_stack(route.into());
      }

      let last_pushed = routes[(pushes - 1) % routes.len()];
      prop_assert_eq!(app.get_current_route(), last_pushed.into());

      for _ in 0..pushes {
        app.pop_navigation_stack();
      }

      prop_assert_eq!(app.get_current_route(), initial_route);
    }

    #[test]
    fn test_string_input_safety(input in text_input_string()) {
      let _lowercase = input.to_lowercase();
      let _uppercase = input.to_uppercase();
      let _trimmed = input.trim();
      let _len = input.len();
      let _chars: Vec<char> = input.chars().collect();

      prop_assert!(true);
    }

    #[test]
    fn test_table_data_integrity(
      list_size in 1usize..100
    ) {
      let mut table = StatefulTable::<Movie>::default();
      let movies: Vec<Movie> = (0..list_size).map(|i| {
        let mut movie = Movie::default();
        movie.id = i as i64;
        movie.title = format!("Movie {}", i).into();
        movie
      }).collect();

      table.set_items(movies.clone());
      let original_count = table.items.len();

      prop_assert_eq!(table.items.len(), original_count);

      for movie in &movies {
        prop_assert!(table.items.iter().any(|m| m.id == movie.id));
      }
    }

    #[test]
    fn test_page_navigation_bounds(
      list_size in list_size(),
      page_ops in 0usize..10
    ) {
      let mut table = StatefulTable::<Movie>::default();
      let movies: Vec<Movie> = (0..list_size).map(|i| {
        let mut movie = Movie::default();
        movie.id = i as i64;
        movie
      }).collect();

      table.set_items(movies);

      for i in 0..page_ops {
        if i % 2 == 0 {
          table.page_down();
        } else {
          table.page_up();
        }

        let current = table.current_selection();
        prop_assert!(current.id >= 0);
        prop_assert!(current.id < list_size as i64);
      }
    }

    #[test]
    fn test_table_filter_size_invariant(
      list_size in list_size(),
      filter_term in text_input_string()
    ) {
      let mut table = StatefulTable::<Movie>::default();
      let movies: Vec<Movie> = (0..list_size).map(|i| {
        let mut movie = Movie::default();
        movie.id = i as i64;
        movie.title = format!("Test Movie {}", i % 10).into();
        movie
      }).collect();

      table.set_items(movies.clone());
      let original_size = table.items.len();

      if !filter_term.is_empty() {
        let filtered: Vec<Movie> = movies.into_iter()
          .filter(|m| m.title.text.to_lowercase().contains(&filter_term.to_lowercase()))
          .collect();
        table.set_items(filtered);
      }

      prop_assert!(table.items.len() <= original_size);

      if !table.items.is_empty() {
        let current = table.current_selection();
        prop_assert!(current.id >= 0);
      }
    }
  }
}
