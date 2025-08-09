use std::ops::RangeTo;
use std::rc::Rc;
use rand::prelude::IndexedRandom;
use rand::{rng, Rng};
use crate::game_state::{create_initial_game_state, GameState};
use crate::moves::all_moves::get_all_moves;
use crate::node::Node;

mod card;
mod resource;
mod player;
mod resources;
mod aristocrat;
mod board;
mod game_state;
mod moves;
mod node;

fn main() {
    let n_players: u8 = 2;
    let mut rng = rng();

    let all_moves = get_all_moves();
    let mut current_state: GameState = create_initial_game_state(n_players, &mut rng);

    let mut current = Node::new(current_state.clone());

    loop {
        let valid_moves: Vec<_> = all_moves
            .iter()
            .filter(|m| m.is_valid(&current_state))
            .collect();

        if valid_moves.is_empty() {
            break;
        }


        let n_valid_moves = valid_moves.len();

        let chosen_child_index = rng.random_range(0..n_valid_moves);

        let mut new_current_state = current_state.clone();
        for (index, m) in valid_moves.iter().enumerate() {
            let new_state = m.perform(&current_state);
            if index == chosen_child_index {
                new_current_state = m.perform(&current_state);
                current = Node::add_child(&current, new_state);

            } else {
                Node::add_child(&current, new_state);
            }
        }
        current_state = new_current_state;
    }
}
