fn main() {
    let x = 10;
    if x == 0 {
        println!("X is equal to zero")
    } else if x < 100 {
        println!("X is a reasonably sized number")
    } else {
        println!("X is a huge number")
    }

    let size = if x < 20 { "small" } else { "large" };
    println!("number size: {}", size)
}