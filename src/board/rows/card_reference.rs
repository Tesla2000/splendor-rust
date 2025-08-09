use crate::card::card::Card;
use crate::game_state::GameState;
use crate::board::rows::row::Row;

struct CardReference {
    row_index: u8,
    card_index: u8,
}

impl CardReference {
    pub fn seek_in_board(&self, game_state: &GameState) -> &Card {
        let row: &Row;
        match self.row_index { 
            0 => {game_state.get_board().get_rows().get_row(self.row_index)}
            _ => {panic!("Not implemented")} 
        }
        
    }
}