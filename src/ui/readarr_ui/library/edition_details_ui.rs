use crate::app::App;
use crate::models::Route;
use crate::models::readarr_models::Edition;
use crate::models::servarr_data::readarr::readarr_data::EDITION_DETAILS_BLOCKS;
use crate::ui::styles::{primary_style, unmonitored_style};
use crate::ui::utils::title_block_centered;
use crate::ui::widgets::loading_block::LoadingBlock;
use crate::ui::widgets::popup::Size;
use crate::ui::{DrawUi, draw_popup};
use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Style, Stylize};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Paragraph, Wrap};

#[cfg(test)]
#[path = "edition_details_ui_tests.rs"]
mod edition_details_ui_tests;

pub(super) struct EditionDetailsUi;

impl DrawUi for EditionDetailsUi {
  fn accepts(route: Route) -> bool {
    let Route::Readarr(active_readarr_block, _) = route else {
      return false;
    };
    EDITION_DETAILS_BLOCKS.contains(&active_readarr_block)
  }

  fn draw(f: &mut Frame<'_>, app: &mut App<'_>, _area: Rect) {
    if let Some(book_details_modal) = app.data.readarr_data.book_details_modal.as_ref()
      && book_details_modal.edition_details_modal.is_some()
    {
      let draw_edition_details_popup = |f: &mut Frame<'_>, app: &mut App<'_>, popup_area: Rect| {
        draw_edition_details(f, app, popup_area);
      };

      draw_popup(f, app, draw_edition_details_popup, Size::Large);
    }
  }
}

fn draw_edition_details(f: &mut Frame<'_>, app: &App<'_>, area: Rect) {
  let block = title_block_centered("Edition Details");

  match app.data.readarr_data.book_details_modal.as_ref() {
    Some(book_details_modal) if !app.is_loading => {
      if let Some(edition_details_modal) = book_details_modal.edition_details_modal.as_ref() {
        let style = if book_details_modal.editions.is_empty() {
          primary_style()
        } else {
          style_from_edition(book_details_modal.editions.current_selection())
        };
        let edition_details = &edition_details_modal.edition_details;
        let text = Text::from(
          edition_details
            .items
            .iter()
            .map(|line| edition_detail_line(line, style))
            .collect::<Vec<Line<'static>>>(),
        );

        let paragraph = Paragraph::new(text)
          .block(block)
          .wrap(Wrap { trim: false })
          .scroll((edition_details.offset, 0));

        f.render_widget(paragraph, area);
      }
    }
    _ => f.render_widget(
      LoadingBlock::new(
        app.is_loading
          || app
            .data
            .readarr_data
            .book_details_modal
            .as_ref()
            .is_none_or(|book_details_modal| book_details_modal.edition_details_modal.is_none()),
        block,
      ),
      area,
    ),
  }
}

fn style_from_edition(edition: &Edition) -> Style {
  if !edition.monitored {
    return unmonitored_style();
  }

  primary_style()
}

fn edition_detail_line(line: &str, style: Style) -> Line<'static> {
  let line = line.trim_end();

  match line.split_once(':') {
    Some((label, value)) => Line::from(vec![
      format!("{label}:").bold().style(style),
      Span::styled(value.to_owned(), style),
    ]),
    None => Line::from(Span::styled(line.to_owned(), style)),
  }
}
