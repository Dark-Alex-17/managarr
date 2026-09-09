use crate::app::App;
use crate::models::Route;
use crate::models::lidarr_models::Artist;
use crate::models::servarr_data::lidarr::lidarr_data::ARTIST_OVERVIEW_BLOCKS;
use crate::ui::styles::{primary_style, unmonitored_style};
use crate::ui::utils::title_block_centered;
use crate::ui::widgets::loading_block::LoadingBlock;
use crate::ui::widgets::popup::Size;
use crate::ui::{DrawUi, draw_popup};
use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::Style;
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Paragraph, Wrap};

#[cfg(test)]
#[path = "artist_overview_ui_tests.rs"]
mod artist_overview_ui_tests;

pub(super) struct ArtistOverviewUi;

impl DrawUi for ArtistOverviewUi {
  fn accepts(route: Route) -> bool {
    let Route::Lidarr(active_lidarr_block, _) = route else {
      return false;
    };
    ARTIST_OVERVIEW_BLOCKS.contains(&active_lidarr_block)
  }

  fn draw(f: &mut Frame<'_>, app: &mut App<'_>, _area: Rect) {
    if app.data.lidarr_data.artist_overview_modal.is_some() {
      let draw_artist_overview_popup = |f: &mut Frame<'_>, app: &mut App<'_>, popup_area: Rect| {
        draw_artist_overview(f, app, popup_area);
      };

      draw_popup(f, app, draw_artist_overview_popup, Size::Large);
    }
  }
}

fn draw_artist_overview(f: &mut Frame<'_>, app: &App<'_>, area: Rect) {
  let block = title_block_centered("Overview");

  match app.data.lidarr_data.artist_overview_modal.as_ref() {
    Some(artist_overview_modal) if !app.is_loading => {
      let style = if app.data.lidarr_data.artists.is_empty() {
        primary_style()
      } else {
        style_from_artist(app.data.lidarr_data.artists.current_selection())
      };
      let overview = &artist_overview_modal.overview;
      let text = Text::from(
        overview
          .items
          .iter()
          .map(|line| overview_line(line, style))
          .collect::<Vec<Line<'static>>>(),
      );

      let paragraph = Paragraph::new(text)
        .block(block)
        .wrap(Wrap { trim: false })
        .scroll((overview.offset, 0));

      f.render_widget(paragraph, area);
    }
    _ => f.render_widget(
      LoadingBlock::new(
        app.is_loading || app.data.lidarr_data.artist_overview_modal.is_none(),
        block,
      ),
      area,
    ),
  }
}

fn style_from_artist(artist: &Artist) -> Style {
  if !artist.monitored {
    return unmonitored_style();
  }

  primary_style()
}

fn overview_line(line: &str, style: Style) -> Line<'static> {
  Line::from(Span::styled(line.trim_end().to_owned(), style))
}
