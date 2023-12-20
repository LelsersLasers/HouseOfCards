
struct Slot {
	card_idx: usize, // Index of card
	cooldown: f32, // Seconds until card can be used again
}

pub struct Hand {
	cards: [Slot; 5],
	active: usize, // Index of active card (0-4)
}