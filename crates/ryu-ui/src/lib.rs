//! Ryu-UI - Ratatui-specific rendering, layout, components (statusline, command palette, line numbers, gutters), theme handling.
//! Knows *how* to render. Knows nothing about editor logic.


pub mod status_bar;
pub mod layout;

use ratatui::{Frame, style::Style, widgets::Paragraph};
use ryu_core::EditorView;

/// Renders the entire editor UI, including main content and status bar.
pub fn render(frame: &mut Frame, view: &EditorView) {
    let [main_area, status_area] = layout::editor_areas(frame.area());

    // Main content area — placeholder until ryu-buffer renders here
    frame.render_widget(
        Paragraph::new("Main editor area (buffer content goes here)")
            .style(Style::default()),
        main_area,
    );

    status_bar::render(frame, status_area, view);
}