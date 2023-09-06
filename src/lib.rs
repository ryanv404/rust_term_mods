//! `term_mods`
//! [![Rust](https://github.com/ryanv404/rust_term_mods/actions/workflows/rust.yml/badge.svg)](https://github.com/ryanv404/rust_term_mods/actions/workflows/rust.yml)
//!
//! A simple library for formatting text output to the terminal.
//!
//! 
//! ## Features
//! Set foreground and background colors using:
//! - Named basic colors, with regular and bright variations.
//! - 256-color mode color numbers.
//! - 24-bit color mode RGB color values.
//! 
//! Format text as bold, faint, hidden, underlined, italicized, or strikethrough.
//! 
//! Control the cursor position, show/hide the cursor, clear the screen, clear a
//! line, or scroll the terminal screen.

#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![deny(clippy::cargo)]

pub mod attrs;
pub mod colors;
pub mod write;
pub mod term;

/// Primary structure for building a stylized string and printing it to stdout.
///
/// # Examples
///
/// ```
/// use term_mods::Style;
///
/// Style::this("hi there!").underline().green().println();
///
/// // Prints green, underlined "hi there!" to stdout with a newline.
/// ```
#[derive(Clone, Copy)]
pub struct Style<'a> {
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
    /// Italicized text
    Italic,
    /// Underlined text
    Underline,
    /// Invert text colors
    Invert,
    /// Strike out text
    Strike,
    /// Hide text
    Hide,
}

/// Terminal manipulation methods and attributes.
pub struct Term;

/// Control sequence introducer.
pub const CSI: &str = "\x1b[";
