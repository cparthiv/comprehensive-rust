fn main() {
    println!("gcd: {}", gcd(143, 52))
}

fn gcd(p0: u32, p1: u32) -> u32 {
    if p1 > 0 {
        gcd(p1, p0 % p1)
    } else {
        p0
    }
}
