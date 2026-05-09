//! Ryu-Core - Minimal shared types, traits, error types, configuration structs, and editor primitives. 
//! No heavy dependencies. Everything here is a contract between crates.
//! 
//! 
//! The keybinding → command dispatch design. 
//! Right now ryu-keybinding presumably maps key sequences to some Command enum or string. 
//! The hard question is: where does that enum live? If it lives in ryu-keybinding, 
//! then ryu-rhai and ryu-editor both need to depend on it to dispatch commands. 
//! Better: define a Command type in ryu-core, and ryu-keybinding only produces Command values — it never executes them. 
//! The executor lives in ryu-editor. This separation is what lets Rhai plugins dispatch the same commands as key presses.
//! 


// ---------------------------------------------------------------------------
// Buffer identity
// ---------------------------------------------------------------------------

/// A unique identifier for a buffer (open document).
/// Buffers outlive windows — closing a window doesn't close the buffer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BufferId(pub u32);

// ---------------------------------------------------------------------------
// Cursor & selection
// ---------------------------------------------------------------------------

/// A position within a buffer, in (line, column) space.
/// Line and column are both 0-indexed internally.
/// Unicode column = grapheme cluster offset, not byte offset.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct CursorPos {
    pub line: usize,
    pub col:  usize,
}

// ---------------------------------------------------------------------------
// View — one window's perspective into a buffer
// ---------------------------------------------------------------------------

/// A View is the per-window state: which buffer it's looking at,
/// the cursor position, scroll offset, and selection.
/// One Buffer can have multiple Views (e.g., a vertical split).
#[derive(Debug, Clone)]
pub struct View {
    pub buffer_id:  BufferId,
    pub cursor:     CursorPos,
    /// The rope line index at the top of the visible area.
    pub scroll_top: usize,
}

// ---------------------------------------------------------------------------
// Viewport — pre-rendered snapshot for the UI layer
// ---------------------------------------------------------------------------

/// A pre-computed snapshot of what a window should display.
/// Produced by ryu-buffer (or ryu-editor), consumed by ryu-ui.
/// Never contains the full Rope — only what's visible right now.
#[derive(Debug, Default, Clone)]
pub struct Viewport {
    /// Only the lines visible on screen (already Unicode-aware strings).
    pub lines:       Vec<String>,
    /// The rope line index at the top of the viewport.
    pub scroll_top:  usize,
    /// Total number of lines in the full document.
    pub total_lines: usize,
}

// ---------------------------------------------------------------------------
// Window — a rectangular region of the terminal displaying a View
// ---------------------------------------------------------------------------

/// A Window renders one View. The editor can have multiple Windows
/// arranged in a split layout. Each Window has an independent cursor
/// and scroll position, even when two Windows share the same Buffer.
#[derive(Debug, Clone)]
pub struct Window {
    pub view:     View,
    pub viewport: Viewport,
    /// Display name: the file name, "[No Name]", or "[scratch]".
    pub title:    String,
}


/// Future for splits
#[derive(Debug, Clone)]
pub struct WindowTree {
}

// ---------------------------------------------------------------------------
// EditorState — the view-model contract between ryu-editor and ryu-ui
// ---------------------------------------------------------------------------

/// Everything ryu-ui needs to render one frame.
/// ryu-editor writes this; ryu-ui reads it.
/// Neither crate depends on the other — only on ryu-core.
#[derive(Debug, Default, Clone)]
pub struct EditorState {
    pub window:   Option<Window>,
    
}

// ---------------------------------------------------------------------------
// KeyMap
// ---------------------------------------------------------------------------

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Keymap {
    #[default]
    Micro,
    Vim,
    Helix,
}

impl Keymap {
    pub fn label(&self) -> &'static str {
        match self {
            Keymap::Micro  => "Micro",
            Keymap::Vim  => "Vim",
            Keymap::Helix => "Helix",
        }
    }
}


#[derive(Debug, Default, Clone, PartialEq)]
pub enum ViMode {
    #[default]
    Normal,
    Insert,
    Visual(VisualType),
    Command,
    Replace,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum VisualType {
    Char,  // Standard 'v'
    Line,  // 'Shift-v'
    Block, // 'Ctrl-v'
}



impl ViMode {
    pub fn label(&self) -> &'static str {
        match self {
            ViMode::Normal  => "NORMAL",
            ViMode::Insert  => "INSERT",
            ViMode::Visual(visual_type)  => match visual_type {
                VisualType::Char => "VISUAL",
                VisualType::Line => "V-LINE",
                VisualType::Block => "V-BLOCK",
            },
            ViMode::Command => "COMMAND",
            ViMode::Replace => "REPLACE",
        }
    }
}