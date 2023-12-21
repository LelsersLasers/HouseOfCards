use macroquad::prelude as mq;
use macroquad::rand::ChooseRandom;

use crate::{consts, powerup, weapon};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Suit {
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
    pub fn new(suit: Suit, value: u8) -> Self {
        Self { suit, value }
    }

    pub fn get_texture_source(&self) -> mq::Rect {
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

    pub fn heal_amount(&self, powerups: &powerup::Powerups) -> f32 {
        if self.suit == Suit::Hearts {
            powerups.hearts_heal_amount()
        } else {
            0.0
        }
    }

    pub fn stun_time(&self, powerups: &powerup::Powerups) -> f32 {
        if self.suit == Suit::Clubs {
            powerups.clubs_stun_time()
        } else {
            0.0
        }
    }

    pub fn damage(&self, powerups: Option<&powerup::Powerups>) -> f32 {
        let mut damage = match self {
            // TODO: new ideas for Joker and Ace
            Self {
                suit: Suit::Joker,
                value: _,
            } => -5.0,
            Self { suit: _, value } => {
                if self.is_face() {
                    20.0
                } else if self.is_ace() {
                    f32::INFINITY
                } else {
                    *value as f32
                }
            }
        };
        if let Some(powerups) = powerups {
            if self.suit == Suit::Spades {
                damage *= powerups.spades_damage_mod();
            }
        }

        damage
    }

    pub fn get_weapon(&self) -> weapon::Weapon {
        if self.suit == Suit::Joker {
            consts::JOKER_WEAPON
        } else if self.is_ace() {
            consts::ACE_WEAPON
        } else if self.is_face() {
            consts::FACE_WEAPON
        } else {
            consts::ELSE_WEAPON
        }
    }

    fn is_face(&self) -> bool {
        self.value > 10
    }

    fn is_ace(&self) -> bool {
        self.value == 1
    }
}

pub struct Deck {
    all_cards: Vec<Card>,
    cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Self {
        let mut all_cards = Vec::with_capacity(54);
        for suit in [Suit::Spades, Suit::Hearts, Suit::Clubs, Suit::Diamonds].iter() {
            for value in 1..=13 {
                all_cards.push(Card::new(*suit, value));
            }
        }
        all_cards.push(Card::new(Suit::Joker, 0));
        all_cards.push(Card::new(Suit::Joker, 1));

        let cards = all_cards.clone();

        let mut deck = Self { all_cards, cards };
        deck.shuffle();
        deck
    }

    pub fn draw_card(&mut self) -> Card {
        let card = self.cards.pop();
        if let Some(card) = card {
            card
        } else {
            self.refresh();
            self.cards.pop().unwrap()
        }
    }

    fn refresh(&mut self) {
        self.cards = self.all_cards.clone();
        self.shuffle();
    }

    pub fn shuffle(&mut self) {
        self.cards.shuffle();
    }
}
