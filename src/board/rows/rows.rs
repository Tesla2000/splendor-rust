use rand::prelude::ThreadRng;
use crate::board::rows::card_reference::CardReference;
use crate::board::rows::row::Row;
use crate::card::card::Card;
use crate::card::cost::Cost;
use crate::card::tier::Tier;
use crate::resource::Resource;

#[derive(Clone)]
pub(crate) struct Rows {
    rows: (Row, Row, Row),
}

impl Rows {
    pub(crate) fn new(rng: &mut ThreadRng) -> Rows {
        Rows {
            rows: (
                Row::new(vec![
                    Card::new(Cost::new(1, 1, 1, 1, 0), Resource::Black, 0, Tier::First),
                    Card::new(Cost::new(1, 1, 2, 1, 0), Resource::Black, 0, Tier::First),
                    Card::new(Cost::new(0, 1, 2, 2, 0), Resource::Black, 0, Tier::First),
                    Card::new(Cost::new(1, 3, 0, 0, 1), Resource::Black, 0, Tier::First),
                    Card::new(Cost::new(2, 1, 0, 0, 0), Resource::Black, 0, Tier::First),
                    Card::new(Cost::new(2, 0, 0, 2, 0), Resource::Black, 0, Tier::First),
                    Card::new(Cost::new(3, 0, 0, 0, 0), Resource::Black, 0, Tier::First),
                    Card::new(Cost::new(0, 0, 4, 0, 0), Resource::Black, 1, Tier::First),
                    Card::new(Cost::new(1, 1, 0, 1, 1), Resource::Blue, 0, Tier::First),
                    Card::new(Cost::new(1, 2, 0, 1, 1), Resource::Blue, 0, Tier::First),
                    Card::new(Cost::new(2, 2, 0, 1, 0), Resource::Blue, 0, Tier::First),
                    Card::new(Cost::new(3, 1, 1, 0, 0), Resource::Blue, 0, Tier::First),
                    Card::new(Cost::new(0, 0, 0, 1, 2), Resource::Blue, 0, Tier::First),
                    Card::new(Cost::new(2, 0, 0, 0, 2), Resource::Blue, 0, Tier::First),
                    Card::new(Cost::new(0, 0, 0, 0, 3), Resource::Blue, 0, Tier::First),
                    Card::new(Cost::new(0, 0, 0, 0, 4), Resource::Blue, 1, Tier::First),
                    Card::new(Cost::new(1, 1, 1, 0, 1), Resource::White, 0, Tier::First),
                    Card::new(Cost::new(2, 1, 1, 0, 1), Resource::White, 0, Tier::First),
                    Card::new(Cost::new(2, 0, 2, 0, 1), Resource::White, 0, Tier::First),
                    Card::new(Cost::new(0, 0, 1, 3, 1), Resource::White, 0, Tier::First),
                    Card::new(Cost::new(0, 2, 0, 0, 1), Resource::White, 0, Tier::First),
                    Card::new(Cost::new(0, 0, 0, 2, 2), Resource::White, 0, Tier::First),
                    Card::new(Cost::new(0, 0, 0, 3, 0), Resource::White, 0, Tier::First),
                    Card::new(Cost::new(4, 0, 0, 0, 0), Resource::White, 1, Tier::First),
                    Card::new(Cost::new(0, 1, 1, 1, 1), Resource::Green, 0, Tier::First),
                    Card::new(Cost::new(0, 1, 1, 1, 2), Resource::Green, 0, Tier::First),
                    Card::new(Cost::new(0, 2, 0, 1, 2), Resource::Green, 0, Tier::First),
                    Card::new(Cost::new(1, 0, 3, 1, 0), Resource::Green, 0, Tier::First),
                    Card::new(Cost::new(0, 0, 2, 1, 0), Resource::Green, 0, Tier::First),
                    Card::new(Cost::new(0, 2, 0, 2, 0), Resource::Green, 0, Tier::First),
                    Card::new(Cost::new(0, 3, 0, 0, 0), Resource::Green, 0, Tier::First),
                    Card::new(Cost::new(0, 0, 0, 0, 4), Resource::Green, 1, Tier::First),
                    Card::new(Cost::new(1, 0, 1, 1, 1), Resource::Red, 0, Tier::First),
                    Card::new(Cost::new(1, 0, 2, 1, 1), Resource::Red, 0, Tier::First),
                    Card::new(Cost::new(1, 0, 2, 0, 2), Resource::Red, 0, Tier::First),
                    Card::new(Cost::new(0, 1, 0, 0, 3), Resource::Red, 0, Tier::First),
                    Card::new(Cost::new(1, 0, 0, 2, 0), Resource::Red, 0, Tier::First),
                    Card::new(Cost::new(0, 2, 0, 0, 2), Resource::Red, 0, Tier::First),
                    Card::new(Cost::new(0, 0, 0, 3, 0), Resource::Red, 0, Tier::First),
                    Card::new(Cost::new(0, 0, 0, 4, 0), Resource::Red, 1, Tier::First),
                ], rng),
                Row::new(vec![
                    Card::new(Cost::new(2, 0, 2, 3, 0), Resource::Black, 1, Tier::Second),
                    Card::new(Cost::new(3, 0, 3, 0, 2), Resource::Black, 1, Tier::Second),
                    Card::new(Cost::new(4, 2, 1, 0, 0), Resource::Black, 2, Tier::Second),
                    Card::new(Cost::new(5, 3, 0, 0, 0), Resource::Black, 2, Tier::Second),
                    Card::new(Cost::new(0, 0, 0, 0, 6), Resource::Black, 3, Tier::Second),
                    Card::new(Cost::new(2, 3, 2, 0, 0), Resource::Blue, 1, Tier::Second),
                    Card::new(Cost::new(3, 0, 3, 2, 0), Resource::Blue, 1, Tier::Second),
                    Card::new(Cost::new(0, 0, 3, 5, 0), Resource::Blue, 2, Tier::Second),
                    Card::new(Cost::new(0, 1, 0, 2, 4), Resource::Blue, 2, Tier::Second),
                    Card::new(Cost::new(0, 0, 0, 5, 0), Resource::Blue, 2, Tier::Second),
                    Card::new(Cost::new(0, 0, 0, 6, 0), Resource::Blue, 3, Tier::Second),
                    Card::new(Cost::new(3, 2, 0, 0, 2), Resource::White, 1, Tier::Second),
                    Card::new(Cost::new(0, 3, 0, 2, 3), Resource::White, 1, Tier::Second),
                    Card::new(Cost::new(1, 4, 0, 0, 2), Resource::White, 2, Tier::Second),
                    Card::new(Cost::new(0, 5, 0, 0, 3), Resource::White, 2, Tier::Second),
                    Card::new(Cost::new(0, 5, 0, 0, 0), Resource::White, 2, Tier::Second),
                    Card::new(Cost::new(0, 0, 0, 6, 0), Resource::White, 3, Tier::Second),
                    Card::new(Cost::new(2, 3, 0, 0, 1), Resource::Green, 2, Tier::Second),
                    Card::new(Cost::new(3, 0, 3, 0, 0), Resource::Green, 2, Tier::Second),
                    Card::new(Cost::new(0, 0, 5, 0, 0), Resource::Green, 2, Tier::Second),
                    Card::new(Cost::new(0, 0, 6, 0, 0), Resource::Green, 3, Tier::Second),
                    Card::new(Cost::new(0, 2, 0, 2, 3), Resource::Red, 1, Tier::Second),
                    Card::new(Cost::new(0, 2, 0, 3, 3), Resource::Red, 1, Tier::Second),
                    Card::new(Cost::new(2, 0, 4, 1, 0), Resource::Red, 2, Tier::Second),
                    Card::new(Cost::new(0, 0, 0, 3, 5), Resource::Red, 2, Tier::Second),
                    Card::new(Cost::new(0, 0, 0, 0, 5), Resource::Red, 2, Tier::Second),
                    Card::new(Cost::new(0, 6, 0, 0, 0), Resource::Red, 3, Tier::Second),
                ], rng),
                Row::new(vec![
                    Card::new(Cost::new(5, 3, 3, 3, 0), Resource::Black, 3, Tier::Third),
                    Card::new(Cost::new(0, 7, 0, 0, 0), Resource::Black, 4, Tier::Third),
                    Card::new(Cost::new(3, 6, 0, 0, 3), Resource::Black, 4, Tier::Third),
                    Card::new(Cost::new(0, 7, 0, 0, 3), Resource::Black, 5, Tier::Third),
                    Card::new(Cost::new(3, 3, 3, 0, 5), Resource::Blue, 3, Tier::Third),
                    Card::new(Cost::new(0, 0, 0, 0, 7), Resource::Blue, 4, Tier::Third),
                    Card::new(Cost::new(0, 0, 3, 6, 3), Resource::Blue, 4, Tier::Third),
                    Card::new(Cost::new(0, 0, 0, 0, 7), Resource::Blue, 5, Tier::Third),
                    Card::new(Cost::new(3, 5, 3, 3, 0), Resource::White, 3, Tier::Third),
                    Card::new(Cost::new(0, 0, 0, 7, 0), Resource::White, 4, Tier::Third),
                    Card::new(Cost::new(3, 0, 3, 0, 6), Resource::White, 4, Tier::Third),
                    Card::new(Cost::new(3, 0, 0, 0, 7), Resource::White, 5, Tier::Third),
                    Card::new(Cost::new(0, 3, 3, 5, 3), Resource::Green, 3, Tier::Third),
                    Card::new(Cost::new(0, 0, 0, 7, 0), Resource::Green, 4, Tier::Third),
                    Card::new(Cost::new(3, 0, 3, 6, 0), Resource::Green, 4, Tier::Third),
                    Card::new(Cost::new(3, 0, 3, 7, 0), Resource::Green, 5, Tier::Third),
                    Card::new(Cost::new(3, 0, 3, 5, 3), Resource::Red, 3, Tier::Third),
                    Card::new(Cost::new(7, 0, 0, 0, 0), Resource::Red, 4, Tier::Third),
                    Card::new(Cost::new(6, 3, 0, 3, 0), Resource::Red, 4, Tier::Third),
                    Card::new(Cost::new(7, 3, 0, 0, 0), Resource::Red, 5, Tier::Third),
                ], rng),
            ),
        }
    }
    
    pub fn get_row(&self, index: u8) -> &Row {
        match index {
            0 => {&self.rows.0}
            1 => {&self.rows.1}
            2 => {&self.rows.2}
            _ => {panic!("Invalid index");}
        }
    }
    pub fn replace_row(&self, updated_row: Row, updated_row_index: u8) -> Self {
        match updated_row_index { 
            0 => {Self {rows: (updated_row, self.rows.1.clone(), self.rows.2.clone())}}
            1 => {Self {rows: (self.rows.0.clone(), updated_row, self.rows.2.clone())}}
            2 => {Self {rows: (self.rows.0.clone(), self.rows.1.clone(), updated_row)}}
            _ => {panic!("Invalid index");} 
        }
    }
}
