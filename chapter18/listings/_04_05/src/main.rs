fn main() {
    // Listing 18-4: Using a pattern to destructure a tuple and create three variables at once
    let (x, y, z) = (1, 2, 3);

    // Listing 18-5: Incorrectly constructing a pattern whose variables don’t match the number of elements in the tuple
    // remove the comment would error out
    // let (x, y) = (1, 2, 3);
}
