use rust_term_mods::Colorize;

fn main() -> std::io::Result<()> {
    println!("Basic Colors:");

    // Prints all 16 basic background colors.
    Colorize::this("   ").bg_black().print()?;
    Colorize::this("   ").bg_red().print()?;
    Colorize::this("   ").bg_green().print()?;
    Colorize::this("   ").bg_yellow().print()?;
    Colorize::this("   ").bg_blue().print()?;
    Colorize::this("   ").bg_magenta().print()?;
    Colorize::this("   ").bg_cyan().print()?;
    Colorize::this("   ").bg_white().print()?;
    Colorize::this("   ").bg_br_black().print()?;
    Colorize::this("   ").bg_br_red().print()?;
    Colorize::this("   ").bg_br_green().print()?;
    Colorize::this("   ").bg_br_yellow().print()?;
    Colorize::this("   ").bg_br_blue().print()?;
    Colorize::this("   ").bg_br_magenta().print()?;
    Colorize::this("   ").bg_br_cyan().print()?;
    Colorize::this("   ").bg_br_white().println()?;

    println!();
    Ok(())
}
