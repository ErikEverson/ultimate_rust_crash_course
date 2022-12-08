use rusty_time::timer::Timer;

pub struct Shot {
    pub x: usize,
    pub y: usize,
    pub exploding: bool,
    pub timer: Timer,
}

impl Shot {
    pub fn new(x: usize, y: usize) -> Self {
        Self {
            x,
            y,
            exploding: false,
            timer: Timer::from_millis(50),
        }
    }
}
