fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");

    assert_eq!("", post.content());
}
