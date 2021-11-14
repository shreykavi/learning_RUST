
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
}