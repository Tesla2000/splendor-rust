use once_cell::sync::Lazy;
use crate::card::card::Card;
use crate::card::cost::Cost;
use crate::card::tier::Tier;
use crate::resource::Resource;

pub struct CardStorage {
    cards: Vec<Card>,
}

impl CardStorage {
    fn new() -> Self {
        let mut cards = Vec::new();
        
        // First tier cards
        cards.push(Card::new(Cost::new(1, 1, 1, 1, 0), Resource::Black, 0, Tier::First));
        cards.push(Card::new(Cost::new(1, 1, 2, 1, 0), Resource::Black, 0, Tier::First));
        cards.push(Card::new(Cost::new(0, 1, 2, 2, 0), Resource::Black, 0, Tier::First));
        cards.push(Card::new(Cost::new(1, 3, 0, 0, 1), Resource::Black, 0, Tier::First));
        cards.push(Card::new(Cost::new(2, 1, 0, 0, 0), Resource::Black, 0, Tier::First));
        cards.push(Card::new(Cost::new(2, 0, 0, 2, 0), Resource::Black, 0, Tier::First));
        cards.push(Card::new(Cost::new(3, 0, 0, 0, 0), Resource::Black, 0, Tier::First));
        cards.push(Card::new(Cost::new(0, 0, 4, 0, 0), Resource::Black, 1, Tier::First));
        cards.push(Card::new(Cost::new(1, 1, 0, 1, 1), Resource::Blue, 0, Tier::First));
        cards.push(Card::new(Cost::new(1, 2, 0, 1, 1), Resource::Blue, 0, Tier::First));
        cards.push(Card::new(Cost::new(2, 2, 0, 1, 0), Resource::Blue, 0, Tier::First));
        cards.push(Card::new(Cost::new(3, 1, 1, 0, 0), Resource::Blue, 0, Tier::First));
        cards.push(Card::new(Cost::new(0, 0, 0, 1, 2), Resource::Blue, 0, Tier::First));
        cards.push(Card::new(Cost::new(2, 0, 0, 0, 2), Resource::Blue, 0, Tier::First));
        cards.push(Card::new(Cost::new(0, 0, 0, 0, 3), Resource::Blue, 0, Tier::First));
        cards.push(Card::new(Cost::new(0, 0, 0, 0, 4), Resource::Blue, 1, Tier::First));
        cards.push(Card::new(Cost::new(1, 1, 1, 0, 1), Resource::White, 0, Tier::First));
        cards.push(Card::new(Cost::new(2, 1, 1, 0, 1), Resource::White, 0, Tier::First));
        cards.push(Card::new(Cost::new(2, 0, 2, 0, 1), Resource::White, 0, Tier::First));
        cards.push(Card::new(Cost::new(0, 0, 1, 3, 1), Resource::White, 0, Tier::First));
        cards.push(Card::new(Cost::new(0, 2, 0, 0, 1), Resource::White, 0, Tier::First));
        cards.push(Card::new(Cost::new(0, 0, 0, 2, 2), Resource::White, 0, Tier::First));
        cards.push(Card::new(Cost::new(0, 0, 0, 3, 0), Resource::White, 0, Tier::First));
        cards.push(Card::new(Cost::new(4, 0, 0, 0, 0), Resource::White, 1, Tier::First));
        cards.push(Card::new(Cost::new(0, 1, 1, 1, 1), Resource::Green, 0, Tier::First));
        cards.push(Card::new(Cost::new(0, 1, 1, 1, 2), Resource::Green, 0, Tier::First));
        cards.push(Card::new(Cost::new(0, 2, 0, 1, 2), Resource::Green, 0, Tier::First));
        cards.push(Card::new(Cost::new(1, 0, 3, 1, 0), Resource::Green, 0, Tier::First));
        cards.push(Card::new(Cost::new(0, 0, 2, 1, 0), Resource::Green, 0, Tier::First));
        cards.push(Card::new(Cost::new(0, 2, 0, 2, 0), Resource::Green, 0, Tier::First));
        cards.push(Card::new(Cost::new(0, 3, 0, 0, 0), Resource::Green, 0, Tier::First));
        cards.push(Card::new(Cost::new(0, 0, 0, 0, 4), Resource::Green, 1, Tier::First));
        cards.push(Card::new(Cost::new(1, 0, 1, 1, 1), Resource::Red, 0, Tier::First));
        cards.push(Card::new(Cost::new(1, 0, 2, 1, 1), Resource::Red, 0, Tier::First));
        cards.push(Card::new(Cost::new(1, 0, 2, 0, 2), Resource::Red, 0, Tier::First));
        cards.push(Card::new(Cost::new(0, 1, 0, 0, 3), Resource::Red, 0, Tier::First));
        cards.push(Card::new(Cost::new(1, 0, 0, 2, 0), Resource::Red, 0, Tier::First));
        cards.push(Card::new(Cost::new(0, 2, 0, 0, 2), Resource::Red, 0, Tier::First));
        cards.push(Card::new(Cost::new(0, 0, 0, 3, 0), Resource::Red, 0, Tier::First));
        cards.push(Card::new(Cost::new(0, 0, 0, 4, 0), Resource::Red, 1, Tier::First));
        
        // Second tier cards
        cards.push(Card::new(Cost::new(2, 0, 2, 3, 0), Resource::Black, 1, Tier::Second));
        cards.push(Card::new(Cost::new(3, 0, 3, 0, 2), Resource::Black, 1, Tier::Second));
        cards.push(Card::new(Cost::new(4, 2, 1, 0, 0), Resource::Black, 2, Tier::Second));
        cards.push(Card::new(Cost::new(5, 3, 0, 0, 0), Resource::Black, 2, Tier::Second));
        cards.push(Card::new(Cost::new(0, 0, 0, 0, 6), Resource::Black, 3, Tier::Second));
        cards.push(Card::new(Cost::new(2, 3, 2, 0, 0), Resource::Blue, 1, Tier::Second));
        cards.push(Card::new(Cost::new(3, 0, 3, 2, 0), Resource::Blue, 1, Tier::Second));
        cards.push(Card::new(Cost::new(0, 0, 3, 5, 0), Resource::Blue, 2, Tier::Second));
        cards.push(Card::new(Cost::new(0, 1, 0, 2, 4), Resource::Blue, 2, Tier::Second));
        cards.push(Card::new(Cost::new(0, 0, 0, 5, 0), Resource::Blue, 2, Tier::Second));
        cards.push(Card::new(Cost::new(0, 0, 0, 6, 0), Resource::Blue, 3, Tier::Second));
        cards.push(Card::new(Cost::new(3, 2, 0, 0, 2), Resource::White, 1, Tier::Second));
        cards.push(Card::new(Cost::new(0, 3, 0, 2, 3), Resource::White, 1, Tier::Second));
        cards.push(Card::new(Cost::new(1, 4, 0, 0, 2), Resource::White, 2, Tier::Second));
        cards.push(Card::new(Cost::new(0, 5, 0, 0, 3), Resource::White, 2, Tier::Second));
        cards.push(Card::new(Cost::new(0, 5, 0, 0, 0), Resource::White, 2, Tier::Second));
        cards.push(Card::new(Cost::new(0, 0, 0, 6, 0), Resource::White, 3, Tier::Second));
        cards.push(Card::new(Cost::new(2, 3, 0, 0, 1), Resource::Green, 2, Tier::Second));
        cards.push(Card::new(Cost::new(3, 0, 3, 0, 0), Resource::Green, 2, Tier::Second));
        cards.push(Card::new(Cost::new(0, 0, 5, 0, 0), Resource::Green, 2, Tier::Second));
        cards.push(Card::new(Cost::new(0, 0, 6, 0, 0), Resource::Green, 3, Tier::Second));
        cards.push(Card::new(Cost::new(0, 2, 0, 2, 3), Resource::Red, 1, Tier::Second));
        cards.push(Card::new(Cost::new(0, 2, 0, 3, 3), Resource::Red, 1, Tier::Second));
        cards.push(Card::new(Cost::new(2, 0, 4, 1, 0), Resource::Red, 2, Tier::Second));
        cards.push(Card::new(Cost::new(0, 0, 0, 3, 5), Resource::Red, 2, Tier::Second));
        cards.push(Card::new(Cost::new(0, 0, 0, 0, 5), Resource::Red, 2, Tier::Second));
        cards.push(Card::new(Cost::new(0, 6, 0, 0, 0), Resource::Red, 3, Tier::Second));
        
        // Third tier cards
        cards.push(Card::new(Cost::new(5, 3, 3, 3, 0), Resource::Black, 3, Tier::Third));
        cards.push(Card::new(Cost::new(0, 7, 0, 0, 0), Resource::Black, 4, Tier::Third));
        cards.push(Card::new(Cost::new(3, 6, 0, 0, 3), Resource::Black, 4, Tier::Third));
        cards.push(Card::new(Cost::new(0, 7, 0, 0, 3), Resource::Black, 5, Tier::Third));
        cards.push(Card::new(Cost::new(3, 3, 3, 0, 5), Resource::Blue, 3, Tier::Third));
        cards.push(Card::new(Cost::new(0, 0, 0, 0, 7), Resource::Blue, 4, Tier::Third));
        cards.push(Card::new(Cost::new(0, 0, 3, 6, 3), Resource::Blue, 4, Tier::Third));
        cards.push(Card::new(Cost::new(0, 0, 0, 0, 7), Resource::Blue, 5, Tier::Third));
        cards.push(Card::new(Cost::new(3, 5, 3, 3, 0), Resource::White, 3, Tier::Third));
        cards.push(Card::new(Cost::new(0, 0, 0, 7, 0), Resource::White, 4, Tier::Third));
        cards.push(Card::new(Cost::new(3, 0, 3, 0, 6), Resource::White, 4, Tier::Third));
        cards.push(Card::new(Cost::new(3, 0, 0, 0, 7), Resource::White, 5, Tier::Third));
        cards.push(Card::new(Cost::new(0, 3, 3, 5, 3), Resource::Green, 3, Tier::Third));
        cards.push(Card::new(Cost::new(0, 0, 0, 7, 0), Resource::Green, 4, Tier::Third));
        cards.push(Card::new(Cost::new(3, 0, 3, 6, 0), Resource::Green, 4, Tier::Third));
        cards.push(Card::new(Cost::new(3, 0, 3, 7, 0), Resource::Green, 5, Tier::Third));
        cards.push(Card::new(Cost::new(3, 0, 3, 5, 3), Resource::Red, 3, Tier::Third));
        cards.push(Card::new(Cost::new(7, 0, 0, 0, 0), Resource::Red, 4, Tier::Third));
        cards.push(Card::new(Cost::new(6, 3, 0, 3, 0), Resource::Red, 4, Tier::Third));
        cards.push(Card::new(Cost::new(7, 3, 0, 0, 0), Resource::Red, 5, Tier::Third));
        
        Self { cards }
    }
    
    pub fn get_card(&self, index: usize) -> &Card {
        &self.cards[index]
    }
    
    pub fn get_first_tier_cards(&self) -> &[Card] {
        &self.cards[0..40]
    }
    
    pub fn get_second_tier_cards(&self) -> &[Card] {
        &self.cards[40..67]
    }
    
    pub fn get_third_tier_cards(&self) -> &[Card] {
        &self.cards[67..87]
    }
}

pub static CARD_STORAGE: Lazy<CardStorage> = Lazy::new(CardStorage::new);