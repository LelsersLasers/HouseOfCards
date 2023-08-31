use macroquad::prelude as mq;

use crate::{colors, consts, mouse};

#[derive(Clone, Copy)]
pub enum PowerupPickLocation {
    Left,
    Center,
    Right,
}

impl PowerupPickLocation {
    pub fn all_locations() -> Vec<PowerupPickLocation> {
        vec![
            PowerupPickLocation::Left,
            PowerupPickLocation::Center,
            PowerupPickLocation::Right,
        ]
    }

    pub fn as_i32(&self) -> i32 {
        match self {
            PowerupPickLocation::Left => 0,
            PowerupPickLocation::Center => 1,
            PowerupPickLocation::Right => 2,
        }
    }
}

struct OutlineDrawDimensions {
    pos: mq::Vec2,
    width: f32,
    height: f32,
}

struct CardDrawDimensions {
    pos: mq::Vec2,
    width: f32,
    height: f32,
}

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum Powerup {
    Damage,
    Health,
    Reload,
    Speed,
    Diamonds,
    Hearts,
    Clubs,
    Spades,
}

impl Powerup {
    pub fn pick_three() -> Vec<Powerup> {
        // three unique random powerups
        let mut powerups = Vec::new();
        let mut powerup = Powerup::pick();
        while powerups.len() < 3 {
            if !powerups.contains(&powerup) {
                powerups.push(powerup);
            }
            powerup = Powerup::pick();
        }

        powerups
    }

    fn pick() -> Powerup {
        // random powerup
        match mq::rand::gen_range(0, 8) {
            0 => Powerup::Damage,
            1 => Powerup::Health,
            2 => Powerup::Reload,
            3 => Powerup::Speed,
            4 => Powerup::Diamonds,
            5 => Powerup::Hearts,
            6 => Powerup::Clubs,
            7 => Powerup::Spades,
            _ => unreachable!(),
        }
    }

    fn outline_draw_dimensions() -> OutlineDrawDimensions {
        let max_width = consts::POWERUP_PICK_OUTLINE_WIDTH * mq::screen_width();
        let max_height = consts::POWERUP_PICK_OUTLINE_HEIGHT * mq::screen_height();

        let width = max_width.min(max_height * consts::POWERUP_PICK_OUTLINE_RATIO);
        let height = max_height.min(max_width / consts::POWERUP_PICK_OUTLINE_RATIO);

        let pos = mq::Vec2::new(
            (mq::screen_width() - width) / 2.0,
            (mq::screen_height() - height) / 2.0,
        );

        OutlineDrawDimensions { pos, width, height }
    }

    fn card_draw_dimensions(location: PowerupPickLocation, scale: f32) -> CardDrawDimensions {
        let OutlineDrawDimensions {
            pos: outline_pos,
            width: outline_width,
            height: outline_height,
        } = Self::outline_draw_dimensions();

        let outline_padding = consts::POWERUP_PICK_OUTLINE_PADDING * scale;

        let width = (outline_width - 4.0 * outline_padding) / 3.0;
        let height = outline_height - 2.0 * outline_padding;

        let card0_pos = outline_pos + mq::Vec2::new(outline_padding, outline_padding);
        let pos =
            card0_pos + mq::Vec2::new((outline_padding + width) * location.as_i32() as f32, 0.0);

        CardDrawDimensions { pos, width, height }
    }

    pub fn draw_outline(scale: f32) {
        let OutlineDrawDimensions { pos, width, height } = Self::outline_draw_dimensions();

        let thickness = consts::POWERUP_PICK_OUTLINE_THICKNESS * scale;

        mq::draw_rectangle(pos.x, pos.y, width, height, colors::NORD4_BIG_ALPHA);
        mq::draw_rectangle_lines(pos.x, pos.y, width, height, thickness, colors::NORD4);
    }

    pub fn clicked_on(
        &self,
        location: PowerupPickLocation,
        mouse_info: &mouse::MouseInfo,
        scale: f32,
    ) -> bool {
        let CardDrawDimensions { pos, width, height } = Self::card_draw_dimensions(location, scale);

        let mouse_pos = mouse_info.get_last_pos();

        mq::is_mouse_button_pressed(mq::MouseButton::Left)
            && mouse_pos.x >= pos.x
            && mouse_pos.x <= pos.x + width
            && mouse_pos.y >= pos.y
            && mouse_pos.y <= pos.y + height
    }

