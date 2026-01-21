use crate::app::App;
use crate::models::Route;
use crate::models::lidarr_models::{LidarrHistoryItem, LidarrRelease, Track};
use crate::models::servarr_data::lidarr::lidarr_data::{ALBUM_DETAILS_BLOCKS, ActiveLidarrBlock};
use crate::ui::lidarr_ui::library::track_details_ui::TrackDetailsUi;
use crate::ui::lidarr_ui::lidarr_ui_utils::create_history_event_details;
use crate::ui::styles::{ManagarrStyle, secondary_style};
use crate::ui::utils::{
  borderless_block, decorate_peer_style, get_width_from_percentage, layout_block_top_border,
};
use crate::ui::widgets::confirmation_prompt::ConfirmationPrompt;
use crate::ui::widgets::loading_block::LoadingBlock;
use crate::ui::widgets::managarr_table::ManagarrTable;
use crate::ui::widgets::message::Message;
use crate::ui::widgets::popup::{Popup, Size};
use crate::ui::{DrawUi, draw_popup, draw_tabs};
use crate::utils::convert_to_gb;
use ratatui::Frame;
use ratatui::layout::{Alignment, Constraint, Rect};
use ratatui::prelude::{Line, Stylize, Text};
use ratatui::widgets::{Cell, Paragraph, Row, Wrap};
use serde_json::Number;

#[cfg(test)]
#[path = "album_details_ui_tests.rs"]
mod album_details_ui_tests;

pub(super) struct AlbumDetailsUi;

impl DrawUi for AlbumDetailsUi {
  fn accepts(route: Route) -> bool {
    let Route::Lidarr(active_lidarr_block, _) = route else {
      return false;
    };
    TrackDetailsUi::accepts(route) || ALBUM_DETAILS_BLOCKS.contains(&active_lidarr_block)
  }

  fn draw(f: &mut Frame<'_>, app: &mut App<'_>, _area: Rect) {
    let route = app.get_current_route();
    if app.data.lidarr_data.album_details_modal.is_some()
      && let Route::Lidarr(active_lidarr_block, _) = app.get_current_route()
    {
      let draw_album_details_popup = |f: &mut Frame<'_>, app: &mut App<'_>, popup_area: Rect| {
        let content_area = draw_tabs(
          f,
          popup_area,
          &format!(
            "{} Details",
            app.data.lidarr_data.albums.current_selection().title.text
          ),
          &app
            .data
            .lidarr_data
            .album_details_modal
            .as_ref()
            .expect("album_details_modal must exist in this context")
            .album_details_tabs,
        );
        draw_album_details(f, app, content_area);

        match active_lidarr_block {
          ActiveLidarrBlock::AutomaticallySearchAlbumPrompt => {
            let prompt = format!(
              "Do you want to trigger an automatic search of your indexers for the album: {}?",
              app.data.lidarr_data.albums.current_selection().title.text
            );
            let confirmation_prompt = ConfirmationPrompt::new()
              .title("Automatic Album Search")
              .prompt(&prompt)
              .yes_no_value(app.data.lidarr_data.prompt_confirm);

            f.render_widget(
              Popup::new(confirmation_prompt).size(Size::MediumPrompt),
              f.area(),
            );
          }
          ActiveLidarrBlock::DeleteTrackFilePrompt => {
            let prompt = format!(
              "Do you really want to delete this track file: \n{}?",
              app
                .data
                .lidarr_data
                .album_details_modal
                .as_ref()
                .expect("album_details_modal must exist in this context")
                .tracks
                .current_selection()
                .title
            );
            let confirmation_prompt = ConfirmationPrompt::new()
              .title("Delete Track File")
              .prompt(&prompt)
              .yes_no_value(app.data.lidarr_data.prompt_confirm);

            f.render_widget(
              Popup::new(confirmation_prompt).size(Size::MediumPrompt),
              f.area(),
            );
          }
          ActiveLidarrBlock::ManualAlbumSearchConfirmPrompt => {
            draw_manual_album_search_confirm_prompt(f, app);
          }
          ActiveLidarrBlock::AlbumHistoryDetails => {
            draw_history_item_details_popup(f, app);
          }
          _ => (),
        }
      };

      draw_popup(f, app, draw_album_details_popup, Size::XLarge);

      if TrackDetailsUi::accepts(route) {
        TrackDetailsUi::draw(f, app, _area);
      }
    }
  }
}

