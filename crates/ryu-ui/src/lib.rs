//! Ryu-UI - Ratatui-specific rendering, layout, components (statusline, command palette, line numbers, gutters), theme handling.
//! Knows *how* to render. Knows nothing about editor logic.

pub mod layout;
pub mod editor_area;


use ratatui::{Frame};
use ryu_core::{EditorState};

/// Renders the entire editor UI, including main content and status bar.
pub fn render(frame: &mut Frame, state: &EditorState) {
    let Some(window) = &state.window else { return };
    let editor_area = layout::editor_area(frame.area());

    editor_area::render(frame, editor_area, &window.viewport);

    if state.cursor_visible {
        frame.set_cursor_position((
            window.view.cursor.col  as u16,
            window.view.cursor.line.saturating_sub(window.view.scroll_top) as u16,
        ));
    }
}