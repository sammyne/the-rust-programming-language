fn main() {
    use std::collections::HashMap;

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    // The type annotation `HashMap<_, _>` is needed here because it’s possible to
    // `collect` into many different data structures and Rust doesn’t know which you
    // want unless you specify. For the parameters for the key and value types,
    // however, we use underscores, and Rust can infer the types that the hash map
    // contains based on the types of the data in the vectors
    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    println!("{:?}", scores);
}
