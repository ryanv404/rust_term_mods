use rust_term_mods::Colorize;

fn main() {
    println!("256-Color Mode Colors:");

    // Prints all 256 colors present in 256-color mode.
    let mut basic_block: Vec<String> = vec![];
    let mut color_block1: Vec<String> = vec![];
    let mut color_block2: Vec<String> = vec![];
    let mut gray_block: Vec<String> = vec![];

    (0..=255).for_each(|n| match n {
        0..=14 => basic_block.push(Colorize::this("   ").bg_256(n).get_ansi()),
        15 => basic_block.push(format!("{}\n", Colorize::this("   ").bg_256(n).get_ansi())),
        16..=33 | 52..=69 | 88..=105 | 124..=141 | 160..=177 | 196..=213 => match n {
            33 | 69 | 105 | 141 | 177 | 213 => {
                color_block1.push(format!("{}\n", Colorize::this("   ").bg_256(n).get_ansi()));
            }
            _ => color_block1.push(Colorize::this("   ").bg_256(n).get_ansi()),
        },
        34..=51 | 70..=87 | 106..=123 | 142..=159 | 178..=195 | 214..=231 => match n {
            51 | 87 | 123 | 159 | 195 | 231 => {
                color_block2.push(format!("{}\n", Colorize::this("   ").bg_256(n).get_ansi()));
            }
            _ => color_block2.push(Colorize::this("   ").bg_256(n).get_ansi()),
        },
        232..=255 => gray_block.push(Colorize::this("   ").bg_256(n).get_ansi()),
    });

    println!(
        "{}{}{}{}",
        basic_block.join(""),
        color_block1.join(""),
        color_block2.join(""),
        gray_block.join("")
    );

    println!();
}
