// Fixed length list where elements are same data type

use std::mem;

pub fn run(){
    let mut habitants: [&str; 3] = ["Spongebob", "Patrick", "Squidward"];

    // reassign values
    habitants[1] = "Mr Crabs";

    println!("{:?}", habitants);

    // get single val
    println!("First inhabitant: {}", habitants[0]);

    // get length
    println!("Array length: {}", habitants.len());

    // Arrays are stack allocated
    println!("Array occupies {} bytes", mem::size_of_val(&habitants));

    // Slice
    let slice: &[&str] = &habitants[0..2];
    println!("Slice: {:?}", slice);
}