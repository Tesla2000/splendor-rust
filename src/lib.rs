use pyo3::prelude::*;

pub mod card;
pub mod resource;
pub mod player;
mod resources;
mod aristocrat;
mod aristocrat_storage;
pub mod board;
pub mod game_state;
pub mod moves;
pub mod state_encoder;

use crate::card::card::Card;
use crate::game_state::create_initial_game_state;
use crate::moves::all_moves::get_all_moves;
use crate::moves::move_trait::Move;
use crate::resource::Resource;
use crate::state_encoder::{OneHotCardEncoder, ParameterEncoder, StateEncoder};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

const WINNING_POINTS: u8 = 15;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum EvaluationResult {
    Winning,
    Losing,
    Draw,
}

impl EvaluationResult {
    fn to_label(self) -> i8 {
        match self {
            EvaluationResult::Winning => 1,
            EvaluationResult::Losing => -1,
            EvaluationResult::Draw => 0,
        }
    }
}

fn get_last_player_index(game_state: &game_state::GameState, n_players: u8) -> usize {
    (game_state.get_current_player_index() + n_players as usize - 1) % n_players as usize
}

fn evaluate_player_zero_state(state: &game_state::GameState, n_players: u8, max_depth: u8) -> EvaluationResult {
    if max_depth == 0 {
        return EvaluationResult::Draw;
    }
    let all_moves = get_all_moves();
    let mut all_children_losing = true;
    let mut has_draw_child = false;
    let mut player_zero_states: Vec<game_state::GameState> = Vec::new();
    for valid_move in all_moves.iter().filter(|m| m.is_valid(state)) {
        let child_state = valid_move.perform(state);
        let child_last_player_idx = get_last_player_index(&child_state, n_players);
        let child_points = child_state.get_players()[child_last_player_idx].get_points();
        let child_n_cards = child_state.get_players()[child_last_player_idx].get_production().sum();
        if child_points >= WINNING_POINTS {
            let mut traces: Vec<Vec<game_state::GameState>> = vec![vec![]];
            for _ in 1..n_players {
                let mut new_traces = Vec::new();
                for trace in &traces {
                    let current = if trace.is_empty() { &child_state } else { trace.last().unwrap() };
                    for m in all_moves.iter().filter(|m| m.is_valid(current)) {
                        let mut new_trace = trace.clone();
                        new_trace.push(m.perform(current));
                        new_traces.push(new_trace);
                    }
                }
                traces = new_traces;
            }
            let is_better = traces.iter().all(|trace| {
                trace.iter().all(|s| {
                    let idx = get_last_player_index(s, n_players);
                    let pts = s.get_players()[idx].get_points();
                    let cards = s.get_players()[idx].get_production().sum();
                    child_points > pts || (child_points == pts && child_n_cards < cards)
                })
            });
            if is_better {
                return EvaluationResult::Winning;
            }
            let has_equal = traces.iter().any(|trace| {
                trace.iter().any(|s| {
                    let idx = get_last_player_index(s, n_players);
                    let pts = s.get_players()[idx].get_points();
                    let cards = s.get_players()[idx].get_production().sum();
                    child_points == pts && child_n_cards == cards
                })
            });
            if has_equal {
                has_draw_child = true;
            }
        }
        let mut has_losing_trace = false;
        let mut traces: Vec<Vec<game_state::GameState>> = vec![vec![]];
        for _ in 1..n_players {
            let mut new_traces = Vec::new();
            for trace in &traces {
                let current = if trace.is_empty() { &child_state } else { trace.last().unwrap() };
                for m in all_moves.iter().filter(|m| m.is_valid(current)) {
                    let mut new_trace = trace.clone();
                    new_trace.push(m.perform(current));
                    new_traces.push(new_trace);
                }
            }
            traces = new_traces;
        }
        for trace in &traces {
            if let Some(last) = trace.last() {
                let idx = get_last_player_index(last, n_players);
                if last.get_players()[idx].get_points() >= WINNING_POINTS {
                    has_losing_trace = true;
                    break;
                }
                if last.get_current_player_index() == 0 {
                    player_zero_states.push(last.clone());
                }
            }
        }
        if !has_losing_trace {
            all_children_losing = false;
        }
    }
    if all_children_losing {
        return EvaluationResult::Losing;
    }
    for s in player_zero_states {
        let result = evaluate_player_zero_state(&s, n_players, max_depth - 1);
        if result == EvaluationResult::Winning {
            return EvaluationResult::Winning;
        }
        if result == EvaluationResult::Draw {
            has_draw_child = true;
        }
    }
    if has_draw_child { EvaluationResult::Draw } else { EvaluationResult::Losing }
}

