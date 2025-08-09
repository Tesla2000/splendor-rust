use crate::board::rows::card_reference::CardReference;
use crate::game_state::{GameState, GameStateBuilder};
use crate::resources::Resources;
use crate::moves::move_trait::Move;

pub(crate) struct Reserve {
    card_reference: CardReference,
}
pub(crate) const RESERVE_RESOURCES: Resources = Resources::new(0, 0, 0, 0, 0, 1);

impl Reserve {
    pub(crate) fn new(card_reference: CardReference) -> Self {
        Self {
            card_reference: card_reference,
        }
    }

}

impl Move for Reserve {
    fn is_valid(&self, game_state: &GameState) -> bool {
        game_state.get_board().get_resources().n_gold() > 0 && game_state.get_current_player().can_add_resources(&RESERVE_RESOURCES) && game_state.get_current_player().can_add_reserve() && self.card_reference.is_in_board(game_state.get_board())
    }

    fn perform(&self, game_state: &GameState) -> GameState {
        let mut game_state_builder = GameStateBuilder::new(game_state);
        game_state_builder.board.resources.n_gold -= 1;
        game_state_builder.players[game_state_builder.current_player_index].resources.n_gold += 1;
        game_state_builder.players[game_state_builder.current_player_index].reserve.push(game_state_builder.board.rows.get(self.card_reference.get_row_index()).remove(self.card_reference.get_card_index()));
        game_state_builder.current_player_index = (game_state_builder.current_player_index + 1) % game_state_builder.players.len();
        game_state_builder.build()
    }
}