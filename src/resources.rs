use crate::card::cost::Cost;
use std::cmp::max;
use getset::Getters;
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
        self._get_n_missing_resources(cost) <= self.n_gold
    }
    pub fn _get_n_missing_resources(&self, cost: &Cost) -> u8 {
        max(0, cost.n_green() - self.n_green)
            + max(0, cost.n_red() - self.n_red)
            + max(0, cost.n_blue() - self.n_blue)
            + max(0, cost.n_black() - self.n_black)
            + max(0, cost.n_white() - self.n_white)
    }
    pub fn pay_cost(&self, cost: &Cost) -> Self {
        // Assuming can pay
        let gold: u8 = self.n_gold - self._get_n_missing_resources(cost);
        if gold < 0 {
            panic!("Not enough gold to pay for this card");
        }
        Self {
            n_green: max(0, cost.n_green() - self.n_green),
            n_red: max(0, cost.n_red() - self.n_red),
            n_blue: max(0, cost.n_blue()- self.n_blue),
            n_black: max(0, cost.n_black() - self.n_black),
            n_white: max(0, cost.n_white() - self.n_white),
            n_gold: gold,
        }
    }
    pub fn remove_gold(&self) -> Self {
        if self.n_gold < 1 {
            panic!("Not enough gold to remove");
        }
        Self {
            n_green: self.n_green,
            n_red: self.n_red,
            n_blue: self.n_blue,
            n_black: self.n_black,
            n_white: self.n_white,
            n_gold: self.n_gold - 1,
        }
    }
    
    pub fn sum(&self) -> u8 {
        self.n_green + self.n_red + self.n_blue + self.n_white + self.n_black + self.n_gold
    }
    pub fn add(&self, other: &Self) -> Self {
        Self {
            n_green: other.n_green + self.n_green,
            n_red: other.n_red + self.n_red,
            n_blue: other.n_blue + self.n_blue,
            n_black: other.n_black + self.n_black,
            n_white: other.n_white + self.n_white,
            n_gold: other.n_gold + self.n_gold,
        }
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
}

#[derive(Default)]
pub struct ResourcesBuilder {
    pub n_green: u8,
    pub n_red: u8,
    pub n_blue: u8,
    pub n_white: u8,
    pub n_black: u8,
}
