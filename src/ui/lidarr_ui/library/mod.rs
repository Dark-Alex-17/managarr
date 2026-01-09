use add_artist_ui::AddArtistUi;
use artist_details_ui::ArtistDetailsUi;
use delete_artist_ui::DeleteArtistUi;
use edit_artist_ui::EditArtistUi;
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
    lidarr_models::{Artist, ArtistStatus},
    servarr_data::lidarr::lidarr_data::{ActiveLidarrBlock, LIBRARY_BLOCKS},
  },
  ui::{
    DrawUi,
    styles::ManagarrStyle,
    utils::{get_width_from_percentage, layout_block_top_border},
  },
};

mod add_artist_ui;
mod artist_details_ui;
mod delete_artist_ui;
mod edit_artist_ui;

#[cfg(test)]
#[path = "library_ui_tests.rs"]
mod library_ui_tests;

pub(super) struct LibraryUi;

impl DrawUi for LibraryUi {
  fn accepts(route: Route) -> bool {
    if let Route::Lidarr(active_lidarr_block, _) = route {
      return AddArtistUi::accepts(route)
        || DeleteArtistUi::accepts(route)
        || EditArtistUi::accepts(route)
        || ArtistDetailsUi::accepts(route)
        || LIBRARY_BLOCKS.contains(&active_lidarr_block);
    }

    false
  }

  fn draw(f: &mut Frame<'_>, app: &mut App<'_>, area: Rect) {
    let route = app.get_current_route();
    draw_library(f, app, area);

    match route {
      _ if AddArtistUi::accepts(route) => AddArtistUi::draw(f, app, area),
      _ if DeleteArtistUi::accepts(route) => DeleteArtistUi::draw(f, app, area),
      _ if EditArtistUi::accepts(route) => EditArtistUi::draw(f, app, area),
      _ if ArtistDetailsUi::accepts(route) => ArtistDetailsUi::draw(f, app, area),
      Route::Lidarr(ActiveLidarrBlock::UpdateAllArtistsPrompt, _) => {
        let confirmation_prompt = ConfirmationPrompt::new()
          .title("Update All Artists")
          .prompt("Do you want to update info and scan your disks for all of your artists?")
          .yes_no_value(app.data.lidarr_data.prompt_confirm);

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
  if let Route::Lidarr(active_lidarr_block, _) = app.get_current_route() {
    let current_selection = if !app.data.lidarr_data.artists.items.is_empty() {
      app.data.lidarr_data.artists.current_selection().clone()
    } else {
      Artist::default()
    };
    let quality_profile_map = &app.data.lidarr_data.quality_profile_map;
    let metadata_profile_map = &app.data.lidarr_data.metadata_profile_map;
    let tags_map = &app.data.lidarr_data.tags_map;
    let content = Some(&mut app.data.lidarr_data.artists);

    let artists_table_row_mapping = |artist: &Artist| {
      artist.artist_name.scroll_left_or_reset(
        get_width_from_percentage(area, 25),
        *artist == current_selection,
        app.ui_scroll_tick_count == 0,
      );
      let monitored = if artist.monitored { "🏷" } else { "" };
      let artist_type = artist.artist_type.clone().unwrap_or_default();
      let size = artist
        .statistics
        .as_ref()
        .map_or(0f64, |stats| convert_to_gb(stats.size_on_disk));
      let quality_profile = quality_profile_map
        .get_by_left(&artist.quality_profile_id)
        .cloned()
        .unwrap_or_default();
      let metadata_profile = metadata_profile_map
        .get_by_left(&artist.metadata_profile_id)
        .cloned()
        .unwrap_or_default();
      let albums = artist
        .statistics
        .as_ref()
        .map_or(0, |stats| stats.album_count);
      let tracks = artist.statistics.as_ref().map_or(String::new(), |stats| {
        format!("{}/{}", stats.track_file_count, stats.total_track_count)
      });
      let tags = artist
        .tags
        .iter()
        .filter_map(|tag_id| {
          let id = tag_id.as_i64()?;
          tags_map.get_by_left(&id).cloned()
        })
        .collect::<Vec<_>>()
        .join(", ");

      decorate_artist_row_with_style(
        artist,
        Row::new(vec![
          Cell::from(artist.artist_name.to_string()),
          Cell::from(artist_type),
          Cell::from(artist.status.to_display_str()),
          Cell::from(quality_profile),
          Cell::from(metadata_profile),
          Cell::from(albums.to_string()),
          Cell::from(tracks),
          Cell::from(format!("{size:.2} GB")),
          Cell::from(monitored.to_owned()),
          Cell::from(tags),
        ]),
      )
    };
    let artists_table = ManagarrTable::new(content, artists_table_row_mapping)
      .block(layout_block_top_border())
      .loading(app.is_loading)
      .sorting(active_lidarr_block == ActiveLidarrBlock::ArtistsSortPrompt)
      .searching(active_lidarr_block == ActiveLidarrBlock::SearchArtists)
      .filtering(active_lidarr_block == ActiveLidarrBlock::FilterArtists)
      .search_produced_empty_results(active_lidarr_block == ActiveLidarrBlock::SearchArtistsError)
      .filter_produced_empty_results(active_lidarr_block == ActiveLidarrBlock::FilterArtistsError)
      .headers([
        "Name",
        "Type",
        "Status",
        "Quality Profile",
        "Metadata Profile",
        "Albums",
        "Tracks",
        "Size",
        "Monitored",
        "Tags",
      ])
      .constraints([
        Constraint::Percentage(22),
        Constraint::Percentage(8),
        Constraint::Percentage(8),
        Constraint::Percentage(12),
        Constraint::Percentage(12),
        Constraint::Percentage(6),
        Constraint::Percentage(8),
        Constraint::Percentage(7),
        Constraint::Percentage(6),
        Constraint::Percentage(11),
      ]);

    if [
      ActiveLidarrBlock::SearchArtists,
      ActiveLidarrBlock::FilterArtists,
    ]
    .contains(&active_lidarr_block)
    {
      artists_table.show_cursor(f, area);
    }

    f.render_widget(artists_table, area);
  }
}

fn decorate_artist_row_with_style<'a>(artist: &Artist, row: Row<'a>) -> Row<'a> {
  if !artist.monitored {
    return row.unmonitored();
  }

  match artist.status {
    ArtistStatus::Ended => {
      if let Some(ref stats) = artist.statistics {
        return if stats.track_file_count == stats.total_track_count && stats.total_track_count > 0 {
          row.downloaded()
        } else {
          row.missing()
        };
      }
      row.indeterminate()
    }
    ArtistStatus::Continuing => {
      if let Some(ref stats) = artist.statistics {
        return if stats.track_file_count == stats.total_track_count && stats.total_track_count > 0 {
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
