use crate::env;

use std::io;

pub enum Mode {
    Normal,
    Insert,
    Caret,
    Passthrough,
}

/// Sends the command `enter-mode {mode}` to qutebrowser to enter the specified mode.
pub fn enter_mode(mode: Mode) -> Result<(), io::Error> {
    let mode_str = match mode {
        Mode::Normal => "normal",
        Mode::Insert => "insert",
        Mode::Caret => "caret",
        Mode::Passthrough => "passthrough",
    };

    let message = format!("enter-mode {}", mode_str);
    send_command(&message)
}

/// Sends text to qutebrowser as raw text input (`fake-key {string}`).
pub fn fake_key(s: &str) -> Result<(), io::Error> {
    let message = format!("fake-key {}", s);
    send_command(&message)
}

pub fn send_command(cmd: &str) -> Result<(), io::Error> {
    let fifo = env::fifo();
    fifo.write(cmd)
}
