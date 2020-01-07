fn main() {
    let s1 = String::from("hello");
    let _h = s1[0];
}

// error[E0277]: the type `std::string::String` cannot be indexed by `{integer}`
//  --> src/main.rs:3:14
//   |
// 3 |     let _h = s1[0];
//   |              ^^^^^ `std::string::String` cannot be indexed by `{integer}`
//   |
//   = help: the trait `std::ops::Index<{integer}>` is not implemented for `std::string::String`
