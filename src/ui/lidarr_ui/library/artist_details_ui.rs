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
use crate::models::lidarr_models::{Album, LidarrHistoryItem, LidarrRelease};
use crate::models::servarr_data::lidarr::lidarr_data::{ARTIST_DETAILS_BLOCKS, ActiveLidarrBlock};
use crate::ui::lidarr_ui::library::album_details_ui::AlbumDetailsUi;
use crate::ui::lidarr_ui::library::delete_album_ui::DeleteAlbumUi;
use crate::ui::lidarr_ui::lidarr_ui_utils::create_history_event_details;
use crate::ui::styles::{ManagarrStyle, secondary_style};
use crate::ui::utils::decorate_peer_style;
use crate::ui::utils::{
  borderless_block, get_width_from_percentage, layout_block_top_border, title_block,
};
use crate::ui::widgets::confirmation_prompt::ConfirmationPrompt;
use crate::ui::widgets::loading_block::LoadingBlock;
use crate::ui::widgets::managarr_table::ManagarrTable;
use crate::ui::widgets::message::Message;
use crate::ui::widgets::popup::{Popup, Size};
use crate::ui::{DrawUi, draw_popup, draw_tabs};
use crate::utils::convert_to_gb;
use ratatui::layout::Alignment;
use ratatui::text::Text;
use serde_json::Number;

#[cfg(test)]
#[path = "artist_details_ui_tests.rs"]
mod artist_details_ui_tests;

pub(super) struct ArtistDetailsUi;

impl DrawUi for ArtistDetailsUi {
  fn accepts(route: Route) -> bool {
    let Route::Lidarr(active_lidarr_block, _) = route else {
      return false;
    };
    AlbumDetailsUi::accepts(route)
      || DeleteAlbumUi::accepts(route)
      || ARTIST_DETAILS_BLOCKS.contains(&active_lidarr_block)
  }

  fn draw(f: &mut Frame<'_>, app: &mut App<'_>, area: Rect) {
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
          Layout::vertical([Constraint::Length(14), Constraint::Fill(0)])
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
          _ if DeleteAlbumUi::accepts(route) => DeleteAlbumUi::draw(f, app, area),
          ActiveLidarrBlock::ArtistHistoryDetails => {
            draw_artist_history_item_details_popup(f, app);
          }
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
          ActiveLidarrBlock::ManualArtistSearchConfirmPrompt => {
            draw_manual_artist_search_confirm_prompt(f, app);
          }
          _ => (),
        }
      };

      draw_popup(f, app, draw_artist_details_popup, Size::XXLarge);

      if AlbumDetailsUi::accepts(route) {
        AlbumDetailsUi::draw(f, app, area);
      }
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
      ActiveLidarrBlock::ArtistHistory => draw_artist_history_table(f, app, area),
      ActiveLidarrBlock::ManualArtistSearch => draw_artist_releases(f, app, area),
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

fn draw_artist_history_table(f: &mut Frame<'_>, app: &mut App<'_>, area: Rect) {
  if !app.is_loading {
    let current_selection = if app.data.lidarr_data.artist_history.is_empty() {
      LidarrHistoryItem::default()
    } else {
      app
        .data
        .lidarr_data
        .artist_history
        .current_selection()
        .clone()
    };

    if let Route::Lidarr(active_lidarr_block, _) = app.get_current_route() {
      let history_row_mapping = |history_item: &LidarrHistoryItem| {
        let LidarrHistoryItem {
          source_title,
          quality,
          event_type,
          date,
          ..
        } = history_item;

        source_title.scroll_left_or_reset(
          get_width_from_percentage(area, 40),
          current_selection == *history_item,
          app.ui_scroll_tick_count == 0,
        );

        Row::new(vec![
          Cell::from(source_title.to_string()),
          Cell::from(event_type.to_string()),
          Cell::from(quality.quality.name.to_owned()),
          Cell::from(date.to_string()),
        ])
        .primary()
      };
      let history_table = ManagarrTable::new(
        Some(&mut app.data.lidarr_data.artist_history),
        history_row_mapping,
      )
      .block(layout_block_top_border())
      .loading(app.is_loading)
      .sorting(active_lidarr_block == ActiveLidarrBlock::ArtistHistorySortPrompt)
      .searching(active_lidarr_block == ActiveLidarrBlock::SearchArtistHistory)
      .search_produced_empty_results(
        active_lidarr_block == ActiveLidarrBlock::SearchArtistHistoryError,
      )
      .filtering(active_lidarr_block == ActiveLidarrBlock::FilterArtistHistory)
      .filter_produced_empty_results(
        active_lidarr_block == ActiveLidarrBlock::FilterArtistHistoryError,
      )
      .headers(["Source Title", "Event Type", "Quality", "Date"])
      .constraints([
        Constraint::Percentage(40),
        Constraint::Percentage(20),
        Constraint::Percentage(15),
        Constraint::Percentage(25),
      ]);

      if [
        ActiveLidarrBlock::SearchArtistHistory,
        ActiveLidarrBlock::FilterArtistHistory,
      ]
      .contains(&active_lidarr_block)
      {
        history_table.show_cursor(f, area);
      }

      f.render_widget(history_table, area);
    }
  } else {
    f.render_widget(
      LoadingBlock::new(
        app.is_loading || app.data.lidarr_data.albums.is_empty(),
        layout_block_top_border(),
      ),
      area,
    );
  }
}

