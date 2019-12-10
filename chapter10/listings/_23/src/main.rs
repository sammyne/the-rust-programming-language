// @note declare generic lifetime parameters inside angle brackets between the function name and the parameter list
// @dev The constraint we want to express in this signature is that all the references in the parameters and the return value must have the same lifetime
// @dev the lifetime of the returned value will be the overlapping scope of x and y
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }
}
