use crate::resources::Resources;

#[derive(Clone)]
pub struct Cost {
    n_green: u8,
    n_red: u8,
    n_blue: u8,
    n_white: u8,
    n_black: u8,
}

impl Cost {
    pub(crate) fn new(
        n_green: u8,
        n_red: u8,
        n_blue: u8,
        n_white: u8,
        n_black: u8,
    ) -> Self {
        Self {
            n_green,
            n_red,
            n_blue,
            n_white,
            n_black,       
        }
    }

    pub fn to_resources(&self) -> Resources {
        Resources::new(
            self.n_green,
            self.n_red,
            self.n_blue,
            self.n_white,
            self.n_black,
            0,
        )
    }

    pub fn n_green(&self) -> u8 {
        self.n_green
    }
    pub fn n_red(&self) -> u8 {
        self.n_red
    }
    pub fn n_blue(&self) -> u8 {
        self.n_blue
    }
    pub fn n_white(&self) -> u8 {
        self.n_white
    }
    pub fn n_black(&self) -> u8 {
        self.n_black
    }
}