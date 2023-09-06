pub use std::{
    io::{self, Write},
    process::{Command, Stdio},
    str::FromStr,
};

pub use crate::{colors::CSI, Term};

impl Term {
    /// Scrolls the terminal screen up `num` lines.
    pub fn scroll_up<W: Write>(num: u8, w: &mut W) -> io::Result<()> {
        w.write_all(format!("{CSI}{num}S").as_bytes())
    }

    /// Scrolls the terminal screen down `num` lines.
    pub fn scroll_down<W: Write>(num: u8, w: &mut W) -> io::Result<()> {
        w.write_all(format!("{CSI}{num}T").as_bytes())
    }

    /// Clears the full terminal screen.
    pub fn clr_scr<W: Write>(w: &mut W) -> io::Result<()> {
        w.write_all(format!("{CSI}2J").as_bytes())
    }

    /// Clears the terminal screen from the cursor to the beginning of the screen.
    pub fn clr_scr_to_start<W: Write>(w: &mut W) -> io::Result<()> {
        w.write_all(format!("{CSI}1J").as_bytes())
    }

    /// Clears the terminal screen from the cursor to the end of the screen.
    pub fn clr_scr_to_end<W: Write>(w: &mut W) -> io::Result<()> {
        w.write_all(format!("{CSI}0J").as_bytes())
    }

    /// Clears the current line.
    pub fn clr_line<W: Write>(w: &mut W) -> io::Result<()> {
        w.write_all(format!("{CSI}2K").as_bytes())
    }

    /// Clears the current line from the cursor to the beginning of the line.
    pub fn clr_line_to_start<W: Write>(w: &mut W) -> io::Result<()> {
        w.write_all(format!("{CSI}1K").as_bytes())
    }

    /// Clears the current line from the cursor to the end of the line.
    pub fn clr_line_to_end<W: Write>(w: &mut W) -> io::Result<()> {
        w.write_all(format!("{CSI}0K").as_bytes())
    }

    /// Shows the terminal cursor.
    pub fn show_cursor<W: Write>(w: &mut W) -> io::Result<()> {
        w.write_all(format!("{CSI}?25h").as_bytes())
    }

    /// Hides the terminal cursor.
    pub fn hide_cursor<W: Write>(w: &mut W) -> io::Result<()> {
        w.write_all(format!("{CSI}?25l").as_bytes())
    }

    /// Moves the cursor `num` cells up.
    pub fn cursor_up<W: Write>(num: u8, w: &mut W) -> io::Result<()> {
        w.write_all(format!("{CSI}{num}A").as_bytes())
    }

    /// Moves the cursor `num` cells down.
    pub fn cursor_down<W: Write>(num: u8, w: &mut W) -> io::Result<()> {
        w.write_all(format!("{CSI}{num}B").as_bytes())
    }

    /// Moves the cursor `num` cells right.
    pub fn cursor_right<W: Write>(num: u8, w: &mut W) -> io::Result<()> {
        w.write_all(format!("{CSI}{num}C").as_bytes())
    }

    /// Moves the cursor `num` cells left.
    pub fn cursor_left<W: Write>(num: u8, w: &mut W) -> io::Result<()> {
        w.write_all(format!("{CSI}{num}D").as_bytes())
    }

    /// Moves the cursor to column `num`.
    pub fn cursor_to_col<W: Write>(num: u8, w: &mut W) -> io::Result<()> {
        w.write_all(format!("{CSI}{num}G").as_bytes())
    }

    /// Moves the cursor to row `row` and column `col`.
    pub fn cursor_goto<W: Write>(row: u8, col: u8, w: &mut W) -> io::Result<()> {
        w.write_all(format!("{CSI}{row};{col}H").as_bytes())
    }

    /// Moves the cursor to the bottom left position on the screen.
    pub fn cursor_bottomleft<W: Write>(rows: u8, w: &mut W) -> io::Result<()> {
        w.write_all(format!("{CSI}{rows};1H").as_bytes())
    }

    /// Moves the cursor to the top left position on the screen.
    pub fn cursor_topleft<W: Write>(w: &mut W) -> io::Result<()> {
        w.write_all(format!("{CSI}1;1H").as_bytes())
    }

    /// Writes a message that is centered on the screen.
    pub fn write_centered<W: Write>(row: u8, width: u8, msg: &str, w: &mut W) -> io::Result<()> {
        let Ok(len) = u8::try_from(msg.len()) else {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Unable to convert the message length to u8."
            ));
        };

        if len > width {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Message cannot be longer than the screen width."
            ));
        }

        let col = (width / 2) - (len / 2);
        w.write_all(format!("{CSI}{row};{col}H{msg}").as_bytes())
    }

    /// Writes a message to a given position on the screen.
    pub fn write<W: Write>(row: u8, col: u8, msg: &str, w: &mut W) -> io::Result<()> {
        w.write_all(format!("{CSI}{row};{col}H{msg}").as_bytes())
    }

    /// Attempts to get the terminal size using `tput` and returns a sensible default
    /// of (20, 80) if there is an error.
    ///
    /// Returns (height, width).
    #[must_use]
    pub fn get_terminal_size() -> (u8, u8) {
        // A child process executed with the `output` method does not inherit
        // the parent process' stdin by default. Therefore, we must ensure that
        // stdin is inherited from the parent process in order for tput to query
        // the correct terminal for its size.
        let Ok(tput_out) = Command::new("tput")
            .args(["cols", "lines"])
            .stdin(Stdio::inherit())
            .output() else { return (20, 80) };

        let mut size_iter = tput_out.stdout.split(|byte| *byte == b'\n');

        // Parse the width and height from the bytes slice.
        let Some(width_bytes)  = size_iter.next() else { return (20, 80) };
        let Some(height_bytes) = size_iter.next() else { return (20, 80) };

        let Ok(width_str)  = std::str::from_utf8(width_bytes)  else { return (20, 80) };
        let Ok(height_str) = std::str::from_utf8(height_bytes) else { return (20, 80) };

        let Ok(width)  = u8::from_str(width_str)  else { return (20, 80) };
        let Ok(height) = u8::from_str(height_str) else { return (20, 80) };

        (height, width)
    }
}
