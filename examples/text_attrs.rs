use term_mods::Style;

fn main() -> std::io::Result<()> {
    println!("Text Attributes:");

    // Prints examples of the available text attributes
    Style::this("Bold").bold().println()?;
    Style::this("Faint").faint().println()?;
    Style::this("Italicized").italic().println()?;
    Style::this("Underlined").underline().println()?;
    println!("{} <- Hidden", Style::this("Hidden").hide().get_ansi());
    Style::this("Strikethrough").strike().println()?;
    Style::this("Inverted").invert().println()?;

    println!();
    Ok(())
}