fn create_encoder(use_one_hot: bool) -> Box<dyn StateEncoder> {
    if use_one_hot {
        Box::new(OneHotCardEncoder::new())
    } else {
        Box::new(ParameterEncoder::new())
    }
}

#[pyclass]
struct SplendorGame {
    n_players: u8,
    game_state: Option<game_state::GameState>,
    seed: Option<u64>,
    encoder: Box<dyn StateEncoder>,
}

#[pymethods]
impl SplendorGame {
    #[new]
    #[pyo3(signature = (n_players, seed=None, use_one_hot_encoder=true))]
    fn new(n_players: u8, seed: Option<u64>, use_one_hot_encoder: bool) -> Self {
        let seed_value = seed.unwrap_or_else(|| {
            rand::thread_rng().gen::<u64>()
        });
        let mut rng = ChaCha8Rng::seed_from_u64(seed_value);
        let initial_state = create_initial_game_state(n_players, &mut rng);
        SplendorGame {
            n_players,
            game_state: Some(initial_state),
            seed: Some(seed_value),
            encoder: create_encoder(use_one_hot_encoder),
        }
    }

    fn get_valid_moves(&self) -> PyResult<Vec<usize>> {
        let state = self.game_state.as_ref()
            .ok_or_else(|| pyo3::exceptions::PyRuntimeError::new_err("Game state not initialized"))?;
        
        let all_moves = get_all_moves();
        
        let valid_move_indices: Vec<usize> = all_moves
            .iter()
            .enumerate()
            .filter(|(_, m)| m.is_valid(state))
            .map(|(i, _)| i)
            .collect();
            
        Ok(valid_move_indices)
    }
    
    #[pyo3(signature = (move_index, seed=None))]
    fn apply_move(&mut self, move_index: usize, seed: Option<u64>) -> PyResult<SplendorGame> {
        let current_state = self.game_state.as_ref()
            .ok_or_else(|| pyo3::exceptions::PyRuntimeError::new_err("Game state not initialized"))?;
        
        let all_moves = get_all_moves();
        
        if move_index >= all_moves.len() {
            return Err(pyo3::exceptions::PyIndexError::new_err(
                format!("Move index {} out of range", move_index)
            ));
        }
        
        let m = &all_moves[move_index];
        
        if !m.is_valid(current_state) {
            return Err(pyo3::exceptions::PyValueError::new_err(
                "Invalid move for current game state"
            ));
        }
        
        let new_state = m.perform(current_state);
        let new_seed = seed.or(self.seed);
        Ok(SplendorGame {
            n_players: self.n_players,
            game_state: Some(new_state),
            seed: new_seed,
            encoder: self.encoder.clone_box(),
        })
    }

    fn get_all_player_points(&self) -> PyResult<Vec<u8>> {
        let state = self.game_state.as_ref()
            .ok_or_else(|| pyo3::exceptions::PyRuntimeError::new_err("Game state not initialized"))?;
        
        let points: Vec<u8> = state.get_players()
            .iter()
            .map(|p| p.get_points())
            .collect();
        
        Ok(points)
    }
    
