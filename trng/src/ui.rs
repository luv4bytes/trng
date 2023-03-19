// TRNG - Brainfucks pretty sister.
// Copyright (C) 2023 Lukas Pfeifer

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use std::io::Stdout;

use libtrng::Interpreter;
use tui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    widgets::{Block, BorderType, Borders},
    Terminal,
};

use crate::widgets;

/// The default window title.
const WINDOW_TITLE: &str = "TRNG - Brainfucks pretty sister.";

/// Holds data used by the UI.
pub struct Ui<'a> {
    interpreter: &'a Interpreter,
}

impl Ui<'_> {
    /// Constructs a new Ui struct. This does not automatically draw the ui.
    /// # Argu
    pub fn new(interpreter: &Interpreter) -> Ui {
        Ui {
            interpreter: interpreter,
        }
    }

    /// Main function for drawing all UI components.
    /// * `terminal` - The terminal to draw on.
    pub fn draw_ui<'a>(
        &'a self,
        terminal: &'a mut Terminal<CrosstermBackend<Stdout>>,
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
}
