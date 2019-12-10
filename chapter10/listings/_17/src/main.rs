// This code won't compile because the value r is referring to has gone out of scope before we try
// to use it.
fn main() {
    let r;

    {
        let x = 5;
        r = &x;
    }

    println!("r: {}", r);
}