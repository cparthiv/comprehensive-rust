fn factorial(number: u32) -> u32 {
    let mut product = 1;
    for i in 1..=number {
        product *= dbg!(i);
    }
    product
}

fn in_progress(number: u32) -> u32 {
    todo!()
}

fn main() {
    let n = 4;
    println!("{n}! = {}", factorial(n))
}