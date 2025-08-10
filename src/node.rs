use std::cell::RefCell;
use std::rc::{Rc, Weak};
use crate::game_state::GameState;

pub struct Node {
    pub game_state: GameState,
    pub wins: u32,
    pub visits: u32,
    pub parent: Weak<RefCell<Node>>,
    pub children: Vec<Rc<RefCell<Node>>>,
}

impl Node {
    pub fn new(game_state: GameState) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            game_state,
            wins: 0,
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
            wins: 0,
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
}