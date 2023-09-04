pub use crate::{colors::CSI, ClearLine, ClearScreen, Cursor, Scroll};

impl Scroll {
    /// Scroll the terminal screen up `num` lines.
    #[must_use]
    pub fn up(num: u8) -> String {
        format!("{CSI}{num}S")
    }

    /// Scroll the terminal screen down `num` lines.
    #[must_use]
    pub fn down(num: u8) -> String {
        format!("{CSI}{num}T")
    }
}

impl ClearScreen {
    /// Clear the full terminal screen.
    #[must_use]
    pub fn all() -> String {
        format!("{CSI}2J")
    }

    /// Clear the terminal screen from the cursor to the end of the screen.
    #[must_use]
    pub fn to_end() -> String {
        format!("{CSI}0J")
    }

    /// Clear the terminal screen from the cursor to the beginning of the screen.
    #[must_use]
    pub fn to_start() -> String {
        format!("{CSI}1J")
    }
}

impl ClearLine {
    /// Clear the current line.
    #[must_use]
    pub fn all() -> String {
        format!("{CSI}2K")
    }

    /// Clear the current line from the cursor to the end of the line.
    #[must_use]
    pub fn to_end() -> String {
        format!("{CSI}0K")
    }

    /// Clear the current line from the cursor to the beginning of the line.
    #[must_use]
    pub fn to_start() -> String {
        format!("{CSI}1K")
    }
}

impl Cursor {
    /// Show the terminal cursor.
    #[must_use]
    pub fn show() -> String {
        format!("{CSI}?25h")
    }

    /// Hide the terminal cursor.
    #[must_use]
    pub fn hide() -> String {
        format!("{CSI}?25l")
    }

    /// Move the cursor `num` cells up.
    #[must_use]
    pub fn up(num: u8) -> String {
        format!("{CSI}{num}A")
    }

    /// Move the cursor `num` cells down.
    #[must_use]
    pub fn down(num: u8) -> String {
        format!("{CSI}{num}B")
    }

    /// Move the cursor `num` cells forward.
    #[must_use]
    pub fn forward(num: u8) -> String {
        format!("{CSI}{num}C")
    }

    /// Move the cursor `num` cells backward.
    #[must_use]
    pub fn back(num: u8) -> String {
        format!("{CSI}{num}D")
    }

    /// Move the cursor to column `num`.
    #[must_use]
    pub fn column(num: u8) -> String {
        format!("{CSI}{num}G")
    }

    /// Move the cursor to row `row` and column `col`.
    #[must_use]
    pub fn goto(row: u8, col: u8) -> String {
        format!("{CSI}{row};{col}H")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_modifier {
        ($label:ident: $modifier:expr => $ansi:expr) => {
            #[test]
            fn $label() {
                assert_eq!($modifier, $ansi.to_string());
            }
        };
    }

    // Terminal scrolling tests
    test_modifier!(scroll_up: Scroll::up(1) => "\x1b[1S");
    test_modifier!(scroll_down: Scroll::down(4) => "\x1b[4T");

    // Clear screen tests
    test_modifier!(clr_scr_all: ClearScreen::all() => "\x1b[2J");
    test_modifier!(clr_scr_to_end: ClearScreen::to_end() => "\x1b[0J");
    test_modifier!(clr_scr_to_start: ClearScreen::to_start() => "\x1b[1J");

    // Clear line tests
    test_modifier!(clr_line_all: ClearLine::all() => "\x1b[2K");
    test_modifier!(clr_line_to_end: ClearLine::to_end() => "\x1b[0K");
    test_modifier!(clr_line_to_start: ClearLine::to_start() => "\x1b[1K");

    // Cursor modifier tests
    test_modifier!(cursor_show: Cursor::show() => "\x1b[?25h");
    test_modifier!(cursor_hide: Cursor::hide() => "\x1b[?25l");
    test_modifier!(cursor_up: Cursor::up(3) => "\x1b[3A");
    test_modifier!(cursor_down: Cursor::down(3) => "\x1b[3B");
    test_modifier!(cursor_forward: Cursor::forward(3) => "\x1b[3C");
    test_modifier!(cursor_back: Cursor::back(3) => "\x1b[3D");
    test_modifier!(cursor_col: Cursor::column(3) => "\x1b[3G");
    test_modifier!(cursor_goto: Cursor::goto(13, 12) => "\x1b[13;12H");
}
