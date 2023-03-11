use tui::widgets::{Block, Borders};

const BLOCK_TITLE: &str = "Operation details";

/// Creates the Op Details widget.
pub fn create_op_details<'a>() -> Block<'a> {
    Block::default()
        .title(BLOCK_TITLE)
        .border_type(tui::widgets::BorderType::Rounded)
        .borders(Borders::ALL)
}
