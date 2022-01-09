use c17_blog::Post;
mod second_lib;

fn main() {
    // Og impl uses states
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());

    // Second impl doesn't exactly follow the state pattern
    // but gives us more safety by not impling .content on 
    // anything but approved posts
    let mut post = second_lib::Post::new();

    post.add_text("I ate a salad for lunch today");

    // Notice we have to reassign these and state isnt
    // fully encapsulated
    let post = post.request_review();
    let post = post.approve();

    assert_eq!("I ate a salad for lunch today", post.content());
}