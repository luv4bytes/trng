use tui::widgets::{Block, Borders};

const BLOCK_TITLE: &str = "Output";

/// Creates the Output widget.
pub fn create_output<'a>() -> Block<'a> {
    Block::default()
        .title(BLOCK_TITLE)
        .border_type(tui::widgets::BorderType::Rounded)
        .borders(Borders::ALL)
}
