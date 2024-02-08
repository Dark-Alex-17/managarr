use ratatui::layout::{Constraint, Rect};
use ratatui::text::Text;
use ratatui::widgets::{Cell, Row};
use ratatui::Frame;

use crate::app::App;
use crate::models::radarr_models::Indexer;
use crate::models::servarr_data::radarr::radarr_data::{ActiveRadarrBlock, INDEXERS_BLOCKS};
use crate::models::Route;
use crate::ui::radarr_ui::indexers::edit_indexer_ui::EditIndexerUi;
use crate::ui::radarr_ui::indexers::indexer_settings_ui::IndexerSettingsUi;
use crate::ui::radarr_ui::indexers::test_all_indexers_ui::TestAllIndexersUi;
use crate::ui::styles::ManagarrStyle;
use crate::ui::utils::layout_block_top_border;
use crate::ui::{draw_prompt_box, draw_prompt_popup_over, draw_table, DrawUi, TableProps};

mod edit_indexer_ui;
mod indexer_settings_ui;
mod test_all_indexers_ui;

#[cfg(test)]
#[path = "indexers_ui_tests.rs"]
mod indexers_ui_tests;

pub(super) struct IndexersUi;

impl DrawUi for IndexersUi {
  fn accepts(route: Route) -> bool {
    if let Route::Radarr(active_radarr_block, _) = route {
      return EditIndexerUi::accepts(route)
        || IndexerSettingsUi::accepts(route)
        || TestAllIndexersUi::accepts(route)
        || INDEXERS_BLOCKS.contains(&active_radarr_block);
    }

    false
  }

  fn draw(f: &mut Frame<'_>, app: &mut App<'_>, area: Rect) {
    let route = *app.get_current_route();
    let mut indexers_matchers = |active_radarr_block| match active_radarr_block {
      ActiveRadarrBlock::Indexers => draw_indexers(f, app, area),
      ActiveRadarrBlock::DeleteIndexerPrompt => {
        draw_prompt_popup_over(f, app, area, draw_indexers, draw_delete_indexer_prompt)
      }
      _ => (),
    };

    match route {
      _ if EditIndexerUi::accepts(route) => EditIndexerUi::draw(f, app, area),
      _ if IndexerSettingsUi::accepts(route) => IndexerSettingsUi::draw(f, app, area),
      _ if TestAllIndexersUi::accepts(route) => TestAllIndexersUi::draw(f, app, area),
      Route::Radarr(active_radarr_block, _) if INDEXERS_BLOCKS.contains(&active_radarr_block) => {
        indexers_matchers(active_radarr_block)
      }
      _ => (),
    }
  }
}

fn draw_indexers(f: &mut Frame<'_>, app: &mut App<'_>, area: Rect) {
  draw_table(
    f,
    area,
    layout_block_top_border(),
    TableProps {
      content: Some(&mut app.data.radarr_data.indexers),
      wrapped_content: None,
      table_headers: vec![
        "Indexer",
        "RSS",
        "Automatic Search",
        "Interactive Search",
        "Priority",
        "Tags",
      ],
      constraints: vec![
        Constraint::Percentage(25),
        Constraint::Percentage(13),
        Constraint::Percentage(13),
        Constraint::Percentage(13),
        Constraint::Percentage(13),
        Constraint::Percentage(23),
      ],
      help: app
        .data
        .radarr_data
        .main_tabs
        .get_active_tab_contextual_help(),
    },
    |indexer: &'_ Indexer| {
      let Indexer {
        name,
        enable_rss,
        enable_automatic_search,
        enable_interactive_search,
        priority,
        tags,
        ..
      } = indexer;
      let bool_to_text = |flag: bool| {
        if flag {
          return Text::from("Enabled").success();
        }

        Text::from("Disabled").failure()
      };

      let rss = bool_to_text(*enable_rss);
      let automatic_search = bool_to_text(*enable_automatic_search);
      let interactive_search = bool_to_text(*enable_interactive_search);
      let tags: String = tags
        .iter()
        .map(|tag_id| {
          app
            .data
            .radarr_data
            .tags_map
            .get_by_left(&tag_id.as_i64().unwrap())
            .unwrap()
            .clone()
        })
        .collect::<Vec<String>>()
        .join(", ");

      Row::new(vec![
        Cell::from(name.clone().unwrap_or_default()),
        Cell::from(rss),
        Cell::from(automatic_search),
        Cell::from(interactive_search),
        Cell::from(priority.to_string()),
        Cell::from(tags),
      ])
      .primary()
    },
    app.is_loading,
    true,
  )
}

fn draw_delete_indexer_prompt(f: &mut Frame<'_>, app: &mut App<'_>, area: Rect) {
  draw_prompt_box(
    f,
    area,
    "Delete Indexer",
    format!(
      "Do you really want to delete this indexer: \n{}?",
      app
        .data
        .radarr_data
        .indexers
        .current_selection()
        .name
        .clone()
        .unwrap_or_default()
    )
    .as_str(),
    app.data.radarr_data.prompt_confirm,
  );
}
