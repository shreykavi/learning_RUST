pub fn run(){
    let s1 = String::from("hello");
    let s2 = s1;

    // This would fail since value has been moved
    // to s2 (https://doc.rust-lang.org/stable/book/ch04-01-what-is-ownership.html#ways-variables-and-data-interact-move)
    // println!("{}, world!", s1);

    // cloning is similar to deep copy and copies data on the heap
    // not just reference of pointer, len, and capacity
    // NOTE: This is not a problem with scalar values (get stored on stack not heap)
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);


    // Here is an example of scope and functions:
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    // fails:
    // println!("{}", s);

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it's okay to still
                                    // use x afterward
    println!("{}", x);

} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.


fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.