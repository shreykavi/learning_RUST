// Trait: group method signatures to define set of behaviors

// If traits are made public they can be imported as follows 
// `use aggregator::Summary;` but due to coherence we can only 
// make these definitions in the same crates

// Each struct impling the following trait must define method and its
// behavior unique to its type
pub trait Summary {
    // The following has no default behavior
    fn summarize_author(&self) -> String;

    // The following will define a default behavior
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

// This fn takes any structs that impls Summary (impl trait syntax)
// pub fn notify(item1: &impl Summary, item2: &impl Summary) {
//     println!("Breaking news! {}", item1.summarize());
// }

// Longer version of the above; note that with more params this 
// become simpler to read (trait bound syntax)
// 
pub fn notify<T: Summary>(item1: &T, item2: &T) {
    println!("Breaking news! {}", item1.summarize());
}

// Can also use + if multiple traits must be impled
// pub fn notify(item: &(impl Summary + Display)) {
//     ...
// }

// trait can also be used in return type
// caveat: can only be used if fn returns a single type
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}

// With generics the where syntax is less cluttered 
// fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
//     ...
// }
// fn some_function<T, U>(t: &T, u: &U) -> i32
//     where T: Display + Clone,
//           U: Clone + Debug
// {}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    // Without this definition it'll use the default behavior
    // fn summarize(&self) -> String {
    //     format!("{}, by {} ({})", self.headline, self.author, self.location)
    // }

    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }

    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub fn run(){
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    println!("New article available! {}", article.summarize());

    notify(&tweet, &tweet);
}

// We can also conditionally impl a method if the generic impls a Trait
use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}
// can only call cmp_display if each T impls Display + PartialOrd
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

// Can conditionally impl a Trait if T contains a specific Trait
// impl<T: Display> ToString for T {
//     // --snip--
// }