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
    let mut rng = rng();

    let root = Node::new(create_initial_game_state(n_players, &mut rng));
    let valid_moves: Vec<_> = get_all_moves()
        .into_iter()
        .filter(|m| m.is_valid(&root.borrow().get_game_state()))
        .collect();
    let all_moves: Vec<_> = get_all_moves();
    for m in &valid_moves {
        let new_state = m.perform(&root.borrow().get_game_state());
        roolout(&Node::add_child(&root, new_state), &all_moves, &mut rng);
    }
    for _ in 0..100 {
        let expanded_leaf = get_expanded_leaf(&root, &mut rng);
        roolout(expanded_leaf, &all_moves, &mut rng)
    }
    println!("Finished")
}

fn roolout(mut current: &Rc<RefCell<Node>>, all_moves: &Vec<Box<dyn Move>>, rng: &mut ThreadRng){
    loop {
        if current.borrow().visits == 0 {
            if !expand_tree(&current, all_moves) {
                println!("No valid moves");
                backpropagate(&current, -1.0);
                break;
            }
        } else {
            let expanded_child = get_expanded_leaf(&current, rng);
            current = expanded_child;
        }


        let new_current;
        let children_count = current.borrow().children.len();
        if children_count > 0 {
            let random_index = rng.random_range(0..children_count);
            new_current = Rc::clone(&current.borrow().children[random_index]);
        } else {
            break;
        }
        current = &new_current;
        if current.borrow().get_game_state().get_current_player_index() == 0 {
            let max_points = current.borrow().get_game_state().get_players().iter().map(|p| p.get_points()).max().unwrap();
            if max_points < 15 {
                continue;
            }
            if current.borrow().get_game_state().get_current_player().get_points() == max_points {
                println!("Player 0 wins");
                backpropagate(&current, 1.0);
            } else {
                println!("Player 0 looses");
                backpropagate(&current, -1.0);
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

fn get_expanded_leaf(root_ref: &Rc<RefCell<Node>>, rng: &mut ThreadRng) -> &Rc<RefCell<Node>> {
    let root = root_ref.borrow();
    let not_visited_leafs= root.children.iter().filter(|c| c.borrow().visits == 0).collect::<Vec<_>>();
    if not_visited_leafs.is_empty() {
        root.children.iter()
            .max_by(|a, b| Node::ucb1(a).partial_cmp(&Node::ucb1(b)).unwrap())
            .expect("No children")
    } else {
        not_visited_leafs[rng.random_range(0..not_visited_leafs.len())]
    }

}

fn backpropagate(node: &Rc<RefCell<Node>>, is_win: f32) {
    node.borrow_mut().visits += 1;
    node.borrow_mut().wins += is_win;
    
    if let Some(parent) = node.borrow().parent.upgrade() {
        backpropagate(&parent, -is_win);
    }
}
