use crate::card::card::Card;
use crate::card::card_storage::CARD_STORAGE;
use crate::board::rows::row::Row;
use crate::resource::Resource;

const TOTAL_CARDS: usize = 90;
const CARDS_PER_ROW: usize = 4;
const CARD_PARAMS_SIZE: usize = 11;

pub trait StateEncoder: Send + Sync {
    fn encode_row(&self, row: &Row) -> Vec<u8>;
    fn row_encoding_size(&self) -> usize;
    fn clone_box(&self) -> Box<dyn StateEncoder>;
}

pub struct OneHotCardEncoder;

impl OneHotCardEncoder {
    pub fn new() -> Self {
        Self
    }
    fn get_card_index(card: &Card) -> Option<usize> {
        let card_ptr = card as *const Card;
        for i in 0..TOTAL_CARDS {
            let storage_card = CARD_STORAGE.get_card(i);
            let storage_ptr = storage_card as *const Card;
            if card_ptr == storage_ptr {
                return Some(i);
            }
        }
        None
    }
}

impl StateEncoder for OneHotCardEncoder {
    fn encode_row(&self, row: &Row) -> Vec<u8> {
        let mut encoding = vec![0u8; TOTAL_CARDS * CARDS_PER_ROW];
        for position in 0..CARDS_PER_ROW {
            if row.has_card(position) {
                let card = row.get_card(position);
                if let Some(card_index) = Self::get_card_index(card) {
                    let offset = position * TOTAL_CARDS;
                    encoding[offset + card_index] = 1;
                }
            }
        }
        encoding
    }
    fn row_encoding_size(&self) -> usize {
        TOTAL_CARDS * CARDS_PER_ROW
    }
    fn clone_box(&self) -> Box<dyn StateEncoder> {
        Box::new(OneHotCardEncoder)
    }
}

pub struct ParameterEncoder;

impl ParameterEncoder {
    pub fn new() -> Self {
        Self
    }
    fn encode_card(card: Option<&Card>) -> Vec<u8> {
        if let Some(card) = card {
            let mut encoding = Vec::with_capacity(CARD_PARAMS_SIZE);
            encoding.push(card.n_points());
            encoding.push(card.cost().n_green());
            encoding.push(card.cost().n_red());
            encoding.push(card.cost().n_blue());
            encoding.push(card.cost().n_black());
            encoding.push(card.cost().n_white());
            match card.production() {
                Resource::Green => encoding.extend_from_slice(&[1, 0, 0, 0, 0]),
                Resource::Red => encoding.extend_from_slice(&[0, 1, 0, 0, 0]),
                Resource::Blue => encoding.extend_from_slice(&[0, 0, 1, 0, 0]),
                Resource::White => encoding.extend_from_slice(&[0, 0, 0, 1, 0]),
                Resource::Black => encoding.extend_from_slice(&[0, 0, 0, 0, 1]),
            }
            encoding
        } else {
            vec![0u8; CARD_PARAMS_SIZE]
        }
    }
}

impl StateEncoder for ParameterEncoder {
    fn encode_row(&self, row: &Row) -> Vec<u8> {
        let mut encoding = Vec::with_capacity(CARD_PARAMS_SIZE * CARDS_PER_ROW);
        for position in 0..CARDS_PER_ROW {
            if row.has_card(position) {
                let card = row.get_card(position);
                encoding.extend(Self::encode_card(Some(card)));
            } else {
                encoding.extend(Self::encode_card(None));
            }
        }
        encoding
    }
    fn row_encoding_size(&self) -> usize {
        CARD_PARAMS_SIZE * CARDS_PER_ROW
    }
    fn clone_box(&self) -> Box<dyn StateEncoder> {
        Box::new(ParameterEncoder)
    }
}
