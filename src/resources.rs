pub struct Resources {
    n_green: u8,
    n_red: u8,
    n_blue: u8,
    n_white: u8,
    n_black: u8,
    n_gold: u8,
}

impl Resources {
    pub fn new(
        n_green: u8,
        n_red: u8,
        n_blue: u8,
        n_white: u8,
        n_black: u8,
        n_gold: u8,
    ) -> Self {
        Self {
            n_green,
            n_red,
            n_blue,
            n_white,
            n_black,
            n_gold,
        }
    }
}

#[derive(Default)]
pub struct ResourcesBuilder {
    pub n_green: u8,
    pub n_red: u8,
    pub n_blue: u8,
    pub n_white: u8,
    pub n_black: u8,
}
