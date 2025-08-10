use rand::prelude::{SliceRandom, ThreadRng};
use crate::board::rows::card_reference::CardReference;
use crate::board::rows::row::Row;
use crate::card::card::Card;
use crate::card::card_storage::CARD_STORAGE;

#[derive(Clone)]
pub(crate) struct Rows {
    rows: (Row, Row, Row),
}

impl Rows {
    pub(crate) fn new(rng: &mut ThreadRng) -> Rows {
        // Create indices for each tier
        let mut first_tier_indices: Vec<usize> = (0..40).collect();
        let mut second_tier_indices: Vec<usize> = (40..67).collect();
        let mut third_tier_indices: Vec<usize> = (67..87).collect();
        
        // Shuffle the indices
        first_tier_indices.shuffle(rng);
        second_tier_indices.shuffle(rng);
        third_tier_indices.shuffle(rng);
        
        Rows {
            rows: (
                Row::new(first_tier_indices),
                Row::new(second_tier_indices),
                Row::new(third_tier_indices),
            ),
        }
    }
    
    pub fn get_row(&self, index: u8) -> &Row {
        match index {
            0 => {&self.rows.0}
            1 => {&self.rows.1}
            2 => {&self.rows.2}
            _ => {panic!("Invalid index");}
        }
    }
    
    pub fn replace_row(&self, updated_row: Row, updated_row_index: u8) -> Self {
        match updated_row_index { 
            0 => {Self {rows: (updated_row, self.rows.1.clone(), self.rows.2.clone())}}
            1 => {Self {rows: (self.rows.0.clone(), updated_row, self.rows.2.clone())}}
            2 => {Self {rows: (self.rows.0.clone(), self.rows.1.clone(), updated_row)}}
            _ => {panic!("Invalid index");} 
        }
    }
    
    pub fn to_builder(&self) -> RowsBuilder {
        RowsBuilder::new(self)
    }
}

pub(crate) struct RowsBuilder {
    pub rows: (crate::board::rows::row::RowBuilder, crate::board::rows::row::RowBuilder, crate::board::rows::row::RowBuilder),
}

impl RowsBuilder {
    pub(crate) fn new(rows: &Rows) -> Self {
        Self {
            rows: (
                rows.rows.0.to_builder(),
                rows.rows.1.to_builder(),
                rows.rows.2.to_builder(),
            ),
        }
    }
    
    pub fn get(&mut self, index: u8) -> &mut crate::board::rows::row::RowBuilder {
        match index {
             0 => {&mut self.rows.0}
             1 => {&mut self.rows.1}
             2 => {&mut self.rows.2}
            _ => {panic!("Invalid index");}
        }
    }
    
    pub fn replace(&mut self, updated_row: crate::board::rows::row::RowBuilder, updated_row_index: u8) {
        match updated_row_index {
            0 => {self.rows.0 = updated_row;}
            1 => {self.rows.1 = updated_row;}
            2 => {self.rows.2 = updated_row;}
            _ => {panic!("Invalid index");}
        }
    }

    pub fn build(self) -> Rows {
        Rows {
            rows: (
                self.rows.0.build(),
                self.rows.1.build(),
                self.rows.2.build(),
            ),
        }
    }
}