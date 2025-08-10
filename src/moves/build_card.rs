use crate::board::rows::card_reference::CardReference;
use crate::game_state::{GameState, GameStateBuilder};
use crate::moves::move_trait::Move;

pub(crate) struct BuildCard {
    card_reference: CardReference,
}

impl BuildCard {
    pub(crate) fn new(card_reference: CardReference) -> Self {
        Self {
            card_reference: card_reference,
        }
    }
}

impl Move for BuildCard {
    fn is_valid(&self, game_state: &GameState) -> bool {
        self.card_reference.is_in_board(game_state.get_board()) && game_state.get_current_player().get_resources().add(&game_state.get_current_player().get_production()).can_pay(self.card_reference.get_from_board(game_state.get_board()).cost())
    }

    fn perform(&self, game_state: &GameState) -> GameState {
        let mut game_state_builder = GameStateBuilder::new(game_state);
        let card = game_state_builder.board.rows.get(self.card_reference.get_row_index()).remove(self.card_reference.get_card_index());
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