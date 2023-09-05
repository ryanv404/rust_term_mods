use rust_term_mods::Colorize;

fn main() {
    println!("Text Attributes:");
    Colorize::this("Bold").bold().println();
    Colorize::this("Faint").faint().println();
    Colorize::this("Italicized").italic().println();
    Colorize::this("Underlined").underline().println();
    Colorize::this("Strikethrough").strike().println();
    Colorize::this("Inverted").invert().println();
}
