use crate::frame::{Drawable, Frame};
use crate::{NUM_COLS, NUM_ROWS};

pub struct Player {
    x: usize,
    y: usize,
}

impl Player {
    pub fn new() -> Self {
        Self {
            // Start in middle
            x: NUM_COLS / 2,
            // Put player at bottom of rendered screen
            y: NUM_ROWS - 1,
        }
    }
    pub fn move_left(&mut self) {
        if self.x > 0 {
            self.x -= 1;
        }
    }
    pub fn move_right(&mut self) {
        if self.x < NUM_COLS - 2 {
            self.x += 1;
        }
    }
}

impl Drawable for Player {
    fn draw(&self, frame: &mut Frame) {
        frame[self.x][self.y] = "A";
    }
}
