struct Person {
    name: String,
    age: u32,
}

pub fn structs() {
    let person = Person {
        name: String::from("Alice"),
        age: 30,
    };

    println!("Name: {}, Age: {}", person.name, person.age);
}