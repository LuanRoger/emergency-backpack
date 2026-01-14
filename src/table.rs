pub fn init_table(rows: usize, cols: usize) -> Vec<Vec<u16>> {
    let mut table: Vec<Vec<u16>> = Vec::with_capacity(rows);
    for _ in 0..rows {
        let row: Vec<u16> = vec![0; cols];
        table.push(row);
    }
    table
}
