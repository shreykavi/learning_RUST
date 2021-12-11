// Generics: allow you to specify not certain types that will be decided by RUST
// Rust uses monomorphization to turn generics into concretes on compile so no performance cost
// 
// You can use as many generic type parameters as you want, but 
// using more than a few makes your code hard to read and could 
// indicate that your code needs restructuring into smaller pieces.

// structs can use generics
struct Point<T> {
    x: T,
    y: T,
}
struct BetterPoint<T, U> {
    x: T,
    y: U,
}

// Can be used within impl
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}
// You can also add variation in the parameters with new generics 
impl<T, U> BetterPoint<T, U> {
    fn mixup<V, W>(self, other: BetterPoint<V, W>) -> BetterPoint<T, W> {
        BetterPoint {
            x: self.x,
            y: other.y,
        }
    }
}


// Enums too; for example the builtins Optional and Result
enum Option<T> {
    Some(T),
    None,
}
enum Result<T, E> {
    Ok(T),
    Err(E),
}

fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// Generic version of the function
// Note that T has to impl PartialOrd trait since 
// std uses that for >

// fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
//     let mut largest = list[0];

//     for &item in list {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }

// The following verison of largest uses refs instead of Clone or Copy:
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = &item;
        }
    }

    largest
}

pub fn run() {
    let number_list = vec![34, 50, 25, 100, 65];

    // let result = largest_i32(&number_list);
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);

    // Using struct with generic:
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    // Note that both generics have to be of the type unless specified serparately
    // let wont_work = Point { x: 5, y: 4.0 };
    let wont_work = BetterPoint { x: 5, y: 4.0 };

    // In this example notice that mixup returns a i32 for x and a char for y
    let p1 = BetterPoint { x: 5, y: 10.4 };
    let p2 = BetterPoint { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}