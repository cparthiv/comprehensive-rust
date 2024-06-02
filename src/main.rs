// Assignment: transpose rows of matrix into columns
#![allow(unused_variables, dead_code)]
fn main() {
    // 2d array. what is the type?
    // let array = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
    let matrix = [
        [101, 102, 103],
        [201, 202, 203],
        [301, 302, 303],
    ];
    println!("matrix: {:?}", matrix);
    let transposed = transpose(matrix);
    println!("transposed matrix: {:?}", transposed);
}

fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let mut transposed = [[0; 3]; 3];
    for i in 0..3 {
        for j in 0..3 {
            transposed[j][i] = matrix[i][j];
        }
    }
    transposed
}
#[test]
fn test_transpose() {
    let matrix = [
        [101, 102, 103], //
        [201, 202, 203],
        [301, 302, 303],
    ];
    let transposed = transpose(matrix);
    assert_eq!(
        transposed,
        [
            [101, 201, 301], //
            [102, 202, 302],
            [103, 203, 303],
        ]
    );
}