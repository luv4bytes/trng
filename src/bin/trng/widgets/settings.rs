use tui::widgets::{Block, Borders};

const BLOCK_TITLE: &str = "Settings";

/// Creates the Settings widget.
pub fn create_settings<'a>() -> Block<'a> {
    Block::default()
        .title(BLOCK_TITLE)
        .border_type(tui::widgets::BorderType::Rounded)
        .borders(Borders::ALL)
}
