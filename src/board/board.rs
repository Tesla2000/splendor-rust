use crate::aristocrat::{Aristocrat, AristocratBuilder};
use crate::board::rows::rows::{Rows, RowsBuilder};
use crate::card::cost::Cost;
use crate::resources::{Resources, ResourcesBuilder};
use rand::prelude::{SliceRandom, ThreadRng};
use crate::board::rows::card_reference::CardReference;

#[derive(Clone)]
pub struct Board {
    resources: Resources,
    rows: Rows,
    aristocrats: Vec<Aristocrat>,
}
const INITIAL_GOLD: u8 = 5;

impl Board {
    pub fn new(n_aristocrats: usize, rng: &mut ThreadRng) -> Self {
        let n_resources: u8;
        match n_aristocrats {
            2 => {n_resources=4;}
            3 => {n_resources=5;}
            4 => {n_resources=7;}
            _ => {panic!("N aristocrats must be 2, 3 or 4.");}
        }
        let mut aristocrats = vec![
            Aristocrat::new(Cost::new(0, 4, 0, 0, 4)),
            Aristocrat::new(Cost::new(0, 3, 0, 3, 3)),
            Aristocrat::new(Cost::new(0, 0, 4, 4, 0)),
            Aristocrat::new(Cost::new(0, 0, 0, 4, 4)),
            Aristocrat::new(Cost::new(0, 0, 4, 0, 4)),
            Aristocrat::new(Cost::new(0, 3, 3, 3, 0)),
            Aristocrat::new(Cost::new(0, 3, 3, 3, 3)),
            Aristocrat::new(Cost::new(0, 0, 0, 4, 4)),
            Aristocrat::new(Cost::new(0, 0, 3, 3, 3)),
            Aristocrat::new(Cost::new(0, 3, 3, 0, 3)),
        ];
        aristocrats.shuffle(rng);

        Self {
            resources: Resources::new(n_resources, n_resources, n_resources, n_resources, n_resources, INITIAL_GOLD),
            rows: Rows::new(rng),
            aristocrats: aristocrats.drain(0..n_aristocrats).collect(),
        }
    }
    
    pub fn get_rows(&self) -> &Rows {
        &self.rows
    }
    
    pub fn get_aristocrats(&self) -> &Vec<Aristocrat> {
        &self.aristocrats
    }
    
    pub fn get_resources(&self) -> &Resources {
        &self.resources
    }
    
    pub fn take_card(&self, card_reference: &CardReference) -> Self {
        let updated_row = self.rows.get_row(card_reference.get_row_index()).take_card(card_reference.get_card_index());
        let updated_rows: Rows = self.rows.replace_row(updated_row, card_reference.get_row_index());
        Self {
            resources: self.resources.clone(),
            rows: updated_rows,
            aristocrats: self.aristocrats.clone(),       
        }
    }
}

pub struct BoardBuilder {
    resources: ResourcesBuilder,
    rows: RowsBuilder,
    aristocrats: Vec<AristocratBuilder>,
}

impl BoardBuilder {
    pub fn new(board: Board) -> Self {
        Self {
            resources: ResourcesBuilder::new(board.resources),
            rows: RowsBuilder::new(board.rows),
            aristocrats: board.aristocrats.into_iter().map(AristocratBuilder::new).collect(),
        }
    }

    pub fn build(self) -> Board {
        Board {
            resources: self.resources.build(),
            rows: self.rows.build(),
            aristocrats: self.aristocrats.into_iter().map(|b| b.build()).collect(),
        }
    }

    // Getters
    pub fn get_resources(&self) -> &ResourcesBuilder {
        &self.resources
    }

    pub fn get_rows(&self) -> &RowsBuilder {
        &self.rows
    }

    pub fn get_aristocrats(&self) -> &Vec<AristocratBuilder> {
        &self.aristocrats
    }

    // Setters
    pub fn set_resources(&mut self, resources: ResourcesBuilder) {
        self.resources = resources;
    }

    pub fn set_rows(&mut self, rows: RowsBuilder) {
        self.rows = rows;
    }

    pub fn set_aristocrats(&mut self, aristocrats: Vec<AristocratBuilder>) {
        self.aristocrats = aristocrats;
    }
}