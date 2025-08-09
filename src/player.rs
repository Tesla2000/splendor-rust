use crate::aristocrat::{Aristocrat, AristocratBuilder, ARISTOCRAT_POINTS};
use crate::card::card::{Card, CardBuilder};
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
}

pub struct PlayerBuilder {
    deck: Vec<CardBuilder>,
    resources: ResourcesBuilder,
    reserve: Vec<CardBuilder>,
    aristocrats: Vec<AristocratBuilder>,
}

impl PlayerBuilder {
    pub fn new(player: Player) -> Self {
        Self {
            deck: player.deck.into_iter().map(CardBuilder::new).collect(),
            resources: ResourcesBuilder::new(player.resources),
            reserve: player.reserve.into_iter().map(CardBuilder::new).collect(),
            aristocrats: player.aristocrats.into_iter().map(AristocratBuilder::new).collect(),
        }
    }

    pub fn build(self) -> Player {
        Player {
            deck: self.deck.into_iter().map(|b| b.build()).collect(),
            resources: self.resources.build(),
            reserve: self.reserve.into_iter().map(|b| b.build()).collect(),
            aristocrats: self.aristocrats.into_iter().map(|b| b.build()).collect(),
        }
    }

    // Getters
    pub fn get_deck(&self) -> &Vec<CardBuilder> {
        &self.deck
    }

    pub fn get_resources(&self) -> &ResourcesBuilder {
        &self.resources
    }

    pub fn get_reserve(&self) -> &Vec<CardBuilder> {
        &self.reserve
    }

    pub fn get_aristocrats(&self) -> &Vec<AristocratBuilder> {
        &self.aristocrats
    }

    // Setters
    pub fn set_deck(&mut self, deck: Vec<CardBuilder>) {
        self.deck = deck;
    }

    pub fn set_resources(&mut self, resources: ResourcesBuilder) {
        self.resources = resources;
    }

    pub fn set_reserve(&mut self, reserve: Vec<CardBuilder>) {
        self.reserve = reserve;
    }

    pub fn set_aristocrats(&mut self, aristocrats: Vec<AristocratBuilder>) {
        self.aristocrats = aristocrats;
    }
}