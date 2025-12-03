use ratatui::Frame;
use ratatui::layout::{Constraint, Rect};
use ratatui::widgets::{Cell, Row};

use crate::app::App;
use crate::models::Route;
use crate::models::radarr_models::Collection;
use crate::models::servarr_data::radarr::radarr_data::{ActiveRadarrBlock, COLLECTIONS_BLOCKS};
use crate::ui::DrawUi;
use crate::ui::radarr_ui::collections::collection_details_ui::CollectionDetailsUi;
use crate::ui::radarr_ui::collections::edit_collection_ui::EditCollectionUi;
use crate::ui::styles::ManagarrStyle;
use crate::ui::utils::{get_width_from_percentage, layout_block_top_border};
use crate::ui::widgets::confirmation_prompt::ConfirmationPrompt;
use crate::ui::widgets::managarr_table::ManagarrTable;
use crate::ui::widgets::popup::{Popup, Size};

mod collection_details_ui;
#[cfg(test)]
#[path = "collections_ui_tests.rs"]
mod collections_ui_tests;
mod edit_collection_ui;

pub(super) struct CollectionsUi;

impl DrawUi for CollectionsUi {
  fn accepts(route: Route) -> bool {
    let Route::Radarr(active_radarr_block, _) = route else {
      return false;
    };
    CollectionDetailsUi::accepts(route)
      || EditCollectionUi::accepts(route)
      || COLLECTIONS_BLOCKS.contains(&active_radarr_block)
  }

  fn draw(f: &mut Frame<'_>, app: &mut App<'_>, area: Rect) {
    let route = app.get_current_route();
    draw_collections(f, app, area);

    match route {
      _ if CollectionDetailsUi::accepts(route) => CollectionDetailsUi::draw(f, app, area),
      _ if EditCollectionUi::accepts(route) => EditCollectionUi::draw(f, app, area),
      Route::Radarr(ActiveRadarrBlock::UpdateAllCollectionsPrompt, _) => {
        let confirmation_prompt = ConfirmationPrompt::new()
          .title("Update All Collections")
          .prompt("Do you want to update all of your collections?")
          .yes_no_value(app.data.radarr_data.prompt_confirm);

        f.render_widget(
          Popup::new(confirmation_prompt).size(Size::MediumPrompt),
          f.area(),
        );
      }
      _ => (),
    }
  }
}

pub(super) fn draw_collections(f: &mut Frame<'_>, app: &mut App<'_>, area: Rect) {
  if let Route::Radarr(active_radarr_block, _) = app.get_current_route() {
    let current_selection = if !app.data.radarr_data.collections.items.is_empty() {
      app.data.radarr_data.collections.current_selection().clone()
    } else {
      Collection::default()
    };
    let quality_profile_map = &app.data.radarr_data.quality_profile_map;
    let content = Some(&mut app.data.radarr_data.collections);
    let collection_row_mapping = |collection: &Collection| {
      let number_of_movies = collection.movies.as_ref().unwrap_or(&Vec::new()).len();
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
        Cell::from(
          collection
            .root_folder_path
            .as_ref()
            .unwrap_or(&String::new())
            .to_owned(),
        ),
        Cell::from(
          quality_profile_map
            .get_by_left(&collection.quality_profile_id)
            .unwrap()
            .to_owned(),
        ),
        Cell::from(search_on_add),
        Cell::from(monitored),
      ])
      .primary()
    };
    let collections_table = ManagarrTable::new(content, collection_row_mapping)
      .loading(
        app.is_loading
          || app.data.radarr_data.movies.is_empty()
          || app.data.radarr_data.quality_profile_map.is_empty(),
      )
      .block(layout_block_top_border())
      .sorting(active_radarr_block == ActiveRadarrBlock::CollectionsSortPrompt)
      .searching(active_radarr_block == ActiveRadarrBlock::SearchCollection)
      .search_produced_empty_results(
        active_radarr_block == ActiveRadarrBlock::SearchCollectionError,
      )
      .filtering(active_radarr_block == ActiveRadarrBlock::FilterCollections)
      .filter_produced_empty_results(
        active_radarr_block == ActiveRadarrBlock::FilterCollectionsError,
      )
      .headers([
        "Collection",
        "Number of Movies",
        "Root Folder Path",
        "Quality Profile",
        "Search on Add",
        "Monitored",
      ])
      .constraints([
        Constraint::Percentage(25),
        Constraint::Percentage(15),
        Constraint::Percentage(15),
        Constraint::Percentage(15),
        Constraint::Percentage(15),
        Constraint::Percentage(15),
      ]);

    if [
      ActiveRadarrBlock::SearchCollection,
      ActiveRadarrBlock::FilterCollections,
    ]
    .contains(&active_radarr_block)
    {
      collections_table.show_cursor(f, area);
    }

    f.render_widget(collections_table, area);
  }
}
