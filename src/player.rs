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
    pub fn get_resources(&self) -> &Resources {
        &self.resources
    }
    pub fn get_reserve(&self) -> &Vec<Card> {
        &self.reserve
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
    pub deck: Vec<Card>,
    pub resources: crate::resources::ResourcesBuilder,
    pub reserve: Vec<Card>,
    pub aristocrats: Vec<Aristocrat>,
}

impl PlayerBuilder {
    fn new(player: &Player) -> Self {
        Self {
            deck: player.deck.clone(),
            resources: player.resources.to_builder(),
            reserve: player.reserve.clone(),
            aristocrats: player.aristocrats.clone(),
        }
    }

    fn get_production(&self) -> Resources {
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
    
    pub fn pay_for_card(&mut self, card: &Card) {
        let production = self.get_production();
        let remaining_cost = Cost::new(
            card.cost.n_green().saturating_sub(production.n_green()),
            card.cost.n_red().saturating_sub(production.n_red()),
            card.cost.n_blue().saturating_sub(production.n_blue()),
            card.cost.n_white().saturating_sub(production.n_white()),
            card.cost.n_black().saturating_sub(production.n_black()),
        );
        self.resources.pay_cost(&remaining_cost)
    }

    pub fn add_resources(&mut self, resources: &ResourcesBuilder) {
        self.resources.add(resources)
    }
    
    pub fn build(self) -> Player {
        Player {
            deck: self.deck,
            resources: self.resources.build(),
            reserve: self.reserve,
            aristocrats: self.aristocrats,
        }
    }
}