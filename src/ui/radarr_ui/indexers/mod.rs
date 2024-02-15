use ratatui::layout::{Constraint, Rect};
use ratatui::style::{Style, Stylize};
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
use crate::ui::utils::{layout_block_top_border, title_block};
use crate::ui::widgets::confirmation_prompt::ConfirmationPrompt;
use crate::ui::widgets::loading_block::LoadingBlock;
use crate::ui::widgets::managarr_table::ManagarrTable;
use crate::ui::widgets::message::Message;
use crate::ui::widgets::popup::{Popup, Size};
use crate::ui::DrawUi;

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
      ActiveRadarrBlock::TestIndexer => {
        draw_indexers(f, app, area);
        if app.is_loading {
          let loading_popup = Popup::new(LoadingBlock::new(
            app.is_loading,
            title_block("Testing Indexer"),
          ))
          .size(Size::LargeMessage);
          f.render_widget(loading_popup, f.size());
        } else {
          let popup = if let Some(result) = app.data.radarr_data.indexer_test_error.as_ref() {
            Popup::new(Message::new(result.clone())).size(Size::LargeMessage)
          } else {
            let message = Message::new("Indexer test succeeded!")
              .title("Success")
              .style(Style::new().success().bold());
            Popup::new(message).size(Size::Message)
          };

          f.render_widget(popup, f.size());
        }
      }
      ActiveRadarrBlock::DeleteIndexerPrompt => {
        let prompt = format!(
          "Do you really want to delete this indexer: \n{}?",
          app
            .data
            .radarr_data
            .indexers
            .current_selection()
            .name
            .clone()
            .unwrap_or_default()
        );
        let confirmation_prompt = ConfirmationPrompt::new()
          .title("Delete Indexer")
          .prompt(&prompt)
          .yes_no_value(app.data.radarr_data.prompt_confirm);

        draw_indexers(f, app, area);
        f.render_widget(Popup::new(confirmation_prompt).size(Size::Prompt), f.size());
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
  let indexers_row_mapping = |indexer: &'_ Indexer| {
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
  };
  let indexers_table_footer = app
    .data
    .radarr_data
    .main_tabs
    .get_active_tab_contextual_help();
  let indexers_table = ManagarrTable::new(
    Some(&mut app.data.radarr_data.indexers),
    indexers_row_mapping,
  )
  .block(layout_block_top_border())
  .footer(indexers_table_footer)
  .loading(app.is_loading)
  .headers([
    "Indexer",
    "RSS",
    "Automatic Search",
    "Interactive Search",
    "Priority",
    "Tags",
  ])
  .constraints([
    Constraint::Percentage(25),
    Constraint::Percentage(13),
    Constraint::Percentage(13),
    Constraint::Percentage(13),
    Constraint::Percentage(13),
    Constraint::Percentage(23),
  ]);

  f.render_widget(indexers_table, area);
}
