use splendor::game_state::GameState;
use splendor::player::Player;

/// Get the player who just finished their turn (current player - 1)
pub fn get_last_player_index(game_state: &GameState, n_players: u8) -> usize {
    (game_state.get_current_player_index() + n_players as usize - 1) % n_players as usize
}
pub fn get_last_player(game_state: &GameState, n_players: u8) -> &Player {
    &game_state.get_players()[get_last_player_index(game_state, n_players)]
}

/// Get the points of the player who just finished their turn
pub fn get_last_player_points(game_state: &GameState, n_players: u8) -> u8 {
    get_last_player(game_state, n_players).get_points()
}

/// Get the value tuple (points, -n_cards) for the player who just finished their turn
pub fn get_value(state: &GameState, n_players: u8) -> (u8, i8) {
    let last_player = get_last_player(state, n_players);
    let points = last_player.get_points();
    let n_cards = last_player.get_production().sum() as i8;
    (points, -n_cards)
}