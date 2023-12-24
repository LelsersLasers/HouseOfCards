use crate::{colors, consts, deck, mouse, util, weapon};
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
        let mut discarded_cards = Vec::new();

        while slots.len() < 5 {
            let card = deck.draw_card();
            if card.is_ace()
                || card.is_face()
                || card.suit == deck::Suit::Joker
                || card.value >= consts::SLOT_MAX_START_VALUE
            {
                discarded_cards.push(card);
                continue;
            }

            let weapon = card.get_weapon();
            slots.push(Slot { card, weapon });
        }

        for card in discarded_cards {
            deck.add_card(card);
        }
        deck.shuffle();

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

    pub fn set_card(&mut self, card: deck::Card) {
        let weapon = card.get_weapon();
        self.slots[self.active] = Slot { card, weapon };
    }

    pub fn try_shoot(&mut self) -> util::Shot {
        self.slots[self.active].weapon.try_shoot()
    }

    pub fn hand_draw_dimensions(scale: f32) -> HandDrawDimensions {
        let max_width = consts::HAND_TOTAL_MAX_WIDTH * scale;
        let max_height = consts::HAND_TOTAL_MAX_HEIGHT * scale;

        let total_width =
            consts::CARD_PX_WIDTH * 5.0 + consts::HAND_SPACING * consts::CARD_PX_WIDTH * 4.0;
        let ratio = total_width / consts::CARD_PX_HEIGHT;

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

    pub fn draw(&self, cards_texture: &mq::Texture2D, scale: f32) -> f32 {
        let HandDrawDimensions {
            mut x,
            y,
            card_width,
            card_height,
            spacing,
        } = Self::hand_draw_dimensions(scale);

        let outline_width = card_width * (1.0 + consts::HAND_SPACING);
        let outline_y = y - consts::HAND_SPACING * card_width / 2.0;
        let outline_height = card_height + card_width * consts::HAND_SPACING;
        let outline_thickness = consts::HAND_OUTLINE_THICKNESS * scale;

        for (i, slot) in self.slots.iter().enumerate() {
            let card = &slot.card;
            let weapon = &slot.weapon;

            let outline_x = x - consts::HAND_SPACING * card_width / 2.0;

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

        y
    }
}

pub fn clicked_on(rect: mq::Rect, need_click_after: f32, mouse_info: &mouse::MouseInfo, on_release: bool) -> bool {
    if need_click_after > mouse_info.last_click_time() {
        return false;
    }

    let mouse_pos_click = mouse_info.get_last_click();
    let mouse_pos_now = mouse_info.get_last_pos();

    (mouse_info.mouse_released() || !on_release) && rect.contains(mouse_pos_click) && rect.contains(mouse_pos_now)
}

pub struct CardChoicesButtonRects {
    pub cards: Vec<mq::Rect>,
    pub swap_button: mq::Rect,
    pub discard_button: mq::Rect,
}

pub fn draw_card_choices(
    card_choices: &[deck::Card],
    cards_texture: &mq::Texture2D,
    font: &mq::Font,
    selected: usize,
    score_text_bottom_y: f32,
    hand_top_y: f32,
    scale: f32,
) -> CardChoicesButtonRects {
    let max_width = consts::CARD_CHOICE_MAX_WIDTH * mq::screen_width();

    let y_gap = hand_top_y - score_text_bottom_y;
    let max_height = consts::CARD_CHOICE_MAX_PERCENT_HEIGHT * (y_gap);

    let total_width =
        consts::CARD_PX_WIDTH * 4.0 + consts::CARD_CHOICE_SPACING * consts::CARD_PX_WIDTH * 3.0;
    let ratio = total_width / consts::CARD_PX_HEIGHT;

    let total_width = max_width.min(max_height * ratio);
    let total_height = max_height.min(max_width / ratio);
    let mut x = (mq::screen_width() - total_width) / 2.0;    
    let y = score_text_bottom_y + (y_gap - total_height) / 2.0;
    let card_width = total_width / (4.0 + consts::CARD_CHOICE_SPACING * 3.0);

    let outline_width = card_width * (1.0 + consts::CARD_CHOICE_SPACING);
    let outline_y = y - consts::CARD_CHOICE_SPACING * card_width / 2.0;
    let outline_height = total_height + card_width * consts::CARD_CHOICE_SPACING;
    let outline_thickness = consts::CARD_CHOICE_OUTLINE_THICKNESS * scale;

    let mut cards_button_rects = Vec::with_capacity(card_choices.len());

    for (i, card) in card_choices.iter().enumerate() {
        let outline_x = x - consts::CARD_CHOICE_SPACING * card_width / 2.0;

        if i == selected {
            mq::draw_rectangle(
                outline_x,
                outline_y,
                outline_width,
                outline_height,
                colors::NORD4_BIG_ALPHA,
            );
            mq::draw_rectangle_lines(
                outline_x,
                outline_y,
                outline_width,
                outline_height,
                outline_thickness,
                colors::NORD5,
            );
        }

        let texture_source = card.get_texture_source();
        mq::draw_texture_ex(
            cards_texture,
            x,
            y,
            mq::WHITE,
            mq::DrawTextureParams {
                dest_size: Some(mq::Vec2::new(card_width, total_height)),
                source: Some(texture_source),
                ..Default::default()
            },
        );

        cards_button_rects.push(mq::Rect::new(x, y, card_width, total_height));

        x += card_width + consts::CARD_CHOICE_SPACING * card_width;
    }

    let button_height = (total_height - consts::CARD_CHOICE_SPACING * card_width) / 2.0;
    let y2 = y + total_height - button_height;
    mq::draw_rectangle(x, y, card_width, button_height, colors::NORD14_BIG_ALPHA);
    mq::draw_rectangle_lines(
        x,
        y,
        card_width,
        button_height,
        outline_thickness * 2.0,
        colors::NORD14,
    );

    let swap_button_rect = mq::Rect::new(x, y, card_width, button_height);

    let text = "Swap";
    let font_size = (consts::CARD_CHOICE_FONT_SIZE * scale) as u16;
    let text_dims = mq::measure_text(text, Some(font), font_size, 1.0);
    let text_pos = mq::Vec2::new(
        x + card_width / 2.0 - text_dims.width / 2.0,
        y + button_height / 2.0 + text_dims.offset_y / 2.25,
    );
    mq::draw_text_ex(
        text,
        text_pos.x,
        text_pos.y,
        mq::TextParams {
            font: Some(font),
            font_size,
            font_scale: 1.0,
            color: colors::NORD14,
            ..mq::TextParams::default()
        },
    );

    mq::draw_rectangle(x, y2, card_width, button_height, colors::NORD11_BIG_ALPHA);
    mq::draw_rectangle_lines(
        x,
        y2,
        card_width,
        button_height,
        outline_thickness * 2.0,
        colors::NORD11,
    );

    let discard_button_rect = mq::Rect::new(x, y2, card_width, button_height);

    let text = "Discard All";
    let font_size = (consts::CARD_CHOICE_FONT_SIZE * scale) as u16;
    let text_dims = mq::measure_text(text, Some(font), font_size, 1.0);
    let text_pos = mq::Vec2::new(
        x + card_width / 2.0 - text_dims.width / 2.0,
        y2 + button_height / 2.0 + text_dims.offset_y / 2.25,
    );
    mq::draw_text_ex(
        text,
        text_pos.x,
        text_pos.y,
        mq::TextParams {
            font: Some(font),
            font_size,
            font_scale: 1.0,
            color: colors::NORD11,
            ..mq::TextParams::default()
        },
    );

    CardChoicesButtonRects {
        cards: cards_button_rects,
        swap_button: swap_button_rect,
        discard_button: discard_button_rect,
    }
}
