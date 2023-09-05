pub use std::{
    io::{self, Write},
    process::{Command, Stdio},
    str::{self, FromStr},
};

pub use crate::{
    colors::CSI, ClearLine, ClearScreen, Cursor, Scroll
};

impl Scroll {
    /// Scrolls the terminal screen up `num` lines.
    #[must_use]
    pub fn up(num: u8) -> String {
        format!("{CSI}{num}S")
    }

    /// Scrolls the terminal screen down `num` lines.
    #[must_use]
    pub fn down(num: u8) -> String {
        format!("{CSI}{num}T")
    }
}

impl ClearScreen {
    /// Clears the full terminal screen.
    #[must_use]
    pub fn all() -> String {
        format!("{CSI}2J")
    }

    /// Clears the terminal screen from the cursor to the end of the screen.
    #[must_use]
    pub fn to_end() -> String {
        format!("{CSI}0J")
    }

    /// Clears the terminal screen from the cursor to the beginning of the screen.
    #[must_use]
    pub fn to_start() -> String {
        format!("{CSI}1J")
    }

    /// Clears the full terminal screen.
    pub fn clr_scr(w: &mut io::StdoutLock) {
        w.write_all(format!("{CSI}2J").as_bytes()).unwrap();
    }
}

impl ClearLine {
    /// Clears the current line.
    #[must_use]
    pub fn all() -> String {
        format!("{CSI}2K")
    }

    /// Clears the current line from the cursor to the end of the line.
    #[must_use]
    pub fn to_end() -> String {
        format!("{CSI}0K")
    }

    /// Clears the current line from the cursor to the beginning of the line.
    #[must_use]
    pub fn to_start() -> String {
        format!("{CSI}1K")
    }

    /// Clears the current line.
    pub fn clr_line(w: &mut io::StdoutLock) {
        w.write_all(format!("{CSI}2K").as_bytes()).unwrap();
    }
}

impl Cursor {
    /// Shows the terminal cursor.
    #[must_use]
    pub fn show() -> String {
        format!("{CSI}?25h")
    }

    /// Hides the terminal cursor.
    #[must_use]
    pub fn hide() -> String {
        format!("{CSI}?25l")
    }

    /// Moves the cursor `num` cells up.
    #[must_use]
    pub fn up(num: u8) -> String {
        format!("{CSI}{num}A")
    }

    /// Moves the cursor `num` cells down.
    #[must_use]
    pub fn down(num: u8) -> String {
        format!("{CSI}{num}B")
    }

    /// Moves the cursor `num` cells forward.
    #[must_use]
    pub fn forward(num: u8) -> String {
        format!("{CSI}{num}C")
    }

    /// Moves the cursor `num` cells backward.
    #[must_use]
    pub fn back(num: u8) -> String {
        format!("{CSI}{num}D")
    }

    /// Moves the cursor to column `num`.
    #[must_use]
    pub fn column(num: u8) -> String {
        format!("{CSI}{num}G")
    }

    /// Moves the cursor to row `row` and column `col`.
    #[must_use]
    pub fn goto(row: u8, col: u8) -> String {
        format!("{CSI}{row};{col}H")
    }

    /// Moves the cursor to the bottom left position on the screen.
    pub fn goto_bl(rows: u8, w: &mut io::StdoutLock) {
        w.write_all(Cursor::goto(rows, 0).as_bytes()).unwrap();
    }

    /// Moves the cursor to the top left position on the screen.
    pub fn goto_tl(w: &mut io::StdoutLock) {
        w.write_all(Cursor::goto(1, 1).as_bytes()).unwrap();
    }
}

/// Writes a message that is centered on the screen.
pub fn write_centered_msg(row: u8, width: u8, msg: &[u8], w: &mut io::StdoutLock) {
    let len = u8::try_from(msg.len()).unwrap();
    if len > width { return; }

    let col = (width / 2) - (len / 2);
    write_msg(row, col, msg, w);
}

/// Writes a message to a given position on the screen.
pub fn write_msg(row: u8, col: u8, msg: &[u8], w: &mut io::StdoutLock) {
    w.write_all(Cursor::goto(row, col).as_bytes()).unwrap();
    w.write_all(msg).unwrap();
}

/// Attempts to get the terminal size using tput and returns a sensible default
/// if there is an error.
///
/// Returns (height, width).
pub fn get_terminal_size() -> (u8, u8) {
    // A child process executed with the output() method does not inherit
    // the parent process' stdin by default.
    //
    // So, we must ensure that stdin is inherited from the parent process
    // in order for tput to query the correct terminal's size.
    let Ok(tput_out) = Command::new("tput")
        .args(["cols", "lines"])
        .stdin(Stdio::inherit())
        .output() else { return (80, 20) };

    let mut size_iter = tput_out.stdout.split(|byte| *byte == b'\n');

    let Some(width_bytes) = size_iter.next() else { return (80, 20) };
    let Some(height_bytes) = size_iter.next() else { return (80, 20) };

    let Ok(width_str) = str::from_utf8(width_bytes) else { return (80, 20) };
    let Ok(height_str) = str::from_utf8(height_bytes) else { return (80, 20) };

    let Ok(width) = u8::from_str(width_str) else { return (80, 20) };
    let Ok(height) = u8::from_str(height_str) else { return (80, 20) };

    (height, width)
}
