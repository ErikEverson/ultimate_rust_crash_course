use std::io::{Stdout, Write};

use crate::frame::Frame;
use crossterm::cursor::MoveTo;
use crossterm::{
    style::{Color, SetBackgroundColor},
    terminal::{Clear, ClearType},
    QueueableCommand,
};

pub fn render(stdout: &mut Stdout, last_frame: &Frame, current_frame: &Frame, force: bool) {
    if force {
        stdout.queue(SetBackgroundColor(Color::Blue)).unwrap();
        stdout.queue(Clear(ClearType::All)).unwrap();
        stdout.queue(SetBackgroundColor(Color::Black)).unwrap();
    }

    for (x, col) in current_frame.iter().enumerate() {
        for (y, string_slice) in col.iter().enumerate() {
            if *string_slice != last_frame[x][y] || force {
                stdout.queue(MoveTo(x as u16, y as u16)).unwrap();
                print!("{}", *string_slice)
            }
        }
    }
    stdout.flush().unwrap();
}
