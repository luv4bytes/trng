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

mod ui;
mod widgets;

use std::io;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use libtrng::Interpreter;
use tui::{backend::CrosstermBackend, Terminal};

fn main() -> Result<(), std::io::Error> {
    let mut terminal = create_terminal()?;
    let interpreter = Interpreter::default();
    let ui = ui::Ui::new(&interpreter);

    loop {
        ui.draw_ui(&mut terminal)?;

        if let Event::Key(key) = event::read()? {
            if let KeyCode::Char('q') = key.code {
                return exit_terminal(&mut terminal);
            }
        }
    }
}

/// Creates the tui terminal.
fn create_terminal() -> Result<Terminal<CrosstermBackend<io::Stdout>>, io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend);

    terminal
}

/// Exits tui and restores the default terminal.
/// * `terminal` - Reference to the created tui terminal.
fn exit_terminal(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> Result<(), io::Error> {
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    Ok(())
}
