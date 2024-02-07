use crate::{frame::{Drawable, Frame}, player::Player};

pub struct GUI {
    points: String,
}

impl GUI {
    pub fn new() -> Self {
        Self {
            points: "0".to_string(),
        }
    }
    pub fn update(&mut self, player: &mut Player){
        self.points = player.points.to_string();
    }
}

impl Drawable for GUI {
    fn draw(&self, frame: &mut Frame) {
        //Score points 
        for (index, c) in self.points.char_indices() {
            frame[1+index][1] = c;
        }
    }
}