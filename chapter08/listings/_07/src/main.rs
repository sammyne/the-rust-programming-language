fn main() {
    let mut v = vec![1, 2, 3, 4, 5];

    let first = &v[0];

    v.push(6);

    println!("The first element is: {}", first);
}

// Output:
// error[E0502]: cannot borrow `v` as mutable because it is also borrowed as immutable
//  --> src/main.rs:6:5
//   |
// 4 |     let first = &v[0];
//   |                  - immutable borrow occurs here
// 5 |
// 6 |     v.push(6);
//   |     ^^^^^^^^^ mutable borrow occurs here
// 7 |
// 8 |     println!("The first element is: {}", first);
//   |                                          ----- immutable borrow later used here
