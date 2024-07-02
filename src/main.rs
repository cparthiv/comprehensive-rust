struct Person {
    name: String,
    age: u8,
}

fn describe(person: &Person){
    println!("{} is {} years old!", person.name, person.age)
}
fn main() {
    let mut peter = Person {
        name: String::from("Peter"),
        age: 20
    };
    peter.age = 28;
    describe(&peter)
}