use crate::game_state::GameState;
use crate::resources::Resources;

pub fn add_card_to_player_reserve(card: &Resources, game_state: &GameState) -> GameState {
    let mut new_players = Vec::new();
    for (i, player) in game_state.get_players().iter().enumerate() {
        if i == game_state.get_current_player_index() {
            new_players.push(player.add_resources(resources))
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