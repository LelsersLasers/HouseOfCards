use macroquad::prelude as mq;

use crate::{colors, consts, deck};

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum Powerup {
    Diamonds,
    Hearts,
    Clubs,
    Spades,
}

impl Powerup {
    pub fn pick_three() -> Vec<Powerup> {
        // three unique random powerups
        let mut powerups = Vec::new();
        let mut powerup = Self::pick_card();
        while powerups.len() < 3 {
            if !powerups.contains(&powerup) {
                powerups.push(powerup);
            }
            powerup = Self::pick_card();
        }

        powerups
    }

    pub fn pick_card() -> Powerup {
        // random card powerup
        match mq::rand::gen_range(0, 4) {
            0 => Powerup::Diamonds,
            1 => Powerup::Hearts,
            2 => Powerup::Clubs,
            3 => Powerup::Spades,
            _ => unreachable!(),
        }
    }

    pub fn draw_small(&self, id: usize, cards_texture: &mq::Texture2D, scale: f32) {
        let id_y = id % consts::POWERUP_DISPLAY_MAX_HEIGHT;
        let id_x = id / consts::POWERUP_DISPLAY_MAX_HEIGHT;

        let spacing = consts::POWERUP_DIPLAY_SPACING * scale;
        let y_offset = consts::POWERUP_DIPLAY_Y_OFFSET * scale;
        let size = consts::POWERUP_DISPLAY_SIZE * scale;

        let x = spacing + id_x as f32 * (size + spacing);
        let y = spacing + id_y as f32 * (size + spacing) + y_offset;

        mq::draw_rectangle(x, y, size, size, self.color_light_version());

        let texture_source = self.suit().get_suit_icon_source();

        mq::draw_texture_ex(
            cards_texture,
            x,
            y,
            mq::WHITE,
            mq::DrawTextureParams {
                dest_size: Some(mq::Vec2::splat(size)),
                source: Some(texture_source),
                ..Default::default()
            },
        );

        mq::draw_rectangle_lines(
            x,
            y,
            size,
            size,
            consts::POWERUP_OUTLINE_THICKNESS * scale,
            self.color(),
        );
    }

    fn suit(&self) -> deck::Suit {
        match self {
            Powerup::Diamonds => deck::Suit::Diamonds,
            Powerup::Hearts => deck::Suit::Hearts,
            Powerup::Clubs => deck::Suit::Clubs,
            Powerup::Spades => deck::Suit::Spades,
        }
    }

    fn color(&self) -> mq::Color {
        match self {
            Powerup::Diamonds => colors::NORD11,
            Powerup::Hearts => colors::NORD14,
            Powerup::Clubs => colors::NORD12,
            Powerup::Spades => colors::NORD15,
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
            Powerup::Diamonds => vec!["Diamonds:", "Pierce", "+1 Enemies"],
            Powerup::Hearts => vec!["Hearts:", "+2% chance", "to heal"],
            Powerup::Clubs => vec!["Clubs:", "+0.1s Stun"],
            Powerup::Spades => vec!["Spades:", "+33% chance", "to double", "damage"],
        }
    }

    fn sub_text(&self) -> Vec<&str> {
        match self {
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

    pub fn draw(&self, cards_texture: &mq::Texture2D, scale: f32) {
        for (i, powerup) in self.powerups.iter().enumerate() {
            powerup.draw_small(i, cards_texture, scale);
        }
    }

    pub fn add(&mut self, powerup: Powerup) {
        self.powerups.push(powerup);
    }

    pub fn count(&self, powerup: &Powerup) -> usize {
        self.powerups.iter().filter(|p| **p == *powerup).count()
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

pub fn draw_powerup_choices(
    powerup_choices: &[Powerup],
    font: &mq::Font,
    score_text_bottom_y: f32,
    hand_top_y: f32,
    scale: f32,
) -> Vec<mq::Rect> {
    let max_width = consts::CARD_CHOICE_MAX_WIDTH * mq::screen_width();

    let y_gap = hand_top_y - score_text_bottom_y;
    let max_height = consts::CARD_CHOICE_MAX_PERCENT_HEIGHT * (y_gap);

    let total_width =
        consts::CARD_PX_WIDTH * 3.0 + consts::CARD_CHOICE_SPACING * consts::CARD_PX_WIDTH * 2.0;
    let ratio = total_width / consts::CARD_PX_HEIGHT;

    let total_width = max_width.min(max_height * ratio);
    let total_height = max_height.min(max_width / ratio);
    let mut x = (mq::screen_width() - total_width) / 2.0;
    let y = score_text_bottom_y + (y_gap - total_height) / 2.0;
    let card_width = total_width / (3.0 + consts::CARD_CHOICE_SPACING * 2.0);

    let mut powerup_button_rects = Vec::with_capacity(powerup_choices.len());

    for powerup in powerup_choices {
        mq::draw_rectangle(
            x,
            y,
            card_width,
            total_height,
            powerup.color_light_version(),
        );
        mq::draw_rectangle_lines(
            x,
            y,
            card_width,
            total_height,
            consts::CARD_CHOICE_OUTLINE_THICKNESS * scale,
            powerup.color(),
        );

        let center = mq::Vec2::new(x + card_width / 2.0, y + total_height / 2.0);
        {
            let main_text = powerup.main_text();
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
                        font: Some(font),
                        font_size: main_text_font_size,
                        font_scale: 1.0,
                        color: powerup.color(),
                        ..mq::TextParams::default()
                    },
                );
                y += text_dims.height;
            }
        }

        {
            let center =
                center + mq::Vec2::new(0.0, consts::POWERUP_PICK_FONT_SPACING_CENTER * scale);
            let sub_text = powerup.sub_text();
            let sub_text_font_size = (consts::POWERUP_PICK_FONT_SMALL * scale).round() as u16;
            let text_dims_small = sub_text
                .iter()
                .map(|t| mq::measure_text(t, Some(font), sub_text_font_size, 1.0))
                .collect::<Vec<_>>();
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
                        font: Some(font),
                        font_size: sub_text_font_size,
                        font_scale: 1.0,
                        color: powerup.color(),
                        ..mq::TextParams::default()
                    },
                );
                y += text_dims.height;
            }
        }

        powerup_button_rects.push(mq::Rect::new(x, y, card_width, total_height));

        x += card_width + consts::CARD_CHOICE_SPACING * card_width;
    }

    powerup_button_rects
}
