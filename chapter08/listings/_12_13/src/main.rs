fn main() {
    let data = "initial contents";

    let _s = data.to_string();

    // the method also works on a literal directly:
    let _s = "initial contents".to_string();

    let _s = String::from("initial contents");
}
