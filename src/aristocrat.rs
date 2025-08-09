use crate::card::cost::Cost;
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
    
    pub fn to_builder(&self) -> AristocratBuilder {
        AristocratBuilder::new(self)
    }
}

pub(crate) struct AristocratBuilder {
    pub cost: crate::card::cost::CostBuilder,
}

impl AristocratBuilder {
    fn new(aristocrat: &Aristocrat) -> Self {
        Self {
            cost: aristocrat.cost.to_builder(),
        }
    }

    pub fn build(self) -> Aristocrat {
        Aristocrat {
            cost: self.cost.build(),
        }
    }
}