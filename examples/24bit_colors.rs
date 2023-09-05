use std::{
    process::{Command, Stdio},
    str,
};

use rust_term_mods::Colorize;

fn main() {
    println!("24-bit RGB Colors:");

    let symbols = vec!["/", "\\"];

    let (width, height) = get_terminal_size();

    let total_cols = width - 1;
    let total_rows = height - 2;
    let total_cells = total_cols * total_rows;

    for cellnum in 0..total_cells {
        let red = match (255 - cellnum * 255 / total_cells).try_into() {
            Ok(r) => r,
            Err(e) => return eprintln!("Error while converting to a u8 integer. {e}"),
        };

        let green = cellnum * 510 / total_cells;
        let green = if green > 255 { 510 - green } else { green };
        let green = match green.try_into() {
            Ok(gr) => gr,
            Err(e) => return eprintln!("Error while converting to a u8 integer. {e}"),
        };

        let blue = match (cellnum * 255 / total_cells).try_into() {
            Ok(b) => b,
            Err(e) => return eprintln!("Error while converting to a u8 integer. {e}"),
        };

        let idx = cellnum as usize;
        let text = symbols[idx % 2];

        Colorize::this(text)
            .bg_rgb(red, green, blue)
            .fg_rgb(255 - red, 255 - green, 255 - blue)
            .print();

        if (cellnum + 1) % total_cols == 0 {
            println!();
        }
    }
}

fn get_terminal_size() -> (u32, u32) {
    let Ok(mut tput_child) = Command::new("tput")
        .args(["cols", "lines"])
        .stdout(Stdio::piped())
        .spawn() else { return (75, 20) };

    let Some(cat_in) = tput_child.stdout.take() else { return (75, 20) };

    let Ok(cat_out) = Command::new("cat")
        .arg("-")
        .stdin(cat_in)
        .output() else { return (75, 20) };

    let Ok(size_str) = str::from_utf8(&cat_out.stdout) else { return (75, 20) };

    let size_vec: Vec<&str> = size_str.trim().split("\n").collect();

    if size_vec.len() != 2 {
        return (75, 20);
    }

    let (Ok(width), Ok(height)) = (
        size_vec[0].parse::<u32>(), size_vec[1].parse::<u32>()
    ) else {
        return (75, 20);
    };

    (width, height)
}
