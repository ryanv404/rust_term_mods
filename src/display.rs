pub use crate::colors::{Attr, Bg, Colorize, Fg, CSI};

// Describes the various printing methods.
enum WriteKind {
    Stdout,
    Stderr,
    StdoutNewline,
    StderrNewline,
}

impl<'a> Colorize<'a> {
    /// Prints the styled string to stdout.
    pub fn print(&mut self) -> std::io::Result<()> {
        self.write_common(WriteKind::Stdout)
    }

    /// Prints the styled string to stderr.
    pub fn eprint(&mut self) -> std::io::Result<()> {
        self.write_common(WriteKind::Stderr)
    }

    /// Prints the styled string to stdout with a newline.
    pub fn println(&mut self) -> std::io::Result<()> {
        self.write_common(WriteKind::StdoutNewline)
    }

    /// Prints the styled string to stderr with a newline.
    pub fn eprintln(&mut self) -> std::io::Result<()> {
        self.write_common(WriteKind::StderrNewline)
    }

    // Common logic for printing to stdout and stderr.
    fn write_common(&mut self, kind: WriteKind) -> std::io::Result<()> {
        use std::io::Write;

        let mut ansi_string = if self.text.is_empty() {
            self.text.to_string()
        } else {
            self.get_ansi()
        };

        match kind {
            WriteKind::Stdout => {
                std::io::stdout().write_all(ansi_string.as_bytes())?;
                std::io::stdout().flush()?;
            },
            WriteKind::Stderr => {
                std::io::stderr().write_all(ansi_string.as_bytes())?;
                std::io::stderr().flush()?;
            },
            WriteKind::StdoutNewline => {
                ansi_string.push('\n');
                std::io::stdout().write_all(ansi_string.as_bytes())?;
                std::io::stdout().flush()?;
            },
            WriteKind::StderrNewline => {
                ansi_string.push('\n');
                std::io::stderr().write_all(ansi_string.as_bytes())?;
                std::io::stderr().flush()?;
            },
        };

        Ok(())
    }
}

impl std::fmt::Display for Fg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Black => write!(f, "30"),
            Self::Red => write!(f, "31"),
            Self::Green => write!(f, "32"),
            Self::Yellow => write!(f, "33"),
            Self::Blue => write!(f, "34"),
            Self::Magenta => write!(f, "35"),
            Self::Cyan => write!(f, "36"),
            Self::White => write!(f, "37"),
            Self::BrightBlack => write!(f, "90"),
            Self::BrightRed => write!(f, "91"),
            Self::BrightGreen => write!(f, "92"),
            Self::BrightYellow => write!(f, "93"),
            Self::BrightBlue => write!(f, "94"),
            Self::BrightMagenta => write!(f, "95"),
            Self::BrightCyan => write!(f, "96"),
            Self::BrightWhite => write!(f, "97"),
            Self::Color256(c) => write!(f, "38;5;{c}"),
            Self::Rgb(r, g, b) => write!(f, "38;2;{r};{g};{b}"),
        }
    }
}

impl std::fmt::Display for Bg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Black => write!(f, "40"),
            Self::Red => write!(f, "41"),
            Self::Green => write!(f, "42"),
            Self::Yellow => write!(f, "43"),
            Self::Blue => write!(f, "44"),
            Self::Magenta => write!(f, "45"),
            Self::Cyan => write!(f, "46"),
            Self::White => write!(f, "47"),
            Self::BrightBlack => write!(f, "100"),
            Self::BrightRed => write!(f, "101"),
            Self::BrightGreen => write!(f, "102"),
            Self::BrightYellow => write!(f, "103"),
            Self::BrightBlue => write!(f, "104"),
            Self::BrightMagenta => write!(f, "105"),
            Self::BrightCyan => write!(f, "106"),
            Self::BrightWhite => write!(f, "107"),
            Self::Color256(c) => write!(f, "48;5;{c}"),
            Self::Rgb(r, g, b) => write!(f, "48;2;{r};{g};{b}"),
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
            Self::Strike => write!(f, "9"),
        }
    }
}
