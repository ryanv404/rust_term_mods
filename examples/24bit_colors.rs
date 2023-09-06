use std::io;

use term_mods::{Style, Term};

fn main() -> io::Result<()> {
    // Prints a full screen 24-bit spectral pattern.
    println!("24-bit RGB Colors:");

    let (height, width) = Term::get_term_size();

    let total_cols = u32::from(width) - 1;
    let total_rows = u32::from(height) - 2;
    let total_cells = total_cols * total_rows;

    let symbols = vec!["/", "\\"];

    for cellnum in 0..total_cells {
        // Red -> u8
        let red = 255 - cellnum * 255 / total_cells;
        let Ok(red) = u8::try_from(red) else {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Error while parsing to a u8 integer."
            ));
        };

        // Green -> u8
        let green = cellnum * 510 / total_cells;
        let green = if green > 255 { 510 - green } else { green };
        let Ok(green) = u8::try_from(green) else {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Error while parsing to a u8 integer."
            ));
        };

        // Blue -> u8
        let blue = cellnum * 255 / total_cells;
        let Ok(blue) = u8::try_from(blue) else {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Error while parsing to a u8 integer."
            ));
        };

        let idx = cellnum as usize;
        let text = symbols[idx % 2];

        // Print current cell to stdout
        Style::this(text)
            .bg_rgb(red, green, blue)
            .fg_rgb(255 - red, 255 - green, 255 - blue)
            .print()?;

        if (cellnum + 1) % total_cols == 0 {
            println!();
        }
    }

    println!();
    Ok(())
}
