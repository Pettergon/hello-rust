/*
primitive types:
Integers: u8, i8, i16, i32, i64, i128
u = can't be negative, i can, both exists for every bitlength (number)
Floats: f32, f64
Boolean: bool
Characters: char
Tuples: ()
Arrays: () - fixed length
*/

// statically typed - needs to know every type at runtime but can infer based on value

pub fn run() {
    // defaults to i32
    let x = 1;

    // defaults to f64
    let y = 3.14;

    // add type
    let z: i64 = 984571472;
    let z2 = 984571472i64;

    // find max size
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);

    // booleans
    let is_sponge: bool = true;

    // get boolean from expression
    let is_greater: bool = 10 <  5;

    // char - single quotes - any unicode
    let a1 = 'a';
    let emoji = '\u{1F600}';


    println!("{:?}", (x, y, z, z2, is_sponge, is_greater, a1, emoji));
}