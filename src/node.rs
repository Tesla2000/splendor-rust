use std::cell::RefCell;
use std::rc::{Rc, Weak};
use rand::prelude::SliceRandom;
use rand::Rng;
use crate::game_state::GameState;
use crate::moves::all_moves::get_n_moves;

pub struct Node {
    pub game_state: GameState,
    pub score: f32,
    pub visits: u32,
    pub parent: Weak<RefCell<Node>>,
    pub children: Vec<Rc<RefCell<Node>>>,
    move_order: Vec<usize>,
    pub next_move_to_perform: usize,
}
impl Node {
    pub fn new<R: Rng>(game_state: GameState, rng: &mut R) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            game_state,
            score: 0.0,
            visits: 0,
            parent: Weak::new(),
            children: Vec::new(),
            move_order: Node::random_move_order(rng),
            next_move_to_perform: 0,
        }))
    }

    /// Add a child node to this node
    pub fn add_child<R: Rng>(
        parent: &Rc<RefCell<Node>>,
        game_state: GameState, rng: &mut R,
    ) -> Rc<RefCell<Node>> {
        let child = Rc::new(RefCell::new(Node {
            game_state,
            score: 0.0,
            visits: 0,
            parent: Rc::downgrade(parent),
            children: Vec::new(),
            move_order: Node::random_move_order(rng),
            next_move_to_perform: 0,
        }));
        parent.borrow_mut().children.push(Rc::clone(&child));
        child
    }

    fn random_move_order<R: Rng>(rng: &mut R) -> Vec<usize> {
        let mut move_order: Vec<usize> = (0..get_n_moves()).collect();
        move_order.shuffle(rng);
        move_order
    }

    pub fn move_order(&self) -> &Vec<usize> {
        &self.move_order
    }

    pub fn get_game_state(&self) -> &GameState {
        &self.game_state
    }

    pub fn ucb1(child_ref: &Rc<RefCell<Node>>) -> f32 {
        let child = child_ref.borrow();
        if child.visits == 0 {
            panic!("UCB1 called on unvisited node");
        }
        if let Some(parent) = child.parent.upgrade() {
            child.score / child.visits as f32 + 2.0 * ((parent.borrow().visits as f32).ln() / child.visits as f32).sqrt()
        } else { 
            panic!("Parent node not found");
        }
    }
}