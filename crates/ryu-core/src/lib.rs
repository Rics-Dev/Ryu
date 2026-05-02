//! Ryu-Core - Minimal shared types, traits, error types, configuration structs, and basic editor primitives. No heavy dependencies.
//! 
//! 
//! The keybinding → command dispatch design. 
//! Right now ryu-keybinding presumably maps key sequences to some Command enum or string. 
//! The hard question is: where does that enum live? If it lives in ryu-keybinding, 
//! then ryu-rhai and ryu-editor both need to depend on it to dispatch commands. 
//! Better: define a Command type in ryu-core, and ryu-keybinding only produces Command values — it never executes them. 
//! The executor lives in ryu-editor. This separation is what lets Rhai plugins dispatch the same commands as key presses.


/// The view-model contract between ryu-editor and ryu-ui.
/// ryu-editor writes into this. ryu-ui reads from this.
/// Neither crate knows the other exists — they only know ryu-core.
#[derive(Debug, Default, Clone)]
pub struct EditorView {
    pub keymap : Keymap,
    pub filename: String,
    pub cursor:   CursorPos,
    pub filetype: String,
    pub modified: bool,
}


#[derive(Debug, Default, Clone, PartialEq)]
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

#[derive(Debug, Default, Clone, Copy)]
pub struct CursorPos {
    pub line: usize,
    pub col:  usize,
}