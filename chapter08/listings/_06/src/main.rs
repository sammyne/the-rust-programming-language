fn main() {
    let v = vec![1, 2, 3, 4, 5];

    // below will panic
    let _does_not_exist = &v[100];

    // When the `get` method is passed an index that is outside the vector, it returns `None`
    // without panicking
    let _does_not_exist = v.get(100);
}
