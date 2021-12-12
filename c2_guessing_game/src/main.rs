use std::io;
use rand::Rng;
use std::cmp::Ordering;

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess{
        if value < 1 {
            panic!(
                "Guess value must be greater than or equal to 1, got {}.",
                value
            );
        } else if value > 100 {
            panic!(
                "Guess value must be less than or equal to 100, got {}.",
                value
            );
        }

        Guess { value }
    }

    pub fn get_value(&self) -> i32 {
        self.value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // This tells it that test should panic. note the expected makes it more precise on exactly what the panic should be
    #[should_panic(expected = "Guess value must be less than or equal to 100")] 
    fn greater_than_100() {
        Guess::new(200);
    }
}

fn main() {
    println!("Guess the number!");

    // range is [0,100]=[0,101) could also be specified by (1..=100)
    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        println!("You guessed: {}", guess);

        // shadows prev val of guess
        // Note: using the match here instead of a .expect("...")
        //       we are handling Ok vs Err on the .parse()
        // let guess: i32 = match guess.trim().parse() {
        //     Ok(num) => num,
        //     Err(_) => continue
        // };

        // Use a struct with built in error checking to handle
        let parsed_guess: Guess = Guess::new(guess.trim().parse().expect("Expected an int.."));

        // match expression is made of arms
        match parsed_guess.get_value().cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
