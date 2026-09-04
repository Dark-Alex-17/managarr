use crate::app::App;
use crate::models::Route;
use crate::models::readarr_models::BlocklistItem;
use crate::models::servarr_data::readarr::readarr_data::{ActiveReadarrBlock, BLOCKLIST_BLOCKS};
use crate::ui::DrawUi;
use crate::ui::styles::{ManagarrStyle, secondary_style};
use crate::ui::utils::layout_block_top_border;
use crate::ui::widgets::confirmation_prompt::ConfirmationPrompt;
use crate::ui::widgets::managarr_table::ManagarrTable;
use crate::ui::widgets::message::Message;
use crate::ui::widgets::popup::{Popup, Size};
use ratatui::Frame;
use ratatui::layout::{Alignment, Constraint, Rect};
use ratatui::style::Stylize;
use ratatui::text::{Line, Text};
use ratatui::widgets::{Cell, Row};

#[cfg(test)]
#[path = "blocklist_ui_tests.rs"]
mod blocklist_ui_tests;

pub(super) struct BlocklistUi;

impl DrawUi for BlocklistUi {
  fn accepts(route: Route) -> bool {
    if let Route::Readarr(active_readarr_block, _) = route {
      return BLOCKLIST_BLOCKS.contains(&active_readarr_block);
    }

    false
  }

  fn draw(f: &mut Frame<'_>, app: &mut App<'_>, area: Rect) {
    if let Route::Readarr(active_readarr_block, _) = app.get_current_route() {
      draw_blocklist_table(f, app, area);

      match active_readarr_block {
        ActiveReadarrBlock::BlocklistItemDetails => {
          draw_blocklist_item_details_popup(f, app);
        }
        ActiveReadarrBlock::DeleteBlocklistItemPrompt => {
          let prompt = format!(
            "Do you want to remove this item from your blocklist: \n{}?",
            app
              .data
              .readarr_data
              .blocklist
              .current_selection()
              .source_title
          );
          let confirmation_prompt = ConfirmationPrompt::new()
            .title("Remove Item from Blocklist")
            .prompt(&prompt)
            .yes_no_value(app.data.readarr_data.prompt_confirm);

          f.render_widget(
            Popup::new(confirmation_prompt).size(Size::MediumPrompt),
            f.area(),
          );
        }
        ActiveReadarrBlock::BlocklistClearAllItemsPrompt => {
          let confirmation_prompt = ConfirmationPrompt::new()
            .title("Clear Blocklist")
            .prompt("Do you want to clear your blocklist?")
            .yes_no_value(app.data.readarr_data.prompt_confirm);

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
  if let Route::Readarr(active_readarr_block, _) = app.get_current_route() {
    let blocklist_row_mapping = |blocklist_item: &BlocklistItem| {
      let BlocklistItem {
        source_title,
        author,
        quality,
        date,
        ..
      } = blocklist_item;

      let title = author.author_name.text.to_owned();

      Row::new(vec![
        Cell::from(title),
        Cell::from(source_title.to_owned()),
        Cell::from(quality.quality.name.to_owned()),
        Cell::from(date.to_string()),
      ])
      .primary()
    };
    let blocklist_table = ManagarrTable::new(
      Some(&mut app.data.readarr_data.blocklist),
      blocklist_row_mapping,
    )
    .block(layout_block_top_border())
    .loading(app.is_loading)
    .sorting(active_readarr_block == ActiveReadarrBlock::BlocklistSortPrompt)
    .headers(["Author Name", "Source Title", "Quality", "Date"])
    .constraints([
      Constraint::Percentage(27),
      Constraint::Percentage(43),
      Constraint::Percentage(13),
      Constraint::Percentage(17),
    ]);

    f.render_widget(blocklist_table, area);
  }
}

fn draw_blocklist_item_details_popup(f: &mut Frame<'_>, app: &mut App<'_>) {
  let current_selection = if app.data.readarr_data.blocklist.items.is_empty() {
    BlocklistItem::default()
  } else {
    app.data.readarr_data.blocklist.current_selection().clone()
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
    .style(secondary_style())
    .alignment(Alignment::Left);

  f.render_widget(Popup::new(message).size(Size::NarrowMessage), f.area());
}
