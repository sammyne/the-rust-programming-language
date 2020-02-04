fn main() {
    let some_option_value = Some(123);

    if let Some(x) = some_option_value {
        println!("{}", x);
    }
}
