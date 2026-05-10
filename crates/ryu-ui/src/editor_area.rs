use ratatui::{Frame, layout::Rect, text::{Line, Text}, widgets::Paragraph};
use ryu_core::{Viewport};

pub fn render(frame: &mut Frame, area: Rect, viewport: &Viewport) {
    
    let text = Text::from_iter(
        viewport.lines.iter().map(|s| Line::raw(s.as_str()))
    );
    
    frame.render_widget(Paragraph::new(text), area);

}