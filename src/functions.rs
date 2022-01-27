pub fn run(){
    greeting("Hello", "Spongebob");

    // bind to value
    let get_sum = add(5, 5);
    println!("Sum: {}", get_sum);

    // closure
    let n3: i32 = 10;
    let add_nums = |n1: i32, n2: i32| n1 + n2 + n3;
    println!("Closure Sum: {}", add_nums(3, 3));

}

fn greeting(greet: &str, name: &str) {
    println!("{} {}", greet, name);
}

// arrow to return something
fn add(n1: i32, n2: i32) -> i32 {
    // no semicolon to tell it to return the line
    n1 + n2
}