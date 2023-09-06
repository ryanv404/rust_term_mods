use std::{
    io::{self, Write},
    process::{Command, Stdio},
    str::FromStr,
};

use crate::{CSI, Term};

#[allow(clippy::missing_errors_doc)]
impl Term {
    /// Scrolls the terminal screen up `num` lines.
    pub fn scroll_u<W: Write>(num: u8, w: &mut W) -> io::Result<()> {
        w.write_all(format!("{CSI}{num}S").as_bytes())
    }

    /// Scrolls the terminal screen down `num` lines.
    pub fn scroll_d<W: Write>(num: u8, w: &mut W) -> io::Result<()> {
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
    pub fn clr_ln<W: Write>(w: &mut W) -> io::Result<()> {
        w.write_all(format!("{CSI}2K").as_bytes())
    }

    /// Clears the current line from the cursor to the beginning of the line.
    pub fn clr_ln_to_start<W: Write>(w: &mut W) -> io::Result<()> {
        w.write_all(format!("{CSI}1K").as_bytes())
    }

    /// Clears the current line from the cursor to the end of the line.
    pub fn clr_ln_to_end<W: Write>(w: &mut W) -> io::Result<()> {
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
    pub fn cursor_u<W: Write>(num: u8, w: &mut W) -> io::Result<()> {
        w.write_all(format!("{CSI}{num}A").as_bytes())
    }

    /// Moves the cursor `num` cells down.
    pub fn cursor_d<W: Write>(num: u8, w: &mut W) -> io::Result<()> {
        w.write_all(format!("{CSI}{num}B").as_bytes())
    }

    /// Moves the cursor `num` cells right.
    pub fn cursor_r<W: Write>(num: u8, w: &mut W) -> io::Result<()> {
        w.write_all(format!("{CSI}{num}C").as_bytes())
    }

    /// Moves the cursor `num` cells left.
    pub fn cursor_l<W: Write>(num: u8, w: &mut W) -> io::Result<()> {
        w.write_all(format!("{CSI}{num}D").as_bytes())
    }

    /// Moves the cursor to column `num`.
    pub fn cursor_col<W: Write>(num: u8, w: &mut W) -> io::Result<()> {
        w.write_all(format!("{CSI}{num}G").as_bytes())
    }

    /// Moves the cursor to row `row` and column `col`.
    pub fn cursor_goto<W: Write>(row: u8, col: u8, w: &mut W) -> io::Result<()> {
        w.write_all(format!("{CSI}{row};{col}H").as_bytes())
    }

    /// Moves the cursor to the bottom left position on the screen.
    pub fn cursor_bl<W: Write>(rows: u8, w: &mut W) -> io::Result<()> {
        w.write_all(format!("{CSI}{rows};1H").as_bytes())
    }

    /// Moves the cursor to the top left position on the screen.
    pub fn cursor_tl<W: Write>(w: &mut W) -> io::Result<()> {
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
    pub fn get_term_size() -> (u8, u8) {
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

//#[cfg(test)]
//mod tests {
//    use crate::Term;

    //macro_rules! test_modifier {
    //    ($label:ident: $modifier:expr => $ansi:expr) => {
    //        #[test]
    //        fn $label() {
    //            assert_eq!($modifier, $ansi.to_string());
    //        }
    //    };
    //}

    // Terminal scrolling tests
    //test_modifier!(scroll_up: Term::scroll_u(1, &mut w) => "\x1b[1S");
    //test_modifier!(scroll_down: Term::scroll_d(4, &mut w) => "\x1b[4T");

    // Clear screen tests
    //test_modifier!(clr_scr_all: Term::clr_scr(&mut w) => "\x1b[2J");
    //test_modifier!(clr_scr_to_end: Term::clr_scr_to_end(&mut w) => "\x1b[0J");
    //test_modifier!(clr_scr_to_start: Term::clr_scr_to_start(&mut w) => "\x1b[1J");

    // Clear line tests
    //test_modifier!(clr_line_all: Term::clr_ln(&mut w) => "\x1b[2K");
    //test_modifier!(clr_line_to_end: Term::clr_ln_to_end(&mut w) => "\x1b[0K");
    //test_modifier!(clr_line_to_start: Term::clr_ln_to_start(&mut w) => "\x1b[1K");

    // Cursor modifier tests
    //test_modifier!(cursor_show: Term::show_cursor(&mut w) => "\x1b[?25h");
    //test_modifier!(cursor_hide: Term::hide_cursor(&mut w) => "\x1b[?25l");
    //test_modifier!(cursor_up: Term::cursor_u(3, &mut w) => "\x1b[3A");
    //test_modifier!(cursor_down: Term::cursor_d(3, &mut w) => "\x1b[3B");
    //test_modifier!(cursor_forward: Term::cursor_r(3, &mut w) => "\x1b[3C");
    //test_modifier!(cursor_back: Term::cursor_l(3, &mut w) => "\x1b[3D");
    //test_modifier!(cursor_col: Term::cursor_col(3, &mut w) => "\x1b[3G");
    //test_modifier!(cursor_goto: Term::cursor_goto(13, 12, &mut w) => "\x1b[13;12H");
    //test_modifier!(cursor_tl: Term::cursor_tl(&mut w) => "\x1b[1;1H");
//}
