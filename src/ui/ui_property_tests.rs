#[cfg(test)]
mod ui_invariant_tests {
  use proptest::prelude::*;
  use ratatui::Terminal;
  use ratatui::backend::TestBackend;
  use ratatui::layout::Rect;

  use crate::app::App;
  use crate::models::radarr_models::Movie;
  use crate::models::servarr_data::radarr::radarr_data::ActiveRadarrBlock;
  use crate::models::sonarr_models::Series;
  use crate::models::{Scrollable, stateful_table::StatefulTable};
  use crate::ui::DrawUi;
  use crate::ui::radarr_ui::RadarrUi;
  use crate::ui::sonarr_ui::SonarrUi;
  use prop::bool::ANY;

  proptest! {
    #[test]
    fn test_radarr_library_never_panics_on_large_datasets(
      num_movies in 0usize..500,
      viewport_height in 10u16..100,
    ) {
      let backend = TestBackend::new(120, viewport_height);
      let mut terminal = Terminal::new(backend).unwrap();
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveRadarrBlock::Movies.into());

      let mut quality_profile_map = bimap::BiMap::new();
      quality_profile_map.insert(0, "Any".to_string());
      app.data.radarr_data.quality_profile_map = quality_profile_map;

      let movies: Vec<Movie> = (0..num_movies)
        .map(|i| Movie {
          id: i as i64,
          title: format!("Movie {}", i).into(),
          ..Movie::default()
        })
        .collect();

      let mut table = StatefulTable::default();
      table.set_items(movies);
      app.data.radarr_data.movies = table;

      terminal
        .draw(|f| {
          RadarrUi::draw(f, &mut app, f.area());
        })
        .unwrap();

      let buffer = terminal.backend().buffer();
      prop_assert!(buffer.area().height <= viewport_height);
      prop_assert!(buffer.area().width <= 120);
    }

    #[test]
    fn test_sonarr_library_never_panics_on_large_datasets(
      num_series in 0usize..500,
      viewport_height in 10u16..100,
    ) {
      let backend = TestBackend::new(120, viewport_height);
      let mut terminal = Terminal::new(backend).unwrap();
      let mut app = App::test_default();
      app.push_navigation_stack(
        crate::models::servarr_data::sonarr::sonarr_data::ActiveSonarrBlock::Series.into()
      );

      let mut language_profiles_map = bimap::BiMap::new();
      language_profiles_map.insert(0, "English".to_string());
      app.data.sonarr_data.language_profiles_map = language_profiles_map;

      let mut quality_profile_map = bimap::BiMap::new();
      quality_profile_map.insert(0, "Any".to_string());
      app.data.sonarr_data.quality_profile_map = quality_profile_map;

      let series: Vec<Series> = (0..num_series)
        .map(|i| Series {
          id: i as i64,
          title: format!("Series {}", i).into(),
          ..Series::default()
        })
        .collect();

      let mut table = StatefulTable::default();
      table.set_items(series);
      app.data.sonarr_data.series = table;

      terminal
        .draw(|f| {
          SonarrUi::draw(f, &mut app, f.area());
        })
        .unwrap();

      let buffer = terminal.backend().buffer();
      prop_assert!(buffer.area().height <= viewport_height);
      prop_assert!(buffer.area().width <= 120);
    }

    #[test]
    fn test_ui_respects_viewport_boundaries(
      viewport_width in 40u16..200,
      viewport_height in 10u16..100,
    ) {
      let backend = TestBackend::new(viewport_width, viewport_height);
      let mut terminal = Terminal::new(backend).unwrap();
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveRadarrBlock::Movies.into());

      let mut quality_profile_map = bimap::BiMap::new();
      quality_profile_map.insert(0, "Any".to_string());
      app.data.radarr_data.quality_profile_map = quality_profile_map;

      let movies = vec![Movie {
        id: 1,
        title: "Very Long Movie Title That Should Be Scrollable And Not Overflow".into(),
        ..Movie::default()
      }];

      let mut table = StatefulTable::default();
      table.set_items(movies);
      app.data.radarr_data.movies = table;

      terminal
        .draw(|f| {
          RadarrUi::draw(f, &mut app, f.area());
        })
        .unwrap();

