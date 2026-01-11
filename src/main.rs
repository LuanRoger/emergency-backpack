use std::vec;

use crate::{algorithms::dynamic_programming, knapsack::Item};

mod algorithms;
mod knapsack;
mod matrix;

const BACKPACK_CAPACITY: u16 = 6;

fn main() {
    let items = vec![Item::new(2, 3), Item::new(3, 4), Item::new(4, 5)];

    let backpack = dynamic_programming(&items, BACKPACK_CAPACITY);
    println!("{}", backpack);
}
