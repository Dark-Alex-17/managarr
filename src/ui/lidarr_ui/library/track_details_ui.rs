use crate::app::App;
use crate::models::Route;
use crate::models::lidarr_models::{LidarrHistoryItem, Track};
use crate::models::servarr_data::lidarr::lidarr_data::{ActiveLidarrBlock, TRACK_DETAILS_BLOCKS};
use crate::ui::lidarr_ui::lidarr_ui_utils::create_history_event_details;
use crate::ui::styles::ManagarrStyle;
use crate::ui::styles::{downloaded_style, missing_style, secondary_style};
use crate::ui::utils::{get_width_from_percentage, layout_block_top_border};
use crate::ui::widgets::loading_block::LoadingBlock;
use crate::ui::widgets::managarr_table::ManagarrTable;
use crate::ui::widgets::message::Message;
use crate::ui::widgets::popup::{Popup, Size};
use crate::ui::{DrawUi, draw_popup, draw_tabs};
use ratatui::Frame;
use ratatui::layout::{Alignment, Constraint, Rect};
use ratatui::style::{Style, Stylize};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Cell, Paragraph, Row, Wrap};

#[cfg(test)]
#[path = "track_details_ui_tests.rs"]
mod track_details_ui_tests;

pub(super) struct TrackDetailsUi;

impl DrawUi for TrackDetailsUi {
  fn accepts(route: Route) -> bool {
    let Route::Lidarr(active_lidarr_block, _) = route else {
      return false;
    };
    TRACK_DETAILS_BLOCKS.contains(&active_lidarr_block)
  }

  fn draw(f: &mut Frame<'_>, app: &mut App<'_>, _area: Rect) {
    if let Some(album_details_modal) = app.data.lidarr_data.album_details_modal.as_ref()
      && album_details_modal.track_details_modal.is_some()
      && let Route::Lidarr(active_lidarr_block, _) = app.get_current_route()
    {
      let draw_track_details_popup = |f: &mut Frame<'_>, app: &mut App<'_>, popup_area: Rect| {
        let content_area = draw_tabs(
          f,
          popup_area,
          "Track Details",
          &app
            .data
            .lidarr_data
            .album_details_modal
            .as_ref()
            .expect("album_details_modal must exist in this context")
            .track_details_modal
            .as_ref()
            .expect("track_details_modal must exist in this context")
            .track_details_tabs,
        );
        draw_track_details_tabs(f, app, content_area);

        if active_lidarr_block == ActiveLidarrBlock::TrackHistoryDetails {
          draw_history_item_details_popup(f, app);
        }
      };

      draw_popup(f, app, draw_track_details_popup, Size::Large);
    }
  }
}

pub fn draw_track_details_tabs(f: &mut Frame<'_>, app: &mut App<'_>, area: Rect) {
  if let Some(album_details_modal) = app.data.lidarr_data.album_details_modal.as_ref()
    && let Some(track_details_modal) = album_details_modal.track_details_modal.as_ref()
    && let Route::Lidarr(active_lidarr_block, _) =
      track_details_modal.track_details_tabs.get_active_route()
  {
    match active_lidarr_block {
      ActiveLidarrBlock::TrackDetails => draw_track_details(f, app, area),
      ActiveLidarrBlock::TrackHistory => draw_track_history_table(f, app, area),
      _ => (),
    }
  }
}

fn draw_track_details(f: &mut Frame<'_>, app: &App<'_>, area: Rect) {
  let block = layout_block_top_border();

  match app.data.lidarr_data.album_details_modal.as_ref() {
    Some(album_details_modal) if !app.is_loading => {
      if let Some(track_details_modal) = album_details_modal.track_details_modal.as_ref() {
        let track = album_details_modal.tracks.current_selection().clone();
        let track_details = &track_details_modal.track_details;
        let text = Text::from(
          track_details
            .items
            .iter()
            .filter(|it| !it.is_empty())
            .map(|line| {
              let split = line.split(':').collect::<Vec<&str>>();
              let title = format!("{}:", split[0]);
              let style = style_from_status(&track);

              Line::from(vec![
                title.bold().style(style),
                Span::styled(split[1..].join(":"), style),
              ])
            })
            .collect::<Vec<Line<'_>>>(),
        );

        let paragraph = Paragraph::new(text)
          .block(block)
          .wrap(Wrap { trim: false })
          .scroll((track_details.offset, 0));

        f.render_widget(paragraph, area);
      }
    }
    _ => f.render_widget(
      LoadingBlock::new(
        app.is_loading
          || app
            .data
            .lidarr_data
            .album_details_modal
            .as_ref()
            .expect("album_details_modal must exist in this context")
            .track_details_modal
            .is_none(),
        block,
      ),
      area,
    ),
  }
}

fn draw_track_history_table(f: &mut Frame<'_>, app: &mut App<'_>, area: Rect) {
  match app.data.lidarr_data.album_details_modal.as_ref() {
    Some(album_details_modal) if !app.is_loading => {
      let Route::Lidarr(active_lidarr_block, _) = app.get_current_route() else {
        panic!("Non-Lidarr route is being used");
      };
      if let Some(track_details_modal) = album_details_modal.track_details_modal.as_ref() {
        let current_selection = if track_details_modal.track_history.is_empty() {
          LidarrHistoryItem::default()
        } else {
          track_details_modal
            .track_history
            .current_selection()
            .clone()
        };

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
        let mut track_history_table = &mut app
          .data
          .lidarr_data
          .album_details_modal
          .as_mut()
          .expect("album_details_modal must exist in this context")
          .track_details_modal
          .as_mut()
          .expect("track_details_modal must exist in this context")
          .track_history;
        let history_table = ManagarrTable::new(Some(&mut track_history_table), history_row_mapping)
          .block(layout_block_top_border())
          .loading(app.is_loading)
          .sorting(active_lidarr_block == ActiveLidarrBlock::TrackHistorySortPrompt)
          .searching(active_lidarr_block == ActiveLidarrBlock::SearchTrackHistory)
          .search_produced_empty_results(
            active_lidarr_block == ActiveLidarrBlock::SearchTrackHistoryError,
          )
          .filtering(active_lidarr_block == ActiveLidarrBlock::FilterTrackHistory)
          .filter_produced_empty_results(
            active_lidarr_block == ActiveLidarrBlock::FilterTrackHistoryError,
          )
          .headers(["Source Title", "Event Type", "Quality", "Date"])
          .constraints([
            Constraint::Percentage(40),
            Constraint::Percentage(20),
            Constraint::Percentage(15),
            Constraint::Percentage(25),
          ]);

        f.render_widget(history_table, area);
      }
    }
    _ => f.render_widget(
      LoadingBlock::new(
        app.is_loading
          || app
            .data
            .lidarr_data
            .album_details_modal
            .as_ref()
            .expect("album_details_modal must exist in this context")
            .track_details_modal
            .is_none(),
        layout_block_top_border(),
      ),
      area,
    ),
  }
}

fn draw_history_item_details_popup(f: &mut Frame<'_>, app: &mut App<'_>) {
  let current_selection =
    if let Some(album_details_modal) = app.data.lidarr_data.album_details_modal.as_ref() {
      if let Some(track_details_modal) = album_details_modal.track_details_modal.as_ref() {
        if track_details_modal.track_history.is_empty() {
          LidarrHistoryItem::default()
        } else {
          track_details_modal
            .track_history
            .current_selection()
            .clone()
        }
      } else {
        LidarrHistoryItem::default()
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

fn style_from_status(track: &Track) -> Style {
  if !track.has_file {
    return missing_style();
  }

  downloaded_style()
}
