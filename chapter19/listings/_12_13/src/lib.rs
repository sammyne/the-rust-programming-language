// Listing 19-12: The definition of the Iterator trait that has an associated type Item
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}

// Listing 19-13: A hypothetical definition of the Iterator trait using generics
pub trait Iterator2<T> {
    fn next(&mut self) -> Option<T>;
}