fn draw_artist_history_item_details_popup(f: &mut Frame<'_>, app: &mut App<'_>) {
  let current_selection = if app.data.lidarr_data.artist_history.is_empty() {
    LidarrHistoryItem::default()
  } else {
    app
      .data
      .lidarr_data
      .artist_history
      .current_selection()
      .clone()
  };

  let line_vec = create_history_event_details(current_selection);
  let text = Text::from(line_vec);

  let message = Message::new(text)
    .title("Details")
    .style(secondary_style())
    .alignment(Alignment::Left);

  f.render_widget(Popup::new(message).size(Size::NarrowLongMessage), f.area());
}

fn draw_artist_releases(f: &mut Frame<'_>, app: &mut App<'_>, area: Rect) {
  let (current_selection, is_empty) = if app.data.lidarr_data.discography_releases.is_empty() {
    (LidarrRelease::default(), true)
  } else {
    (
      app
        .data
        .lidarr_data
        .discography_releases
        .current_selection()
        .clone(),
      app.data.lidarr_data.discography_releases.is_empty(),
    )
  };

  if let Route::Lidarr(active_lidarr_block, _) = app.get_current_route() {
    let release_row_mapping = |release: &LidarrRelease| {
      let LidarrRelease {
        protocol,
        age,
        title,
        indexer,
        size,
        rejected,
        seeders,
        leechers,
        quality,
        ..
      } = release;

      let age = format!("{age} days");
      title.scroll_left_or_reset(
        get_width_from_percentage(area, 35),
        current_selection == *release
          && active_lidarr_block != ActiveLidarrBlock::ManualArtistSearchConfirmPrompt,
        app.ui_scroll_tick_count == 0,
      );
      let size = convert_to_gb(*size);
      let rejected_str = if *rejected { "⛔" } else { "" };
      let peers = if seeders.is_none() || leechers.is_none() {
        Text::from("")
      } else {
        let seeders = seeders
          .clone()
          .unwrap_or(Number::from(0u64))
          .as_u64()
          .unwrap();
        let leechers = leechers
          .clone()
          .unwrap_or(Number::from(0u64))
          .as_u64()
          .unwrap();

        decorate_peer_style(
          seeders,
          leechers,
          Text::from(format!("{seeders} / {leechers}")),
        )
      };

      let quality_name = quality.quality.name.clone();

      Row::new(vec![
        Cell::from(protocol.clone()),
        Cell::from(age),
        Cell::from(rejected_str),
        Cell::from(title.to_string()),
        Cell::from(indexer.clone()),
        Cell::from(format!("{size:.1} GB")),
        Cell::from(peers),
        Cell::from(quality_name),
      ])
      .primary()
    };
    let mut release_table = &mut app.data.lidarr_data.discography_releases;
    let artist_release_table = ManagarrTable::new(Some(&mut release_table), release_row_mapping)
      .block(layout_block_top_border())
      .loading(app.is_loading || is_empty)
      .sorting(active_lidarr_block == ActiveLidarrBlock::ManualArtistSearchSortPrompt)
      .headers([
        "Source", "Age", "⛔", "Title", "Indexer", "Size", "Peers", "Quality",
      ])
      .constraints([
        Constraint::Length(9),
        Constraint::Length(10),
        Constraint::Length(5),
        Constraint::Percentage(35),
        Constraint::Percentage(15),
        Constraint::Length(12),
        Constraint::Length(12),
        Constraint::Percentage(10),
      ]);

    f.render_widget(artist_release_table, area);
  }
}

fn draw_manual_artist_search_confirm_prompt(f: &mut Frame<'_>, app: &mut App<'_>) {
  let current_selection = app
    .data
    .lidarr_data
    .discography_releases
    .current_selection();
  let title = if current_selection.rejected {
    "Download Rejected Release"
  } else {
    "Download Release"
  };
  let prompt = if current_selection.rejected {
    format!(
      "Do you really want to download the rejected release: {}?",
      &current_selection.title.text
    )
  } else {
    format!(
      "Do you want to download the release: {}?",
      &current_selection.title.text
    )
  };

  if current_selection.rejected {
    let mut lines_vec = vec![Line::from("Rejection reasons: ".primary().bold())];
    let mut rejections_spans = current_selection
      .rejections
      .clone()
      .unwrap_or_default()
      .iter()
      .map(|item| Line::from(format!("• {item}").primary().bold()))
      .collect::<Vec<Line<'_>>>();
    lines_vec.append(&mut rejections_spans);

    let content_paragraph = Paragraph::new(lines_vec)
      .block(borderless_block())
      .wrap(Wrap { trim: false })
      .left_aligned();
    let confirmation_prompt = ConfirmationPrompt::new()
      .title(title)
      .prompt(&prompt)
      .content(content_paragraph)
      .yes_no_value(app.data.lidarr_data.prompt_confirm);

    f.render_widget(Popup::new(confirmation_prompt).size(Size::Small), f.area());
  } else {
    let confirmation_prompt = ConfirmationPrompt::new()
      .title(title)
      .prompt(&prompt)
      .yes_no_value(app.data.lidarr_data.prompt_confirm);

    f.render_widget(
      Popup::new(confirmation_prompt).size(Size::MediumPrompt),
      f.area(),
    );
  }
}