      let buffer = terminal.backend().buffer();
      prop_assert_eq!(buffer.area().width, viewport_width);
      prop_assert_eq!(buffer.area().height, viewport_height);
    }

    #[test]
    fn test_centered_rect_never_exceeds_parent(
      parent_width in 20u16..200,
      parent_height in 10u16..100,
      percent_x in 1u16..=100,
      percent_y in 1u16..=100,
    ) {
      use crate::ui::utils::centered_rect;

      let parent = Rect {
        x: 0,
        y: 0,
        width: parent_width,
        height: parent_height,
      };

      let centered = centered_rect(percent_x, percent_y, parent);

      prop_assert!(centered.x >= parent.x);
      prop_assert!(centered.y >= parent.y);
      prop_assert!(centered.x + centered.width <= parent.x + parent.width);
      prop_assert!(centered.y + centered.height <= parent.y + parent.height);
    }

    #[test]
    fn test_table_navigation_stays_in_bounds(
      num_items in 1usize..100,
      num_scrolls in 0usize..200,
    ) {
      let mut table = StatefulTable::default();
      let movies: Vec<Movie> = (0..num_items)
        .map(|i| Movie {
          id: i as i64,
          title: format!("Movie {}", i).into(),
          ..Movie::default()
        })
        .collect();

      table.set_items(movies);

      for _ in 0..num_scrolls {
        table.scroll_down();
      }

      let current_item = table.current_selection();
      prop_assert!(current_item.id >= 0 && (current_item.id as usize) < num_items);

      for _ in 0..num_scrolls {
        table.scroll_up();
      }

      let current_item_after = table.current_selection();
      prop_assert!(current_item_after.id >= 0 && (current_item_after.id as usize) < num_items);
    }

    #[test]
    fn test_empty_tables_handle_navigation_gracefully(
      num_scroll_attempts in 0usize..50,
    ) {
      let mut table = StatefulTable::<Movie>::default();

      for _ in 0..num_scroll_attempts {
        table.scroll_down();
        table.scroll_up();
      }

      prop_assert!(true);
    }

    #[test]
    fn test_loading_state_never_panics(
      is_loading in ANY,
      num_items in 0usize..100,
    ) {
      let backend = TestBackend::new(120, 30);
      let mut terminal = Terminal::new(backend).unwrap();
      let mut app = App::test_default();
      app.is_loading = is_loading;
      app.push_navigation_stack(ActiveRadarrBlock::Movies.into());

      let mut quality_profile_map = bimap::BiMap::new();
      quality_profile_map.insert(0, "Any".to_string());
      app.data.radarr_data.quality_profile_map = quality_profile_map;

      let movies: Vec<Movie> = (0..num_items)
        .map(|i| Movie {
          id: i as i64,
          title: format!("Movie {}", i).into(),
          ..Movie::default()
        })
        .collect();

      let mut table = StatefulTable::default();
      table.set_items(movies);
      app.data.radarr_data.movies = table;

      terminal
        .draw(|f| {
          RadarrUi::draw(f, &mut app, f.area());
        })
        .unwrap();

      prop_assert!(true);
    }

    #[test]
    fn test_navigation_stack_maintains_route_integrity(
      stack_depth in 1usize..10,
    ) {
      let mut app = App::test_default();

      let blocks = [ActiveRadarrBlock::Movies,
        ActiveRadarrBlock::Collections,
        ActiveRadarrBlock::Downloads,
        ActiveRadarrBlock::Blocklist,
        ActiveRadarrBlock::RootFolders];

      for i in 0..stack_depth {
        let block = blocks[i % blocks.len()];
        app.push_navigation_stack(block.into());
      }

      let current_route = app.get_current_route();
      let expected_block = blocks[(stack_depth - 1) % blocks.len()];
      prop_assert!(matches!(current_route, crate::models::Route::Radarr(block, _) if block == expected_block));

      for _ in 0..stack_depth {
        app.pop_navigation_stack();
      }

      let final_route = app.get_current_route();
      prop_assert!(matches!(final_route, crate::models::Route::Radarr(_, _)));
    }
  }
}
