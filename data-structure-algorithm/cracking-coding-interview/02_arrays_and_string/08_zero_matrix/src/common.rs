pub fn nullify_row(matrix: &mut [Vec<i32>], row: usize) {
    for i in 0..matrix[0].len() {
        matrix[row][i] = 0;
    }
}

pub fn nullify_column(matrix: &mut [Vec<i32>], col: usize) {
    (0..matrix.len()).for_each(|i| {
        matrix[i][col] = 0;
    });
}

pub fn matrices_are_equal(m1: &[Vec<i32>], m2: &[Vec<i32>]) -> bool {
    if m1.len() != m2.len() || m1[0].len() != m2[0].len() {
        return false;
    }

    for k in 0..m1.len() {
        for j in 0..m1[0].len() {
            if m1[k][j] != m2[k][j] {
                return false;
            }
        }
    }

    true
}

pub fn clone_matrix(m: &[Vec<i32>]) -> Vec<Vec<i32>> {
    let mut c = vec![vec![0; m[0].len()]; m.len()];
    for i in 0..m.len() {
        for j in 0..m[0].len() {
            c[i][j] = m[i][j];
        }
    }
    c
}
