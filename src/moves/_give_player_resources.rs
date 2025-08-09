use crate::card::cost::Cost;
use crate::game_state::{GameState, GameStateBuilder};

pub fn give_player_resources(resources: &Cost, game_state: &GameState) -> GameState {
    let mut game_state_builder = GameStateBuilder::new(game_state);
    game_state_builder.board.resources.pay_cost(resources);
    game_state_builder.add_resources_to_player(&resources.to_resources().to_builder());
    game_state_builder.current_player_index = (game_state_builder.current_player_index + 1) % game_state_builder.players.len();
    game_state_builder.build()
}