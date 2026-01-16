use std::fmt::Display;

use crate::table::Table;

pub struct Item {
    pub name: String,
    pub weight: u16,
    pub value: u16,
}

pub struct Backpack {
    pub items: Vec<Item>,
    pub capacity: u16,
    pub table: Table,
}

impl Backpack {
    pub fn new(capacity: u16, items: Vec<Item>, table: Table) -> Self {
        Backpack {
            items,
            capacity,
            table,
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

impl Display for Backpack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let items_display: String = self
            .items
            .iter()
            .fold(String::new(), |agg, value| agg + &value.name + ", ");

        write!(
            f,
            "(Peso: {}; Valor: {}; CAP: {}) -> {}",
            self.total_weight(),
            self.total_value(),
            self.capacity,
            items_display
        )
    }
}

impl Item {
    pub fn new(name: String, weight: u16, value: u16) -> Self {
        Item {
            name,
            weight,
            value,
        }
    }
}

impl Clone for Item {
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            weight: self.weight.clone(),
            value: self.value.clone(),
        }
    }
}
