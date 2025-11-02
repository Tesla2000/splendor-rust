use once_cell::sync::Lazy;
use crate::moves::get_three::GetThree;
use crate::moves::get_two::GetTwo;
use crate::moves::reserve::Reserve;
use crate::moves::build_card::BuildCard;
use crate::moves::build_from_reserve::BuildFromReserve;
use crate::moves::move_trait::Move;
use crate::resource::Resource;
use crate::board::rows::card_reference::CardReference;
use crate::moves::reserve_from_hidden::ReserveFromHidden;

pub struct AllMoves {
    moves: Vec<Box<dyn Move>>,
}

impl AllMoves {
    fn new() -> Self {
        let mut moves: Vec<Box<dyn Move>> = Vec::new();
        
        // Get 3 resources - all unique combinations (except gold)
        // There are C(5,3) = 10 combinations
        moves.push(Box::new(GetThree::new(Resource::Green, Resource::Blue, Resource::Red)));
        moves.push(Box::new(GetThree::new(Resource::Green, Resource::Blue, Resource::White)));
        moves.push(Box::new(GetThree::new(Resource::Green, Resource::Blue, Resource::Black)));
        moves.push(Box::new(GetThree::new(Resource::Green, Resource::Red, Resource::White)));
        moves.push(Box::new(GetThree::new(Resource::Green, Resource::Red, Resource::Black)));
        moves.push(Box::new(GetThree::new(Resource::Green, Resource::White, Resource::Black)));
        moves.push(Box::new(GetThree::new(Resource::Blue, Resource::Red, Resource::White)));
        moves.push(Box::new(GetThree::new(Resource::Blue, Resource::Red, Resource::Black)));
        moves.push(Box::new(GetThree::new(Resource::Blue, Resource::White, Resource::Black)));
        moves.push(Box::new(GetThree::new(Resource::Red, Resource::White, Resource::Black)));
        
        // Get 2 resources of each type (except gold)
        moves.push(Box::new(GetTwo::new(Resource::Green)));
        moves.push(Box::new(GetTwo::new(Resource::Blue)));
        moves.push(Box::new(GetTwo::new(Resource::Red)));
        moves.push(Box::new(GetTwo::new(Resource::White)));
        moves.push(Box::new(GetTwo::new(Resource::Black)));
        
        // Reserve any card - tier index 0-2, card index 0-3
        for tier_index in 0..3 {
            for card_index in 0..4 {
                moves.push(Box::new(Reserve::new(CardReference::new(tier_index, card_index))));
                moves.push(Box::new(BuildCard::new(CardReference::new(tier_index, card_index))));
            }
        }
        
        // Reserve top - index 0-2
        for index in 0..3 {
            moves.push(Box::new(ReserveFromHidden::new(index)));
        }
        // Build from reserve - index 0-3
        for index in 0..3 {
            moves.push(Box::new(BuildFromReserve::new(index)));
        }
        Self { moves }
    }
    
    pub fn get_all(&self) -> &Vec<Box<dyn Move>> {
        &self.moves
    }
    

}

unsafe impl Sync for AllMoves {}
unsafe impl Send for AllMoves {}

static ALL_MOVES: Lazy<AllMoves> = Lazy::new(AllMoves::new);

pub fn get_all_moves() -> &'static Vec<Box<dyn Move>> {
    ALL_MOVES.get_all()
}

