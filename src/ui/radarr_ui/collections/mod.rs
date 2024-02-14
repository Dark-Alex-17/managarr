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
use crate::ui::styles::ManagarrStyle;
use crate::ui::utils::{get_width_from_percentage, layout_block_top_border};
use crate::ui::widgets::error_message::ErrorMessage;
use crate::ui::widgets::managarr_table::ManagarrTable;
use crate::ui::widgets::popup::{Popup, Size};
use crate::ui::{draw_input_box_popup, draw_popup_over, draw_prompt_box, DrawUi};

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

  fn draw(f: &mut Frame<'_>, app: &mut App<'_>, area: Rect) {
    let route = *app.get_current_route();
    let mut collections_ui_matcher = |active_radarr_block| match active_radarr_block {
      ActiveRadarrBlock::Collections => draw_collections(f, app, area),
      ActiveRadarrBlock::SearchCollection => draw_popup_over(
        f,
        app,
        area,
        draw_collections,
        draw_collection_search_box,
        Size::InputBox,
      ),
      ActiveRadarrBlock::SearchCollectionError => {
        draw_collections(f, app, area);
        let popup = Popup::new(ErrorMessage::new("Collection not found!")).size(Size::Error);
        f.render_widget(popup, f.size());
      }
      ActiveRadarrBlock::FilterCollections => draw_popup_over(
        f,
        app,
        area,
        draw_collections,
        draw_filter_collections_box,
        Size::InputBox,
      ),
      ActiveRadarrBlock::FilterCollectionsError => {
        draw_collections(f, app, area);
        let popup = Popup::new(ErrorMessage::new(
          "No collections found matching the given filter!",
        ))
        .size(Size::Error);
        f.render_widget(popup, f.size());
      }
      ActiveRadarrBlock::UpdateAllCollectionsPrompt => draw_popup_over(
        f,
        app,
        area,
        draw_collections,
        draw_update_all_collections_prompt,
        Size::Prompt,
      ),
      _ => (),
    };

    match route {
      _ if CollectionDetailsUi::accepts(route) => CollectionDetailsUi::draw(f, app, area),
      _ if EditCollectionUi::accepts(route) => EditCollectionUi::draw(f, app, area),
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
  let current_selection = if !app.data.radarr_data.collections.items.is_empty() {
    app.data.radarr_data.collections.current_selection().clone()
  } else {
    Collection::default()
  };
  let quality_profile_map = &app.data.radarr_data.quality_profile_map;
  let content = Some(&mut app.data.radarr_data.collections);
  let collections_table_footer = app
    .data
    .radarr_data
    .main_tabs
    .get_active_tab_contextual_help();
  let collection_row_mapping = |collection: &Collection| {
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
    ])
    .primary()
  };
  let collections_table = ManagarrTable::new(content, collection_row_mapping)
    .loading(app.is_loading)
    .footer(collections_table_footer)
    .block(layout_block_top_border())
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

  f.render_widget(collections_table, area);
}

fn draw_update_all_collections_prompt(f: &mut Frame<'_>, app: &mut App<'_>, area: Rect) {
  draw_prompt_box(
    f,
    area,
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
    app.data.radarr_data.collections.search.as_ref().unwrap(),
  );
}

fn draw_filter_collections_box(f: &mut Frame<'_>, app: &mut App<'_>, area: Rect) {
  draw_input_box_popup(
    f,
    area,
    "Filter",
    app.data.radarr_data.collections.filter.as_ref().unwrap(),
  )
}
