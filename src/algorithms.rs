use crate::knapsack::{Item, Knapsack};
use crate::matrix::init_matrix;

pub fn dynamic_programming(items: &Vec<Item>, capacity: u16) -> Knapsack {
    let rows = items.len();
    let columns = capacity as usize;
    let mut table = init_matrix(rows + 1, columns + 1);

    for i in 1..=rows {
        for w in 1..=columns {
            let current_item = items[i - 1];
            let item_weight = current_item.weight as usize;
            let item_value = current_item.value;

            if item_weight <= w {
                let first = table[i - 1][w];
                let second = table[i - 1][w - item_weight] + item_value;
                let first_or_second_max = first.max(second);

                table[i][w] = first_or_second_max;
            } else {
                table[i][w] = table[i - 1][w];
            }
        }
    }

    let mut knapsack = Knapsack::new(capacity);
    let mut i = rows;
    let mut w = columns;

    while i > 0 && w > 0 {
        if table[i][w] != table[i - 1][w] {
            let current_item = items[i - 1];
            knapsack.add_item(current_item);
            w -= current_item.weight as usize;
        }
        i -= 1;
    }

    knapsack
}
