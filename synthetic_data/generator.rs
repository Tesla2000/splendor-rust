use rand::Rng;
use splendor::game_state::{create_initial_game_state, GameState};
use splendor::moves::all_moves::get_all_moves;
use splendor::state_encoder::StateEncoder;
use std::collections::VecDeque;

use super::constants::WINNING_POINTS;
use super::evaluate_player_zero_state::evaluate_player_zero_state;
use super::getters::get_last_player_points;
use super::state_to_bytes::game_state_to_bytes;

fn play_game<R: Rng>(n_players: u8, rng: &mut R, state_history: &mut VecDeque<GameState>) -> (i32, GameState) {
    let mut current_state = create_initial_game_state(n_players, rng);
    let mut move_num = 0;
    loop {
        move_num += 1;
        let all_moves = get_all_moves();
        let valid_move_indices: Vec<usize> = all_moves
            .iter()
            .enumerate()
            .filter(|(_, m)| m.is_valid(&current_state))
            .map(|(i, _)| i)
            .collect();
        if valid_move_indices.is_empty() {
            state_history.clear();
            return play_game(n_players, rng, state_history);
        }
        let random_index = valid_move_indices[rng.gen_range(0..valid_move_indices.len())];
        let chosen_move = &all_moves[random_index];
        state_history.push_back(current_state.clone());
        current_state = chosen_move.perform(&current_state);
        if get_last_player_points(&current_state, n_players) >= WINNING_POINTS {
            return (move_num, current_state);
        }
    }
}

pub fn generate_synthetic_data<R: Rng + Clone>(
    num_games: u32,
    n_players: u8,
    rng: &mut R,
    n_moves_limit: i32,
    max_depth: u8,
    encoder: &dyn StateEncoder,
) -> (Vec<Vec<u8>>, Vec<i8>, Vec<u8>, Vec<R>) {
    let history_size = n_players as usize;
    let mut all_states: Vec<Vec<u8>> = Vec::new();
    let mut all_labels: Vec<i8> = Vec::new();
    let mut all_n_moves: Vec<u8> = Vec::new();
    let mut rng_states_before_requirement: Vec<R> = Vec::new();
    let mut games_generated = 0;
    while games_generated < num_games {
        let rng_snapshot = rng.clone();
        let mut state_history: VecDeque<GameState> = VecDeque::with_capacity(history_size);
        let (n_moves, _final_state) = play_game(n_players, rng, &mut state_history);
        if n_moves > n_moves_limit {
            continue;
        }
        rng_states_before_requirement.push(rng_snapshot);
        let player_zero_state = state_history
            .iter()
            .rev()
            .find(|state| state.get_current_player_index() == 0)
            .expect("Player zero state must exist in history");
        let evaluation_result = evaluate_player_zero_state(player_zero_state, n_players, max_depth);
        let state_bytes = game_state_to_bytes(player_zero_state, encoder);
        all_states.push(state_bytes);
        all_labels.push(evaluation_result.to_label());
        all_n_moves.push(n_moves as u8);
        games_generated += 1;
    }
    (all_states, all_labels, all_n_moves, rng_states_before_requirement)
}
