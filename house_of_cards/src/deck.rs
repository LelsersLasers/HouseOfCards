use macroquad::prelude as mq;
use macroquad::rand::ChooseRandom;

use crate::{colors, consts, weapon};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Suit {
    Spades,
    Hearts,
    Clubs,
    Diamonds,
    Joker,
    Back,
}

#[derive(Clone, Copy)]
pub struct Card {
    suit: Suit,
    value: u8,
}

impl Card {
    fn new(suit: Suit, value: u8) -> Self {
        Self { suit, value }
    }

    fn get_texture_source(&self) -> mq::Rect {
        let (idx_x, idx_y) = match self {
            Self {
                suit: Suit::Joker | Suit::Back,
                value,
            } => (0, (self.suit == Suit::Joker) as usize * 2 + *value as usize),
            Self { suit, value } => {
                let suit_idx = match suit {
                    Suit::Hearts => 0,
                    Suit::Spades => 1,
                    Suit::Diamonds => 2,
                    Suit::Clubs => 3,
                    _ => unreachable!(),
                };
                if self.is_ace() {
                    (13, suit_idx)
                } else {
                    (*value as usize - 1, suit_idx)
                }
            }
        };

        let start = mq::Vec2::splat(24.0);
        let card_width = 132.0;
        let card_height = 180.0;
        let card_spacing = 8.0;

        let x = start.x + idx_x as f32 * (card_width + card_spacing);
        let y = start.y + idx_y as f32 * (card_height + card_spacing);

        mq::Rect::new(x, y, card_width, card_height)
    }

    pub fn is_red(&self) -> bool {
        match self.suit {
            Suit::Hearts | Suit::Diamonds => true,
            Suit::Spades | Suit::Clubs => false,
            Suit::Joker | Suit::Back => self.value == 0,
        }
    }
    // pub fn is_black(&self) -> bool {
    //     match self.suit {
    //         Suit::Hearts | Suit::Diamonds => false,
    //         Suit::Spades | Suit::Clubs => true,
    //         Suit::Joker | Suit::Back => self.value == 1,
    //     }
    // }

    pub fn damage(&self) -> f32 {
        match self {
            Self {
                suit: Suit::Joker,
                value,
            } => -5.0,
            Self { suit: _, value } => {
                if self.is_face() {
                    10.0
                } else if self.is_ace() {
                    f32::INFINITY
                } else {
                    *value as f32
                }
            }
        }
    }

    fn is_face(&self) -> bool {
        self.value > 10
    }

    fn is_ace(&self) -> bool {
        self.value == 1
    }
}

struct DiscardCardDrawInfo {
    card: Card,
    rotation: f32, // in radians
    offset: mq::Vec2,
}

impl DiscardCardDrawInfo {
    fn new(card: Card) -> Self {
        let rotation = mq::rand::gen_range(-consts::DISCARD_ROTATION, consts::DISCARD_ROTATION);
        let offset = mq::Vec2::new(
            mq::rand::gen_range(-consts::DISCARD_OFFSET, consts::DISCARD_OFFSET),
            mq::rand::gen_range(-consts::DISCARD_OFFSET, consts::DISCARD_OFFSET),
        );
        Self {
            card,
            rotation,
            offset,
        }
    }
}

pub struct Deck {
    cards: Vec<Card>,
    discard: Vec<DiscardCardDrawInfo>,
    cards_texture: mq::Texture2D,
}

impl Deck {
    pub fn new(cards_texture: mq::Texture2D) -> Self {
        let mut cards = Vec::with_capacity(54);
        for suit in [Suit::Spades, Suit::Hearts, Suit::Clubs, Suit::Diamonds].iter() {
            for value in 1..=13 {
                cards.push(Card::new(*suit, value));
            }
        }
        cards.push(Card::new(Suit::Joker, 0));
        cards.push(Card::new(Suit::Joker, 1));

        let discard = Vec::with_capacity(54);

        let mut deck = Self {
            cards,
            discard,
            cards_texture,
        };
        deck.shuffle();
        deck
    }

