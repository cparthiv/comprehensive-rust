fn main() {
    print_tuple((1, 3))
}

fn print_tuple(tuple: (i32, i32)) {
    // Extracting variables (2 lines)
    // let left = tuple.0;
    // let right = tuple.1;

    // Destructuring tuple (1 line)
    let (left, right) = tuple;
    println!("left: {left}, right: {right}");
}