// Reference Pointers - Point to a resource in memory

pub fn run (){
    // Primitive Array
    let arr1 = [1,2,3];
    let arr2 = arr1;

    // With non-primitives, if you assign another variable to a piece of data, the first variable will no longer hold that value. You'll need to use a reference (&) to point to the resource

    // Vector
    let mut vec1 = vec![1,2,3];
    let vec2 = &vec1; 
    print_vec(&vec1);
    println!("Values: {:?}", (&vec1, vec2));

    let mut s = String::from("hello");
    // We cannot borrow s as mutable more than once at a time; following line would error
    // let r1 = &mut s;
    change(&mut s);

    // We also cannot have a mutable reference while we have an immutable one
    // If we added a `println!("{} and {}", r1, r2);` before mut this would fix issue as it would end their scopes
    // let r1 = &s; // no problem
    // let r2 = &s; // no problem
    // let r3 = &mut s; // BIG PROBLEM

    // println!("{} {}", r1, s)
}

// Example of pointer fn
// Note: vec is a borrowed var (referenced) and so we cannot change it
fn print_vec(vec: &Vec<i32>){
    println!("{:?}", vec);
}

// Accepts mutable reference (we can change it)
fn change(some_string: &mut String){
    some_string.push_str(" WORLD")
}