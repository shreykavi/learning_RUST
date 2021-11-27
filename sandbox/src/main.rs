mod print;
mod strings;
mod tuples;
mod types;
mod vars;
mod arrays;
mod vectors;
mod conditionals;
mod loops;
mod functions;
mod pointer_ref;
mod structs;
mod enums;
mod cli;
mod ownership;

// Including module from path
// #[path = "practice_code/outside_mod.rs"] mod outside_mod;
mod practice_code;

// import separate crate
use modules_and_crates;

fn main() {
    // print::run();
    // vars::run();
    // types::run();
    // strings::run();
    // tuples::run();
    // arrays::run();
    // vectors::run();
    // conditionals::run();
    // loops::run();
    // functions::run();
    // pointer_ref::run();
    // structs::run();
    enums::run();
    // cli::run();
    // practice_code::run();
    // modules_and_crates::front_of_house::hosting::add_to_waitlist();
    // ownership::run();
}
