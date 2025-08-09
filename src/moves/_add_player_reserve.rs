use crate::board::rows::card_reference::CardReference;
use crate::game_state::GameState;
use crate::moves::reserve::RESERVE_RESOURCES;
pub fn add_player_reserve(card_reference: &CardReference, game_state: &GameState) -> GameState {
    let board = game_state.get_board();
    let new_board = board.take_card(card_reference);
    let new_board_resources = game_state.get_board().get_resources().remove_gold();
    let mut new_players = Vec::new();
    for (i, player) in game_state.get_players().iter().enumerate() {
        if i == game_state.get_current_player_index() {
            new_players.push(player.add_resources(&RESERVE_RESOURCES).add_reserve(card_reference.get_from_board(board)))
        } else {
            new_players.push(player.clone())
        }
    }
    GameState::from_existing(
        new_players,
        game_state.get_current_player_index(),
        game_state.get_board().clone(),
    )
}