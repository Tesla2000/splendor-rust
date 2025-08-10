use rand::prelude::{SliceRandom, ThreadRng};
use crate::board::rows::row::Row;
use crate::card::tier::Tier;
use crate::card::card_storage::CARD_STORAGE;
use std::collections::HashMap;

#[derive(Clone)]
pub(crate) struct Rows {
    rows: HashMap<Tier, Row>,
}

impl Rows {
    pub(crate) fn new(rng: &mut ThreadRng) -> Rows {
        let mut rows = HashMap::new();
        
        // Get indices for each tier and shuffle them
        for tier in [Tier::First, Tier::Second, Tier::Third] {
            let mut tier_indices = CARD_STORAGE.get_tier_indices(tier);
            tier_indices.shuffle(rng);
            rows.insert(tier, Row::new(tier_indices));
        }
        
        Rows { rows }
    }
    
    pub fn get_row(&self, index: u8) -> &Row {
        match index {
            0 => self.rows.get(&Tier::First).expect("First tier row not found"),
            1 => self.rows.get(&Tier::Second).expect("Second tier row not found"),
            2 => self.rows.get(&Tier::Third).expect("Third tier row not found"),
            _ => panic!("Invalid index"),
        }
    }
    
    pub fn to_builder(&self) -> RowsBuilder {
        RowsBuilder::new(self)
    }
}

pub(crate) struct RowsBuilder {
    pub rows: HashMap<Tier, crate::board::rows::row::RowBuilder>,
}

impl RowsBuilder {
    pub(crate) fn new(rows: &Rows) -> Self {
        let mut builder_rows = HashMap::new();
        for (tier, row) in &rows.rows {
            builder_rows.insert(*tier, row.to_builder());
        }
        Self { rows: builder_rows }
    }
    
    pub fn get(&mut self, index: u8) -> &mut crate::board::rows::row::RowBuilder {
        match index {
            0 => self.rows.get_mut(&Tier::First).expect("First tier row not found"),
            1 => self.rows.get_mut(&Tier::Second).expect("Second tier row not found"),
            2 => self.rows.get_mut(&Tier::Third).expect("Third tier row not found"),
            _ => panic!("Invalid index"),
        }
    }

    pub fn build(self) -> Rows {
        let mut rows = HashMap::new();
        for (tier, row_builder) in self.rows {
            rows.insert(tier, row_builder.build());
        }
        Rows { rows }
    }
}