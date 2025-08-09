use crate::card::card::Card;
use crate::game_state::GameState;
use crate::moves::_give_player_resources::give_player_resources;
use crate::resources::Resources;

struct Reserve {
    row_index: usize,
    card_index: usize,
}

const RESERVE_RESOURCES: Resources = Resources::new(0, 0, 0, 0, 0, 1);
impl Reserve {
    fn new(row_index: usize, card_index: usize) -> Self {
        Self {
            row_index: row_index,
            card_index: card_index,
        }
    }

    pub fn is_valid(&self, game_state: &GameState) -> bool {
        game_state.get_board().get_resources().n_gold() > 0 && game_state.get_current_player().can_add_resources(&RESERVE_RESOURCES) && game_state.get_current_player().can_add_reserve()
    }

    pub fn perform(&self, game_state: &GameState) -> GameState {
        game_state.get_board().get_resources().pay(&self.resources);
        give_player_resources(&RESERVE_RESOURCES, game_state);
        give_player_resources(&RESERVE_RESOURCES, game_state)
    }
    
}