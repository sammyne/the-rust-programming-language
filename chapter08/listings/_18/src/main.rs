fn main() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");

    // The `+` operator uses the `add` method, whose
    // signature looks something like this:
    //
    // fn add(self, s: &str) -> String {
    //
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used

    println!("{}", s3);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("{}", s);
}
