pub fn init_matrix(rows: usize, cols: usize) -> Vec<Vec<u16>> {
    let mut table: Vec<Vec<u16>> = Vec::with_capacity(rows);
    for _ in 0..rows {
        let mut row: Vec<u16> = Vec::with_capacity(cols);
        row.resize(cols, 0);
        table.push(row);
    }
    table
}
