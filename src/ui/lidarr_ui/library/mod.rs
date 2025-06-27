use add_artist_ui::AddArtistUi;
use delete_artist_ui::DeleteArtistUi;
use edit_artist_ui::EditArtistUi;
use ratatui::{
  layout::{Constraint, Rect},
  widgets::{Cell, Row},
  Frame,
};
use artist_details_ui::ArtistDetailsUi;

use crate::ui::widgets::{
  confirmation_prompt::ConfirmationPrompt,
  popup::{Popup, Size},
};
use crate::{
  app::App,
  models::{
    servarr_data::lidarr::lidarr_data::{ActiveLidarrBlock, LIBRARY_BLOCKS},
    lidarr_models::{Artist, ArtistStatus},
    Route,
  },
  ui::{
    styles::ManagarrStyle,
    utils::{get_width_from_percentage, layout_block_top_border},
    widgets::managarr_table::ManagarrTable,
    DrawUi,
  },
};

mod add_artist_ui;
mod delete_artist_ui;
mod edit_artist_ui;
mod artist_details_ui;

mod track_details_ui;
#[cfg(test)]
#[path = "library_ui_tests.rs"]
mod library_ui_tests;
mod album_details_ui;

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
    let metadata_profile_map = &app.data.lidarr_data.metadata_profiles_map;
    let tags_map = &app.data.lidarr_data.tags_map;
    let content = Some(&mut app.data.lidarr_data.artists);
    let help_footer = app
      .data
      .lidarr_data
      .main_tabs
      .get_active_tab_contextual_help();

    let artist_table_row_mapping = |artist: &Artist| {
      artist.artist_name.scroll_left_or_reset(
        get_width_from_percentage(area, 23),
        *artist == current_selection,
        app.tick_count % app.ticks_until_scroll == 0,
      );
      let monitored = if artist.monitored { "🏷" } else { "" };
      let quality_profile = quality_profile_map
        .get_by_left(&artist.quality_profile_id)
        .unwrap()
        .to_owned();
      let metadata_profile = metadata_profile_map
        .get_by_left(&artist.metadata_profile_id)
        .unwrap()
        .to_owned();
      let empty_tag = String::new();
      let tags = if !artist.tags.is_empty() {
        artist
          .tags
          .iter()
          .map(|tag_id| {
            tags_map
              .get_by_left(&tag_id.as_i64().unwrap())
              .unwrap_or(&empty_tag)
              .clone()
          })
          .collect::<Vec<String>>()
          .join(", ")
      } else {
        String::new()
      };

      decorate_artist_row_with_style(
        artist,
        Row::new(vec![
          Cell::from(artist.artist_name.to_string()),
          Cell::from(artist.status.to_display_str()),
          Cell::from(quality_profile),
          Cell::from(metadata_profile),
          Cell::from(monitored.to_owned()),
          Cell::from(tags),
        ]),
      )
    };
    let artists_table = ManagarrTable::new(content, artist_table_row_mapping)
      .block(layout_block_top_border())
      .loading(app.is_loading)
      .footer(help_footer)
      .sorting(active_lidarr_block == ActiveLidarrBlock::ArtistSortPrompt)
      .searching(active_lidarr_block == ActiveLidarrBlock::SearchArtists)
      .filtering(active_lidarr_block == ActiveLidarrBlock::FilterArtists)
      .search_produced_empty_results(active_lidarr_block == ActiveLidarrBlock::SearchArtistsError)
      .filter_produced_empty_results(active_lidarr_block == ActiveLidarrBlock::FilterArtistsError)
      .headers([
        "Artist Name",
        "Status",
        "Quality Profile",
        "Metadata Profile",
        "Monitored",
        "Tags",
      ])
      .constraints([
        Constraint::Percentage(23),
        Constraint::Percentage(14),
        Constraint::Percentage(13),
        Constraint::Percentage(10),
        Constraint::Percentage(6),
        Constraint::Percentage(12),
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
      if let Some(ref albums) = artist.albums {
        return if albums
          .iter()
          .filter(|album| album.monitored)
          .filter(|album| album.statistics.is_some())
          .all(|album| {
            album
              .statistics
              .as_ref()
              .expect("Album Statistics is undefined")
              .track_file_count
              == album
                .statistics
                .as_ref()
                .expect("Album statistics is undefined")
                .track_count
          }) {
          row.downloaded()
        } else {
          row.missing()
        };
      }

      row.indeterminate()
    }
    ArtistStatus::Active => {
      if let Some(ref albums) = artist.albums {
        return if albums
          .iter()
          .filter(|album| album.monitored)
          .filter(|album| album.statistics.is_some())
          .all(|album| {
            album
              .statistics
              .as_ref()
              .expect("Album Statistics is undefined")
              .track_file_count
              == album
                .statistics
                .as_ref()
                .expect("Album statistics is undefined")
                .track_count
          }) {
          row.unreleased()
        } else {
          row.missing()
        };
      }

      row.indeterminate()
    }
    ArtistStatus::Upcoming => row.unreleased(),
    _ => row.indeterminate(),
  }
}
