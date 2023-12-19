
struct Slot {
	card_idx: usize, // Index of card in hand
	cooldown: f32, // Seconds until card can be used again
}

pub struct Hand {
	cards: Vec<Slot>,
	active: usize, // Index of active card (0-4)
}