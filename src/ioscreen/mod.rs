use std::vec::Vec;
#[cfg(target_os = "windows")]
mod ioscreen_win;
#[cfg(target_os = "windows")]
use ioscreen_win::*;

pub struct Canvas {
    display_data: Vec<Vec<char>>
}

pub fn clear_screen() {
    clr_scr();
}

pub fn getch() -> Option<char> {
    backend_getch()
}

pub fn init() {
    backend_init()
}

pub fn deinit() {
    backend_deinit()
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        let display_data: Vec<Vec<char>> = (0..height).map(|_| {
            (0..width).map(|_| {' '}).collect()
        }).collect();
        Canvas {display_data}
    }

    pub fn set_char(&mut self, x: usize, y: usize, ch: char) {
        self.display_data[y][x] = ch;
    }

    pub fn display(&self) {
        display(&self.display_data);
    }

    pub fn clear(&mut self) {
        for row in &mut self.display_data {
            for col in 0..row.len() {
                row[col] = ' ';
            }
        }
    }
}
