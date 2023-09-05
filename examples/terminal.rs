use std::io::{self, Write};

use rust_term_mods::terminal::{
    Cursor,
    write_centered_msg, write_msg, get_terminal_size,
};

fn main() {
    let (height, width) = get_terminal_size();
    let mut stdout = io::stdout().lock();

    make_test_block(height - 1, width, &mut stdout);

    write_centered_msg((height / 2) - 1, width, b"|                                 |", &mut stdout);
    write_centered_msg(height / 2, width, b"| THIS IS A CENTERED TEST MESSAGE |", &mut stdout);
    write_centered_msg((height / 2) + 1, width, b"|                                 |", &mut stdout);

    Cursor::goto_bl(height, &mut stdout);
    stdout.flush().unwrap();

    println!();
}

fn make_test_block(rows: u8, cols: u8, w: &mut io::StdoutLock) {
    let mut bytes: Vec<u8> = vec![];

    for _ in 1..=rows {
        (0..=cols - 1).for_each(|_| bytes.push(b'X'));
        bytes.push(b'\n');
    }

    w.write_all(&bytes).unwrap();

    (1..=rows).for_each(|r| write_msg(r, 1, &format!("{r:<3}").as_bytes(), w));
}
