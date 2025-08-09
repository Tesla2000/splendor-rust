use crate::board::rows::card_reference::CardReference;
use crate::game_state::{GameState, GameStateBuilder};
use crate::moves::move_trait::Move;

pub(crate) struct BuildFromReserve {
    index: usize,
}

impl BuildFromReserve {
    pub(crate) fn new(index: usize) -> Self {
        Self {
            index: index,
        }
    }

}

impl Move for BuildFromReserve {
    fn is_valid(&self, game_state: &GameState) -> bool {
        let reserve = game_state.get_current_player().get_reserve();
        self.index < reserve.len() && game_state.get_current_player().get_resources().add(&game_state.get_current_player().get_production()).can_pay(&reserve[self.index].cost)
    }

    fn perform(&self, game_state: &GameState) -> GameState {
        let mut game_state_builder = GameStateBuilder::new(game_state);
        let player = game_state_builder.get_current_player();
        let card = player.reserve.remove(self.index);
        player.pay_for_card(&card);
        player.deck.push(card);
        game_state_builder.current_player_index = (game_state_builder.current_player_index + 1) % game_state_builder.players.len();
        game_state_builder.build()
    }
}
