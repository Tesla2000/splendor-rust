use pyo3::prelude::*;

mod card;
mod resource;
mod player;
mod resources;
mod aristocrat;
mod aristocrat_storage;
mod board;
mod game_state;
mod moves;
mod node;

use crate::game_state::create_initial_game_state;
use crate::moves::all_moves::get_all_moves;
use crate::moves::move_trait::Move;
use crate::node::Node;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use std::cell::RefCell;
use std::rc::Rc;
use crate::card::card::Card;
use crate::resource::Resource;

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
    
    #[pyo3(signature = (n_simulations, seed=None))]
    fn run_simulation(&self, n_simulations: u16, seed: Option<u64>) -> PyResult<Vec<(usize, u32, f32, f32)>> {
        let seed_value = seed.or(self.seed).unwrap_or_else(|| rand::thread_rng().gen::<u64>());
        let mut rng = ChaCha8Rng::seed_from_u64(seed_value);
        let current_state = self.game_state.as_ref()
            .ok_or_else(|| pyo3::exceptions::PyRuntimeError::new_err("Game state not initialized"))?;
        let root = Node::new(current_state.clone(), &mut rng);
        let all_moves = get_all_moves();
        
        // Initial expansion
        let valid_moves: Vec<_> = all_moves
            .iter()
            .filter(|m| m.is_valid(&root.borrow().get_game_state()))
            .collect();
            
        for m in &valid_moves {
            let new_state = m.perform(&root.borrow().get_game_state());
            let child = Node::add_child(&root, new_state, &mut rng);
            rollout(&child, &all_moves, &mut rng, false);
        }
        
        // Run simulations
        for _ in 0..n_simulations {
            let leaf_index = get_expanded_leaf_index_all_visited(&root);
            let expanded_leaf = Rc::clone(&root.borrow().children[leaf_index]);
            rollout(&expanded_leaf, &all_moves, &mut rng, false);
        }
        
        // Collect results
        let mut results = Vec::new();
        for (i, child) in root.borrow().children.iter().enumerate() {
            let child_ref = child.borrow();
            let win_rate = if child_ref.visits > 0 {
                (child_ref.visits as f32 + child_ref.score) / child_ref.visits as f32 / 2.0
            } else {
                0.0
            };
            results.push((i, child_ref.visits, child_ref.score, win_rate));
        }
        
        Ok(results)
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

        for player in state.get_players() {
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



fn rollout<R: Rng>(current: &Rc<RefCell<Node>>, all_moves: &Vec<Box<dyn Move>>, rng: &mut R, print: bool) {
    use crate::moves::all_moves::get_n_moves;
    
    let mut current_owned = Rc::clone(current);
    loop {
        let next_move_to_perform = current_owned.borrow().next_move_to_perform;
        if next_move_to_perform < get_n_moves() {
            if !add_leaf(&current_owned, all_moves, rng) {
                if print {
                    println!("No valid moves");
                }
                backpropagate(&current_owned, -1.0);
                break;
            }
        }
        let leaf_index = get_expanded_leaf_index(&current_owned);
        
        let children_count = current_owned.borrow().children.len();
        if children_count > 0 {
            let new_current = Rc::clone(&current_owned.borrow().children[leaf_index]);
            current_owned = new_current;
        } else {
            break;
        }
        
        if current_owned.borrow().get_game_state().get_current_player_index() == 0 {
            let max_points = current_owned.borrow().get_game_state().get_players().iter().map(|p| p.get_points()).max().unwrap();
            if max_points < 15 {
                continue;
            }
            let n_points = current_owned.borrow().get_game_state().get_current_player().get_points();
            if n_points == max_points {
                if print {
                    println!("Player 0 wins");
                }
                backpropagate(&current_owned, 1.0);
            } else {
                if print {
                    println!("Player 0 looses");
                }
                backpropagate(&current_owned, -1.0);
            }
            break;
        }
    }
}

fn add_leaf<R: Rng>(current: &Rc<RefCell<Node>>, all_moves: &Vec<Box<dyn Move>>, rng: &mut R) -> bool {
    let valid_move_data = {
        let current_node = current.borrow();
        let next_move_to_perform = current_node.next_move_to_perform;
        
        let valid_move = current_node.move_order()
            .iter()
            .skip(next_move_to_perform)
            .map(|move_index| all_moves[*move_index].as_ref())
            .enumerate()
            .find(|(_, move_)| move_.is_valid(current_node.get_game_state()));
        valid_move
    };
    
    let valid_moves = valid_move_data.is_some();
    
    if let Some((index, m)) = valid_move_data {
        let new_state = m.perform(&current.borrow().game_state);
        Node::add_child(current, new_state, rng);
        current.borrow_mut().next_move_to_perform += index + 1;
    }
    
    valid_moves
}

fn get_expanded_leaf_index(node_ref: &Rc<RefCell<Node>>) -> usize {
    let node = node_ref.borrow();
    
    if node.children[node.children.len() - 1].borrow().visits > 0 {
        get_expanded_leaf_index_all_visited(node_ref)
    } else {
        node.children.len() - 1
    }
}

fn get_expanded_leaf_index_all_visited(node_ref: &Rc<RefCell<Node>>) -> usize {
    let node = node_ref.borrow();
    node.children.iter()
        .enumerate()
        .max_by(|(_, a), (_, b)| Node::ucb1(a).partial_cmp(&Node::ucb1(b)).unwrap())
        .map(|(i, _)| i)
        .expect("No children")
}

fn backpropagate(node: &Rc<RefCell<Node>>, is_win: f32) {
    {
        let mut node_mut = node.borrow_mut();
        node_mut.visits += 1;
        node_mut.score += is_win;
    }
    
    if let Some(parent) = node.borrow().parent.upgrade() {
        backpropagate(&parent, -is_win);
    }
}

#[pymodule]
fn splendor(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<SplendorGame>()?;
    Ok(())
}