use crate::{Bg, CSI, Fg, Style};

impl<'a> Style<'a> {
    /// Constructs a default Style object.
    #[must_use]
    pub fn new() -> Self {
        Self {
            attr: None,
            bg: None,
            fg: None,
            text: "",
        }
    }

    /// Constructs the text object that will be stylized.
    #[must_use]
    pub fn this(s: &'a str) -> Self {
        Self {
            text: s,
            ..Self::new()
        }
    }

    /// Sets the text color to red.
    ///
    /// ```
    /// use term_mods::Style;
    ///
    /// Style::this("Oh no!").red().println();
    /// // Prints a red "Oh no!" to stdout with a newline.
    /// ```
    #[must_use]
    pub fn red(&mut self) -> Self {
        self.fg = Some(Fg::Red);
        *self
    }

    /// Sets the text color to green.
    #[must_use]
    pub fn green(&mut self) -> Self {
        self.fg = Some(Fg::Green);
        *self
    }

    /// Sets the text color to yellow.
    #[must_use]
    pub fn yellow(&mut self) -> Self {
        self.fg = Some(Fg::Yellow);
        *self
    }

    /// Sets the text color to blue.
    #[must_use]
    pub fn blue(&mut self) -> Self {
        self.fg = Some(Fg::Blue);
        *self
    }

    /// Sets the text color to magenta.
    #[must_use]
    pub fn magenta(&mut self) -> Self {
        self.fg = Some(Fg::Magenta);
        *self
    }

    /// Sets the text color to cyan.
    #[must_use]
    pub fn cyan(&mut self) -> Self {
        self.fg = Some(Fg::Cyan);
        *self
    }

    /// Sets the text color to black.
    #[must_use]
    pub fn black(&mut self) -> Self {
        self.fg = Some(Fg::Black);
        *self
    }

    /// Sets the text color to white.
    #[must_use]
    pub fn white(&mut self) -> Self {
        self.fg = Some(Fg::White);
        *self
    }

    /// Sets the text color to bright red.
    #[must_use]
    pub fn br_red(&mut self) -> Self {
        self.fg = Some(Fg::BrightRed);
        *self
    }

    /// Sets the text color to bright green.
    #[must_use]
    pub fn br_green(&mut self) -> Self {
        self.fg = Some(Fg::BrightGreen);
        *self
    }

    /// Sets the text color to bright yellow.
    #[must_use]
    pub fn br_yellow(&mut self) -> Self {
        self.fg = Some(Fg::BrightYellow);
        *self
    }

    /// Sets the text color to bright blue.
    #[must_use]
    pub fn br_blue(&mut self) -> Self {
        self.fg = Some(Fg::BrightBlue);
        *self
    }

    /// Sets the text color to bright magenta.
    #[must_use]
    pub fn br_magenta(&mut self) -> Self {
        self.fg = Some(Fg::BrightMagenta);
        *self
    }

    /// Sets the text color to bright cyan.
    #[must_use]
    pub fn br_cyan(&mut self) -> Self {
        self.fg = Some(Fg::BrightCyan);
        *self
    }

    /// Sets the text color to bright black.
    #[must_use]
    pub fn br_black(&mut self) -> Self {
        self.fg = Some(Fg::BrightBlack);
        *self
    }

    /// Sets the text color to bright white.
    #[must_use]
    pub fn br_white(&mut self) -> Self {
        self.fg = Some(Fg::BrightWhite);
        *self
    }

    /// Sets the text color using 256-color mode.
    #[must_use]
    pub fn fg_256(&mut self, c: u8) -> Self {
        self.fg = Some(Fg::Color256(c));
        *self
    }

    /// Sets the text color using an RGB value.
    ///
    /// ```
    /// use term_mods::Style;
    ///
    /// Style::this("I'm RGB").fg_rgb(25, 123, 92).println();
    /// // Colors the text "I'm RGB" with the color represented by RGB 25, 123, 92 and
    /// // prints it to stdout with a newline.
    /// ```
    #[must_use]
    pub fn fg_rgb(&mut self, r: u8, g: u8, b: u8) -> Self {
        self.fg = Some(Fg::Rgb(r, g, b));
        *self
    }

    /// Sets the background to red.
    #[must_use]
    pub fn bg_red(&mut self) -> Self {
        self.bg = Some(Bg::Red);
        *self
    }

    /// Sets the background to green.
    #[must_use]
    pub fn bg_green(&mut self) -> Self {
        self.bg = Some(Bg::Green);
        *self
    }

    /// Sets the background to yellow.
    #[must_use]
    pub fn bg_yellow(&mut self) -> Self {
        self.bg = Some(Bg::Yellow);
        *self
    }

    /// Sets the background to blue.
    #[must_use]
    pub fn bg_blue(&mut self) -> Self {
        self.bg = Some(Bg::Blue);
        *self
    }

