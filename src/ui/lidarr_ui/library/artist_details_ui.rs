use crate::app::App;
use crate::models::lidarr_models::{
  Album, AlbumStatistics, LidarrHistoryEventType, LidarrHistoryItem,
};
use crate::models::servarr_data::lidarr::lidarr_data::{ActiveLidarrBlock, ARTIST_DETAILS_BLOCKS};
use crate::models::Route;
use crate::ui::lidarr_ui::library::album_details_ui::AlbumDetailsUi;
use crate::ui::lidarr_ui::library::track_details_ui::TrackDetailsUi;
// TODO
// use crate::ui::lidarr_ui::lidarr_ui_utils::{
//   create_download_failed_history_event_details,
//   create_download_folder_imported_history_event_details, create_grabbed_history_event_details,
//   create_no_data_history_event_details, create_track_file_deleted_history_event_details,
//   create_track_file_renamed_history_event_details,
// };
use crate::ui::styles::ManagarrStyle;
use crate::ui::utils::{
  borderless_block, get_width_from_percentage, layout_block_top_border, title_block,
};
use crate::ui::widgets::confirmation_prompt::ConfirmationPrompt;
use crate::ui::widgets::loading_block::LoadingBlock;
use crate::ui::widgets::managarr_table::ManagarrTable;
use crate::ui::widgets::message::Message;
use crate::ui::widgets::popup::{Popup, Size};
use crate::ui::{draw_popup, draw_tabs, DrawUi};
use crate::utils::convert_to_gb;
use chrono::Utc;
use deunicode::deunicode;
use ratatui::layout::{Alignment, Constraint, Layout, Rect};
use ratatui::style::{Style, Stylize};
use ratatui::text::{Line, Text};
use ratatui::widgets::{Cell, Paragraph, Row, Wrap};
use ratatui::Frame;
use regex::Regex;
#[cfg(test)]
#[path = "artist_details_ui_tests.rs"]
mod artist_details_ui_tests;
pub(super) struct ArtistDetailsUi;
impl DrawUi for ArtistDetailsUi {
  fn accepts(route: Route) -> bool {
    if let Route::Lidarr(active_lidarr_block, _) = route {
      return AlbumDetailsUi::accepts(route)
        || TrackDetailsUi::accepts(route)
        || ARTIST_DETAILS_BLOCKS.contains(&active_lidarr_block);
    }
    false
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
            let prompt = format!(              "Do you want to trigger an automatic search of your indexers for all monitored album(s) for the artist: {}", app.data.lidarr_data.artists.current_selection().artist_name            );
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
          ActiveLidarrBlock::ArtistHistoryDetails => {
            draw_history_item_details_popup(f, app, popup_area);
          }
          _ => (),
        };
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
    .unwrap()
    .to_owned();
  let metadata_profile = app
    .data
    .lidarr_data
    .metadata_profiles_map
    .get_by_left(&current_selection.metadata_profile_id)
    .unwrap()
    .to_owned();
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
      "Artist Name: ".primary().bold(),
      current_selection.artist_name.text.clone().primary().bold(),
    ]),
    Line::from(vec!["Overview: ".primary().bold(), overview.default()]),
    Line::from(vec![
      "Status: ".primary().bold(),
      current_selection.status.to_display_str().default(),
    ]),
    Line::from(vec![
      "Genres: ".primary().bold(),
      current_selection.genres.join(", ").default(),
    ]),
    Line::from(vec![
      "Rating: ".primary().bold(),
      format!("{}%", (current_selection.ratings.value * 10.0) as i32).default(),
    ]),
    Line::from(vec![
      "Path: ".primary().bold(),
      current_selection.path.clone().default(),
    ]),
    Line::from(vec![
      "Quality Profile: ".primary().bold(),
      quality_profile.default(),
    ]),
    Line::from(vec![
      "Metadata Profile: ".primary().bold(),
      metadata_profile.default(),
    ]),
    Line::from(vec!["Monitored: ".primary().bold(), monitored.default()]),
  ];
  if let Some(stats) = current_selection.statistics.as_ref() {
    let size = convert_to_gb(stats.size_on_disk);
    artist_description.extend(vec![Line::from(vec![
      "Size on Disk: ".primary().bold(),
      format!("{size:.2} GB").default(),
    ])]);
  }
  let description_paragraph = Paragraph::new(artist_description)
    .block(borderless_block())
    .wrap(Wrap { trim: true });
  f.render_widget(description_paragraph, area);
}
pub fn draw_artist_details(f: &mut Frame<'_>, app: &mut App<'_>, area: Rect) {
  if let Route::Lidarr(active_lidarr_block, _) =
    app.data.lidarr_data.artist_info_tabs.get_active_route()
  {
    match active_lidarr_block {
      ActiveLidarrBlock::ArtistDetails => draw_albums_table(f, app, area),
      ActiveLidarrBlock::ArtistHistory => draw_artist_history_table(f, app, area),
      _ => (),
    }
  }
}
fn draw_albums_table(f: &mut Frame<'_>, app: &mut App<'_>, area: Rect) {
  if let Route::Lidarr(active_lidarr_block, _) = app.get_current_route() {
    let content = Some(&mut app.data.lidarr_data.albums);
    let help_footer = app
      .data
      .lidarr_data
      .artist_info_tabs
      .get_active_tab_contextual_help();
    let album_row_mapping = |album: &Album| {
      let Album {
        title,
        monitored,
        statistics,
        ..
      } = album;
      let AlbumStatistics {
        track_file_count,
        track_count,
        size_on_disk,
        next_airing,
        ..
      } = if let Some(stats) = statistics {
        stats
      } else {
        &AlbumStatistics::default()
      };
      let album_monitored = if album.monitored { "🏷" } else { "" };
      let size = convert_to_gb(*size_on_disk);
      let row = Row::new(vec![
        Cell::from(album_monitored.to_owned()),
        Cell::from(title.to_string()),
        Cell::from(format!("{}/{}", track_file_count, track_count)),
        Cell::from(format!("{size:.2} GB")),
      ]);
      if !monitored {
        row.unmonitored()
      } else if track_file_count == track_count {
        row.downloaded()
      } else if let Some(next_airing_utc) = next_airing.as_ref() {
        if next_airing_utc > &Utc::now() {
          return row.unreleased();
        } else {
          return row.missing();
        }
      } else {
        row.missing()
      }
    };
    let is_searching = active_lidarr_block == ActiveLidarrBlock::SearchAlbum;
    let album_table = ManagarrTable::new(content, album_row_mapping)
      .block(layout_block_top_border())
      .loading(app.is_loading)
      .footer(help_footer)
      .searching(is_searching)
      .search_produced_empty_results(active_lidarr_block == ActiveLidarrBlock::SearchAlbumError)
      .headers(["Monitored", "Album", "Track Count", "Size on Disk"])
      .constraints([
        Constraint::Percentage(6),
        Constraint::Ratio(1, 3),
        Constraint::Ratio(1, 3),
        Constraint::Ratio(1, 3),
      ]);
    if is_searching {
      album_table.show_cursor(f, area);
    }
    f.render_widget(album_table, area);
  }
}
fn draw_artist_history_table(f: &mut Frame<'_>, app: &mut App<'_>, area: Rect) {
  match app.data.lidarr_data.artist_history.as_ref() {
    Some(artist_history) if !app.is_loading => {
      let current_selection = if artist_history.is_empty() {
        LidarrHistoryItem::default()
      } else {
        artist_history.current_selection().clone()
      };
      let artist_history_table_footer = app
        .data
        .lidarr_data
        .artist_info_tabs
        .get_active_tab_contextual_help();
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
            app.tick_count % app.ticks_until_scroll == 0,
          );
          Row::new(vec![
            Cell::from(source_title.to_string()),
            Cell::from(event_type.to_string()),
            Cell::from(quality.quality.name.to_owned()),
            Cell::from(date.to_string()),
          ])
          .primary()
        };
        let mut artist_history_table = app.data.lidarr_data.artist_history.as_mut().unwrap();
        let history_table =
          ManagarrTable::new(Some(&mut artist_history_table), history_row_mapping)
            .block(layout_block_top_border())
            .loading(app.is_loading)
            .footer(artist_history_table_footer)
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
              Constraint::Percentage(15),
              Constraint::Percentage(13),
              Constraint::Percentage(20),
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
    }
    _ => f.render_widget(
      LoadingBlock::new(
        app.is_loading || app.data.lidarr_data.albums.is_empty(),
        layout_block_top_border(),
      ),
      area,
    ),
  }
}
fn draw_history_item_details_popup(f: &mut Frame<'_>, app: &mut App<'_>, area: Rect) {
  let current_selection =
    if let Some(artist_history_items) = app.data.lidarr_data.artist_history.as_ref() {
      if artist_history_items.is_empty() {
        LidarrHistoryItem::default()
      } else {
        artist_history_items.current_selection().clone()
      }
    } else {
      LidarrHistoryItem::default()
    };
  // TODO
  // let line_vec = match current_selection.event_type {
  //   LidarrHistoryEventType::Grabbed => create_grabbed_history_event_details(current_selection),
  //   LidarrHistoryEventType::DownloadFolderImported => {
  //     create_download_folder_imported_history_event_details(current_selection)
  //   }
  //   LidarrHistoryEventType::DownloadFailed => {
  //     create_download_failed_history_event_details(current_selection)
  //   }
  //   LidarrHistoryEventType::TrackFileDeleted => {
  //     create_track_file_deleted_history_event_details(current_selection)
  //   }
  //   LidarrHistoryEventType::TrackFileRenamed => {
  //     create_track_file_renamed_history_event_details(current_selection)
  //   }
  //   _ => create_no_data_history_event_details(current_selection),
  // };
  // let text = Text::from(line_vec);
  let text = Text::from(String::new());
  let message = Message::new(text)
    .title("Details")
    .style(Style::new().secondary())
    .alignment(Alignment::Left);
  f.render_widget(Popup::new(message).size(Size::NarrowMessage), area);
}
