use macroquad::prelude as mq;

use crate::{consts, player, util};

pub struct Camera {
    pub pos: mq::Vec2, // tiles
}

impl Camera {
    pub fn new() -> Self {
        Self {
            pos: mq::Vec2::new(0.0, 0.0),
        }
    }

    pub fn update(&mut self, player: &player::Player, delta: f32) -> util::Moved {
        // soft follow

        let old_pos = self.pos;

        let target = player.pos;
        let dif = target - self.pos;
        let movement = dif * delta * consts::CAMERA_FOLLOW_SPEED;

        self.pos += movement;

        util::Moved(old_pos != self.pos)
    }
}
