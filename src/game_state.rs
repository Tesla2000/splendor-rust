use rand::prelude::ThreadRng;
use crate::board::board::Board;
use crate::player::Player;

pub struct GameState {
    players: Vec<Player>,
    current_player_index: usize,
    board: Board,
}

impl GameState {
    pub fn from_existing(players: Vec<Player>, current_player_index: usize, board: Board) -> Self {
        Self {
            players: players,
            current_player_index: current_player_index,
            board: board,
        }
    }
    pub fn get_board(&self) -> &Board {
        &self.board
    }
    pub fn get_players(&self) -> &Vec<Player> {
        &self.players
    }
    
    pub fn get_current_player_index(&self) -> usize {
        self.current_player_index
    }
    pub fn get_current_player(&self) -> &Player {
        &self.players.get(self.current_player_index).unwrap()
    }
}

pub fn create_initial_game_state(n_players: u8, rng: &mut ThreadRng) -> GameState {
    let mut players: Vec<Player> = Vec::new();
    for _ in 0..n_players {
        players.push(Player::new())
    }
    GameState {
        players,
        current_player_index: 0,
        board: Board::new(n_players as usize, rng),
    }
}
