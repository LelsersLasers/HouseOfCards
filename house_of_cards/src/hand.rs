use crate::{colors, consts, deck, util, weapon};
use macroquad::prelude as mq;

struct Slot {
    card: deck::Card,
    weapon: weapon::Weapon,
}

pub struct HandDrawDimensions {
    pub x: f32,
    pub y: f32,
    pub card_width: f32,
    pub card_height: f32,
    pub spacing: f32,
}

pub struct Hand {
    slots: Vec<Slot>,  // len = 5
    pub active: usize, // Index of active card (0-4)
}
impl Hand {
    pub fn new(deck: &mut deck::Deck) -> Self {
        let mut slots = Vec::with_capacity(5);
        for _ in 0..5 {
            let card = deck.draw_card();
            let weapon = card.get_weapon();
            slots.push(Slot { card, weapon });
        }
        Self { slots, active: 0 }
    }
    pub fn active_weapon(&self) -> &weapon::Weapon {
        &self.slots[self.active].weapon
    }

    pub fn active_card(&self) -> deck::Card {
        self.slots[self.active].card
    }

    pub fn can_shoot(&self) -> bool {
        self.active_weapon().can_shoot()
    }

    pub fn get_ms_penalty(&self) -> f32 {
        self.active_weapon().get_ms_penalty()
    }

    pub fn update(&mut self, delta: f32) {
        for slot in self.slots.iter_mut() {
            slot.weapon.update(delta);
        }
    }

    pub fn try_shoot(&mut self) -> util::Shot {
        self.slots[self.active].weapon.try_shoot()
    }

    pub fn hand_draw_dimensions(scale: f32) -> HandDrawDimensions {
        let max_width = consts::HAND_TOTAL_MAX_WIDTH * scale;
        let max_height = consts::HAND_TOTAL_MAX_HEIGHT * scale;

        let card_width = 132.0;
        let card_height = 180.0;
        let total_width = card_width * 5.0 + consts::HAND_SPACING * card_width * 4.0;
        let ratio = total_width / card_height;

        let total_width = max_width.min(max_height * ratio);
        let total_height = max_height.min(max_width / ratio);
        let x = (mq::screen_width() - total_width) / 2.0;
        let y = mq::screen_height() - total_height - consts::HAND_BOTTOM_PADDING * scale;
        let card_width = total_width / (5.0 + consts::HAND_SPACING * 4.0);

        HandDrawDimensions {
            x,
            y,
            card_width,
            card_height: total_height,
            spacing: consts::HAND_SPACING * card_width,
        }
    }

    pub fn draw(&self, cards_texture: &mq::Texture2D, scale: f32) {
        let HandDrawDimensions {
            mut x,
            y,
            card_width,
            card_height,
            spacing,
        } = Self::hand_draw_dimensions(scale);

        for (i, slot) in self.slots.iter().enumerate() {
            let card = &slot.card;
            let weapon = &slot.weapon;

            let outline_x = x - consts::HAND_SPACING * card_width / 2.0;
            let outline_width = card_width * (1.0 + consts::HAND_SPACING);
            let outline_y = y - consts::HAND_SPACING * card_width / 2.0;
            let outline_height = card_height + card_width * consts::HAND_SPACING;
            let outline_thickness = consts::HAND_OUTLINE_THICKNESS * scale;

            let ratio = weapon.time_until_next_shot / (1.0 / weapon.fire_rate);
            let inner_height = ratio * outline_height;
            let inner_y = outline_y + (outline_height - inner_height);

            if i == self.active {
                mq::draw_rectangle(
                    outline_x,
                    outline_y,
                    outline_width,
                    outline_height,
                    colors::NORD4_BIG_ALPHA,
                );
                mq::draw_rectangle(
                    outline_x,
                    inner_y,
                    outline_width,
                    inner_height,
                    colors::NORD14,
                );
                mq::draw_rectangle_lines(
                    outline_x,
                    outline_y,
                    outline_width,
                    outline_height,
                    outline_thickness,
                    colors::NORD5,
                );
            } else {
                mq::draw_rectangle(
                    outline_x,
                    inner_y,
                    outline_width,
                    inner_height,
                    colors::NORD11,
                );
            }

            let texture_source = card.get_texture_source();
            mq::draw_texture_ex(
                cards_texture,
                x,
                y,
                mq::WHITE,
                mq::DrawTextureParams {
                    dest_size: Some(mq::Vec2::new(card_width, card_height)),
                    source: Some(texture_source),
                    ..Default::default()
                },
            );

            x += card_width + spacing;
        }
    }
}
