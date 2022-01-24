pub fn run() {
    // Printing
    println!("Hello from print.rs");

    // Formatting
    println!("{} World", "Hello");

    // postional args
    println!("{0} is a {1} and {0} does {2}", "Spongebob", "sponge", "burgers");

    // named args
    println!("{subject} {verb} {object}", object="a Rustacean", subject="I", verb="am");

    // placeholder
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

    // placeholder for debug
    println!("{:?}", (12, true, "hi"));
    
}