    /// Sets the background to magenta.
    #[must_use]
    pub fn bg_magenta(&mut self) -> Self {
        self.bg = Some(Bg::Magenta);
        *self
    }

    /// Sets the background to cyan.
    #[must_use]
    pub fn bg_cyan(&mut self) -> Self {
        self.bg = Some(Bg::Cyan);
        *self
    }

    /// Sets the background to black.
    #[must_use]
    pub fn bg_black(&mut self) -> Self {
        self.bg = Some(Bg::Black);
        *self
    }

    /// Sets the background to white.
    #[must_use]
    pub fn bg_white(&mut self) -> Self {
        self.bg = Some(Bg::White);
        *self
    }

    /// Sets the background to bright red.
    #[must_use]
    pub fn bg_br_red(&mut self) -> Self {
        self.bg = Some(Bg::BrightRed);
        *self
    }

    /// Sets the background to bright green.
    #[must_use]
    pub fn bg_br_green(&mut self) -> Self {
        self.bg = Some(Bg::BrightGreen);
        *self
    }

    /// Sets the background to bright yellow.
    #[must_use]
    pub fn bg_br_yellow(&mut self) -> Self {
        self.bg = Some(Bg::BrightYellow);
        *self
    }

    /// Sets the background to bright blue.
    #[must_use]
    pub fn bg_br_blue(&mut self) -> Self {
        self.bg = Some(Bg::BrightBlue);
        *self
    }

    /// Sets the background to bright magenta.
    #[must_use]
    pub fn bg_br_magenta(&mut self) -> Self {
        self.bg = Some(Bg::BrightMagenta);
        *self
    }

    /// Sets the background to bright cyan.
    #[must_use]
    pub fn bg_br_cyan(&mut self) -> Self {
        self.bg = Some(Bg::BrightCyan);
        *self
    }

    /// Sets the background to bright black.
    #[must_use]
    pub fn bg_br_black(&mut self) -> Self {
        self.bg = Some(Bg::BrightBlack);
        *self
    }

    /// Sets the background to bright white.
    #[must_use]
    pub fn bg_br_white(&mut self) -> Self {
        self.bg = Some(Bg::BrightWhite);
        *self
    }

    /// Sets the background color using 256-color mode.
    #[must_use]
    pub fn bg_256(&mut self, c: u8) -> Self {
        self.bg = Some(Bg::Color256(c));
        *self
    }

    /// Sets the background color using an RGB value.
    #[must_use]
    pub fn bg_rgb(&mut self, r: u8, g: u8, b: u8) -> Self {
        self.bg = Some(Bg::Rgb(r, g, b));
        *self
    }

    // Handles foreground component of the ANSI string.
    fn get_fg_code(&mut self, ansi_str: &mut String) -> bool {
        match (self.attr, self.fg) {
            (_, None) => false,
            (Some(_), Some(fg_color)) => {
                ansi_str.push_str(&format!(";{fg_color}"));
                true
            },
            (None, Some(fg_color)) => {
                ansi_str.push_str(&format!("{fg_color}"));
                true
            }
        }
    }

    // Handles background component of the ANSI string.
    fn get_bg_code(&mut self, ansi_str: &mut String) -> bool {
        match (self.attr, self.fg, self.bg) {
            (_, _, None) => false,
            (Some(_), _, Some(bg_color)) | (_, Some(_), Some(bg_color)) => {
                ansi_str.push_str(&format!(";{bg_color}"));
                true
            },
            (None, None, Some(bg_color)) => {
                ansi_str.push_str(&format!("{bg_color}"));
                true
            }
        }
    }

    /// Builds and returns the ANSI string represented by the Style object.
    ///
    /// ```
    /// use term_mods::Style;
    ///
    /// let ansi_string = Style::this("I'm bright green!").br_green().get_ansi();
    ///
    /// assert_eq!(ansi_string, "\x1b[92mI'm bright green!\x1b[0m".to_string());
    /// ```
    pub fn get_ansi(&mut self) -> String {
        let mut ansi_str = CSI.to_string();

        let has_attr       = self.get_attr_code(&mut ansi_str);
        let has_foreground = self.get_fg_code(&mut ansi_str);
        let has_background = self.get_bg_code(&mut ansi_str);

        if has_attr || has_foreground || has_background {
            format!("{ansi_str}m{}{CSI}0m", self.text)
        } else {
            self.text.to_string()
        }
    }
}

