use splendor::state_encoder::{OneHotCardEncoder, ParameterEncoder, StateEncoder};
use std::env;
use std::fs;

mod constants;
mod getters;
mod generate_traces_from_player_zero_state;
mod evaluate_player_zero_state;
mod save_data;
mod state_to_bytes;
mod rng_state;
pub mod generator;

#[cfg(test)]
mod test_rng_state;

use generator::generate_synthetic_data;
use save_data::save_states_with_labels;
use rng_state::{create_or_load_rng, save_rng_states_batch};

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
        1
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
    let output_dir: String = if args.len() > 5 {
        args[5].clone()
    } else {
        ".".to_string()
    };
    let rng_states_dir: String = if args.len() > 6 {
        args[6].clone()
    } else {
        "rng_states".to_string()
    };
    let max_depth: u8 = 1;
    let save_interval: u32 = 1000;
    fs::create_dir_all(&output_dir).expect("Failed to create output directory");
    fs::create_dir_all(&rng_states_dir).expect("Failed to create rng_states directory");
    let encoder = create_encoder(use_one_hot_encoder);
    let initial_rng_path = format!("{}/rng_state_0.bin", rng_states_dir);
    let mut rng = create_or_load_rng(seed, &initial_rng_path);
    println!("Running {} games (one-hot: {})...", num_games, use_one_hot_encoder);
    println!("Output directory: {}", output_dir);
    println!("RNG states directory: {}", rng_states_dir);
    let mut all_states: Vec<Vec<u8>> = Vec::new();
    let mut all_labels: Vec<i8> = Vec::new();
    let mut all_n_moves: Vec<u8> = Vec::new();
    let mut all_rng_states = Vec::new();
    for game_num in 1..=num_games {
        let (states, labels, n_moves, rng_states) = generate_synthetic_data(
            1,
            n_players,
            &mut rng,
            n_moves_limit,
            max_depth,
            encoder.as_ref(),
        );
        all_states.extend(states);
        all_labels.extend(labels);
        all_n_moves.extend(n_moves);
        all_rng_states.extend(rng_states);
        if game_num % save_interval == 0 {
            println!("Saving checkpoint at {} / {} games...", game_num, num_games);
            save_states_with_labels(
                all_states.clone(),
                all_labels.clone(),
                all_n_moves.clone(),
                &format!("{}/states_{}.npy", output_dir, game_num),
                &format!("{}/labels_{}.npy", output_dir, game_num),
                &format!("{}/n_moves_{}.npy", output_dir, game_num),
            )
            .expect("Failed to save checkpoint data");
            let rng_states_path = format!("{}/rng_states_{}.bin", rng_states_dir, game_num);
            save_rng_states_batch(&all_rng_states, &rng_states_path)
                .expect("Failed to save RNG states batch");
            all_states.clear();
            all_labels.clear();
            all_n_moves.clear();
            all_rng_states.clear();
        }
        if game_num % 100 == 0 {
            println!("Completed {} / {} games", game_num, num_games);
        }
    }
    if !all_states.is_empty() {
        println!("\nSaving final {} collected states...", all_states.len());
        save_states_with_labels(
            all_states,
            all_labels,
            all_n_moves,
            &format!("{}/states_{}.npy", output_dir, num_games),
            &format!("{}/labels_{}.npy", output_dir, num_games),
            &format!("{}/n_moves_{}.npy", output_dir, num_games),
        )
        .expect("Failed to save data");
        let rng_states_path = format!("{}/rng_states_{}.bin", rng_states_dir, num_games);
        save_rng_states_batch(&all_rng_states, &rng_states_path)
            .expect("Failed to save final RNG states batch");
    }
}