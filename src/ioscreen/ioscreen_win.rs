use std::vec::Vec;
use crossterm::{ExecutableCommand, terminal, cursor};
use std::io::{stdout, Write};

pub fn clr_scr() {
    let mut stdout = stdout();
    stdout.execute(terminal::Clear(terminal::ClearType::All)).unwrap();
}

pub fn display(data: &Vec<Vec<char>>) {
    let mut stdout = stdout();
    stdout.execute(cursor::MoveTo(0, 0 as u16)).unwrap();

    let data_to_display = String::from_iter(data.iter().map(|row| {
        String::from_iter(row.iter()) + "\n"
    }));

    stdout.write(data_to_display.as_bytes());

    // Reset cursor position
    stdout.execute(cursor::MoveTo(0, 0)).unwrap();
}

pub fn getch() {
    
}