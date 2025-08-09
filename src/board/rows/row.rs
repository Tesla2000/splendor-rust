use crate::card::card::{Card, CardBuilder};
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
}

pub(crate) struct RowBuilder {
    visible: Vec<CardBuilder>,
    hidden: Vec<CardBuilder>,
}

impl RowBuilder {
    pub fn new(row: Row) -> Self {
        Self {
            visible: row.visible.into_iter().map(CardBuilder::new).collect(),
            hidden: row.hidden.into_iter().map(CardBuilder::new).collect(),
        }
    }

    pub fn build(self) -> Row {
        Row {
            visible: self.visible.into_iter().map(|b| b.build()).collect(),
            hidden: self.hidden.into_iter().map(|b| b.build()).collect(),
        }
    }

    // Getters
    pub fn get_visible(&self) -> &Vec<CardBuilder> {
        &self.visible
    }

    pub fn get_hidden(&self) -> &Vec<CardBuilder> {
        &self.hidden
    }

    // Setters
    pub fn set_visible(&mut self, visible: Vec<CardBuilder>) {
        self.visible = visible;
    }

    pub fn set_hidden(&mut self, hidden: Vec<CardBuilder>) {
        self.hidden = hidden;
    }
}