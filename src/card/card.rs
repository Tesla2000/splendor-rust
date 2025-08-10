use crate::card::cost::Cost;
use crate::card::tier::Tier;
use crate::resource::Resource;

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

