use std::io::Stdout;

use crate::frame::Frame;
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

    for (x, col) in current_frame.iter().enumerated() {}
}
