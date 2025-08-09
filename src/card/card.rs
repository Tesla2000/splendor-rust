use crate::card::cost::Cost;
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
