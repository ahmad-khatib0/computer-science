use rand::Rng;

pub fn random_matrix(nrows: usize, ncols: usize, min: i32, max: i32) -> Vec<Vec<i32>> {
    let mut rng = rand::rng();
    let mut m = vec![vec![0; ncols]; nrows];

    (0..nrows).for_each(|i| {
        for j in 0..ncols {
            m[i][j] = rng.random_range(min..=max);
        }
    });
    m
}

pub fn print_matrix(matrix: &Vec<Vec<i32>>) {
    for row in matrix {
        for &val in row {
            print!("{:4}", val);
        }
        println!();
    }
}
