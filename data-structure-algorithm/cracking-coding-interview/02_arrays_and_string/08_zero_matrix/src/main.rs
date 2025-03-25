//
//
// 1.8 Zero Matrix: Write an algorithm such that if an element in an
// MxN matrix is 0, its entire row and column are set to 0
// Hints #17, #74, #702
//

use rust_lib::assort_methods::{print_matrix, random_matrix};

mod common;
mod question1;
mod question2;

fn main() {
    let nrows = 4;
    let ncols = 4;
    let mut matrix = random_matrix(nrows, ncols, -10, 10);

    print_matrix(&matrix);

    question1::zero_matrix(&mut matrix);
    question2::zero_matrix(&mut matrix);
    println!();

    print_matrix(&matrix);
}

#[cfg(test)]
mod test {
    use crate::common::clone_matrix;

    use super::*;

    #[test]
    fn test_zero_matrix() {
        let nrows = 5;
        let ncols = 5;
        let mut m1 = random_matrix(nrows, ncols, -10, 10);
        let mut m2 = clone_matrix(&m1);

        print_matrix(&m1);

        question1::zero_matrix(&mut m1);
        question2::zero_matrix(&mut m2);
        println!();

        assert!(common::matrices_are_equal(&m1, &m2));
    }
}
