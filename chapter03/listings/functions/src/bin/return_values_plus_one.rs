fn main() {
    let x = plus_one(5);

    println!("The value of x is: {}", x);
}

fn plus_one(x: i32) -> i32 {
    // change 'x+1' to 'x+1;' would be an error 
    x + 1
}