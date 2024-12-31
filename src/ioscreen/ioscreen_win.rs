use std::vec::Vec;
use crossterm::{ExecutableCommand, terminal, cursor};
use crossterm::event::{KeyCode, KeyEventKind, KeyEventState};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use crossterm::event;
use std::time::Duration;
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

pub fn backend_init() {
    let _ = enable_raw_mode();
}

pub fn backend_deinit() {
    let _ = disable_raw_mode();
}

pub fn clear_event_queue() {
    while let Ok(true) = event::poll(Duration::from_micros(100)) {
        if let Ok(_) = crossterm::event::read() {
            continue;
        } else {
            break;
        }
    }
}

pub fn backend_getch()->Option<char> {
    // Gets a single character from the user
    // and returns it as a char
    let mut result = None;
    while let Ok(true) = event::poll(Duration::from_millis(20)) {
        if let Ok(evt) = crossterm::event::read() {
            if let crossterm::event::Event::Key(kevent) = evt {
                if kevent.kind == KeyEventKind::Press {
                    if let KeyCode::Char(ch) = kevent.code {
                        result = Some(ch);
                        break;
                    } else {
                        continue;
                    }
                } else {
                    continue;
                }
            } else {
                continue;
            }
        } else {
            continue;
        }
    }
    clear_event_queue();
    result
}