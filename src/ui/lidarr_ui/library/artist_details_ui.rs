use chrono::Utc;
use deunicode::deunicode;
use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::Stylize;
use ratatui::text::Line;
use ratatui::widgets::{Cell, Paragraph, Row, Wrap};
use regex::Regex;

use crate::app::App;
use crate::models::Route;
use crate::models::lidarr_models::Album;
use crate::models::servarr_data::lidarr::lidarr_data::{ARTIST_DETAILS_BLOCKS, ActiveLidarrBlock};
use crate::ui::styles::ManagarrStyle;
use crate::ui::utils::{
  borderless_block, get_width_from_percentage, layout_block_top_border, title_block,
};
use crate::ui::widgets::confirmation_prompt::ConfirmationPrompt;
use crate::ui::widgets::managarr_table::ManagarrTable;
use crate::ui::widgets::popup::{Popup, Size};
use crate::ui::{DrawUi, draw_popup, draw_tabs};
use crate::utils::convert_to_gb;

#[cfg(test)]
#[path = "artist_details_ui_tests.rs"]
mod artist_details_ui_tests;

pub(super) struct ArtistDetailsUi;

impl DrawUi for ArtistDetailsUi {
  fn accepts(route: Route) -> bool {
    let Route::Lidarr(active_lidarr_block, _) = route else {
      return false;
    };
    ARTIST_DETAILS_BLOCKS.contains(&active_lidarr_block)
  }

  fn draw(f: &mut Frame<'_>, app: &mut App<'_>, _area: Rect) {
    let route = app.get_current_route();
    if let Route::Lidarr(active_lidarr_block, _) = route {
      let draw_artist_details_popup = |f: &mut Frame<'_>, app: &mut App<'_>, popup_area: Rect| {
        f.render_widget(
          title_block(
            &app
              .data
              .lidarr_data
              .artists
              .current_selection()
              .artist_name
              .text,
          ),
          popup_area,
        );
        let [description_area, detail_area] =
          Layout::vertical([Constraint::Percentage(37), Constraint::Fill(0)])
            .margin(1)
            .areas(popup_area);
        draw_artist_description(f, app, description_area);
        let content_area = draw_tabs(
          f,
          detail_area,
          "Artist Details",
          &app.data.lidarr_data.artist_info_tabs,
        );
        draw_artist_details(f, app, content_area);

        match active_lidarr_block {
          ActiveLidarrBlock::AutomaticallySearchArtistPrompt => {
            let prompt = format!(
              "Do you want to trigger an automatic search of your indexers for all monitored album(s) for the artist: {}?",
              app.data.lidarr_data.artists.current_selection().artist_name
            );
            let confirmation_prompt = ConfirmationPrompt::new()
              .title("Automatic Artist Search")
              .prompt(&prompt)
              .yes_no_value(app.data.lidarr_data.prompt_confirm);

            f.render_widget(
              Popup::new(confirmation_prompt).size(Size::MediumPrompt),
              f.area(),
            );
          }
          ActiveLidarrBlock::UpdateAndScanArtistPrompt => {
            let prompt = format!(
              "Do you want to trigger an update and disk scan for the artist: {}?",
              app.data.lidarr_data.artists.current_selection().artist_name
            );
            let confirmation_prompt = ConfirmationPrompt::new()
              .title("Update and Scan")
              .prompt(&prompt)
              .yes_no_value(app.data.lidarr_data.prompt_confirm);

            f.render_widget(
              Popup::new(confirmation_prompt).size(Size::MediumPrompt),
              f.area(),
            );
          }
          _ => (),
        }
      };

      draw_popup(f, app, draw_artist_details_popup, Size::XXLarge);
    }
  }
}

