// create custom data types
// structs get upper case

// traditional struct
struct Color{
    red: u8,
    blue: u8,
    green: u8
}

struct Person{
    first_name: String,
    last_name: String
}


// add function to struct
impl Person{
    // contructor
    fn new(first: &str, last: &str) -> Person {
        Person{
            first_name: first.to_string(),
            last_name: last.to_string(),
        }
    }

    // get fullname &self = this
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    // set last name
    fn set_last_name(&mut self, last: &str) {
        self.last_name = last.to_string();
    }

    // name to tuple no & because no copy
    fn to_tuple(self) -> (String, String){
        (self.first_name, self.last_name)
    }
}

// tuple struct
struct Color2(u8, u8, u8);

pub fn run() {
    let mut c = Color {
        red: 255,
        green: 0,
        blue: 0
    };

    c.blue = 200;

    println!("Color: {} {} {}", c.red, c.green, c.blue);

    let mut c2 = Color2(255,0,0);

    c2.0 = 200;

    println!("Color: {} {} {}", c2.0, c2.1, c2.2);


    let mut p = Person::new("Spongebob", "Star");
    p.set_last_name("Squarepants");
    println!("Person: {}", p.full_name());
    println!("Person: {:?}", p.to_tuple());
}

