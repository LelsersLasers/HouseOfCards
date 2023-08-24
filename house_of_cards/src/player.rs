use macroquad::prelude as mq;

pub struct Player {
	pub pos: mq::Vec2
}

impl Player {
	pub fn new() -> Self {
		Self {
			pos: mq::vec2(0.0, 0.0)
		}
	}
}