    pub fn draw_card(&mut self) -> Option<Card> {
        let card = self.cards.pop();
        if let Some(card) = card {
            self.discard.push(DiscardCardDrawInfo::new(card));
        }
        card
    }

    pub fn is_full(&self) -> bool {
        self.discard.is_empty()
    }

    pub fn is_empty(&self) -> bool {
        self.cards.is_empty()
    }

    pub fn combine(&mut self) {
        let mut discard_cards = self
            .discard
            .iter()
            .map(|discard_card| discard_card.card)
            .collect::<Vec<_>>();
        discard_cards.reverse();
        self.cards.append(&mut discard_cards);

        self.discard.clear();
    }

    pub fn shuffle(&mut self) {
        self.cards.shuffle();
    }

    pub fn draw(&self, weapon: &weapon::Weapon, scale: f32) {
        // Draw stack of cards in top right corner
        // Draw discard pile on the left of the stack

        let deck_spacing_outside = consts::DECK_SPACING_OUTSIDE * scale;
        let deck_spacing_inside = consts::DECK_SPACING_INSIDE * scale;

        let deck_width = consts::DECK_WIDTH * scale;
        let deck_height = consts::DECK_HEIGHT * scale;

        let deck_outline_width = deck_width + 2.0 * deck_spacing_inside;
        let deck_outline_height = deck_height + 2.0 * deck_spacing_inside;

        let deck_thickness = consts::DECK_THICKNESS * scale;

        let cards_outline_corner = mq::Vec2::new(
            mq::screen_width() - (deck_spacing_outside + deck_outline_width),
            deck_spacing_outside,
        );

        let cards_corner = cards_outline_corner + mq::Vec2::splat(deck_spacing_inside);

        mq::draw_rectangle(
            cards_outline_corner.x,
            cards_outline_corner.y,
            deck_outline_width,
            deck_outline_height,
            if weapon.can_shoot() || self.is_empty() {
                colors::NORD3_ALPHA
            } else if weapon.is_reloading() {
                colors::NORD11_ALPHA
            } else {
                colors::NORD14_ALPHA
            },
        );
        mq::draw_rectangle_lines(
            cards_outline_corner.x,
            cards_outline_corner.y,
            deck_outline_width,
            deck_outline_height,
            deck_thickness,
            colors::NORD1,
        );

        if !self.cards.is_empty() {
            let card_back = Card::new(Suit::Back, 1);
            let texture_source = card_back.get_texture_source();
            mq::draw_texture_ex(
                self.cards_texture,
                cards_corner.x,
                cards_corner.y,
                mq::WHITE,
                mq::DrawTextureParams {
                    dest_size: Some(mq::Vec2::new(deck_width, deck_height)),
                    source: Some(texture_source),
                    ..Default::default()
                },
            );
        }

        let discard_outline_corner = mq::Vec2::new(
            cards_outline_corner.x - (deck_spacing_outside + deck_outline_width),
            deck_spacing_outside,
        );
        let discard_corner = discard_outline_corner + mq::Vec2::splat(deck_spacing_inside);

        mq::draw_rectangle(
            discard_outline_corner.x,
            discard_outline_corner.y,
            deck_outline_width,
            deck_outline_height,
            colors::NORD3_ALPHA,
        );
        mq::draw_rectangle_lines(
            discard_outline_corner.x,
            discard_outline_corner.y,
            deck_outline_width,
            deck_outline_height,
            deck_thickness,
            colors::NORD1,
        );

        let start_index = self.discard.len().saturating_sub(consts::DISCARD_TO_DRAW);
        for i in start_index..self.discard.len() {
            let card = &self.discard[i];
            let texture_source = card.card.get_texture_source();
            let x = discard_corner.x + card.offset.x * deck_width;
            let y = discard_corner.y + card.offset.y * deck_height;
            mq::draw_texture_ex(
                self.cards_texture,
                x,
                y,
                mq::WHITE,
                mq::DrawTextureParams {
                    dest_size: Some(mq::Vec2::new(deck_width, deck_height)),
                    source: Some(texture_source),
                    rotation: card.rotation,
                    ..Default::default()
                },
            );
        }
    }
}
