//! Ryu-Editor - Main editor state machine, orchestration of buffer + keybinding + rhai + ui. Handles input loop, commands, async events (LSP, file I/O).\
//! this should know what to render (mode, filename, cursor pos) but not how
//! ryu-editor answers: "what should the user see right now?"
//! This is where the decision "I need lines 10 through 49" is made. 
//! Then it asks ryu-buffer for each of those lines individually, assembles the Viewport, and hands it to ryu-ui

use std::path::PathBuf;
use color_eyre::Result;
use crossterm::event::{Event, EventStream, KeyCode, KeyModifiers};
use futures::StreamExt;
use ratatui::DefaultTerminal;
use ryu_buffer::Buffer;
use ryu_core::{BufferId, CursorPos, EditorState, View, Viewport, Window};

struct Editor {
    buffer: Buffer,
    view:   View,
}

impl Editor {
    fn new(file: Option<PathBuf>) -> Result<Self> {
        let id = BufferId(0);
        let buffer = match file {
            Some(path) => Buffer::from_file(id, path)?,
            None       => Buffer::scratch(id),
        };
        let view = View {
            buffer_id:  id,
            cursor:     CursorPos::default(),
            scroll_top: 0,
        };
        Ok(Self { buffer, view })
    }

    /// Compute the Viewport + EditorState the UI needs for this frame.
    fn render_state(&self, visible_rows: usize) -> EditorState {
        let total_lines = self.buffer.len_lines();
        let scroll_top  = self.view.scroll_top;
        let end_line    = (scroll_top + visible_rows).min(total_lines);
    
        let lines = (scroll_top..end_line)
            .map(|i| self.buffer.line(i))
            .collect();
    
        let viewport = Viewport { lines, scroll_top, total_lines };
        let window   = Window {
            view:     self.view.clone(),
            viewport,
            title:    self.buffer.display_name().to_string(),
        };
    
        EditorState { window: Some(window) }
    }
}

pub async fn run(file: Option<PathBuf>) -> Result<()> {
    let terminal = ratatui::init();
    let result   = run_app(terminal, file).await;
    ratatui::restore();
    result
}

async fn run_app(mut terminal: DefaultTerminal, file: Option<PathBuf>) -> Result<()> {
    let editor       = Editor::new(file)?;
    let mut event_stream = EventStream::new();

    loop {
        let size  = terminal.size()?;
        // Reserve 1 row for the status bar (future), rest is text area
        let visible_rows = size.height.saturating_sub(1) as usize;
        let state = editor.render_state(visible_rows);

        terminal.draw(|f| ryu_ui::render(f, &state))?;

        tokio::select! {
            maybe_event = event_stream.next() => {
                match maybe_event {
                    Some(Ok(Event::Key(key))) => {
                        if key.code == KeyCode::Char('q')
                            && key.modifiers.contains(KeyModifiers::CONTROL)
                        {
                            break;
                        }
                    }
                    Some(Err(e)) => return Err(e.into()),
                    _ => {}
                }
            }
        }
    }

    Ok(())
}