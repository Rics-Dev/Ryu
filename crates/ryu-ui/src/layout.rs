//! Layout helpers — splits the terminal area into named regions.

use ratatui::layout::{Constraint, Layout, Rect};

/// Returns [main_area, status_area] — status bar pinned to the bottom.
pub fn editor_areas(area: Rect) -> [Rect; 2] {
    Layout::vertical([
        Constraint::Min(1),     // editor body
        Constraint::Length(1),  // status bar
    ])
    .areas(area)
}