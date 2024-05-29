fn main() {
    let mut i = 0;
    loop {
        i += 1;
        // Exit out of loop
        if i > 5 { break; }
        // Start next iteration immediately
        if i % 2 == 0 { continue; }
        println!("{}", i)
    }
}