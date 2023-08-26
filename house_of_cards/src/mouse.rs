use macroquad::prelude as mq;

use crate::consts;

pub struct MouseInfo {
    last_pos: mq::Vec2,
    time_since_idle: f32,
    active: bool,
    show: bool,
}

impl MouseInfo {
    pub fn new() -> Self {
        Self {
            last_pos: mq::mouse_position().into(),
            time_since_idle: 0.0,
            active: false,
            show: false,
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

    pub fn draw(&self, scale: f32) {
        // crosshair
        if self.show || self.active {
            let crosshair_size = scale * consts::CROSSHAIR_SIZE;
            let crosshair_thickness = scale * consts::CROSSHAIR_THICKNESS;

            let crosshair_x = self.last_pos.x;
            let crosshair_y = self.last_pos.y;

            mq::draw_rectangle(
                crosshair_x - crosshair_size / 2.0,
                crosshair_y - crosshair_thickness / 2.0,
                crosshair_size,
                crosshair_thickness,
                consts::MOUSE_COLOR,
            );

            mq::draw_rectangle(
                crosshair_x - crosshair_thickness / 2.0,
                crosshair_y - crosshair_size / 2.0,
                crosshair_thickness,
                crosshair_size,
                consts::MOUSE_COLOR,
            );
        }
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
