use term_mods::Style;

fn main() -> std::io::Result<()> {
    println!("Basic Colors:");

    // Prints all 16 basic background colors.
    Style::this("   ").bg_black().print()?;
    Style::this("   ").bg_red().print()?;
    Style::this("   ").bg_green().print()?;
    Style::this("   ").bg_yellow().print()?;
    Style::this("   ").bg_blue().print()?;
    Style::this("   ").bg_magenta().print()?;
    Style::this("   ").bg_cyan().print()?;
    Style::this("   ").bg_white().print()?;
    Style::this("   ").bg_br_black().print()?;
    Style::this("   ").bg_br_red().print()?;
    Style::this("   ").bg_br_green().print()?;
    Style::this("   ").bg_br_yellow().print()?;
    Style::this("   ").bg_br_blue().print()?;
    Style::this("   ").bg_br_magenta().print()?;
    Style::this("   ").bg_br_cyan().print()?;
    Style::this("   ").bg_br_white().println()?;

    println!();
    Ok(())
}
