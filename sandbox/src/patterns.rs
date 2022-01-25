// Irrefutable: Pattern that will never fail to match (ex. assigning a var)
// Refutable: Patterns that will fail to match in some cases
// this concept is only important when sene

fn conditional_ex(){
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "24".parse();
    
    if let Some(color) = favorite_color {
        println!("Using your favorite color, {}, as the background", color);
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        // Notice the previous line introduces a shadow var for the Ok variant of age
        // but the variant isnt available until the next scope is open
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }
}

fn while_ex(){
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    // Loops until some doesnt return any value on pop
    // this is matching for a Some pattern
    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
}

fn for_ex(){
    let v = vec!['a', 'b', 'c'];

    // This is matching for an enumerated value pattern
    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }
}

fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}

fn print_ex() {
    // In this ex the point tuple is pattern matched to the functions params
    let point = (3, 5);
    print_coordinates(&point);
}

fn refutable_ex(){
    // The following fails since assignments cannot be made with None in Rust
    // A value must exist and so we need an irrefutable pattern
    // 
    // let Some(x) = some_option_value {}; // where some_option_value can be 5 for ex. 

    // By adding `if` we make it conditionally use the value if returned
    // But if we use an irrefutable pattern it would error
    // 
    // if let Some(x) = 5 {
    //     println!("{}", x);
    // }
}

struct Point {
    x: i32,
    y: i32,
}
enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

fn diff_match_exs(){
    // Multiple patterns "|"
    let x = 1;
    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    // Range match "..="
    let x = 5;
    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }

    let x = 'c';
    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }

    // Destructing
    // 
    // this extracts values from the underlying struct
    let p = Point { x: 0, y: 7 };
    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);
    // shorthand:
    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);
    // matched destruction
    let p = Point { x: 0, y: 7 };
    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }
    // works with nested enums too:
    // let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));
    // match msg {
    //     Message::ChangeColor(Color::Rgb(r, g, b)) => println!(
    //         "Change the color to red {}, green {}, and blue {}",
    //         r, g, b
    //     ),
    //     Message::ChangeColor(Color::Hsv(h, s, v)) => println!(
    //         "Change the color to hue {}, saturation {}, and value {}",
    //         h, s, v
    //     ),
    //     _ => (),
    // }

    // Ignoring Remaining Parts ".." 
    let origin = Point { x: 0, y: 0};
    match origin {
        Point { x, .. } => println!("x is {}", x),
    }

    // Conditional match
    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {}", n),
        _ => println!("Default case, x = {:?}", x),
    }
    println!("at the end: x = {:?}, y = {}", x, y);

    // Capturing Range Binding "@"
    enum Message {
        Hello { id: i32 },
    }

    let msg = Message::Hello { id: 5 };

    match msg {
        Message::Hello {
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {}", id_variable),
        Message::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        Message::Hello { id } => println!("Found some other id: {}", id),
    }
}

pub fn run(){
    conditional_ex();
    while_ex();
    for_ex();
    print_ex();
    refutable_ex();
    diff_match_exs();
}