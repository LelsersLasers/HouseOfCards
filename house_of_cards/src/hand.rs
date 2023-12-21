use crate::{colors, consts, deck, util, weapon};
use macroquad::prelude as mq;

struct Slot {
    card: deck::Card,
    weapon: weapon::Weapon,
}

pub struct Hand {
    slots: Vec<Slot>,  // len = 5
    pub active: usize, // Index of active card (0-4)
    cards_texture: mq::Texture2D,
}
impl Hand {
    pub fn new(deck: &mut deck::Deck, cards_texture: mq::Texture2D) -> Self {
        let mut slots = Vec::with_capacity(5);
        for _ in 0..5 {
            let card = deck.draw_card();
            let weapon = card.get_weapon();
            slots.push(Slot { card, weapon });
        }
        Self {
            slots,
            active: 0,
            cards_texture,
        }
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

    pub fn draw(&self, scale: f32) {
        let max_width = consts::HAND_TOTAL_MAX_WIDTH * scale;
        let max_height = consts::HAND_TOTAL_MAX_HEIGHT * scale;

        let card_width = 132.0;
        let card_height = 180.0;
        let total_width = card_width * 5.0 + consts::HAND_SPACING * card_width * 4.0;
        let ratio = total_width / card_height;

        let total_width = max_width.min(max_height * ratio);
        let total_height = max_height.min(max_width / ratio);
        let mut x = (mq::screen_width() - total_width) / 2.0;
        let y = mq::screen_height() - total_height - consts::HAND_BOTTOM_PADDING * scale;
        let width = total_width / (5.0 + consts::HAND_SPACING * 4.0);

        for (i, slot) in self.slots.iter().enumerate() {
            let card = &slot.card;
            let weapon = &slot.weapon;

            let outline_x = x - consts::HAND_SPACING * width / 2.0;
            let outline_width = width * (1.0 + consts::HAND_SPACING);
            let outline_y = y - consts::HAND_SPACING * width / 2.0;
            let outline_height = total_height + width * consts::HAND_SPACING;
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
                self.cards_texture,
                x,
                y,
                mq::WHITE,
                mq::DrawTextureParams {
                    dest_size: Some(mq::Vec2::new(width, total_height)),
                    source: Some(texture_source),
                    ..Default::default()
                },
            );

            x += width + consts::HAND_SPACING * width;
        }

        // mq::draw_rectangle(x, y, total_width, total_height, colors::NORD11_ALPHA);
    }
}
