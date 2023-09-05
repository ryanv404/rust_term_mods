#![cfg(test)]

use crate::{Colorize, Cursor, ClearScreen, ClearLine, Scroll};

macro_rules! test_style {
    ($label:ident: $style:expr => $ansi:literal) => {
        #[test]
        fn $label() {
            assert_eq!($style.get_ansi(), $ansi.to_string());
        }
    };
}

macro_rules! test_modifier {
    ($label:ident: $modifier:expr => $ansi:expr) => {
        #[test]
        fn $label() {
            assert_eq!($modifier, $ansi.to_string());
        }
    };
}

// Foreground color tests
test_style!(black: Colorize::this("X").black() => "\x1b[30mX\x1b[0m");
test_style!(red: Colorize::this("X").red() => "\x1b[31mX\x1b[0m");
test_style!(green: Colorize::this("X").green() => "\x1b[32mX\x1b[0m");
test_style!(yellow: Colorize::this("X").yellow() => "\x1b[33mX\x1b[0m");
test_style!(blue: Colorize::this("X").blue() => "\x1b[34mX\x1b[0m");
test_style!(magenta: Colorize::this("X").magenta() => "\x1b[35mX\x1b[0m");
test_style!(cyan: Colorize::this("X").cyan() => "\x1b[36mX\x1b[0m");
test_style!(white: Colorize::this("X").white() => "\x1b[37mX\x1b[0m");
test_style!(br_black: Colorize::this("X").br_black() => "\x1b[90mX\x1b[0m");
test_style!(br_red: Colorize::this("X").br_red() => "\x1b[91mX\x1b[0m");
test_style!(br_green: Colorize::this("X").br_green() => "\x1b[92mX\x1b[0m");
test_style!(br_yellow: Colorize::this("X").br_yellow() => "\x1b[93mX\x1b[0m");
test_style!(br_blue: Colorize::this("X").br_blue() => "\x1b[94mX\x1b[0m");
test_style!(br_magenta: Colorize::this("X").br_magenta() => "\x1b[95mX\x1b[0m");
test_style!(br_cyan: Colorize::this("X").br_cyan() => "\x1b[96mX\x1b[0m");
test_style!(br_white: Colorize::this("X").br_white() => "\x1b[97mX\x1b[0m");

// Background color tests
test_style!(bg_black: Colorize::this("X").bg_black() => "\x1b[40mX\x1b[0m");
test_style!(bg_red: Colorize::this("X").bg_red() => "\x1b[41mX\x1b[0m");
test_style!(bg_green: Colorize::this("X").bg_green() => "\x1b[42mX\x1b[0m");
test_style!(bg_yellow: Colorize::this("X").bg_yellow() => "\x1b[43mX\x1b[0m");
test_style!(bg_blue: Colorize::this("X").bg_blue() => "\x1b[44mX\x1b[0m");
test_style!(bg_magenta: Colorize::this("X").bg_magenta() => "\x1b[45mX\x1b[0m");
test_style!(bg_cyan: Colorize::this("X").bg_cyan() => "\x1b[46mX\x1b[0m");
test_style!(bg_white: Colorize::this("X").bg_white() => "\x1b[47mX\x1b[0m");
test_style!(bg_br_black: Colorize::this("X").bg_br_black() => "\x1b[100mX\x1b[0m");
test_style!(bg_br_red: Colorize::this("X").bg_br_red() => "\x1b[101mX\x1b[0m");
test_style!(bg_br_green: Colorize::this("X").bg_br_green() => "\x1b[102mX\x1b[0m");
test_style!(bg_br_yellow: Colorize::this("X").bg_br_yellow() => "\x1b[103mX\x1b[0m");
test_style!(bg_br_blue: Colorize::this("X").bg_br_blue() => "\x1b[104mX\x1b[0m");
test_style!(bg_br_magenta: Colorize::this("X").bg_br_magenta() => "\x1b[105mX\x1b[0m");
test_style!(bg_br_cyan: Colorize::this("X").bg_br_cyan() => "\x1b[106mX\x1b[0m");
test_style!(bg_br_white: Colorize::this("X").bg_br_white() => "\x1b[107mX\x1b[0m");

// Style combination tests
test_style!(
    bold_green_bg_yellow:
    Colorize::this("X").bold().green().bg_yellow() => "\x1b[1;32;43mX\x1b[0m"
);
test_style!(
    bg_red_underline_cyan:
    Colorize::this("X").bg_red().underline().cyan() => "\x1b[4;36;41mX\x1b[0m"
);
test_style!(
    br_black_bg_white_italic:
    Colorize::this("X").br_black().bg_white().italic() => "\x1b[3;90;47mX\x1b[0m"
);

// 256-Color mode tests
test_style!(fg_256: Colorize::this("X").fg_256(123) => "\x1b[38;5;123mX\x1b[0m");
test_style!(bg_256: Colorize::this("X").bg_256(243) => "\x1b[48;5;243mX\x1b[0m");
test_style!(
    fg_256_bg_256:
    Colorize::this("X").fg_256(123).bg_256(243) => "\x1b[38;5;123;48;5;243mX\x1b[0m"
);

// RGB color tests
test_style!(
    rgb_fg:
    Colorize::this("X").fg_rgb(123, 87, 92) => "\x1b[38;2;123;87;92mX\x1b[0m"
);
test_style!(
    rgb_bg:
    Colorize::this("X").bg_rgb(99, 63, 243) => "\x1b[48;2;99;63;243mX\x1b[0m"
);
test_style!(
    rgb_fg_bg_strike:
    Colorize::this("X").bg_rgb(23, 24, 25).fg_rgb(123, 52, 212).strike() =>
    "\x1b[9;38;2;123;52;212;48;2;23;24;25mX\x1b[0m"
);

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
