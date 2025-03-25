use crate::common::{nullify_column, nullify_row};

// We can reduce the space to 0 ( 1 ) by using the first row as a replacement for the row
// array and the first colu m n as a replacement for the column a rray. Th is works as follows:
//
pub fn zero_matrix(m: &mut [Vec<i32>]) {
    let mut row_has_zero = false;
    let mut col_has_zero = false;

    // Check if first row has a zero
    for i in 0..m[0].len() {
        if m[0][i] == 0 {
            row_has_zero = true;
            break;
        }
    }

    // Check if first column has a zero
    (0..m.len()).for_each(|i| {
        if m[i][0] == 0 {
            col_has_zero = true;
        }
    });

    // Check for zeros in the rest of the array
    for i in 1..m.len() {
        for j in 1..m[0].len() {
            if m[i][j] == 0 {
                m[i][0] = 0;
                m[0][j] = 0;
            }
        }
    }

    // Nullify rows based on values in first column
    for i in 1..m.len() {
        if m[i][0] == 0 {
            nullify_row(m, i);
        }
    }

    // Nullify columns based on values in first row
    for j in 1..m[0].len() {
        if m[0][j] == 0 {
            nullify_column(m, j);
        }
    }

    // Nullify first row if needed
    if row_has_zero {
        nullify_row(m, 0);
    }

    // Nullify first column if needed
    if col_has_zero {
        nullify_column(m, 0);
    }
}
