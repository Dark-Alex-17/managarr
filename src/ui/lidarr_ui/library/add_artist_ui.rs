use std::sync::atomic::Ordering;

use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::widgets::{Cell, Row};

use crate::App;
use crate::models::Route;
use crate::models::lidarr_models::AddArtistSearchResult;
use crate::models::servarr_data::lidarr::lidarr_data::{ADD_ARTIST_BLOCKS, ActiveLidarrBlock};
use crate::ui::styles::ManagarrStyle;
use crate::ui::utils::{get_width_from_percentage, layout_block, title_block_centered};
use crate::ui::widgets::input_box::InputBox;
use crate::ui::widgets::managarr_table::ManagarrTable;
use crate::ui::widgets::message::Message;
use crate::ui::widgets::popup::{Popup, Size};
use crate::ui::{DrawUi, draw_popup};

#[cfg(test)]
#[path = "add_artist_ui_tests.rs"]
mod add_artist_ui_tests;

pub(super) struct AddArtistUi;

impl DrawUi for AddArtistUi {
  fn accepts(route: Route) -> bool {
    let Route::Lidarr(active_lidarr_block, _) = route else {
      return false;
    };
    ADD_ARTIST_BLOCKS.contains(&active_lidarr_block)
  }

  fn draw(f: &mut Frame<'_>, app: &mut App<'_>, _area: Rect) {
    draw_popup(f, app, draw_add_artist_search, Size::Large);
  }
}

fn draw_add_artist_search(f: &mut Frame<'_>, app: &mut App<'_>, area: Rect) {
  let is_loading = app.is_loading || app.data.lidarr_data.add_searched_artists.is_none();
  let current_selection = if let Some(add_searched_artists) =
    app.data.lidarr_data.add_searched_artists.as_ref()
    && !add_searched_artists.is_empty()
  {
    add_searched_artists.current_selection().clone()
  } else {
    AddArtistSearchResult::default()
  };

  let [search_box_area, results_area] =
    Layout::vertical([Constraint::Length(3), Constraint::Fill(0)])
      .margin(1)
      .areas(area);
  let block_content = &app
    .data
    .lidarr_data
    .add_artist_search
    .as_ref()
    .expect("add_artist_search must be populated")
    .text;
  let offset = app
    .data
    .lidarr_data
    .add_artist_search
    .as_ref()
    .expect("add_artist_search must be populated")
    .offset
    .load(Ordering::SeqCst);

  let search_results_row_mapping = |artist: &AddArtistSearchResult| {
    let rating = artist
      .ratings
      .as_ref()
      .map_or(String::new(), |r| format!("{:.1}", r.value));
    let in_library = if app
      .data
      .lidarr_data
      .artists
      .items
      .iter()
      .any(|a| a.foreign_artist_id == artist.foreign_artist_id)
    {
      "✔"
    } else {
      ""
    };

    artist.artist_name.scroll_left_or_reset(
      get_width_from_percentage(area, 27),
      *artist == current_selection,
      app.ui_scroll_tick_count == 0,
    );

    Row::new(vec![
      Cell::from(in_library),
      Cell::from(artist.artist_name.to_string()),
      Cell::from(artist.artist_type.clone().unwrap_or_default()),
      Cell::from(artist.status.to_display_str()),
      Cell::from(rating),
      Cell::from(artist.genres.join(", ")),
    ])
    .primary()
  };

  if let Route::Lidarr(active_lidarr_block, _) = app.get_current_route() {
    match active_lidarr_block {
      ActiveLidarrBlock::AddArtistSearchInput => {
        let search_box = InputBox::new(block_content)
          .offset(offset)
          .block(title_block_centered("Add Artist"));

        search_box.show_cursor(f, search_box_area);
        f.render_widget(layout_block().default_color(), results_area);
        f.render_widget(search_box, search_box_area);
      }
      ActiveLidarrBlock::AddArtistEmptySearchResults => {
        let error_message = Message::new("No artists found matching your query!");
        let error_message_popup = Popup::new(error_message).size(Size::Message);

        f.render_widget(layout_block().default_color(), results_area);
        f.render_widget(error_message_popup, f.area());
      }
      ActiveLidarrBlock::AddArtistSearchResults => {
        let search_results_table = ManagarrTable::new(
          app.data.lidarr_data.add_searched_artists.as_mut(),
          search_results_row_mapping,
        )
        .loading(is_loading)
        .block(layout_block().default_color())
        .headers(["✔", "Name", "Type", "Status", "Rating", "Genres"])
        .constraints([
          Constraint::Percentage(3),
          Constraint::Percentage(27),
          Constraint::Percentage(12),
          Constraint::Percentage(12),
          Constraint::Percentage(8),
          Constraint::Percentage(38),
        ]);

        f.render_widget(search_results_table, results_area);
      }
      _ => (),
    }
  }

  f.render_widget(
    InputBox::new(block_content)
      .offset(offset)
      .block(title_block_centered("Add Artist")),
    search_box_area,
  );
}
