use crate::{Attr, Style};

impl<'a> Style<'a> {
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

    /// Hides the text
    #[must_use]
    pub fn hide(&mut self) -> Self {
        self.attr = Some(Attr::Hide);
        *self
    }

    // Handles attribute component of the ANSI string.
    pub fn get_attr_code(&mut self, ansi_str: &mut String) -> bool {
        if let Some(attr) = self.attr {
            ansi_str.push_str(&format!("{attr}"));
            true
        } else {
            false
        }
    }
}

impl std::fmt::Display for Attr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Bold => write!(f, "1"),
            Self::Faint => write!(f, "2"),
            Self::Italic => write!(f, "3"),
            Self::Underline => write!(f, "4"),
            Self::Invert => write!(f, "7"),
            Self::Hide => write!(f, "8"),
            Self::Strike => write!(f, "9"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Style;

    macro_rules! test_attr {
        ($label:ident: $style:expr => $ansi:literal) => {
            #[test]
            fn $label() {
                assert_eq!($style.get_ansi(), $ansi.to_string());
            }
        };
    }

    // Text attribute tests
    test_attr!(bold: Style::this("X").bold() => "\x1b[1mX\x1b[0m");
    test_attr!(faint: Style::this("X").faint() => "\x1b[2mX\x1b[0m");
    test_attr!(italic: Style::this("X").italic() => "\x1b[3mX\x1b[0m");
    test_attr!(underline: Style::this("X").underline() => "\x1b[4mX\x1b[0m");
    test_attr!(invert: Style::this("X").invert() => "\x1b[7mX\x1b[0m");
    test_attr!(hide: Style::this("X").hide() => "\x1b[8mX\x1b[0m");
    test_attr!(strike: Style::this("X").strike() => "\x1b[9mX\x1b[0m");
}
