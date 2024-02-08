use ratatui::layout::{Constraint, Rect};
use ratatui::widgets::{Cell, Row};
use ratatui::Frame;

pub(super) use collection_details_ui::draw_collection_details;

use crate::app::App;
use crate::models::radarr_models::Collection;
use crate::models::servarr_data::radarr::radarr_data::{ActiveRadarrBlock, COLLECTIONS_BLOCKS};
use crate::models::Route;
use crate::ui::radarr_ui::collections::collection_details_ui::CollectionDetailsUi;
use crate::ui::radarr_ui::collections::edit_collection_ui::EditCollectionUi;
use crate::ui::utils::{get_width_from_percentage, layout_block_top_border};
use crate::ui::{
  draw_error_message_popup, draw_input_box_popup, draw_popup_over, draw_prompt_box,
  draw_prompt_popup_over, draw_table, DrawUi, TableProps,
};
use crate::ui::styles::ManagarrStyle;

mod collection_details_ui;
#[cfg(test)]
#[path = "collections_ui_tests.rs"]
mod collections_ui_tests;
mod edit_collection_ui;

pub(super) struct CollectionsUi;

impl DrawUi for CollectionsUi {
  fn accepts(route: Route) -> bool {
    if let Route::Radarr(active_radarr_block, _) = route {
      return CollectionDetailsUi::accepts(route)
        || EditCollectionUi::accepts(route)
        || COLLECTIONS_BLOCKS.contains(&active_radarr_block);
    }

    false
  }

  fn draw(f: &mut Frame<'_>, app: &mut App<'_>, content_rect: Rect) {
    let route = *app.get_current_route();
    let mut collections_ui_matcher = |active_radarr_block| match active_radarr_block {
      ActiveRadarrBlock::Collections => draw_collections(f, app, content_rect),
      ActiveRadarrBlock::SearchCollection => draw_popup_over(
        f,
        app,
        content_rect,
        draw_collections,
        draw_collection_search_box,
        30,
        13,
      ),
      ActiveRadarrBlock::SearchCollectionError => draw_popup_over(
        f,
        app,
        content_rect,
        draw_collections,
        draw_search_collection_error_box,
        30,
        8,
      ),
      ActiveRadarrBlock::FilterCollections => draw_popup_over(
        f,
        app,
        content_rect,
        draw_collections,
        draw_filter_collections_box,
        30,
        13,
      ),
      ActiveRadarrBlock::FilterCollectionsError => draw_popup_over(
        f,
        app,
        content_rect,
        draw_collections,
        draw_filter_collections_error_box,
        30,
        8,
      ),
      ActiveRadarrBlock::UpdateAllCollectionsPrompt => draw_prompt_popup_over(
        f,
        app,
        content_rect,
        draw_collections,
        draw_update_all_collections_prompt,
      ),
      _ => (),
    };

    match route {
      _ if CollectionDetailsUi::accepts(route) => CollectionDetailsUi::draw(f, app, content_rect),
      _ if EditCollectionUi::accepts(route) => EditCollectionUi::draw(f, app, content_rect),
      Route::Radarr(active_radarr_block, _)
        if COLLECTIONS_BLOCKS.contains(&active_radarr_block) =>
      {
        collections_ui_matcher(active_radarr_block)
      }
      _ => (),
    }
  }
}

pub(super) fn draw_collections(f: &mut Frame<'_>, app: &mut App<'_>, area: Rect) {
  let current_selection =
    if let Some(filtered_collections) = app.data.radarr_data.filtered_collections.as_ref() {
      filtered_collections.current_selection().clone()
    } else if !app.data.radarr_data.collections.items.is_empty() {
      app.data.radarr_data.collections.current_selection().clone()
    } else {
      Collection::default()
    };
  let quality_profile_map = &app.data.radarr_data.quality_profile_map;
  let content = match app.data.radarr_data.filtered_collections.as_mut() {
    Some(filtered_collections) if !app.data.radarr_data.is_filtering => Some(filtered_collections),
    _ => Some(&mut app.data.radarr_data.collections),
  };
  draw_table(
    f,
    area,
    layout_block_top_border(),
    TableProps {
      content,
      wrapped_content: None,
      table_headers: vec![
        "Collection",
        "Number of Movies",
        "Root Folder Path",
        "Quality Profile",
        "Search on Add",
        "Monitored",
      ],
      constraints: vec![
        Constraint::Percentage(25),
        Constraint::Percentage(15),
        Constraint::Percentage(15),
        Constraint::Percentage(15),
        Constraint::Percentage(15),
        Constraint::Percentage(15),
      ],
      help: app
        .data
        .radarr_data
        .main_tabs
        .get_active_tab_contextual_help(),
    },
    |collection| {
      let number_of_movies = collection.movies.clone().unwrap_or_default().len();
      collection.title.scroll_left_or_reset(
        get_width_from_percentage(area, 25),
        *collection == current_selection,
        app.tick_count % app.ticks_until_scroll == 0,
      );
      let monitored = if collection.monitored { "🏷" } else { "" };
      let search_on_add = if collection.search_on_add {
        "Yes"
      } else {
        "No"
      };

      Row::new(vec![
        Cell::from(collection.title.to_string()),
        Cell::from(number_of_movies.to_string()),
        Cell::from(collection.root_folder_path.clone().unwrap_or_default()),
        Cell::from(
          quality_profile_map
            .get_by_left(&collection.quality_profile_id)
            .unwrap()
            .to_owned(),
        ),
        Cell::from(search_on_add),
        Cell::from(monitored),
      ]).primary()
    },
    app.is_loading,
    true,
  );
}

fn draw_update_all_collections_prompt(f: &mut Frame<'_>, app: &mut App<'_>, prompt_area: Rect) {
  draw_prompt_box(
    f,
    prompt_area,
    "Update All Collections",
    "Do you want to update all of your collections?",
    app.data.radarr_data.prompt_confirm,
  );
}

fn draw_collection_search_box(f: &mut Frame<'_>, app: &mut App<'_>, area: Rect) {
  draw_input_box_popup(
    f,
    area,
    "Search",
    app.data.radarr_data.search.as_ref().unwrap(),
  );
}

fn draw_filter_collections_box(f: &mut Frame<'_>, app: &mut App<'_>, area: Rect) {
  draw_input_box_popup(
    f,
    area,
    "Filter",
    app.data.radarr_data.filter.as_ref().unwrap(),
  )
}

fn draw_search_collection_error_box(f: &mut Frame<'_>, _: &mut App<'_>, area: Rect) {
  draw_error_message_popup(f, area, "Collection not found!");
}

fn draw_filter_collections_error_box(f: &mut Frame<'_>, _: &mut App<'_>, area: Rect) {
  draw_error_message_popup(f, area, "No collections found matching the given filter!");
}
