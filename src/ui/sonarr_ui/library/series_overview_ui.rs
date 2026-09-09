use crate::app::App;
use crate::models::Route;
use crate::models::servarr_data::sonarr::sonarr_data::SERIES_OVERVIEW_BLOCKS;
use crate::models::sonarr_models::Series;
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
#[path = "series_overview_ui_tests.rs"]
mod series_overview_ui_tests;

pub(super) struct SeriesOverviewUi;

impl DrawUi for SeriesOverviewUi {
  fn accepts(route: Route) -> bool {
    let Route::Sonarr(active_sonarr_block, _) = route else {
      return false;
    };
    SERIES_OVERVIEW_BLOCKS.contains(&active_sonarr_block)
  }

  fn draw(f: &mut Frame<'_>, app: &mut App<'_>, _area: Rect) {
    if app.data.sonarr_data.series_overview_modal.is_some() {
      let draw_series_overview_popup = |f: &mut Frame<'_>, app: &mut App<'_>, popup_area: Rect| {
        draw_series_overview(f, app, popup_area);
      };

      draw_popup(f, app, draw_series_overview_popup, Size::Large);
    }
  }
}

fn draw_series_overview(f: &mut Frame<'_>, app: &App<'_>, area: Rect) {
  let block = title_block_centered("Overview");

  match app.data.sonarr_data.series_overview_modal.as_ref() {
    Some(series_overview_modal) if !app.is_loading => {
      let style = if app.data.sonarr_data.series.is_empty() {
        primary_style()
      } else {
        style_from_series(app.data.sonarr_data.series.current_selection())
      };
      let overview = &series_overview_modal.overview;
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
        app.is_loading || app.data.sonarr_data.series_overview_modal.is_none(),
        block,
      ),
      area,
    ),
  }
}

fn style_from_series(series: &Series) -> Style {
  if !series.monitored {
    return unmonitored_style();
  }

  primary_style()
}

fn overview_line(line: &str, style: Style) -> Line<'static> {
  Line::from(Span::styled(line.trim_end().to_owned(), style))
}
