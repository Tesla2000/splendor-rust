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
}