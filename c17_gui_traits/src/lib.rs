// Note: When we use trait objects, Rust must use dynamic dispatch. 
//      The compiler doesn’t know all the types that might be used with the
//      code that is using trait objects, so it doesn’t know which method 
//      implemented on which type to call. Instead, at runtime, Rust uses 
//      the pointers inside the trait object to know which method to call. 
//      There is a runtime cost when this lookup happens that doesn’t occur 
//      with static dispatch.

pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,

    // The following would fail because Clone returns Self
    // and trait objects cannot return Self since when a 
    // trait object is used it forgets the concrete type.
    // 
    // pub failing_ex: Vec<Box<dyn Clone>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // code to actually draw a button
        println!("Drew Button");
    }
}
