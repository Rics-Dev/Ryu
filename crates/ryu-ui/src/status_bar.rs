//! Status bar component — the single row at the bottom of the editor.

use ratatui::{
    Frame,
    layout::{Constraint, Flex, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
};
use ryu_core::EditorView;

pub fn render(frame: &mut Frame, area: Rect, view: &EditorView) {
    // Fill background first so the whole row is colored
    frame.render_widget(
        Paragraph::new("").style(Style::default().bg(Color::Rgb(45, 45, 74))),
        area,
    );

    let left = Line::from(vec![
        Span::styled(format!(" {} ", view.keymap.label()), mode_style()),
        Span::styled(format!(" {} ", view.filename), left_style()),
    ]);

    let right = Line::from(vec![
        Span::styled(
            format!(" UTF-8 │ {} │ Ln {}, Col {} ", view.filetype, view.cursor.line, view.cursor.col),
            right_style(),
        ),
    ]);

    let left_width  = left.width()  as u16;
    let right_width = right.width() as u16;

    let [left_area, right_area] = Layout::horizontal([
        Constraint::Length(left_width),
        Constraint::Length(right_width),
    ])
    .flex(Flex::SpaceBetween)
    .areas(area);

    frame.render_widget(Paragraph::new(left),  left_area);
    frame.render_widget(Paragraph::new(right), right_area);
}

fn mode_style() -> Style {
    Style::default()
        .fg(Color::Rgb(212, 208, 255))
        .bg(Color::Rgb(95, 94, 138))
        .add_modifier(Modifier::BOLD)
}

fn left_style() -> Style {
    Style::default()
        .fg(Color::Rgb(160, 160, 192))
        .bg(Color::Rgb(45, 45, 74))
}

fn right_style() -> Style {
    Style::default()
        .fg(Color::Rgb(106, 106, 138))
        .bg(Color::Rgb(45, 45, 74))
}