use std::io::{self, Write};

use term_mods::{Term, Style};

fn main() -> io::Result<()> {
    let (height, width) = Term::get_term_size();

    let mut stdout = io::stdout().lock();

    make_text_block(height - 1, width, &mut stdout)?;

    let Ok(msg_len) = u8::try_from("| THIS IS A CENTERED TEXT MESSAGE |".len()) else {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "Unable to parse a u8 integer from message length."
        ));
    };

    let msg    = format!(
        "| {} |",
        Style::this("THIS IS A CENTERED TEXT MESSAGE").cyan().get_ansi()
    );
    let border = "|                                 |";

    Term::write_centered((height / 2) - 1, width, border, &mut stdout)?;
    Term::write(height / 2, (width / 2) - (msg_len / 2), &msg, &mut stdout)?;
    Term::write_centered((height / 2) + 1, width, border, &mut stdout)?;
    Term::cursor_bl(height, &mut stdout)?;

    stdout.flush()?;
    Ok(())
}

fn make_text_block<W: Write>(rows: u8, cols: u8, w: &mut W) -> io::Result<()> {
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
        let linenum = Style::this(&format!("{row:<3}")).green().get_ansi();
        Term::write(row, 1, linenum.as_str(), w)?;
    }

    w.flush()?;
    Ok(())
}
