use pyo3::prelude::*;

mod card;
mod resource;
pub mod player;
mod resources;
mod aristocrat;
mod aristocrat_storage;
mod board;
pub mod game_state;
pub mod moves;

use crate::card::card::Card;
use crate::game_state::create_initial_game_state;
use crate::moves::all_moves::get_all_moves;
use crate::moves::move_trait::Move;
use crate::resource::Resource;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

#[pyclass]
struct SplendorGame {
    n_players: u8,
    game_state: Option<game_state::GameState>,
    seed: Option<u64>,
}

#[pymethods]
impl SplendorGame {
    #[new]
    #[pyo3(signature = (n_players, seed=None))]
    fn new(n_players: u8, seed: Option<u64>) -> Self {
        let seed_value = seed.unwrap_or_else(|| {
            // Use system randomness if no seed provided
            rand::thread_rng().gen::<u64>()
        });
        let mut rng = ChaCha8Rng::seed_from_u64(seed_value);
        let initial_state = create_initial_game_state(n_players, &mut rng);
        SplendorGame { 
            n_players,
            game_state: Some(initial_state),
            seed: Some(seed_value),
        }
    }
    
    fn get_n_players(&self) -> u8 {
        self.n_players
    }
    
    fn get_seed(&self) -> Option<u64> {
        self.seed
    }
    
    fn set_seed(&mut self, seed: u64) {
        self.seed = Some(seed);
    }
    
    fn reset_with_seed(&mut self, seed: u64) {
        self.seed = Some(seed);
        let mut rng = ChaCha8Rng::seed_from_u64(seed);
        let initial_state = create_initial_game_state(self.n_players, &mut rng);
        self.game_state = Some(initial_state);
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
        
        // Use provided seed or inherit from parent game
        let new_seed = seed.or(self.seed);

        Ok(SplendorGame {
            n_players: self.n_players,
            game_state: Some(new_state),
            seed: new_seed,
        })
    }
    
    fn get_player_points(&self, player_index: usize) -> PyResult<u8> {
        let state = self.game_state.as_ref()
            .ok_or_else(|| pyo3::exceptions::PyRuntimeError::new_err("Game state not initialized"))?;
        
        let players = state.get_players();
        if player_index >= players.len() {
            return Err(pyo3::exceptions::PyIndexError::new_err(
                format!("Player index {} out of range", player_index)
            ));
        }
        
        Ok(players[player_index].get_points())
    }
    
    fn get_current_player_index(&self) -> PyResult<usize> {
        let state = self.game_state.as_ref()
            .ok_or_else(|| pyo3::exceptions::PyRuntimeError::new_err("Game state not initialized"))?;
        
        Ok(state.get_current_player_index())
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
            for i in 0..4 {
                if row.has_card(i) {
                    add_card_to_state(&mut output, Option::from(&row.get_card(i)))
                } else {
                    add_card_to_state(&mut output, None)
                }
            }
        }
        Ok(output)
    }
}

#[pymodule]
fn splendor(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<SplendorGame>()?;
    Ok(())
}