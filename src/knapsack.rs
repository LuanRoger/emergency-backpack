use std::fmt::Display;

pub struct Item {
    pub weight: u16,
    pub value: u16,
}

pub struct Knapsack {
    pub items: Vec<Item>,
    pub capacity: u16,
}

impl Knapsack {
    pub fn new(capacity: u16) -> Self {
        Knapsack {
            items: Vec::new(),
            capacity,
        }
    }

    pub fn add_item(&mut self, item: Item) {
        self.items.push(item);
    }

    pub fn total_weight(&self) -> u32 {
        self.items
            .iter()
            .fold(0, |agg, value| agg + value.weight as u32)
    }

    pub fn total_value(&self) -> u32 {
        self.items
            .iter()
            .fold(0, |agg, value| agg + value.value as u32)
    }
}

impl Display for Knapsack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "(Peso: {}; Valor: {}) -> CAP: {}",
            self.total_weight(),
            self.total_value(),
            self.capacity
        )
    }
}

impl Item {
    pub fn new(weight: u16, value: u16) -> Self {
        Item { weight, value }
    }
}

impl Clone for Item {
    fn clone(&self) -> Self {
        Self {
            weight: self.weight.clone(),
            value: self.value.clone(),
        }
    }
}

impl Copy for Item {}
