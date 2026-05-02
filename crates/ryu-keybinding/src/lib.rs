//! Ryu-Keybindings - Keybinding engine, parser, keymap storage, personalities (Micro, Vim, Helix, Custom). Handles mode switching.
//! 
//! The keybinding → command dispatch design. 
//! Right now ryu-keybinding presumably maps key sequences to some Command enum or string. 
//! The hard question is: where does that enum live? If it lives in ryu-keybinding, 
//! then ryu-rhai and ryu-editor both need to depend on it to dispatch commands. 
//! Better: define a Command type in ryu-core, and ryu-keybinding only produces Command values — it never executes them. 
//! The executor lives in ryu-editor. This separation is what lets Rhai plugins dispatch the same commands as key presses.