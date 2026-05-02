//! Ryu-Editor - Main editor state machine, orchestration of buffer + keybinding + rhai + ui. Handles input loop, commands, async events (LSP, file I/O).\
//! this should know what to render (mode, filename, cursor pos) but not how

use color_eyre::Result;
use crossterm::event::{Event, EventStream, KeyCode, KeyModifiers};
use futures::{StreamExt};
use ratatui::{DefaultTerminal};
use ryu_core::{CursorPos, EditorView, Keymap};
use unicode_width::UnicodeWidthStr;

/// Run
pub async fn run() -> Result<()> {
    let terminal = ratatui::init();
    let result = run_app(terminal).await;
    ratatui::restore();
    result
}

async fn run_app(mut terminal: DefaultTerminal) -> Result<()> {
    let mut event_stream = EventStream::new();
    loop {
        let view = EditorView {
            keymap:     Keymap::Micro,
            filename: "main.rs".into(),
            cursor:   CursorPos { line: 1, col: 1 },
            filetype: "Rust".into(),
            modified: false,
        };
        terminal.draw(|f| ryu_ui::render(f, &view))?;
        
        tokio::select! {
            maybe_event = event_stream.next() => {
                if let Some(Ok(Event::Key(key))) = maybe_event {
                    if key.code == KeyCode::Char('q') && key.modifiers.contains(KeyModifiers::CONTROL) {
                        break;
                    }
                }
            }
        }
    }
    Ok(())
}