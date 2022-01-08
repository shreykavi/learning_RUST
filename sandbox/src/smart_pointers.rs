// Smart Pointers: Similar to ref pointers but have additional metadata and capabilities
// implements Deref and Drop

use std::rc::Rc;
use std::cell::RefCell;

// Cons is construct fn which is used for recursive lists (not v useful in rust)
#[derive(Debug)]
enum List {
    // Using a box here allows rust to infer size correctly (since its now a pointer which is predictable)
    // Cons(i32, List),
    // Cons(i32, Box<List>),
    // Cons(i32, Rc<List>), // this was changed to Rc for multiple owners ex
    Cons(Rc<RefCell<i32>>, Rc<List>), // this change allows us to have both multiple owners and that you can mutate!
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
// fn multi_owner_ex(){
//     // Ex fails due to multiple owners
//     // let a = List::Cons(5, Box::new(List::Cons(10, Box::new(List::Nil))));
//     // let b = List::Cons(3, Box::new(a));
//     // let c = List::Cons(4, Box::new(a));

//     // in this example we use Rc::clone rather than .clone() -> 
//     //          When looking for performance problems in the code,
//     //          we only need to consider the deep-copy clones and can 
//     //          disregard calls to Rc::clone
//     let a = Rc::new(List::Cons(5, Rc::new(List::Cons(10, Rc::new(List::Nil)))));
//     println!("count after creating a = {}", Rc::strong_count(&a));
//     let b = List::Cons(3, Rc::clone(&a));
//     println!("count after creating b = {}", Rc::strong_count(&a));

//     {
//         let c = List::Cons(4, Rc::clone(&a));
//         println!("count after creating c = {}", Rc::strong_count(&a));
//     }
//     println!("count after c goes out of scope = {}", Rc::strong_count(&a));
// }

pub fn interior_mut_ex(){
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(List::Cons(Rc::clone(&value), Rc::new(List::Nil)));

    let b = List::Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = List::Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}

pub fn run(){
    // box_ex();
    // multi_owner_ex();
    interior_mut_ex();
}


// interior mutability in a test mocker example:
pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger
                .send("Warning: You've used up over 75% of your quota!");
        }
    }
}

// In this test we use RefCell to allow us to borrow a mut ref after a immut ref
#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
        // sent_messages: Vec<String>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
                // sent_messages: vec![],
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            // Since self is & (immutable) is cannot be borrowed as mutable here
            // self.sent_messages.push(String::from(message));
            
            self.sent_messages.borrow_mut().push(String::from(message));

            // Note this doesnt let us break the paradism of only being able to have 1
            // mut ref at a time (Notice: RefCell panics on Runtime rather than compile)

            // let mut one_borrow = self.sent_messages.borrow_mut();
            // let mut two_borrow = self.sent_messages.borrow_mut();
            // one_borrow.push(String::from(message));
            // two_borrow.push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        // assert_eq!(mock_messenger.sent_messages.len(), 1);
        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}