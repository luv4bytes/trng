use std::io::Stdout;

use tui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    widgets::{Block, BorderType, Borders},
    Terminal,
};

use crate::widgets;

/// The default window title.
const WINDOW_TITLE: &str = "TRNG - Brainfucks pretty sister.";

/// Main function for drawing all UI components.
/// * `terminal` - The terminal to draw on.
pub fn draw_ui(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
) -> Result<tui::terminal::CompletedFrame, std::io::Error> {
    terminal.draw(|frame| {
        let base_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
            .margin(1)
            .split(frame.size());

        let outer_block = Block::default()
            .title(WINDOW_TITLE)
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);
        frame.render_widget(outer_block, frame.size());

        let left_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(80), Constraint::Percentage(20)].as_ref())
            .split(base_chunks[0]);

        let data_tape_widget = widgets::create_data_tape();
        frame.render_widget(data_tape_widget, left_chunks[0]);

        let output_widget = widgets::create_output();
        frame.render_widget(output_widget, left_chunks[1]);

        let right_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Percentage(20),
                    Constraint::Percentage(60),
                    Constraint::Percentage(20),
                ]
                .as_ref(),
            )
            .split(base_chunks[1]);

        let settings_widget = widgets::create_settings();
        frame.render_widget(settings_widget, right_chunks[0]);

        let editor_widget = widgets::create_code_editor();
        frame.render_widget(editor_widget, right_chunks[1]);

        let op_details_widget = widgets::create_op_details();
        frame.render_widget(op_details_widget, right_chunks[2]);
    })
}
