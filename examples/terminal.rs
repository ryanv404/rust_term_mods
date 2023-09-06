use std::io::{self, Write};

use rust_term_mods::{Term, Colorize};

fn main() -> io::Result<()> {
    let (height, width) = Term::get_terminal_size();

    let mut stdout = io::stdout().lock();

    make_test_block(height - 1, width, &mut stdout)?;

    let Ok(msg_len) = u8::try_from("| THIS IS A CENTERED TEST MESSAGE |".len()) else {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "Unable to parse a u8 integer from message length."
        ));
    };

    let msg    = format!(
        "| {} |",
        Colorize::this("THIS IS A CENTERED TEST MESSAGE").br_red().get_ansi()
    );
    let border = "|                                 |";

    Term::write_centered((height / 2) - 1, width, border, &mut stdout)?;
    Term::write(height / 2, (width / 2) - (msg_len / 2), &msg, &mut stdout)?;
    Term::write_centered((height / 2) + 1, width, border, &mut stdout)?;

    Term::cursor_bottomleft(height, &mut stdout)?;

    stdout.flush()?;
    println!();
    Ok(())
}

fn make_test_block<W: Write>(rows: u8, cols: u8, w: &mut W) -> io::Result<()> {
    let mut grid = String::new();

    // Fill the screen with 'X's
    for _ in 1..=rows {
        (0..=cols - 1).for_each(|_| grid.push('X'));
        grid.push('\n');
    }

    Term::clr_scr(w)?;
    Term::write(1, 1, &grid, w)?;

    // Line numbering
    for row in 1..=rows {
        Term::write(row, 1, format!("{row:<3}").as_str(), w)?;
    }

    w.flush()?;
    Ok(())
}
