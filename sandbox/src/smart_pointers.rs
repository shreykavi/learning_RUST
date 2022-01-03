// Smart Pointers: Similar to ref pointers but have additional metadata and capabilities
// implements Deref and Drop

use std::rc::Rc;

// Cons is construct fn which is used for recursive lists (not v useful in rust)
enum List {
    // Using a box here allows rust to infer size correctly (since its now a pointer which is predictable)
    // Cons(i32, List),
    // Cons(i32, Box<List>),
    Cons(i32, Rc<List>), // this was changed to Rc for multiple owners ex
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

// Drop is used when rust cleans up
struct CustomSmartPointer {
    data: String,
}
impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn box_ex(){
    // Box<T> lets us store data on heap (heap storage is more efficient)
    // on deallocation all pointers and data on heap are collected
    let b = Box::new(5);
    println!("b = {}", b);

    // let list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))));

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


    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    // We are not allowed to call drop as a method
    // c.drop();
    drop(c);
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");
}

/// This shows an example of Rc counting the read only owners of the data
fn multi_owner_ex(){
    // Ex fails due to multiple owners
    // let a = List::Cons(5, Box::new(List::Cons(10, Box::new(List::Nil))));
    // let b = List::Cons(3, Box::new(a));
    // let c = List::Cons(4, Box::new(a));

    // in this example we use Rc::clone rather than .clone() -> 
    //          When looking for performance problems in the code,
    //          we only need to consider the deep-copy clones and can 
    //          disregard calls to Rc::clone
    let a = Rc::new(List::Cons(5, Rc::new(List::Cons(10, Rc::new(List::Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = List::Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));

    {
        let c = List::Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}


pub fn run(){
    // box_ex();
    multi_owner_ex();
}