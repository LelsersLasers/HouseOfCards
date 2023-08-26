use macroquad::prelude as mq;
use macroquad::rand::ChooseRandom;

use crate::{colors, consts};

#[derive(Clone, Copy)]
enum Suit {
    Spades,
    Hearts,
    Clubs,
    Diamonds,
    Joker,
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

    fn is_red(&self) -> bool {
        match self.suit {
            Suit::Hearts | Suit::Diamonds => true,
            Suit::Spades | Suit::Clubs => false,
            Suit::Joker => self.value == 0,
        }
    }
    fn is_black(&self) -> bool {
        match self.suit {
            Suit::Hearts | Suit::Diamonds => false,
            Suit::Spades | Suit::Clubs => true,
            Suit::Joker => self.value == 1,
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
        ) + mq::Vec2::splat(0.5);
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
}

impl Deck {
    pub fn new() -> Self {
        let mut cards = Vec::with_capacity(54);
        for suit in [Suit::Spades, Suit::Hearts, Suit::Clubs, Suit::Diamonds].iter() {
            for value in 1..=13 {
                cards.push(Card::new(*suit, value));
            }
        }
        cards.push(Card::new(Suit::Joker, 0));
        cards.push(Card::new(Suit::Joker, 1));

        let discard = Vec::with_capacity(54);

        Self { cards, discard }
    }

    pub fn draw_card(&mut self) -> Option<Card> {
        let card = self.cards.pop();
        if let Some(card) = card {
            self.discard.push(DiscardCardDrawInfo::new(card));
        }
        card
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

    pub fn draw(&self, scale: f32) {
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

        let cards_corner =
            cards_outline_corner + mq::Vec2::new(deck_spacing_inside, deck_spacing_inside);

        mq::draw_rectangle(
            cards_outline_corner.x,
            cards_outline_corner.y,
            deck_outline_width,
            deck_outline_height,
            colors::NORD3_ALPHA,
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
            mq::draw_rectangle(
                cards_corner.x,
                cards_corner.y,
                deck_width,
                deck_height,
                colors::NORD6,
            );
            mq::draw_rectangle_lines(
                cards_corner.x,
                cards_corner.y,
                deck_width,
                deck_height,
                deck_thickness,
                colors::NORD4,
            );
            // TODO: red line pattern
        }

        let discard_outline_corner = mq::Vec2::new(
            cards_outline_corner.x - (deck_spacing_outside + deck_outline_width),
            deck_spacing_outside,
        );

        let discord_corner =
            discard_outline_corner + mq::Vec2::new(deck_spacing_inside, deck_spacing_inside);
        let discard_center = discard_outline_corner
            + mq::Vec2::new(deck_outline_width / 2.0, deck_outline_height / 2.0);

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
            mq::draw_rectangle_ex(
                discard_center.x,
                discard_center.y,
                deck_width,
                deck_height,
                mq::DrawRectangleParams {
                    color: colors::NORD6,
                    rotation: card.rotation,
                    offset: card.offset,
                },
            );
        }
    }
}
