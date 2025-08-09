use crate::card::cost::Cost;
use crate::game_state::GameState;
use crate::moves::_give_player_resources::give_player_resources;
use crate::resource::Resource;
use crate::moves::move_trait::Move;

pub(crate) struct GetTwo{
    resources: Cost,
    resource: Resource
}

impl GetTwo {
    pub fn new(resource: Resource) -> Self {
        match resource {
            Resource::Green => {Self {
                resources: Cost::new(2, 0, 0, 0, 0),
                resource: resource
            }}
            Resource::Red => {Self {
                resources: Cost::new(0, 2, 0, 0, 0),
                resource: resource
            }}
            Resource::Blue => {
                Self {
                    resources: Cost::new(0, 0, 2, 0, 0),
                    resource: resource
                }}
            Resource::White => {
                Self {
                    resources: Cost::new(0, 0, 0, 2, 0),
                    resource: resource
                }}
            Resource::Black => {            Self {
                resources: Cost::new(0, 0, 0, 0, 2),
                resource: resource
            }}
        }
    }
    
}

impl Move for GetTwo {
    fn is_valid(&self, game_state: &GameState) -> bool {
        let at_least_four: bool;
        match self.resource {
            Resource::Green => {at_least_four = game_state.get_board().get_resources().n_green() >= 4;}
            Resource::Red => {at_least_four = game_state.get_board().get_resources().n_red() >= 4;}
            Resource::Blue => {at_least_four = game_state.get_board().get_resources().n_blue() >= 4;}
            Resource::White => {at_least_four = game_state.get_board().get_resources().n_white() >= 4;}
            Resource::Black => {at_least_four = game_state.get_board().get_resources().n_black() >= 4;}
        }
        at_least_four && game_state.get_current_player().can_add_resources(&self.resources.to_resources())
    }

    fn perform(&self, game_state: &GameState) -> GameState {
        self.finalize(give_player_resources(&self.resources, game_state))
    }
}
