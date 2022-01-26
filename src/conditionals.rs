pub fn run(){
    let price: u8 = 5;
    let has_pickles: bool = true;
    let has_secret_sauce = true;

    // if/else
    if price <= 5 && has_pickles || has_secret_sauce{
        println!("One crabby patty please!");
    } else if price <= 5 && has_secret_sauce {
        println!("One crabby patty without pickles please!");
    } else {
        println!("I need to think about it!");
    }

    // shorthand
    let has_ordered = if price <= 5 {true} else {false};
    println!("Has ordered: {}", has_ordered);
}