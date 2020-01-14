use submodules;

mod common;

#[test]
fn it_adds_two() {
    assert_eq!(4, submodules::add_two(2));
}
