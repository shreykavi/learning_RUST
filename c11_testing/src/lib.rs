//  TESTS: cargo test
// try out: `cargo test --help` and `cargo test -- --help`
// Examples:
// cargo test -- --test-threads=1 
// cargo test -- --show-output
// cargo test exploration : this runs 1 test

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

// This is how you specify unittests and they go in same file as code
// note that these dont get included in the binary
#[cfg(test)]
mod tests {
    // Need this to bring Rectangle into scope of inner mod
    use super::*;

    #[test]
    fn exploration() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    // #[test]
    // fn another() {
    //     panic!("Make this test fail");
    // }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn it_adds_two() {
        // Check if both eqal
        assert_eq!(4, add_two(2));

        // everything after assert params go to format! 
        // assert_eq!(5, add_two(2), "The two values are not EQUAL! {} != {}", 5, add_two(2));

        // assert_ne! for not equal
    }

    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}
