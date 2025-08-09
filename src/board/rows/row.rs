use crate::card::card::Card;
use rand::seq::SliceRandom;
use rand::rngs::ThreadRng;

const CARD_COUNT: usize = 4;
#[derive(Clone)]
pub(crate) struct Row {
    visible: Vec<Card>,
    hidden: Vec<Card>,
}

impl Row {
    pub(crate) fn new(mut cards: Vec<Card>, rng: &mut ThreadRng) -> Row {
        cards.shuffle(rng);
        let visible: Vec<Card> = cards.drain(0..CARD_COUNT).collect();
        Row {
            visible,
            hidden: cards,
        }
    }
}