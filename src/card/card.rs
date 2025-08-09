use crate::card::cost::{Cost, CostBuilder};
use crate::card::tier::Tier;
use crate::resource::Resource;

#[derive(Clone)]
pub struct Card {
    pub cost: Cost,
    pub production: Resource,
    pub n_points: u8,
    pub tier: Tier,
}

impl Card {
    pub(crate) const  fn new(
        cost: Cost,
        production: Resource,
        n_points: u8,
        tier: Tier,
    ) -> Self {
        Self {
            cost,
            production,
            n_points,
            tier,
        }
    }
}

const NULL_CARD: Card = Card::new(Cost::new(0, 0, 0, 0, 0), Resource::Green, 0, Tier::First);

pub struct CardBuilder {
    cost: CostBuilder,
    production: Resource,
    n_points: u8,
    tier: Tier,
}

impl CardBuilder {
    pub fn new(card: Card) -> Self {
        Self {
            cost: CostBuilder::new(card.cost),
            production: card.production,
            n_points: card.n_points,
            tier: card.tier,
        }
    }

    pub fn build(self) -> Card {
        Card {
            cost: self.cost.build(),
            production: self.production,
            n_points: self.n_points,
            tier: self.tier,
        }
    }

    // Getters
    pub fn get_cost(&self) -> &CostBuilder {
        &self.cost
    }

    pub fn get_production(&self) -> &Resource {
        &self.production
    }

    pub fn get_n_points(&self) -> u8 {
        self.n_points
    }

    pub fn get_tier(&self) -> &Tier {
        &self.tier
    }

    // Setters
    pub fn set_cost(&mut self, cost: CostBuilder) {
        self.cost = cost;
    }

    pub fn set_production(&mut self, production: Resource) {
        self.production = production;
    }

    pub fn set_n_points(&mut self, n_points: u8) {
        self.n_points = n_points;
    }

    pub fn set_tier(&mut self, tier: Tier) {
        self.tier = tier;
    }
}
