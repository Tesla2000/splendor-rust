use rand::prelude::ThreadRng;
use crate::board::board::Board;
use crate::player::Player;

pub struct GameState {
    players: Vec<Player>,
    current_player_index: u8,
    board: Board,
}

pub fn create_initial_game_state(n_players: u8, rng: &mut ThreadRng) -> GameState {
    let mut players: Vec<Player> = Vec::new();
    for _ in 0..n_players {
        players.push(Player::new())
    }
    GameState{
        players,
        current_player_index: 0,
        board: Board::new(n_players as usize, rng),
    }
}
