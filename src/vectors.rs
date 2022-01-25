// Resizable Arrays

use std::mem;

pub fn run(){
    let mut habitants: Vec<&str> = vec!["Spongebob", "Patrick", "Squidward"];

    // reassign values
    habitants[1] = "Mr Crabs";

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
}