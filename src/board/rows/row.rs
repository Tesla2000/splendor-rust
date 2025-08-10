use crate::card::card::Card;
use crate::card::card_storage::CARD_STORAGE;

const CARD_COUNT: usize = 4;

#[derive(Clone)]
pub(crate) struct Row {
    visible: Vec<usize>,  // Indices into CARD_STORAGE
    hidden: Vec<usize>,   // Indices into CARD_STORAGE
}

impl Row {
    pub(crate) fn new(mut card_indices: Vec<usize>) -> Row {
        let visible: Vec<usize> = card_indices.drain(0..CARD_COUNT.min(card_indices.len())).collect();
        Row {
            visible,
            hidden: card_indices,
        }
    }

    pub fn has_card(&self, index: usize) -> bool {
        index < self.visible.len()
    }

    pub fn get_card(&self, index: usize) -> &'static Card {
        CARD_STORAGE.get_card(self.visible[index])
    }
    
    pub fn to_builder(&self) -> RowBuilder {
        RowBuilder::new(self)
    }
    
    pub fn get_hidden(&self) -> Vec<&'static Card> {
        self.hidden.iter().map(|&i| CARD_STORAGE.get_card(i)).collect()
    }
}

pub(crate) struct RowBuilder {
    visible: Vec<usize>,
    hidden: Vec<usize>,
}

impl RowBuilder {
    pub(crate) fn new(row: &Row) -> Self {
        Self {
            visible: row.visible.clone(),
            hidden: row.hidden.clone(),
        }
    }

    pub fn remove(&mut self, index: usize) -> &'static Card {
        let card_index = self.visible.remove(index);
        if self.hidden.is_empty() {
            if !self.visible.is_empty() {
                self.visible.pop();
            }
        } else {
            self.visible.push(self.hidden.remove(0));
        }
        CARD_STORAGE.get_card(card_index)
    }
    
    pub fn remove_from_hidden(&mut self) -> &'static Card {
        if self.hidden.is_empty() {
            panic!("No hidden cards to remove");
        }
        let card_index = self.hidden.remove(0);
        CARD_STORAGE.get_card(card_index)
    }

    pub fn build(self) -> Row {
        Row {
            visible: self.visible,
            hidden: self.hidden,
        }
    }
}