    fn is_game_over(&self) -> PyResult<bool> {
        let state = self.game_state.as_ref()
            .ok_or_else(|| pyo3::exceptions::PyRuntimeError::new_err("Game state not initialized"))?;
        
        let max_points = state.get_players()
            .iter()
            .map(|p| p.get_points())
            .max()
            .unwrap_or(0);
        
        Ok(max_points >= 15)
    }
    
    fn get_winner(&self) -> PyResult<Option<usize>> {
        let state = self.game_state.as_ref()
            .ok_or_else(|| pyo3::exceptions::PyRuntimeError::new_err("Game state not initialized"))?;
        
        let players = state.get_players();
        let max_points = players
            .iter()
            .map(|p| p.get_points())
            .max()
            .unwrap_or(0);
        
        if max_points >= 15 {
            // Find player with max points
            for (index, player) in players.iter().enumerate() {
                if player.get_points() == max_points {
                    return Ok(Some(index));
                }
            }
        }
        
        Ok(None)
    }

    fn get_game_state(&self) -> PyResult<Vec<u8>> {
        fn add_card_to_state(state: &mut Vec<u8>, card: Option<&&Card>) {
            if let Some(card) = card {
                state.push(card.n_points());
                state.push(card.cost().n_green());
                state.push(card.cost().n_red());
                state.push(card.cost().n_blue());
                state.push(card.cost().n_black());
                state.push(card.cost().n_white());
                match card.production() {
                    Resource::Green => {state.append(&mut vec![1, 0, 0, 0, 0])}
                    Resource::Red => {state.append(&mut vec![0, 1, 0, 0, 0])}
                    Resource::Blue => {state.append(&mut vec![0, 0, 1, 0, 0])}
                    Resource::White => {state.append(&mut vec![0, 0, 0, 1, 0])}
                    Resource::Black => {state.append(&mut vec![0, 0, 0, 0, 1])}
                }
            } else {
                state.push(0);
                state.push(0);
                state.push(0);
                state.push(0);
                state.push(0);
                state.push(0);
                state.append(&mut vec![0, 0, 0, 0, 0])
            }

        }
        let state = self.game_state.as_ref()
            .ok_or_else(|| pyo3::exceptions::PyRuntimeError::new_err("Game state not initialized"))?;
        let mut output = Vec::new();

        let players_in_order: Vec<_>  = state.get_players().iter().chain(state.get_players().iter()).collect();
        for player in &players_in_order[state.get_current_player_index()..state.get_current_player_index()+state.get_players().len()] {
            output.push(player.get_points());
            let resources = player.get_resources();
            output.push(resources.n_green());
            output.push(resources.n_red());
            output.push(resources.n_blue());
            output.push(resources.n_black());
            output.push(resources.n_white());
            output.push(resources.n_gold());
            let production = player.get_production();
            output.push(production.n_green());
            output.push(production.n_red());
            output.push(production.n_blue());
            output.push(production.n_black());
            output.push(production.n_white());
            for i in 0..3 {
                add_card_to_state(&mut output, player.get_reserve().get(i));
            }
        }
        for row_index in 0..3 {
            let row = state.get_board().get_rows().get_row(row_index);
            output.extend(self.encoder.encode_row(row));
        }
        Ok(output)
    }
}

