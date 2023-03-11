use tui::widgets::{Block, Borders};

const BLOCK_TITLE: &str = "Data Tape";

/// Creates the Data Tape widget.
pub fn create_data_tape<'a>() -> Block<'a> {
    Block::default()
        .title(BLOCK_TITLE)
        .border_type(tui::widgets::BorderType::Rounded)
        .borders(Borders::ALL)
}
