use splendor::state_encoder::{OneHotCardEncoder, ParameterEncoder, StateEncoder};
use std::env;

mod constants;
mod getters;
mod generate_traces_from_player_zero_state;
mod evaluate_player_zero_state;
mod save_data;
mod state_to_bytes;
pub mod generator;

use generator::generate_synthetic_data;
use save_data::save_states_with_labels;

fn create_encoder(use_one_hot: bool) -> Box<dyn StateEncoder> {
    if use_one_hot {
        Box::new(OneHotCardEncoder::new())
    } else {
        Box::new(ParameterEncoder::new())
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let n_players: u8 = 2;
    let num_games: u32 = if args.len() > 1 {
        args[1].parse().expect("First argument must be a valid number of games")
    } else {
        1_000
    };
    let seed: u64 = if args.len() > 2 {
        args[2].parse().expect("Second argument must be a valid seed")
    } else {
        42
    };
    let n_moves_limit = if args.len() > 3 {
        args[3].parse().expect("Third argument must be a valid number of moves")
    } else {
        69
    };
    let use_one_hot_encoder: bool = if args.len() > 4 {
        args[4].parse().expect("Fourth argument must be a boolean for use_one_hot_encoder")
    } else {
        true
    };
    let max_depth: u8 = 1;
    let encoder = create_encoder(use_one_hot_encoder);
    println!("Running {} games with seed {} (one-hot: {})...", num_games, seed, use_one_hot_encoder);
    let (all_states, all_labels, all_n_moves) = generate_synthetic_data(
        num_games,
        n_players,
        seed,
        n_moves_limit,
        max_depth,
        encoder.as_ref(),
    );
    println!("\nSaving {} collected states...", all_states.len());
    save_states_with_labels(
        all_states,
        all_labels,
        all_n_moves,
        "states.npy",
        "labels.npy",
        "n_moves.npy",
    )
    .expect("Failed to save data");
}