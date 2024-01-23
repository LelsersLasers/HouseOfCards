use macroquad::prelude as mq;

use crate::{camera,  colors, consts};

pub enum DamageNumberColor {
	PlayerHeal,
	PlayerDamage,
	EnemyDamage,
}

pub struct DamageNumber {
	damage: i32,
	time: f32,
	pos: mq::Vec2,
	color: DamageNumberColor
}
impl DamageNumber {
	pub fn new(damage: i32, time: f32, pos: mq::Vec2, color: DamageNumberColor) -> Self {
		let pos_offset = mq::Vec2::new(
			mq::rand::gen_range(-consts::DAMAGE_NUMBER_RAND_POS, consts::DAMAGE_NUMBER_RAND_POS),
			mq::rand::gen_range(-consts::DAMAGE_NUMBER_RAND_POS, consts::DAMAGE_NUMBER_RAND_POS),
		);
		Self {
			damage,
			time,
			pos: pos + pos_offset,
			color
		}
	}
	pub fn update(&mut self, delta: f32) {
		self.time -= delta;
	}
	pub fn should_keep(&self) -> bool {
		self.time > 0.0
	}
	pub fn draw(&self, camera: &camera::Camera, font: &mq::Font, scale: f32) {
		let draw_pos = (self.pos - camera.pos) * scale / consts::TILES_PER_SCALE as f32
            + mq::Vec2::new(mq::screen_width() / 2.0, mq::screen_height() / 2.0);

		let text = format!("{}", self.damage);
		let font_size = (consts::DAMAGE_FONT_SIZE * scale) as u16;

		let text_dims = mq::measure_text(&text, Some(font), font_size, 1.0);
		let x = draw_pos.x;
		let y = draw_pos.y + text_dims.offset_y;

		let color = match self.color {
			DamageNumberColor::PlayerHeal => colors::NORD14,
			DamageNumberColor::PlayerDamage => colors::NORD11,
			DamageNumberColor::EnemyDamage => colors::NORD4,
		};

        mq::draw_text_ex(
            &text,
            x,
            y,
            mq::TextParams {
                font: Some(font),
                font_size,
                color,
                ..Default::default()
            },
        );
	}
}