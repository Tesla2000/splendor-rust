use std::cell::RefCell;
use std::process::Child;
use std::rc::{Rc, Weak};
use crate::game_state::GameState;

pub struct Node {
    pub game_state: GameState,
    pub wins: f32,
    pub visits: u32,
    pub parent: Weak<RefCell<Node>>,
    pub children: Vec<Rc<RefCell<Node>>>,
}

impl Node {
    pub fn new(game_state: GameState) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            game_state,
            wins: 0.0,
            visits: 0,
            parent: Weak::new(),
            children: Vec::new(),
        }))
    }

    /// Add a child node to this node
    pub fn add_child(
        parent: &Rc<RefCell<Node>>,
        game_state: GameState,
    ) -> Rc<RefCell<Node>> {
        let child = Rc::new(RefCell::new(Node {
            game_state,
            wins: 0.0,
            visits: 0,
            parent: Rc::downgrade(parent),
            children: Vec::new(),
        }));
        parent.borrow_mut().children.push(Rc::clone(&child));
        child
    }

    pub fn get_game_state(&self) -> &GameState {
        &self.game_state
    }

    pub fn ucb1(child_ref: &Rc<RefCell<Node>>) -> f32 {
        let child = child_ref.borrow();
        if let Some(parent) = child.parent.upgrade() {
            child.wins / child.visits as f32 + 2.0 * ((parent.borrow().visits as f32).ln() / child.visits as f32).sqrt()
        } else { 
            panic!("Parent node not found");
        }
    }
}