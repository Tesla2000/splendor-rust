use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use splendor::game_state::{create_initial_game_state, GameState};
use splendor::moves::all_moves::get_all_moves;
use std::collections::VecDeque;

mod constants;
mod getters;
mod generate_traces_from_player_zero_state;
mod evaluate_player_zero_state;
mod save_data;
mod state_to_bytes;

use constants::WINNING_POINTS;
use evaluate_player_zero_state::evaluate_player_zero_state;
use getters::get_last_player_points;
use save_data::save_states_with_labels;
use state_to_bytes::game_state_to_bytes;

fn play_game<R: Rng>(n_players: u8, rng: &mut R, state_history: &mut VecDeque<GameState>) -> (i32, GameState) {
    let mut current_state = create_initial_game_state(n_players, rng);
    let mut move_num = 0;
    loop {
        move_num = move_num + 1;
        // Get valid moves
        let all_moves = get_all_moves();
        let valid_move_indices: Vec<usize> = all_moves
            .iter()
            .enumerate()
            .filter(|(_, m)| m.is_valid(&current_state))
            .map(|(i, _)| i)
            .collect();

        // Check if there are no valid moves (game ended for other reasons) - restart recursively
        if valid_move_indices.is_empty() {
            state_history.clear();
            return play_game(n_players, rng, state_history);
        }

        // Select random valid move
        let random_index = valid_move_indices[rng.gen_range(0..valid_move_indices.len())];
        let chosen_move = &all_moves[random_index];

        state_history.push_back(current_state.clone());
        current_state = chosen_move.perform(&current_state);

        // Check win condition: the player who just finished their turn reached winning points
        if get_last_player_points(&current_state, n_players) >= WINNING_POINTS {
            return (move_num, current_state);
        }
    }
}

fn main() {
    let n_players: u8 = 2;
    let seed: u64 = 42;
    let mut rng = ChaCha8Rng::seed_from_u64(seed);
    let history_size = n_players as usize;

    // Track move count distribution
    let num_games = 200_000;
    let n_moves_limit = 69;
    println!("Running {} games...", num_games);

    // Collections for saving data
    let mut all_states: Vec<Vec<u8>> = Vec::new();
    let mut all_labels: Vec<i8> = Vec::new();
    let mut all_n_moves: Vec<i32> = Vec::new();

    for i in 0..num_games {
        let mut state_history: VecDeque<GameState> = VecDeque::with_capacity(history_size);
        let (n_moves, _final_state) = play_game(n_players, &mut rng, &mut state_history);

        if (i + 1) % 100 == 0 {
            println!("Completed {} games...", i + 1);
        }
        if n_moves > n_moves_limit {
            continue;
        }

        // Find the most recent state where player_index is 0
        let player_zero_state = state_history
            .iter()
            .rev()
            .find(|state| state.get_current_player_index() == 0)
            .expect("Player zero state must exist in history");

        let evaluation_result = evaluate_player_zero_state(player_zero_state, n_players);

        // Convert state to bytes and save
        let state_bytes = game_state_to_bytes(player_zero_state);
        all_states.push(state_bytes);
        all_labels.push(evaluation_result.to_label());
        all_n_moves.push(n_moves);
    }

    // Save all collected data
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