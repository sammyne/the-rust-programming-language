fn main() {
    let s = Some(String::from("Hello!"));

    if let Some(_s) = s {
        println!("found a string");
    }

    // error out as 'value borrowed here after partial move'
    println!("{:?}", s);
}
