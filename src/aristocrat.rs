use crate::card::cost::{Cost, CostBuilder};
use crate::player::Player;
use crate::resources::Resources;

pub const ARISTOCRAT_POINTS: u8 = 3;
#[derive(Clone)]
pub struct Aristocrat {
    cost: Cost,
}
impl Aristocrat {
    pub fn new(resources: Cost) -> Self {
        Self {
            cost: resources
        }
    }
    pub fn can_be_taken_by(&self, player: &Player) -> bool {
        player.get_production().can_pay(&self.cost)
    }
}

pub struct AristocratBuilder {
    cost: CostBuilder,
}

impl AristocratBuilder {
    pub fn new(aristocrat: Aristocrat) -> Self {
        Self {
            cost: CostBuilder::new(aristocrat.cost),
        }
    }

    pub fn build(self) -> Aristocrat {
        Aristocrat {
            cost: self.cost.build(),
        }
    }

    // Getters
    pub fn get_cost(&self) -> &CostBuilder {
        &self.cost
    }

    // Setters
    pub fn set_cost(&mut self, cost: CostBuilder) {
        self.cost = cost;
    }
}