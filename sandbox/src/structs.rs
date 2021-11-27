struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

struct Color2(u8,u8,u8);

struct Person {
    first_name: String,
    last_name: String
}

impl Person {
    // Construct person
    fn new(first:&str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string()
        }
    }

    // Get full name 
    fn full_name(&self) -> String {
        // Having no semicolon here makes this and expression == return
        // With a semicolon this would be a statement == no return 
        format!("{} {}", self.first_name, self.last_name) 
    }
    
    // Set last name
    fn set_last_name(&mut self, last: &str) {
        self.last_name = last.to_string();
    }
    
    // Name to tuple
    fn to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }

    // This is an associated func; like a method but doesnt need ref to self
    fn print_mr() {
        println!("Mr. ");
    }
}

// The following line allows rust to print insts of this struct
#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    // User {
    //     email: email,
    //     username: username,
    //     active: true,
    //     sign_in_count: 1,
    // }

    // Notice we didnt have to repeat all the attributes since the params match
    // them, this is a shorthand 
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

pub fn run() {
    let mut c = Color {
        red: 255,
        green: 0,
        blue: 0
    };
    println!("Color: {} {} {}", c.red, c.green, c.blue);

    let mut c = Color2(255, 0, 0);

    c.0 = 200;

    println!("Color: {} {} {}", c.0, c.1, c.2);
    let mut person = Person::new("Mary", "Doe");
    person.set_last_name("Jane");
    println!("Person {}", person.full_name());
    println!("Person Tuple {:?}", person.to_tuple());

    Person::print_mr();

    let mut user1 = build_user(String::from("someone@example.com"),String::from("someusername123"));
    println!("User Email {:?}", user1.email);

    // We can also create a copy of structs and change particular attributes as follows:
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
    // This would case an error since this attr is now borrowed by user2
    // println!("User Email {:?}", user1.username);

    // {:#?} does a pretty Debug print
    println!("User {:#?}", user2);

    // This is how we debug print (to stderr)
    dbg!(&user2);
}