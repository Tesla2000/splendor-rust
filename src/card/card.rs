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
    
    pub fn to_builder(&self) -> CardBuilder {
        CardBuilder::new(self)
    }
}

const NULL_CARD: Card = Card::new(Cost::new(0, 0, 0, 0, 0), Resource::Green, 0, Tier::First);

pub(crate) struct CardBuilder {
    pub cost: crate::card::cost::CostBuilder,
    pub production: Resource,
    pub n_points: u8,
    pub tier: Tier,
}

impl CardBuilder {
    fn new(card: &Card) -> Self {
        Self {
            cost: card.cost.to_builder(),
            production: card.production.clone(),
            n_points: card.n_points,
            tier: card.tier.clone(),
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
}
