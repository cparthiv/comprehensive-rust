fn main() {
    let a = 10;
    println!("before: {a}");
    // Scope is limited to the enclosing block
    {
        let a = "hello";
        println!("Inner scope: {a}");
        let a = true;
        println!("Shadowed in inner scope: {a}");
    }
    println!("after: {a}")
}