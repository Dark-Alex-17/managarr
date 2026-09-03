use author_details_ui::AuthorDetailsUi;
use ratatui::{
  Frame,
  layout::{Constraint, Rect},
  widgets::{Cell, Row},
};

use crate::ui::widgets::managarr_table::ManagarrTable;
use crate::ui::widgets::{
  confirmation_prompt::ConfirmationPrompt,
  popup::{Popup, Size},
};
use crate::utils::convert_to_gb;
use crate::{
  app::App,
  models::{
    Route,
    readarr_models::{Author, AuthorStatus},
    servarr_data::readarr::readarr_data::{ActiveReadarrBlock, LIBRARY_BLOCKS},
  },
  ui::{
    DrawUi,
    styles::ManagarrStyle,
    utils::{get_width_from_percentage, layout_block_top_border},
  },
};

mod author_details_ui;

#[cfg(test)]
#[path = "library_ui_tests.rs"]
mod library_ui_tests;

pub(super) struct LibraryUi;

impl DrawUi for LibraryUi {
  fn accepts(route: Route) -> bool {
    if let Route::Readarr(active_readarr_block, _) = route {
      return AuthorDetailsUi::accepts(route) || LIBRARY_BLOCKS.contains(&active_readarr_block);
    }

    false
  }

  fn draw(f: &mut Frame<'_>, app: &mut App<'_>, area: Rect) {
    let route = app.get_current_route();
    draw_library(f, app, area);

    match route {
      _ if AuthorDetailsUi::accepts(route) => AuthorDetailsUi::draw(f, app, area),
      Route::Readarr(ActiveReadarrBlock::UpdateAllAuthorsPrompt, _) => {
        let confirmation_prompt = ConfirmationPrompt::new()
          .title("Update All Authors")
          .prompt("Do you want to update info and scan your disks for all of your authors?")
          .yes_no_value(app.data.readarr_data.prompt_confirm);

        f.render_widget(
          Popup::new(confirmation_prompt).size(Size::MediumPrompt),
          f.area(),
        );
      }
      _ => (),
    }
  }
}

fn draw_library(f: &mut Frame<'_>, app: &mut App<'_>, area: Rect) {
  if let Route::Readarr(active_readarr_block, _) = app.get_current_route() {
    let current_selection = if !app.data.readarr_data.authors.items.is_empty() {
      app.data.readarr_data.authors.current_selection().clone()
    } else {
      Author::default()
    };
    let quality_profile_map = &app.data.readarr_data.quality_profile_map;
    let metadata_profile_map = &app.data.readarr_data.metadata_profile_map;
    let tags_map = &app.data.readarr_data.tags_map;
    let content = Some(&mut app.data.readarr_data.authors);

    let authors_table_row_mapping = |author: &Author| {
      author.author_name.scroll_left_or_reset(
        get_width_from_percentage(area, 25),
        *author == current_selection,
        app.should_text_scroll,
      );
      let monitored = if author.monitored { "🏷" } else { "" };
      let size = author
        .statistics
        .as_ref()
        .map_or(0f64, |stats| convert_to_gb(stats.size_on_disk));
      let quality_profile = quality_profile_map
        .get_by_left(&author.quality_profile_id)
        .cloned()
        .unwrap_or_default();
      let metadata_profile = metadata_profile_map
        .get_by_left(&author.metadata_profile_id)
        .cloned()
        .unwrap_or_default();
      let books = author
        .statistics
        .as_ref()
        .map_or(0, |stats| stats.book_count);
      let tags = author
        .tags
        .iter()
        .filter_map(|tag_id| {
          let id = tag_id.as_i64()?;
          tags_map.get_by_left(&id).cloned()
        })
        .collect::<Vec<_>>()
        .join(", ");

      decorate_author_row_with_style(
        author,
        Row::new(vec![
          Cell::from(author.author_name.to_string()),
          Cell::from(author.status.to_display_str()),
          Cell::from(quality_profile),
          Cell::from(metadata_profile),
          Cell::from(books.to_string()),
          Cell::from(format!("{size:.2} GB")),
          Cell::from(monitored.to_owned()),
          Cell::from(tags),
        ]),
      )
    };
    let authors_table = ManagarrTable::new(content, authors_table_row_mapping)
      .block(layout_block_top_border())
      .loading(app.is_loading)
      .sorting(active_readarr_block == ActiveReadarrBlock::AuthorsSortPrompt)
      .searching(active_readarr_block == ActiveReadarrBlock::SearchAuthors)
      .filtering(active_readarr_block == ActiveReadarrBlock::FilterAuthors)
      .search_produced_empty_results(active_readarr_block == ActiveReadarrBlock::SearchAuthorsError)
      .filter_produced_empty_results(active_readarr_block == ActiveReadarrBlock::FilterAuthorsError)
      .headers([
        "Name",
        "Status",
        "Quality Profile",
        "Metadata Profile",
        "Books",
        "Size",
        "Monitored",
        "Tags",
      ])
      .constraints([
        Constraint::Percentage(34),
        Constraint::Percentage(8),
        Constraint::Percentage(13),
        Constraint::Percentage(13),
        Constraint::Percentage(7),
        Constraint::Percentage(8),
        Constraint::Percentage(6),
        Constraint::Percentage(11),
      ]);

    if [
      ActiveReadarrBlock::SearchAuthors,
      ActiveReadarrBlock::FilterAuthors,
    ]
    .contains(&active_readarr_block)
    {
      authors_table.show_cursor(f, area);
    }

    f.render_widget(authors_table, area);
  }
}

fn decorate_author_row_with_style<'a>(author: &Author, row: Row<'a>) -> Row<'a> {
  if !author.monitored {
    return row.unmonitored();
  }

  match author.status {
    AuthorStatus::Ended => {
      if let Some(ref stats) = author.statistics {
        return if stats.book_file_count == stats.total_book_count && stats.total_book_count > 0 {
          row.downloaded()
        } else {
          row.missing()
        };
      }
      row.indeterminate()
    }
    AuthorStatus::Continuing => {
      if let Some(ref stats) = author.statistics {
        return if stats.book_file_count == stats.total_book_count && stats.total_book_count > 0 {
          row.unreleased()
        } else {
          row.missing()
        };
      }
      row.indeterminate()
    }
    _ => row.indeterminate(),
  }
}
