//! `rust_term_mods`
//!
//! A simple library for terminal manipulation and text styling using
//! ANSI control sequences.

#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![deny(clippy::cargo)]

pub mod colors;
pub mod display;
pub mod terminal;

#[cfg(test)]
mod tests;

/// Primary structure for building a stylized string and printing it to stdout.
///
/// # Examples
///
/// ```
/// use rust_term_mods::Colorize;
///
/// Colorize::this("hi there!").underline().green().println();
///
/// // Prints green, underlined "hi there!" to stdout with a newline.
/// ```
#[derive(Clone, Copy)]
pub struct Colorize<'a> {
    // Indicates whether a text attribute is present.
    pub has_attr: bool,

    // Indicates whether a background color is present.
    pub has_bg: bool,

    // Indicates whether a foreground color is present.
    pub has_fg: bool,

    // Text attributes
    pub attr: Option<Attr>,

    // Background color
    pub bg: Option<Bg>,

    // Text color
    pub fg: Option<Fg>,

    // Text to be stylized.
    pub text: &'a str,
}

/// Foreground colors
#[derive(Clone, Copy)]
pub enum Fg {
    /// Black
    Black,
    /// Red
    Red,
    /// Green
    Green,
    /// Yellow
    Yellow,
    /// Blue
    Blue,
    /// Magenta
    Magenta,
    /// Cyan
    Cyan,
    /// White
    White,
    /// Bright black
    BrightBlack,
    /// Bright red
    BrightRed,
    /// Bright green
    BrightGreen,
    /// Bright yellow
    BrightYellow,
    /// Bright blue
    BrightBlue,
    /// Bright magenta
    BrightMagenta,
    /// Bright cyan
    BrightCyan,
    /// Bright white
    BrightWhite,
    /// A 256-color mode color
    Color256(u8),
    /// A 24-bit RGB color
    Rgb(u8, u8, u8),
}

/// Background colors
#[derive(Clone, Copy)]
pub enum Bg {
    /// Black
    Black,
    /// Red
    Red,
    /// Green
    Green,
    /// Yellow
    Yellow,
    /// Blue
    Blue,
    /// Magenta
    Magenta,
    /// Cyan
    Cyan,
    /// White
    White,
    /// Bright black
    BrightBlack,
    /// Bright red
    BrightRed,
    /// Bright green
    BrightGreen,
    /// Bright yellow
    BrightYellow,
    /// Bright blue
    BrightBlue,
    /// Bright magenta
    BrightMagenta,
    /// Bright cyan
    BrightCyan,
    /// Bright white
    BrightWhite,
    /// A 256-color mode color
    Color256(u8),
    /// A 24-bit RGB color
    Rgb(u8, u8, u8),
}

/// Text attributes
#[derive(Clone, Copy)]
pub enum Attr {
    /// Bold text
    Bold,
    /// Faint text
    Faint,
    /// Italicize text
    Italic,
    /// Underlined text
    Underline,
    /// Invert text colors
    Invert,
    /// Strike out text
    Strike,
}

/// Terminal manipulation methods and attributes.
pub struct Term;