    pub fn draw(&self, location: PowerupPickLocation, font: mq::Font, scale: f32) {
        let CardDrawDimensions { pos, width, height } = Self::card_draw_dimensions(location, scale);

        mq::draw_rectangle(pos.x, pos.y, width, height, self.color_light_version());

        mq::draw_rectangle_lines(
            pos.x,
            pos.y,
            width,
            height,
            consts::POWERUP_PICK_OUTLINE_THICKNESS * scale,
            self.color(),
        );

        let center = mq::Vec2::new(pos.x + width / 2.0, pos.y + height / 2.0);
        {
            // let center = center - mq::Vec2::new(0.0, consts::POWERUP_PICK_FONT_SPACING_CENTER * scale);
            let main_text = self.main_text();
            let main_text_font_size = (consts::POWERUP_PICK_FONT_LARGE * scale).round() as u16;
            let text_dims_large = main_text
                .iter()
                .map(|t| mq::measure_text(t, Some(font), main_text_font_size, 1.0))
                .collect::<Vec<_>>();
            let total_height = text_dims_large.iter().map(|d| d.height).sum::<f32>();
            let mut y = center.y - total_height;

            for i in 0..main_text.len() {
                let text = main_text[i];
                let text_dims = text_dims_large[i];
                let x = center.x - text_dims_large[i].width / 2.0;

                mq::draw_text_ex(
                    text,
                    x,
                    y + text_dims.offset_y,
                    mq::TextParams {
                        font,
                        font_size: main_text_font_size,
                        font_scale: 1.0,
                        color: self.color(),
                        ..mq::TextParams::default()
                    },
                );
                y += text_dims.height;
            }
        }

        {
            let center =
                center + mq::Vec2::new(0.0, consts::POWERUP_PICK_FONT_SPACING_CENTER * scale);
            let sub_text = self.sub_text();
            let sub_text_font_size = (consts::POWERUP_PICK_FONT_SMALL * scale).round() as u16;
            let text_dims_small = sub_text
                .iter()
                .map(|t| mq::measure_text(t, Some(font), sub_text_font_size, 1.0))
                .collect::<Vec<_>>();
            // let total_height = text_dims_small.iter().map(|d| d.height).sum::<f32>();
            let mut y = center.y;

            for i in 0..sub_text.len() {
                let text = sub_text[i];
                let text_dims = text_dims_small[i];
                let x = center.x - text_dims_small[i].width / 2.0;

                mq::draw_text_ex(
                    text,
                    x,
                    y + text_dims.offset_y,
                    mq::TextParams {
                        font,
                        font_size: sub_text_font_size,
                        font_scale: 1.0,
                        color: self.color(),
                        ..mq::TextParams::default()
                    },
                );
                y += text_dims.height;
            }
        }
    }

    fn color(&self) -> mq::Color {
        match self {
            Powerup::Damage => colors::NORD11,
            Powerup::Health => colors::NORD14,
            Powerup::Reload => colors::NORD12,
            Powerup::Speed => colors::NORD13,
            Powerup::Diamonds => colors::NORD7,
            Powerup::Hearts => colors::NORD8,
            Powerup::Clubs => colors::NORD9,
            Powerup::Spades => colors::NORD10,
        }
    }

    fn color_light_version(&self) -> mq::Color {
        let mut color = self.color();
        color.r = (color.r + 0.3).min(1.0);
        color.g = (color.g + 0.3).min(1.0);
        color.b = (color.b + 0.3).min(1.0);

        color
    }

    fn main_text(&self) -> Vec<&str> {
        match self {
            Powerup::Damage => vec!["+1 Damage"],
            Powerup::Health => vec!["+2 Health"],
            Powerup::Reload => vec!["+33% Reload Speed"],
            Powerup::Speed => vec!["+5% Speed"],
            Powerup::Diamonds => vec!["Diamonds:", "Pierce +1 Enemies"],
            Powerup::Hearts => vec!["Hearts:", "+5% chance to heal"],
            Powerup::Clubs => vec!["Clubs:", "+0.25s Stun"],
            Powerup::Spades => vec!["Spades:", "+10% chance to", "double damage"],
        }
    }

    fn sub_text(&self) -> Vec<&str> {
        match self {
            Powerup::Damage => vec!["for all cards"],
            Powerup::Health => vec!["gain hp and max hp"],
            Powerup::Reload => vec!["than current reload speed"],
            Powerup::Speed => vec!["from base speed"],
            Powerup::Diamonds => vec!["bullets go through", "an additional enemy"],
            Powerup::Hearts => vec!["1 hp on hit"],
            Powerup::Clubs => vec!["on hit"],
            Powerup::Spades => vec!["can stack"],
        }
    }
}

pub struct Powerups {
    pub powerups: Vec<Powerup>,
}

impl Powerups {
    pub fn new() -> Self {
        Self {
            powerups: Vec::new(),
        }
    }

    pub fn add(&mut self, powerup: Powerup) {
        self.powerups.push(powerup);
    }

    pub fn count(&self, powerup: &Powerup) -> usize {
        self.powerups.iter().filter(|p| **p == *powerup).count()
    }

    pub fn damage_add(&self) -> f32 {
        self.count(&Powerup::Damage) as f32 * consts::DAMAGE_ADD
    }

    // pub fn health_add(&self) -> f32 {
    //     self.count(&Powerup::Health) as f32 * consts::HEALTH_ADD
    // }

    pub fn reload_mod(&self) -> f32 {
        (1.0 - consts::RELOAD_MOD).powi(self.count(&Powerup::Reload) as i32)
    }

    pub fn speed_mod(&self) -> f32 {
        self.count(&Powerup::Speed) as f32 * consts::SPEED_MOD + 1.0
    }

    pub fn diamonds_bullet_hp(&self) -> i32 {
        self.count(&Powerup::Diamonds) as i32 + 1
    }

    pub fn hearts_heal_amount(&self) -> f32 {
        let count = self.count(&Powerup::Hearts);
        let mut amount = 0.0;
        for _ in 0..count {
            if mq::rand::gen_range(0.0, 1.0) < consts::HEARTS_HEAL_CHANCE {
                amount += 1.0;
            }
        }

        amount
    }

    pub fn clubs_stun_time(&self) -> f32 {
        self.count(&Powerup::Clubs) as f32 * consts::CLUBS_STUN_TIME
    }

    pub fn spades_damage_mod(&self) -> f32 {
        let count = self.count(&Powerup::Spades);
        let mut modifier = 1.0;
        for _ in 0..count {
            if mq::rand::gen_range(0.0, 1.0) < consts::SPADES_DAMAGE_CHANCE {
                modifier *= 2.0;
            }
        }

        modifier
    }
}
