use itertools::Itertools;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    text::{Span, Text},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

use crate::{app::App, grid::Grid};

pub fn ui(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(1),
        ])
        .split(frame.area());

    render_title(frame, chunks[0]);
    render_main_window(frame, chunks[1]);
    render_footer(frame, chunks[2]);
}

fn render_title(frame: &mut Frame, r: Rect) {
    let title_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .style(Style::new().white().on_black());

    let title = Paragraph::new(Text::styled("Sudoku", Style::default().fg(Color::Green)))
        .centered()
        .block(title_block);

    frame.render_widget(title, r)
}

fn render_footer(frame: &mut Frame, r: Rect) {
    let keys = [("M/m", "Menu"), ("Q/Esc", "Quit")];
    let spans = keys
        .iter()
        .flat_map(|(key, desc)| {
            let key = Span::styled(
                format!(" {key} "),
                Style::new().fg(Color::Black).bg(Color::DarkGray),
            );
            let desc = Span::styled(
                format!(" {desc} "),
                Style::new().fg(Color::DarkGray).bg(Color::Black),
            );
            [key, desc]
        })
        .collect_vec();

    let footer = ratatui::text::Line::from(spans)
        .centered()
        .style(Style::new().on_black());

    frame.render_widget(footer, r);
}

fn render_main_window(frame: &mut Frame, r: Rect) {
    let middle_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(70), Constraint::Fill(1)])
        .split(r);

    let board_chunk = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Fill(1),
            Constraint::Percentage(95),
            Constraint::Fill(1),
        ])
        .split(
            Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Fill(1),
                    Constraint::Percentage(95),
                    Constraint::Fill(1),
                ])
                .split(middle_chunks[0])[1],
        )[1];

    let grid = Grid::default();

    frame.render_widget(grid, board_chunk);
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
