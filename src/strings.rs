// primitive str = immutable and has a fixed length
// String = growable, data structure

pub fn run(){
    // defaults to str
    let hello = "Hello";

    // String
    let mut hi = String::from("Hi ");

    // get length - usable on both types
    println!("Length: {}", hello.len());

    // all from now only for String

    // add a char
    hi.push('S');

    // adds a string of any length
    hi.push_str("quidward");

    // capacity in bytes (memory usage)
    println!("Capactiy: {}", hi.capacity());

    // is empty
    println!("Is Empty: {}", hi.is_empty());

    // contains
    println!("Contains 'Squid': {}", hi.contains("Squid"));

    // replace
    println!("Replace: {}", hi.replace("Squidward", "Patrick"));

    // loop through String by whitespace
    for token in hi.split_whitespace(){
        println!("{}", token);
    }

    // create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    // assertion testing
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());

    println!("{}", s);

    println!("{:?}", (hello, hi));
}