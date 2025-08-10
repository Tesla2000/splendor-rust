use crate::card::cost::Cost;
#[derive(Clone)]
pub struct Resources {
    n_green: u8,
    n_red: u8,
    n_blue: u8,
    n_white: u8,
    n_black: u8,
    n_gold: u8,
}

impl Resources {
    pub const fn new(n_green: u8, n_red: u8, n_blue: u8, n_white: u8, n_black: u8, n_gold: u8) -> Self {
        Self {
            n_green,
            n_red,
            n_blue,
            n_white,
            n_black,
            n_gold,
        }
    }
    pub fn can_pay(&self, cost: &Cost) -> bool {
        self.get_n_missing_resources(cost) <= self.n_gold
    }
    fn get_n_missing_resources(&self, cost: &Cost) -> u8 {
        cost.n_green().saturating_sub(self.n_green)
            + cost.n_red().saturating_sub(self.n_red)
            + cost.n_blue().saturating_sub(self.n_blue)
            + cost.n_black().saturating_sub(self.n_black)
            + cost.n_white().saturating_sub(self.n_white)
    }



    pub fn add(&self, other: &Self) -> Self {
        Self {
            n_green: self.n_green + other.n_green,
            n_red: self.n_red + other.n_red,
            n_blue: self.n_blue + other.n_blue,
            n_black: self.n_black + other.n_black,
            n_white: self.n_white + other.n_white,
            n_gold: self.n_gold + other.n_gold,
        }
    }
    
    pub fn sum(&self) -> u8 {
        self.n_green + self.n_red + self.n_blue + self.n_white + self.n_black + self.n_gold
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
    pub fn n_gold(&self) -> u8 {
        self.n_gold
    }
    
    pub fn to_builder(&self) -> ResourcesBuilder {
        ResourcesBuilder::new(self)
    }
}

#[derive(Default)]
pub(crate) struct ResourcesBuilder {
    pub n_green: u8,
    pub n_red: u8,
    pub n_blue: u8,
    pub n_white: u8,
    pub n_black: u8,
    pub n_gold: u8,
}

impl ResourcesBuilder {
    fn new(resources: &Resources) -> Self {
        Self {
            n_green: resources.n_green,
            n_red: resources.n_red,
            n_blue: resources.n_blue,
            n_white: resources.n_white,
            n_black: resources.n_black,
            n_gold: resources.n_gold,
        }
    }

    pub fn build(self) -> Resources {
        Resources {
            n_green: self.n_green,
            n_red: self.n_red,
            n_blue: self.n_blue,
            n_white: self.n_white,
            n_black: self.n_black,
            n_gold: self.n_gold,
        }
    }

    pub fn pay_cost(&mut self, cost: &Cost) {
        // Assuming can pay
        let gold: u8 = self.n_gold - self.get_n_missing_resources(cost);
        self.n_green = self.n_green.saturating_sub(cost.n_green());
        self.n_red = self.n_red.saturating_sub(cost.n_red());
        self.n_blue = self.n_blue.saturating_sub(cost.n_blue());
        self.n_white = self.n_green.saturating_sub(cost.n_white());
        self.n_black = self.n_green.saturating_sub(cost.n_black());
        self.n_gold = gold;
    }

    pub fn add(&mut self, other: &Self) {
        self.n_green += other.n_green;
        self.n_red += other.n_green;
        self.n_blue += other.n_blue;
        self.n_black += other.n_black;
        self.n_white += other.n_white;
        self.n_gold += other.n_gold;
    }

    fn get_n_missing_resources(&self, cost: &Cost) -> u8 {
        cost.n_green().saturating_sub(self.n_green)
            + cost.n_red().saturating_sub(self.n_red)
            + cost.n_blue().saturating_sub(self.n_blue)
            + cost.n_black().saturating_sub(self.n_black)
            + cost.n_white().saturating_sub(self.n_white)
    }
}
