use once_cell::sync::Lazy;
use crate::aristocrat::Aristocrat;
use crate::card::cost::Cost;

pub struct AristocratStorage {
    aristocrats: Vec<Aristocrat>,
}

impl AristocratStorage {
    fn new() -> Self {
        let aristocrats = vec![
            Aristocrat::new(Cost::new(0, 4, 0, 0, 4)),
            Aristocrat::new(Cost::new(0, 3, 0, 3, 3)),
            Aristocrat::new(Cost::new(0, 0, 4, 4, 0)),
            Aristocrat::new(Cost::new(0, 0, 0, 4, 4)),
            Aristocrat::new(Cost::new(0, 0, 4, 0, 4)),
            Aristocrat::new(Cost::new(0, 3, 3, 3, 0)),
            Aristocrat::new(Cost::new(0, 3, 3, 3, 3)),
            Aristocrat::new(Cost::new(0, 0, 0, 4, 4)),
            Aristocrat::new(Cost::new(0, 0, 3, 3, 3)),
            Aristocrat::new(Cost::new(0, 3, 3, 0, 3)),
        ];
        
        Self { aristocrats }
    }
    
    pub fn get_aristocrat(&self, index: usize) -> &Aristocrat {
        &self.aristocrats[index]
    }
    

    pub fn len(&self) -> usize {
        self.aristocrats.len()
    }
}

pub static ARISTOCRAT_STORAGE: Lazy<AristocratStorage> = Lazy::new(AristocratStorage::new);