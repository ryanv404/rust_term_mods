pub use crate::{Attr, Bg, Colorize, Fg};

/// Control sequence introducer
pub const CSI: &str = "\x1b[";

/// Reset text color and attributes
pub const RESET: &str = "\x1b[0m";

impl<'a> Colorize<'a> {
    /// Constructs the text object that will be stylized.
    #[must_use]
    pub fn this(s: &'a str) -> Self {
        Self {
            has_attr: false,
            has_bg: false,
            has_fg: false,
            attr: None,
            bg: None,
            fg: None,
            text: s,
        }
    }

    /// Sets the text color to red.
    ///
    /// ```
    /// use rust_term_mods::Colorize;
    ///
    /// Colorize::this("Oh no!").red().println();
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
    /// use rust_term_mods::Colorize;
    ///
    /// Colorize::this("I'm RGB").fg_rgb(25, 123, 92).println();
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

    /// Makes the text bold.
    #[must_use]
    pub fn bold(&mut self) -> Self {
        self.attr = Some(Attr::Bold);
        *self
    }

    /// Makes the text faint.
    #[must_use]
    pub fn faint(&mut self) -> Self {
        self.attr = Some(Attr::Faint);
        *self
    }

    /// Makes the text italicized.
    #[must_use]
    pub fn italic(&mut self) -> Self {
        self.attr = Some(Attr::Italic);
        *self
    }

    /// Makes the text underlined.
    #[must_use]
    pub fn underline(&mut self) -> Self {
        self.attr = Some(Attr::Underline);
        *self
    }

    /// Inverts the text and background colors.
    #[must_use]
    pub fn invert(&mut self) -> Self {
        self.attr = Some(Attr::Invert);
        *self
    }

    /// Inverts the text and background colors.
    #[must_use]
    pub fn strike(&mut self) -> Self {
        self.attr = Some(Attr::Strike);
        *self
    }

    // Handles attribute component of the ANSI string.
    fn get_attr_code(&mut self, ansi_str: &mut String) {
        if let Some(attr) = self.attr {
            self.has_attr = true;
            ansi_str.push_str(&format!("{attr}"));
        }
    }

    // Handles foreground component of the ANSI string.
    fn get_fg_code(&mut self, ansi_str: &mut String) {
        if let Some(fg_color) = self.fg {
            self.has_fg = true;
            if self.has_attr {
                ansi_str.push(';');
            }
            ansi_str.push_str(&format!("{fg_color}"));
        }
    }

    // Handles background component of the ANSI string.
    fn get_bg_code(&mut self, ansi_str: &mut String) {
        if let Some(bg_color) = self.bg {
            self.has_bg = true;
            if self.has_attr || self.has_fg {
                ansi_str.push(';');
            }
            ansi_str.push_str(&format!("{bg_color}"));
        }
    }

    /// Builds and returns the ANSI string represented by the Colorize object.
    ///
    /// ```
    /// use rust_term_mods::Colorize;
    ///
    /// let ansi_string = Colorize::this("I'm bright green!").br_green().get_ansi();
    ///
    /// assert_eq!(ansi_string, "\x1b[92mI'm bright green!\x1b[0m".to_string());
    /// ```
    pub fn get_ansi(&mut self) -> String {
        let mut ansi_str = String::from(CSI);

        self.get_attr_code(&mut ansi_str);
        self.get_fg_code(&mut ansi_str);
        self.get_bg_code(&mut ansi_str);

        if self.has_attr || self.has_fg || self.has_bg {
            format!("{ansi_str}m{}{RESET}", self.text)
        } else {
            self.text.to_string()
        }
    }
}
