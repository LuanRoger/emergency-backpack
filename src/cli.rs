use clap::Parser;

use crate::default_dp_exec;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short, long, default_value_t = false)]
    pub no_ui: bool,
}

pub fn cli(capacity: u16) {
    let backpack = default_dp_exec!(capacity);

    let mut total_weight: u16 = 0;
    let mut total_value: u16 = 0;
    backpack.items.iter().for_each(|item| {
        total_weight += item.weight;
        total_value += item.value;
        println!(
            "{} -> (Pesso: {}; Importância: {};)",
            item.name, item.weight, item.value
        );
    });

    println!(
        "Pesso total: {}, Valor total: {}",
        total_weight, total_value
    );
    print!("Pesso permitido: {}", capacity);
}

pub fn get_args() -> Args {
    Args::parse()
}
