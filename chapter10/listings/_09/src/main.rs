struct Point<T> {
    x: T,
    y: T,
}

// @note T must go just after impl so we can use it to specify that we're implementing methods
// on the type Point<T>
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

fn main() {
    let p = Point { x: 5, y: 10 };

    println!("p.x = {}", p.x());
}
