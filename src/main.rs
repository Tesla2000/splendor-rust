use crate::game_state::create_initial_game_state;
use crate::moves::all_moves::get_all_moves;
use crate::moves::move_trait::Move;
use crate::node::Node;
use rand::prelude::{IndexedRandom, IteratorRandom};
use rand::rngs::ThreadRng;
use rand::{rng, Rng};
use std::cell::RefCell;
use std::rc::Rc;

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
    let print = false;
    let mut rng = rng();

    let root = Node::new(create_initial_game_state(n_players, &mut rng));
    let valid_moves: Vec<_> = get_all_moves()
        .into_iter()
        .filter(|m| m.is_valid(&root.borrow().get_game_state()))
        .collect();
    let all_moves: Vec<_> = get_all_moves();
    for m in &valid_moves {
        let new_state = m.perform(&root.borrow().get_game_state());
        roolout(&Node::add_child(&root, new_state), &all_moves, &mut rng, print);
    }
    for _ in 0..10000 {
        let leaf_index = get_expanded_leaf_index_all_visited(&root);
        let expanded_leaf = Rc::clone(&root.borrow().children[leaf_index]);
        roolout(&expanded_leaf, &all_moves, &mut rng, print)
    }
    println!("\nRoot children statistics:");
    println!("Child | Visits | Score | Win Rate");
    println!("------|--------|-------|----------");
    for (i, child) in root.borrow().children.iter().enumerate() {
        let child_ref = child.borrow();
        let win_rate = if child_ref.visits > 0 {
            (child_ref.visits as f32 + child_ref.score) / child_ref.visits as f32 / 2.0
        } else {
            0.0
        };
        println!("{:5} | {:6} | {:5.1} | {:8.2}%", 
            i, 
            child_ref.visits, 
            child_ref.score,
            win_rate * 100.0
        );
    }
}

fn roolout(current: &Rc<RefCell<Node>>, all_moves: &Vec<Box<dyn Move>>, rng: &mut ThreadRng, print: bool){
    let mut current_owned = Rc::clone(current);
    loop {
        let n_visits = current_owned.borrow().visits;
        if n_visits == 0 {
            if !expand_tree(&current_owned, all_moves) {
                if print {
                    println!("No valid moves");
                }
                backpropagate(&current_owned, -1.0);
                break;
            }
        }
        let leaf_index = get_expanded_leaf_index(&current_owned, rng);

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

fn expand_tree(current: &Rc<RefCell<Node>>, all_moves: &Vec<Box<dyn Move>>) -> bool {
    // False if there are no valid moves
    let valid_moves: Vec<_> = all_moves
        .iter()
        .filter(|m| m.is_valid(current.borrow().get_game_state()))
        .collect();

    for m in &valid_moves {
        let new_state = m.perform(current.borrow().get_game_state());
        Node::add_child(current, new_state);
    }
    !valid_moves.is_empty()
}

fn get_expanded_leaf_index(node_ref: &Rc<RefCell<Node>>, rng: &mut ThreadRng) -> usize {
    let node = node_ref.borrow();
    let not_visited_indices: Vec<usize> = node.children.iter()
        .enumerate()
        .filter(|(_, c)| c.borrow().visits == 0)
        .map(|(i, _)| i)
        .collect();
    
    if not_visited_indices.is_empty() {
        get_expanded_leaf_index_all_visited(node_ref)
    } else {
        not_visited_indices[rng.random_range(0..not_visited_indices.len())]
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
