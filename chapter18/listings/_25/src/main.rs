fn main() {
    let numbers = (2, 4, 8, 16, 32);

    // error out as '.. can only be used once per tuple pattern'
    match numbers {
        (.., second, ..) => println!("Some numbers: {}", second),
    }
}