#[pyfunction]
#[pyo3(signature = (num_games, n_players=2, seed=42, n_moves_limit=69, use_one_hot_encoder=true, max_depth=1))]
fn generate_synthetic_data(
    num_games: u32,
    n_players: u8,
    seed: u64,
    n_moves_limit: i32,
    use_one_hot_encoder: bool,
    max_depth: u8,
) -> PyResult<(Vec<Vec<u8>>, Vec<i8>, Vec<u8>)> {
    use std::collections::VecDeque;
    let mut rng = ChaCha8Rng::seed_from_u64(seed);
    let history_size = n_players as usize;
    let encoder = create_encoder(use_one_hot_encoder);
    let mut all_states: Vec<Vec<u8>> = Vec::new();
    let mut all_labels: Vec<i8> = Vec::new();
    let mut all_n_moves: Vec<u8> = Vec::new();
    let mut games_generated = 0;
    while games_generated < num_games {
        let mut state_history: VecDeque<game_state::GameState> = VecDeque::with_capacity(history_size);
        let mut current_state = create_initial_game_state(n_players, &mut rng);
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
                current_state = create_initial_game_state(n_players, &mut rng);
                move_num = 0;
                continue;
            }
            let random_index = valid_move_indices[rng.gen_range(0..valid_move_indices.len())];
            let chosen_move = &all_moves[random_index];
            state_history.push_back(current_state.clone());
            current_state = chosen_move.perform(&current_state);
            let last_player_index = get_last_player_index(&current_state, n_players);
            let last_player_points = current_state.get_players()[last_player_index].get_points();
            if last_player_points >= WINNING_POINTS {
                break;
            }
        }
        if move_num > n_moves_limit {
            continue;
        }
        let player_zero_state = state_history
            .iter()
            .rev()
            .find(|state| state.get_current_player_index() == 0)
            .ok_or_else(|| pyo3::exceptions::PyRuntimeError::new_err("Player zero state must exist in history"))?;
        let evaluation_result = evaluate_player_zero_state(player_zero_state, n_players, max_depth);
        let mut state_bytes = Vec::new();
        let players_in_order: Vec<_> = player_zero_state
            .get_players()
            .iter()
            .chain(player_zero_state.get_players().iter())
            .collect();
        let current_idx = player_zero_state.get_current_player_index();
        let n_players_usize = player_zero_state.get_players().len();
        for player in &players_in_order[current_idx..current_idx + n_players_usize] {
            state_bytes.push(player.get_points());
            let resources = player.get_resources();
            state_bytes.push(resources.n_green());
            state_bytes.push(resources.n_red());
            state_bytes.push(resources.n_blue());
            state_bytes.push(resources.n_black());
            state_bytes.push(resources.n_white());
            state_bytes.push(resources.n_gold());
            let production = player.get_production();
            state_bytes.push(production.n_green());
            state_bytes.push(production.n_red());
            state_bytes.push(production.n_blue());
            state_bytes.push(production.n_black());
            state_bytes.push(production.n_white());
            for i in 0..3 {
                if let Some(card) = player.get_reserve().get(i) {
                    state_bytes.push(card.n_points());
                    state_bytes.push(card.cost().n_green());
                    state_bytes.push(card.cost().n_red());
                    state_bytes.push(card.cost().n_blue());
                    state_bytes.push(card.cost().n_black());
                    state_bytes.push(card.cost().n_white());
                    match card.production() {
                        Resource::Green => state_bytes.extend_from_slice(&[1, 0, 0, 0, 0]),
                        Resource::Red => state_bytes.extend_from_slice(&[0, 1, 0, 0, 0]),
                        Resource::Blue => state_bytes.extend_from_slice(&[0, 0, 1, 0, 0]),
                        Resource::White => state_bytes.extend_from_slice(&[0, 0, 0, 1, 0]),
                        Resource::Black => state_bytes.extend_from_slice(&[0, 0, 0, 0, 1]),
                    }
                } else {
                    state_bytes.extend_from_slice(&[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
                }
            }
        }
        for row_index in 0..3 {
            let row = player_zero_state.get_board().get_rows().get_row(row_index);
            state_bytes.extend(encoder.encode_row(row));
        }
        all_states.push(state_bytes);
        all_labels.push(evaluation_result.to_label());
        all_n_moves.push(move_num as u8);
        games_generated += 1;
    }
    Ok((all_states, all_labels, all_n_moves))
}

#[pymodule]
fn splendor(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<SplendorGame>()?;
    m.add_function(wrap_pyfunction!(generate_synthetic_data, m)?)?;
    Ok(())
}