pub fn draw_album_details(f: &mut Frame<'_>, app: &mut App<'_>, area: Rect) {
  if let Some(album_details_modal) = app.data.lidarr_data.album_details_modal.as_ref()
    && let Route::Lidarr(active_lidarr_block, _) =
      album_details_modal.album_details_tabs.get_active_route()
  {
    match active_lidarr_block {
      ActiveLidarrBlock::AlbumDetails => draw_tracks_table(f, app, area),
      ActiveLidarrBlock::AlbumHistory => draw_album_history_table(f, app, area),
      ActiveLidarrBlock::ManualAlbumSearch => draw_album_releases(f, app, area),
      _ => (),
    }
  }
}

fn draw_tracks_table(f: &mut Frame<'_>, app: &mut App<'_>, area: Rect) {
  if let Route::Lidarr(active_lidarr_block, _) = app.get_current_route() {
    let track_files = app
      .data
      .lidarr_data
      .album_details_modal
      .as_ref()
      .expect("Album details modal is unpopulated")
      .track_files
      .items
      .clone();
    let content = Some(
      &mut app
        .data
        .lidarr_data
        .album_details_modal
        .as_mut()
        .expect("Album details modal is unpopulated")
        .tracks,
    );

    let track_row_mapping = |track: &Track| {
      let Track {
        track_number,
        title,
        duration,
        track_file_id,
        has_file,
        ..
      } = track;

      let track_file = track_files
        .iter()
        .find(|track_file| track_file.id == *track_file_id);
      let quality = if let Some(track_file) = track_file {
        track_file.quality.quality.name.to_owned()
      } else {
        String::new()
      };

      let audio_info = track_file
        .and_then(|tf| tf.media_info.as_ref())
        .map(|mi| {
          let codec = mi.audio_codec.as_deref().unwrap_or("");
          let channels = format!("{}.0", mi.audio_channels);
          let bitrate = mi.audio_bit_rate.as_deref().unwrap_or("");
          let sample_rate = mi.audio_sample_rate.as_deref().unwrap_or("");
          let bits = mi.audio_bits.as_deref().unwrap_or("");
          format!("{codec} - {channels} - {bitrate} - {sample_rate} - {bits}")
        })
        .unwrap_or_default();

      let duration_secs = duration / 1000;
      let mins = duration_secs / 60;
      let secs = duration_secs % 60;
      let duration_str = format!("{mins}:{secs:02}");

      let row = Row::new(vec![
        Cell::from(track_number.clone()),
        Cell::from(title.clone()),
        Cell::from(duration_str),
        Cell::from(audio_info),
        Cell::from(quality),
      ]);

      if *has_file {
        row.downloaded()
      } else {
        row.missing()
      }
    };

    let is_searching = active_lidarr_block == ActiveLidarrBlock::SearchTracks;
    let tracks_table = ManagarrTable::new(content, track_row_mapping)
      .block(layout_block_top_border())
      .loading(app.is_loading)
      .searching(is_searching)
      .search_produced_empty_results(active_lidarr_block == ActiveLidarrBlock::SearchTracksError)
      .headers(["#", "Title", "Duration", "Audio Info", "Quality"])
      .constraints([
        Constraint::Percentage(5),
        Constraint::Percentage(35),
        Constraint::Percentage(8),
        Constraint::Percentage(37),
        Constraint::Percentage(15),
      ]);

    if is_searching {
      tracks_table.show_cursor(f, area);
    }

    f.render_widget(tracks_table, area);
  }
}

