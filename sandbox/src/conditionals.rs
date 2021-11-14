pub fn run(){
    let age: u8 = 38;
    let check_id: bool = false;
    let knows_person_of_age: bool = true;

    // If/Else
    if age >= 21 && check_id || knows_person_of_age {
        println!("Bartender: What would you like to drink?");
    } else if age < 21 && check_id {
        println!("Bartender: Sorry, you have to leave.");
    } else {
        println!("Bartender: I'll need to see your ID.");
    }

    // Shorthand If
    let is_of_age = if age >= 21 { true } else { false };
    println!("Is of age: {}", is_of_age);
    
    // The return value of all arms must be same type
    // The following code would not compile
    // let is_of_age = if age >= 21 { true } else { 21 };

    // Note: that conditional expressions must evaluate to a bool
    // The following code would not compile
    // if 3 {
    //     println!("This cant work!")
    // }
}