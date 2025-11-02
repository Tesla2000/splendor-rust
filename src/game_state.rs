use rand::Rng;
use crate::board::board::Board;
use crate::card::card::Card;
use crate::player::Player;
use crate::resource::Resource;
use crate::resources::ResourcesBuilder;

#[derive(Clone)]
pub struct GameState {
    players: Vec<Player>,
    current_player_index: usize,
    board: Board,
}

impl GameState {
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

pub fn create_initial_game_state<R: Rng>(n_players: u8, rng: &mut R) -> GameState {
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
    pub players: Vec<crate::player::PlayerBuilder>,
    pub current_player_index: usize,
    pub board: crate::board::board::BoardBuilder,
}

impl GameStateBuilder {
    pub fn new(game_state: &GameState) -> Self {
        Self {
            players: game_state.players.iter().map(|p| p.to_builder()).collect(),
            current_player_index: game_state.current_player_index,
            board: game_state.board.to_builder(),
        }
    }

    pub fn get_current_player(&mut self) -> &mut crate::player::PlayerBuilder {
        &mut self.players[self.current_player_index]
    }

    pub(crate) fn add_resources_to_player(&mut self, resources_builder: &ResourcesBuilder) {
        self.players[self.current_player_index].add_resources(resources_builder)
    }

    pub fn build(self) -> GameState {
        GameState {
            players: self.players.into_iter().map(|b| b.build()).collect(),
            current_player_index: self.current_player_index,
            board: self.board.build(),
        }
    }
}
