use crate::common::{nullify_column, nullify_row};

pub fn zero_matrix(m: &mut [Vec<i32>]) {
    let mut row = vec![false; m.len()];
    let mut column = vec![false; m[0].len()];

    // Store the row and column index with value 0
    for i in 0..m.len() {
        (0..m[0].len()).for_each(|j| {
            if m[i][j] == 0 {
                row[i] = true;
                column[j] = true;
            }
        });
    }

    (0..row.len()).for_each(|i| {
        if row[i] {
            nullify_row(m, i);
        }
    });

    (0..column.len()).for_each(|i| {
        if column[i] {
            nullify_column(m, i)
        }
    });
}
