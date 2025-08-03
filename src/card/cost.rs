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
}