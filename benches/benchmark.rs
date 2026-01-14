use std::hint::black_box;

use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};

use emergency_backpack::{algorithms::dynamic_programming, backpack::Item};

/// Small dataset - 5 items, capacity 11
fn bench_small_dataset(c: &mut Criterion) {
    let items = vec![
        Item::new(String::from("Caderno"), 3, 2),
        Item::new(String::from("Livro"), 4, 3),
        Item::new(String::from("Caneca"), 5, 4),
        Item::new(String::from("Caneta"), 1, 1),
        Item::new(String::from("Garrafa"), 2, 2),
    ];
    let capacity = 11;

    c.bench_function("dp_small_5_items", |b| {
        b.iter(|| dynamic_programming(black_box(&items), black_box(capacity)))
    });
}

/// Medium dataset - 20 items, capacity 50
fn bench_medium_dataset(c: &mut Criterion) {
    let items: Vec<Item> = (0..20)
        .map(|i| {
            Item::new(
                format!("Item_{}", i),
                (i % 10 + 1) as u16, // weight: 1-10
                (i % 15 + 1) as u16, // value: 1-15
            )
        })
        .collect();
    let capacity = 50;

    c.bench_function("dp_medium_20_items", |b| {
        b.iter(|| dynamic_programming(black_box(&items), black_box(capacity)))
    });
}

/// Large dataset - 50 items, capacity 100
fn bench_large_dataset(c: &mut Criterion) {
    let items: Vec<Item> = (0..50)
        .map(|i| {
            Item::new(
                format!("Item_{}", i),
                (i % 20 + 1) as u16, // weight: 1-20
                (i % 30 + 1) as u16, // value: 1-30
            )
        })
        .collect();
    let capacity = 100;

    c.bench_function("dp_large_50_items", |b| {
        b.iter(|| dynamic_programming(black_box(&items), black_box(capacity)))
    });
}

/// Very large dataset - 100 items, capacity 200
fn bench_very_large_dataset(c: &mut Criterion) {
    let items: Vec<Item> = (0..100)
        .map(|i| {
            Item::new(
                format!("Item_{}", i),
                (i % 25 + 1) as u16, // weight: 1-25
                (i % 40 + 1) as u16, // value: 1-40
            )
        })
        .collect();
    let capacity = 200;

    c.bench_function("dp_very_large_100_items", |b| {
        b.iter(|| dynamic_programming(black_box(&items), black_box(capacity)))
    });
}

/// Benchmark with varying number of items (capacity fixed at 100)
fn bench_varying_items(c: &mut Criterion) {
    let mut group = c.benchmark_group("dp_varying_items");

    for num_items in [5, 10, 20, 30, 40, 50].iter() {
        let items: Vec<Item> = (0..*num_items)
            .map(|i| {
                Item::new(
                    format!("Item_{}", i),
                    (i % 15 + 1) as u16,
                    (i % 20 + 1) as u16,
                )
            })
            .collect();
        let capacity = 100;

        group.bench_with_input(BenchmarkId::from_parameter(num_items), num_items, |b, _| {
            b.iter(|| dynamic_programming(black_box(&items), black_box(capacity)))
        });
    }
    group.finish();
}

/// Benchmark with varying capacity (items fixed at 30)
fn bench_varying_capacity(c: &mut Criterion) {
    let mut group = c.benchmark_group("dp_varying_capacity");

    let items: Vec<Item> = (0..30)
        .map(|i| {
            Item::new(
                format!("Item_{}", i),
                (i % 15 + 1) as u16,
                (i % 20 + 1) as u16,
            )
        })
        .collect();

    for capacity in [25, 50, 100, 150, 200, 250].iter() {
        group.bench_with_input(
            BenchmarkId::from_parameter(capacity),
            capacity,
            |b, &cap| b.iter(|| dynamic_programming(black_box(&items), black_box(cap))),
        );
    }
    group.finish();
}

/// Edge case: heavy items (most won't fit)
fn bench_heavy_items(c: &mut Criterion) {
    let items: Vec<Item> = (0..20)
        .map(|i| {
            Item::new(
                format!("Heavy_{}", i),
                (i + 40) as u16, // weight: 40-59 (heavy)
                (i + 10) as u16, // value: 10-29
            )
        })
        .collect();
    let capacity = 100;

    c.bench_function("dp_heavy_items", |b| {
        b.iter(|| dynamic_programming(black_box(&items), black_box(capacity)))
    });
}

/// Edge case: light items (most will fit)
fn bench_light_items(c: &mut Criterion) {
    let items: Vec<Item> = (0..30)
        .map(|i| {
            Item::new(
                format!("Light_{}", i),
                (i % 3 + 1) as u16, // weight: 1-3 (light)
                (i + 5) as u16,     // value: 5-34
            )
        })
        .collect();
    let capacity = 100;

    c.bench_function("dp_light_items", |b| {
        b.iter(|| dynamic_programming(black_box(&items), black_box(capacity)))
    });
}

/// Edge case: uniform items (all same weight and value)
fn bench_uniform_items(c: &mut Criterion) {
    let items: Vec<Item> = (0..25)
        .map(|i| {
            Item::new(
                format!("Uniform_{}", i),
                5,  // all weight 5
                10, // all value 10
            )
        })
        .collect();
    let capacity = 75;

    c.bench_function("dp_uniform_items", |b| {
        b.iter(|| dynamic_programming(black_box(&items), black_box(capacity)))
    });
}

/// Realistic emergency backpack scenario
fn bench_emergency_scenario(c: &mut Criterion) {
    let items = vec![
        Item::new(String::from("Água 1L"), 10, 100),
        Item::new(String::from("Lanterna"), 3, 80),
        Item::new(String::from("Bateria"), 1, 40),
        Item::new(String::from("Kit Primeiros Socorros"), 8, 95),
        Item::new(String::from("Manta Térmica"), 2, 70),
        Item::new(String::from("Apito"), 1, 50),
        Item::new(String::from("Faca Multiuso"), 2, 60),
        Item::new(String::from("Corda 10m"), 5, 55),
        Item::new(String::from("Fósforos"), 1, 45),
        Item::new(String::from("Comida Enlatada"), 6, 75),
        Item::new(String::from("Rádio Portátil"), 4, 65),
        Item::new(String::from("Mapa da Região"), 1, 30),
        Item::new(String::from("Bússola"), 1, 35),
        Item::new(String::from("Protetor Solar"), 2, 25),
        Item::new(String::from("Repelente"), 2, 25),
        Item::new(String::from("Documento de Identidade"), 1, 90),
        Item::new(String::from("Dinheiro"), 1, 85),
        Item::new(String::from("Carregador Portátil"), 3, 70),
        Item::new(String::from("Medicamentos"), 2, 80),
        Item::new(String::from("Cobertor"), 7, 60),
    ];
    let capacity = 50;

    c.bench_function("dp_emergency_realistic", |b| {
        b.iter(|| dynamic_programming(black_box(&items), black_box(capacity)))
    });
}

criterion_group!(
    benches,
    bench_small_dataset,
    bench_medium_dataset,
    bench_large_dataset,
    bench_very_large_dataset,
    bench_varying_items,
    bench_varying_capacity,
    bench_heavy_items,
    bench_light_items,
    bench_uniform_items,
    bench_emergency_scenario,
);

criterion_main!(benches);
