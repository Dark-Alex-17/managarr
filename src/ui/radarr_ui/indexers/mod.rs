use tui::backend::Backend;
use tui::layout::{Constraint, Rect};
use tui::text::Text;
use tui::widgets::{Cell, Row};
use tui::Frame;

use crate::app::App;
use crate::models::radarr_models::Indexer;
use crate::models::servarr_data::radarr::radarr_data::{ActiveRadarrBlock, INDEXERS_BLOCKS};
use crate::models::Route;
use crate::ui::radarr_ui::indexers::indexer_settings_ui::IndexerSettingsUi;
use crate::ui::utils::{layout_block_top_border, style_failure, style_primary, style_success};
use crate::ui::{draw_prompt_box, draw_prompt_popup_over, draw_table, DrawUi, TableProps};

mod indexer_settings_ui;

#[cfg(test)]
#[path = "indexers_ui_tests.rs"]
mod indexers_ui_tests;

pub(super) struct IndexersUi {}

impl DrawUi for IndexersUi {
  fn accepts(route: Route) -> bool {
    if let Route::Radarr(active_radarr_block, _) = route {
      return IndexerSettingsUi::accepts(route) || INDEXERS_BLOCKS.contains(&active_radarr_block);
    }

    false
  }

  fn draw<B: Backend>(f: &mut Frame<'_, B>, app: &mut App<'_>, content_rect: Rect) {
    let route = *app.get_current_route();
    let mut indexers_matchers = |active_radarr_block| match active_radarr_block {
      ActiveRadarrBlock::Indexers => draw_indexers(f, app, content_rect),
      ActiveRadarrBlock::DeleteIndexerPrompt => draw_prompt_popup_over(
        f,
        app,
        content_rect,
        draw_indexers,
        draw_delete_indexer_prompt,
      ),
      _ => (),
    };

    match route {
      _ if IndexerSettingsUi::accepts(route) => IndexerSettingsUi::draw(f, app, content_rect),
      Route::Radarr(active_radarr_block, _) if INDEXERS_BLOCKS.contains(&active_radarr_block) => {
        indexers_matchers(active_radarr_block)
      }
      _ => (),
    }
  }
}

fn draw_indexers<B: Backend>(f: &mut Frame<'_, B>, app: &mut App<'_>, area: Rect) {
  draw_table(
    f,
    area,
    layout_block_top_border(),
    TableProps {
      content: &mut app.data.radarr_data.indexers,
      table_headers: vec![
        "Indexer",
        "RSS",
        "Automatic Search",
        "Interactive Search",
        "Priority",
      ],
      constraints: vec![
        Constraint::Ratio(1, 5),
        Constraint::Ratio(1, 5),
        Constraint::Ratio(1, 5),
        Constraint::Ratio(1, 5),
        Constraint::Ratio(1, 5),
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
        ..
      } = indexer;
      let bool_to_text = |flag: bool| {
        if flag {
          return ("Enabled", style_success());
        }

        ("Disabled", style_failure())
      };

      let (rss_text, rss_style) = bool_to_text(*enable_rss);
      let mut rss = Text::from(rss_text);
      rss.patch_style(rss_style);

      let (auto_search_text, auto_search_style) = bool_to_text(*enable_automatic_search);
      let mut automatic_search = Text::from(auto_search_text);
      automatic_search.patch_style(auto_search_style);

      let (interactive_search_text, interactive_search_style) =
        bool_to_text(*enable_interactive_search);
      let mut interactive_search = Text::from(interactive_search_text);
      interactive_search.patch_style(interactive_search_style);

      Row::new(vec![
        Cell::from(name.clone().unwrap_or_default()),
        Cell::from(rss),
        Cell::from(automatic_search),
        Cell::from(interactive_search),
        Cell::from(priority.as_u64().unwrap().to_string()),
      ])
      .style(style_primary())
    },
    app.is_loading,
    true,
  )
}

fn draw_delete_indexer_prompt<B: Backend>(
  f: &mut Frame<'_, B>,
  app: &mut App<'_>,
  prompt_area: Rect,
) {
  draw_prompt_box(
    f,
    prompt_area,
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
