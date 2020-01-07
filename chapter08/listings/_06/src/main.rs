fn main() {
    let v = vec![1, 2, 3, 4, 5];

    // below will panic
    let _does_not_exist = &v[100];

    // but not this one
    let _does_not_exist = v.get(100);
}
