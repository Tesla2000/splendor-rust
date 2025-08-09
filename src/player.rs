use crate::aristocrat::{Aristocrat, ARISTOCRAT_POINTS};
use crate::card::card::Card;
use crate::card::cost::Cost;
use crate::resource::Resource;
use crate::resources::{Resources, ResourcesBuilder};

#[derive(Clone)]
pub struct Player {
    deck: Vec<Card>,
    resources: Resources,
    reserve: Vec<Card>,
    aristocrats: Vec<Aristocrat>,
}

const MAX_RESERVE_CARDS: usize = 3;
impl Player {
    pub fn new() -> Self {
        Self {
            deck: Vec::new(),
            resources: Resources::new(0, 0, 0, 0, 0, 0),
            reserve: Vec::new(),
            aristocrats: Vec::new(),
        }
    }
    pub fn get_production(&self) -> Resources {
        let mut resources_builder = ResourcesBuilder::default();
        for card in &self.deck {
            match card.production {
                Resource::Green => { resources_builder.n_green += 1}
                Resource::Blue => { resources_builder.n_blue += 1}
                Resource::Red => { resources_builder.n_red += 1}
                Resource::White => { resources_builder.n_white += 1}
                Resource::Black => { resources_builder.n_black += 1}
            }
        }
        Resources::new(
            resources_builder.n_green,
            resources_builder.n_blue,
            resources_builder.n_red,
            resources_builder.n_white,
            resources_builder.n_black,
            0,
        )
    }
    
    pub fn get_points(&self) -> u8 {
        let aristocrat_points = ARISTOCRAT_POINTS * self.aristocrats.len() as u8;
        let mut card_points: u8 = 0;
        for card in &self.deck {
            card_points += card.n_points;
        }
        aristocrat_points + card_points
    }

    pub fn add_resources(&self, resources: &Resources) -> Self {
        Self {
            deck: self.deck.clone(),
            resources: self.resources.add(resources),
            reserve: self.reserve.clone(),
            aristocrats: self.aristocrats.clone(),
        }
    }

    pub fn can_add_resources(&self, resources: &Resources) -> bool {
        self.resources.sum() + resources.sum() <= 10
    }

    pub fn can_add_reserve(&self) -> bool {
        self.reserve.len() < MAX_RESERVE_CARDS
    }
}