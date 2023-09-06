use std::io::{self, Write};

use rust_term_mods::terminal::{
    get_terminal_size, write_centered_msg, write_msg, Cursor
};

fn main() {
    let mut stdout = io::stdout().lock();
    let (height, width) = get_terminal_size();

    make_test_block(height - 1, width, &mut stdout);

    let msg = "| THIS IS A CENTERED TEST MESSAGE |";
    let border = "|                                 |";

    let _ = write_centered_msg((height / 2) - 1, width, border, &mut stdout);
    let _ = write_centered_msg(height / 2, width, msg, &mut stdout);
    let _ = write_centered_msg((height / 2) + 1, width, border, &mut stdout);

    let _ = Cursor::goto_bl(height, &mut stdout);
    stdout.flush().unwrap();

    println!();
}

fn make_test_block(rows: u8, cols: u8, w: &mut io::StdoutLock) {
    let mut bytes: Vec<u8> = vec![];

    (1..=rows).for_each(|_| {
        (0..=cols - 1).for_each(|_| bytes.push(b'X'));
        bytes.push(b'\n');
    });

    w.write_all(&bytes).unwrap();

    (1..=rows).for_each(|r| {
        let _ = write_msg(r, 1, format!("{r:<3}").as_str(), w);
    });
}
