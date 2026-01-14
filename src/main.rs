use emergency_backpack::{algorithms::dynamic_programming, backpack::Item};

const BACKPACK_CAPACITY: u16 = 6;

fn main() {
    let items = vec![
        Item::new(String::from("Caderno"), 3, 2),
        Item::new(String::from("Livro"), 4, 3),
        Item::new(String::from("Caneca"), 5, 4),
        Item::new(String::from("Caneta"), 1, 1),
        Item::new(String::from("Garrafa"), 2, 2),
    ];

    let backpack = dynamic_programming(&items, BACKPACK_CAPACITY);
    println!("{}", backpack);
}
