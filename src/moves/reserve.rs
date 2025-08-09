use crate::card::card::Card;
use crate::game_state::GameState;
use crate::moves::_give_player_resources::give_player_resources;
use crate::resources::Resources;
use crate::board::rows::card_reference::CardReference;
use crate::moves::_add_player_reserve::add_player_reserve;

struct Reserve {
    card_reference: CardReference,
}
pub(crate) const RESERVE_RESOURCES: Resources = Resources::new(0, 0, 0, 0, 0, 1);

impl Reserve {
    fn new(card_reference: CardReference) -> Self {
        Self {
            card_reference: card_reference,
        }
    }

    pub fn is_valid(&self, game_state: &GameState) -> bool {
        game_state.get_board().get_resources().n_gold() > 0 && game_state.get_current_player().can_add_resources(&RESERVE_RESOURCES) && game_state.get_current_player().can_add_reserve() && self.card_reference.is_in_board(game_state.get_board())
    }

    pub fn perform(&self, game_state: &GameState) -> GameState {
        add_player_reserve(&self.card_reference, game_state);
        
    }
    
}