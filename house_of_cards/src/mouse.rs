use macroquad::prelude as mq;

use crate::consts;

pub struct MouseInfo {
	last_pos: mq::Vec2,
	time_since_idle: f32,
	active: bool
}

impl MouseInfo {
	pub fn new() -> Self {
		Self {
			last_pos: mq::mouse_position().into(),
			time_since_idle: 0.0,
			active: false
		}
	}

	pub fn update(&mut self, delta: f32) {
		let current_pos = mq::mouse_position().into();
		if current_pos != self.last_pos {
			self.time_since_idle = 0.0;
			self.active = true;
		} else {
			self.time_since_idle += delta;
			if self.time_since_idle >= consts::TIME_TO_MOUSE_IDLE {
				self.active = false;
			}
		}
		self.last_pos = current_pos;
	}

	pub fn is_active(&self) -> bool {
		self.active
	}

	pub fn get_last_pos(&self) -> mq::Vec2 {
		self.last_pos
	}

	pub fn mouse_pos(&self) -> Option<mq::Vec2> {
		if self.active {
			Some(self.last_pos)
		} else {
			None
		}
	}

	pub fn set_active(&mut self, active: bool) {
		self.active = active;
		if active {
			self.time_since_idle = 0.0;
		}
	}
}