use macroquad::prelude as mq;

use crate::{camera, colors, consts, hitbox, joystick, mouse, powerup, util, weapon};

pub struct Player {
    pub pos: mq::Vec2,  // in tiles
    pub direction: f32, // in radians
    pub weapon: weapon::Weapon,
    pub health: f32,
    pub max_health: f32,
    pub xp: i32,
    pub level: i32,
    hp_bar_ratio: f32,
    pub xp_bar_ratio: f32,
}

impl Player {
    pub fn new(weapon: weapon::Weapon) -> Self {
        Self {
            pos: mq::Vec2::ZERO,
            direction: 0.0,
            weapon,
            health: consts::PLAYER_MAX_HEALTH,
            max_health: consts::PLAYER_MAX_HEALTH,
            xp: 0,
            level: 1,
            hp_bar_ratio: 1.0,
            xp_bar_ratio: 0.0,
        }
    }

    pub fn handle_input(
        &mut self,
        mouse_info: &mut mouse::MouseInfo,
        movement_joystick_result: joystick::JoystickUpdateResult,
        aim_joystick_result: joystick::JoystickUpdateResult,
        powerups: &powerup::Powerups,
        auto_shoot: bool,
        delta: f32,
    ) -> util::Shot {
        let movement = (if movement_joystick_result.active {
            movement_joystick_result.pos
        } else if mq::is_mouse_button_down(mq::MouseButton::Right) {
            let mouse_pos = mouse_info.get_last_pos();
            let mouse_pos_relative_to_center =
                mouse_pos - mq::Vec2::new(mq::screen_width() / 2.0, mq::screen_height() / 2.0);
            let angle = mouse_pos_relative_to_center
                .y
                .atan2(mouse_pos_relative_to_center.x);
            mq::Vec2::new(angle.cos(), angle.sin())
        } else {
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
            movement
        })
        .normalize_or_zero();

        let speed = consts::PLAYER_SPEED * delta * self.weapon.get_ms_penalty();
        self.pos += movement * speed * powerups.speed_mod();

        let aim = (if aim_joystick_result.active {
            aim_joystick_result.pos
        } else {
            let mut aim = mq::Vec2::ZERO;
            if mq::is_key_down(mq::KeyCode::Up) {
                aim.y -= 1.0;
            }
            if mq::is_key_down(mq::KeyCode::Down) {
                aim.y += 1.0;
            }
            if mq::is_key_down(mq::KeyCode::Left) {
                aim.x -= 1.0;
            }
            if mq::is_key_down(mq::KeyCode::Right) {
                aim.x += 1.0;
            }
            aim
        })
        .normalize_or_zero();

        if aim != mq::Vec2::ZERO {
            self.direction = aim.y.atan2(aim.x);
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
        
        self.update_bar_ratios(delta);

        self.weapon.update(delta);
        // uses short-circuiting to only `try_shoot` if the player is requesting to shoot
        // `.0` is used to get the `bool` from the `Shot` struct
        util::Shot(
            (mq::is_key_down(mq::KeyCode::Space)
                || mq::is_mouse_button_down(mq::MouseButton::Left)
                || aim_joystick_result.active
                || auto_shoot)
                && self.weapon.try_shoot().0,
        )
    }

    pub fn update_bar_ratios(&mut self, delta: f32) {
        {
            let old_ratio = self.hp_bar_ratio;
            let target_ratio = self.health / self.max_health;
            let dif = target_ratio - old_ratio;
            self.hp_bar_ratio += dif * delta * consts::BAR_UPDATE_SPEED;
        }
        {
            let old_ratio = self.xp_bar_ratio;
            let target_ratio = self.xp as f32 / consts::XP_PER_LEVEL(self.level) as f32;
            let dif = target_ratio - old_ratio;
            self.xp_bar_ratio += dif * delta * consts::BAR_UPDATE_SPEED;
        }
    }

    pub fn draw(&self, camera: &camera::Camera, scale: f32) {
        // player: circle
        // player direction: triangle

        let player_size = consts::PLAYER_SIZE * scale;

        let draw_pos = (self.pos - camera.pos) * scale / consts::TILES_PER_SCALE as f32
            + mq::Vec2::new(mq::screen_width() / 2.0, mq::screen_height() / 2.0);

        mq::draw_circle(draw_pos.x, draw_pos.y, player_size, colors::NORD4);

        // equilateral triangle with side lengths = diameter of circle
        let triangle_side_length = player_size * 2.0;
        let triangle_height = triangle_side_length * 3.0_f32.sqrt() / 2.0;

        let top_point = mq::Vec2::new(
            draw_pos.x + triangle_height * self.direction.cos(),
            draw_pos.y + triangle_height * self.direction.sin(),
        );

        let side_point_1 = mq::Vec2::new(
            draw_pos.x
                + triangle_side_length * (self.direction + std::f32::consts::PI / 2.0).cos() / 2.0,
            draw_pos.y
                + triangle_side_length * (self.direction + std::f32::consts::PI / 2.0).sin() / 2.0,
        );
        let side_point_2 = mq::Vec2::new(
            draw_pos.x
                + triangle_side_length * (self.direction - std::f32::consts::PI / 2.0).cos() / 2.0,
            draw_pos.y
                + triangle_side_length * (self.direction - std::f32::consts::PI / 2.0).sin() / 2.0,
        );

        mq::draw_triangle(top_point, side_point_1, side_point_2, colors::NORD4);
    }

    pub fn draw_bars(&self, font: mq::Font, scale: f32) {
        let bar_width = scale * consts::PLAYER_HP_BAR_WIDTH;
        let bar_height = scale * consts::PLAYER_HP_BAR_HEIGHT;
        let bar_thickness = scale * consts::PLAYER_HP_BAR_THICKNESS;

        // center horizontally
        let x = mq::screen_width() / 2.0 - bar_width / 2.0;
        let y = mq::screen_height() - bar_height / 2.0 - scale * consts::PLAYER_HP_BAR_BOT_OFFSET;

        let font_size = (bar_height * consts::PLAYER_BARS_FONT_RATIO).round() as u16;

        // XP bar
        mq::draw_rectangle(x, y, bar_width, bar_height, colors::NORD6_ALPHA);
        mq::draw_rectangle(x, y, bar_width * self.xp_bar_ratio, bar_height, colors::NORD8);
        mq::draw_rectangle_lines(x, y, bar_width, bar_height, bar_thickness, colors::NORD6);

        // XP text
        let text = format!("Level {}", self.level);
        let text_dims = mq::measure_text(&text, Some(font), font_size, 1.0);
        let text_pos = mq::Vec2::new(
            mq::screen_width() / 2.0 - text_dims.width / 2.0,
            y + bar_height / 2.0 + text_dims.offset_y / 2.25,
        );

        mq::draw_text_ex(
            &text,
            text_pos.x,
            text_pos.y,
            mq::TextParams {
                font,
                font_size,
                font_scale: 1.0,
                color: colors::NORD0,
                ..mq::TextParams::default()
            },
        );

        // HP bar
        let y = y - bar_height - scale * consts::PLAYER_XP_BAR_OFFSET;
        mq::draw_rectangle(x, y, bar_width, bar_height, colors::NORD6_ALPHA);
        mq::draw_rectangle(x, y, bar_width * self.hp_bar_ratio, bar_height, colors::NORD14);
        mq::draw_rectangle_lines(x, y, bar_width, bar_height, bar_thickness, colors::NORD6);

        // HP text
        let text = format!(
            "{:.0} / {:.0}",
            self.health.round(),
            self.max_health.round()
        );
        let text_dims = mq::measure_text(&text, Some(font), font_size, 1.0);
        let text_pos = mq::Vec2::new(
            mq::screen_width() / 2.0 - text_dims.width / 2.0,
            y + bar_height / 2.0 + text_dims.offset_y / 2.5,
        );

        mq::draw_text_ex(
            &text,
            text_pos.x,
            text_pos.y,
            mq::TextParams {
                font,
                font_size,
                font_scale: 1.0,
                color: colors::NORD0,
                ..mq::TextParams::default()
            },
        );
    }
}

impl hitbox::Circle for Player {
    fn center(&self) -> mq::Vec2 {
        self.pos
    }

    fn radius(&self) -> f32 {
        consts::PLAYER_SIZE * consts::TILES_PER_SCALE as f32
    }
}
