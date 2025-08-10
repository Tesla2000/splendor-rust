use crate::board::rows::card_reference::CardReference;
use crate::game_state::{GameState, GameStateBuilder};
use crate::resources::Resources;
use crate::moves::move_trait::Move;

pub(crate) struct ReserveFromHidden {
    row_index: u8,
}
pub(crate) const RESERVE_RESOURCES: Resources = Resources::new(0, 0, 0, 0, 0, 1);

impl ReserveFromHidden {
    pub(crate) fn new(row_index: u8) -> Self {
        Self {
            row_index: row_index,
        }
    }

}

impl Move for ReserveFromHidden {
    fn is_valid(&self, game_state: &GameState) -> bool {
        !game_state.get_board().get_rows().get_row(self.row_index).get_hidden().is_empty() && game_state.get_board().get_resources().n_gold() > 0 && game_state.get_current_player().can_add_resources(&RESERVE_RESOURCES) && game_state.get_current_player().can_add_reserve()
    }

    fn perform(&self, game_state: &GameState) -> GameState {
        let mut game_state_builder = GameStateBuilder::new(game_state);
        game_state_builder.board.resources.n_gold -= 1;
        game_state_builder.players[game_state_builder.current_player_index].resources.n_gold += 1;
        game_state_builder.players[game_state_builder.current_player_index].reserve.push(game_state_builder.board.rows.get(self.row_index).remove(0));
        self.finalize(game_state_builder)
    }
}