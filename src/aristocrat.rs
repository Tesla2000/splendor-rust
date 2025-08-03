use crate::card::cost::Cost;
use crate::resources::Resources;

pub const ARISTOCRAT_POINTS: u8 = 3;
pub struct Aristocrat {
    cost: Cost,
}
impl Aristocrat {
    pub fn new(resources: Cost) -> Self {
        Self {
            cost: resources
        }
    }
}