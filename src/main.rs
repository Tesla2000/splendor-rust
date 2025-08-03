use rand::rng;
use crate::game_state::{create_initial_game_state, GameState};

mod card;
mod resource;
mod player;
mod resources;
mod aristocrat;
mod board;
mod game_state;

fn main() {
    let n_players: u8 = 2;
    let mut rng = rng();
    let initial_state: GameState = create_initial_game_state(n_players, &mut rng);
    println!("Created initial state");
    return;
}