fn draw_artist_description(f: &mut Frame<'_>, app: &mut App<'_>, area: Rect) {
  let current_selection = app.data.lidarr_data.artists.current_selection();
  let monitored = if current_selection.monitored {
    "Yes"
  } else {
    "No"
  };
  let quality_profile = app
    .data
    .lidarr_data
    .quality_profile_map
    .get_by_left(&current_selection.quality_profile_id)
    .cloned()
    .unwrap_or_default();
  let metadata_profile = app
    .data
    .lidarr_data
    .metadata_profile_map
    .get_by_left(&current_selection.metadata_profile_id)
    .cloned()
    .unwrap_or_default();
  let overview = Regex::new(r"[\r\n\t]")
    .unwrap()
    .replace_all(
      &deunicode(
        current_selection
          .overview
          .as_ref()
          .unwrap_or(&String::new()),
      ),
      "",
    )
    .to_string();

  let mut artist_description = vec![
    Line::from(vec![
      "Artist: ".primary().bold(),
      current_selection.artist_name.text.clone().primary().bold(),
    ]),
    Line::from(vec![
      "Overview: ".primary().bold(),
      overview.default_color(),
    ]),
    Line::from(vec![
      "Disambiguation: ".primary().bold(),
      current_selection
        .disambiguation
        .clone()
        .unwrap_or_default()
        .default_color(),
    ]),
    Line::from(vec![
      "Type: ".primary().bold(),
      current_selection
        .artist_type
        .clone()
        .unwrap_or_default()
        .default_color(),
    ]),
    Line::from(vec![
      "Status: ".primary().bold(),
      current_selection.status.to_display_str().default_color(),
    ]),
    Line::from(vec![
      "Genres: ".primary().bold(),
      current_selection.genres.join(", ").default_color(),
    ]),
    Line::from(vec![
      "Rating: ".primary().bold(),
      current_selection
        .ratings
        .as_ref()
        .map_or_else(
          || "N/A".to_owned(),
          |r| format!("{}%", (r.value * 10.0) as i32),
        )
        .default_color(),
    ]),
    Line::from(vec![
      "Path: ".primary().bold(),
      current_selection.path.clone().default_color(),
    ]),
    Line::from(vec![
      "Quality Profile: ".primary().bold(),
      quality_profile.default_color(),
    ]),
    Line::from(vec![
      "Metadata Profile: ".primary().bold(),
      metadata_profile.default_color(),
    ]),
    Line::from(vec![
      "Monitored: ".primary().bold(),
      monitored.default_color(),
    ]),
  ];

  if let Some(stats) = current_selection.statistics.as_ref() {
    let size = convert_to_gb(stats.size_on_disk);
    artist_description.extend(vec![
      Line::from(vec![
        "Albums: ".primary().bold(),
        stats.album_count.to_string().default_color(),
      ]),
      Line::from(vec![
        "Tracks: ".primary().bold(),
        format!("{}/{}", stats.track_file_count, stats.total_track_count).default_color(),
      ]),
      Line::from(vec![
        "Size on Disk: ".primary().bold(),
        format!("{size:.2} GB").default_color(),
      ]),
    ]);
  }

  let description_paragraph = Paragraph::new(artist_description)
    .block(borderless_block())
    .wrap(Wrap { trim: true });
  f.render_widget(description_paragraph, area);
}

fn draw_artist_details(f: &mut Frame<'_>, app: &mut App<'_>, area: Rect) {
  if let Route::Lidarr(active_lidarr_block, _) =
    app.data.lidarr_data.artist_info_tabs.get_active_route()
  {
    match active_lidarr_block {
      ActiveLidarrBlock::ArtistDetails => draw_albums_table(f, app, area),
      _ => (),
    }
  }
}

fn draw_albums_table(f: &mut Frame<'_>, app: &mut App<'_>, area: Rect) {
  if let Route::Lidarr(active_lidarr_block, _) = app.get_current_route() {
    let current_selection = if app.data.lidarr_data.albums.is_empty() {
      Album::default()
    } else {
      app.data.lidarr_data.albums.current_selection().clone()
    };
    let content = Some(&mut app.data.lidarr_data.albums);
    let album_row_mapping = |album: &Album| {
      album.title.scroll_left_or_reset(
        get_width_from_percentage(area, 33),
        *album == current_selection,
        app.ui_scroll_tick_count == 0,
      );
      let monitored = if album.monitored { "🏷" } else { "" };
      let album_type = album.album_type.clone().unwrap_or_default();
      let release_date = album
        .release_date
        .map_or_else(|| "N/A".to_owned(), |d| d.format("%Y-%m-%d").to_string());
      let track_count = album.statistics.as_ref().map_or_else(
        || "0/0".to_owned(),
        |s| format!("{}/{}", s.track_file_count, s.total_track_count),
      );
      let size = album
        .statistics
        .as_ref()
        .map_or(0f64, |s| convert_to_gb(s.size_on_disk));
      let duration_mins = album.duration / 60000;

      let row = Row::new(vec![
        Cell::from(monitored.to_owned()),
        Cell::from(album.title.to_string()),
        Cell::from(album_type),
        Cell::from(track_count),
        Cell::from(format!("{duration_mins} min")),
        Cell::from(release_date),
        Cell::from(format!("{size:.2} GB")),
      ]);

      if !album.monitored {
        row.unmonitored()
      } else if let Some(stats) = album.statistics.as_ref() {
        if stats.track_file_count == stats.total_track_count && stats.total_track_count > 0 {
          row.downloaded()
        } else if let Some(release_date) = album.release_date.as_ref() {
          if release_date > &Utc::now() {
            row.unreleased()
          } else {
            row.missing()
          }
        } else {
          row.missing()
        }
      } else {
        row.indeterminate()
      }
    };

    let is_searching = active_lidarr_block == ActiveLidarrBlock::SearchAlbums;
    let album_table = ManagarrTable::new(content, album_row_mapping)
      .block(layout_block_top_border())
      .loading(app.is_loading)
      .searching(is_searching)
      .search_produced_empty_results(active_lidarr_block == ActiveLidarrBlock::SearchAlbumsError)
      .headers([
        "Monitored",
        "Title",
        "Type",
        "Tracks",
        "Duration",
        "Release Date",
        "Size",
      ])
      .constraints([
        Constraint::Percentage(7),
        Constraint::Percentage(35),
        Constraint::Percentage(10),
        Constraint::Percentage(10),
        Constraint::Percentage(10),
        Constraint::Percentage(13),
        Constraint::Percentage(15),
      ]);

    if is_searching {
      album_table.show_cursor(f, area);
    }

    f.render_widget(album_table, area);
  }
}
