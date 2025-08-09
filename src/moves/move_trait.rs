use crate::game_state::GameState;

pub trait Move {
    fn is_valid(&self, game_state: &GameState) -> bool;
    fn perform(&self, game_state: &GameState) -> GameState;
}