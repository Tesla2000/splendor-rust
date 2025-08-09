use crate::game_state::{GameState, GameStateBuilder};

pub trait Move {
    fn is_valid(&self, game_state: &GameState) -> bool;
    fn perform(&self, game_state: &GameState) -> GameState;
    fn finalize(&self, mut game_state_builder: GameStateBuilder) -> GameState {
        game_state_builder.current_player_index = (game_state_builder.current_player_index + 1) % game_state_builder.players.len();
        game_state_builder.build()
    }
}