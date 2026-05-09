use ratatui::{Frame, layout::Rect, text::{Line, Text}, widgets::Paragraph};
use ryu_core::{Viewport};

pub fn render(frame: &mut Frame, area: Rect, view_port: &Viewport) {
    let lines: Vec<Line> = view_port
        .lines
        .iter()
        .map(|s: &String| Line::raw(s.as_str()))
        .collect();

    frame.render_widget(
        Paragraph::new(Text::from(lines)),
        area,
    );
}