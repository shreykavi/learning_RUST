/*
Primitive Types

Scalar Types:
    Integers: u8, i8, u16, i16, u32, i64, u64, i64, u128, i128, isize, usize (last 2 are based on arch of computer)
    Floats: f32, f64
    Boolean: bool
    Characters: char
Tuples
Arrays
*/

pub fn run() {
    // Default is i32
    let x = 1;

    // Default is f64
    let y = 2.5;

    // Add explicit type
    let z: i64 = 4545454545455;

    // Find max size
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);

    // Boolean
    let is_active = true;

    // Get boolean from expression
    let is_greater: bool = 10 < 5;

    let a1 = 'a';
    let face = '\u{1F600}'; //Emoji unicodes also work

    println!("{:?}", (x, y, z, is_active, a1, face));

    // Different ways to represent numbers
    let dec = 98_222;
    let hex = 0xff;
    let oct = 0o77;
    let bin = 0b1111_0000;
    let byte = b'A';

    println!("{:?} {:?} {:?} {:?} {:?}", dec, hex, oct, bin, byte);

    // int overflow example
    let mut x: i8 = 0;
    // x = 257; // note this would cause compilation error but if arithmetic 
    // caused this overflow it could pass and be handled in runtime
    println!("{}", x);
}
