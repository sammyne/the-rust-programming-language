fn main() {
    // Listing 19-1: Creating raw pointers from references
    let mut num = 5;

    let _r1 = &num as *const i32;
    let _r2 = &mut num as *mut i32;

    // Listing 19-2: Creating a raw pointer to an arbitrary memory address
    let address = 0x012345usize;
    let _r = address as *const i32;
}
