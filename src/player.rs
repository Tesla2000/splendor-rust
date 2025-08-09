use crate::aristocrat::{Aristocrat, ARISTOCRAT_POINTS};
use crate::card::card::Card;
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

    pub fn can_add_resources(&self, resources: &Resources) -> bool {
        self.resources.sum() + resources.sum() <= 10
    }

    pub fn can_add_reserve(&self) -> bool {
        self.reserve.len() < MAX_RESERVE_CARDS
    }

    pub fn add_reserve(&self, card: &Card) -> Self {
        let mut reserve = self.reserve.clone();
        reserve.push(card.clone());
        Self {
            deck: self.deck.clone(),
            resources: self.resources.clone(),
            reserve: reserve,
            aristocrats: self.aristocrats.clone(),
        }
    }
    
    pub fn to_builder(&self) -> PlayerBuilder {
        PlayerBuilder::new(self)
    }
}

pub(crate) struct PlayerBuilder {
    pub deck: Vec<crate::card::card::CardBuilder>,
    pub resources: crate::resources::ResourcesBuilder,
    pub reserve: Vec<crate::card::card::CardBuilder>,
    pub aristocrats: Vec<crate::aristocrat::AristocratBuilder>,
}

impl PlayerBuilder {
    fn new(player: &Player) -> Self {
        Self {
            deck: player.deck.iter().map(|c| c.to_builder()).collect(),
            resources: player.resources.to_builder(),
            reserve: player.reserve.iter().map(|c| c.to_builder()).collect(),
            aristocrats: player.aristocrats.iter().map(|a| a.to_builder()).collect(),
        }
    }

    pub fn add_resources(&mut self, resources: &ResourcesBuilder) {
        self.resources.add(resources)
    }
    
    pub fn build(self) -> Player {
        Player {
            deck: self.deck.into_iter().map(|b| b.build()).collect(),
            resources: self.resources.build(),
            reserve: self.reserve.into_iter().map(|b| b.build()).collect(),
            aristocrats: self.aristocrats.into_iter().map(|b| b.build()).collect(),
        }
    }
}