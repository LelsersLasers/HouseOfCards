use macroquad::prelude as mq;

use crate::{colors, consts, mouse};

pub struct Player {
    pub pos: mq::Vec2,  // in tiles
    pub direction: f32, // in radians
}

impl Player {
    pub fn new() -> Self {
        Self {
            pos: mq::Vec2::ZERO,
            direction: 0.0,
        }
    }

    pub fn handle_input(&mut self, mouse_info: &mut mouse::MouseInfo, delta: f32) -> bool {
        // WASD keys to move (no arrow keys)
        // diagonal movement is allowed
        let mut movement = mq::Vec2::ZERO;
        if mq::is_key_down(mq::KeyCode::W) {
            movement.y -= 1.0;
        }
        if mq::is_key_down(mq::KeyCode::S) {
            movement.y += 1.0;
        }
        if mq::is_key_down(mq::KeyCode::A) {
            movement.x -= 1.0;
        }
        if mq::is_key_down(mq::KeyCode::D) {
            movement.x += 1.0;
        }
        movement = movement.normalize_or_zero();

        // arrow keys to aim
        // diagonal aiming is allowed
        let mut aim_vec = mq::Vec2::ZERO;
        if mq::is_key_down(mq::KeyCode::Up) {
            aim_vec.y -= 1.0;
        }
        if mq::is_key_down(mq::KeyCode::Down) {
            aim_vec.y += 1.0;
        }
        if mq::is_key_down(mq::KeyCode::Left) {
            aim_vec.x -= 1.0;
        }
        if mq::is_key_down(mq::KeyCode::Right) {
            aim_vec.x += 1.0;
        }
        aim_vec = aim_vec.normalize_or_zero();

        // update player position
        let speed = consts::PLAYER_SPEED * delta;
        self.pos += movement * speed;

        // update player direction
        if aim_vec != mq::Vec2::ZERO {
            self.direction = aim_vec.y.atan2(aim_vec.x);
            mouse_info.set_active(false);
        } else if let Some(mouse_pos) = mouse_info.mouse_pos() {
            let mouse_pos_relative_to_center =
                mouse_pos - mq::Vec2::new(mq::screen_width() / 2.0, mq::screen_height() / 2.0);
            self.direction = mouse_pos_relative_to_center
                .y
                .atan2(mouse_pos_relative_to_center.x);
        } else if movement != mq::Vec2::ZERO {
            self.direction = movement.y.atan2(movement.x);
        }

        movement != mq::Vec2::ZERO
    }

    pub fn draw(&self, scale: f32) {
        // player: circle
        // player direction: triangle

        let player_size = consts::PLAYER_SIZE * scale;
        let player_position = mq::Vec2::new(mq::screen_width() / 2.0, mq::screen_height() / 2.0);
        mq::draw_circle(
            player_position.x,
            player_position.y,
            player_size,
            colors::NORD4,
        );

        // equilateral triangle with side lengths = diameter of circle
        let triangle_side_length = player_size * 2.0;
        let triangle_height = triangle_side_length * 3.0_f32.sqrt() / 2.0;

        let top_point = mq::Vec2::new(
            player_position.x + triangle_height * self.direction.cos(),
            player_position.y + triangle_height * self.direction.sin(),
        );

        let side_point_1 = mq::Vec2::new(
            player_position.x
                + triangle_side_length * (self.direction + std::f32::consts::PI / 2.0).cos() / 2.0,
            player_position.y
                + triangle_side_length * (self.direction + std::f32::consts::PI / 2.0).sin() / 2.0,
        );
        let side_point_2 = mq::Vec2::new(
            player_position.x
                + triangle_side_length * (self.direction - std::f32::consts::PI / 2.0).cos() / 2.0,
            player_position.y
                + triangle_side_length * (self.direction - std::f32::consts::PI / 2.0).sin() / 2.0,
        );

        mq::draw_triangle(top_point, side_point_1, side_point_2, colors::NORD4);
    }
}
