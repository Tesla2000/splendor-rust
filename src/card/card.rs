use crate::card::cost::Cost;
use crate::card::tier::Tier;
use crate::resource::Resource;

pub struct Card {
    pub cost: Cost,
    pub production: Resource,
    pub n_points: u8,
    pub tier: Tier,
}

impl Card {
    pub(crate) fn new(
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