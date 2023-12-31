use term_mods::Style;

fn main() {
    println!("256-Color Mode Colors:");

    // Prints all 256 colors present in 256-color mode.
    let mut basic_block: Vec<String> = vec![];
    let mut color_block1: Vec<String> = vec![];
    let mut color_block2: Vec<String> = vec![];
    let mut gray_block: Vec<String> = vec![];

    (0..=255).for_each(|n| match n {
        0..=14 => {
            basic_block.push(Style::this("   ").bg_256(n).get_ansi());
        },
        15 => {
            basic_block.push(format!("{}\n", Style::this("   ").bg_256(n).get_ansi()));
        },
        33 | 69 | 105 | 141 | 177 | 213 => {
            color_block1.push(format!("{}\n", Style::this("   ").bg_256(n).get_ansi()));
        },
        16..=32 | 52..=68 | 88..=104 | 124..=140 | 160..=176 | 196..=212 => {
            color_block1.push(Style::this("   ").bg_256(n).get_ansi());
        },
        51 | 87 | 123 | 159 | 195 | 231 => {
            color_block2.push(format!("{}\n", Style::this("   ").bg_256(n).get_ansi()));
        },
        34..=50 | 70..=86 | 106..=122 | 142..=158 | 178..=194 | 214..=230 => {
            color_block2.push(Style::this("   ").bg_256(n).get_ansi());
        },
        232..=255 => {
            gray_block.push(Style::this("   ").bg_256(n).get_ansi());
        }
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
