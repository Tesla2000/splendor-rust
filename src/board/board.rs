use crate::aristocrat::Aristocrat;
use crate::aristocrat_storage::ARISTOCRAT_STORAGE;
use crate::board::rows::rows::Rows;
use crate::resources::Resources;
use rand::prelude::{SliceRandom, ThreadRng};
use crate::board::rows::card_reference::CardReference;

#[derive(Clone)]
pub struct Board {
    resources: Resources,
    rows: Rows,
    aristocrats: Vec<usize>,  // Indices into ARISTOCRAT_STORAGE
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
        let mut aristocrat_indices: Vec<usize> = (0..ARISTOCRAT_STORAGE.len()).collect();
        aristocrat_indices.shuffle(rng);

        Self {
            resources: Resources::new(n_resources, n_resources, n_resources, n_resources, n_resources, INITIAL_GOLD),
            rows: Rows::new(rng),
            aristocrats: aristocrat_indices.drain(0..n_aristocrats).collect(),
        }
    }
    
    pub fn get_rows(&self) -> &Rows {
        &self.rows
    }
    
    pub fn get_aristocrats(&self) -> Vec<&'static Aristocrat> {
        self.aristocrats.iter().map(|&i| ARISTOCRAT_STORAGE.get_aristocrat(i)).collect()
    }
    
    pub fn get_resources(&self) -> &Resources {
        &self.resources
    }
    

    pub fn to_builder(&self) -> BoardBuilder {
        BoardBuilder::new(self)
    }
}

pub(crate) struct BoardBuilder {
    pub resources: crate::resources::ResourcesBuilder,
    pub rows: crate::board::rows::rows::RowsBuilder,
    pub aristocrats: Vec<usize>,
}

impl BoardBuilder {
    fn new(board: &Board) -> Self {
        Self {
            resources: board.resources.to_builder(),
            rows: board.rows.to_builder(),
            aristocrats: board.aristocrats.clone(),
        }
    }

    pub fn build(self) -> Board {
        Board {
            resources: self.resources.build(),
            rows: self.rows.build(),
            aristocrats: self.aristocrats,
        }
    }
}