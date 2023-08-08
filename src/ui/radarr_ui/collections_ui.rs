use tui::backend::Backend;
use tui::layout::{Constraint, Rect};
use tui::widgets::{Cell, Row};
use tui::Frame;

use crate::app::radarr::ActiveRadarrBlock;
use crate::app::App;
use crate::models::radarr_models::Collection;
use crate::models::Route;
use crate::ui::radarr_ui::{draw_filter_box, draw_search_box};
use crate::ui::utils::{get_width_from_percentage, layout_block_top_border, style_primary};
use crate::ui::{
  draw_popup_over, draw_prompt_box, draw_prompt_popup_over, draw_table, DrawUi, TableProps,
};

pub(super) struct CollectionsUi {}

impl DrawUi for CollectionsUi {
  fn draw<B: Backend>(f: &mut Frame<'_, B>, app: &mut App<'_>, content_rect: Rect) {
    if let Route::Radarr(active_radarr_block, _) = *app.get_current_route() {
      match active_radarr_block {
        ActiveRadarrBlock::Collections => draw_collections(f, app, content_rect),
        ActiveRadarrBlock::SearchCollection => draw_popup_over(
          f,
          app,
          content_rect,
          draw_collections,
          draw_search_box,
          30,
          11,
        ),
        ActiveRadarrBlock::FilterCollections => draw_popup_over(
          f,
          app,
          content_rect,
          draw_collections,
          draw_filter_box,
          30,
          11,
        ),
        ActiveRadarrBlock::UpdateAllCollectionsPrompt => draw_prompt_popup_over(
          f,
          app,
          content_rect,
          draw_collections,
          draw_update_all_collections_prompt,
        ),
        _ => (),
      }
    }
  }
}

pub(super) fn draw_collections<B: Backend>(f: &mut Frame<'_, B>, app: &mut App<'_>, area: Rect) {
  let current_selection = if !app.data.radarr_data.filtered_collections.items.is_empty() {
    app
      .data
      .radarr_data
      .filtered_collections
      .current_selection()
      .clone()
  } else if !app.data.radarr_data.collections.items.is_empty() {
    app.data.radarr_data.collections.current_selection().clone()
  } else {
    Collection::default()
  };
  let quality_profile_map = &app.data.radarr_data.quality_profile_map;
  let content = if !app.data.radarr_data.filtered_collections.items.is_empty()
    && !app.data.radarr_data.is_filtering
  {
    &mut app.data.radarr_data.filtered_collections
  } else {
    &mut app.data.radarr_data.collections
  };
  draw_table(
    f,
    area,
    layout_block_top_border(),
    TableProps {
      content,
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
            .get_by_left(&collection.quality_profile_id.as_u64().unwrap())
            .unwrap()
            .to_owned(),
        ),
        Cell::from(search_on_add),
        Cell::from(monitored),
      ])
      .style(style_primary())
    },
    app.is_loading,
    true,
  );
}

fn draw_update_all_collections_prompt<B: Backend>(
  f: &mut Frame<'_, B>,
  app: &mut App<'_>,
  prompt_area: Rect,
) {
  draw_prompt_box(
    f,
    prompt_area,
    "Update All Collections",
    "Do you want to update all of your collections?",
    app.data.radarr_data.prompt_confirm,
  );
}
