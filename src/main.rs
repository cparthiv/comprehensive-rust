fn main() {
    let z = 13;
    // Block
    // Each block has a value and a type which are those of the last expression of the block
    let x = {
        let y = 10;
        println!("y: {y}");
        z - y
    };
    println!("x: {x}");
}