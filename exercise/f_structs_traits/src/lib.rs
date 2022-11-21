
pub trait Bite {
    fn bite(self: &mut Self);
}

#[derive(Debug)] // This enables using the debugging format string "{:?}"
pub struct Carrot {
    pub percent_left: f32,
}

impl Bite for Carrot {
    fn bite(self: &mut Self) {
        // Eat 20% of the remaining carrot. It may take awhile to eat it all...
        self.percent_left *= 0.8;
    }
}

#[derive(Debug)]
pub struct Grapes {
    pub grapes_left: i32,
}

impl Bite for Grapes {
    fn bite(self: &mut Self) {
        self.grapes_left -= 1;
    }
}

pub fn bunny_nibbles<T: Bite>(food: &mut T) {
    food.bite();
    food.bite();
    food.bite();
    food.bite();
}