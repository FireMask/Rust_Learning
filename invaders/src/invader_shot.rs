use std::time::Duration;
use rusty_time::Timer;

use crate::{frame::{Drawable, Frame}, NUM_ROWS};

pub struct InvadersShots {
    pub x: usize,
    pub y: usize,
    pub exploding: bool,
    move_timer: Timer,
}

impl InvadersShots {
    pub fn new(x: usize, y: usize) -> Self {
        Self {
            x,
            y,
            exploding: false,
            move_timer: Timer::new(Duration::from_millis(200)),
        }
    }
    pub fn update(&mut self, delta: Duration) {        
        self.move_timer.tick(delta);
        if self.move_timer.finished() && !self.exploding {
            if self.y <= NUM_ROWS - 2 {
                self.y += 1;
            }
            self.move_timer.reset();
        }
    }
    pub fn dead(&self) -> bool {
        (self.exploding && self.move_timer.finished()) || (self.y >= NUM_ROWS - 1)
    }
    pub fn explode(&mut self) {
        self.exploding = true;
        self.move_timer = Timer::new(Duration::from_millis(250));
    }
}

impl Drawable for InvadersShots {
    fn draw(&self, frame: &mut Frame) {
        frame[self.x][self.y] = if self.exploding { 'X' } else { 'O' }
    }
}