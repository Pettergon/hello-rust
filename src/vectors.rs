// Resizable Arrays

use std::mem;

pub fn run(){
    let mut habitants: Vec<&str> = vec!["Spongebob", "Patrick", "Squidward"];

    // reassign values
    habitants[1] = "Mr Crabs";

    // Add on to vector
    habitants.push("Patrick");
    habitants.push("Sandy");

    // pop last value
    habitants.pop();

    println!("{:?}", habitants);

    // get single val
    println!("First inhabitant: {}", habitants[0]);

    // get length
    println!("Vector length: {}", habitants.len());

    // Arrays are stack allocated
    println!("Vector occupies {} bytes", mem::size_of_val(&habitants));

    // Slice
    let slice: &[&str] = &habitants[0..2];
    println!("Slice: {:?}", slice);

    // Loop through
    for x in habitants.iter(){
        println!("Habitant: {}", x);
    }

    let mut numbers: Vec<i32> = vec![2, 4, 6, 8, 10];
    // loop and mutate
    for x in numbers.iter_mut(){
        *x *= 2
    }
    println!("Numbers Vec: {:?}", numbers);


}