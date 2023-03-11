use tui::widgets::{Block, Borders};

const BLOCK_TITLE: &str = "Code Editor";

/// Creates the Code Editor widget.
pub fn create_code_editor<'a>() -> Block<'a> {
    Block::default()
        .title(BLOCK_TITLE)
        .border_type(tui::widgets::BorderType::Rounded)
        .borders(Borders::ALL)
}
