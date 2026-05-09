//! Ryu-Buffer - Text buffer implementation (ropey + tree-sitter integration), undo/redo, basic operations (insert/delete), selections, cursor management.

use std::path::PathBuf;
use ropey::Rope;
use ryu_core::BufferId;

pub struct Buffer {
    pub id:   BufferId,
    /// None = scratch buffer (no file on disk yet)
    pub path: Option<PathBuf>,
    rope: Rope,
    pub dirty: bool, // unsaved changes
}

impl Buffer {
    /// Create a scratch buffer with no file.
    pub fn scratch(id: BufferId) -> Self {
        Self {
            id,
            path:  None,
            rope:  Rope::new(),
            dirty: false,
        }
    }

    /// Load a file from disk into a buffer.
    pub fn from_file(id: BufferId, path: PathBuf) -> color_eyre::Result<Self> {
        let text = std::fs::read_to_string(&path)?;
        Ok(Self {
            id,
            path:  Some(path),
            rope:  Rope::from_str(&text),
            dirty: false,
        })
    }

    pub fn len_lines(&self) -> usize {
        self.rope.len_lines()
    }

    /// Returns the line at `index` as a String, with the trailing newline stripped.
    pub fn line(&self, index: usize) -> String {
        let s = self.rope.line(index).to_string();
        s.trim_end_matches('\n').to_string()
    }

    /// Display name for the title bar / status line.
    pub fn display_name(&self) -> &str {
        self.path
            .as_deref()
            .and_then(|p| p.file_name())
            .and_then(|n| n.to_str())
            .unwrap_or("[scratch]")
    }
}