fn draw_album_history_table(f: &mut Frame<'_>, app: &mut App<'_>, area: Rect) {
  match app.data.lidarr_data.album_details_modal.as_ref() {
    Some(album_details_modal) if !app.is_loading => {
      let current_selection = if album_details_modal.album_history.is_empty() {
        LidarrHistoryItem::default()
      } else {
        album_details_modal
          .album_history
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
        let mut album_history_table = &mut app
          .data
          .lidarr_data
          .album_details_modal
          .as_mut()
          .expect("album_details_modal must exist in this context")
          .album_history;
        let history_table = ManagarrTable::new(Some(&mut album_history_table), history_row_mapping)
          .block(layout_block_top_border())
          .loading(app.is_loading)
          .sorting(active_lidarr_block == ActiveLidarrBlock::AlbumHistorySortPrompt)
          .searching(active_lidarr_block == ActiveLidarrBlock::SearchAlbumHistory)
          .search_produced_empty_results(
            active_lidarr_block == ActiveLidarrBlock::SearchAlbumHistoryError,
          )
          .filtering(active_lidarr_block == ActiveLidarrBlock::FilterAlbumHistory)
          .filter_produced_empty_results(
            active_lidarr_block == ActiveLidarrBlock::FilterAlbumHistoryError,
          )
          .headers(["Source Title", "Event Type", "Quality", "Date"])
          .constraints([
            Constraint::Percentage(40),
            Constraint::Percentage(20),
            Constraint::Percentage(15),
            Constraint::Percentage(25),
          ]);

        if [
          ActiveLidarrBlock::SearchAlbumHistory,
          ActiveLidarrBlock::FilterAlbumHistory,
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
        app.is_loading || app.data.lidarr_data.album_details_modal.is_none(),
        layout_block_top_border(),
      ),
      area,
    ),
  }
}

fn draw_album_releases(f: &mut Frame<'_>, app: &mut App<'_>, area: Rect) {
  match app.data.lidarr_data.album_details_modal.as_ref() {
    Some(album_details_modal) if !app.is_loading => {
      let (current_selection, is_empty) = if album_details_modal.album_releases.is_empty() {
        (LidarrRelease::default(), true)
      } else {
        (
          album_details_modal
            .album_releases
            .current_selection()
            .clone(),
          album_details_modal.album_releases.is_empty(),
        )
      };

      if let Route::Lidarr(active_lidarr_block, _) = app.get_current_route() {
        let album_release_row_mapping = |release: &LidarrRelease| {
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
              && active_lidarr_block != ActiveLidarrBlock::ManualAlbumSearchConfirmPrompt,
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
        let mut album_release_table = &mut app
          .data
          .lidarr_data
          .album_details_modal
          .as_mut()
          .expect("album_details_modal must exist in this context")
          .album_releases;
        let release_table =
          ManagarrTable::new(Some(&mut album_release_table), album_release_row_mapping)
            .block(layout_block_top_border())
            .loading(app.is_loading || is_empty)
            .sorting(active_lidarr_block == ActiveLidarrBlock::ManualAlbumSearchSortPrompt)
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

        f.render_widget(release_table, area);
      }
    }
    _ => f.render_widget(
      LoadingBlock::new(
        app.is_loading || app.data.lidarr_data.album_details_modal.is_none(),
        layout_block_top_border(),
      ),
      area,
    ),
  }
}

fn draw_manual_album_search_confirm_prompt(f: &mut Frame<'_>, app: &mut App<'_>) {
  let current_selection = app
    .data
    .lidarr_data
    .album_details_modal
    .as_ref()
    .expect("album_details_modal must exist in this context")
    .album_releases
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

fn draw_history_item_details_popup(f: &mut Frame<'_>, app: &mut App<'_>) {
  let current_selection =
    if let Some(album_details_modal) = app.data.lidarr_data.album_details_modal.as_ref() {
      if album_details_modal.album_history.is_empty() {
        LidarrHistoryItem::default()
      } else {
        album_details_modal
          .album_history
          .current_selection()
          .clone()
      }
    } else {
      LidarrHistoryItem::default()
    };

  let line_vec = create_history_event_details(current_selection);
  let text = Text::from(line_vec);

  let message = Message::new(text)
    .title("Details")
    .style(secondary_style())
    .alignment(Alignment::Left);

  f.render_widget(Popup::new(message).size(Size::NarrowLongMessage), f.area());
}
