use crate::card::cost::Cost;
use crate::game_state::GameState;
use crate::moves::_give_player_resources::give_player_resources;
use crate::resource::Resource;

struct GetTwo{
    resources: Cost
}

impl GetTwo {
    pub fn new(resource: Resource) -> Self {
        if resource == Resource::Green {
            Self {
                resources: Cost::new(2, 0, 0, 0, 0)
            };
        }
        if resource == Resource::Red {
            Self {
                resources: Cost::new(0, 2, 0, 0, 0)
            };
        }
        if resource == Resource::Blue {
            Self {
                resources: Cost::new(0, 0, 2, 0, 0)
            };
        }
        if resource == Resource::White {
            Self {
                resources: Cost::new(0, 0, 0, 2, 0)
            };
        }
        if resource == Resource::Black {
            Self {
                resources: Cost::new(0, 0, 0, 0, 2)
            };
        }
        panic!("Invalid resource")
        
    }
    
    pub fn is_valid(&self, game_state: &GameState) -> bool {
        game_state.get_board().get_resources().pay_cost(&self.resources).can_pay(&self.resources) && game_state.get_current_player().can_add_resources(&self.resources.to_resources())
    }

    pub fn perform(&self, game_state: &GameState) -> GameState {
        game_state.get_board().get_resources().pay_cost(&self.resources);
        give_player_resources(&self.resources.to_resources(), game_state)
    }
}
