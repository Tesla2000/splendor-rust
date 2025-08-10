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
        self.index < reserve.len() && game_state.get_current_player().get_resources().add(&game_state.get_current_player().get_production()).can_pay(reserve[self.index].cost())
    }

    fn perform(&self, game_state: &GameState) -> GameState {
        let mut game_state_builder = GameStateBuilder::new(game_state);
        let card = game_state_builder.get_current_player().reserve.remove(self.index);
        game_state_builder.board.resources.add(&card.cost().to_resources().to_builder());
        let player = game_state_builder.get_current_player();
        player.pay_for_card(&card);
        player.deck.push(card);
        let board_aristocrats = game_state.get_board().get_aristocrats();
        for (index, aristocrat) in board_aristocrats.iter().enumerate() {
            if aristocrat.can_be_taken_by(game_state.get_current_player()) {
                player.aristocrats.push(aristocrat);
                game_state_builder.board.aristocrats.remove(index);
                break;
            }
        }
        self.finalize(game_state_builder)
    }
}
