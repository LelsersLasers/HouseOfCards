use macroquad::rand::ChooseRandom;



#[derive(Clone, Copy)]
enum Suit {
    Spades,
    Hearts,
    Clubs,
    Diamonds,
    Joker
}

#[derive(Clone, Copy)]
struct Card {
    suit: Suit,
    value: u8
}

impl Card {
    fn new(suit: Suit, value: u8) -> Self {
        Self {
            suit,
            value
        }
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

struct Deck {
    cards: Vec<Card>,
    discard: Vec<Card>
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



        Self {
            cards,
            discard,
        }
    }

    pub fn draw_card(&mut self) -> Option<Card> {
        let card = self.cards.pop();
        if let Some(card) = card {
            self.discard.push(card);
        }
        card
    }

    pub fn combine(&mut self) {
        self.discard.reverse();
        self.cards.append(&mut self.discard);
    }

    pub fn shuffle(&mut self) {
        self.cards.shuffle();
    }
}