impl<'a> Default for Style<'a> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::Style;

    macro_rules! test_style {
        ($label:ident: $style:expr => $ansi:literal) => {
            #[test]
            fn $label() {
                assert_eq!($style.get_ansi(), $ansi.to_string());
            }
        };
    }

    // Foreground color tests
    test_style!(black: Style::this("X").black() => "\x1b[30mX\x1b[0m");
    test_style!(red: Style::this("X").red() => "\x1b[31mX\x1b[0m");
    test_style!(green: Style::this("X").green() => "\x1b[32mX\x1b[0m");
    test_style!(yellow: Style::this("X").yellow() => "\x1b[33mX\x1b[0m");
    test_style!(blue: Style::this("X").blue() => "\x1b[34mX\x1b[0m");
    test_style!(magenta: Style::this("X").magenta() => "\x1b[35mX\x1b[0m");
    test_style!(cyan: Style::this("X").cyan() => "\x1b[36mX\x1b[0m");
    test_style!(white: Style::this("X").white() => "\x1b[37mX\x1b[0m");
    test_style!(br_black: Style::this("X").br_black() => "\x1b[90mX\x1b[0m");
    test_style!(br_red: Style::this("X").br_red() => "\x1b[91mX\x1b[0m");
    test_style!(br_green: Style::this("X").br_green() => "\x1b[92mX\x1b[0m");
    test_style!(br_yellow: Style::this("X").br_yellow() => "\x1b[93mX\x1b[0m");
    test_style!(br_blue: Style::this("X").br_blue() => "\x1b[94mX\x1b[0m");
    test_style!(br_magenta: Style::this("X").br_magenta() => "\x1b[95mX\x1b[0m");
    test_style!(br_cyan: Style::this("X").br_cyan() => "\x1b[96mX\x1b[0m");
    test_style!(br_white: Style::this("X").br_white() => "\x1b[97mX\x1b[0m");

    // Background color tests
    test_style!(bg_black: Style::this("X").bg_black() => "\x1b[40mX\x1b[0m");
    test_style!(bg_red: Style::this("X").bg_red() => "\x1b[41mX\x1b[0m");
    test_style!(bg_green: Style::this("X").bg_green() => "\x1b[42mX\x1b[0m");
    test_style!(bg_yellow: Style::this("X").bg_yellow() => "\x1b[43mX\x1b[0m");
    test_style!(bg_blue: Style::this("X").bg_blue() => "\x1b[44mX\x1b[0m");
    test_style!(bg_magenta: Style::this("X").bg_magenta() => "\x1b[45mX\x1b[0m");
    test_style!(bg_cyan: Style::this("X").bg_cyan() => "\x1b[46mX\x1b[0m");
    test_style!(bg_white: Style::this("X").bg_white() => "\x1b[47mX\x1b[0m");
    test_style!(bg_br_black: Style::this("X").bg_br_black() => "\x1b[100mX\x1b[0m");
    test_style!(bg_br_red: Style::this("X").bg_br_red() => "\x1b[101mX\x1b[0m");
    test_style!(bg_br_green: Style::this("X").bg_br_green() => "\x1b[102mX\x1b[0m");
    test_style!(bg_br_yellow: Style::this("X").bg_br_yellow() => "\x1b[103mX\x1b[0m");
    test_style!(bg_br_blue: Style::this("X").bg_br_blue() => "\x1b[104mX\x1b[0m");
    test_style!(bg_br_magenta: Style::this("X").bg_br_magenta() => "\x1b[105mX\x1b[0m");
    test_style!(bg_br_cyan: Style::this("X").bg_br_cyan() => "\x1b[106mX\x1b[0m");

    // Style combination tests
    test_style!(
        bold_green_bg_yellow:
        Style::this("X").bold().green().bg_yellow() => "\x1b[1;32;43mX\x1b[0m"
    );
    test_style!(
        bg_red_underline_cyan:
        Style::this("X").bg_red().underline().cyan() => "\x1b[4;36;41mX\x1b[0m"
    );
    test_style!(
        br_black_bg_white_italic:
        Style::this("X").br_black().bg_white().italic() => "\x1b[3;90;47mX\x1b[0m"
    );

    // 256-Color mode tests
    test_style!(fg_256: Style::this("X").fg_256(123) => "\x1b[38;5;123mX\x1b[0m");
    test_style!(bg_256: Style::this("X").bg_256(243) => "\x1b[48;5;243mX\x1b[0m");
    test_style!(
        fg_256_bg_256:
        Style::this("X").fg_256(123).bg_256(243) => "\x1b[38;5;123;48;5;243mX\x1b[0m"
    );

    // RGB color tests
    test_style!(
        rgb_fg:
        Style::this("X").fg_rgb(123, 87, 92) => "\x1b[38;2;123;87;92mX\x1b[0m"
    );
    test_style!(
        rgb_bg:
        Style::this("X").bg_rgb(99, 63, 243) => "\x1b[48;2;99;63;243mX\x1b[0m"
    );
    test_style!(
        rgb_fg_bg_strike:
        Style::this("X").bg_rgb(23, 24, 25).fg_rgb(123, 52, 212).strike() =>
        "\x1b[9;38;2;123;52;212;48;2;23;24;25mX\x1b[0m"
    );
}
