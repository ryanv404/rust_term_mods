use std::{
    process::{Command, Stdio},
    str::{self, FromStr},
};

use rust_term_mods::Colorize;

fn main() {
    println!("24-bit RGB Colors:");

    let (width, height) = get_terminal_size();

    let total_cols = width - 1;
    let total_rows = height - 2;
    let total_cells = total_cols * total_rows;

    let symbols = vec!["/", "\\"];

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

    println!();
}

fn get_terminal_size() -> (u32, u32) {
    // Must ensure that stdin is inherited from the parent for tput to
    // return the correct terminal size.
    let Ok(tput_out) = Command::new("tput")
        .args(["cols", "lines"])
        .stdin(Stdio::inherit())
        .output() else { return (75, 20) };

    let mut size_iter = tput_out.stdout.split(|byte| *byte == b'\n');

    let Some(width_bytes) = size_iter.next() else { return (75, 20) };
    let Some(height_bytes) = size_iter.next() else { return (75, 20) };

    let Ok(width_str) = str::from_utf8(width_bytes) else { return (75, 20) };
    let Ok(height_str) = str::from_utf8(height_bytes) else { return (75, 20) };

    let Ok(width) = u32::from_str(width_str) else { return (75, 20) };
    let Ok(height) = u32::from_str(height_str) else { return (75, 20) };

    (width, height)
}
