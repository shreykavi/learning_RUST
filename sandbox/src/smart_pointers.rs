// Smart Pointers: Similar to ref pointers but have additional metadata and capabilities
// implements Deref and Drop

// Cons is construct fn which is used for recursive lists (not v useful in rust)
enum List {
    // Using a box here allows rust to infer size correctly (since its now a pointer which is predictable)
    // Cons(i32, List),
    Cons(i32, Box<List>),
    Nil,
}

// Custom box type
use std::ops::Deref;

struct MyBox<T>(T);
impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}
// Deref allows us to use *
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

pub fn run(){
    // Box<T> lets us store data on heap (heap storage is more efficient)
    // on deallocation all pointers and data on heap are collected
    let b = Box::new(5);
    println!("b = {}", b);

    let list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))));

    let x = 5;
    let y = MyBox::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y); // ex of Deref

    // This impls "deref coercion" which derefs &MyBox<String> into &String and then into &str
    let m = MyBox::new(String::from("Rust"));
    hello(&m);
    // deref coercion happens in the following cases:
    // - From &T to &U when T: Deref<Target=U>
    // - From &mut T to &mut U when T: DerefMut<Target=U>
    // - From &mut T to &U when T: Deref<Target=U>
}