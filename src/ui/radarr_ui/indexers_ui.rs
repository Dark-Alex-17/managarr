use tui::backend::Backend;
use tui::layout::{Constraint, Rect};
use tui::text::Text;
use tui::widgets::{Cell, Row};
use tui::Frame;

use crate::app::radarr::ActiveRadarrBlock;
use crate::app::App;
use crate::models::radarr_models::Indexer;
use crate::models::Route;
use crate::ui::utils::{layout_block_top_border, style_failure, style_primary, style_success};
use crate::ui::{draw_prompt_box, draw_prompt_popup_over, draw_table, DrawUi, TableProps};

pub(super) struct IndexersUi {}

impl DrawUi for IndexersUi {
  fn draw<B: Backend>(f: &mut Frame<'_, B>, app: &mut App<'_>, content_rect: Rect) {
    if let Route::Radarr(active_radarr_block, _) = *app.get_current_route() {
      match active_radarr_block {
        ActiveRadarrBlock::Indexers => draw_indexers(f, app, content_rect),
        ActiveRadarrBlock::DeleteIndexerPrompt => draw_prompt_popup_over(
          f,
          app,
          content_rect,
          draw_indexers,
          draw_delete_indexer_prompt,
        ),
        _ => (),
      }
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
      "Do you really want to delete this indexer: {}?",
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
