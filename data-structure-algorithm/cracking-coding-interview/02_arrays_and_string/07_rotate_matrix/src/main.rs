//
// 1.7 Rotate Matrix: Given an image represented by an NxN matrix, where each
// pixel in the image is 4 bytes, write a method to rotate the image by 90 degrees.
// Can you do this in place?          Hints: #51, #100
//

fn rotate(matrix: &mut [Vec<i32>]) -> bool {
    let n = matrix.len();
    if n == 0 || n != matrix[0].len() {
        return false; // not a squared matrix
    }

    // we could start from the inner layer and work outwards.)
    for layer in 0..n / 2 {
        let first = layer;
        let last = n - 1 - layer;

        for i in first..last {
            let offset = i - first;
            let top = matrix[first][i];

            // left -> top
            matrix[first][i] = matrix[last - offset][first];
            // bottom -> left
            matrix[last - offset][first] = matrix[last][last - offset];
            // right -> bottom
            matrix[last][last - offset] = matrix[i][last];
            // top -> right
            matrix[i][last] = top; // right <- saved top
        }
    }
    true
}

fn print_matrix(matrix: &[Vec<i32>]) {
    for row in matrix {
        for val in row {
            print!("{} ", val);
        }
        println!();
    }
}

fn main() {
    let mut matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    let mut matrix2 = vec![
        vec![1, 2, 3, 4],
        vec![5, 6, 7, 8],
        vec![9, 10, 11, 12],
        vec![13, 14, 15, 16],
    ];

    println!("Original matrix:");
    print_matrix(&matrix);
    rotate(&mut matrix);
    println!("\nRotated matrix:");
    print_matrix(&matrix);

    println!("Original matrix:");
    print_matrix(&matrix2);
    rotate(&mut matrix2);
    println!("\nRotated matrix:");
    print_matrix(&matrix2);
}
