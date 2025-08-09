use crate::card::cost::Cost;
use crate::game_state::GameState;
use crate::moves::_give_player_resources::give_player_resources;
use crate::resource::Resource;
use crate::moves::move_trait::Move;

pub(crate) struct GetThree{
    resources: Cost
}

impl GetThree {
    pub fn new(resource1: Resource, resource2: Resource, resource3: Resource) -> Self {
        if (resource1 == resource2 || resource1 == resource3 || resource2 == resource3) {
            panic!("Resources must be unique.");
        }
        let mut green: u8 = 0;
        if Resource::Green == resource1 || Resource::Green == resource2 || Resource::Green == resource3 {
            green = 1;
        }
        let mut blue: u8 = 0;
        if Resource::Blue == resource1 || Resource::Blue == resource2 || Resource::Blue == resource3 {
            blue = 1;
        }
        let mut red: u8 = 0;
        if Resource::Red == resource1 || Resource::Red == resource2 || Resource::Red == resource3 {
            red = 1;
        }
        let mut black: u8 = 0;
        if Resource::Black == resource1 || Resource::Black == resource2 || Resource::Black == resource3 {
            black = 1;
        }
        let mut white: u8 = 0;
        if Resource::White == resource1 || Resource::White == resource2 || Resource::White == resource3 {
            white = 1;
        }
        Self {
            resources: Cost::new(green, red, blue, white, black),
        }
    }

}

impl Move for GetThree {
    fn is_valid(&self, game_state: &GameState) -> bool {
        game_state.get_board().get_resources().can_pay(&self.resources) && game_state.get_current_player().can_add_resources(&self.resources.to_resources())
    }

    fn perform(&self, game_state: &GameState) -> GameState {
        give_player_resources(&self.resources, game_state)
    }
}
