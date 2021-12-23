use std::thread;
use std::time::Duration;
use std::collections::HashMap;

// This cacher allows us to memoize the closure
struct Cacher<T,P,R>
where
    T: Fn(&P) -> R,
    P: std::cmp::Eq + std::hash::Hash
{
    calculation: T,
    values: HashMap<P,R>,
}
impl<T,P,R> Cacher<T,P,R>
where
    T: Fn(&P) -> R,
    P: std::cmp::Eq + std::hash::Hash
{
    fn new(calculation: T) -> Cacher<T,P,R> {
        Cacher {
            calculation,
            values: HashMap::new(),
        }
    }

    fn value(&mut self, arg: P) -> &R {
        use std::collections::hash_map::Entry;
        match self.values.entry(arg) {
            Entry::Occupied(entry) => entry.into_mut(),
            Entry::Vacant(entry) => {
                let v = (self.calculation)(entry.key());
                entry.insert(v)
            }
        }
    }
}

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

fn generate_workout(intensity: u32, random_number: u32) {
    // let expensive_result = simulated_expensive_calculation(intensity);

    // We could type annotate this closure but overly verbose and rust can infer 
    // types in such a narrow scope
    // Execute this as: expensive_closure(intensity)
    // 
    // let expensive_closure = |num| {
    //     println!("calculating slowly...");
    //     thread::sleep(Duration::from_secs(2));
    //     num
    // };

    // Using memoization cacher
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        *num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}

fn examples(){
    let example_closure = |x| x;

    // The second line here would cause an error since
    // we use the same closure with different types
    let s = example_closure(String::from("hello"));
    // let n = example_closure(5);


    // Notice that x is not a param to the closure but
    // can still be used, closures capture env in memory
    // All vars are implemented with: (and which is inferred)
    // FnOnce (consumes), FnMut (can change env), Fn (immutable)
    let x = 4;
    let equal_to_x = |z| z == x;
    let y = 4;
    assert!(equal_to_x(y));

    // Example of using move keyword
    let x = vec![1, 2, 3];
    let equal_to_x = move |z| z == x;
    // println!("can't use x here: {:?}", x); // Once moved it cant be used in this scope
    let y = vec![1, 2, 3];
    assert!(equal_to_x(y));
}

pub fn run(){
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);
}

#[test]
fn call_with_different_values() {
    // closure takes a param and rets it
    let mut c = Cacher::new(|a| *a);

    let v1 = c.value(1);
    let v2 = c.value(2);

    assert_eq!(*v2, 2);
}