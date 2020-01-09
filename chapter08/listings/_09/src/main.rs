fn main() {
    let mut v = vec![100, 32, 57];
    // To change the value that the mutable reference refers to, we have to use the
    // dereference operator (`*`) to get to the value in `i` before we can use the
    // `+=` operator
    for i in &mut v {
        *i += 50;
    }

    println!("{:?}", &v);
}
