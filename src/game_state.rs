use rand::prelude::ThreadRng;
use crate::board::board::{Board, BoardBuilder};
use crate::player::{Player, PlayerBuilder};

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

pub struct GameStateBuilder {
    players: Vec<PlayerBuilder>,
    current_player_index: usize,
    board: BoardBuilder,
}

impl GameStateBuilder {
    pub fn new(game_state: GameState) -> Self {
        Self {
            players: game_state.players.into_iter().map(PlayerBuilder::new).collect(),
            current_player_index: game_state.current_player_index,
            board: BoardBuilder::new(game_state.board),
        }
    }

    pub fn build(self) -> GameState {
        GameState {
            players: self.players.into_iter().map(|b| b.build()).collect(),
            current_player_index: self.current_player_index,
            board: self.board.build(),
        }
    }

    // Getters
    pub fn get_players(&self) -> &Vec<PlayerBuilder> {
        &self.players
    }

    pub fn get_current_player_index(&self) -> usize {
        self.current_player_index
    }

    pub fn get_board(&self) -> &BoardBuilder {
        &self.board
    }

    // Setters
    pub fn set_players(&mut self, players: Vec<PlayerBuilder>) {
        self.players = players;
    }

    pub fn set_current_player_index(&mut self, current_player_index: usize) {
        self.current_player_index = current_player_index;
    }

    pub fn set_board(&mut self, board: BoardBuilder) {
        self.board = board;
    }
}
