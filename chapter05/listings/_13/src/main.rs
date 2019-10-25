#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// To define the function within the context of `Rectangle`, we start an `impl`
// (implementation) block
impl Rectangle {
    // function within `impl` block with `self` as first parameter is a method 
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        // The method syntax goes after an instance: we add a dot followed by the method name,
        // parentheses, and any arguments
        rect1.area()
    );
}
