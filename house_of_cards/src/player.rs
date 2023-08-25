use macroquad::prelude as mq;

use crate::{colors, consts};

pub struct Player {
    pub pos: mq::Vec2,  // in tiles
    pub direction: f32, // in degrees
}

impl Player {
    pub fn new() -> Self {
        Self {
            pos: mq::Vec2::ZERO,
            direction: 0.0,
        }
    }

    pub fn handle_movement(&mut self, delta: f32) {
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

        // update player position

        let speed = consts::PLAYER_SPEED * delta;
        self.pos += movement * speed;

        // update player direction
        if movement != mq::Vec2::ZERO {
            self.direction = movement.y.atan2(movement.x);
        }
    }

    pub fn draw(&self, scale: f32) {
        // player: circle
        // player direction: triangle

        let player_size = consts::PLAYER_SIZE * scale / consts::TILES_PER_SCALE as f32;
        let player_position = mq::Vec2::new(
            mq::screen_width() / 2.0,
            mq::screen_height() / 2.0,
        );
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
            player_position.x + triangle_side_length * (self.direction + std::f32::consts::PI / 2.0).cos() / 2.0,
            player_position.y + triangle_side_length * (self.direction + std::f32::consts::PI / 2.0).sin() / 2.0,
        );
        let side_point_2 = mq::Vec2::new(
            player_position.x + triangle_side_length * (self.direction - std::f32::consts::PI / 2.0).cos() / 2.0,
            player_position.y + triangle_side_length * (self.direction - std::f32::consts::PI / 2.0).sin() / 2.0,
        );

        mq::draw_triangle(top_point, side_point_1, side_point_2,
            colors::NORD4,
        );


    }
}
