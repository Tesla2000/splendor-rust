use crate::card::cost::Cost;
use crate::game_state::GameState;

pub fn give_player_resources(resources: &Cost, game_state: &GameState) -> GameState {
    game_state.get_board().get_resources().pay_cost(resources);
    let mut new_players = Vec::new();
    for (i, player) in game_state.get_players().iter().enumerate() {
        if i == game_state.get_current_player_index() {
            new_players.push(player.add_resources(&resources.to_resources()))
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