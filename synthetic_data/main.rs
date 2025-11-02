use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use splendor::game_state::{create_initial_game_state, GameState};
use splendor::moves::all_moves::get_all_moves;
use std::collections::VecDeque;

mod constants;
mod getters;
mod generate_traces_from_player_zero_state;
mod evaluate_player_zero_state;

use constants::WINNING_POINTS;
use evaluate_player_zero_state::evaluate_player_zero_state;
use getters::get_last_player_points;

fn play_game<R: Rng>(n_players: u8, rng: &mut R, state_history: &mut VecDeque<GameState>) -> GameState {
    let mut current_state = create_initial_game_state(n_players, rng);

    loop {
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
            return current_state;
        }
    }
}

fn main() {
    let n_players: u8 = 2;
    let seed: u64 = 42;
    let mut rng = ChaCha8Rng::seed_from_u64(seed);
    let history_size = n_players as usize;
    for _ in 0..10 {
        let mut state_history: VecDeque<GameState> = VecDeque::with_capacity(history_size);

        play_game(n_players, &mut rng, &mut state_history);

        // Find the most recent state where player_index is 0
        let player_zero_state = state_history
            .iter()
            .rev()
            .find(|state| state.get_current_player_index() == 0)
            .expect("Player zero state must exist in history");

        let result = evaluate_player_zero_state(player_zero_state, n_players);

        println!("Player 0 state is {:?}", result);
        println!("Point {:?}, {:?}", player_zero_state.get_players()[0].get_points(), player_zero_state.get_players()[1].get_points());
    }

}