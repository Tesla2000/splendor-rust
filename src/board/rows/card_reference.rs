use crate::board::board::Board;
use crate::card::card::Card;

pub(crate) struct CardReference {
    row_index: u8,
    card_index: usize,
}

impl CardReference {
    pub fn get_from_board<'a>(&self, board: &'a Board) -> &'a Card {
        board.get_rows().get_row(self.row_index).get_card(self.card_index)
    }
    
    pub fn is_in_board(&self, board: &Board) -> bool {
        board.get_rows().get_row(self.row_index).has_card(self.card_index)
    }
    
    pub fn get_row_index(&self) -> u8 {
        self.row_index
    }
    pub fn get_card_index(&self) -> usize {
        self.card_index
    }
}