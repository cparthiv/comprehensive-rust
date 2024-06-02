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
    assert_eq!(transposed, matrix)
}

fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    unimplemented!()
}