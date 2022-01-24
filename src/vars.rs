// variables hold primitive data or references to data
// immutable by default
// block scoped

pub fn run() {
    let name = "Spongebob";
    // mut makes variable mutable
    let mut age = 22;
    println!("my name is {} and I am {}", name, age);
    age = 23;
    println!("my name is {} and I am {}", name, age);

    // constants - types need to be defined
    const ID: i32 = 00001;
    println!("ID: {}", ID);

    // multiple variables at once
    let (my_name, my_age) = ("Patrick", 27);
    println!("{} is {}", my_name, my_age);

}