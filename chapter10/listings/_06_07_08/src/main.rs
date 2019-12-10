// declare the name of the type parameter inside angle brackets
// just after the name of the struct
// @note x and y must be of the same type
struct Point<T> {
    x: T,
    y: T,
}

// Point with x and y of different types
struct PointV2<T, U> {
    x: T,
    y: U,
}

fn main() {
    // Listing 10-6: A Point<T> struct that holds x and y values of type T
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };

    // Listing 10-7: The fields x and y must be the same type because both have the same
    // generic data type T
    // let wont_work = Point { x: 5, y: 4.0 };

    // Listing 10-8: A PointV2<T, U> generic over two types so that x and y can be values of
    // different types
    let integer_and_float = PointV2 { x: 5, y: 4.0 };
}
