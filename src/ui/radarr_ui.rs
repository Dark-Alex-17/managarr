use std::ops::Sub;

use chrono::{Duration, Utc};
use tui::{Frame, symbols};
use tui::backend::Backend;
use tui::layout::{Alignment, Constraint, Rect};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans, Text};
use tui::widgets::{Block, Borders, Cell, LineGauge, Paragraph, Row, Table};

use crate::app::App;
use crate::app::radarr::RadarrData;
use crate::logos::RADARR_LOGO;
use crate::network::radarr_network::Movie;
use crate::ui::HIGHLIGHT_SYMBOL;
use crate::ui::utils::{style_default, style_highlight, style_primary, style_secondary, style_tertiary, title_block, vertical_chunks, vertical_chunks_with_margin};

pub(super) fn draw_radarr_ui<B: Backend>(f: &mut Frame<'_, B>, app: &mut App, area: Rect) {
    let block = Block::default().borders(Borders::ALL).title(Spans::from(vec![
        Span::styled("Movies", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
    ]));

    let rows = app.data.radarr_data.movies.items
        .iter()
        .map(|movie| {
            let runtime = movie.runtime.as_u64().unwrap();
            let hours = runtime / 60;
            let minutes = runtime % 60;
            let file_size: f64 = movie.size_on_disk.as_u64().unwrap() as f64 / 1024f64.powi(3);

            Row::new(vec![
                Cell::from(movie.title.to_owned()),
                Cell::from(movie.year.to_string()),
                Cell::from(format!("{:0width$}:{:0width$}", hours, minutes, width = 2)),
                Cell::from(format!("{:.2} GB", file_size)),
                Cell::from(app.data.radarr_data.quality_profile_map.get(&movie.quality_profile_id.as_u64().unwrap()).unwrap().to_owned())
            ]).style(determine_row_style(app, movie))
        });
    let header_row = Row::new(vec!["Title", "Year", "Runtime", "Size", "Quality Profile"])
        .style(style_default())
        .bottom_margin(0);
    let constraints = vec![
        Constraint::Percentage(20),
        Constraint::Percentage(20),
        Constraint::Percentage(20),
        Constraint::Percentage(20),
        Constraint::Percentage(20),
    ];

    let table = Table::new(rows)
        .header(header_row)
        .block(block)
        .highlight_style(style_highlight())
        .highlight_symbol(HIGHLIGHT_SYMBOL)
        .widths(&constraints);

    f.render_stateful_widget(table, area, &mut app.data.radarr_data.movies.state)
}

pub(super) fn draw_stats<B: Backend>(f: &mut Frame<'_, B>, app: &App, area: Rect) {
    let RadarrData {
        free_space,
        total_space,
        start_time,
        ..
    } = app.data.radarr_data;
    let ratio = if total_space == 0 {
        0f64
    } else {
        1f64 - (free_space as f64 / total_space as f64)
    };

    f.render_widget(title_block("Stats"), area);

    let chunks =
        vertical_chunks_with_margin(vec![
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Min(2)], area, 1);

    let version_paragraph = Paragraph::new(Text::from(format!(
        "Radarr Version:  {}",
        app.data.radarr_data.version
    ))).block(Block::default());

    let uptime = Utc::now().sub(start_time);
    let days = uptime.num_days();
    let day_difference = uptime.sub(Duration::days(days));
    let hours = day_difference.num_hours();
    let hour_difference = day_difference.sub(Duration::hours(hours));
    let minutes = hour_difference.num_minutes();
    let seconds = hour_difference.sub(Duration::minutes(minutes)).num_seconds();

    let uptime_paragraph = Paragraph::new(Text::from(format!(
        "Uptime: {}d {:0width$}:{:0width$}:{:0width$}",
        days, hours, minutes, seconds, width = 2
    ))).block(Block::default());

    let space_gauge = LineGauge::default()
        .block(Block::default().title("Storage:"))
        .gauge_style(Style::default().fg(Color::Cyan))
        .line_set(symbols::line::THICK)
        .ratio(ratio)
        .label(Spans::from(format!("{:.0}%", ratio * 100.0)));

    f.render_widget(version_paragraph, chunks[0]);
    f.render_widget(uptime_paragraph, chunks[1]);
    f.render_widget(space_gauge, chunks[2]);
}

pub(super) fn draw_logo<B: Backend>(f: &mut Frame<'_, B>, area: Rect) {
    let chunks = vertical_chunks(
        vec![Constraint::Percentage(60), Constraint::Percentage(40)],
        area,
    );
    let logo = Paragraph::new(Text::from(RADARR_LOGO))
        .block(Block::default())
        .alignment(Alignment::Center);

    f.render_widget(logo, chunks[0]);
}

fn determine_row_style(app: &App, movie: &Movie) -> Style {
    let downloads_vec = &app.data.radarr_data.downloads.items;

    if !movie.has_file {
        if let Some(download) = downloads_vec.iter()
            .find(|&download | download.movie_id == movie.id) {
            if download.status == "downloading" {
                return style_secondary()
            }
        }

        return style_tertiary()
    }

    style_primary()
}