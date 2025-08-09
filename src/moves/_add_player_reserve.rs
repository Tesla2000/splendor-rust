use crate::board::rows::card_reference::CardReference;
use crate::card::card::Card;
use crate::game_state::{GameState, GameStateBuilder};
pub fn add_player_reserve(card_reference: &CardReference, game_state: &GameState) -> GameState {
    let mut game_state_builder = GameStateBuilder::new(game_state);
    game_state_builder.board.resources.n_gold -= 1;
    game_state_builder.players[game_state_builder.current_player_index].resources.n_gold += 1;
    let card = card_reference.get_from_board(game_state.get_board());
    game_state_builder.players[game_state_builder.current_player_index].reserve.push()
    game_state_builder.add_resources_to_player(&resources.to_resources().to_builder());
    game_state_builder.build()
}