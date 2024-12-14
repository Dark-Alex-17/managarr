use crate::app::App;
use crate::models::servarr_data::sonarr::sonarr_data::{ActiveSonarrBlock, BLOCKLIST_BLOCKS};
use crate::models::sonarr_models::BlocklistItem;
use crate::models::Route;
use crate::ui::styles::ManagarrStyle;
use crate::ui::utils::layout_block_top_border;
use crate::ui::widgets::confirmation_prompt::ConfirmationPrompt;
use crate::ui::widgets::managarr_table::ManagarrTable;
use crate::ui::widgets::message::Message;
use crate::ui::widgets::popup::{Popup, Size};
use crate::ui::DrawUi;
use ratatui::layout::{Alignment, Constraint, Rect};
use ratatui::style::{Style, Stylize};
use ratatui::text::{Line, Text};
use ratatui::widgets::{Cell, Row};
use ratatui::Frame;

#[cfg(test)]
#[path = "blocklist_ui_tests.rs"]
mod blocklist_ui_tests;

pub(super) struct BlocklistUi;

impl DrawUi for BlocklistUi {
  fn accepts(route: Route) -> bool {
    if let Route::Sonarr(active_sonarr_block, _) = route {
      return BLOCKLIST_BLOCKS.contains(&active_sonarr_block);
    }

    false
  }

  fn draw(f: &mut Frame<'_>, app: &mut App<'_>, area: Rect) {
    if let Route::Sonarr(active_sonarr_block, _) = app.get_current_route() {
      draw_blocklist_table(f, app, area);

      match active_sonarr_block {
        ActiveSonarrBlock::BlocklistItemDetails => {
          draw_blocklist_item_details_popup(f, app);
        }
        ActiveSonarrBlock::DeleteBlocklistItemPrompt => {
          let prompt = format!(
            "Do you want to remove this item from your blocklist: \n{}?",
            app
              .data
              .sonarr_data
              .blocklist
              .current_selection()
              .source_title
          );
          let confirmation_prompt = ConfirmationPrompt::new()
            .title("Remove Item from Blocklist")
            .prompt(&prompt)
            .yes_no_value(app.data.sonarr_data.prompt_confirm);

          f.render_widget(
            Popup::new(confirmation_prompt).size(Size::MediumPrompt),
            f.area(),
          );
        }
        ActiveSonarrBlock::BlocklistClearAllItemsPrompt => {
          let confirmation_prompt = ConfirmationPrompt::new()
            .title("Clear Blocklist")
            .prompt("Do you want to clear your blocklist?")
            .yes_no_value(app.data.sonarr_data.prompt_confirm);

          f.render_widget(
            Popup::new(confirmation_prompt).size(Size::SmallPrompt),
            f.area(),
          );
        }
        _ => (),
      }
    }
  }
}

fn draw_blocklist_table(f: &mut Frame<'_>, app: &mut App<'_>, area: Rect) {
  if let Route::Sonarr(active_sonarr_block, _) = app.get_current_route() {
    let blocklist_table_footer = app
      .data
      .sonarr_data
      .main_tabs
      .get_active_tab_contextual_help();

    let blocklist_row_mapping = |blocklist_item: &BlocklistItem| {
      let BlocklistItem {
        source_title,
        series_title,
        languages,
        quality,
        date,
        ..
      } = blocklist_item;

      let title = series_title.as_ref().unwrap_or(&String::new()).to_owned();
      let languages_string = languages
        .iter()
        .map(|lang| lang.name.to_owned())
        .collect::<Vec<String>>()
        .join(", ");

      Row::new(vec![
        Cell::from(title),
        Cell::from(source_title.to_owned()),
        Cell::from(languages_string),
        Cell::from(quality.quality.name.to_owned()),
        Cell::from(date.to_string()),
      ])
      .primary()
    };
    let blocklist_table = ManagarrTable::new(
      Some(&mut app.data.sonarr_data.blocklist),
      blocklist_row_mapping,
    )
    .block(layout_block_top_border())
    .loading(app.is_loading)
    .footer(blocklist_table_footer)
    .sorting(active_sonarr_block == ActiveSonarrBlock::BlocklistSortPrompt)
    .headers([
      "Series Title",
      "Source Title",
      "Language",
      "Quality",
      "Date",
    ])
    .constraints([
      Constraint::Percentage(25),
      Constraint::Percentage(40),
      Constraint::Percentage(10),
      Constraint::Percentage(10),
      Constraint::Percentage(15),
    ]);

    f.render_widget(blocklist_table, area);
  }
}

fn draw_blocklist_item_details_popup(f: &mut Frame<'_>, app: &mut App<'_>) {
  let current_selection = if app.data.sonarr_data.blocklist.items.is_empty() {
    BlocklistItem::default()
  } else {
    app.data.sonarr_data.blocklist.current_selection().clone()
  };
  let BlocklistItem {
    source_title,
    protocol,
    indexer,
    message,
    ..
  } = current_selection;
  let text = Text::from(vec![
    Line::from(vec![
      "Name: ".bold().secondary(),
      source_title.to_owned().secondary(),
    ]),
    Line::from(vec![
      "Protocol: ".bold().secondary(),
      protocol.to_owned().secondary(),
    ]),
    Line::from(vec![
      "Indexer: ".bold().secondary(),
      indexer.to_owned().secondary(),
    ]),
    Line::from(vec![
      "Message: ".bold().secondary(),
      message.to_owned().secondary(),
    ]),
  ]);

  let message = Message::new(text)
    .title("Details")
    .style(Style::new().secondary())
    .alignment(Alignment::Left);

  f.render_widget(Popup::new(message).size(Size::NarrowMessage), f.area());
}
