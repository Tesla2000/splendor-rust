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

    pub fn has_card(&self, index: usize) -> bool {
        index < self.visible.len()
    }

    pub fn get_card(&self, index: usize) -> &Card {
        &self.visible[index]
    }
    
    pub fn to_builder(&self) -> RowBuilder {
        RowBuilder::new(self)
    }
    
    pub fn get_hidden(&self) -> &Vec<Card> {
        &self.hidden
    }
}

pub(crate) struct RowBuilder {
    visible: Vec<Card>,
    hidden: Vec<Card>,
}

impl RowBuilder {
    pub(crate) fn new(row: &Row) -> Self {
        Self {
            visible: row.visible.clone(),
            hidden: row.hidden.clone(),
        }
    }

    pub fn remove(&mut self, index: usize) -> Card {
        let card = self.visible.remove(index);
        if self.hidden.is_empty() {
            self.visible.pop();
        } else {
            self.visible.push(self.hidden.remove(0));
        }
        card
    }

    pub fn build(self) -> Row {
        Row {
            visible: self.visible,
            hidden: self.hidden,
        }
    }
}