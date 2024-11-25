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
            Constraint::Percentage(60),
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

fn get_grid_position(mouse_x: i32, mouse_y: i32) -> Option<(i32, i32)> {
    None
    /*
    let relative_x = mouse_x - app.start_x as i32;
    let relative_y = mouse_y - app.start_y as i32;

    if relative_x <= 0
        || relative_y <= 0
        || relative_x >= (9 * app.cell_width) as i32
        || relative_y >= (9 * app.cell_height) as i32
    {
        return None;
    }

    let grid_x = relative_x / app.cell_width as i32;
    let grid_y = relative_x / app.cell_width as i32;

    if (0..9).contains(&grid_x) && (0..9).contains(&grid_y) {
        Some((grid_x, grid_y))
    } else {
        None
    }*/
}
