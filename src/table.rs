use std::ops::{Deref, DerefMut};

pub struct Table(Vec<Vec<u16>>);

impl Table {
    pub fn new(capacity: usize) -> Self {
        Self(Vec::with_capacity(capacity))
    }
}

impl Deref for Table {
    type Target = Vec<Vec<u16>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Table {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub fn init_table(rows: usize, cols: usize) -> Table {
    let mut table: Table = Table::new(rows);
    for _ in 0..rows {
        let row: Vec<u16> = vec![0; cols];
        table.push(row);
    }
    table
}
