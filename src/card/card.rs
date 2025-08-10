use crate::card::cost::Cost;
use crate::card::tier::Tier;
use crate::resource::Resource;

#[derive(Clone)]
pub struct Card {
    cost: Cost,
    production: Resource,
    n_points: u8,
    tier: Tier,
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
    
    pub fn cost(&self) -> &Cost {
        &self.cost
    }
    
    pub fn production(&self) -> Resource {
        self.production
    }
    
    pub fn n_points(&self) -> u8 {
        self.n_points
    }
    
    pub fn tier(&self) -> Tier {
        self.tier
    }
}

const NULL_CARD: Card = Card::new(Cost::new(0, 0, 0, 0, 0), Resource::Green, 0, Tier::First);
