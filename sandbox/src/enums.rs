// Enums are types which have a few definite values

enum Movement {
    // Variants
    Up, Down, Left, Right, ZAxis
}

fn move_avatar(m: Movement){
    // Perform action depending on info
    match m {
        Movement::Up => println!("Avatar moving up"),
        Movement::Down => println!("Avatar moving down"),
        Movement::Left => println!("Avatar moving left"),
        Movement::Right => println!("Avatar moving right"),
        _ => () // Do nothing in catchall case
    }
}

//// Another example using multiple types within enum + impl
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}

//// Example of enum with match and unique quarters
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

// NOTE: Matches in Rust are exhaustive; if we didnt include None case this would bug out
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
        other => None, // This is a catch-all but the pattern will never be reached
        _ => None, // This is the same as 'other' except doesnt use the val
    }
}

pub fn run (){
    let avatar1 = Movement::Left;
    let avatar2 = Movement::Up;
    let avatar3 = Movement::Right;
    let avatar4 = Movement::Down;

    move_avatar(avatar1);
    move_avatar(avatar2);
    move_avatar(avatar3);
    move_avatar(avatar4);

    let m = Message::Write(String::from("hello"));
    m.call();

    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    // Note this following line fails because of the Optional
    // rust forces us to make Optional into <T> and handle all cases
    // where it could be a None
    // let sum = x + y;

    value_in_cents(Coin::Dime);
    value_in_cents(Coin::Quarter(UsState::Alaska)); // Must pass in state for quarters

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    // `if let` is a non exhaustive pattern match
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }
}