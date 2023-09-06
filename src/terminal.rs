pub use std::{
    io::{self, Write},
    process::{Command, Stdio},
    str::{self, FromStr},
};

pub use crate::{colors::CSI, Clear, Cursor, Scroll};

impl Scroll {
    /// Scrolls the terminal screen up `num` lines.
    pub fn up(num: u8, w: &mut io::StdoutLock) -> io::Result<()> {
        w.write_all(format!("{CSI}{num}S").as_bytes())
    }

    /// Scrolls the terminal screen down `num` lines.
    pub fn down(num: u8, w: &mut io::StdoutLock) -> io::Result<()> {
        w.write_all(format!("{CSI}{num}T").as_bytes())
    }
}

impl Clear {
    /// Clears the full terminal screen.
    pub fn scr(w: &mut io::StdoutLock) -> io::Result<()> {
        w.write_all(format!("{CSI}2J").as_bytes())
    }

    /// Clears the terminal screen from the cursor to the beginning of the screen.
    pub fn scr_to_start(w: &mut io::StdoutLock) -> io::Result<()> {
        w.write_all(format!("{CSI}1J").as_bytes())
    }

    /// Clears the terminal screen from the cursor to the end of the screen.
    pub fn scr_to_end(w: &mut io::StdoutLock) -> io::Result<()> {
        w.write_all(format!("{CSI}0J").as_bytes())
    }

    /// Clears the current line.
    pub fn line(w: &mut io::StdoutLock) -> io::Result<()> {
        w.write_all(format!("{CSI}2K").as_bytes())
    }

    /// Clears the current line from the cursor to the beginning of the line.
    pub fn line_to_start(w: &mut io::StdoutLock) -> io::Result<()> {
        w.write_all(format!("{CSI}1K").as_bytes())
    }

    /// Clears the current line from the cursor to the end of the line.
    pub fn line_to_end(w: &mut io::StdoutLock) -> io::Result<()> {
        w.write_all(format!("{CSI}0K").as_bytes())
    }
}

impl Cursor {
    /// Shows the terminal cursor.
    pub fn show(w: &mut io::StdoutLock) -> io::Result<()> {
        w.write_all(format!("{CSI}?25h").as_bytes())
    }

    /// Hides the terminal cursor.
    pub fn hide(w: &mut io::StdoutLock) -> io::Result<()> {
        w.write_all(format!("{CSI}?25l").as_bytes())
    }

    /// Moves the cursor `num` cells up.
    pub fn up(num: u8, w: &mut io::StdoutLock) -> io::Result<()> {
        w.write_all(format!("{CSI}{num}A").as_bytes())
    }

    /// Moves the cursor `num` cells down.
    pub fn down(num: u8, w: &mut io::StdoutLock) -> io::Result<()> {
        w.write_all(format!("{CSI}{num}B").as_bytes())
    }

    /// Moves the cursor `num` cells forward.
    pub fn forward(num: u8, w: &mut io::StdoutLock) -> io::Result<()> {
        w.write_all(format!("{CSI}{num}C").as_bytes())
    }

    /// Moves the cursor `num` cells backward.
    pub fn back(num: u8, w: &mut io::StdoutLock) -> io::Result<()> {
        w.write_all(format!("{CSI}{num}D").as_bytes())
    }

    /// Moves the cursor to column `num`.
    pub fn column(num: u8, w: &mut io::StdoutLock) -> io::Result<()> {
        w.write_all(format!("{CSI}{num}G").as_bytes())
    }

    /// Moves the cursor to row `row` and column `col`.
    pub fn goto(row: u8, col: u8, w: &mut io::StdoutLock) -> io::Result<()> {
        w.write_all(format!("{CSI}{row};{col}H").as_bytes())
    }

    /// Moves the cursor to the bottom left position on the screen.
    pub fn goto_bl(rows: u8, w: &mut io::StdoutLock) -> io::Result<()> {
        w.write_all(format!("{CSI}{rows};1H").as_bytes())
    }

    /// Moves the cursor to the top left position on the screen.
    pub fn goto_tl(w: &mut io::StdoutLock) -> io::Result<()> {
        w.write_all(format!("{CSI}1;1H").as_bytes())
    }
}

/// Writes a message that is centered on the screen.
pub fn write_centered_msg(row: u8, width: u8, msg: &str, w: &mut io::StdoutLock) -> io::Result<()> {
    let len = u8::try_from(msg.len()).unwrap();

    if len > width {
        return Ok(());
    }

    let col = (width / 2) - (len / 2);
    w.write_all(format!("{CSI}{row};{col}H{msg}").as_bytes())
}

/// Writes a message to a given position on the screen.
pub fn write_msg(row: u8, col: u8, msg: &str, w: &mut io::StdoutLock) -> io::Result<()> {
    w.write_all(format!("{CSI}{row};{col}H{msg}").as_bytes())
}

/// Attempts to get the terminal size using tput and returns a sensible default
/// if there is an error.
///
/// Returns (height, width).
#[must_use]
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
