// types that have few definitve values

enum Movement{
    // variants
    UP,
    DOWN,
    LEFT,
    RIGHT
}

fn move_avatar(m: Movement){
    // Perform action depending on info
    match m{
        Movement::UP => println!("Going up"),
        Movement::DOWN => println!("Going down"),
        Movement::LEFT => println!("Going left"),
        Movement::RIGHT => println!("Going right"),
    }
}

pub fn run(){
    let avatar1 = Movement::LEFT;
    let avatar2 = Movement::UP;
    let avatar3 = Movement::DOWN;
    let avatar4 = Movement::RIGHT;

    move_avatar(avatar1);
    move_avatar(avatar2);
    move_avatar(avatar3);
    move_avatar(avatar4);
}