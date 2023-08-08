use tui::backend::Backend;
use tui::Frame;
use tui::layout::{Constraint, Direction, Layout};
use tui::style::{Color, Style};
use tui::widgets::{Block, Borders, Gauge};

use crate::app::App;
use crate::app::radarr::RadarrData;

pub fn ui<B: Backend>(f: &mut Frame<B>, app: &App) {
    let RadarrData { free_space, total_space } = app.data.radarr_data;
    let ratio = if total_space == 0 {
        0f64
    } else {
        1f64 - (free_space as f64 / total_space as f64)
    };

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(f.size());

    let gauge = Gauge::default()
        .block(Block::default()
            .title("Free Space")
            .borders(Borders::ALL))
        .gauge_style(Style::default().fg(Color::Cyan))
        .ratio(ratio);

    f.render_widget(gauge, chunks[0]);
}