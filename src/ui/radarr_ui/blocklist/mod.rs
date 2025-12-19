use crate::app::App;
use crate::models::Route;
use crate::models::radarr_models::BlocklistItem;
use crate::models::servarr_data::radarr::radarr_data::{ActiveRadarrBlock, BLOCKLIST_BLOCKS};
use crate::ui::DrawUi;
use crate::ui::styles::ManagarrStyle;
use crate::ui::utils::{get_width_from_percentage, layout_block_top_border};
use crate::ui::widgets::confirmation_prompt::ConfirmationPrompt;
use crate::ui::widgets::managarr_table::ManagarrTable;
use crate::ui::widgets::message::Message;
use crate::ui::widgets::popup::{Popup, Size};
use ratatui::Frame;
use ratatui::layout::{Alignment, Constraint, Rect};
use ratatui::style::{Style, Stylize};
use ratatui::text::{Line, Text};
use ratatui::widgets::{Cell, Row};

#[cfg(test)]
#[path = "blocklist_ui_tests.rs"]
mod blocklist_ui_tests;

pub(super) struct BlocklistUi;

impl DrawUi for BlocklistUi {
  fn accepts(route: Route) -> bool {
    let Route::Radarr(active_radarr_block, _) = route else {
      return false;
    };
    BLOCKLIST_BLOCKS.contains(&active_radarr_block)
  }

  fn draw(f: &mut Frame<'_>, app: &mut App<'_>, area: Rect) {
    if let Route::Radarr(active_radarr_block, _) = app.get_current_route() {
      draw_blocklist_table(f, app, area);

      match active_radarr_block {
        ActiveRadarrBlock::BlocklistItemDetails => {
          draw_blocklist_item_details_popup(f, app);
        }
        ActiveRadarrBlock::DeleteBlocklistItemPrompt => {
          let prompt = format!(
            "Do you want to remove this item from your blocklist: \n{}?",
            app
              .data
              .radarr_data
              .blocklist
              .current_selection()
              .source_title
          );
          let confirmation_prompt = ConfirmationPrompt::new()
            .title("Remove Item from Blocklist")
            .prompt(&prompt)
            .yes_no_value(app.data.radarr_data.prompt_confirm);

          f.render_widget(
            Popup::new(confirmation_prompt).size(Size::MediumPrompt),
            f.area(),
          );
        }
        ActiveRadarrBlock::BlocklistClearAllItemsPrompt => {
          let confirmation_prompt = ConfirmationPrompt::new()
            .title("Clear Blocklist")
            .prompt("Do you want to clear your blocklist?")
            .yes_no_value(app.data.radarr_data.prompt_confirm);

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
  if let Route::Radarr(active_radarr_block, _) = app.get_current_route() {
    let current_selection = if app.data.radarr_data.blocklist.items.is_empty() {
      BlocklistItem::default()
    } else {
      app.data.radarr_data.blocklist.current_selection().clone()
    };

    let blocklist_row_mapping = |blocklist_item: &BlocklistItem| {
      let BlocklistItem {
        source_title,
        languages,
        quality,
        custom_formats,
        date,
        movie,
        ..
      } = blocklist_item;

      movie.title.scroll_left_or_reset(
        get_width_from_percentage(area, 20),
        current_selection == *blocklist_item,
        app.ui_scroll_tick_count == 0,
      );

      let languages_string = languages
        .iter()
        .map(|lang| lang.name.to_owned())
        .collect::<Vec<String>>()
        .join(", ");
      let custom_formats_string = if let Some(formats) = custom_formats.as_ref() {
        formats
          .iter()
          .map(|cf| cf.name.to_owned())
          .collect::<Vec<String>>()
          .join(", ")
      } else {
        "".to_owned()
      };

      Row::new(vec![
        Cell::from(movie.title.to_string()),
        Cell::from(source_title.to_owned()),
        Cell::from(languages_string),
        Cell::from(quality.quality.name.to_owned()),
        Cell::from(custom_formats_string),
        Cell::from(date.to_string()),
      ])
      .primary()
    };
    let blocklist_table = ManagarrTable::new(
      Some(&mut app.data.radarr_data.blocklist),
      blocklist_row_mapping,
    )
    .block(layout_block_top_border())
    .loading(app.is_loading)
    .sorting(active_radarr_block == ActiveRadarrBlock::BlocklistSortPrompt)
    .headers([
      "Movie Title",
      "Source Title",
      "Languages",
      "Quality",
      "Formats",
      "Date",
    ])
    .constraints([
      Constraint::Percentage(20),
      Constraint::Percentage(35),
      Constraint::Percentage(10),
      Constraint::Percentage(10),
      Constraint::Percentage(10),
      Constraint::Percentage(15),
    ]);

    f.render_widget(blocklist_table, area);
  }
}

fn draw_blocklist_item_details_popup(f: &mut Frame<'_>, app: &mut App<'_>) {
  let current_selection = if app.data.radarr_data.blocklist.items.is_empty() {
    BlocklistItem::default()
  } else {
    app.data.radarr_data.blocklist.current_selection().clone()
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
