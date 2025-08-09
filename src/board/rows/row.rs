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
        self.visible.len() < index
    }

    pub fn take_card(&self, index: usize) -> Self {
        let mut visible = self.visible.clone();
        let mut hidden = self.hidden.clone();
        visible.remove(index);
        visible.push(hidden.remove(0));
        Self{
            visible,
            hidden,       
        }
        
    }

    pub fn get_card(&self, index: usize) -> &Card {
        &self.visible[index]
    }
    
    pub fn to_builder(&self) -> RowBuilder {
        RowBuilder::new(self)
    }
}

pub(crate) struct RowBuilder {
    pub visible: Vec<crate::card::card::CardBuilder>,
    pub hidden: Vec<crate::card::card::CardBuilder>,
}

impl RowBuilder {
    pub(crate) fn new(row: &Row) -> Self {
        Self {
            visible: row.visible.iter().map(|c| c.to_builder()).collect(),
            hidden: row.hidden.iter().map(|c| c.to_builder()).collect(),
        }
    }

    pub fn build(self) -> Row {
        Row {
            visible: self.visible.into_iter().map(|b| b.build()).collect(),
            hidden: self.hidden.into_iter().map(|b| b.build()).collect(),
        }
    }
}